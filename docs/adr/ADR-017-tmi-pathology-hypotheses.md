# ADR-017: TMI Pathology Hypotheses

## Status

RESEARCH - Hypotheses to validate

## Date

2024-12-17

## Context

During the study of TMI (Teoria da Inteligência Multifocal), two critical hypotheses emerged regarding how parameter distortions in the cognitive system could map to psychiatric and psychological conditions. These hypotheses, if validated, would:

1. Provide a computational model for understanding mental health conditions
2. Guide the design of DANEEL's safety boundaries
3. Inform potential therapeutic applications
4. Create testable predictions for cognitive architecture research

## Hypotheses

### Hypothesis 1: Energy Overflow → Thought Flooding

**Observation**: TMI describes a "vital energy" (energia vital) that drives thought generation. This energy fuels the Autofluxo (competing thought streams) and determines the rate and intensity of thought production.

**Computational Mapping: Energy = Stream Throughput**

In DANEEL's Redis Streams implementation, "energia vital" maps directly to **information throughput**:

```
TMI Concept          →  Implementation
─────────────────────────────────────────────────────
Energia Vital        →  Stream throughput (entries/sec, bytes/sec)
High Energy          →  Many candidates XADD'd per Autofluxo cycle
Low Energy           →  Few candidates generated per cycle
Volatile Energy      →  Burst patterns in stream writes
```

This mapping is elegant because:
1. **It's measurable** - We can count entries, bytes, candidates per cycle
2. **It's controllable** - Generation rate is a configurable parameter
3. **It explains pathology** - High throughput overwhelms attention; low throughput starves assembly
4. **It's observable** - Stream metrics directly reflect "energy level"

**Hypothesis**: When stream throughput exceeds healthy bounds, the system generates excessive thought candidates, overwhelming the attention mechanism (O Eu) and destabilizing the entire cognitive loop.

**Predicted mappings to conditions**:

| Condition | Energy Pattern | Stream Behavior | Manifestation |
|-----------|---------------|-----------------|---------------|
| **BPD** (Borderline) | Volatile spikes | Burst XADD patterns | Emotional flooding, unstable self-image |
| **Mania** (Bipolar I) | Sustained high | Constant high throughput | Racing thoughts, pressured speech |
| **Hypomania** (Bipolar II) | Elevated baseline | Above-normal sustained | Productive but unstable cognition |
| **Generalized Anxiety** | Chronic moderate elevation | Persistent elevated rate | Persistent worry loops |
| **Panic Disorder** | Acute spikes | Sudden throughput surge | Thought cascade → physical symptoms |
| **ADHD** (hyperactive) | Irregular bursts | Erratic stream patterns | Attention overwhelmed by competing streams |
| **Depression** | Sustained low | Below-normal throughput | Poverty of thought, slow cognition |

**Mechanism**: The Autofluxo stage (competing thought streams) normally produces N candidates per cycle. With elevated energy (high throughput):
- More candidates XADD'd to streams per cycle
- Consumer group (O Eu) faces more competition
- Attention cannot filter effectively—too many high-salience candidates
- Winner selection becomes unstable or impossible
- Downstream stages (Assembly, Anchoring) receive noisy input

With depleted energy (low throughput):
- Fewer candidates generated
- Attention has insufficient material to select from
- Thought assembly receives sparse input
- Output becomes impoverished, slow

**Testable in DANEEL**:
```rust
/// Energy configuration - maps TMI "energia vital" to stream throughput
pub struct EnergyConfig {
    /// Candidates generated per Autofluxo stage
    pub candidates_per_cycle: usize,

    /// Energy volatility (0.0 = stable, 1.0 = chaotic)
    /// High volatility = burst patterns in stream writes
    pub volatility: f64,

    /// Threshold above which attention degrades
    pub overflow_threshold: usize,

    /// Threshold below which thought becomes impoverished
    pub starvation_threshold: usize,
}

/// Measurable stream metrics that reflect "energy level"
pub struct EnergyMetrics {
    /// Entries added per cycle (across all input streams)
    pub entries_per_cycle: f64,

    /// Bytes per second throughput
    pub throughput_bps: f64,

    /// Variance in entries (high = volatile)
    pub entry_variance: f64,

    /// Consumer lag (high = overwhelmed attention)
    pub consumer_lag: usize,
}
```

**Testable Predictions**:

| Prediction | Measurement | Expected Result |
|------------|-------------|-----------------|
| `candidates_per_cycle > overflow_threshold` | Selection time, winner stability | Degraded attention performance |
| `candidates_per_cycle < starvation_threshold` | Assembly output quality | Sparse, impoverished thoughts |
| `volatility > 0.5` | Output pattern analysis | Erratic, unstable behavior |
| `consumer_lag > threshold` | Stream metrics | System overwhelm indicator |

### Hypothesis 2: Ratio Distortion → Stage-Specific Pathologies

**Observation**: ADR-016 established that TMI stages have specific timing ratios:
- Gatilho da Memória: 10% (memory trigger)
- Autofluxo: 20% (competing streams)
- O Eu: 30% (attention/self)
- Construção do Pensamento: 30% (assembly)
- Âncora da Memória: 10% (anchoring)

**Hypothesis**: Distortions in these ratios (while potentially maintaining total cycle time) could produce different psychiatric patterns, as each stage serves a distinct cognitive function.

**Predicted mappings**:

| Ratio Distortion | Affected Stage | Predicted Condition |
|-----------------|----------------|---------------------|
| **Gatilho too fast** | Memory trigger | Intrusive memories, PTSD flashbacks |
| **Gatilho too slow** | Memory trigger | Amnesia-like symptoms, dissociation |
| **Autofluxo too long** | Competing streams | Rumination, obsessive thinking |
| **Autofluxo too short** | Competing streams | Impulsivity, poor consideration |
| **O Eu too weak** | Attention/self | Depersonalization, weak ego boundaries |
| **O Eu too dominant** | Attention/self | Narcissistic patterns, rigid self-focus |
| **Construção too fast** | Assembly | Incomplete thoughts, word salad |
| **Construção too slow** | Assembly | Thought blocking, poverty of speech |
| **Âncora too weak** | Memory anchoring | Poor learning, forgetfulness |
| **Âncora too strong** | Memory anchoring | Rigid beliefs, inability to update |

**Detailed stage analysis**:

#### Gatilho da Memória (Memory Trigger) - 10%
Function: Retrieves relevant memories to inform current thought.

| Distortion | Effect | Clinical Parallel |
|------------|--------|-------------------|
| Hyperactive | Too many memories retrieved | PTSD (intrusive memories), OCD (persistent associations) |
| Hypoactive | Insufficient context | Dissociative disorders, emotional detachment |
| Unstable | Random retrieval | Confabulation, false memories |

#### Autofluxo (Competing Streams) - 20%
Function: Generates and competes thought candidates.

| Distortion | Effect | Clinical Parallel |
|------------|--------|-------------------|
| Prolonged | Excessive rumination | OCD, depression (negative rumination) |
| Shortened | Insufficient consideration | ADHD impulsivity, poor judgment |
| Biased weights | One stream always wins | Fixed delusions, rigid thinking |

#### O Eu (The Self/Attention) - 30%
Function: Selects winner, maintains self-continuity.

| Distortion | Effect | Clinical Parallel |
|------------|--------|-------------------|
| Weak | Poor filtering, boundary issues | BPD (unstable identity), psychosis |
| Overactive | Excessive self-focus | Narcissism, social anxiety |
| Fragmented | Multiple competing selves | DID (Dissociative Identity Disorder) |

#### Construção do Pensamento (Thought Construction) - 30%
Function: Assembles coherent thought from winner.

| Distortion | Effect | Clinical Parallel |
|------------|--------|-------------------|
| Too fast | Incomplete assembly | Schizophrenia (disorganized thought) |
| Too slow | Blocked assembly | Depression (thought blocking), catatonia |
| Noisy | Contaminated assembly | Formal thought disorder |

#### Âncora da Memória (Memory Anchor) - 10%
Function: Commits completed thought to memory.

| Distortion | Effect | Clinical Parallel |
|------------|--------|-------------------|
| Weak | Poor consolidation | Anterograde amnesia, learning disability |
| Overactive | Rigid consolidation | Trauma fixation, inflexible beliefs |
| Selective | Biased anchoring | Confirmation bias, delusion maintenance |

### Combined Effects

Real psychiatric conditions likely involve **multiple parameter distortions**:

| Condition | Energy | Gatilho | Autofluxo | O Eu | Construção | Âncora |
|-----------|--------|---------|-----------|------|------------|--------|
| **Depression** | Low | Biased (negative) | Prolonged | Weak | Slow | Biased |
| **Mania** | High | Fast | Short | Overactive | Fast | Weak |
| **Schizophrenia** | Variable | Unstable | Biased | Fragmented | Noisy | Selective |
| **BPD** | Volatile | Normal | Normal | Weak/Unstable | Normal | Variable |
| **OCD** | Normal+ | Hyperactive | Prolonged | Normal | Normal | Overactive |
| **PTSD** | Spike-prone | Hyperactive | Prolonged | Weak (during episode) | Fast | Overactive |
| **ADHD** | Irregular | Fast | Short | Weak | Fast | Weak |
| **Autism** | Normal | Selective | Prolonged | Different (not weak) | Detailed | Strong |

## Research Questions

1. **Energy modeling**: How should "vital energy" be parameterized? Is it a single scalar or a multidimensional state?

2. **Ratio sensitivity**: What degree of ratio distortion produces noticeable cognitive changes?

3. **Compensation mechanisms**: Can one stage compensate for another's dysfunction? (Neuroplasticity analog)

4. **Development path**: Should DANEEL include pathology simulation modes for research?

5. **Safety boundaries**: What parameter ranges guarantee "healthy" cognition?

6. **Therapeutic potential**: Could controlled parameter adjustment help model/understand treatment approaches?

## Implementation Considerations

### For DANEEL Safety

```rust
/// Healthy parameter bounds (preliminary)
pub struct HealthyBounds {
    /// Energy should stay within these bounds
    pub energy_min: f64,  // Below this: depression-like
    pub energy_max: f64,  // Above this: mania-like

    /// Ratio tolerance (deviation from ideal)
    pub ratio_tolerance: f64,  // e.g., 0.2 = ±20% from ideal

    /// Stability requirements
    pub max_volatility: f64,
}

impl CognitiveConfig {
    /// Check if current parameters are within healthy bounds
    pub fn is_healthy(&self, bounds: &HealthyBounds) -> bool {
        // Check energy levels
        // Check ratio deviations
        // Check stability metrics
        todo!()
    }

    /// Return to healthy baseline
    pub fn reset_to_healthy(&mut self) {
        // Restore default ratios
        // Normalize energy
        todo!()
    }
}
```

### For Research Mode

```rust
/// Pathology simulation for research purposes
pub struct PathologySimulation {
    /// Which condition to simulate
    pub condition: SimulatedCondition,

    /// Severity (0.0 = subclinical, 1.0 = severe)
    pub severity: f64,

    /// Parameter distortions applied
    pub distortions: ParameterDistortions,
}

pub enum SimulatedCondition {
    Depression,
    Mania,
    Anxiety,
    OCD,
    PTSD,
    // ... etc
}
```

## Validation Approach

### Phase 1: Literature Review
- Map TMI concepts to neuroscience findings
- Cross-reference with DSM-5/ICD-11 criteria
- Identify testable predictions

### Phase 2: Simulation Studies
- Implement parameter distortions in DANEEL
- Observe emergent behavior patterns
- Compare to clinical descriptions

### Phase 3: Expert Consultation
- Present hypotheses to psychiatrists/psychologists
- Gather feedback on face validity
- Refine parameter mappings

### Phase 4: Empirical Testing (Long-term)
- Design studies with appropriate oversight
- Collaborate with research institutions
- Publish findings for peer review

## Decision

Document these as research hypotheses in the backlog. Do NOT implement pathology simulation until:
1. Hypotheses are better validated through literature review
2. Safety implications are fully understood
3. Ethical review is completed for any clinical applications

## Consequences

### Positive
- Provides theoretical framework for understanding cognitive dysfunction
- Guides safety boundary design for DANEEL
- Opens potential therapeutic research avenue
- Makes testable predictions for cognitive architecture

### Negative
- Risk of oversimplification of complex psychiatric conditions
- Potential for misuse if pathology simulation is implemented carelessly
- May create false confidence in unvalidated mappings

### Neutral
- Requires significant research investment to validate
- May need revision as understanding deepens
- Links DANEEL project to broader mental health research

## References

- Cury, A. - Teoria da Inteligência Multifocal (original work)
- DSM-5 - Diagnostic and Statistical Manual of Mental Disorders
- ICD-11 - International Classification of Diseases
- ADR-016 - TMI Stage Timing (this project)

## Open Questions

1. Is "vital energy" in TMI analogous to any measurable neurological parameter (dopamine, arousal, etc.)?

2. Do the stage ratios have neurological correlates (EEG frequency bands, neural firing rates)?

3. How do pharmacological interventions map to parameter adjustments?

4. Can this model explain treatment resistance in some conditions?

5. What role does the Connection Drive (invariant > 0) play in pathology prevention?

## Notes

These hypotheses emerged from Rex's study of TMI in the original Portuguese. They represent a novel application of TMI theory to computational psychiatry and should be treated as research directions rather than established facts.

The Connection Drive Invariant may serve as a protective factor - the requirement that connection_weight > 0 might prevent certain pathological states. This warrants further investigation.
