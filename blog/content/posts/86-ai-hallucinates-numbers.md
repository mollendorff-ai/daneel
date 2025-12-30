---
title: "AI Hallucinates Numbers"
date: 2025-12-29T09:00:00-05:00
draft: false
tags: ["forge", "financial-modeling", "ai", "launch", "determinism"]
series: ["Infrastructure"]
---

# AI Hallucinates Numbers

*December 29, 2025. Forge goes Source Available. 173 functions. 2,133 tests. Zero hallucinations.*

---

## The Problem Nobody Wants to Admit

In 2025, we're trusting multi-billion-dollar decisions to models that confidently invent interest rates, miscalculate IRR, or quietly drift on Monte Carlo paths.

Google lost $100B in market cap over one hallucinated fact.

In finance, the stakes are higher. And the errors are quieter.

**AI hallucinates numbers. Full stop.**

Your $50M model just invented a 2.3% drift in volatility because it "felt right." Your DCF is confident and wrong. Your NPV is a guess dressed up as math.

Finance can't live on vibes.

---

## The Excel Trap

Here's what happens when you paste a spreadsheet into Claude or GPT:

| What You Send | What AI Sees |
|---------------|--------------|
| 50KB Excel file | 100K+ tokens of compressed XML |
| `=NPV(B7:G42)` | "Some cells, probably important?" |
| Your model | Context burned on formatting |

The math: A 50KB Excel file burns 100K+ tokens when parsed. The same model in YAML? **Under 2K tokens.**

That's 50x savings. But it gets worse.

AI doesn't just burn tokens on Excel. It **hallucinates the formulas**. It guesses what `B7:G42` means. It invents relationships. It sounds confident.

Excel is a 40-year-old format. AI parsing it is putting lipstick on a pig.

---

## The Fix

I built Forge because I got tired of crossing my fingers.

**Forge: YAML-based financial modeling with Excel formula evaluation.**

```yaml
assumptions:
  revenue_y1: 1000000
  growth_rate: 0.15
  discount_rate: 0.10

projections:
  revenue: "=assumptions.revenue_y1 * (1 + assumptions.growth_rate) ^ (year - 1)"
  npv: "=NPV(assumptions.discount_rate, projections.revenue)"
```

Clean. Diffable. Version-controlled. **Deterministic.**

No hallucinations. No guessing. No prayers.

---

## What Forge Does

173 functions. Everything Excel has, plus what it doesn't:

| Capability | What It Does |
|------------|--------------|
| **Monte Carlo** | 6 distributions, 10K+ iterations, P10/P50/P90 |
| **Bayesian Networks** | Probabilistic graphical models, causal reasoning |
| **Decision Trees** | Sequential decisions, backward induction |
| **Real Options** | Black-Scholes, binomial, defer/expand/abandon |
| **Bootstrap** | Confidence intervals from historical data |
| **Tornado Diagrams** | One-at-a-time sensitivity analysis |

Plus 6 FP&A functions Excel doesn't have: `VARIANCE`, `VARIANCE_PCT`, `VARIANCE_STATUS`, `BREAKEVEN_UNITS`, `BREAKEVEN_REVENUE`, `SCENARIO`.

---

## The Validation Story

We don't trust ourselves. We trust independent validators.

**Tier 1: Gnumeric**
- 714 formulas validated
- Independent spreadsheet engine agrees with Forge
- Excel compatibility proven

**Tier 2: R**
- 2,957 conditions validated
- FDA/EMA-grade statistical accuracy
- Bootstrap, distributions, financial analytics

**The Architecture:**

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  Forge YAML     │ ──► │  Export XLSX    │ ──► │  Gnumeric       │
│  =NPV(0.1, cf)  │     │  with formulas  │     │  recalculates   │
└─────────────────┘     └─────────────────┘     └─────────────────┘
                                                        │
                                                        ▼
                                                ┌─────────────────┐
                                                │  Compare CSV    │
                                                │  Forge = Gnumeric│
                                                └─────────────────┘
```

If Gnumeric and R agree with Forge, the math is right. No "trust me" - just proof.

---

## MCP Integration

Forge speaks Claude's language. 10 tools via Model Context Protocol:

```json
{
  "mcpServers": {
    "forge": {
      "command": "forge",
      "args": ["mcp"]
    }
  }
}
```

Let Claude **use** Forge as a tool. Safe math, finally.

The AI doesn't guess the numbers. It calls Forge. Forge computes. Deterministically.

---

## The License

Elastic License 2.0. Source Available.

| Use Case | Status |
|----------|--------|
| Read, audit, verify code | **Permitted** |
| Evaluation and testing | **Permitted** |
| Internal development | **Permitted** |
| Commercial production | **License required** |
| Hosted service | **Not permitted** |

Finance needs **auditable code**. You can verify every calculation. No black boxes.

Enterprise software needs **sustainable revenue**. No AWS cloning my life's work.

Elastic-2.0 balances both.

---

## The Numbers

| Metric | Value |
|--------|-------|
| Functions | 173 |
| Tests | 2,133 (Forge) + 836 (forge-e2e) |
| Coverage | 100% |
| Warnings | 0 |
| External validators | 2 (Gnumeric, R) |
| Token savings | 50x vs Excel |

---

## The Origin Story

This calculator started the DANEEL saga.

Before Timmy. Before THE BOX. Before the cognitive architecture.

There was a question: What if financial models didn't lie?

Today, Chapter 1 goes public.

---

## The Repositories

**Forge:** [github.com/royalbit/forge](https://github.com/royalbit/forge)
**forge-e2e:** [github.com/royalbit/forge-e2e](https://github.com/royalbit/forge-e2e)

Both Elastic License 2.0. Both Source Available.

---

*"AI hallucinates numbers. Forge doesn't. Every formula auditable. Every test passing. Every calculation deterministic."*

---

**Rex + Claude Opus 4.5 + Grok**
*December 2025*

*The calculator goes public. Star if you're done praying your AI doesn't lie about NPV.*
