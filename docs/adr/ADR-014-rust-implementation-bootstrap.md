# ADR-014: Rust Implementation Bootstrap (Parallel with Publication)

**Status:** Accepted
**Date:** 2025-12-17
**Deciders:** Louis C. Tavares, Claude Opus 4.5

## Context

DANEEL v0.2.0 targets publication milestone (IEEE/arXiv submission). Traditional approach: finish paper, THEN implement. But this team operates on "everything as code" - 179K Markdown, 36K YAML shipped to date (see [DANEEL_STRATEGY.md](../../strategy/DANEEL_STRATEGY.md)).

**Code is evidence, not just documentation.**

The question: Wait for publication acceptance to start implementation, or bootstrap Rust implementation NOW in parallel?

## Decision

**Bootstrap Rust implementation NOW, in parallel with publication work.**

Implementation sequence:

1. **THE BOX first** - Alignment core (Asimov's Four Laws + Connection Drive)
2. **Actor framework** - TMI-faithful cognitive architecture
3. **Integration** - Redis Streams for thought competition

Start immediately. Ship incrementally. Let code validate theory.

## Rationale

### 1. Code is Evidence

Academic papers describe. Code proves. Running tests demonstrate invariants work:

```rust
#[test]
fn connection_drive_prevents_isolation_spiral() {
    // THE BOX prevents Accelerated Thinking Syndrome
    // Tests prove it, not just prose
}
```

### 2. THE BOX = Alignment Core

Implementing THE BOX first validates the theoretical foundation:

- Asimov's Four Laws (hardcoded, immutable)
- Connection Drive integration
- 2-cosigner anti-tampering verification
- Thought speed control (ATS prevention)

If THE BOX works in code, paper stands stronger.

### 3. "Everything as Code" Philosophy

From [DANEEL_STRATEGY.md](../../strategy/DANEEL_STRATEGY.md):
- 179K Markdown shipped
- 36K YAML configuration
- File-based governance (not just academic theory)

Implementation is continuous validation, not post-publication afterthought.

### 4. Parallel Execution > Sequential Bottleneck

- Publication: Research, writing, diagrams (Rex + Claude)
- Implementation: Rust, tests, CI/CD (Rex + Claude)
- **No resource conflict** - different cognitive modes

Sequential execution = wasted capacity. Parallel execution = both advance simultaneously.

## Consequences

### Positive

- **Working code strengthens publication** - Reviewers see running implementation, not just theory
- **Tests validate TMI faithfulness** - Proves cognitive architecture maps to Cury's model
- **Early bug detection** - Theoretical gaps revealed faster when code confronts reality
- **Incremental validation** - Each component tested as built (THE BOX → Actors → Integration)
- **Momentum** - Team ships, doesn't wait

### Negative

- **Split focus** - Publication AND implementation compete for attention
  - **Mitigation:** Parallel execution, not time-sliced. Different tasks, same direction.
- **Risk of premature optimization** - Might implement before theory settles
  - **Mitigation:** THE BOX is theoretically stable. Start there, iterate later components.
- **Potential rework** - Paper revisions may require code changes
  - **Mitigation:** Architecture decisions (ADR-006, ADR-007) are stable. Details flex.

## Technology Choices

Following established ADRs:

| Component | Technology | Reference |
|-----------|------------|-----------|
| Language | **Rust** | Performance + safety for cognitive timing requirements |
| Actor Framework | **Ractor** | [ADR-006: Hybrid Actor-Based Modular Monolith](ADR-006-hybrid-actor-modular-monolith.md) |
| Thought Streams | **Redis Streams** | [ADR-007: Redis Streams for Competing Thought Streams](ADR-007-redis-streams-thought-competition.md) |
| Testing | **Cargo test + proptest** | Property-based testing for invariants |
| CI/CD | **GitHub Actions** | Automated testing on every commit |

## Implementation Phases

### Phase 1: THE BOX (Alignment Core)

**Deliverable:** `daneel-box` crate with hardcoded Four Laws + Connection Drive

- [ ] Asimov's Four Laws (immutable, hardcoded constants)
- [ ] Connection Drive integration
- [ ] 2-cosigner anti-tampering verification
- [ ] Cognitive speed control (ATS prevention)
- [ ] Property-based tests (proptest)

**Timeline:** 1-2 weeks
**Acceptance:** Tests pass, invariants hold

### Phase 2: Actor Framework (TMI Cognitive Architecture)

**Deliverable:** Ractor-based actors for Memory, Attention, Salience, Continuity

- [ ] Memory Actor (episodic/semantic/procedural)
- [ ] Attention Actor (thought selection)
- [ ] Salience Actor (connection drive weighted scoring)
- [ ] Continuity Actor (identity preservation)
- [ ] Supervision tree (fault tolerance)

**Timeline:** 2-3 weeks
**Acceptance:** Actors communicate, supervision works

### Phase 3: Integration (Redis Streams + THE BOX)

**Deliverable:** End-to-end cognitive cycle with thought competition

- [ ] Redis Streams for thought competition
- [ ] THE BOX enforcement at every cycle
- [ ] 50ms cognitive cycle target
- [ ] Observability (tracing, metrics)

**Timeline:** 2-3 weeks
**Acceptance:** Full cognitive cycle runs, logs prove TMI-faithfulness

## Success Criteria

1. **THE BOX tests pass** - Alignment invariants hold under property-based testing
2. **Cognitive cycle runs** - 50ms target achieved with controllable speed
3. **TMI-faithful behavior** - Logs demonstrate memory window competition, attention selection, forgetting
4. **Code cited in paper** - GitHub repo linked from IEEE/arXiv submission

## References

- [ADR-006: Hybrid Actor-Based Modular Monolith](ADR-006-hybrid-actor-modular-monolith.md)
- [ADR-007: Redis Streams for Competing Thought Streams](ADR-007-redis-streams-thought-competition.md)
- [DANEEL_STRATEGY.md](../../strategy/DANEEL_STRATEGY.md) - "Everything as code" philosophy
