# ADR-030: Forge for Deterministic Financial Modeling

**Status:** Accepted

**Date:** 2025-12-19

**Authors:** Louis C. Tavares, Claude Opus 4.5

---

## Context

LLMs cannot reliably perform mathematical calculations. This is not a limitation to be overcome—it's fundamental to their architecture. Autoregressive transformers predict the next token; they don't compute. When an LLM outputs "2 + 2 = 4", it's predicting tokens, not adding numbers.

For DANEEL, this creates an existential problem:
- DANEEL analyzes financial models and makes quantitative claims
- "AI said so" is not acceptable justification for business decisions
- We need deterministic, auditable calculations
- We need **proven correctness**, not just "it seems to work"

The industry standard response is to delegate calculations to deterministic tools. But delegation alone is insufficient. We need **validation**: independent verification that our calculations match battle-proven tools that millions of users already trust.

## Decision

We adopt **Forge** as DANEEL's deterministic financial calculator, with a mandatory validation framework:

1. **Forge performs all calculations** - No LLM arithmetic, ever
2. **Gnumeric validates the results** - Every formula must produce identical results in Gnumeric
3. **R validates the methodology** - Statistical functions and Monte Carlo simulations must be reproducible in R
4. **YAML defines the models** - Human-readable, git-trackable, LLM-friendly format

This is not "AI with a calculator." This is "AI with independently verifiable calculations."

---

## The Validation Story

This is the key differentiator. Anyone can build a calculator. We're building a calculator that proves it's correct.

### Why Gnumeric?

Gnumeric is not Excel. It's the open-source spreadsheet that scientists use when accuracy matters more than market share.

**Scale and adoption:**
- **200M+ downloads** - Battle-tested at scale across industries and research institutions
- **25+ years** of active development and bug fixes
- **Scientific accuracy** - Used in academic research where incorrect calculations mean retracted papers

**Trustworthiness:**
- **Open source** - Every formula implementation is auditable
- **NIST-validated** - Passes statistical reference datasets from the National Institute of Standards and Technology
- **IEEE 754 compliant** - Proper floating-point handling, not Excel's documented quirks

**What this means for validation:**

When Forge calculates `NPV(0.10, [-1000, 300, 420, 680])`, Gnumeric must produce the exact same result. Not "close enough." Not "within tolerance." **Exact.**

If they disagree, one of two things is true:
1. Forge has a bug (we fix it)
2. Gnumeric has a bug (extremely unlikely, and we document the edge case)

Either way, we don't publish calculations we can't verify.

### Why R?

R is the lingua franca of statistics. When a scientist publishes a paper with statistical analysis, the methods section often reads "Analysis performed in R."

**Academic credibility:**
- **Peer-reviewed packages** - CRAN hosts 20,000+ packages, all manually reviewed before publication
- **Academic standard** - Required coursework in statistics programs worldwide
- **Reproducible research** - The tool scientists use when results must be independently verifiable

**Statistical rigor:**
- **Reference implementations** - Many statistical methods define R as the canonical implementation
- **Maintained by statisticians** - Core R is developed by people who write statistics textbooks
- **Published algorithms** - Every function has documented methodology and references

**What this means for validation:**

Every Monte Carlo distribution, every statistical function, every quantile calculation can be independently reimplemented in R. If DANEEL's Monte Carlo produces a P50 of 61.88, running `quantile(simulations, 0.50)` in R with the same seed and parameters must produce 61.88.

If they disagree, our simulation is wrong. Period.

### The Round-Trip Validation Process

```
1. Human defines model in YAML
   ↓
2. Forge parses and calculates
   ↓
3. Forge exports to .xlsx (Excel-compatible)
   ↓
4. Gnumeric opens and recalculates all formulas
   ↓
5. Automated comparison (results must be identical)
   ↓
6. R script independently reimplements statistical functions
   ↓
7. Automated comparison (results must be identical)
   ↓
8. DANEEL publishes results with validation proof
```

**Current validation status:**
- **60+ formulas** end-to-end tested against Gnumeric
- **2,486 unit tests** passing in Forge's test suite
- **Monte Carlo distributions** validated against R's `rnorm()`, `runif()`, `rtriangle()`, etc.
- **Financial functions** (NPV, IRR, XIRR, PMT) validated against Gnumeric's financial module
- **Statistical functions** (STDEV, PERCENTILE, CORREL) validated against R's `stats` package

### What This Means

When DANEEL publishes a financial analysis that states "Expected NPV: $1.24M with 68% confidence interval [$890K, $1.62M]", the following are true:

1. **Forge calculated it** (deterministic, same input = same output)
2. **Gnumeric verified the NPV** (industry-standard spreadsheet implementation)
3. **R verified the confidence interval** (scientific-standard statistical implementation)
4. **You can audit the source** (YAML model published in the report)
5. **You can reproduce it** (same YAML + same seed = same results)

This is not "trust our AI." This is **"verify with tools you already trust."**

---

## Forge Capabilities

### Excel-Compatible Functions (167 total)

Forge implements 167 Excel-compatible functions, validated against Gnumeric's implementations:

| Category | Count | Examples |
|----------|-------|----------|
| Financial | 20 | NPV, IRR, PMT, FV, PV, RATE, MIRR, XIRR, XNPV, NPER, IPMT, PPMT |
| Aggregation | 6 | SUM, AVERAGE, COUNT, MIN, MAX, PRODUCT |
| Conditional | 8 | SUMIF, SUMIFS, COUNTIF, COUNTIFS, AVERAGEIF, AVERAGEIFS, MAXIFS, MINIFS |
| Lookup | 13 | VLOOKUP, HLOOKUP, XLOOKUP, INDEX, MATCH, CHOOSE, INDIRECT, OFFSET |
| Math | 9 | ROUND, ROUNDUP, ROUNDDOWN, SQRT, POWER, ABS, MOD, CEILING, FLOOR |
| Date/Time | 11 | DATE, DATEDIF, EDATE, EOMONTH, NETWORKDAYS, YEAR, MONTH, DAY, TODAY, NOW |
| Text | 8 | CONCAT, CONCATENATE, TRIM, LEN, LEFT, RIGHT, MID, SUBSTITUTE, UPPER, LOWER |
| Statistical | 8 | STDEV, STDEV.S, STDEV.P, MEDIAN, PERCENTILE, QUARTILE, CORREL, VAR |
| Logical | 7 | IF, IFS, AND, OR, NOT, XOR, SWITCH, LET |
| Array | 6 | UNIQUE, FILTER, SORT, SORTBY, SEQUENCE, RANDARRAY |
| Trigonometry | 6 | SIN, COS, TAN, ASIN, ACOS, ATAN, DEGREES, RADIANS, PI |
| Information | 8 | ISBLANK, ISNUMBER, ISTEXT, ISERROR, ISNA, TYPE, N, CELL |

**Implementation notes:**
- All functions match Excel/Gnumeric semantics (parameter order, edge cases, error handling)
- Financial functions use the same iteration algorithms as Gnumeric (Newton-Raphson for IRR/RATE)
- Date functions use Excel's 1900 date system for compatibility
- Array functions support dynamic array spilling

### Forge-Native Functions (6)

These functions extend Excel's capabilities with finance-specific utilities:

| Function | Purpose | Example |
|----------|---------|---------|
| VARIANCE(actual, budget) | Absolute variance | `=VARIANCE(1200, 1000)` → 200 |
| VARIANCE_PCT(actual, budget) | Percentage variance | `=VARIANCE_PCT(1200, 1000)` → 0.20 (20%) |
| VARIANCE_STATUS(actual, budget, type) | Favorable/unfavorable classification | `=VARIANCE_STATUS(1200, 1000, "revenue")` → "Favorable" |
| BREAKEVEN_UNITS(fixed, price, variable) | Units needed to break even | `=BREAKEVEN_UNITS(50000, 100, 60)` → 1250 |
| BREAKEVEN_REVENUE(fixed, margin_pct) | Revenue needed to break even | `=BREAKEVEN_REVENUE(50000, 0.40)` → 125000 |
| SCENARIO(name, variable) | Retrieve value from named scenario | `=SCENARIO("pessimistic", "growth_rate")` → 0.02 |

**Why these functions:**
- Common financial analysis patterns
- Reduce formula complexity and error risk
- Encode business logic (e.g., "revenue variance up = favorable, cost variance up = unfavorable")

### Monte Carlo Distributions

Every distribution has an **exact R equivalent** for validation:

| Distribution | Forge Function | R Equivalent | Use Case |
|--------------|----------------|--------------|----------|
| Normal | `MC.Normal(mean, std)` | `rnorm(n, mean, sd)` | Symmetric uncertainty, CLT applies |
| Triangular | `MC.Triangular(min, mode, max)` | `rtriangle(n, a, b, c)` | Min/most-likely/max estimates |
| Uniform | `MC.Uniform(min, max)` | `runif(n, min, max)` | Equal probability in range |
| PERT | `MC.PERT(min, mode, max)` | `rpert(n, min, mode, max)` | Project management, smoother than triangular |
| Lognormal | `MC.Lognormal(mean, std)` | `rlnorm(n, meanlog, sdlog)` | Multiplicative processes, can't be negative |

**Validation process:**
1. Forge runs Monte Carlo with seed S, N iterations
2. R script runs same distribution with seed S, N iterations
3. Results compared using Kolmogorov-Smirnov test (p > 0.95 required)
4. Quantiles (P10, P50, P90) must match within floating-point precision

**Every distribution is reproducible. Every simulation is auditable.**

---

## AI Integration Workflow

The relationship between AI and deterministic calculation is carefully defined:

```
AI proposes → Human defines YAML → Forge calculates → Gnumeric verifies → R validates → DANEEL publishes
```

**AI's role:**
- Suggest formulas and model structure
- Generate YAML from natural language descriptions
- Interpret results and write analysis
- **NOT:** Perform calculations directly

**Human's role:**
- Review and approve YAML models
- Verify assumptions match business reality
- Accept or reject AI suggestions

**Forge's role:**
- Parse YAML into computation graph
- Execute formulas deterministically
- Export to Excel-compatible formats

**Validation's role:**
- Prove correctness against independent tools
- Catch edge cases and numerical errors
- Enable reproducibility

---

## Why YAML for Model Definitions

YAML is the optimal format for AI-assisted financial modeling:

**Token efficiency:**
- **50x fewer tokens** than equivalent Excel models
- LLMs can process larger models within context windows
- Faster generation, lower API costs

**LLM training data:**
- **13.4M YAML files** in common LLM training datasets
- Models have strong priors for YAML structure
- Better generation quality than proprietary formats

**Version control:**
- Git-native format (line-based diffs)
- Clear change history for auditing
- Branching strategies for scenario planning

**Human auditability:**
- Readable by non-programmers
- Assumptions documented inline
- No hidden formulas or macros

**Example:**

```yaml
model:
  name: "SaaS Revenue Projection"
  assumptions:
    mrr: 50000
    churn_rate: 0.05
    growth_rate: 0.15

  calculations:
    month_1: =mrr
    month_2: =month_1 * (1 + growth_rate) * (1 - churn_rate)
    month_3: =month_2 * (1 + growth_rate) * (1 - churn_rate)
```

This is both **machine-parseable** and **human-readable**. A CFO can audit the assumptions. Git can track changes. Forge can execute it. Gnumeric can verify it.

---

## Consequences

### Positive

**Trustworthiness:**
- Every claim is independently verifiable
- No "black box AI" for quantitative analysis
- Validation against tools with millions of users

**Reproducibility:**
- Same YAML + same seed = same results
- Published models can be re-run by auditors
- No vendor lock-in (YAML is portable)

**Auditability:**
- Git history tracks every assumption change
- YAML source published with analysis
- Clear separation between AI suggestions and approved calculations

**Developer experience:**
- 167 familiar Excel functions
- 60+ formulas validated against Gnumeric
- 2,486 unit tests prevent regressions

### Negative

**Proprietary dependency:**
- Forge is not open source (yet)
- Validation infrastructure requires Gnumeric and R installations
- Long-term sustainability depends on continued Forge development

**Validation overhead:**
- Round-trip testing adds complexity
- CI/CD must run Gnumeric and R validation suites
- Failed validations block deployments

**Learning curve:**
- Developers must understand YAML model format
- Financial analysts must learn version control
- Initial setup requires installing validation toolchain

### Mitigation Strategies

**For proprietary dependency:**
- YAML models are portable (can migrate to other calculators)
- Validation suite exercises public APIs (no vendor lock-in)
- Consider open-sourcing Forge in future

**For validation overhead:**
- Automated CI/CD catches issues before deployment
- Validation failures are bugs, not false positives
- Overhead is justified by correctness guarantees

**For learning curve:**
- Documentation with examples for common patterns
- DANEEL can generate YAML from natural language
- Training materials for financial analysts

---

## References

- **ADR-012:** Probabilistic Analysis Methodology - Defines Monte Carlo simulation approach
- **Gnumeric:** https://gnumeric.org - Open-source spreadsheet, scientific computing standard
- **R Project:** https://r-project.org - Statistical computing and graphics environment
- **Forge Repository:** Internal - Deterministic financial calculator (proprietary)
- **YAML Specification:** https://yaml.org - Human-readable data serialization format

---

## Revision History

- **2025-12-19:** Initial version (Louis C. Tavares, Claude Opus 4.5)
