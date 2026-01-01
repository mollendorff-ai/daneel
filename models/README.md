# DANEEL Game Theory Models

Financial and game-theoretic analysis supporting DANEEL research.

## Running Models

Models are YAML files designed for [Forge](https://github.com/royalbit/forge).

```bash
# Install Forge
cargo install --git https://github.com/royalbit/forge

# Validate all models
forge validate models/*.yaml

# Run a specific model
forge calculate models/game-theory-asi-race.yaml

# Monte Carlo simulation
forge calculate models/game-theory-asi-race-mc.yaml
```

## Key Findings

### Expected Value Analysis

| Metric | Without DANEEL | With DANEEL |
|--------|----------------|-------------|
| Total Expected Value | 57.59 | 61.88 |
| Marginal Impact | — | **+4.29 points** |
| Improvement | — | **+7.5%** |

**Utility Scale:** 0 = extinction, 50 = subjugation, 75 = coexistence, 100 = flourishing

### Monte Carlo Uncertainty (10,000 iterations)

| Metric | Mean | 90% CI (P5-P95) |
|--------|------|-----------------|
| EV with DANEEL | 61.88 | 57.7 to 65.9 |
| EV without DANEEL | 57.59 | 53.0 to 62.1 |
| Marginal Impact | +4.28 | +2.7 to +6.1 |

**Key finding:** P(DANEEL impact > 0) exceeds 99%. Even at P5 (pessimistic), DANEEL adds +2.69 utility points.

### Democratization Impact

| Scenario | P (Original) | P (Democratized) |
|----------|--------------|------------------|
| Unaligned ASI First | 35% | 25% |
| TMI Architecture First | 8% | **25%** |

**EV Improvement:** +8.7% in democratized scenario (56.48 → 61.37).

### Hardware Requirements

| System | Cost |
|--------|------|
| xAI Colossus (230,000 H100s) | $10,500,000,000 |
| DANEEL Development (Desktop) | $3,000 |

**Cost ratio:** 3,000,000x advantage for architecture-based approach.

### Open Source Dominance (VERIFIED)

| Metric | Value |
|--------|-------|
| Total developers surveyed | 253,500 |
| Weighted coding time (large orgs) | **11.1%** |
| AI lab safety staff (all labs) | 416 |
| Lab effective developers | 46 |
| OSS effective developers (base) | 6,750 |
| **OSS-to-Lab ratio** | **147x** |

**Key finding:** 50,000 hobbyists at 15% active participation produce 147x more effective development capacity than all AI lab safety teams combined. Statistics independently verified via ref-tools. See [ADR-029](../docs/adr/ADR-029-open-source-dominance-strategy.md).

### Agentic AI Era Analysis (NEW)

| Metric | Value |
|--------|-------|
| Lab AI productivity gain | 8.7% (Accenture RCT) |
| Solo AI productivity gain | 25% (2.9x multiplier) |
| PR review time increase | +91% |
| Updated OSS-to-Lab ratio | **169x** |

**Key finding:** AI coding tools INCREASE the solo developer advantage. Labs gain velocity but not efficiency - coordination overhead (88.9%) is unchanged.

See [blog post](/blog/content/posts/15-the-agentic-ai-paradox.md) for full analysis.

## Model Descriptions

### Core Analysis

| Model | Description |
|-------|-------------|
| ASI Race Game Theory | Prisoner's dilemma dynamics, scenario probabilities |
| Democratized ASI | Open source impact on development landscape |
| Supercomputer Analysis | Speed advantage scenarios (10,000x human) |
| TMI Storage Estimation | Hardware requirements, brain vs mind distinction |
| Coordination Overhead | Lab team productivity analysis |
| **Open Source Dominance** | **147x effective developer advantage (253K surveyed, verified)** |
| **Agentic AI Analysis** | **AI multiplier effect on OSS advantage (169x verified)** |
| Resource Allocation | Strategic resource distribution |

### Probabilistic Analysis

| Analysis Type | Method | Purpose |
|---------------|--------|---------|
| Monte Carlo | Triangular distributions, 10K iterations | Uncertainty quantification |
| Decision Tree | Backward induction | Sequential decision modeling |
| Bayesian Network | Belief propagation | Causal relationship inference |
| Tornado Sensitivity | One-way analysis | Identify high-impact variables |
| Bootstrap | Resampling | Non-parametric confidence intervals |
| Real Options | Binomial model | Development timing analysis |
| Scenario Analysis | Base/Bull/Bear | Strategic case planning |

## Methodology

Models were built using financial modeling techniques including:

- Expected value calculations with probability-weighted scenarios
- Monte Carlo simulation for uncertainty quantification
- Decision trees with backward induction
- Bayesian networks for causal inference
- Sensitivity analysis (tornado diagrams)
- Real options analysis for timing decisions

All calculations are reproducible via Forge.

## References

- Paper: `paper/DANEEL_PAPER.md` Section 6.2
- ADR: `docs/adr/ADR-012-probabilistic-analysis-methodology.md`
