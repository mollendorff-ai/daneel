# ADR-001: Theory of Multifocal Intelligence as Theoretical Basis

**Status:** Accepted
**Date:** 2025-12-13
**Deciders:** Louis C. Tavares, Claude Opus 4.5

## Context

AI alignment approaches are predominantly constraint-based (RLHF, Constitutional AI, safety filters).
These approaches train values after the fact rather than building them into cognitive architecture.

We needed a theoretical framework for human-like cognition that:

1. Explains how thoughts are constructed, not just expressed
2. Distinguishes pre-linguistic from semantic thought
3. Accounts for emotional coloring of cognition
4. Provides a model for persistent self ("I")

## Decision

Adopt Dr. Augusto Cury's Theory of Multifocal Intelligence (TMI) as the theoretical foundation.

**Key TMI concepts to implement:**

- Memory Windows (dynamic active/stored memory)
- The "I" as Manager (metacognitive navigation)
- Thought Construction (multi-input assembly)
- Emotional Coloring (affect shapes thought formation)

## Research Gap Evidence

| Search Query | Platform | Results |
|--------------|----------|---------|
| "multifocal intelligence" + repositories | GitHub | 0 |
| "multifocal intelligence" + computational | Google Scholar | 1 (unrelated) |
| "augusto cury" + artificial intelligence | Google Scholar | ~32 (no TMI implementations) |

**This is the first computational implementation of TMI.**

## Consequences

**Positive:**

- Novel approach unexplored in AI research
- Human-like architecture may produce human-like values
- Pre-linguistic thought enables values before language
- Rich theoretical foundation (30+ million books sold)

**Negative:**

- No prior implementations to reference
- TMI was designed for human therapy, not computation
- Requires significant translation work
- Untested hypothesis that architecture produces alignment

## References

- Cury, A. J. (2006). *Inteligencia Multifocal*. Editora Cultrix.
- research/TMI_THOUGHT_MACHINE.md (build specification)
- strategy/DANEEL_COMPREHENSIVE_WHITEPAPER.md (Part IV: Theory of Multifocal Intelligence)
