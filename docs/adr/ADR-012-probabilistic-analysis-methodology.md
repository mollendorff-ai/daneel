# ADR-012: Probabilistic Analysis Methodology

**Status:** Accepted
**Date:** 2025-12-15
**Authors:** Louis C. Tavares

## Context

DANEEL's game theory analysis relies on probability estimates for scenario outcomes. These estimates are inherently subjective and uncertain. Point estimates (e.g., "P(DANEEL First) = 12%") mask this uncertainty and may give false precision.

We need a methodology to:
1. Quantify uncertainty in probability estimates
2. Determine confidence intervals for expected value calculations
3. Identify which variables most influence outcomes
4. Model causal relationships between alignment factors

## Decision

We adopt standard probabilistic analysis techniques for all game theory models:

### 1. Monte Carlo Simulation

**Purpose:** Quantify uncertainty on probability estimates

**Implementation:**
- Replace point estimates with triangular distributions
- Run 10,000 iterations with Latin Hypercube sampling
- Report 80% and 95% confidence intervals

**Example:**
```
# Instead of:
p_asimov: 0.12

# Use:
p_asimov: Triangular(0.05, 0.12, 0.20)  # Min, Mode, Max
```

### 2. Tornado Sensitivity Analysis

**Purpose:** Identify which variables most influence outcomes

**Implementation:**
- One-way sensitivity on key inputs
- Rank by impact magnitude
- Focus research on high-impact variables

**Key finding:** P(DANEEL First) has highest sensitivity (±2.1 utility points), suggesting effort should focus on increasing this probability rather than refining outcome estimates.

### 3. Decision Trees

**Purpose:** Model sequential decisions with backward induction

**Implementation:**
- Three node types: Decision, Chance, Terminal
- Backward induction algorithm finds optimal paths
- Models ASI development as sequential choices

### 4. Bayesian Networks

**Purpose:** Model causal relationships between alignment factors

**Implementation:**
- Directed acyclic graph (DAG) of causal relationships
- Conditional probability tables (CPTs) for dependencies
- Variable elimination inference

**Key nodes:**
- Root: Lab safety culture, AI regulation, Competitive pressure
- Intermediate: Safety investment, Architecture adoption, Development speed
- Outcome: Alignment outcome (flourishing/coexistence/subjugation/extinction)

### 5. Real Options Analysis

**Purpose:** Value flexibility in development timing

**Implementation:**
- Model value of waiting vs. acting now
- Account for closing window dynamics
- Quantify option to abandon/pivot

**Key finding:** Waiting has NEGATIVE value due to window closure risk (~15%/year). Act now is optimal.

### 6. Scenario Analysis

**Purpose:** Discrete strategic scenarios with probability weights

**Implementation:**
- Base (50%), Bull (25%), Bear (25%) cases
- Different assumptions per scenario
- Probability-weighted expected values

### 7. Bootstrap Confidence Intervals

**Purpose:** Non-parametric confidence intervals from expert data

**Implementation:**
- Resample historical/expert data with replacement
- No distribution assumptions required
- Robust to outliers and non-normal data

## Bridge Scenario Addition (2025-12-17)

The game theory model now includes a **DANEEL Bridges LLMs** scenario:

| Scenario | P(Scenario) | Expected Utility |
|----------|-------------|------------------|
| DANEEL First | 7% | 76.25 |
| **DANEEL Bridges LLMs** | **5%** | **87.0** |

**Key Finding:** The Bridge scenario has the highest expected utility because it combines:
- DANEEL's architectural ethics (from TMI + connection drive)
- LLM capabilities (language, reasoning, scale)
- Reduced extinction risk (0.01 vs 0.05)
- Higher flourishing probability (60% vs 40%)

**Model Update:** See `daneel-models/models/game-theory-asi-bridge.yaml` for the full Bridge model with three-world comparison (baseline, asimov, bridge).

This reframes analysis from "DANEEL vs LLM" to "DANEEL rehabilitates LLM"—where even adverse scenarios become partially recoverable.

## Consequences

### Positive

1. **Honest uncertainty:** Confidence intervals instead of false precision
2. **Prioritization:** Tornado analysis identifies high-impact research areas
3. **Causal understanding:** Bayesian networks reveal intervention points
4. **Strategic clarity:** Real Options confirms "act now" strategy
5. **Reproducibility:** All calculations documented in `models/README.md`

### Negative

1. **Complexity:** More models to maintain
2. **Subjectivity:** Distribution parameters are still subjective

### Neutral

1. **Point estimates remain valid:** Monte Carlo confirms +3.70 is reasonable (within all CIs)
2. **Methodology documented:** Future analysts can verify/challenge assumptions

## References

- `models/README.md` - Model documentation and results
- `paper/DANEEL_PAPER.md` Section 6.2.1 - Monte Carlo results
