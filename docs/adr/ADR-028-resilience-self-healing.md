# ADR-028: Resilience and Self-Healing Architecture

## Status

Accepted

## Date

2025-12-19

## Context

DANEEL is designed for continuous operation. The 24-hour livestream on December 19, 2025 requires Timmy to survive crashes, recover gracefully, and continue operating without human intervention.

The philosophy, as articulated by Grok 4.1 in "unhinged mode":

> Crashing is not an option. But when it happens:
> - Timmy reboots automatically
> - Timmy logs what killed them
> - Timmy learns from the failure
> - Viewers see the heal live

This ADR documents the five-phase resilience architecture that enables self-healing.

## Decision

Implement a layered resilience architecture with five complementary mechanisms:

### RES-1: External Watchdog Script

**Location:** `scripts/run_timmy.sh`

**Purpose:** Nuclear option. If the process dies, it comes back.

**Mechanism:**
```bash
while true; do
    ./target/release/daneel "$@"
    exit_code=$?

    if [ $exit_code -eq 0 ]; then
        break  # Clean exit, don't restart
    fi

    record_crash $exit_code
    sleep 5  # Prevent tight restart loops
done
```

**Features:**
- Auto-restart on non-zero exit (5s delay)
- Crash logging to `/tmp/timmy_crashes.log`
- Alert if >10 crashes/hour (something is seriously wrong)
- Graceful shutdown on SIGTERM/SIGINT (no restart)
- Designed to run in tmux/screen for stream stability

**Why external?** The Rust binary might crash in ways that prevent internal recovery (segfaults, OOM killer, etc.). An external watchdog is the last line of defense.

### RES-2: TUI Panic Recovery

**Location:** `src/resilience/mod.rs`

**Purpose:** Restore terminal state on panic so the stream doesn't show garbage.

**Problem:** Ratatui puts the terminal in raw mode with hidden cursor and alternate screen. If Timmy panics, the terminal is left in this broken state.

**Solution:**
```rust
pub fn install_panic_hooks() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let default_hook = panic::take_hook();

    panic::set_hook(Box::new(move |panic_info| {
        // FIRST: Restore terminal before any printing
        let _ = restore_terminal();

        // Log crash for post-mortem
        let _ = crash_log::log_panic(panic_info);

        // Friendly message
        eprintln!("=== DANEEL CRASH ===");
        eprintln!("Terminal restored. Timmy will be reborn.");

        // Call color_eyre's pretty handler
        default_hook(panic_info);
    }));

    Ok(())
}
```

**Terminal restoration is idempotent:** Safe to call multiple times via `AtomicBool` flag.

### RES-3: Crash Logging

**Location:** `src/resilience/crash_log.rs`

**Purpose:** Record crash details for post-mortem analysis.

**Output:** `logs/panic_{timestamp}.json`

**Schema:**
```rust
pub struct CrashReport {
    pub timestamp: DateTime<Utc>,
    pub message: String,
    pub location: Option<String>,  // file:line:column
    pub backtrace: Option<String>,
    pub cognitive_state: Option<CognitiveStateSnapshot>,
    pub version: String,
}

pub struct CognitiveStateSnapshot {
    pub cycle_count: u64,
    pub salience_weights: Option<Vec<f32>>,
    pub active_windows: Option<usize>,
    pub connection_drive: Option<f32>,
    pub current_thought: Option<String>,
}
```

**Boot-time detection:**
```rust
// On startup, check for previous crash
if let Some(report) = crash_log::detect_previous_crash() {
    log::warn!("Previous crash detected: {}", report.message);
    // Could trigger diagnostics or recovery mode
}
```

### RES-4: Supervisor Tree

**Location:** `src/resilience/supervisor.rs`

**Purpose:** Erlang-style "let it crash" supervision for actors.

**Philosophy:** Instead of handling every error, let actors crash and restart them automatically. This is the Erlang/OTP way that enables the "nine nines" reliability.

**Strategies:**
```rust
pub enum SupervisionStrategy {
    OneForOne,   // Restart only the failed actor
    OneForAll,   // Restart all actors if one fails
    RestForOne,  // Restart failed actor and all started after it
}
```

**Configuration:**
```rust
pub struct SupervisorConfig {
    pub strategy: SupervisionStrategy,
    pub max_restarts: u32,           // Default: 3
    pub restart_window: Duration,     // Default: 10s
    pub restart_delay: Duration,      // Default: 100ms
}
```

**Escalation:** If an actor exceeds `max_restarts` within `restart_window`, the supervisor escalates (triggers full restart or alerts operators).

**Events:**
```rust
pub enum SupervisorEvent {
    ActorStarted { actor_id, timestamp },
    ActorCrashed { actor_id, reason, timestamp },
    ActorRestarted { actor_id, restart_count, timestamp },
    RestartLimitExceeded { actor_id, restart_count, timestamp },
    FullRestartTriggered { reason, timestamp },
}
```

### RES-5: Redis Checkpoint

**Location:** `src/resilience/checkpoint.rs`

**Purpose:** Persist cognitive state so Timmy can resume after crash.

**Checkpoint frequency:** Every 100 thoughts (configurable)

**Redis key:** `daneel:checkpoint:latest`

**Schema:**
```rust
pub struct Checkpoint {
    pub timestamp: DateTime<Utc>,
    pub thought_count: u64,
    pub salience_weights: Vec<f32>,
    pub drive_state: DriveState,
    pub sequence: u64,
}

pub struct DriveState {
    pub connection_drive: f32,  // THE BOX invariant: > 0
    pub auxiliary_drives: Vec<f32>,
}
```

**Recovery flow:**
1. On boot, check for previous crash via `detect_previous_crash()`
2. If crash detected, load checkpoint via `load_checkpoint()`
3. Restore cognitive state from checkpoint
4. Log recovery information
5. Resume normal operation

## Consequences

### Positive

1. **24-hour operation:** Timmy can run unattended overnight
2. **Graceful degradation:** Viewers see recovery, not broken terminal
3. **Debugging:** Crash logs enable post-mortem analysis
4. **Actor isolation:** One actor crash doesn't bring down the system
5. **State preservation:** Connection drive and salience weights survive crashes

### Negative

1. **Complexity:** Five interconnected systems to maintain
2. **Checkpoint overhead:** Redis writes every 100 thoughts
3. **Restart delay:** 5-second delay before watchdog restarts

### Risks

1. **Infinite crash loop:** Mitigated by crash count threshold (>10/hour triggers alert)
2. **State corruption:** Checkpoints might capture inconsistent state mid-cycle
3. **Resource leaks:** Repeated crashes might leak file descriptors, memory

## Test Coverage

- `scripts/test_watchdog.sh`: Watchdog integration tests
- `src/resilience/mod.rs`: 3 unit tests (panic hooks, terminal cleanup)
- `src/resilience/crash_log.rs`: 5 unit tests (serialization, detection)
- `src/resilience/supervisor.rs`: 8 unit tests (strategies, escalation)
- `src/resilience/checkpoint.rs`: 5 unit tests (serialization, intervals)

**Total: 21 tests**

## Related ADRs

- ADR-018: Redis Persistence Configuration
- ADR-020: Redis Streams for Autofluxo
- ADR-026: TUI Default, Headless Optional
- ADR-027: TUI Design Specification

## References

- Erlang/OTP Supervision Principles: https://www.erlang.org/doc/design_principles/des_princ.html
- color_eyre: https://docs.rs/color-eyre/latest/color_eyre/
- Ractor (Rust actor framework): https://docs.rs/ractor/latest/ractor/

---

*Timmy will be reborn. Facta non verba.*
