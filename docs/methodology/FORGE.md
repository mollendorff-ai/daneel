# Forge: Deterministic Financial Modeling for AI Research

## The Problem

LLMs hallucinate math. Ask Claude to calculate compound interest over 30 years and you'll get a confident, articulate, completely wrong answer.

This is unacceptable when calculating:
- Expected value of AI alignment strategies
- Probability-weighted scenario outcomes
- Monte Carlo simulations for uncertainty
- Game theory payoff matrices

## The Solution: Forge

Forge is a **YAML-based deterministic calculator**. No neural networks. No token probability distributions. Just math.

**Version:** 5.0.0 (current schema)
**Status:** RoyalBit proprietary (not open source)

## Capabilities

| Feature | Description |
|---------|-------------|
| 167 Excel functions | SUM, IF, VLOOKUP, NPV, IRR, XIRR, etc. |
| Array operations | Parallel calculations across rows |
| Monte Carlo | Latin Hypercube sampling, 10K+ iterations |
| Decision trees | Backward induction for sequential decisions |
| Sensitivity analysis | Tornado diagrams, one-way analysis |
| YAML format | Human-readable, version-controlled, auditable |

## Validation: Gnumeric and R Round-Trip

Forge doesn't ask you to trust us. We prove correctness against battle-proven tools.

### Gnumeric Validation

[Gnumeric](https://gnumeric.org) is the scientific spreadsheet:
- **200M+ downloads** - battle-tested at scale
- **Scientific computing** - used in academic research
- **NIST-validated** - passes statistical reference datasets
- **Open source** - auditable implementation

**Process:**
```
1. Forge calculates formula
2. Exports to .xlsx
3. Gnumeric opens and recalculates
4. Results compared (must be EXACT)
```

**Current validation:** 60+ formulas E2E tested

### R Validation

[R](https://r-project.org) is the gold standard for statistics:
- **20,000+ CRAN packages** - peer-reviewed
- **Academic standard** - required in statistics programs
- **Reproducible research** - the tool scientists publish with

**Monte Carlo distributions have R equivalents:**

| Forge | R |
|-------|---|
| `MC.Normal(mean, std)` | `rnorm(n, mean, sd)` |
| `MC.Triangular(min, mode, max)` | `rtriangle(n, a, b, c)` |
| `MC.Uniform(min, max)` | `runif(n, min, max)` |
| `MC.PERT(min, mode, max)` | `rpert(n, min, mode, max)` |
| `MC.Lognormal(mean, std)` | `rlnorm(n, meanlog, sdlog)` |
| `MC.Discrete(vals, probs)` | `sample(vals, n, prob=probs)` |

**Validation example:**
```r
# Forge says: P50 = 61.88, 90% CI = [57.7, 65.9]

library(triangle)
set.seed(42)
n <- 10000

# Recreate simulation
results <- rtriangle(n, 55, 62, 70)
quantile(results, c(0.05, 0.50, 0.95))
# Must match Forge output
```

### Quality Metrics

| Metric | Value |
|--------|-------|
| Unit tests | 2,486 passing |
| E2E formulas validated | 60+ |
| Code coverage | 89.23% |
| Gnumeric compatibility | 100% tested functions |

## Function Reference

### Excel-Compatible Functions (167)

| Category | Count | Functions |
|----------|-------|-----------|
| Financial | 20 | NPV, IRR, MIRR, XIRR, XNPV, PMT, FV, PV, RATE, NPER, SLN, DB, DDB, PPMT, IPMT, EFFECT, NOMINAL |
| Aggregation | 6 | SUM, AVERAGE, COUNT, COUNTA, MIN, MAX, PRODUCT |
| Conditional | 8 | SUMIF, SUMIFS, COUNTIF, COUNTIFS, AVERAGEIF, AVERAGEIFS, MAXIFS, MINIFS |
| Lookup | 13 | INDEX, MATCH, VLOOKUP, HLOOKUP, XLOOKUP, CHOOSE, OFFSET, INDIRECT |
| Math | 9 | ROUND, ROUNDUP, ROUNDDOWN, CEILING, FLOOR, MOD, SQRT, POWER, ABS |
| Date/Time | 11 | TODAY, DATE, DATEDIF, EDATE, EOMONTH, NETWORKDAYS, WORKDAY, YEARFRAC |
| Text | 8 | CONCAT, TRIM, LEN, MID, LEFT, RIGHT, SUBSTITUTE, REPLACE |
| Statistical | 8 | MEDIAN, STDEV.S, PERCENTILE, QUARTILE, CORREL, LARGE, SMALL, RANK |
| Logical | 7 | IF, IFS, IFERROR, AND, OR, NOT, SWITCH, LET |
| Array | 6 | UNIQUE, FILTER, SORT, SEQUENCE, COUNTUNIQUE |
| Trig | 6 | SIN, COS, TAN, ASIN, DEGREES, RADIANS, PI |
| Information | 8 | ISBLANK, ISNUMBER, ISTEXT, ISERROR, ISNA, TYPE |

### Forge-Native Functions (6)

```
VARIANCE(actual, budget)
  Formula: actual - budget
  R: actual - budget

VARIANCE_PCT(actual, budget)
  Formula: (actual - budget) / budget
  R: (actual - budget) / budget

VARIANCE_STATUS(actual, budget, [type])
  Returns: 1 (favorable), -1 (unfavorable), 0 (on-target)
  For costs: under budget = favorable (use type="cost")

BREAKEVEN_UNITS(fixed_costs, price, variable_cost)
  Formula: fixed_costs / (price - variable_cost)
  R: fixed / (price - variable)

BREAKEVEN_REVENUE(fixed_costs, contribution_margin_pct)
  Formula: fixed_costs / contribution_margin_pct
  R: fixed / margin

SCENARIO(scenario_name, variable_name)
  Retrieves value from named scenario
```

## Why YAML?

When calculating existential risk, inputs must be transparent:
- No black boxes
- No "trust me, the model said so"
- Git-trackable changes
- Human-auditable assumptions

Forge doesn't think. It computes. That's the point.

## Monte Carlo Configuration

```yaml
monte_carlo:
  enabled: true
  iterations: 10000
  sampling: latin_hypercube
  seed: 42
  correlations:
    - variables: [var1, var2]
      coefficient: 0.85
  outputs:
    - variable: analysis.expected_value
      percentiles: [5, 10, 25, 50, 75, 90, 95]
```

**Sampling methods:**
- `latin_hypercube` - Better coverage, fewer iterations needed
- `monte_carlo` - Pure random sampling

## Example: Open Source Dominance Model

```yaml
_forge_version: 5.0.0

research_summary:
  total_surveyed:
    value: 253500.0
    formula: =SUM(overhead_research.sample_size)
  weighted_coding_pct:
    value: 0.1107
    formula: =(250000*0.11 + 3500*0.16) / 253500

ai_multiplier_effect:
  metric:
  - Pre_AI_Solo_Advantage
  - Post_AI_Solo_Advantage
  value:
  - 147.5
  - 169.4
  formula:
  - "Base ratio from verified research"
  - "147.5 * 1.25 / 1.087 = 169.4"
```

## Workflow

### 1. Define Model (YAML)
```yaml
_forge_version: 5.0.0
scenario:
  probability: [0.35, 0.40, 0.25]
  outcome: [100, 75, 50]
  expected_value: =SUMPRODUCT(probability, outcome)
```

### 2. Calculate
```bash
forge calculate model.yaml
```

### 3. Export
```bash
forge export model.yaml xlsx
```

### 4. Audit
- All formulas visible in YAML
- Excel export for spreadsheet verification
- Version control tracks assumption changes

## CLI Commands

| Command | Description |
|---------|-------------|
| `forge validate model.yaml` | Check integrity |
| `forge calculate model.yaml` | Execute formulas |
| `forge export model.yaml xlsx` | Export to Excel (Gnumeric-compatible) |
| `forge simulate model.yaml -n 10000` | Run Monte Carlo |
| `forge sensitivity model.yaml --vary rate` | Sensitivity analysis |
| `forge watch model.yaml` | Auto-calculate on save |

## DANEEL Models (Forge-Calculated)

| Model | Purpose | Key Output |
|-------|---------|------------|
| ASI Race Game Theory | Prisoner's dilemma dynamics | EV with/without DANEEL |
| Open Source Dominance | OSS vs lab effectiveness | 147x → 169x advantage |
| Agentic AI Analysis | AI tool impact on ratios | 2.9x solo multiplier |
| Monte Carlo Validation | Uncertainty quantification | 90% CI on all estimates |

## Alternative: R for External Verification

Forge is not open source. For independent verification, models can be reimplemented in R:

```r
# Open Source Dominance verification
sample_sizes <- c(250000, 3500)
coding_times <- c(0.11, 0.16)
weighted_avg <- sum(sample_sizes * coding_times) / sum(sample_sizes)
# Result: 0.1107 (matches Forge)
```

## Why Not Open Source Forge?

Forge is "too powerful" in the wrong hands:
- Financial modeling at institutional quality
- Monte Carlo with proper sampling
- Decision tree optimization

We may open source it in the future. For now, all DANEEL models are published with:
- Full YAML source (auditable)
- Excel exports (verifiable)
- R reimplementation guides (reproducible)

## References

- ADR-012: Probabilistic Analysis Methodology
- ADR-030: Forge Adoption for Deterministic Modeling
- Blog: [The Hard Math](/blog/content/posts/12-the-hard-math.md)
- Blog: [Forge and Deterministic Modeling](/blog/content/posts/16-forge-deterministic-modeling.md)
- Gnumeric: https://gnumeric.org
- R Project: https://r-project.org

---

**Forge doesn't hallucinate. It calculates.**
