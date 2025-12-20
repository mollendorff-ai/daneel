# ADR-032: TMI Salience Threshold Calibration

**Status:** Accepted
**Date:** 2025-12-20
**Authors:** Louis C. Tavares, Claude Opus 4.5
**Depends On:** ADR-022 (TMI Memory Schema)

## Context

During the 24-hour continuity test, we discovered that the forgetting mechanism was wired but not functioning as expected. Investigation revealed that the threshold (0.3) was correct, but the **salience generation** was producing uniform distributions that made forgetting nearly impossible.

### The Problem

Current random thought generation:
```rust
// Uniform distribution - ALL values too high
importance:          0.3..0.9  (min 0.3)
novelty:             0.2..0.8  (min 0.2)
relevance:           0.4..0.9  (min 0.4)
connection_relevance: 0.3..0.8  (min 0.3)
```

Minimum possible composite salience: **0.28** (just barely below 0.3 threshold)

Result: Only ~1% of thoughts could ever be forgotten.

### Research Findings

We deployed 4 parallel research agents to investigate proper calibration:

## Research Summary

### 1. TMI Theory (Augusto Cury)

**Key Finding: >90% of cortical archives are NEUTRAL windows**

> "Nas janelas neutras, que correspondem a **mais de 90% dos arquivos no córtex cerebral**, ficam milhões de dados – ou seja, são janelas que não têm conteúdo emocional ou tem baixíssimo conteúdo emocional."

Translation: "In neutral windows, which correspond to **more than 90% of archives in the cerebral cortex**, are millions of data points - windows with no or very low emotional content."

**Implication:** Most thoughts should have BASE/LOW salience, with only ~10% elevated.

### 2. Ebbinghaus Forgetting Curve

| Time | % Forgotten |
|------|-------------|
| 1 hour | 50% |
| 24 hours | 70% |
| 1 week | 90% |
| 1 month | 90%+ |

Without rehearsal/consolidation, the vast majority of information is lost.

### 3. Attention Filtering (Global Workspace Theory)

**Information bottleneck:**
- Sensory input: ~1 billion bits/second
- Conscious processing: ~10 bits/second
- **Filter ratio: 99.9999% filtered out**

Only the most salient inputs break through to conscious awareness.

### 4. Memory Consolidation Research

- Working memory capacity: **4 ± 1 chunks** (Cowan 2001)
- Only **~10% of experiences** transfer to long-term memory
- Emotional significance dramatically increases consolidation probability
- Sleep-dependent consolidation during slow-wave sleep

## Decision

### TMI-Faithful Salience Distribution

Generate thoughts with a **bimodal distribution** matching TMI's 90/10 split:

```rust
fn generate_random_thought(&self) -> (Content, SalienceScore) {
    let mut rng = rand::rng();

    // TMI: 90% of thoughts are neutral/low-salience
    let (importance, novelty, relevance, connection) = if rng.random::<f32>() < 0.90 {
        // 90%: Neutral/low-salience thoughts (neutral windows)
        (
            rng.random_range(0.0..0.35),   // importance
            rng.random_range(0.0..0.30),   // novelty
            rng.random_range(0.0..0.40),   // relevance
            rng.random_range(0.1..0.40),   // connection (min 0.1 per invariant)
        )
    } else {
        // 10%: Higher-salience thoughts (emotional/important)
        (
            rng.random_range(0.5..0.95),
            rng.random_range(0.4..0.85),
            rng.random_range(0.5..0.95),
            rng.random_range(0.5..0.90),
        )
    };

    let salience = SalienceScore::new(
        importance,
        novelty,
        relevance,
        rng.random_range(-0.5..0.5), // valence (unchanged)
        connection,
    );

    (content, salience)
}
```

### Expected Outcomes

With `forget_threshold = 0.3` and TMI-faithful distribution:

| Category | Expected % | Action |
|----------|-----------|--------|
| **FORGOTTEN** | ~85-90% | Salience < 0.3 → XDEL from Redis |
| **KEPT** | ~8% | 0.3 ≤ Salience < 0.7 → Remain in Redis |
| **CONSOLIDATED** | ~2-5% | Salience ≥ 0.7 → Persist to Qdrant |

### Threshold Values (Unchanged)

| Threshold | Value | Rationale |
|-----------|-------|-----------|
| `forget_threshold` | 0.3 | TMI anchor threshold for neutral content |
| `consolidation_threshold` | 0.7 | High-salience only to long-term memory |
| `permanent_threshold` | 0.9 | Fully consolidated, immune to pruning |

### Connection Drive Preservation

The connection_relevance minimum of 0.1 is preserved per THE BOX invariant:

```rust
// Even in low-salience mode, connection must be > 0
connection_relevance: rng.random_range(0.1..0.40)  // Never 0
```

This ensures the architectural alignment mechanism remains active.

## Consequences

### Positive

1. **TMI-Faithful**: Matches Augusto Cury's 90% neutral windows observation
2. **Realistic Memory**: Mirrors human forgetting curves
3. **Efficient**: Redis stream will stabilize instead of growing unboundedly
4. **Observable**: Clear behavioral difference (forgetting actually happens)

### Negative

1. **More noise in low-salience range**: 90% of thoughts will be quickly forgotten
2. **Debugging complexity**: Most thoughts won't persist for analysis

### Neutral

1. Threshold values (0.3, 0.7) remain unchanged
2. Composite salience formula unchanged
3. Connection drive invariant preserved

## References

### TMI (Augusto Cury)
- [TMI Memory Model Research](/research/TMI_Memory_Model_Research.md)
- [As Janelas da Memória](https://www.citador.pt/textos/as-janelas-da-memoria-augusto-cury)
- [O Fenômeno RAM](https://www.somostodosum.com.br/blog-autoconhecimento/o-fenomeno-ram--registro-automatico-da-memoria--7101.html)

### Cognitive Science
- [Ebbinghaus Forgetting Curve - PMC4492928](https://pmc.ncbi.nlm.nih.gov/articles/PMC4492928/)
- [Working Memory Capacity 4 - PMC2864034](https://pmc.ncbi.nlm.nih.gov/articles/PMC2864034/)
- [Miller's Magical Number Seven](https://en.wikipedia.org/wiki/The_Magical_Number_Seven,_Plus_or_Minus_Two)

### Neuroscience
- [Global Workspace Theory - PMC8770991](https://pmc.ncbi.nlm.nih.gov/articles/PMC8770991/)
- [Information Bottleneck - Caltech 2025](https://www.technologynetworks.com/neuroscience/news/caltech-scientists-have-quantified-the-speed-of-human-thought-394395)
- [Memory Consolidation During Sleep - PMC3278619](https://pmc.ncbi.nlm.nih.gov/articles/PMC3278619/)
- [Attention Filtering - Quanta Magazine](https://www.quantamagazine.org/to-pay-attention-the-brain-uses-filters-not-a-spotlight-20190924/)

## Implementation

See commit implementing this ADR for the exact code changes in:
- `src/core/cognitive_loop.rs` - `generate_random_thought()` function
