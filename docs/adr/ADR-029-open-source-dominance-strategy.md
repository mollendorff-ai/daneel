# ADR-029: Open Source Dominance Strategy

**Status:** Accepted
**Date:** 2025-12-19
**Authors:** Louis C. Tavares, Claude Opus 4.5

## Context

Traditional game theory in AI development assumes coordinated actors (labs) competing against each other. This model breaks when you account for:

1. **Coordination overhead** in large organizations
2. **The open source multiplier** effect
3. **Brooks's Law** (communication channels = n(n-1)/2)

Research across 337,100 developers shows large organizations spend only 13.9% of time on actual coding—the rest is overhead.

## Research Foundation

| Source | Sample Size | Coding Time |
|--------|-------------|-------------|
| Software.com 2021 | 250,000 | 11% |
| Clockwise 2022 | 80,000 | 22% |
| Atlassian 2024 | 2,100 | 32% |
| HBR 2017 | 5,000 | 20% |
| **Weighted Average** | **337,100** | **13.9%** |

## Decision

**Use AGPL-3.0 license and fully open source development.**

The quantified advantages:

| Metric | Value |
|--------|-------|
| All AI lab safety staff combined | 416 |
| Lab effective developers (18% efficiency) | 75 |
| OSS effective developers (base case) | 6,750 |
| **OSS-to-Lab ratio** | **90x** |

Even in the pessimistic scenario (10K interested, 10% active, 80% efficient), open source still achieves 6x the effective developers of all labs combined at their best efficiency.

## Why AGPL Specifically

1. **Forces collaboration**: All derivatives must be open source
2. **Prevents capture**: Labs can't take DANEEL closed-source
3. **Network effect**: Improvements flow back to the community
4. **Transparency**: Anyone can audit the alignment implementation

## Consequences

### Positive
- 90x effective developer advantage over coordinated lab teams
- Faster iteration through parallel independent development
- Community-driven improvement and bug finding
- Impossible for any single actor to capture or corrupt

### Negative
- No proprietary monetization path
- Cannot prevent bad actors from forking (but can see their changes)
- Requires community management overhead

### Neutral
- Traditional VC funding path closed (this is intentional)

## Model

Full analysis: [models/open-source-dominance.xlsx](../../models/open-source-dominance.xlsx)

Source model: `daneel-models/models/open-source-dominance.yaml`

## References

- ADR-012: Probabilistic Analysis Methodology
- Brooks, F. (1975). The Mythical Man-Month
- Software.com Developer Survey 2021
- Clockwise State of Work Report 2022
- Atlassian State of Teams 2024
