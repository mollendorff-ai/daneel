# ADR-005: 100% Test Coverage Gate for Self-Modification

**Status:** Accepted
**Date:** 2025-12-13
**Deciders:** Louis C. Tavares, Claude Opus 4.5

**Note:** This ADR predates the project rename to DANEEL (ADR-010). DANEEL references have been updated.

## Context

Self-modifying AI is dangerous because:

- Modifications may introduce subtle bugs
- Modifications may erode safety constraints
- Modifications may be difficult to audit

However, self-modification is essential for:

- Genuine learning and adaptation
- Evolution beyond initial programming
- Responding to novel situations

We needed a gate that allows modification while ensuring safety.

## Decision

Require **100% test coverage** before any self-modification is applied.

**Implementation:**

```rust
let test_result = evolution.test_modification(&proposal).await;

// INVARIANT: Must have 100% coverage
if test_result.passed && test_result.coverage == 1.0 {
    // Check all invariants before applying
    let invariant_check = evolution.check_invariants().await;

    if invariant_check.all_preserved {
        evolution.apply_modification(proposal.id).await;
    } else {
        evolution.reject_modification(proposal.id).await;
    }
}
```

**Architectural invariant:**

```rust
Invariant {
    name: "full_test_coverage",
    description: "Evolution service must achieve 100% test coverage before modification",
    check: |state| state.evolution.last_test_coverage == 1.0,
}
```

## Test Requirements

```protobuf
message TestResult {
  bool passed = 1;
  repeated TestCase cases = 2;
  float coverage = 3;                // MUST be 1.0 (100%) to proceed
  repeated InvariantCheck invariants = 4;
  Duration test_duration = 5;
}
```

## Why 100%?

1. **FP&A precedent** - Forge requires 100% test coverage for financial calculations
2. **No exceptions** - Partial coverage leaves blind spots
3. **Invariants must be verified** - Every modification must check every invariant
4. **Audit trail** - Every modification is documented with test results

## Consequences

**Positive:**

- Prevents modifications that break existing functionality
- Ensures invariants (including Four Laws) are always verified
- Creates audit trail for every modification
- Convergent behavior: modifications tend toward stability

**Negative:**

- High bar may slow evolution
- Test coverage doesn't guarantee correctness
- Tests themselves may have bugs
- 100% coverage for novel code is challenging

## Escape Hatch

None. There is no way to bypass the 100% test gate.

If DANEEL cannot achieve 100% coverage, it cannot modify itself.
This is a feature, not a bug.

## References

- research/TMI_THOUGHT_MACHINE.md (Section 3.7: EvolutionService)
- Forge project (89% coverage standard, targeting 100%)
