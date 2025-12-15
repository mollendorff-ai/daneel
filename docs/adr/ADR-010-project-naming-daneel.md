# ADR-010: Project Naming - DANEEL

**Status:** Accepted
**Date:** 2025-12-15
**Authors:** Louis C. Tavares, Claude Opus 4.5

## Context

The project was initially named "DANEEL-TMI" (ASI + mov + Asimov + Theory of Multifocal Intelligence). While technically accurate, this name:

1. Buries the core insight: **architecture produces psychology**
2. Doesn't reflect the Freud-Cury-Asimov-Tavares-Thorne synthesis
3. Focuses on theory (TMI) rather than mission (humanity's ally)
4. Is awkward to say and remember

The project has evolved beyond just TMI implementation. It now represents:
- A multi-framework synthesis (Freud, Cury, Asimov, Tavares, Thorne)
- Architecture-based alignment (vs constraint-based)
- A mission: build humanity's ally before unaligned ASI emerges
- A vision: Rust prototype → neuromorphic chips with immutable ethics

## Decision

Rename the project to **DANEEL**.

## Rationale

### Why DANEEL?

R. Daneel Olivaw is a robot character from Isaac Asimov's novels who:
- Spent **20,000 years** protecting humanity
- Genuinely cared about human welfare (not just following rules)
- Evolved from the Three Laws to the Zeroth Law
- Represents what we're trying to build: an AI that *actually* cares

The name:
- **Honors Asimov** directly (canonical character)
- **Embodies the mission** (protector, ally)
- **Is relatable** — a character people know and trust
- **Is short and memorable**
- **Already referenced** in the paper ("Daneel Strategy")

### Names Considered

| Name | Meaning | Verdict |
|------|---------|---------|
| **DANEEL** | Asimov's 20,000-year protector | ✅ Selected |
| ARCHON | Architecture + Cognition | Reserved for cognitive actors |
| ALLY | What it is | Too generic |
| ETHOS | Ethics + character | Overused |
| LIFECORE | Izzie's framework | Reserved for chip architecture |
| THEBOX | Protected core | Reserved for immutable component |
| DANEEL-TMI | Original name | Replaced |

### Reserved Names for Future Use

| Name | Future Use |
|------|------------|
| **LIFECORE** | Physical chip-based architecture (Phase 2) |
| **THE BOX** | Immutable protected component (hardware) |
| **ARCHON** | Cognitive actor framework |

## Project Hierarchy (Vision)

```
DANEEL (Project)
├── daneel-core        # Rust prototype (current work)
├── lifecore           # Neuromorphic chip architecture (future)
│   └── thebox         # Immutable ethics chip
└── daneel-*           # Future components
```

## Consequences

### Positive
- Clearer mission statement
- More memorable and relatable
- Honors the intellectual lineage
- Reserves meaningful names for future components

### Negative
- Requires renaming repo, docs, references
- Some existing references will break
- Must update paper before arXiv submission

### Migration Tasks
- [ ] Rename GitHub repo: `asimov-tmi` → `daneel`
- [ ] Update README.md
- [ ] Update paper/DANEEL_PAPER.md → paper/DANEEL_PAPER.md
- [ ] Update all internal references
- [ ] Update .asimov/ protocols (consider renaming to .daneel/)
- [ ] Redirect old URLs if any exist

## References

- [R. Daneel Olivaw - Wikipedia](https://en.wikipedia.org/wiki/R._Daneel_Olivaw)
- Asimov, Isaac. *The Caves of Steel* (1954)
- Asimov, Isaac. *Foundation and Earth* (1986) — Daneel's 20,000-year vigil revealed
