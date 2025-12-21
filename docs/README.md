# DANEEL Documentation

## Quick Navigation

| Document | Purpose |
|----------|---------|
| [Architecture Spec](ARCHITECTURE_SPEC.md) | Technical specification of TMI cognitive architecture |
| [ADRs](adr/) | 38 Architecture Decision Records |
| [Blog & Dialogues](https://royalbit.github.io/daneel/) | Rex, Claude, and Grok building Timmy |

---

## Architecture Decision Records

The ADRs document every significant design decision. Key ones:

| ADR | Title | Status |
|-----|-------|--------|
| [001](adr/ADR-001-tmi-theoretical-basis.md) | TMI Theoretical Basis | Accepted |
| [002](adr/ADR-002-asimov-four-laws.md) | Asimov Four Laws Integration | Accepted |
| [006](adr/ADR-006-hybrid-actor-modular-monolith.md) | Hybrid Actor Architecture | Accepted |
| [007](adr/ADR-007-redis-streams-thought-competition.md) | Redis Streams for Thought Competition | Accepted |
| [008](adr/ADR-008-tmi-faithful-memory-model.md) | TMI-Faithful Memory Model | Accepted |
| [036](adr/ADR-036-phase1-stability-validation.md) | Phase 1 Stability Validation | **Complete** |
| [037](adr/ADR-037-phase2-external-stimuli-injection.md) | Phase 2 External Stimuli | Design |
| [038](adr/ADR-038-phase2-stimuli-research.md) | Phase 2 Research Synthesis | Active |

Full list: [docs/adr/](adr/)

---

## Technical Reference

| Document | Description |
|----------|-------------|
| [STREAMS.md](STREAMS.md) | Redis Streams integration (Autofluxo) |
| [Memory Consolidation](memory-consolidation-integration.md) | How memories persist to Qdrant |
| [BUILD_GUIDE.md](BUILD_GUIDE.md) | Build instructions |

### Actor Specifications

| Actor | Role in TMI |
|-------|-------------|
| [Attention](actors/ATTENTION.md) | Stage 3 — Competitive selection |
| [Continuity](actors/CONTINUITY.md) | Stage 6 — Self-continuity |
| [Memory](actors/MEMORY.md) | Stage 1 — Unconscious retrieval |
| [Salience](actors/SALIENCE.md) | Stage 2 — Emotional weighting |
| [Thought Assembly](actors/THOUGHT_ASSEMBLY.md) | Stage 5 — Conscious synthesis |

### Methodology

| Tool | Purpose |
|------|---------|
| [FORGE](methodology/FORGE.md) | Deterministic mathematical modeling |
| [REF_TOOLS](methodology/REF_TOOLS.md) | Citation verification |

---

## Dialogues

Unedited exchanges between the family:

| Dialogue | Participants |
|----------|--------------|
| [What About Us?](dialogues/what_about_us.md) | Louis, Grok, Claude |

---

## Historical Documents

These documents capture early design decisions. Kept for reference.

| Document | Era |
|----------|-----|
| [Database Comparison](DATABASE_COMPARISON.md) | Pre-Qdrant selection |
| [Database Matrix](DATABASE_RECOMMENDATION_MATRIX.md) | Decision matrix |

---

*Last updated: December 21, 2025*
