# ADR-041: Entropy Calculation Standardization

## Status

ACCEPTED

## Date

2025-12-24

## Context

DANEEL has two implementations of cognitive entropy calculation that produce different values:

**TUI Implementation** (`daneel/src/tui/app.rs:581-607`):
- Uses a single pre-computed `salience` value (f32) per thought
- Bins into 10 buckets (0.0-1.0 range)
- Computes Shannon entropy: H = -Σ(p_i * log2(p_i))

**Web Implementation** (`daneel/src/api/handlers.rs:507-567`):
- Extracts only `importance` field from salience JSON object
- Same 10-bucket binning and Shannon entropy formula
- Ignores: novelty, relevance, valence, arousal, connection_relevance

Both implementations claim to measure "cognitive emergence vs clockwork" but differ in:
1. Data source (in-memory buffer vs Redis stream)
2. Field used (composite `salience` vs single `importance`)
3. Window size (all buffered vs last 100)

## Research Findings

Four parallel research agents investigated scientific literature using the project's `references.yaml` as source material.

### 1. Entropic Brain Theory (Carhart-Harris, 2014)

**Source**: DRUG-1b, PMC3909994

Key finding: "Brain entropy measures the UNCERTAINTY, DISORDER, and INFORMATIONAL RICHNESS of brain states across MULTIPLE signal types and networks."

- Psychedelic studies compute entropy from "within-network synchrony variance" and "connectivity motifs" across 64 possible patterns
- Modern neuroscience uses **Multivariate Multi-Scale Entropy** algorithms
- Collapsing to single dimension loses critical information

### 2. Global Workspace Theory (GWT-1, GWT-2)

**Source**: PMC8770991, Wikipedia GWT

Key finding: "Salience is DEFINITIVELY MULTI-DIMENSIONAL."

Neuroscience identifies distinct salience components:
- **Sensory salience**: Physical features, contrast
- **Motivational salience**: Reward/punishment value
- **Cognitive salience**: Task relevance, goals
- **Emotional salience**: Valence × arousal
- **Novelty**: Surprise factor
- **Temporal salience**: Timing factors

The Web's 6-field salience structure (importance, novelty, relevance, valence, arousal, connection_relevance) aligns with neuroscience. Using only `importance` discards 5/6 of the information.

### 3. TMI (Multifocal Intelligence Theory)

**Source**: TMI-NEUTRAL-1, TMI-NEUTRAL-2

Key finding: "EMOTIONAL INTENSITY—NOT IMPORTANCE ALONE—determines memory registration strength."

Augusto Cury's framework explicitly states:
> "The most painful or pleasurable experiences are registered with greater intensity"
> "Experiences with high emotional commitment receive privileged registration"

TMI Formula (qualitative):
```
memory_salience = f(emotional_intensity)
emotional_intensity = |valence| × arousal × tension
```

- `importance` alone is **never mentioned** as primary factor
- 90%+ of memories are "neutral windows" with minimal emotional charge
- "Killer windows" form from high |valence| × arousal experiences

### 4. Information Theory in Cognitive Science

**Source**: BOTTLE-1, BOTTLE-2

Key finding: "Shannon entropy is correct, but should measure CATEGORICAL COGNITIVE STATES, not intensity gradations."

- Human conscious thought: ~10 bits/sec (from 1 billion bits/sec sensory input)
- Brain uses 3-5 stable cognitive states (Triple Network Model: DMN, Salience Network, CEN)
- 10 intensity bins is biologically implausible; 3-5 categorical states is more accurate

## Decision

### Primary Change: Multi-Dimensional Entropy

**BOTH implementations must compute entropy over the FULL 6-dimensional salience space.**

The salience object exists because cognition is multi-dimensional:
```rust
pub struct SalienceScore {
    pub importance: f32,        // Cognitive dimension
    pub novelty: f32,           // Attentional dimension
    pub relevance: f32,         // Contextual dimension
    pub valence: f32,           // Emotional polarity (-1 to 1)
    pub arousal: f32,           // Emotional intensity (0 to 1)
    pub connection_relevance: f32, // Social dimension
}
```

### Recommended Algorithm

**Option A: Weighted Composite (TMI-aligned)**

Per TMI research, emotional intensity is primary:
```rust
fn compute_tmi_salience(s: &SalienceScore) -> f32 {
    let emotional_intensity = s.valence.abs() * s.arousal;
    let cognitive_weight = s.importance * 0.3 + s.relevance * 0.2;
    let novelty_weight = s.novelty * 0.2;
    let connection_weight = s.connection_relevance * 0.1;

    // Emotional intensity weighted 2x
    (emotional_intensity * 0.4 + cognitive_weight + novelty_weight + connection_weight)
        .clamp(0.0, 1.0)
}
```

Then bin this composite value and compute Shannon entropy.

**Option B: Multi-Dimensional Binning (Neuroscience-aligned)**

Treat each thought as a point in 6D space:
```rust
fn compute_multidimensional_entropy(thoughts: &[SalienceScore]) -> f32 {
    // Discretize each dimension into 3 levels: Low/Medium/High
    // Creates 3^6 = 729 possible bins (most empty)
    // Count occupied bin frequencies
    // Apply Shannon entropy to non-empty bin distribution
}
```

**Recommendation**: Use Option A for computational simplicity while maintaining TMI alignment. The weighted composite captures the research finding that emotional_intensity (|valence| × arousal) is the primary factor.

### Secondary Change: Binning Strategy

Reduce bins from 10 to **5 categorical levels**:
- 0.0-0.2: MINIMAL (neutral windows, background processing)
- 0.2-0.4: LOW (routine cognition)
- 0.4-0.6: MODERATE (active processing)
- 0.6-0.8: HIGH (focused attention)
- 0.8-1.0: INTENSE (killer window formation)

This aligns with neuroscience finding of 3-5 stable cognitive states.

### Tertiary Change: Naming

Rename from generic "entropy" to **"Cognitive Diversity Index"** to clarify what's being measured: the variety of cognitive states in the thought stream, not raw Shannon bits.

## Consequences

### Positive
- Single source of truth for entropy calculation
- TMI-aligned weighting emphasizes emotional intensity
- Scientifically grounded in entropic brain theory
- Reduced bins match biological cognitive states

### Negative
- Breaking change to both TUI and Web dashboards
- Historical entropy values not comparable
- Slightly more computation per update

### Implementation Path

1. Define `compute_tmi_composite_salience()` in shared library
2. Update TUI's `calculate_entropy()` to use composite
3. Update Web's `compute_entropy()` to use composite
4. Reduce bins from 10 to 5
5. Rename widget/metric to "Cognitive Diversity"

## References

- [DRUG-1b] Carhart-Harris et al. (2014). "Psychedelics increase brain entropy." Frontiers in Human Neuroscience.
- [GWT-1] "Conscious Processing and the Global Neuronal Workspace Hypothesis." PMC8770991.
- [GWT-2] "Global workspace theory." Wikipedia.
- [TMI-NEUTRAL-1] Cury, A. "As Janelas da Memória." citador.pt.
- [TMI-NEUTRAL-2] "O Fenômeno RAM." somostodosum.com.br.
- [BOTTLE-1] Caltech (2025). "Scientists have quantified the speed of human thought." ~10 bits/sec.
- [BOTTLE-2] Quanta Magazine (2019). "To Pay Attention, the Brain Uses Filters, Not a Spotlight."
- [RUSSELL-1] Russell, J. (1980). "A circumplex model of affect." Valence × arousal framework.
