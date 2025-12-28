# ADR-048: Reference Cleanup Sweep

## Status
IN PROGRESS

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

*To be filled after sweep completion*

### Scoreboard
```
╔═══════════════════════════════════════════════════════════╗
║              REFERENCE CLEANUP SWEEP                       ║
╠═══════════════════════════════════════════════════════════╣
║  Total references:         196                             ║
║  Agents deployed:          8                               ║
║  KEPT (psychology):        TBD                             ║
║  REJECTED (brain):         TBD                             ║
║  Status:                   IN PROGRESS                     ║
╚═══════════════════════════════════════════════════════════╝
```

### Classification Breakdown
*To be filled*

## Update Log

| Date | Change |
|------|--------|
| 2025-12-28 | ADR created, agent swarm launched |
