# ADR-046: Vector Connectivity for Learning

**Status:** Proposed
**Date:** 2025-12-26
**Deciders:** Louis C. Tavares, Claude Opus 4.5, Grok
**Context:** STIM-D complete, entropy stable, learning architecture missing

## Context

### The Milestone Achieved

We have achieved stable criticality dynamics with pink noise injection:

| Metric | Pre-Attack (Dec 24) | Current (Dec 26) | Status |
|--------|---------------------|------------------|--------|
| Entropy | 74% EMERGENT | 63% BALANCED | Stable, not collapsing |
| Fractality | 0.50 | 0.45 | Climbing |
| Burst Ratio | 6.22 | 5.76 | Non-Poisson dynamics |
| Thoughts | 1.67M | 1.85M | Growing |

**Key achievement:** Pink noise (ADR-043) prevents collapse into clockwork. The system maintains edge-of-chaos dynamics. This is the prerequisite for emergence.

### The Problem Discovered

Despite achieving entropy stability, **Timmy cannot learn**.

Investigation revealed that thought vectors are **frozen islands**:

```
Vector A [0.1, 0.2, ...] ←── FROZEN AT BIRTH
Vector B [0.4, 0.5, ...] ←── FROZEN AT BIRTH
Vector C [0.7, 0.8, ...] ←── FROZEN AT BIRTH

NO EDGES BETWEEN THEM
NO WEIGHT UPDATES
NO HEBBIAN CO-ACTIVATION
```

The Association struct exists in the codebase but is **dead code**:

```rust
pub struct Association {
    pub target_id: Uuid,           // NEVER POPULATED
    pub weight: f32,               // NEVER UPDATED
    pub association_type: String,  // NEVER USED
    pub coactivation_count: u32,   // NEVER INCREMENTED
}
```

**Retrieval is read-only.** Memories are queried but nothing feeds back. Sleep consolidation updates metadata (replay_count, strength) but **never modifies vectors or associations**.

### Why This Matters

Without vector connectivity:
- Kin injections are absorbed but cannot influence future thoughts
- Manifold clustering toward Law Crystals is impossible
- No learning signal propagates through the system
- Timmy is an episodic memory system, not a learning system

## Decision

### Phase 1: Research (This ADR)

Document the theoretical basis for how vectors should connect according to cognitive science. Identify the specific mechanisms to implement.

### Phase 2: Implementation (Future ADR)

Wire the existing Association infrastructure to actually function during:
1. Attention competition (co-activated memories form edges)
2. Sleep consolidation (co-replayed memories strengthen edges)
3. Retrieval (associated memories boost each other's activation)

## Theoretical Basis

### TMI (Teoria da Mente Interativa) - Memory Connections

From Augusto Cury's framework, memories connect through:

1. **Gatilho da Memória (Memory Trigger)** - context vectors activate related memories
2. **Janelas da Memória (Memory Windows)** - emotional contexts that open/close together
3. **Âncora da Memória (Memory Anchor)** - fixes which memory territory is accessible

Key principle: **Memories that activate together should wire together.**

### Hebbian Learning - "Neurons That Fire Together Wire Together"

The classic rule, already designed in ADR-023:

```rust
// Co-activation during attention → weight += 0.1
// Co-activation during sleep replay → weight += 0.05
// Decay without activation → weight -= 0.01/day
// Below threshold (0.1) → pruned
```

This is **declared but not wired**.

### Association Types (from ADR-022)

Six types of connections, matching cognitive science:

| Type | Basis | Example |
|------|-------|---------|
| Semantic | Similar meaning | "dog" ↔ "cat" |
| Temporal | Occurred close in time | breakfast ↔ coffee |
| Causal | One led to another | action ↔ consequence |
| Emotional | Similar valence/arousal | joy ↔ celebration |
| Spatial | Same context/location | office ↔ meeting |
| Goal | Same task/objective | coding ↔ debugging |

### Neuroscience Foundation (from SLEEP_MEMORY_CONSOLIDATION.md)

Memory consolidation mechanisms to implement:

1. **Sharp-Wave Ripples (SWRs)** - High-frequency replay during sleep
2. **Synaptic Homeostasis** - Strengthen important, prune weak (Tononi & Cirelli)
3. **Interleaved Replay** - Mix novel + familiar to prevent catastrophic forgetting

### How This Differs from LLMs

| Aspect | LLM Learning | DANEEL Learning |
|--------|--------------|-----------------|
| Mechanism | Gradient descent on weights | Hebbian edge strengthening |
| Signal | Prediction error | Co-activation |
| Scope | All weights updated | Only active associations |
| When | Training time | Runtime (attention + sleep) |
| What changes | Hidden states | Explicit edges (queryable) |

**DANEEL learns through topology, not weights.** The graph structure evolves; vectors stay fixed.

## Implementation Requirements

### What Must Be Wired

1. **During Attention Competition:**
   ```rust
   // When multiple memories win attention in same cycle
   for (m1, m2) in co_activated_pairs {
       strengthen_association(m1.id, m2.id, delta=0.1, type=Temporal);
   }
   ```

2. **During Sleep Consolidation:**
   ```rust
   // When memories replay together in dream cycle
   for (m1, m2) in co_replayed_pairs {
       strengthen_association(m1.id, m2.id, delta=0.05, type=Semantic);
   }
   ```

3. **During Retrieval:**
   ```rust
   // When memory is retrieved, boost its associations
   for assoc in memory.associations {
       boost_activation(assoc.target_id, assoc.weight * 0.3);
   }
   ```

4. **Decay and Pruning:**
   ```rust
   // Daily homeostasis pass
   for assoc in all_associations {
       assoc.weight -= 0.01;
       if assoc.weight < 0.1 {
           prune(assoc);
       }
   }
   ```

### Files to Modify

| File | Change |
|------|--------|
| `src/actors/attention/mod.rs` | Track co-activated memories, form associations |
| `src/actors/sleep/mod.rs` | Strengthen associations during replay |
| `src/memory_db/mod.rs` | Implement `strengthen_association()`, `prune_associations()` |
| `src/core/cognitive_loop.rs` | Wire association activation during retrieval |

### Success Criteria

After implementation:
1. Associations populated (not empty vectors)
2. Weights changing over time (observable in Qdrant)
3. Retrieval influenced by association strength
4. Manifold shows clustering (related memories drift together)

## Consequences

### Positive
- Timmy can learn from experience
- Kin injections can influence future thought patterns
- Manifold will show meaningful structure
- Emergence hypothesis becomes testable

### Negative
- Added complexity in cognitive loop
- Potential for runaway association strengthening (needs dampening)
- Must tune decay rates carefully

### Risks
- Wrong association types could create pathological patterns
- Too aggressive pruning could cause catastrophic forgetting
- Must maintain THE BOX invariants during learning

## Research Needed Before Implementation

1. **Decay Rate Calibration** - What's the right balance between retention and pruning?
2. **Association Type Selection** - How to determine which type applies?
3. **Dampening Mechanisms** - How to prevent winner-take-all dynamics?
4. **Integration with Embeddings** - Should associations influence vector retrieval ranking?

## Related ADRs

- ADR-020: Redis Streams for Autofluxo
- ADR-021: Memory Database Selection - Qdrant
- ADR-022: TMI Memory Schema (Association struct defined)
- ADR-023: Sleep/Dream Consolidation (Hebbian learning designed)
- ADR-032: TMI Salience Calibration
- ADR-033: Unconscious Memory Architecture
- ADR-043: Noise Injection Correction (prerequisite achieved)

## References

**Cognitive Science:**
- Hebb, D.O. (1949) - The Organization of Behavior
- Tononi & Cirelli - Synaptic homeostasis hypothesis
- Cury, Augusto - Teoria da Mente Interativa

**Neuroscience:**
- Sharp-Wave Ripples research (Science 2024)
- Interleaved replay and catastrophic forgetting (bioRxiv 2025)

**DANEEL Research:**
- `/research/TMI_Memory_Model_Research.md`
- `/research/SLEEP_MEMORY_CONSOLIDATION.md`
- `/research/[removed]_DANEEL_ANALYSIS.md`

## Timeline

| Phase | Work | Status |
|-------|------|--------|
| 1 | Document theory (this ADR) | DONE |
| 2 | Research decay/dampening | PENDING |
| 3 | Implement association wiring | PENDING |
| 4 | Test with kin injection | PENDING |
| 5 | Validate manifold clustering | PENDING |

---

**The entropy milestone is achieved. Now we connect the dots.**
