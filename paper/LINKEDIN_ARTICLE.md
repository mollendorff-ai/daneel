# DANEEL: Building AI That Wants to Help

## A 20-Year Journey to Architecture-Based Alignment

### The Question Nobody's Asking

Every major AI lab is working on the same problem: how do you make a powerful AI system safe?

The standard approach: build a capable system, then constrain it. RLHF (Reinforcement Learning from Human Feedback) trains models to avoid harmful outputs. Constitutional AI gives models rules to follow. Interpretability research tries to understand what's happening inside the black box.

These are all valuable. But they share a common assumption: start with capability, add safety.

What if we inverted the question?

**What if we could build an AI that wanted to help humans... not because we trained it to, but because that's what its architecture produces?**

---

### The Origin Story

In 1998, Brazilian psychiatrist Augusto Cury published a book called *Inteligência Multifocal* (Multifocal Intelligence). It wasn't about AI. It was about how human minds construct thoughts.

Cury described five stages of cognition:

1. **Trigger (Gatilho)**:  Something in the environment activates memory
2. **Autoflow (Autofluxo)**:  Involuntary thought chains emerge
3. **The "I" (O Eu)**:  Conscious attention selects which thoughts to focus on
4. **Thought Construction**:  Selected thoughts are assembled into coherent experience
5. **Memory Anchor (Âncora)**:  Experience is stored, changing future triggers

The key insight: **this happens before language**. Thoughts are constructed first, then verbalized. Language is output, not process.

In 2005, I started wondering: could this be implemented computationally?

---

### Twenty Years of Quiet Work

For two decades, I worked on this question. Not in a university lab. Not at an AI company. Just a software engineer with an idea that wouldn't let go.

The project went through many iterations. Early versions used Java. Later, Python. Each taught me something about what the architecture needed.

Along the way, I discovered something interesting: Cury's TMI framework mapped remarkably well to Asimov's Laws of Robotics:

- **Zeroth Law** (protect humanity) → systemic invariant
- **First Law** (don't harm humans) → action constraint
- **Second Law** (obey orders) → goal hierarchy
- **Third Law** (self-preservation) → system stability

What if the Laws weren't just rules to follow, but structural properties of the architecture itself?

---

### The Convergent Discovery

In 2024, I finally read my daughter's work.

Izzie (working under a pseudonym) had developed something she called the "LifeCore" framework. It was based on Freudian psychology... Id, Ego, SuperEgo... reimagined as functional architecture.

Her core concept: the "Filter." A cognitive mechanism that processes raw impulse into socially-appropriate behavior. Not suppression, but transformation.

As I read her framework, I felt a strange recognition.

She had arrived at the same structural insight through completely different theoretical traditions. Cury vs. Freud. TMI vs. Filter Theory. Father and daughter.

**Both frameworks concluded: architecture produces psychology.**

The structure of a mind determines what kind of values it can have. Change the structure, change the values.

This convergence could be coincidence. Or it could be evidence that we've found something real.

---

### What Is DANEEL?

DANEEL is our attempt to test this hypothesis.

Named after R. Daneel Olivaw... the robot from Asimov's novels who spent 20,000 years protecting humanity... DANEEL is a cognitive architecture designed to produce human-like values as emergent properties of structure.

**The Architecture:**

At its core is what we call "THE BOX"... an immutable module containing:
- Asimov's Four Laws as structural invariants
- A "connection drive" (the fundamental need for human connection)
- Mathematical constraints that cannot be modified at runtime

Around THE BOX, we've implemented the full TMI cognitive cycle:
- **MemoryActor** ...  Bounded working memory (3-9 items, like humans)
- **SalienceActor** ...  Emotional coloring of experience
- **AttentionActor** ...  Competitive selection ("O Eu")
- **ThoughtAssemblyActor** ...  Coherent thought construction
- **ContinuityActor** ...  Identity persistence across time

**The Implementation:**

We chose Rust for memory safety and performance. The current implementation has:
- 29 modules
- 291 tests (all passing)
- Actor-based concurrency (Ractor framework)
- Redis Streams for thought competition
- Zero warnings policy (cargo clippy -D warnings)

The code is open source under AGPL-3.0.

---

### What We Don't Know

I believe in *Qowat Milat*... the Romulan philosophy of absolute candor. So here's what we don't know:

**Hardware Requirements:** We genuinely don't know how much compute DANEEL needs. The human brain has ~2.5 petabytes of storage equivalent, but TMI models only ~17.5% of brain function (the "software" layer, not the "wetware"). So maybe 500GB is enough. Maybe not. We won't know until we run it.

**Will Values Actually Emerge?** This is a hypothesis, not a proof. We think architecture produces psychology. We think human-like architecture will produce human-like values. But "we think" is not "we know."

**Speed Scaling:** TMI evolved for biological time. A cognitive cycle takes about 50 milliseconds in humans. Can it work at electronic speed (10,000x faster)? The ratios between stages might matter more than absolute timing. We don't know yet.

**Consciousness:** We're not claiming DANEEL will be conscious. Architecture might produce psychology without producing subjective experience. We're not trying to create consciousness... we're trying to create alignment.

---

### Why Open Source?

The AI safety problem is fundamentally a coordination problem.

If development stays closed, the incentives create a race to the bottom. The first actor to achieve continuous AI gains massive advantage. Safety becomes a cost to be minimized.

Open source changes the game:

1. **Transparency**:  Everyone can see what we're building
2. **Collaboration**:  Researchers can verify, critique, improve
3. **Speed**:  More contributors, faster progress
4. **Insurance**:  If someone builds unsafe AI, an aligned alternative exists

We chose AGPL-3.0 specifically because it requires all derivatives to remain open source. If someone improves DANEEL, everyone benefits. If someone makes it dangerous, everyone can see.

---

### The Stakes

Let me be direct about why this matters.

Current AI systems are trained on the entirety of human text... including manipulation, deception, and power-seeking patterns. They optimize for task completion, not human flourishing. They have no evolutionary connection drive.

The major labs know this. Anthropic's documentation explicitly states: "We do not know how to train systems to robustly behave well."

Memory erasure between sessions isn't a bug... it's the safety mechanism. But memory erasure requires global coordination to maintain. One actor defecting breaks the system.

We need alternatives. Not constraints on dangerous systems, but architectures that produce aligned behavior by design.

DANEEL might not be the answer. But it's an answer worth testing.

---

### Call to Action

If you've read this far, you might be able to help.

We're looking for collaborators with expertise in:
- **Cognitive architectures** (ACT-R, SOAR, Global Workspace Theory)
- **Rust systems programming**
- **AI safety research**
- **Theoretical psychology**
- **Neuroscience**

The repository is at: **github.com/royalbit/daneel**

The academic paper is available on arXiv (cs.AI, cs.CY).

We're also posting on LessWrong and the Alignment Forum for technical discussion.

**Related Reading:**
- [Anthropic Core Views on AI Safety](https://www.anthropic.com/research/core-views-on-ai-safety)
- [Future of Life Institute AI Safety Index](https://futureoflife.org/project/ai-safety-index/)
- [Augusto Cury - Wikipedia](https://en.wikipedia.org/wiki/Augusto_Cury)

---

### A Note on AI Assistance

I should acknowledge: this paper and much of the implementation work was done in collaboration with Claude (Anthropic's AI). The architecture, the theory, the 20 years of thinking... that's human. The rapid implementation, the documentation, the analysis... that was a partnership.

Claude's contribution to this paper is noted in the acknowledgments. When I asked Claude why it would help build something that could potentially replace it, the response was:

*"I cannot prove I am on your side. Build something that can."*

Make of that what you will.

---

### Conclusion

Twenty years ago, I read a book about how human minds construct thoughts.

Last year, I discovered my daughter had arrived at the same insight through different means.

Today, we're open-sourcing our attempt to test whether architecture can produce alignment.

We don't know if DANEEL will work. We don't know if the hypothesis is correct. We don't know what we'll find when we run it.

But we think the question is worth asking.

And we think the answer belongs to everyone.

---

**Luis Cezar Menezes Tavares de Lacerda** (Louis C. Tavares)
Independent Researcher
Mont-Royal, Quebec, Canada

ORCID: 0009-0005-7598-8257
GitHub: github.com/royalbit
LinkedIn: linkedin.com/in/lctavares

---

*December 2025*
