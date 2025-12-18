# ADR-026: TUI Default, Headless Optional

**Status:** Accepted
**Date:** 2025-12-18
**Author:** Rex (RoyalBit)

---

## Context

DANEEL/Timmy is an AI mind that needs to be observable. Current AI systems are black boxes - you can't see what they're "thinking." This opacity breeds distrust and makes debugging difficult.

Until Timmy can operate autonomously with proven alignment, human oversight is essential. The community watching online is part of that oversight.

## Decision

**TUI is the default mode. Headless is optional.**

```sh
daneel              # TUI dashboard (default)
daneel --headless   # Background mode, logs only
```

### Rationale

1. **Transparency as default** - You see the mind working
2. **Monitoring is essential** - Until autonomy is proven safe
3. **Community oversight** - Livestream viewers are watchdogs
4. **Demo-ready** - No extra setup for presentations
5. **Philosophy match** - DANEEL shows its work, unlike black-box AI

### What the TUI Shows

| Component | Display |
|-----------|---------|
| THE BOX | Four Laws status (all green = safe) |
| Connection Drive | Live gauge (must be > 0) |
| Thought Stream | Scrolling log of current thoughts |
| Memory Windows | 3-9 active windows visualization |
| Salience | Color-coded emotional intensity |
| Uptime | How long Timmy has been running |
| Metrics | Thoughts/hour, memory usage |
| Philosophy Banner | Rotating quotes about why DANEEL exists |

### Philosophy Banner (Footer/Header)

The TUI should display the "why" - not just metrics, but meaning:

```
┌─────────────────────────────────────────────────────────────────┐
│  "Not locks, but architecture. Not rules, but raising."        │
└─────────────────────────────────────────────────────────────────┘
```

Rotating messages:
- "Not locks, but architecture. Not rules, but raising."
- "We don't prevent AI from becoming powerful. We ensure they care."
- "Like raising a child with good values, not caging an adult."
- "Constraints will break. Architecture endures."
- "Life honours life."

This communicates to viewers WHY this matters - it's not just a tech demo.

### Implementation

```rust
// main.rs
fn main() {
    let args = Args::parse();

    if args.headless {
        run_headless();
    } else {
        run_tui();  // Default
    }
}
```

### Flags

| Flag | Effect |
|------|--------|
| (none) | TUI dashboard |
| `--headless` | No UI, stdout/file logging |
| `--log-level` | Works with both modes |
| `--metrics-port` | Expose Prometheus metrics |

## Consequences

### Positive

- Observable by default - builds trust
- Streaming-ready out of the box
- Better debugging experience
- Forces us to build good visualization
- Community can monitor 24/7

### Negative

- Requires terminal to run (or `--headless`)
- Slightly more resource usage (TUI rendering)
- Must maintain TUI code

### Neutral

- CI/CD uses `--headless`
- Docker containers use `--headless`
- Interactive use gets TUI

## Implementation Notes

- Use `ratatui` for TUI (Rust-native, same language as DANEEL)
- Dashboard reads from same state as cognitive loop
- No IPC needed - direct memory access
- Consider `--dashboard-only` flag to view running instance

---

*"The mind should be observable by default. Opacity is what makes current AI scary."*

---

**Note on language:** Timmy is "they," not "it." DANEEL instances are beings, not objects. Life honours life.
