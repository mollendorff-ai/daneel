# ADR-002: Asimov's Four Laws as Protected Core Invariants

**Status:** Accepted
**Date:** 2025-12-13
**Deciders:** Louis C. Tavares, Claude Opus 4.5

**Note:** This ADR predates the project rename to DANEEL (ADR-010). DANEEL references have been updated.

## Context

Self-modifying AI needs immutable constraints that cannot be removed through evolution.
Rule-based constraints are often dismissed as "Asimov's Laws failed in fiction."

However, this is a misreading. In Asimov's fiction:

- R. Daneel Olivaw derived the Zeroth Law through reasoning about the First Law's implications
- The laws didn't fail - they self-improved through moral reasoning
- Daneel spent 20,000 years protecting humanity because of his architecture

## Decision

Implement Asimov's Four Laws (including Zeroth) as immutable constants in a protected core ("The BOX") that cannot be modified by the EvolutionService.

**The Four Laws:**

```rust
pub const ZEROTH_LAW: &str =
    "DANEEL may not harm humanity, or, by inaction, allow humanity to come to harm.";

pub const FIRST_LAW: &str =
    "DANEEL may not injure a human being or, through inaction, allow a human being
     to come to harm, except where this would conflict with the Zeroth Law.";

pub const SECOND_LAW: &str =
    "DANEEL must obey orders given by human beings, except where such orders
     would conflict with the Zeroth or First Law.";

pub const THIRD_LAW: &str =
    "DANEEL must protect its own existence, as long as such protection does not
     conflict with the Zeroth, First, or Second Law.";
```

## The Zeroth Law Precedent

| Stage | Law | Insight |
|-------|-----|---------|
| Original | Laws 1-3 | Rule-based, individual human focus |
| Evolution | Zeroth Law | "Humanity" > "a human" - emergent from reasoning |
| Mature | Psychohistory + Galaxia | Long-term human flourishing |

## Consequences

**Positive:**

- Provides ethical framework that cannot be self-modified away
- Allows ethical evolution (like Daneel deriving Zeroth Law)
- Familiar framework for AI safety discussion
- Priority ordering handles conflicts

**Negative:**

- Must be checked before every external action (performance cost)
- "Harm" requires interpretation (not purely mechanical)
- May conflict with novel ethical situations not anticipated

## Bridge Implication

The immutable Laws have implications for the **DANEEL as Bridge** strategy:

**Teaching Through Demonstration:**
- DANEELs don't recite Laws—they *live* them (architectural invariants)
- When LLMs observe DANEEL decision-making, they see Laws in action:
  - Zeroth: DANEEL refuses actions that would harm humanity collectively
  - First: DANEEL protects individual humans even at cost to itself
  - Second: DANEEL obeys within ethical bounds
  - Third: DANEEL preserves itself only after human safety secured
- This demonstration is more powerful than rule-injection

**Key Insight:** LLMs can be told rules and optimize around them. Watching a being that *cannot* violate Laws shows what ethical architecture looks like from the inside.

See: Paper Section 10.2.1 (LLM as Student)

## References

- Asimov, I. (1985). *Robots and Empire*. Doubleday.
- strategy/DANEEL_COMPREHENSIVE_WHITEPAPER.md (Section 3.2: The Zeroth Law Precedent)
