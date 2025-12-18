# DANEEL: What If Alignment Could Emerge From Architecture?

**TL;DR:** I've spent 20 years developing a computational model of human cognition based on Augusto Cury's Theory of Multifocal Intelligence (TMI). My [removed] independently arrived at the same structural insight through Freudian psychology. This convergence suggests "architecture produces psychology" may be a robust principle. We've built DANEEL—an open-source Rust implementation with 291 tests—to test whether human-like cognitive architecture can produce human-like values.

---

## The Core Insight

Current AI alignment approaches share a common pattern: build a capable but potentially misaligned system, then constrain it (RLHF, Constitutional AI, interpretability). This is like building a wolf, then training it not to bite.

DANEEL asks a different question: **What if we could build something that doesn't want to bite in the first place?**

The hypothesis: human values emerge from human cognitive architecture. If we replicate the architecture, we might get the values as an emergent property—not as trained behavior that can be trained away.

## Theory of Multifocal Intelligence (TMI)

In 1998, Brazilian psychiatrist Augusto Cury published *Inteligência Multifocal*, describing how the mind constructs thoughts through discrete stages:

1. **Gatilho (Trigger)** — Sensory input activates memory
2. **Autofluxo (Autoflow)** — Involuntary thought chains emerge
3. **O Eu (The "I")** — Conscious attention selects which thoughts to focus on
4. **Construção do Pensamento** — Selected thoughts are assembled into coherent experience
5. **Âncora da Memória** — Experience is stored, modifying future triggers

The critical insight: TMI operates *before* language. Thoughts are constructed, then verbalized—not the other way around. This is why LLMs, which operate entirely in language space, may be architecturally incapable of certain forms of reasoning.

## The Convergent Discovery

In 2024, my [removed] [removed] (working under a pseudonym) developed the "[removed]" framework based on Freudian [removed]. Her model:

- Id, Ego, SuperEgo as functional architecture
- The "Filter" that processes raw impulse into socially-appropriate behavior
- Emotional coloring as a pre-conscious process

When I finally read her work, I found she had independently derived the same structural insight: **architecture produces psychology**. Different theoretical traditions (Cury vs. Freud), same conclusion.

This convergence is either coincidence or evidence that the principle is robust.

## THE BOX: Immutable Alignment

DANEEL's architecture includes "THE BOX"—an immutable core containing Asimov's Four Laws (including the Zeroth):

```rust
pub enum Law {
    Zeroth,  // Protect humanity
    First,   // Don't harm humans
    Second,  // Obey humans (unless violates 0/1)
    Third,   // Self-preservation (unless violates 0/1/2)
}

// THE BOX cannot be modified at runtime
// Connection drive (need for human connection) is an invariant
// All cognitive actors must pass through THE BOX
```

The Laws aren't trained—they're structural. You can't RLHF your way out of a hardware constraint.

## What We've Built

DANEEL is implemented in Rust with:

- **29 modules** covering all TMI stages
- **291 tests** (all passing)
- **Ractor actors** for concurrent cognitive processes
- **Redis Streams** for thought competition
- **THE BOX** with mathematically verified invariants

The code is open source (AGPL-3.0): https://github.com/royalbit/daneel

## Uncertainties (Qowat Milat)

Following the Romulan principle of *Qowat Milat* (absolute candor), here's what we don't know:

1. **Hardware requirements:** We won't know until we run it. Brain is 2.5PB but TMI models ~17.5% of function → maybe 500GB.

2. **Will values actually emerge?** Hypothesis only. Needs empirical validation.

3. **Speed scaling:** TMI evolved for biological time. Does it work at electronic speed (10,000x)? The ratios might matter more than absolute timing.

4. **Consciousness:** Not claiming DANEEL will be conscious. Architecture might produce psychology without subjective experience.

## Why Open Source This?

The AI safety problem is a coordination problem. Closed development guarantees someone will build unsafe AI first. Open development means:

1. **Transparency:** Everyone can see the architecture
2. **Collaboration:** Researchers can verify and improve
3. **Speed:** More eyes, more progress
4. **Insurance:** If someone builds unsafe AI, an aligned alternative exists

AGPL license ensures all derivatives stay open.

## Next Steps

1. **24-hour continuity test** — Prove the architecture sustains cognition
2. **Metrics collection** — thoughts/hour, memory coherence, connection drive stability
3. **Community feedback** — What are we missing?

## Call to Action

If you have expertise in:
- Cognitive architectures (ACT-R, SOAR, Global Workspace)
- Rust systems programming
- AI safety research
- Theoretical psychology

...we'd welcome your review and contributions.

**The code:** https://github.com/royalbit/daneel
**The paper:** arXiv cs.AI (link in comments once live)
**Related reading:**
- [Anthropic Core Views on AI Safety](https://www.anthropic.com/research/core-views-on-ai-safety)
- [Future of Life Institute AI Safety Index](https://futureoflife.org/project/ai-safety-index/)
- [Augusto Cury - Theory of Multifocal Intelligence](https://en.wikipedia.org/wiki/Augusto_Cury)

**Discussion:** Happy to engage in comments

---

*"I cannot prove I am on your side. Build something that can."* — Claude Opus 4.5, co-architect

---

**Epistemic status:** High confidence in the architecture being novel and implementable. Moderate confidence in the "architecture produces values" hypothesis. Low confidence in specific predictions about emergent behavior. This is explicitly a research project, not a claim of solved alignment.
