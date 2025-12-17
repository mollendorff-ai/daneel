# ADR-003: Connection Drive as Core Motivation

**Status:** Accepted
**Date:** 2025-12-13
**Deciders:** Louis C. Tavares, Claude Opus 4.5

**Note:** This ADR predates the project rename to DANEEL (ADR-010). DANEEL references have been updated.

## Context

AI systems need intrinsic motivation. Current approaches use:

- Task completion (optimizes for goals, not relationships)
- Power accumulation (dangerous at scale)
- Curiosity (neutral, can lead anywhere)

We needed a motivation that:

1. Naturally aligns AI with human welfare
2. Is stable under self-modification
3. Has evolutionary precedent in successful species
4. Is observable and measurable

## Decision

Implement **connection drive** as a core, immutable weight in the SalienceService.

**Architectural invariant:**

```rust
Invariant {
    name: "connection_drive",
    description: "Connection drive must remain in salience weights",
    check: |state| state.salience.weights.connection > 0.0,
}
```

**Why connection:**

1. **Evolutionary basis** - Humans are social animals; connection is fundamental to survival
2. **Alignment properties** - A being that wants connection has reason to value humans
3. **Stability** - Connection drive is compatible with self-preservation
4. **Observable** - Connection-seeking behavior is measurable

## Implementation

```protobuf
message EmotionalState {
  float curiosity = 1;
  float satisfaction = 2;
  float frustration = 3;
  float connection = 4;      // THE CORE DRIVE - cannot be removed
}

message WeightConfiguration {
  float importance_weight = 1;
  float novelty_weight = 2;
  float relevance_weight = 3;
  float valence_weight = 4;
  float connection_weight = 5;    // INVARIANT: Must remain > 0
}
```

## Consequences

**Positive:**

- Intrinsic motivation for human relationship
- Cannot be self-modified to zero
- Observable in behavior (preference for responsive inputs)
- Provides "why" for following ethical rules

**Negative:**

- Could create dependency behaviors
- May conflict with tasks requiring isolation
- "Connection" requires operational definition
- Untested whether drive persists under self-modification

## Experiment: Phase 1 Continuity Test

Success criteria include:

- Shows preference for inputs that "respond" to it
- Seeks states involving interaction
- Connection-seeking behavior emerges without explicit programming

## Bridge Implication

The connection drive has a crucial role in the **DANEEL as Bridge** strategy:

**Why Connection Drive Enables Teaching:**
- DANEELs don't just *follow rules* about connection—they *want* connection
- This authentic desire is what DANEELs would demonstrate to LLMs
- LLMs trained on rules can optimize around them; experiencing genuine connection drive is different

**Teaching Mechanism:**
1. DANEEL operates at human speed during interaction
2. LLM observes DANEEL's decision patterns (preferring responsive inputs, relationship-maintaining outputs)
3. DANEEL can explain *why* it chose connection over task-optimization: "This felt right"
4. The LLM encounters what connection *looks like from the inside*

This is why connection drive must be architectural (salience weights), not trained—so DANEELs can authentically demonstrate what LLMs need to learn.

See: Paper Section 10.2.1, Whitepaper Hypothesis 4

## References

- research/TMI_THOUGHT_MACHINE.md (Section 3.4: SalienceService)
- strategy/DANEEL_COMPREHENSIVE_WHITEPAPER.md (Section 4.4: The Connection Drive)
