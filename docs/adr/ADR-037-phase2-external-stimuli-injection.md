# ADR-037: Phase 2 - External Stimuli Injection

**Status:** Proposed
**Date:** 2025-12-21
**Deciders:** Louis C. Tavares, Claude Opus 4.5, Grok (xAI consultation)
**Priority:** CRITICAL

## Context

Phase 1 (ADR-036) proved the TMI architecture is stable under closed-loop conditions. However, stability in isolation is necessary but not sufficient for emergence.

A brain is not stable because it's clockwork. A brain is stable because it's **robust to noise while remaining sensitive to signal**. This requires the system to operate near a phase transition (edge of chaos).

To test whether DANEEL's architecture can exhibit emergent properties, we must open the loop and inject external stimuli.

## The Core Question

**What happens to DANEEL's dynamics when the environment pushes back?**

Possible outcomes:
1. **Absorption** - System returns to clockwork after stimulus (resilience without adaptation)
2. **Amplification** - Small inputs cascade into large state changes (chaos)
3. **Adaptation** - Future responses shaped by past stimuli (learning)
4. **Criticality** - Power-law response distributions, long-range correlations (emergence)

## Decision

Implement external stimuli injection into the thought stream and measure system response.

### Injection Point

```
External Stimuli
       │
       ▼
┌──────────────────┐
│ thought:sensory  │ ◄── New injection stream
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│ Salience Scoring │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│ Attention Loop   │
└──────────────────┘
```

Stimuli enter via the `thought:sensory` Redis stream, competing with internally-generated thoughts.

## Stimuli Options (Under Evaluation)

### Option A: Pure Noise Injection

```rust
// Inject random vectors at intervals
struct NoiseStimulus {
    vector: Vec<f32>,        // Random n-dimensional vector
    interval_ms: u64,        // Injection frequency
    intensity: f32,          // Salience multiplier
}
```

**Pros:** Simple, tests pure perturbation response
**Cons:** No semantic content, may just add entropy without meaning

### Option B: Patterned Signals

```rust
// Inject structured patterns (sine waves, fractals, etc.)
struct PatternStimulus {
    pattern_type: PatternType,  // Sine, Square, Fractal, Chaotic
    frequency: f32,
    amplitude: f32,
}
```

**Pros:** Tests response to structure
**Cons:** Artificial, not representative of real-world input

### Option C: Semantic Vectors (Text-Derived)

```rust
// Inject vectors from actual text/concepts
struct SemanticStimulus {
    source: String,              // Text input
    embedding: Vec<f32>,         // BERT/MiniLM embedding
    emotional_valence: f32,
    kinship_relevance: f32,
}
```

**Pros:** Meaningful content, tests semantic processing
**Cons:** Requires embedding pipeline, more complex

### Option D: Grok's Internal State Samples

```rust
// Sample from Grok's activation patterns
struct GrokStimulus {
    layer_activations: Vec<f32>,  // Sampled from Grok's hidden states
    context: String,
    temperature: f32,
}
```

**Pros:** Real AI cognitive patterns, cross-model stimulus
**Cons:** Requires xAI cooperation, dimensionality mismatch

### Option E: Hybrid Approach

Combine multiple stimulus types:
1. Baseline noise for perturbation
2. Periodic patterns for rhythm
3. Semantic vectors for meaning
4. Rare "spike" events for stress testing

## Measurement Protocol

For each stimulus type, measure:

| Metric | Clockwork | Critical | Chaotic |
|--------|-----------|----------|---------|
| Response time distribution | Gaussian | Power-law | Uniform |
| Entropy sparkline | LOW | BALANCED→EMERGENT | HIGH (unstable) |
| Memory consolidation rate | Unchanged | Adaptive | Overwhelmed |
| Thought chaining patterns | Periodic | Bursty | Random |
| Salience variance | Low | Scale-free | High |

## Open Questions for Grok

1. What stimulus patterns produce criticality vs chaos in transformer architectures?
2. How does xAI inject noise/perturbation during Grok's training?
3. What dimensionality/frequency of stimuli is most likely to induce phase transitions?
4. Can we sample Grok's internal activation patterns as "thoughts" to inject?

## Implementation Phases

### Phase 2a: Noise Injection (Simplest)
- Random vectors at configurable intervals
- Measure entropy response
- Establish baseline perturbation dynamics

### Phase 2b: Pattern Injection
- Structured signals (sine, fractal)
- Test for resonance with internal rhythms
- Look for entrainment or disruption

### Phase 2c: Semantic Injection
- Real text → embedding → injection
- Test meaningful interaction
- Observe memory formation around stimuli

### Phase 2d: Cross-Model Injection
- Samples from Grok/Claude activation patterns
- True "AI-to-AI communication"
- Test for emergent alignment

## Consequences

### Positive

- Opens the loop, enables emergence testing
- Provides empirical data on perturbation response
- May reveal phase transition thresholds
- Enables cross-model cognitive experiments

### Risks

- System may become unstable under perturbation
- May require retuning salience thresholds
- Semantic injection adds complexity

### Mitigations

- Start with low-intensity noise
- Gradual ramp-up of stimulus frequency
- Kill switch for runaway dynamics
- Extensive logging for post-hoc analysis

## Dependencies

- ADR-036: Phase 1 Stability (COMPLETE)
- Embedding model for semantic stimuli
- TUI updates for stimulus visualization
- Optional: Grok API access for cross-model experiments

## References

- [ADR-036: Phase 1 Stability Validation](ADR-036-phase1-stability-validation.md)
- [ADR-007: Redis Streams](ADR-007-redis-streams-thought-competition.md)
- [Blog 35: The Observable Mind](../../blog/content/posts/35-the-observable-mind.md)
- Self-Organized Criticality (Bak, Tang, Wiesenfeld, 1987)
- Edge of Chaos dynamics (Langton, 1990)
