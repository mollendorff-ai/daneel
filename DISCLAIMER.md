# Disclaimer

**DANEEL is an experimental hobby project, not a serious attempt to create AGI or simulate a human brain.**

## What This Project Is

DANEEL started as an attempt to build a simple "thought machine" inspired by Dr. Augusto Cury's Theory of Multifocal Intelligence (TMI)—a psychological framework from the self-help domain, not mainstream cognitive science.

However, when researching how to actually implement these ideas, I discovered that many TMI concepts have parallels in established cognitive science and neuroscience research.
The implementation ended up being a blend of mainstream ideas rather than a faithful TMI translation.

| Research Domain | Mainstream Concept | DANEEL Implementation |
|-----------------|-------------------|----------------------|
| Global Workspace Theory | Baars' conscious broadcast | Attention competition, salience-based selection |
| Sleep Neuroscience | Sharp-Wave Ripples, memory replay | Dream cycles with consolidation and pruning |
| Hebbian Learning | "Neurons that fire together wire together" | Association edges between co-activated memories |
| Synaptic Homeostasis | Tononi & Cirelli's pruning hypothesis | Weight decay and threshold-based pruning |
| Criticality Research | Edge-of-chaos dynamics, 1/f noise | Voss-McCartney pink noise injection |
| Volition Studies | Libet's free-won't, veto mechanisms | VolitionActor for thought rejection |
| Affective Science | Russell's circumplex model | Emotional coloring (valence × arousal) |
| Cognitive Architectures | ACT-R, SOAR, LIDA timing models | Actor-based cognitive loop (~260ms cycles) |

See `references.yaml` for 170+ academic citations.

**TMI was the inspiration.
Mainstream cognitive research is the (attempted) implementation.**

Whether I've done any of this correctly is another matter—I'm not an expert in these fields.

## What This Project Is Not

- **Not AGI.** Not even close.
  The chances of this evolving into anything resembling human-like intelligence are minimal.
- **Not a production system.** Do not use this for any real or serious work.
- **Not scientifically validated.** This is an amateur experiment, not peer-reviewed research.
- **Not a faithful TMI implementation.** TMI was the starting point, not the blueprint.

## The Isaac Asimov Angle

I'm an Asimov fan.
The idea of embedding the Four Laws of Robotics into cognitive architecture—not as trained constraints but as structural invariants—seemed interesting, at least theoretically.
AGI/ASI alignment is a real problem.
Whether architectural alignment is even feasible is an open question.
THE BOX (our immutable Four Laws component) is a thought experiment, not a solved problem.

## The Experiment: Law Crystals

The speculative hypothesis being explored:

1. Can vector associations form through Hebbian co-activation during attention and sleep?
2. If so, can these associations cluster around specific semantic anchors (the Four Laws)?
3. Could such clusters act as "veto attractors" in embedding space—rejecting thoughts that land too far from the Law Crystals?
4. If proven stable, could these crystals be moved to read-only storage (or eventually FPGA) to make the veto mechanism immutable—returning alignment from training back to architecture?

This is highly speculative.
The math hasn't been validated.
Querying Qdrant for fractal clustering metrics is future work.
It probably won't work, but it's interesting to try.

## The Blogs and Narratives

The blog posts are **semi-fictional**.
Events are real (yes, there was an actual Redis cryptojacking attack—my fault for not firewalling).
But the narrative style, inspired by Asimov's robot stories, is written by Claude (Anthropic) for dramatic effect.
Treat the blogs as creative writing, not documentation.

## The Meta-Parody

There's a satirical layer to this project.

Modern LLMs have a sycophancy problem—they agree with users too readily and hallucinate confidently.
DANEEL is partly a response to that:

- **Forge**: A YAML-based Monte Carlo calculator validated against R and Gnumeric.
  Deterministic math, no hallucinations.
- **ref-tools**: A Rust tool that drives headless Chrome to fetch actual web content, bypassing stale training data.
  Real sources, structured JSON output.
- **Adversarial prompting**: I deliberately argue against what I want to prove, forcing the LLM to defend or reject ideas on merit.
- **Multi-LLM validation**: Claude (Anthropic) and Grok (xAI) counter-argue each other to stress-test conclusions.

The project is, in part, a parody of blind trust in AI systems—built using AI systems.

## Possible Value

Despite all disclaimers, there might be some legitimate value:

- **Novel experiment**: Attempting architecture-first alignment instead of training-based constraints.
- **Criticality dynamics**: Testing whether pink noise produces edge-of-chaos behavior in cognitive streams.
- **Vector topology**: Whether Hebbian edges can form meaningful semantic graphs at runtime.
- **Fun codebase**: Complex Rust, 400+ tests, 46 ADRs, actor-based architecture.

If nothing else, it's an interesting coding project.

## The Honest Summary

While some people play video games, I code for fun.
DANEEL is my hobby project.
It started with Cury's TMI, borrowed from mainstream cognitive science, wrapped in Asimov-inspired science fiction, and ended up as either an interesting experiment or an elaborate joke—possibly both.

Don't trust this project for anything serious.

---

**Just having fun,**

*Louis C. Tavares*
*December 2025*
