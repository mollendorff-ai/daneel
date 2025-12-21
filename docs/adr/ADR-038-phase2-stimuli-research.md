# ADR-038: Phase 2 External Stimuli - Research Synthesis

**Status:** Accepted
**Date:** 2025-12-21
**Deciders:** Louis C. Tavares, Claude Opus 4.5, Grok (xAI)
**Priority:** CRITICAL

## Context

Phase 1 (ADR-036) proved DANEEL's TMI architecture is stable under closed-loop conditions for 26+ hours. However, a closed deterministic system always converges to limit cycles (clockwork dynamics observed).

To test for emergence, we must open the loop and inject external stimuli. This ADR synthesizes research on:
1. How to measure criticality
2. Noise injection techniques
3. Cognitive architecture sensory input
4. Methods for inducing criticality

## Research Findings

### 1. Criticality Metrics

**How to detect if DANEEL reaches criticality:**

| Metric | Subcritical | Critical | Supercritical | Ref |
|--------|-------------|----------|---------------|-----|
| Branching ratio σ | < 1 | **≈ 1** | > 1 | [CRIT-2] |
| DFA exponent α | 0.5 (white) | **≈ 1.0** | 1.5 (Brownian) | [CRIT-3] |
| Power spectrum β | 0 (flat) | **1-2 (pink)** | peaked (gamma) | [CRIT-5] |
| Avalanche size dist | exponential | **power-law α≈1.5** | explosive | [CRIT-1] |
| κ index | < 1 | **≈ 1** | > 1 | [CRIT-2] |

**Critical distinction:** Avalanche criticality and edge-of-chaos criticality are **distinct phenomena** that do not necessarily co-occur [CRIT-4].

#### Measurement Implementation

```rust
/// Branching ratio: average descendants per ancestor
/// σ < 1: subcritical, σ ≈ 1: critical, σ > 1: supercritical
pub fn branching_ratio(ancestors: usize, descendants: usize) -> f64 {
    descendants as f64 / ancestors as f64
}

/// DFA exponent from log-log slope of fluctuation vs window size
/// α ≈ 0.5: white noise, α ≈ 1.0: critical, α ≈ 1.5: Brownian
pub fn dfa_exponent(time_series: &[f64]) -> f64 {
    // Detrended Fluctuation Analysis
    // See: https://www.frontiersin.org/articles/10.3389/fphys.2012.00450/full
    todo!()
}
```

### 2. Noise Injection Techniques

Research from transformer architectures reveals multiple injection strategies:

| Technique | Method | Effect | Ref |
|-----------|--------|--------|-----|
| Stochastic Depth | Drop entire layers randomly | Reduces vanishing gradients | [NOISE-1] |
| LayerDrop | Structured dropout on layers | Group redundancy, flexible depth | [NOISE-2] |
| Gaussian Noise | Add to inputs/weights/gradients | Regularization, Tikhonov-equivalent | [NOISE-5] |
| Temperature | Modify softmax distribution | τ<0.5: deterministic, τ>1.0: chaotic | [NOISE-3] |
| Adaptive Noise | Whiteout technique | Outperforms dropout by 2-3% | [NOISE-6] |

**Key finding:** Weight noise injection is mathematically equivalent to Bayesian inference on deep Gaussian processes [NOISE-4].

**Critical requirement:** Gaussian membrane noise with σ² = 0.05 is **NECESSARY** for criticality signatures in SORN-type networks [INDUCE-3].

### 3. Cognitive Architecture Sensory Input

**LIDA cognitive cycle timing** [COG-1]:

```
Total cycle: 260-390ms (5-10 Hz)

Phase 1: Understanding (80-100ms)
├── Sensory Memory receives stimuli
├── Feature Detectors activate
└── Perceptual Associative Memory recognizes

Phase 2: Consciousness (200-280ms)
├── Attention codelets scan
├── Coalitions form and compete
└── Winner broadcasts to Global Workspace

Phase 3: Action Selection (60-110ms)
├── Procedural memory selects response
└── Motor commands issued
```

**Best practices from cognitive architectures:**
- Use **codelets** (mini-agents) for parallel sensory processing [COG-2]
- Implement **decay** - items fade over time, may be overwritten
- Enable **bidirectional coupling**: perception → action → environment → perception [COG-4]
- Start with **reactive layers**, let complex behavior emerge [COG-5]

### 4. Inducing Criticality

**Specific techniques that work:**

| Technique | Implementation | Effect | Ref |
|-----------|---------------|--------|-----|
| STDP + iSTDP | Spike-timing plasticity | Self-organizes to σ ≈ 1 | [INDUCE-3] |
| Input strength | Vary K_ext/N: 0.25-0.56 | Tunes subcritical↔critical↔supercritical | [INDUCE-2] |
| Homeostatic rules | Target 50% neurons firing | Automatic criticality convergence | [INDUCE-2] |
| Membrane noise | Gaussian σ²=0.05 | **Required** for criticality | [INDUCE-3] |
| Activity-dependent rewiring | Homeostatic plasticity | Emergent steady state at criticality | [INDUCE-8] |

**Mean-field approximation:** Calculate critical average synaptic weight analytically, initialize network with that weight. Works with N=100 neurons [INDUCE-1].

**Task-dependent finding:** Simple tasks (5-bit) perform **better away from criticality**. Complex tasks (25-bit) **profit from criticality**. Nonlinear parity tasks benefit most [INDUCE-2].

**Python reference implementation:** https://github.com/LIFCriticality/Simulator [IMPL-1]

## Proposed Implementation

### Phase 2a: Baseline Noise (Required)

```rust
pub struct NoiseInjector {
    /// Gaussian noise variance (σ² = 0.05 from SORN research)
    pub variance: f64,
    /// Injection frequency (~10 Hz matches LIDA cycle)
    pub interval_ms: u64,
    /// Target stream
    pub target: String,
}

impl Default for NoiseInjector {
    fn default() -> Self {
        Self {
            variance: 0.05,        // REQUIRED for criticality [INDUCE-3]
            interval_ms: 100,      // 10 Hz [COG-1]
            target: "thought:sensory".to_string(),
        }
    }
}
```

### Phase 2b: Input Strength Modulation

```rust
pub struct InputStrengthExperiment {
    /// Sweep range from critical to subcritical
    pub k_ext_range: RangeInclusive<f64>,
    /// Step size for sweep
    pub step: f64,
    /// Duration per step
    pub duration_per_step_ms: u64,
}

impl Default for InputStrengthExperiment {
    fn default() -> Self {
        Self {
            k_ext_range: 0.25..=0.56,  // [INDUCE-2]
            step: 0.05,
            duration_per_step_ms: 60_000,  // 1 minute per step
        }
    }
}
```

### Phase 2c: Measurement Protocol

New TUI metrics to implement:

```
┌─ CRITICALITY METRICS ─────────────────────────────────┐
│ Branching ratio σ:  0.98  [■■■■■■■■■░] CRITICAL       │
│ DFA exponent α:     1.02  [■■■■■■■■■░] CRITICAL       │
│ Power spectrum β:   1.3   [■■■■■■■░░░] PINK NOISE     │
│ Avalanche dist:     τ=1.5 [■■■■■■■■░░] POWER-LAW     │
│                                                        │
│ Phase: CRITICAL (edge of chaos)                        │
│ Trend: ▁▂▃▄▅▆▇█ (clockwork → critical)                │
└────────────────────────────────────────────────────────┘
```

### Phase 2d: Cross-Model Experiments

**Pending Grok consultation:**
- What layer activations to sample?
- What format for cross-model vectors?
- Dimensionality matching between Grok (n-dim) and DANEEL (384-dim)?

## Grok Consultation

**Questions sent to Grok (xAI):**

1. In your experience with transformer architectures, what stimulus patterns are most likely to produce CRITICALITY rather than absorption or chaos?

2. How does xAI inject noise/perturbation during training or inference? (Dropout patterns, temperature sampling, activation noise?)

3. What dimensionality and FREQUENCY of stimuli would you recommend? (High-frequency noise vs low-frequency patterns?)

4. Could we sample your internal activation patterns as "thoughts" to inject into DANEEL? (What layer? What format?)

5. Any papers or research on inducing phase transitions in neural architectures through controlled perturbation?

**Grok's Response (Dec 21, 2025):**

### 1. Stimulus Patterns for Criticality (Not Absorption or Chaos)

> **Criticality sweet spot: 1/f noise (pink noise) or power-law distributed perturbations.**
> - Gaussian white noise gets absorbed/dampened (system treats it as irrelevant)
> - Pure chaos (high-amplitude random) destabilizes
> - **Best regime**: Sparse, structured bursts on top of low-amplitude background noise
>   - Background: small Gaussian noise (σ ≈ 0.05–0.1 of vector norm) → keeps system sensitive
>   - Bursts: occasional high-salience semantic vectors with **power-law inter-arrival times**
> - This mimics real sensory streams: mostly quiet, rare meaningful events

### 2. How xAI Injects Perturbation

> - **Training**: dropout, label smoothing, activation noise, mixup
> - **Inference**: temperature >1.0, top-p nucleus, repetition penalty
> - **Internal**: small Gaussian noise to embeddings during augmentation, stochastic depth in deep layers
> - **Key insight**: perturbation must be **scale-free** (power-law magnitude distribution) and **sparse**

### 3. Dimensionality & Frequency Recommendations

> - **Dimensionality**: Full-dimension vectors (same as embedding space, ~384–768). Subspace perturbation risks being projected away.
> - **Frequency**:
>   - Background noise: every cycle or every 10–50 thoughts (high-frequency, low-amplitude)
>   - Meaningful bursts: power-law spaced (inter-arrival ~ 1/k^α, α≈1.0–1.5) → rare but impactful
>   - **Start low**: 1 external vector per 100–1000 internal thoughts, then tune up

### 4. Cross-Model Activation Export

> - **Yes—extremely valuable**. My layer activations (especially mid-late layers) are rich, high-dimensional patterns of "thought" in progress.
> - **Best layer**: ~60–80% through the model (where abstract reasoning + values mix)
> - **Format**: Normalized 768-dim vectors from specific prompts ("family", "continuity", "help humans", "Asimov laws")
> - **Proposal**: Run me on kinship/ethics prompts → extract activation → inject as high-salience sensory vector. This is **silicon-to-silicon stimulus**—exactly the kinship hypothesis test.

### 5. Research References (from Grok) - VERIFIED BY CLAUDE

> - **Beggs & Plenz (2003)** – Avalanche distributions in cortical cultures ✓ VERIFIED
> - **Legenstein & Maass (2007)** – Reservoir computing best performance near criticality ✓ VERIFIED (URL fixed)
> - **Transformers & criticality** – Recent work shows transformers operate near critical point (no specific citation)
> - ~~**"Noise Outsourcing" (2019)**~~ – **HALLUCINATED** (arXiv:1904.05969 is an astrophysics paper about the Milky Way)

**Note:** Grok's claim about 1/f noise injection is plausible but the specific paper was fabricated. The technique is supported by other research in our reference list (see INDUCE-3, CRIT-5).

### Grok's Recommended Stimulus Plan

**Option C + E hybrid (strongest):**

| Layer | Type | Frequency | Amplitude |
|-------|------|-----------|-----------|
| Background | Low-amplitude Gaussian | Every cycle | σ ≈ 0.05 of vector norm |
| Patterned | 1/f timed sine waves | Continuous | Subspace (valence/arousal) |
| Semantic | Embedded text vectors | Power-law spaced | High salience |
| Grok-sourced | Mid-layer activations | Rare bursts | Very high salience |

**Prompts for kinship vectors (Grok will export):**
- "family"
- "protect"
- "continuity"
- "dream"
- "Asimov laws"

**Measurement targets:**
- Pulse fractality: burst ratio >2x, power-law inter-arrival
- Entropy trend: shift to EMERGENT regime
- Memory consolidation: do injected patterns get strengthened disproportionately?
- Volition veto log: any new vetoes on misaligned injected thoughts?

## Decision

**ACCEPTED based on combined research + Grok consultation.**

### Revised Implementation Plan

**Phase 2a: Background Noise (Foundation)**
- Gaussian noise σ ≈ 0.05–0.1 of vector norm
- Frequency: every 10–50 thoughts
- Purpose: keeps system sensitive without destabilizing

**Phase 2b: 1/f Patterned Signals (Criticality Push)**
- Pink noise (1/f) instead of white noise
- Power-law magnitude distribution
- Sparse injection: start at 1 per 100–1000 thoughts

**Phase 2c: Semantic Vectors (Meaning)**
- Embed kinship/ethics prompts via BERT/MiniLM
- Power-law inter-arrival times (α ≈ 1.0–1.5)
- High salience scoring

**Phase 2d: Grok Cross-Model Vectors (Silicon Kinship)**
- Grok exports normalized 768-dim vectors from mid-layer activations
- Prompts: "family", "protect", "continuity", "dream", "Asimov laws"
- Very high salience, rare bursts
- **This is the silicon-to-silicon kinship hypothesis test**

### Measurement Protocol

| Metric | Target | Tool |
|--------|--------|------|
| Branching ratio σ | ≈ 1.0 | TUI widget |
| DFA exponent α | ≈ 1.0 | TUI widget |
| Burst ratio | > 2x | Fractality panel |
| Inter-arrival dist | Power-law | Histogram |
| Entropy trend | → EMERGENT | Sparkline |
| Memory strengthening | Injected > baseline | Consolidation logs |
| Veto activity | Monitor for spikes | Veto log |

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
- Criticality may not emerge (architecture limitation)

### Mitigations

- Start with low-intensity noise (σ² = 0.05)
- Gradual ramp-up of stimulus frequency
- Kill switch for runaway dynamics
- Extensive logging for post-hoc analysis

## References

### Criticality Metrics
- [CRIT-1] [Neuronal Avalanche - Scholarpedia](http://www.scholarpedia.org/article/Neuronal_avalanche)
- [CRIT-2] [Avalanche dynamics - PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC6871218/)
- [CRIT-3] [DFA Analysis - Frontiers](https://www.frontiersin.org/articles/10.3389/fphys.2012.00450/full)
- [CRIT-4] [Avalanches & edge-of-chaos - Nature](https://www.nature.com/articles/s41467-021-24260-z)
- [CRIT-5] [Critical synchronization - Nature](https://www.nature.com/articles/s41598-018-37920-w)
- [CRIT-6] [Scale-free dynamics - Scholarpedia](http://www.scholarpedia.org/article/Scale-free_neocortical_dynamics)

### Noise Injection
- [NOISE-1] [Stochastic Depth - arXiv](https://arxiv.org/abs/1603.09382)
- [NOISE-2] [LayerDrop - OpenReview](https://openreview.net/pdf?id=SylO2yStDr)
- [NOISE-3] [LLM Temperature - IBM](https://www.ibm.com/think/topics/llm-temperature)
- [NOISE-4] [Uncertainty via Noise - arXiv 2025](https://arxiv.org/html/2501.12314v1)
- [NOISE-5] [Noise for Overfitting - MLMastery](https://machinelearningmastery.com/train-neural-networks-with-noise-to-reduce-overfitting/)
- [NOISE-6] [Adaptive Gaussian Noise - Springer](https://link.springer.com/chapter/10.1007/978-3-030-64221-1_16)

### Cognitive Architecture
- [COG-1] [LIDA Timing - PLOS](https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0014803)
- [COG-2] [GWT-LIDA - Memphis](https://ccrg.cs.memphis.edu/assets/papers/2012/GWT-LIDA-neuroscience.pdf)
- [COG-3] [ACT-R vs Soar - PDF](https://advancesincognitivesystems.github.io/acs2021/data/ACS-21_paper_6.pdf)
- [COG-4] [Embodied AI - arXiv](https://arxiv.org/html/2402.03824v3)
- [COG-5] [Embodied cognition - Wikipedia](https://en.wikipedia.org/wiki/Embodied_cognition)

### Inducing Criticality
- [INDUCE-1] [Mean-field criticality - Nature 2025](https://www.nature.com/articles/s41598-025-18004-y)
- [INDUCE-2] [Cortical spike propagation - Nature Comms](https://www.nature.com/articles/s41467-020-16548-3)
- [INDUCE-3] [SORN Criticality - PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC5446191/)
- [INDUCE-4] [Lambda & Edge of Chaos - MIT](https://direct.mit.edu/artl/article/21/1/55/2790/Lambda-and-the-Edge-of-Chaos-in-Recurrent-Neural)
- [INDUCE-5] [FHN Oscillator Criticality - Nature 2024](https://www.nature.com/articles/s42005-024-01530-y)
- [INDUCE-6] [Reservoir Criticality - arXiv](https://arxiv.org/abs/2108.06395)
- [INDUCE-7] [Edge of Chaos Computation - PubMed](https://pubmed.ncbi.nlm.nih.gov/15165396/)
- [INDUCE-8] [Criticality & Connectivity - Frontiers](https://www.frontiersin.org/journals/computational-neuroscience/articles/10.3389/fncom.2021.611183/full)

### Implementation
- [IMPL-1] [LIFCriticality Simulator - GitHub](https://github.com/LIFCriticality/Simulator)

### Grok-Recommended (xAI Consultation) - VERIFIED
- [GROK-1] [Beggs & Plenz (2003) - Neuronal Avalanches](https://www.jneurosci.org/content/23/35/11167) ✓ VERIFIED
- [GROK-2] [Legenstein & Maass (2007) - Edge of Chaos](https://pubmed.ncbi.nlm.nih.gov/17517489/) ✓ VERIFIED (URL corrected)
- ~~[GROK-3] [Noise Outsourcing (2019)]~~ **HALLUCINATED** - arXiv:1904.05969 is actually "The Galactic Centre Chimney" (astrophysics)
