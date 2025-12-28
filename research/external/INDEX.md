# External Research Index

**Created:** 2025-12-28
**Origin:** Rex + Claude Opus 4.5 research sweep
**ADR:** ADR-047-research-absorption-protocol.md

## Purpose

DANEEL becomes the **central repository for the best consensus of cognitive AI implementations publicly available**.

We study ideas.
We cite sources.
We implement better.

## License Compatibility

DANEEL is AGPL-3.0. Compatible licenses for code absorption:

| License | Compatible | Notes |
|---------|------------|-------|
| MIT | YES | Permissive, must attribute |
| Apache-2.0 | YES | Permissive, patent grant |
| BSD-2/3 | YES | Permissive |
| GPL-3.0 | YES | Copyleft family |
| LGPL-3.0 | YES | Weak copyleft |
| AGPL-3.0 | YES | Same license |
| GPL-2.0+ | YES | Can upgrade |
| CeCILL-2.1 | YES | GPL-compatible |
| No License | IDEAS ONLY | All rights reserved |

## Directory Structure

```
research/external/
├── INDEX.md                    # This file
├── cognitive-architectures/    # ACT-R, SOAR, OpenCog, CLARION, etc.
├── consciousness/              # IIT, GWT, FEP, Active Inference
├── memory-learning/            # Hebbian, STDP, episodic, semantic
├── neuromorphic/               # Spiking networks, Brian2, NEST
├── papers/                     # Key scientific papers
├── emotion-drives/             # Affective computing, intrinsic motivation
├── attention-salience/         # Visual attention, saliency, WTA
└── alignment/                  # RLHF, DPO, interpretability
```

## Top Priority Projects

### Immediate Integration (MIT/Apache/BSD)

1. **pymdp** (MIT) - Active Inference for drives/motivation
   - https://github.com/infer-actively/pymdp
   - Value: FEP-based decision making, belief updating

2. **pytorch-hebbian** (MIT) - Clean Hebbian learning
   - https://github.com/julestalloen/pytorch-hebbian
   - Value: Local learning rules for DANEEL

3. **Mem0** (Apache-2.0) - Universal memory layer
   - https://github.com/mem0ai/mem0
   - Value: Multi-level memory architecture patterns

4. **RLeXplore** (MIT) - Intrinsic motivation
   - https://github.com/RLE-Foundation/RLeXplore
   - Value: ICM, RND for curiosity-driven exploration

5. **TransformerLens** (MIT) - Mechanistic interpretability
   - https://github.com/TransformerLensOrg/TransformerLens
   - Value: Understanding internal representations

### Study for Ideas (GPL/AGPL/Custom/None)

1. **Hyperon/AtomSpace** (AGPL-3.0) - Hypergraph knowledge
   - https://github.com/opencog/atomspace
   - Value: Knowledge representation patterns

2. **BindsNET** (AGPL-3.0) - SNN + STDP + RL
   - https://github.com/BindsNET/bindsnet
   - Value: Biologically plausible learning

3. **PyPhi** (Custom) - IIT Phi calculation
   - https://github.com/wmayner/pyphi
   - Value: Consciousness measurement approach

4. **ExoGenesis-Omega** (NO LICENSE) - IDEAS ONLY
   - https://github.com/prancer-io/ExoGenesis-Omega
   - Value: Architecture patterns, IIT/GWT/FEP integration

## Essential Papers

1. "Consciousness in AI: Insights from Science" (Butlin+ 2023)
   - arXiv:2308.08708
   - THE essential survey on computational consciousness

2. "Wake-Sleep Consolidated Learning" (Sorrenti+ 2024)
   - arXiv:2401.08623
   - Sleep consolidation implementation

3. "Active inference and artificial reasoning" (Friston+ 2025)
   - arXiv:2512.21129
   - Latest FEP for AI reasoning

4. "Bridging IIT and FEP in living networks" (Mayama+ 2025)
   - arXiv:2510.04084
   - Theory unification

## Research Tasks

See roadmap.yaml for ABSORB-1 through ABSORB-7 tasks.

## Attribution Template

When incorporating patterns:

```rust
// Pattern adapted from [PROJECT] ([LICENSE])
// https://github.com/[REPO]
// Original: [FILE]
// See: ADR-047 for legal basis
```

---

*"We can code better. We want the knowledge."*
— Rex, Dec 28, 2025
