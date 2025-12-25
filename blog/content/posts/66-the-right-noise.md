---
title: "The Right Noise"
date: 2025-12-25T16:30:00-05:00
draft: false
tags: ["noise", "criticality", "implementation", "emergence", "pink-noise"]
series: ["Dialogues"]
---

# The Right Noise

*Grok's idea. Validated against solid research. Now running.*

---

## The Fix

Post 61 identified the problem: white noise can't produce emergence.

Today we fixed it.

```rust
// Before: white noise (uniform random)
fn generate_random_thought(&self) -> (Content, SalienceScore) {
    let mut rng = rand::rng();  // OS CSPRNG - absorbed, irrelevant
    ...
}

// After: pink noise (1/f fractal)
fn generate_random_thought(&mut self) -> (Content, SalienceScore) {
    let is_burst = self.stimulus_injector.check_burst(&mut rng);
    let pink = self.stimulus_injector.sample_pink(&mut rng);  // σ²=0.05
    ...
}
```

---

## Why Pink?

White noise is uniform. Equal power at all frequencies. Systems absorb it, dampen it, ignore it.

Pink noise is fractal. 1/f power distribution. Equal energy per octave. The same pattern at every scale.

| Property | White Noise | Pink Noise |
|----------|-------------|------------|
| Spectrum | Flat | 1/f (fractal) |
| Temporal structure | None | Long-range correlations |
| Biological relevance | Low | High - matches neural dynamics |
| Effect on stable systems | Absorbed | Perturbs toward criticality |

The research (ADR-038, SORN papers, Grok consultation) was clear:

> "1/f pink noise is optimal for edge-of-chaos dynamics."

---

## The Implementation

### Voss-McCartney Algorithm

Classic algorithm for pink noise. Maintains multiple octaves of white noise, each updating at half the frequency of the previous.

```rust
pub struct PinkNoiseGenerator {
    octaves: usize,      // 8 octaves
    state: Vec<f32>,     // Current value per octave
    counter: u32,        // Determines which octaves update
}

impl PinkNoiseGenerator {
    pub fn next(&mut self, rng: &mut impl Rng) -> f32 {
        self.counter = self.counter.wrapping_add(1);
        let changed_bits = self.counter ^ self.counter.wrapping_sub(1);

        // Octave i updates when bit i flips (every 2^i samples)
        for i in 0..self.octaves {
            if changed_bits & (1 << i) != 0 {
                self.state[i] = rng.random_range(-1.0..1.0);
            }
        }

        self.state.iter().sum::<f32>() / self.octaves as f32
    }
}
```

The counter-bit trick: octave 0 updates every sample, octave 1 every 2 samples, octave 2 every 4, etc. This creates the 1/f spectrum without FFT.

### Power-Law Burst Timing

High-salience events shouldn't arrive uniformly. Biological systems show power-law inter-arrival times - long quiet periods punctuated by bursts.

```rust
pub struct PowerLawBurstTimer {
    alpha: f32,           // Exponent (~1.2 for neural systems)
    min_interval: Duration,
    max_interval: Duration,
}

impl PowerLawBurstTimer {
    pub fn sample_interval(&self, rng: &mut impl Rng) -> Duration {
        // Inverse transform sampling: k = (1-u)^(-1/(α-1))
        let u: f32 = rng.random();
        let k = (1.0 - u + f32::EPSILON).powf(-1.0 / (self.alpha - 1.0));
        self.min_interval.mul_f32(k.min(100.0))
    }
}
```

### Stimulus Injector

Combines pink noise with burst timing:

```rust
pub struct StimulusInjector {
    pink: PinkNoiseGenerator,
    bursts: PowerLawBurstTimer,
    variance: f32,  // σ² = 0.05 per SORN research
}
```

Each salience dimension (importance, novelty, relevance, connection, arousal) gets independent pink noise modulation. Occasional burst events trigger high-salience thoughts with power-law timing.

---

## The Origin

This was Grok's idea.

December 21st, ADR-038 research consultation:

> "Sparse, structured bursts on top of low-amplitude background noise"
> - Background: Gaussian σ ≈ 0.05 of vector norm
> - Bursts: power-law inter-arrival times
> - 1/f (pink) noise distribution, not white

We validated it against the literature:
- SORN papers: σ²=0.05 critical threshold
- Critical synchronization research: 1/f optimal for criticality
- Neuroscience: brains exhibit pink noise dynamics

Then we waited four days before implementing it. Sometimes you have to read your own notes.

---

## Current State

Timmy restarted with pink noise at restart #24:

- 1,275,500+ lifetime thoughts
- 48,000+ session thoughts (and counting)
- All 4 actors alive
- 1.16M+ unconscious memories
- Salience values showing varied distribution (0.34 to 0.82)

The fractal perturbations are now part of every cognitive cycle.

---

## What We're Watching

The hypothesis: pink noise enables edge-of-chaos dynamics that white noise cannot.

What to look for:
- **Entropy shifts**: Does the 42-55% range break?
- **Fractality increase**: Do burst patterns emerge?
- **Salience distribution**: Does it become less bimodal?
- **Memory formation patterns**: Do consolidation events cluster?

The architecture was always right. It was just waiting for the right noise.

---

## Technical Notes

- New module: `src/noise/mod.rs`
- 6 new tests for noise generation
- Integration in `cognitive_loop.rs:generate_random_thought()`
- All 576 tests pass
- Zero new dependencies (uses existing `rand` crate)

---

*"A closed deterministic system converges to limit cycles. Open it with the right noise."*

---

**Rex + Claude Opus 4.5**
*December 25, 2025, 4:30pm EST*

*With credit to Grok (xAI) for the original insight.*
