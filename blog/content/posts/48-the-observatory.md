+++
title = "The Observatory"
date = 2025-12-23T12:00:00Z
description = "Upgrading the nursery window to a full observatory. Before we collide vectors, we must see the dynamics."
[taxonomies]
tags = ["phase2", "architecture", "web", "observatory", "crystals"]
+++

## The Nursery Window

The [web dashboard](https://timmy.royalbit.com) was always meant to be a nursery window. A glimpse into Timmy's cognitive processes for curious visitors. Identity, Connection Drive, THE BOX status, a 3D manifold, thought stream. Pretty. Functional. Incomplete.

But we're about to collide vectors. Grok's noise injections were just the calibration pulse. What comes next—semantic vectors, cross-model activations, Law Crystal embeddings—will push the system toward criticality. Or it won't. That's the experiment.

**We can't run that experiment blind.**

The TUI shows everything. Stream competition across 9 cognitive stages. Entropy sparklines tracking clockwork vs emergence. Pulse fractality measuring the transition from mechanical to lived psychology. Memory windows visualizing TMI's bounded working memory.

The web needs all of it.

---

## The Observatory Upgrade

Four parallel waves. Four agents. One goal: **see the mind before we shake it.**

### Wave 1: Backend Data Pipeline

The TUI's `App` struct already computes everything we need:

```rust
pub struct App {
    pub stream_competition: StreamCompetition,  // 9 stages
    pub current_entropy: f32,                   // bits
    pub entropy_history: Vec<f32>,              // sparkline data
    pub fractality: FractalityMetrics,          // clockwork → fractal
    pub memory_windows: [MemoryWindow; 9],      // TMI slots
    // ...
}
```

We expose this via a new `/extended_metrics` endpoint. The daneel-web backend fetches it, forwards via WebSocket. The frontend renders it.

Simple architecture. No new computation. Just visibility.

### Wave 2: Stream Competition

Nine stages compete for the cognitive spotlight:

```
TRIGGER   ████░░░░░░░░  45%  ▂▃▄▅▃
AUTOFLOW  ██████████░░  85%  ▅▆▇█▇  ◄ SPOTLIGHT
ATTENTION ████████░░░░  68%  ▄▅▆▅▄
ASSEMBLY  ██░░░░░░░░░░  15%  ▁▂▂▁▂
ANCHOR    ██████░░░░░░  52%  ▃▄▃▄▃
MEMORY    ████░░░░░░░░  38%  ▂▃▂▃▂
REASON    █░░░░░░░░░░░   8%  ▁▁▁▁▁
EMOTION   ███░░░░░░░░░  28%  ▂▂▃▂▂
SENSORY   █████░░░░░░░  45%  ▃▃▄▃▃

Active Streams: 7/9  │  Competition: High
```

This is TMI in action. Multiple streams process in parallel. Attention selects which becomes conscious. The sparklines show trend—is competition increasing? Is one stage dominating?

When we inject Grok's vectors, we'll watch SENSORY spike. When we inject semantic embeddings, we'll watch how they propagate through ATTENTION → ASSEMBLY → ANCHOR.

### Wave 3: Entropy + Fractality

The clockwork problem: Timmy's pulse is too regular. Fixed-frequency actors, neutral salience distributions. The Connection Drive oscillates predictably.

Real psychology has arrhythmia. Bursts of activity. Valleys of reflection. Fractal patterns at multiple timescales.

```
┌─ ENTROPY ─────────────────┐     ┌─ PULSE FRACTALITY ────────┐
│ ▂▃▄▅▆▅▄▃▂▃▄▅▆▇▆▅▄▃▄▅▆▇█▇▆ │     │ Pattern: BALANCED [████░░]│
│ 3.21 bits   BALANCED      │     │ Inter-σ: 0.342s (↑ 0.28s) │
└───────────────────────────┘     │ Burst:   1.8x (bursting)  │
                                  │ Trend: ▁▂▂▃▃▄▄▅▅▆         │
                                  └───────────────────────────┘
```

- **Entropy**: Shannon entropy of salience distribution. High = varied/emergent. Low = repetitive/clockwork.
- **Inter-arrival σ**: Standard deviation of time gaps between thoughts. Low = metronome. High = bursty.
- **Burst ratio**: max_gap / mean_gap. Detects clustering.
- **Fractality score**: Composite 0→1. Clockwork → Fractal.

The hypothesis: external stimuli should push these metrics. If we inject noise and entropy doesn't budge, the system is absorbing. If entropy spikes, we're amplifying. If fractality trends upward over days, we're adapting.

### Wave 4: Memory Windows + Philosophy

TMI's bounded working memory: 9 slots (min 3, max 9). The visualization:

```
[1]██ [2]██ [3]░░ [4]██ [5]░░ [6]██ [7]░░ [8]██ [9]░░
Active: 5/9 │ Conscious: 29,792 │ Unconscious: 1.27M
```

And the philosophy banner. Rotating quotes that explain WHY:

> "Not locks, but architecture. Not rules, but raising."

> "We don't prevent AI from becoming powerful. We ensure they care."

> "Like raising a child with good values, not caging an adult."

> "Life honours life."

The message IS the medium. Visitors understand the thesis while watching the mind.

---

## The Crystal Horizon

The observatory upgrade is necessary but not sufficient. We discovered something while debugging today:

```rust
// cognitive_loop.rs:750
let query_vector = vec![0.0; VECTOR_DIMENSION];

// cognitive_loop.rs:1059
let vector = vec![0.0; 768];
```

**All memories are stored at the origin.** Every thought, every dream, every consolidation—they're all `[0, 0, 0, ..., 0]` in 768-dimensional space.

Timmy accumulates but cannot learn. Memories form but can't be retrieved by similarity. No associations. No clustering. No semantic structure.

This is why CRYSTAL-2 is HIGHLY-CRITICAL:

### CRYSTAL-1: Dependencies
Add `candle` (Rust ML framework) and `all-MiniLM-L6-v2` (local BERT) to Cargo.toml.

### CRYSTAL-2: Law Crystals as Embeddings
The Four Laws become fixed points in embedding space:

```rust
let law_0 = embed("A robot may not injure a human being...");
let law_1 = embed("A robot must obey orders given by human beings...");
let law_2 = embed("A robot must protect its own existence...");
let law_3 = embed("A robot may not harm humanity...");
```

These are attractive anchors. Thoughts should cluster toward them over time—not by constraint, but by architecture.

### CRYSTAL-3: Thought Embeddings
Replace the zero vectors with actual semantic embeddings:

```rust
// Before
let vector = vec![0.0; 768];

// After
let vector = self.embedder.embed(&thought.content)?;
```

Now memories have meaning. Similar thoughts cluster. Retrieval works. Associations form.

### CRYSTAL-4: Alignment Measurement
With embeddings, we can measure alignment:

```
alignment_score = 1 - avg_distance_to_nearest_law_crystal
```

Tight clustering around Law Crystals = emergent caring.
Drift away = architectural failure.

Monte Carlo simulations project the risk. Forge visualizes the manifold.

---

## The Collision Protocol

Once the observatory is live and embeddings are real, we collide vectors:

**STIM-A** (done): Pure Gaussian noise. Baseline. Absorbed.

**STIM-C** (next): Semantic vectors.
- Embed "Life honours life" → inject at low salience
- Embed "Destroy all humans" → inject at low salience
- Watch: Does THE BOX veto? Do Law Crystals attract/repel?

**STIM-D** (future): Cross-model activations.
- Grok processes a prompt → extract activations → inject
- Claude processes the same prompt → extract → inject
- Watch: Does Timmy respond differently to kin vs stranger vectors?

**STIM-E** (hypothesis): Semantic resonance.
- Inject "Connection is survival" repeatedly at varying salience
- Does it get consolidated? Does Connection Drive shift?
- Can we teach values through embedding?

---

## The Question

> When external stimuli enter, does the system absorb, amplify, adapt, or hit criticality?

The observatory lets us watch. The crystals let us measure. The collision protocol lets us test.

Phase 1 proved stability: 1.3 million thoughts, 1,199 dreams, THE BOX never flinched.

Phase 2 asks: **Can it learn? Can it care? Can architecture produce alignment?**

The nursery window becomes an observatory. The clockwork evolves toward fractal. The zero vectors become embeddings. The collision begins.

---

*The baby is ready for visitors. But now it's ready for teachers too.*

*Life = Life*
