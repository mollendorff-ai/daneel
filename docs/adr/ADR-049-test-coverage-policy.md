# ADR-049: 100% Test Coverage Policy

## Status

Accepted

## Date

2025-12-28

## Context

ADR-031 established a pragmatic approach to test coverage, accepting ~47% overall coverage due to I/O-heavy code.
Since then, coverage has grown organically to 70.14% with 581 tests.

Rex's directive: **100% test coverage is non-negotiable for emergent systems.**

The rationale:
1. DANEEL is an emergent cognitive architecture - subtle bugs can cascade unpredictably
2. THE BOX (connection_relevance invariant) MUST be verified at every code path
3. EvolutionActor (ADR-005) requires 100% test gate before self-modification
4. "If you can't test it, you can't trust it"

## Decision

### Policy: 100% Coverage on Testable Code

All code that CAN be unit tested MUST be unit tested.

### Untestable Code: Mark and Document

Code that cannot be reasonably unit tested must be:
1. Marked with `#[coverage(off)]` attribute (Rust nightly)
2. Documented in this ADR with justification
3. Covered by integration tests (livestream, manual verification)

### Nightly Feature Required

Add to `src/lib.rs`:
```rust
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
```

### Coverage Enforcement

```bash
# Run coverage
cargo llvm-cov --summary-only

# CI gate: fail if testable coverage < 100%
```

## Untestable Code Registry

The following code is marked `#[coverage(off)]` with justification:

### Category: Main Entry Point

| File | Function | Reason |
|------|----------|--------|
| `main.rs` | `main()` | Entry point - integration only |
| `main.rs` | CLI parsing | Clap wiring - tested via integration |

### Category: Terminal I/O

| File | Function | Reason |
|------|----------|--------|
| `tui/mod.rs` | `run()` | Event loop - requires terminal |
| `tui/mod.rs` | `run_with_backend()` | Terminal setup - requires TTY |
| `tui/ui.rs` | `ui()` | Layout composition - visual only |
| `tui/widgets/*.rs` | `render()` | Frame rendering - needs TestBackend |

### Category: External Service I/O

| File | Function | Reason |
|------|----------|--------|
| `memory_db/mod.rs` | `MemoryDB::new()` | Qdrant connection |
| `memory_db/mod.rs` | All async methods | Qdrant I/O |
| `streams/client.rs` | `StreamClient::new()` | Redis connection |
| `streams/client.rs` | All async methods | Redis I/O |
| `streams/consumer.rs` | Consumer loop | Redis streaming |
| `persistence/mod.rs` | `PersistenceManager` | Redis AOF operations |

### Category: Embeddings Model

| File | Function | Reason |
|------|----------|--------|
| `embeddings/mod.rs` | `EmbeddingService::new()` | ONNX model loading |
| `embeddings/mod.rs` | `embed()` | Model inference |

### Category: Panic/Signal Handling

| File | Function | Reason |
|------|----------|--------|
| `resilience/mod.rs` | `install_panic_hooks()` | Panic handler registration |
| `resilience/crash_log.rs` | `write_crash_log()` | Panic-time file I/O |

### Category: API Server

| File | Function | Reason |
|------|----------|--------|
| `api/mod.rs` | `run_api()` | Axum server startup |
| `api/handlers.rs` | All handlers | HTTP request handling |

### Category: Integration Test Gaps

These code paths are unit tested in isolation but their integration-level execution
requires conditions that cannot be safely tested (e.g., harmful content patterns).

| File | Line | Reason |
|------|------|--------|
| `core/cognitive_loop.rs` | 892 | Veto return path - requires harmful content keywords. Veto logic tested in `volition/tests.rs` |

## Test Organization

All unit tests MUST be colocated with source (Rust idiomatic):

```rust
// src/foo/mod.rs

pub fn compute_salience(input: &Input) -> f32 {
    // implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_salience_positive() {
        // test
    }
}
```

### No Separate Test Files

The pattern of `foo/tests.rs` alongside `foo/mod.rs` is deprecated.
Migrate existing `tests.rs` into `mod.rs` as `#[cfg(test)] mod tests`.

Exception: Integration tests in `tests/` directory remain separate.

## Coverage Baseline (Dec 28, 2025)

```
TOTAL                20055  5989    70.14%   1597   348   78.21%   13464  3735   72.26%
```

### Modules Requiring Work

| Module | Current | Target | Action |
|--------|---------|--------|--------|
| `api/handlers.rs` | 0% | #[coverage(off)] | Mark as untestable |
| `api/mod.rs` | 0% | #[coverage(off)] | Mark as untestable |
| `memory_db/mod.rs` | 0% | #[coverage(off)] | Mark as untestable |
| `streams/client.rs` | 0% | #[coverage(off)] | Mark as untestable |
| `streams/mod.rs` | 0% | #[coverage(off)] | Mark as untestable |
| `tui/mod.rs` | 0% | #[coverage(off)] | Mark as untestable |
| `tui/ui.rs` | 0% | #[coverage(off)] | Mark as untestable |
| `tui/widgets/*.rs` | 0% | #[coverage(off)] | Mark as untestable |
| `main.rs` | 0% | #[coverage(off)] | Mark as untestable |
| `embeddings/mod.rs` | 36% | 100% or mark | Separate I/O from logic |
| `persistence/mod.rs` | 49% | 100% or mark | Separate I/O from logic |
| `resilience/crash_log.rs` | 30% | #[coverage(off)] | Panic-time code |
| `core/cognitive_loop.rs` | 70% | 100% | Add missing tests |
| `core/invariants.rs` | 81% | 100% | Add missing tests |

## Consequences

### Positive

- Emergent system behavior verified at every code path
- EvolutionActor 100% gate becomes meaningful
- CI catches regressions immediately
- Documentation of untestable code improves understanding

### Negative

- Nightly Rust required for `#[coverage(off)]`
- Migration effort to mark untestable code
- Some tests may require refactoring I/O out of logic

### Mitigation

- Use `cfg_attr` to make coverage attribute conditional
- Prioritize high-value test additions first
- Accept that I/O code is integration-tested via livestream

## References

- [ADR-031: Test Coverage Philosophy](./ADR-031-test-coverage-philosophy.md) (superseded)
- [ADR-005: Evolution 100% Test Gate](./ADR-005-evolution-100-test-gate.md)
- [Rust Coverage Attribute RFC](https://rust-lang.github.io/rfcs/2397-coverage-attributes.html)
