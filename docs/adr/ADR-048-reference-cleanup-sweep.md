# ADR-048: Reference Cleanup Sweep

## Status
ACCEPTED

## Date
2025-12-28

## Deciders
- Rex (Louis C. Tavares)
- Claude Opus 4.5

## Context

ADR-047 established the psychology-first vs brain-first distinction for DANEEL's research references. During that work, we classified external projects but did not systematically review our own `references.yaml` file (196 entries).

Many references were added during early research phases before the psychology-first approach was crystallized. Some may be:
- Brain-level (neuron simulation, biophysical models)
- Consciousness-first (IIT Phi, direct consciousness modeling)
- Too low-level (neuromorphic hardware, spiking networks)
- LLM/ML tools (not cognitive architecture)

## Decision

Perform a systematic sweep of all 196 references using parallel agents with `ref-tools`:

1. **Classification Schema**
   - `psychology`: Thought-flow, cognitive architecture, attention, memory, emotion → KEEP
   - `brain`: Neuron simulation, biophysical models, spiking networks → REJECT
   - `consciousness-first`: IIT Phi, direct consciousness modeling → REJECT
   - `theory`: Foundational papers, cognitive science theories → KEEP
   - `infrastructure`: Tools, platforms, languages → KEEP
   - `llm-tool`: LLM wrappers, ML utilities → REJECT

2. **Execution**
   - 8 parallel agents
   - Each agent processes ~25 references
   - Uses `ref-tools fetch <url>` for classification
   - Returns: URL, title, classification, recommendation (KEEP/REJECT)

3. **Consolidation**
   - Merge agent results
   - Move rejected to `rejected-references.yaml`
   - Update `references.yaml`

## Classification Criteria

| KEEP (Psychology-First) | REJECT (Brain-First) |
|------------------------|---------------------|
| Cognitive architectures (SOAR, ACT-R) | Neuron simulators (Brian2, NEST) |
| Global Workspace Theory | Spiking networks (detailed) |
| Executive attention | IIT Phi calculation |
| Memory consolidation | FEP prediction error |
| Spreading activation | Biophysical models |
| Emotion/affect models | Neuromorphic hardware |
| Symbolic reasoning | LLM wrappers |

## Consequences

### Positive
- Clean, focused reference list
- Consistent with psychology-first approach
- Easier to maintain and cite

### Negative
- Some interesting brain-level work moves to rejected
- Requires agent compute time

## Results

Sweep completed with corrections. 48 references removed, 148 retained.

### Scoreboard (CORRECTED)
```
╔═══════════════════════════════════════════════════════════╗
║              REFERENCE CLEANUP SWEEP                       ║
╠═══════════════════════════════════════════════════════════╣
║  Total references:         196                             ║
║  Agents deployed:          8 (+ 1 manual batch)            ║
║  Initial removal:          76                              ║
║  Restored (correction):    28                              ║
║  FINAL KEPT:               148                             ║
║  FINAL REJECTED:           48                              ║
║  Status:                   COMPLETE (CORRECTED)            ║
╚═══════════════════════════════════════════════════════════╝
```

### Correction Notice

Initial agent classification wrongly rejected references that are core to DANEEL:

1. **THE BOX references** (5 restored):
   - Legal personhood (Stanford Encyclopedia, Santa Clara case)
   - Robot rights books (MIT Press)
   - AI ethics (Oxford)
   - Te Awa Tupua Act (NZ legal personhood precedent)

   **Reason:** THE BOX is DANEEL's alignment mechanism. Legal/ethics research
   supports the Crystal Laws and Asimov foundation.

2. **ADR-038 criticality papers** (23 restored):
   - Avalanche dynamics, DFA, power spectrum
   - Pink noise injection techniques
   - Edge-of-chaos measurement

   **Reason:** These papers are cited in ADR-038 for measuring fractality
   in DANEEL's vectorial/embedding space - NOT brain simulation. Pink noise
   injection and fractality measurement are Phase 2 requirements.

### Classification Breakdown (Corrected)

| Type | Count | Action |
|------|-------|--------|
| neuromorphic hardware (Intel, IBM, DARPA, KAIST) | 12 | REJECTED |
| off-topic (policy, navigation, news) | 10 | REJECTED |
| consciousness-first (IIT direct, entropic brain) | 8 | REJECTED |
| brain (place cells, hippocampal neurons) | 8 | REJECTED |
| llm-tool (LLM emergent abilities, SAE protein) | 4 | REJECTED |
| dead-link (404, inaccessible) | 3 | REJECTED |
| personal (profiles, ORCID) | 3 | REJECTED |

### Key Distinctions

**KEPT (psychology-first + alignment):**
- THE BOX ethics/personhood → alignment foundation
- ADR-038 criticality → fractality measurement (vectorial, not brain)
- Pink noise, DFA, avalanche → Phase 2 emergence metrics

**REJECTED (brain-first):**
- Neuromorphic hardware (simulating neurons in silicon)
- Place cells/hippocampal neurons (brain-level, not cognitive)
- IIT Phi direct calculation (consciousness-first, not psychology-first)

### Files Updated

- `references.yaml`: 196 → 148 entries
- `rejected-references.yaml`: 28 → 76 entries

## Update Log

| Date | Change |
|------|--------|
| 2025-12-28 | ADR created, agent swarm launched |
| 2025-12-28 | All 8 agents completed, 1 batch required manual classification |
| 2025-12-28 | Initial consolidation: 76 removed, 120 kept |
| 2025-12-28 | **CORRECTION**: Restored 28 refs (THE BOX + ADR-038 criticality) |
| 2025-12-28 | Final: 48 removed, 148 kept, ADR ACCEPTED |
