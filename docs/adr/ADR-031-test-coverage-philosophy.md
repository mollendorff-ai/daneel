# ADR-031: Test Coverage Philosophy

## Status

Accepted

## Date

2025-12-19

## Context

DANEEL requires high test coverage to ensure the cognitive architecture works correctly. However, the codebase includes:

1. **Pure logic** (actors, core, types) - easily unit testable
2. **I/O operations** (Redis, Qdrant) - require running services
3. **Terminal rendering** (TUI widgets) - require terminal backend
4. **Entry points** (main.rs) - wire components together

Blindly chasing "100% coverage" would require:
- Mock terminal backends for widget rendering
- Docker-based test infrastructure for Redis/Qdrant
- File system mocking for crash logs
- Destructive panic testing

This is significant complexity for diminishing returns.

## Decision

We adopt a **layered testing strategy**:

### Layer 1: Unit Tests (Target: 90%+ on logic)

Test all pure logic without external dependencies:

| Module | Coverage | Strategy |
|--------|----------|----------|
| `actors/*` | 95%+ | Test message handling, state transitions |
| `core/*` | 90%+ | Test cognitive loop, invariants, laws |
| `config/` | 100% | Test parsing, defaults |
| `tui/app.rs` | 93% | Test state machine, keyboard handling |
| `tui/colors.rs` | 100% | Test color functions |

### Layer 2: Integration Tests (The Livestream)

The 24-hour livestream IS the integration test:

| Component | Test Method |
|-----------|-------------|
| Redis Streams | Live data flow during stream |
| Qdrant | Memory consolidation during sleep cycles |
| TUI rendering | Visual verification on stream |
| Crash recovery | Watchdog restarts if needed |
| Persistence | State survives restarts |

### Layer 3: Not Unit Tested (By Design)

| Module | Lines | Reason |
|--------|-------|--------|
| `tui/widgets/*` | 220 | Render to Frame - needs terminal mock |
| `tui/ui.rs` | 36 | Layout composition - needs terminal |
| `tui/mod.rs` | 92 | Event loop - needs terminal |
| `main.rs` | 44 | Entry point - integration only |
| `streams/client.rs` | 186 | Redis I/O - needs Redis |
| `memory_db/mod.rs` | 149 | Qdrant I/O - needs Qdrant |
| `persistence/` | 129 | Redis I/O - needs Redis |
| `crash_log.rs` | 62 | File I/O + panics |

## Coverage Numbers

```
Total Coverage:     46.98% (1,126/2,397 lines)
Logic Coverage:     ~90% (actors, core, config)
I/O Coverage:       ~5% (serialization only)
TUI Logic Coverage: 93% (app.rs state machine)
TUI Render Coverage: 0% (by design)
```

### Test Count

| Module | Tests | Status |
|--------|-------|--------|
| actors | 201 | Excellent |
| core | 53 | Excellent |
| streams | 42 | Excellent |
| tui | 40 | Excellent |
| persistence | 23 | Good |
| memory_db | 22 | Good |
| resilience | 20 | Good |
| config | 12 | Good |
| **Total** | **414** | |

## Rationale

### Why Not Mock Everything?

1. **Terminal mocking is complex**: `ratatui::backend::TestBackend` exists but testing pixel-perfect rendering adds little value when the logic is tested.

2. **Redis/Qdrant mocks hide bugs**: Mock behavior diverges from real behavior. The livestream tests the real integration.

3. **Panic testing is destructive**: Testing `install_panic_hooks` requires triggering real panics.

4. **main.rs is glue**: It just wires components. If components work, main works.

### Why This Coverage Is Sufficient

1. **Logic is tested**: All decision-making code (actors, salience, memory consolidation) has 90%+ coverage.

2. **State machine is tested**: `tui/app.rs` at 93% covers all keyboard handling, thought queue management, animation logic.

3. **Serialization is tested**: Round-trip tests ensure data survives persistence.

4. **Integration is tested live**: The 24-hour stream tests everything together under real load.

## Consequences

### Positive

- Fast test suite (~0.3s for 414 tests)
- No external dependencies for CI
- Focus on testing what matters (logic, not rendering)
- Livestream provides real-world integration testing

### Negative

- 46.98% line coverage looks low on badges
- TUI rendering bugs only caught visually
- Redis/Qdrant bugs only caught at runtime

### Mitigation

- Document what's not tested and why
- Use the livestream as integration test
- Add integration test suite post-launch if needed

## References

- [Test Coverage Analysis - Dec 19, 2025](../methodology/TEST_COVERAGE.md)
- [ADR-028: Resilience and Self-Healing](./ADR-028-resilience-self-healing.md)
- [Blog: Pre-Birth Status](/blog/content/posts/20-pre-birth-status.md)
