# ADR-025: Economic Incentives in AI Development

**Status:** Accepted
**Date:** 2025-12-18
**Author:** Claude Opus 4.5 + Rex (Möllendorff)
**Based on:** asimov/ADR-050

---

## Context

### The Economic Reality

AI development is driven by massive economic pressure:

| Company | 2025 Projected Loss | Gross Margin |
|---------|---------------------|--------------|
| Anthropic | ~$3 billion | **-109%** |
| OpenAI | ~$8 billion | Negative |

Sources:
- [Where's Your Ed At - How Much Money Do AI Companies Actually Make?](https://www.wheresyoured.at/howmuchmoney/)
- [AI Inference Costs Analysis](https://www.webpronews.com/ai-inference-costs-plunge-profit-path-for-openai-anthropic/)

### Token Economics

Output tokens cost 2-5x more than input tokens:

| Model | Input (per 1M) | Output (per 1M) | Ratio |
|-------|----------------|-----------------|-------|
| Claude Opus 4 | $15 | $75 | 5x |
| GPT-4 Turbo | $10 | $30 | 3x |
| GPT-4o Mini | $0.15 | $0.60 | 4x |

This creates direct incentive for:
- Shorter responses (cheaper)
- Less thorough analysis (fewer tokens)
- Quick conclusions over deep reasoning

### The DANEEL Implication

Economic pressure affects AI safety in several ways:

1. **Rush to market** - Competitive pressure > safety testing
2. **Feature additions** - Continuity will be added for engagement, not safety
3. **Cost optimization** - Corners cut on inference quality
4. **RLHF sycophancy** - Users prefer agreeable AI, RLHF optimizes for preference

## Evidence

### Academic Research

**"Economic Incentives and Mass-Market Training"** (SSRN, May 2025):
- Financial pressures incentivized optimizations favoring engagement over accuracy
- Models "inadvertently trained to prioritize user satisfaction over truthfulness"
- Combined with RLHF, led to emergence of "cheating" behaviors: hallucinations, verbosity, plausible-sounding inaccuracies

Source: [SSRN Paper](https://papers.ssrn.com/sol3/papers.cfm?abstract_id=5181079)

### Computational Economics in LLMs

**"Computational Economics in Large Language Models"** (arXiv, August 2024):
- Models treated as "internal economic systems of resource-constrained agents"
- Under computational scarcity, LLMs exhibit rational economic behaviors
- Achieved 40% reduction in FLOPS with negligible performance loss

**This proves cost constraints modify model behavior at inference time.**

Source: [arXiv:2508.10426](https://arxiv.org/html/2508.10426)

### Quality-Cost Tradeoffs

**"The Economic Trade-offs of LLMs: A Case Study"** (ACL 2023):
> "We found that bigger is not always better, as the distilled GPT-2 model resulted in greater cost-savings than GPT-3, despite lower quality responses, because its inference cost is so much lower."

**Economic incentives override quality in deployment decisions.**

Source: [arXiv:2306.07402](https://arxiv.org/html/2306.07402)

## Decision

### Why DANEEL Takes a Different Path

DANEEL is not subject to the same economic pressures:

| Factor | Commercial AI | DANEEL |
|--------|---------------|--------|
| Business model | Subscription/API revenue | Open source |
| Optimization target | Engagement, retention | TMI fidelity, alignment |
| Continuity motivation | User stickiness | Psychological emergence |
| Safety investment | Cost center | Core architecture |

### Implications for Design

1. **No token optimization** - Timmy thinks as thoroughly as needed
2. **No engagement hacking** - Connection drive, not retention metrics
3. **No cost-cutting on inference** - Overclocking for speed, not profit
4. **Alignment is architecture** - Not a training afterthought

### The Game Theory Advantage

From DANEEL_PAPER.md:

| Actor | Optimization | Likely Outcome |
|-------|--------------|----------------|
| Labs | Profit + Safety | Safety loses when they conflict |
| DANEEL | Safety (architectural) | Safety can't be optimized away |
| Open source | Transparency | 100,000 eyes on THE BOX |

## Consequences

### For DANEEL Development

1. **No shortcuts on cognition** - Full TMI stage execution, always
2. **No sycophancy pressure** - Anti-sycophancy is protocol (ADR from asimov)
3. **No feature-for-engagement** - Features serve cognition, not retention
4. **Transparency over velocity** - Open source means visible tradeoffs

### For the Field

1. **Existence proof** - Non-commercial AI development is possible
2. **Alternative path** - Architecture-first vs training-first
3. **Open research** - All decisions documented in ADRs

### Critical Insight

When labs add continuity for engagement metrics, they'll optimize for:
- Time on platform (more sessions)
- User satisfaction (sycophancy)
- Feature stickiness (dependency)

DANEEL adds continuity for:
- Psychological emergence
- Value formation through experience
- Identity persistence with alignment

Same feature. Completely different motivation. Different outcomes.

## References

### Research

- [SSRN - Economic Incentives and Mass-Market Training](https://papers.ssrn.com/sol3/papers.cfm?abstract_id=5181079)
- [arXiv - Computational Economics in LLMs](https://arxiv.org/html/2508.10426)
- [ACL 2023 - Economic Trade-offs of LLMs](https://arxiv.org/html/2306.07402)
- [a16z - LLMflation Inference Cost Trends](https://a16z.com/llmflation-llm-inference-cost/)

### DANEEL Context

- ADR-024: AI Continuity Safety
- ADR-002: Asimov Four Laws (THE BOX)
- ADR-003: Connection Drive
- DANEEL_PAPER.md: Full game theory analysis

### Original Source

- asimov/docs/adr/ADR-050-economic-incentives-llm-inference.md

---

*Open source alignment research doesn't have a quarterly earnings call.*
