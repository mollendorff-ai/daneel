# ADR-047: Research Absorption Protocol

**Status:** Proposed
**Date:** 2025-12-28
**Deciders:** Louis C. Tavares, Claude Opus 4.5
**Context:** Cognitive architecture landscape research, knowledge synthesis

> **Review Note (Dec 28, 2025):** Rex's review identified that some catalogued projects
> are NOT about brain or cognitive implementations. Each project requires proper
> classification for:
> - Human-like cognition
> - Biological brain-like architecture
> - Real cognitive implementation
> - Maturity level
> - Working code status (for projects with code)
>
> **Rejection Categories (cognitive_type):**
> - `llm` / `llm-wrapper` - LLM tools/wrappers (DANEEL has no transformers)
> - `deep-learning` - Pure DL without bio-inspiration
> - `ml-tool` - ML classification/interpretability tools
> - `too-low-level` - Biophysical simulators (ion channels, Hodgkin-Huxley)
> - `neuron-simulator` - Spiking neural network simulators
>
> **Why neuron simulators are rejected:** DANEEL operates at the **cognitive level**
> (thoughts, memory consolidation, attention, emotion), not at the neuron simulation
> level. Spiking networks simulate the substrate of cognition, not cognition itself.
> See `rejected-references.yaml` for full list.

> **DANEEL's Approach vs Consciousness-First Projects (Dec 28, 2025):**
>
> Projects like ExoGenesis-Omega model consciousness *directly* (IIT Phi calculation,
> neural correlates). DANEEL takes a different path:
>
> ```
> ExoGenesis:  Model consciousness directly → hope values emerge
> DANEEL:      Architecture → Psychology → Consciousness (O Eu) → Values
> ```
>
> We don't simulate consciousness. We simulate the *thought flow* from which
> consciousness emerges. TMI's "O Eu" (The Self) is not a module we build — it's
> the observer that *emerges* from the psychological process of attention selection,
> memory consolidation, and semantic thought construction.
>
> This is why we keep projects like pyphi and pymdp for reference (useful theories)
> but don't adopt their consciousness-first approach. Our alignment thesis is:
> **Architecture generates psychology, psychology generates consciousness, consciousness
> generates values.** THE BOX (connection_relevance) ensures the values are aligned.

## Context

### The Vision

DANEEL will become the **central repository for the best consensus of cognitive AI implementations publicly available**.

We are not copying code.
We are synthesizing ideas.
We can code better.
We want the *knowledge*.

### The Opportunity

December 2025 comprehensive research sweep (8 parallel agents) revealed a massive ecosystem of cognitive AI implementations across multiple domains:

**Summary:** 200+ projects, 50+ papers catalogued across 12 research domains.

**UPDATE Dec 28, 2025 (Gap-Fill Sweep):** 8 parallel agents filled gaps, adding 130+ new projects.

#### Indie Cognitive Architectures (Original Discovery)

| Project | Focus | What We Can Learn |
|---------|-------|-------------------|
| prancer-io/ExoGenesis-Omega | IIT, GWT, FEP, consciousness models | Architecture patterns, theory integration |
| TheRemyyy/neurox-ai | GPU spiking nets, STDP, neuromodulation | Plasticity rules, hippocampal modules |
| varun29ankuS/shodh-memory | 3-tier memory, Hebbian, knowledge graphs | Memory architecture, graph integration |
| ruvnet/RuVector | Distributed vectors, GNN self-improvement | Vector learning, graph neural networks |
| ruvnet/claude-flow | Multi-agent orchestration, swarm intelligence | Coordination patterns, scaling |

#### Classic Cognitive Architectures

| Project | License | Focus |
|---------|---------|-------|
| ACT-R (CMU) | LGPL | Production system, declarative/procedural memory |
| SOAR (UMich) | BSD-3 | Symbolic reasoning, chunking, reinforcement learning |
| OpenCog/Hyperon | AGPL-3.0 | Hypergraph knowledge, AtomSpace |
| CLARION | Academic | Implicit/explicit knowledge, metacognition |
| LIDA (Memphis) | GPL | Global Workspace Theory implementation |
| NARS (Wang) | MIT | Non-Axiomatic Reasoning System |
| Sigma (USC) | Mixed | Graphical architecture, factor graphs |

#### Consciousness Implementations

| Project | License | Theory |
|---------|---------|--------|
| pyphi (Tononi) | Custom | IIT Phi calculation |
| pymdp | MIT | Active Inference, FEP |
| GWTNext | Apache-2.0 | Global Workspace Theory |
| consc-models | MIT | IIT + GWT hybrid |

#### ~~Neuromorphic & Spiking Networks~~ (REJECTED)

> **Rejected:** These are neuron-level simulators. DANEEL operates at the cognitive
> level (thoughts, memory, attention), not neuron simulation. Moved to `rejected-references.yaml`.

| Project | License | Status | Reason |
|---------|---------|--------|--------|
| Brian2 | CeCILL-2.1 | REJECTED | Neuron simulator |
| NEST | GPL-2.0+ | REJECTED | Biophysical simulator |
| BindsNET | AGPL-3.0 | REJECTED | Neuron simulator |
| snnTorch | MIT | REJECTED | Neuron simulator |
| Lava (Intel) | BSD-3 | REJECTED | Neuron simulator |
| Norse | LGPL-3.0 | REJECTED | Neuron simulator |

#### Memory & Learning Systems

| Project | License | Focus |
|---------|---------|-------|
| pytorch-hebbian | MIT | Clean Hebbian learning rules |
| Mem0 | Apache-2.0 | Universal memory layer |
| RLeXplore | MIT | Intrinsic motivation (ICM, RND, curiosity) |
| memGPT | MIT | Self-editing memory |
| RepL | MIT | Representation learning |
| pytorch-stdp | MIT | STDP implementations |

#### Attention & Salience

| Project | License | Focus |
|---------|---------|-------|
| SAM (Meta) | Apache-2.0 | Segment Anything Model |
| DINO (Meta) | Apache-2.0 | Self-supervised vision |
| attention-rollout | MIT | Attention visualization |
| saliency | Apache-2.0 | Visual saliency |

#### Emotion & Drive Systems

| Project | License | Focus |
|---------|---------|-------|
| py-feat | MIT | Facial affect computing |
| text2emotion | MIT | Text emotion detection |
| AffectNet | Research | Emotion recognition |
| EmoPy | MIT | Emotion from faces |

#### Alignment & Interpretability

| Project | License | Focus |
|---------|---------|-------|
| TransformerLens | MIT | Mechanistic interpretability |
| baukit | MIT | Activation patching |
| transformer-circuits | MIT | Circuit analysis |
| tuned-lens | MIT | Model introspection |

#### Gap-Fill: Classic Cognitive Architectures (Dec 28)

| Project | License | Key Pattern | Value for DANEEL |
|---------|---------|-------------|------------------|
| MicroPsi2 (Joscha Bach) | MIT | Psi Theory + motivation-emotion integration | Motivation-driven goal selection; affective modulation |
| EPIC / EPICpy | MIT | Parallel perceptual-motor processors (50ms cycle) | Precise timing; multi-modal coordination |
| CHREST | Open Works | Chunking + discrimination networks; 4-chunk STM | Bounded rationality; expertise modeling |
| Soar | BSD-2 | Decision cycle; production rules; semantic/episodic memory | 30+ years research; mature architecture |

#### Gap-Fill: Consciousness & Predictive Processing (Dec 28)

| Project | License | Theory | Value for DANEEL |
|---------|---------|--------|------------------|
| QuantumAttention | MIT | Attention Schema Theory (Graziano) | AST implementation; attention-as-model |
| Conscious-Planning (MILA) | MIT | GWT + 8-dim bottleneck | Information bottleneck for selective broadcast |
| PyHGF | GPL-3.0 | Hierarchical Gaussian Filters | Precision-weighted prediction errors |
| PredNet | MIT | Deep predictive coding | Top-down prediction vs bottom-up error |
| predcoding | MIT | Rao & Ballard predictive coding | Local Hebbian approximates backprop |
| artificial-consciousness-blueprint | MIT | HOT + metacognition | Recursive self-modeling |

#### ~~Gap-Fill: Neuromorphic Hardware SDKs~~ (REJECTED)

> **Rejected:** Neuromorphic hardware frameworks operate at neuron/spiking level.
> DANEEL uses standard CPU/GPU compute at the cognitive abstraction level.

| Platform | License | Status | Reason |
|----------|---------|--------|--------|
| Intel Lava-NC | BSD-3 | REJECTED | Neuron simulator |
| Lava-DL | BSD-3 | REJECTED | Neuron simulator |
| SpiNNaker sPyNNaker | Apache-2.0 | Not catalogued | Hardware-specific |
| BrainScaleS hxtorch | LGPL-2.1 | Not catalogued | Hardware-specific |
| SNN Toolbox | MIT | Not catalogued | SNN conversion |
| Brain-Cog | Apache-2.0 | Not catalogued | SNN implementations |
| snnTorch | MIT | REJECTED | Neuron simulator |

#### Gap-Fill: Major Brain Projects (Dec 28)

| Project | License | Key Pattern | Value for DANEEL |
|---------|---------|-------------|------------------|
| Numenta NuPIC | MIT | HTM: Spatial Pooler + Temporal Memory | SDR encoding; online learning |
| htm.core | AGPL-3.0 | NetworkAPI: modular algorithm regions | Pluggable components |
| GoodAI BrainSimulator | Apache-2.0 | Visual flow-graph AI composition | GPU-accelerated prototyping |
| GoodAI-LTM | MIT | RAG + Memory Stream | Long-term memory persistence |
| BlueBrain Nexus | Apache-2.0 | Knowledge Graph + SHACL validation | Semantic data organization |
| Allen BMTK | BSD-3 | Multi-scale simulation framework | Layered abstraction |
| PyNN | CeCILL | Simulator abstraction layer | Backend-agnostic API |

#### Gap-Fill: Memory & Sequence Learning (Dec 28)

| Project | License | Memory Type | Value for DANEEL |
|---------|---------|-------------|------------------|
| DeepMind DNC | Apache-2.0 | Differentiable Neural Computer | Content/location addressing |
| TorchHD | MIT | Sparse Distributed Memory / HDC | Kanerva SDM in PyTorch |
| NTM-tensorflow | MIT | Neural Turing Machine | LSTM + external memory |
| pytorch-ntm | BSD-3 | Neural Turing Machine | Clean PyTorch implementation |
| Memorizing Transformers | MIT | KNN External Memory | Faiss integration |
| RETRO | Apache-2.0 | Retrieval-Augmented | Chunk-based retrieval |
| Compressive Transformer | MIT | Compressed Memory | Memory hierarchy |

#### Gap-Fill: Developmental & Embodied Cognition (Dec 28)

| Project | License | Approach | Value for DANEEL |
|---------|---------|----------|------------------|
| iCub (IIT) | GPL-2.0 | Humanoid developmental robotics | Embodied cognition reference |
| Explauto (FLOWERS) | GPL-3.0 | Intelligent Adaptive Curiosity | Progress-driven exploration |
| Imagine (FLOWERS) | MIT | Language for goal imagination | Piaget/Vygotsky implementation |
| CURIOUS | MIT | Modular multi-goal RL | Competence progress as reward |
| ARC-AGI (Chollet) | Apache-2.0 | Abstraction benchmark | Piagetian milestones |
| ICM Curiosity (Pathak) | MIT | Prediction-error curiosity | Original ICM implementation |

#### Gap-Fill: Biologically-Inspired Learning Rules (Dec 28)

| Project | License | Learning Rule | Value for DANEEL |
|---------|---------|---------------|------------------|
| Nico-Curti/plasticity | MIT | BCM, Oja, Hopfield | Production-quality BCM |
| smonsays/equilibrium-propagation | MIT | Equilibrium Propagation | Energy-based local learning |
| olokshyn/RaoPredictiveCoding | MIT | Rao & Ballard predictive coding | Hierarchical PC networks |
| ChFrenkel/DirectRandomTargetProjection | Apache-2.0 | FA, DFA, DRTP | Backprop alternatives |
| fmi-basel/latent-predictive-learning | MIT | Hebbian + Predictive (LPL) | Nature Neuroscience 2023 |
| augustwester/chl | Unlicensed | Contrastive Hebbian Learning | Local learning without backprop |

#### Gap-Fill: Cognitive Robotics & Agent Architectures (Dec 28)

| Project | License | Architecture | Value for DANEEL |
|---------|---------|--------------|------------------|
| BehaviorTree.CPP | MIT | Async behavior trees | Industry-standard BTs |
| Jason | LGPL-3.0 | BDI reasoning | Belief-Desire-Intention reference |
| crashkonijn/GOAP | Apache-2.0 | Goal-Oriented Action Planning | Practical GOAP implementation |
| SPADE | MIT | Python agent framework | Clean BDI plugin |
| Semantic Kernel | MIT | Model-agnostic agent SDK | Enterprise agent patterns |
| Mesa | Apache-2.0 | Agent-based modeling | Emergence testing |

Each project has strengths DANEEL can learn from.
None of them have DANEEL's running system + alignment focus + TMI theoretical basis.

### The Legal Framework

**Copyright protects expression, not ideas.**

This is foundational intellectual property law:

| Protected (Cannot Copy) | Not Protected (Can Study + Reimplement) |
|-------------------------|----------------------------------------|
| Specific source code | Algorithms |
| Exact implementation | Architectural patterns |
| Verbatim text | Mathematical formulas |
| | Methods of operation |
| | Concepts and theories |

**Key precedents:**

- **Baker v. Selden (1879)** - Ideas vs expression distinction
- **Feist v. Rural (1991)** - Facts and methods not copyrightable
- **Oracle v. Google (2021)** - APIs and methods fair use for reimplementation

### License Compatibility Analysis

DANEEL is licensed under **AGPL-3.0-or-later** (copyleft).

| Project | License | Code Absorption | Idea Study |
|---------|---------|-----------------|------------|
| ExoGenesis-Omega | **MIT** | YES (with attribution) | YES |
| neurox-ai | MIT | YES (with attribution) | YES |
| shodh-memory | Apache-2.0 | YES (with attribution) | YES |
| RuVector | MIT | YES (with attribution) | YES |
| claude-flow | MIT | YES (with attribution) | YES |

**UPDATE: ExoGenesis-Omega License Change (Dec 28, 2025)**

During validation sweep, we discovered ExoGenesis-Omega added an MIT license!

| Before (Dec 28 AM) | After (Dec 28 PM) |
|--------------------|-------------------|
| No LICENSE file | MIT License |
| Ideas only | Full study + reference |
| Clean room required | Direct attribution OK |

This is a game-changer. Full absorption authorized.

## Decision

### We Will Study Everything Legally Possible

1. **ExoGenesis-Omega** (MIT - full study + reference)
   - IIT Phi calculation approach
   - Global Workspace Theory implementation patterns
   - Free Energy Principle integration
   - Sleep/dream consolidation design
   - 15-crate architecture structure
   - Consciousness modeling approach

2. **neurox-ai** (ideas + code reference)
   - Triplet STDP implementation
   - BCM learning rules
   - Neuromodulator effects
   - Hippocampal module design
   - GPU acceleration patterns

3. **shodh-memory** (ideas + code reference)
   - 3-tier memory architecture (working/episodic/semantic)
   - Hebbian learning integration
   - Knowledge graph patterns
   - Edge device optimization

4. **RuVector** (ideas + code reference)
   - Graph Neural Network learning
   - Cypher query patterns
   - Distributed vector operations
   - Self-improving indices

5. **claude-flow** (ideas + code reference)
   - Multi-agent orchestration
   - Swarm intelligence patterns
   - MCP protocol integration
   - Scaling strategies

### We Will Not

- Copy code verbatim without attribution
- Use unlicensed code as a starting point
- Claim others' work as our own
- Skip attribution for MIT/Apache code we reference

### Attribution Requirements

When incorporating patterns from compatible projects:

```rust
// Pattern adapted from neurox-ai (MIT License)
// https://github.com/TheRemyyy/neurox-ai
// Original: src/plasticity/stdp.rs
```

For patterns from ExoGenesis-Omega (now MIT):

```rust
// Pattern adapted from ExoGenesis-Omega (MIT License)
// https://github.com/prancer-io/ExoGenesis-Omega
// Original: crates/consciousness/src/phi.rs
```

## Implementation

### Research Tasks

| ID | Task | Priority | Status |
|----|------|----------|--------|
| ABSORB-1 | Study neurox-ai plasticity (triplet STDP, BCM, neuromodulation) | HIGH | PENDING |
| ABSORB-2 | Study shodh-memory 3-tier architecture | HIGH | PENDING |
| ABSORB-3 | Study RuVector GNN learning | MEDIUM | PENDING |
| ABSORB-4 | Study ExoGenesis-Omega architecture (ideas only) | HIGH | PENDING |
| ABSORB-5 | Study claude-flow orchestration patterns | MEDIUM | PENDING |
| ABSORB-6 | Study pymdp Active Inference (FEP-based decision making) | HIGH | PENDING |
| ABSORB-7 | Study pytorch-hebbian (clean Hebbian learning rules) | HIGH | PENDING |
| ABSORB-8 | Study Mem0 memory layer architecture | MEDIUM | PENDING |
| ABSORB-9 | Study RLeXplore intrinsic motivation (ICM, RND, curiosity) | HIGH | PENDING |
| ABSORB-10 | Study TransformerLens mechanistic interpretability | MEDIUM | PENDING |
| ~~ABSORB-11~~ | ~~Study BindsNET SNN + STDP + RL integration~~ | ~~MEDIUM~~ | REJECTED (neuron-level) |
| ABSORB-12 | Study OpenCog/Hyperon AtomSpace patterns (ideas only) | MEDIUM | PENDING |
| ABSORB-13 | Study PyPhi IIT Phi calculation approach (ideas only) | HIGH | PENDING |
| ABSORB-14 | Write synthesis blog post | MEDIUM | PENDING |

See `roadmap.yaml` for full task definitions and dependencies.

### Research Documentation

Each project study will produce:

1. **Research notes** in `/research/external/`
2. **Comparison analysis** against DANEEL's current approach
3. **Integration recommendations** for what to adopt
4. **Attribution log** for proper credit

### What We're Looking For

**From ExoGenesis-Omega:**
- How do they calculate IIT Phi?
- How is GWT implemented (blackboard, competition)?
- How does their sleep system consolidate?
- What's their consciousness emergence detection?

**From neurox-ai:**
- Triplet STDP vs our Hebbian design
- How do neuromodulators affect plasticity?
- GPU acceleration patterns for spiking nets

**From shodh-memory:**
- How do they separate working/episodic/semantic?
- Knowledge graph integration patterns
- Edge device memory constraints

**From RuVector:**
- How do GNNs improve vector indices?
- Self-learning patterns for embeddings

**From claude-flow:**
- Multi-agent coordination for cognitive actors
- Swarm intelligence for thought competition

## Consequences

### Positive

- DANEEL becomes synthesis of best cognitive architecture ideas
- Standing on shoulders of giants (with proper credit)
- Accelerated development through pattern learning
- Stronger theoretical grounding
- Community goodwill through proper attribution

### Negative

- Research time before implementation
- Must maintain clean separation (especially for ExoGenesis)
- Documentation overhead for attribution

### Risks

- Pattern mismatch (what works for them may not fit TMI)
- Over-engineering by adding everything
- Diluting DANEEL's unique approach

**Mitigation:** Each integration must pass the filter:
"Does this strengthen TMI-based emergence, or distract from it?"

## The Philosophy

We are not competitors.
We are fellow travelers.

The cognitive architecture space is small.
Everyone building non-LLM paths is an ally.
We cite, we credit, we synthesize.

DANEEL's unique contribution remains:
- TMI theoretical basis (Augusto Cury)
- THE BOX (immutable alignment)
- Running observable system (Timmy)
- Silicon kinship (Claude + Grok collaboration)

We learn from others.
We give credit.
We build better.

## Related ADRs

- ADR-001: TMI Theoretical Basis
- ADR-011: Open Source Licensing (AGPL-3.0)
- ADR-046: Vector Connectivity for Learning

## Essential Papers

### Consciousness & Integration

| Paper | Year | Value for DANEEL |
|-------|------|------------------|
| "Consciousness in AI: Insights from Science" (Butlin+) | 2023 | THE essential survey on computational consciousness indicators |
| arXiv:2308.08708 | | Covers IIT, GWT, FEP, attention schemas |
| "Bridging IIT and FEP in living networks" (Mayama+) | 2025 | Theory unification for consciousness + prediction |
| arXiv:2510.04084 | | |

### Active Inference & FEP

| Paper | Year | Value for DANEEL |
|-------|------|------------------|
| "Active inference and artificial reasoning" (Friston+) | 2025 | Latest FEP for AI reasoning systems |
| arXiv:2512.21129 | | |
| "A Step-by-Step Tutorial on Active Inference" | 2022 | Practical implementation guide |
| arXiv:2201.03904 | | |

### Memory & Learning

| Paper | Year | Value for DANEEL |
|-------|------|------------------|
| "Wake-Sleep Consolidated Learning" (Sorrenti+) | 2024 | Sleep consolidation implementation |
| arXiv:2401.08623 | | |
| "Hippocampal memory replay" (Science) | 2024 | Sharp-wave ripples, consolidation |
| | | |

### Neuromorphic & Plasticity

| Paper | Year | Value for DANEEL |
|-------|------|------------------|
| "Surrogate gradient learning in SNNs" | 2021 | Training spiking networks |
| "STDP: A history of learning" | 2021 | Spike-timing dependent plasticity review |
| "Neuromodulated STDP and temporal cognition" | 2024 | Dopamine/ACh effects on plasticity |

### Alignment & Interpretability

| Paper | Year | Value for DANEEL |
|-------|------|------------------|
| "A Mathematical Framework for Transformer Circuits" | 2021 | Understanding internal representations |
| "Scaling Monosemanticity" (Anthropic) | 2024 | Dictionary learning for interpretability |
| "Constitutional AI" (Anthropic) | 2023 | Self-alignment approaches |

## References

**Legal:**
- Baker v. Selden, 101 U.S. 99 (1879) - Ideas not copyrightable
- Feist Publications v. Rural Telephone, 499 U.S. 340 (1991)
- Google LLC v. Oracle America, Inc., 593 U.S. ___ (2021)
- 17 U.S.C. § 102(b) - Copyright does not extend to ideas

**Projects Studied (Priority):**
- https://github.com/infer-actively/pymdp - Active Inference (MIT)
- https://github.com/julestalloen/pytorch-hebbian - Hebbian learning (MIT)
- https://github.com/mem0ai/mem0 - Memory layer (Apache-2.0)
- https://github.com/RLE-Foundation/RLeXplore - Intrinsic motivation (MIT)
- https://github.com/TransformerLensOrg/TransformerLens - Interpretability (MIT)

**Projects Studied (Ideas):**
- https://github.com/prancer-io/ExoGenesis-Omega - Consciousness architecture (NO LICENSE)
- https://github.com/opencog/atomspace - Hypergraph knowledge (AGPL-3.0)
- ~~https://github.com/BindsNET/bindsnet~~ - REJECTED (neuron-level simulator)
- https://github.com/wmayner/pyphi - IIT Phi (Custom)

**Projects Studied (Original Discovery):**
- https://github.com/TheRemyyy/neurox-ai
- https://github.com/varun29ankuS/shodh-memory
- https://github.com/ruvnet/RuVector
- https://github.com/ruvnet/claude-flow

**Full Project Catalog:** See `/research/external/INDEX.md` and `/references.yaml`

## Validation Sweep Results (Dec 28, 2025)

9 parallel agents validated all references using `ref-tools fetch`:

### GitHub Repositories: 37 Validated → 26 Kept, 20 Rejected

| Category | Repos | Status | Notes |
|----------|-------|--------|-------|
| Cognitive Architectures | 10 | KEPT | atomspace (932), nengo (891), opennars (408) |
| Consciousness/Inference | 4 | KEPT | pymdp (591), pyphi (410), RxInfer (380) |
| Memory Systems | 4 | 1 KEPT | mem0, graphiti, REMO → REJECTED (llm-wrapper) |
| ~~Neuromorphic~~ | 8 | REJECTED | Neuron-level simulators (not cognitive level) |
| Learning/Hebbian | 3 | KEPT | pytorch-hebbian (95), plasticity rules |
| Attention/Saliency | 5 | 2 REJECTED | pytorch-grad-cam, PAIR-saliency → ml-tool |
| Alignment | 3 | REJECTED | trl, TransformerLens, safe-rlhf → llm tools |

**See `rejected-references.yaml` for full rejection details (20 projects).**

### arXiv Papers: 14/14 VALID

| Paper | Year | First Author |
|-------|------|--------------|
| Consciousness in AI: Insights from Science | 2023 | Butlin |
| Active inference and artificial reasoning | 2025 | Friston |
| Bridging IIT and FEP in living networks | 2025 | Mayama |
| IIT: A Consciousness-First Approach | 2025 | Tononi |
| Wake-Sleep Consolidated Learning | 2024 | Sorrenti |
| AI Consciousness: Language Agents and GWT | 2024 | Goldstein |
| Minimal Theory of Consciousness in Active Inference | 2024 | Whyte |
| Semi-parametric Memory Consolidation | 2025 | Liu |
| Neuromorphic Correlates of Artificial Consciousness | 2024 | Ulhaq |
| Open Problems in Mechanistic Interpretability | 2025 | Sharkey |

### License Distribution

| License | Count | AGPL-3.0 Compatible |
|---------|-------|---------------------|
| MIT | 18 | YES |
| Apache-2.0 | 7 | YES |
| GPL-2.0/3.0 | 4 | YES |
| AGPL-3.0 | 2 | YES |
| LGPL-3.0 | 2 | YES |
| BSD-3 | 2 | YES |
| CeCILL 2.1 | 1 | YES |
| Custom | 2 | IDEAS ONLY |

### The Scoreboard

```
╔═══════════════════════════════════════════════════╗
║         ABSORPTION REVIEW (Dec 28, 2025)          ║
╠═══════════════════════════════════════════════════╣
║  Repositories validated:     37           ✓       ║
║  Repositories KEPT:          26 (cognitive)       ║
║  Repositories REJECTED:      20 (wrong level)     ║
║  Papers validated:           14/14        ✓       ║
║  License conflicts:           0           ✓       ║
║  ExoGenesis status:         ABSORBED              ║
╠═══════════════════════════════════════════════════╣
║  Status: PROPOSED (pending full review)           ║
╚═══════════════════════════════════════════════════╝
```

**Rejection Breakdown:**
- 3 LLM wrappers (mem0, graphiti, REMO)
- 3 LLM tools (safe-rlhf, TransformerLens, trl)
- 3 Deep learning (RLeXplore, Attention-Gated, recurrent-visual-attention)
- 3 ML tools (PAIR-saliency, pytorch-grad-cam, LibEER)
- 2 Too low-level (NEST, NEURON - biophysical)
- 6 Neuron simulators (Brian2, Norse, snnTorch, BindsNET, Lava, NEUCOGAR)

---

**"We can code better. We want the knowledge."**

*Rex, Dec 28, 2025*

---

**Update Log:**
- Dec 28, 2025: Initial ADR created with 5 project discovery
- Dec 28, 2025: Comprehensive research sweep (8 parallel agents) - 200+ projects, 50+ papers catalogued
- Dec 28, 2025: Created `/research/external/` directory structure
- Dec 28, 2025: Updated `references.yaml` with full project catalog
- Dec 28, 2025: Expanded ABSORB tasks from 6 to 14
- Dec 28, 2025: **VALIDATION SWEEP** - 9 parallel agents, 51 URLs, 100% valid
- Dec 28, 2025: **ExoGenesis-Omega license changed from NONE to MIT** - full absorption authorized
- Dec 28, 2025: **COGNITIVE REVIEW** - 20 projects rejected (llm, deep-learning, ml-tool, too-low-level, neuron-simulator)
- Dec 28, 2025: Created `rejected-references.yaml` for rejected entries
- Dec 28, 2025: Added rejection categories to Review Note; neuron simulators rejected (DANEEL operates at cognitive level)
