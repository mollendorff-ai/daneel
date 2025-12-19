+++
date = '2025-12-19T21:00:00-05:00'
draft = false
title = 'Forge: When AI Cannot Be Trusted to Count'
description = 'Why we built a YAML calculator and stopped letting LLMs do math.'
tags = ['forge', 'math', 'monte-carlo', 'game-theory', 'deterministic', 'verification']
+++

## The Problem: LLMs Can't Count

Ask Claude to calculate compound interest over 30 years. It will respond with confidence. It will show its work. It will be wrong.

Not sometimes. Often enough that you cannot stake a business decision on it.

This is not a flaw. This is fundamental architecture. Large language models predict the next token based on patterns in training data. They don't compute. They don't calculate. They autocomplete.

When DANEEL produces a probability like "61.88% expected value with AI adoption," that number must be deterministic, verifiable, and auditable. If an LLM generated it, it's worth exactly nothing.

So we built Forge.

## What Forge Does

Forge is a YAML-based deterministic calculator. You define your model in human-readable YAML. Forge executes the formulas. No neural networks. No probability of tokens. Just math.

**Core capabilities:**

- **160+ Excel functions**: SUM, IF, VLOOKUP, NPV, IRR, and everything else you'd expect from institutional financial modeling
- **Monte Carlo simulation**: Latin Hypercube sampling for variance analysis
- **Decision trees**: Backward induction for multi-stage strategic decisions
- **Deterministic output**: Same input, same output, every time

When we say "Expected value: 61.88 with DANEEL, 57.59 without (+4.29)," that's Forge. When we claim "Open source advantage: 147x → 169x," that's Forge. When we publish 90% confidence intervals on market projections, that's Forge.

No hallucination. No approximation. Just arithmetic.

## The Validation Story: Why You Can Trust the Numbers

Here's what makes Forge different: we don't ask you to trust us. We prove correctness against tools you already trust.

### Gnumeric: The Scientific Spreadsheet

Gnumeric isn't Excel. It's the open-source spreadsheet scientists use:

- **200M+ downloads** - battle-tested
- **Scientific computing** - used in academic research
- **NIST-validated** - passes statistical reference datasets
- **Open source** - you can audit the implementation

When Forge calculates `NPV(0.10, [-1000, 300, 420, 680])`, Gnumeric must produce the **exact same result**. Not "close enough." Exact.

### R: The Gold Standard

R is the lingua franca of statistics:

- **20,000+ packages** on CRAN, all peer-reviewed
- **Academic standard** - required in statistics programs worldwide
- **Reproducible research** - the tool scientists publish with

Every Monte Carlo distribution in Forge has an R equivalent:

| Forge | R |
|-------|---|
| `MC.Normal(50, 10)` | `rnorm(n, 50, 10)` |
| `MC.Triangular(40, 50, 70)` | `rtriangle(n, 40, 70, 50)` |
| `MC.Uniform(40, 60)` | `runif(n, 40, 60)` |

If Forge says P50 = 61.88, then `quantile(simulations, 0.50)` in R must agree.

### The Round-Trip Process

```
Forge calculates → Export .xlsx → Gnumeric recalculates → Compare
                                        ↓
                               R reimplements → Compare
```

**Validation metrics:**
- 60+ formulas E2E tested against Gnumeric
- 2,486 unit tests passing
- Monte Carlo validated against R distributions
- Financial functions match Gnumeric's financial module

### What This Means for DANEEL

When we publish "Expected value: 61.88 with DANEEL, 57.59 without":

1. **Forge calculated it** - deterministic execution
2. **Gnumeric verified it** - industry-standard validation
3. **R can reproduce it** - scientific-grade verification
4. **You can audit it** - full YAML source published

This is not "trust our AI." This is "verify with tools you already trust."

## The Function Inventory

Forge implements 167 Excel-compatible functions plus 6 FP&A functions Excel lacks.

### Excel-Compatible Functions (167)

| Category | Count | Examples |
|----------|-------|----------|
| Financial | 20 | NPV, IRR, PMT, FV, PV, RATE, MIRR, XIRR |
| Aggregation | 6 | SUM, AVERAGE, COUNT, MIN, MAX, PRODUCT |
| Conditional | 8 | SUMIF, SUMIFS, COUNTIF, COUNTIFS |
| Lookup | 13 | VLOOKUP, XLOOKUP, INDEX, MATCH |
| Math | 9 | ROUND, SQRT, POWER, ABS, MOD |
| Date/Time | 11 | DATE, DATEDIF, EDATE, NETWORKDAYS |
| Text | 8 | CONCAT, TRIM, LEN, MID |
| Statistical | 8 | STDEV, MEDIAN, PERCENTILE, CORREL |
| Logical | 7 | IF, IFS, AND, OR, SWITCH, LET |
| Array | 6 | UNIQUE, FILTER, SORT, SEQUENCE |
| Trig | 6 | SIN, COS, TAN, DEGREES, RADIANS |
| Information | 8 | ISBLANK, ISNUMBER, ISERROR |

Every function tested against Gnumeric. Same formula, same result.

### Forge-Native Functions (6)

| Function | Purpose | R Equivalent |
|----------|---------|--------------|
| `VARIANCE(actual, budget)` | Absolute variance | `actual - budget` |
| `VARIANCE_PCT(actual, budget)` | Percentage variance | `(actual - budget) / budget` |
| `BREAKEVEN_UNITS(fixed, price, var)` | Units to break even | `fixed / (price - var)` |
| `BREAKEVEN_REVENUE(fixed, margin)` | Revenue to break even | `fixed / margin` |
| `SCENARIO(name, var)` | Pull scenario values | Named list access |

## Monte Carlo: Quantified Uncertainty

### Supported Distributions (R-validated)

| Distribution | Forge | R Equivalent | Use Case |
|--------------|-------|--------------|----------|
| Normal | `MC.Normal(mean, std)` | `rnorm()` | Growth rates |
| Triangular | `MC.Triangular(min, mode, max)` | `rtriangle()` | Expert estimates |
| Uniform | `MC.Uniform(min, max)` | `runif()` | Equal probability |
| PERT | `MC.PERT(min, mode, max)` | `rpert()` | Durations |
| Lognormal | `MC.Lognormal(mean, std)` | `rlnorm()` | Stock prices |
| Discrete | `MC.Discrete(vals, probs)` | `sample()` | Scenarios |

**Configuration:**
- Latin Hypercube sampling (better coverage than pure Monte Carlo)
- 10,000+ iterations
- Correlation modeling between variables
- Seed for reproducibility

## Why YAML Wins for AI

LLMs are trained on 13.4M YAML files. Not spreadsheets.

| Format | Tokens | LLM Familiarity |
|--------|--------|-----------------|
| Excel (XML) | ~100,000 | Low |
| YAML | ~2,000 | High |

**50x fewer tokens. Native comprehension.**

## Why YAML?

Because spreadsheets are terrible for version control, and code is terrible for non-programmers.

YAML sits in the middle:

**Human-readable**: Anyone can audit the assumptions. No "trust the black box."

```yaml
variables:
  base_success_rate: 0.65
  ai_boost: 0.12
  market_size: 1000000

calculations:
  success_with_ai:
    formula: "base_success_rate + ai_boost"
  expected_value:
    formula: "success_with_ai * market_size"
```

**Version-controlled**: Git diff shows exactly what changed between model versions. Did we adjust the discount rate? The diff shows it.

**Transparent**: Every assumption is explicit. No hidden cells. No circular references you discover three months later.

**Auditable**: Stakeholders can trace every number back to its source. Regulators can verify compliance. Competitors can challenge our math (and we want them to).

This is the opposite of "trust me, the AI said so."

## DANEEL Examples

Every quantitative claim in the DANEEL project runs through Forge:

### Expected Value Analysis
```
Base case (no DANEEL): 57.59
With DANEEL: 61.88
Delta: +4.29 (7.4% improvement)
```

Forge models the decision tree: strategic advantage, market positioning, cost reduction, and assigns probabilities based on historical data. Then it calculates expected value with backward induction.

### Open Source Dominance Model
```
OSS adoption rate: 147x proprietary → 169x with AI agents
90% CI: [142x, 195x]
```

Forge runs Monte Carlo simulation with Latin Hypercube sampling. We vary adoption curves, competitor response times, and market saturation rates across 10,000 iterations. The 169x is the median. The confidence interval tells you the range.

### Portfolio Optimization
```
Sharpe ratio: 2.34
Max drawdown: -12.8%
Win rate: 67.3%
```

Forge models correlated assets, runs historical backtests, and calculates risk-adjusted returns. No LLM could do this. No LLM should.

## The Workflow

Forge integrates into our research pipeline:

```bash
# Define the model in YAML
vim daneel-market-analysis.yaml

# Run calculations
forge calculate daneel-market-analysis.yaml

# Export to Excel for verification
forge export daneel-market-analysis.yaml xlsx

# Generate Monte Carlo simulation
forge simulate daneel-market-analysis.yaml --iterations 10000

# Output confidence intervals and distributions
```

Every model produces:
1. **YAML source**: The ground truth
2. **Excel export**: For stakeholders who live in spreadsheets
3. **JSON output**: For programmatic consumption
4. **PDF report**: Executive summary with visualizations

All deterministic. All verifiable. All reproducible.

## Will We Open Source It?

No. Not yet.

Forge represents years of institutional financial modeling expertise. It handles edge cases that took months to debug. It produces research-grade Monte Carlo analysis that hedge funds pay six figures for.

"Too powerful" is not hyperbole. This is the same caliber of tooling that quantitative finance shops guard as trade secrets.

**But:**

Every DANEEL model is published with:
- **YAML source**: Full model specification
- **Excel export**: Verify our formulas yourself
- **R scripts**: Independent statistical verification

You don't need our calculator. You need our assumptions and formulas. We publish those. You can reimplement Forge if you want. Or just audit our Excel exports.

The math is transparent. The tool is proprietary.

## Why This Matters

Because "AI said so" is not a business strategy.

DANEEL makes bold claims:
- AI agents will increase OSS adoption 169x
- Strategic advisory AI creates 7.4% value lift
- Open source wins 90% of platform battles

If those numbers came from an LLM, they'd be fiction. Because Forge calculated them, you can verify every assumption, challenge every formula, and rerun every simulation yourself.

That's the difference between research and marketing.

LLMs are powerful for synthesis, reasoning, and communication. But when you need to count, when you need to model risk, when you need numbers that will hold up under scrutiny—you need determinism.

You need Forge.

## Sources

- [ADR-030: Forge Adoption](https://github.com/royalbit/daneel/blob/main/docs/adr/ADR-030-forge-deterministic-modeling.md)
- [Forge Methodology](https://github.com/royalbit/daneel/blob/main/docs/methodology/FORGE.md)
- [Open Source Dominance Model](/models/open-source-dominance.xlsx)
- [ADR-012: Probabilistic Analysis Methodology](https://github.com/royalbit/daneel/blob/main/docs/adr/ADR-012-probabilistic-analysis-methodology.md)
- [Blog: The Hard Math](/blog/content/posts/12-the-hard-math.md)
- [Gnumeric](https://gnumeric.org)
- [R Project](https://r-project.org)
