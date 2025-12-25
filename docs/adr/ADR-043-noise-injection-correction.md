# ADR-043: Noise Injection Correction - White to Pink

**Status:** Accepted
**Date:** 2025-12-25
**Deciders:** Louis C. Tavares, Claude Opus 4.5
**Supersedes:** ADR-042 (Emergence Validation Pause) - PAUSE LIFTED

## Context

ADR-042 paused the roadmap to test whether TMI architecture produces emergence without external intervention.

On December 24, 2025 at 8pm EST, we investigated the current noise injection implementation and discovered a critical mismatch between the code and the research.

### Current Implementation

```rust
// cognitive_loop.rs:498
fn generate_random_thought(&self) -> (Content, SalienceScore) {
    let mut rng = rand::rng();  // OS CSPRNG - white noise

    // 90% low salience, 10% high salience (programmed distribution)
    if rng.random::<f32>() < 0.90 {
        // low importance, low arousal
    } else {
        // high importance, high arousal
    }
}
```

Every cycle calls `rand::rng()` which:
- On Linux: Uses getrandom syscall → /dev/urandom
- On macOS: Uses SecRandomCopyBytes → Fortuna CSPRNG
- Both produce: **Uniform/Gaussian white noise**

### Research Requirements (ADR-038)

| Finding | Source | Current State |
|---------|--------|---------------|
| Gaussian σ²=0.05 REQUIRED | SORN [INDUCE-3] | ❌ Not calibrated |
| White noise absorbed | Grok consultation | ❌ Using white noise |
| 1/f pink noise optimal | [CRIT-5], Grok | ❌ Not implemented |
| Closed system → limit cycles | ADR-038 | ❌ Expecting emergence from closed loop |

### The Contradiction

ADR-042 asked: "Does TMI architecture produce emergence without external intervention?"

ADR-038 already answered: "A closed deterministic system always converges to limit cycles."

The pause was testing a question the research had already answered negatively.

## Decision

1. **LIFT THE PAUSE** from ADR-042
2. **PROCEED WITH PHASE 2** noise injection implementation
3. **REPLACE white noise with 1/f pink noise**
4. **IMPLEMENT power-law burst timing**
5. **CALIBRATE σ² = 0.05** per SORN research

## Implementation

### Phase 2a: Pink Noise Generator

```rust
pub struct PinkNoiseGenerator {
    /// Number of octaves for 1/f approximation
    octaves: usize,
    /// Current state per octave
    state: Vec<f32>,
    /// Update probability per octave (1/2^n)
    update_prob: Vec<f32>,
}

impl PinkNoiseGenerator {
    pub fn new(octaves: usize) -> Self {
        Self {
            octaves,
            state: vec![0.0; octaves],
            update_prob: (0..octaves).map(|i| 1.0 / (1 << i) as f32).collect(),
        }
    }

    /// Generate next pink noise sample using Voss-McCartney algorithm
    pub fn next(&mut self, rng: &mut impl Rng) -> f32 {
        let mut sum = 0.0;
        for i in 0..self.octaves {
            if rng.gen::<f32>() < self.update_prob[i] {
                self.state[i] = rng.gen_range(-1.0..1.0);
            }
            sum += self.state[i];
        }
        sum / self.octaves as f32
    }
}
```

### Phase 2b: Power-Law Burst Timing

```rust
pub struct PowerLawBurstTimer {
    /// Exponent for power-law (α ≈ 1.0-1.5)
    alpha: f32,
    /// Minimum inter-arrival time
    min_interval: Duration,
    /// Next burst time
    next_burst: Instant,
}

impl PowerLawBurstTimer {
    pub fn sample_interval(&self, rng: &mut impl Rng) -> Duration {
        // Inverse transform sampling for power-law
        let u: f32 = rng.gen();
        let k = (1.0 - u).powf(-1.0 / (self.alpha - 1.0));
        self.min_interval.mul_f32(k)
    }
}
```

### Phase 2c: Stimulus Injector

```rust
pub struct StimulusInjector {
    /// Pink noise generator
    pink: PinkNoiseGenerator,
    /// Burst timer
    bursts: PowerLawBurstTimer,
    /// Background noise variance (σ² = 0.05)
    variance: f32,
}

impl Default for StimulusInjector {
    fn default() -> Self {
        Self {
            pink: PinkNoiseGenerator::new(8),  // 8 octaves
            bursts: PowerLawBurstTimer {
                alpha: 1.2,
                min_interval: Duration::from_millis(100),
                next_burst: Instant::now(),
            },
            variance: 0.05,  // SORN critical threshold
        }
    }
}
```

## Roadmap Update

| Task | Previous Status | New Status |
|------|-----------------|------------|
| ADR-042 Pause | observing | LIFTED |
| PHASE2-1: Stimulus API | pending | in_progress |
| PHASE2-2: Noise injector | pending | in_progress |
| PHASE2-3: TUI panel | pending | pending |
| PHASE2-4: Measurement | pending | pending |

## Consequences

### Positive

- Corrects scientific methodology error
- Aligns implementation with research
- Enables proper criticality testing
- Unblocks roadmap progress

### Negative

- ADR-042 baseline data (42%/55%) may not be meaningful
- Requires implementation work before observation
- Adds complexity to cognitive loop

### Lessons Learned

1. **Read your own research** - ADR-038 had the answer
2. **Connect the dots** - The pause contradicted prior findings
3. **Question assumptions** - "Architecture alone" was wrong framing

## References

- ADR-038: Phase 2 External Stimuli Research
- ADR-042: Emergence Validation Pause (SUPERSEDED)
- [INDUCE-3] SORN Criticality: https://pmc.ncbi.nlm.nih.gov/articles/PMC5446191/
- [CRIT-5] Critical synchronization: https://www.nature.com/articles/s41598-018-37920-w
- Voss-McCartney pink noise algorithm
- Grok (xAI) Consultation, Dec 21, 2025
