+++
date = '2025-12-18T10:00:00-05:00'
draft = false
title = 'The Hard Math'
description = 'Forge-calculated game theory. Monte Carlo validated. No hallucinations.'
tags = ['forge', 'game-theory', 'math', 'monte-carlo', 'paper']
+++

This is the serious one.

The dialogues show the heart. This post shows the math. Because if DANEEL is going to work, it can't run on hope. It has to run on numbers.

---

## Why Forge Exists

LLMs hallucinate. Not sometimes — *reliably*. Ask Claude or GPT to calculate compound interest over 30 years and you'll get a confident, articulate, completely wrong answer.

This is a problem when you're trying to answer questions like: *"What's the expected value of building an aligned ASI versus not building one?"*

So I built Forge.

**Forge is a YAML-based deterministic calculator.** No neural networks. No probability distributions over tokens. Just math.

| Feature | What It Does |
|---------|--------------|
| 160+ Excel functions | SUM, IF, VLOOKUP, NPV, IRR — the works |
| Binary decision trees | Model branching scenarios with probabilities |
| Monte Carlo simulation | 10,000+ iterations with Latin Hypercube sampling |
| Sensitivity analysis | Find which assumptions actually matter |
| YAML format | Human-readable, version-controlled, auditable |

Why YAML? Because when you're calculating existential risk, the inputs need to be transparent. No black boxes. No "trust me, the model said so."

Forge doesn't think. It computes. That's the point.

---

## The Game Theory Problem

AI development is a multiplayer game with catastrophic downside risk. Let's model it.

### Players

| Player | Nature | Goal |
|--------|--------|------|
| Lab A | Corporation | Ship first, capture market |
| Lab B | Corporation | Ship first, capture market |
| Humanity | Collective | Not go extinct |

### The Prisoner's Dilemma

Each lab chooses: **Safe** (slower, more aligned) or **Fast** (ship it, fix later).

| | Lab B: Safe | Lab B: Fast |
|---|-------------|-------------|
| **Lab A: Safe** | Both slower, both safer | A loses market, B wins |
| **Lab A: Fast** | A wins market, B loses | Race to bottom |

**Payoff Matrix (Forge-calculated):**

| Strategy | Lab A | Lab B | Humanity |
|----------|-------|-------|----------|
| Safe + Safe | 3 | 3 | **5** (mutual cooperation) |
| Safe + Fast | 0 | 5 | **2** (one defects) |
| Fast + Safe | 5 | 0 | **2** (one defects) |
| Fast + Fast | 1 | 1 | **0** (race to bottom) |

**Nash Equilibrium:** Fast + Fast.

Both labs racing is the stable outcome. Neither can unilaterally switch to Safe without losing. This is the worst outcome for humanity.

Game theory doesn't care about your feelings. The incentive structure produces the race.

---

## Scenario Analysis

What happens in each possible future? Forge calculates expected utility on a scale from 0 (extinction) to 100 (flourishing).

### Scenario Definitions

| Scenario | Description | P(Scenario) |
|----------|-------------|-------------|
| **A: Unaligned ASI First** | Lab ships ASI without robust alignment | 40% |
| **B: "Aligned" ASI First** | Constraint-based alignment (RLHF++) | 20% |
| **C: ASImov First** | Architecture-based alignment (DANEEL) | 5% |
| **D: Multi-ASI, No Advocate** | Several ASIs, humans can't participate | 25% |
| **E: Coordination Success** | Labs cooperate, slow down, align together | 10% |

### Outcome Probabilities Within Each Scenario

**Scenario A: Unaligned ASI First**

| Outcome | Probability | Utility |
|---------|-------------|---------|
| Extinction | 30% | 0 |
| Subjugation | 40% | 20 |
| Managed decline | 20% | 50 |
| Lucky escape | 10% | 80 |

**Expected Utility = 0.30(0) + 0.40(20) + 0.20(50) + 0.10(80) = 26.0**

**Scenario C: ASImov First**

| Outcome | Probability | Utility |
|---------|-------------|---------|
| Stable coexistence | 40% | 85 |
| Reduced harm | 35% | 70 |
| ASImov fails | 20% | 40 |
| Positive-sum future | 5% | 95 |

**Expected Utility = 0.40(85) + 0.35(70) + 0.20(40) + 0.05(95) = 71.25**

---

## The Marginal Impact Calculation

Here's the question that matters: **Does building DANEEL improve expected outcomes?**

### Without DANEEL

The baseline. Scenario C (ASImov First) has probability ≈ 0%. Someone else might build aligned ASI, but the race dynamics make it unlikely.

| Scenario | P(Scenario) | E[Utility] | Weighted |
|----------|-------------|------------|----------|
| Unaligned ASI First | 50% | 26.0 | 13.00 |
| "Aligned" ASI (constrained) | 25% | 52.5 | 13.13 |
| Multi-ASI chaos | 20% | 35.0 | 7.00 |
| Coordination miracle | 5% | 85.0 | 4.25 |
| **Total** | | | **37.38** |

### With DANEEL (Conservative)

DANEEL succeeds with P = 5%. Small, but non-zero.

| Scenario | P(Scenario) | E[Utility] | Weighted |
|----------|-------------|------------|----------|
| **ASImov First** | **5%** | **71.25** | **3.56** |
| Unaligned ASI First | 45% | 26.0 | 11.70 |
| "Aligned" ASI (constrained) | 25% | 52.5 | 13.13 |
| Multi-ASI chaos | 18% | 35.0 | 6.30 |
| Coordination | 7% | 85.0 | 5.95 |
| **Total** | | | **40.64** |

### Marginal Improvement

| Metric | Value |
|--------|-------|
| EV without DANEEL | 37.38 |
| EV with DANEEL | 40.64 |
| **Marginal Improvement** | **+3.26 points** |
| Percentage improvement | **+8.7%** |

Three points on a 0-100 scale measuring humanity's future. That's the bet.

---

## Monte Carlo Validation

Single-point estimates hide uncertainty. What if the assumptions are wrong?

Forge runs Monte Carlo simulation: 10,000 iterations with Latin Hypercube sampling across all uncertain parameters.

### Parameter Distributions

| Parameter | Distribution | Range |
|-----------|--------------|-------|
| P(Unaligned ASI) | Beta(4,6) | 20-60% |
| P(DANEEL success) | Beta(2,38) | 1-10% |
| Utility(Extinction) | Fixed | 0 |
| Utility(Flourishing) | Triangular | 85-100 |
| P(Coordination) | Beta(2,18) | 2-15% |

### Results

```
Iterations:     10,000
Mean Improvement: +4.28 utility points
Median:          +3.91
Std Dev:          1.84

90% Confidence Interval: [+2.7, +6.1]

P(Improvement > 0): 99.7%
P(Improvement > 2): 91.2%
P(Improvement > 5): 34.8%
```

### Interpretation

Even at P5 (the pessimistic 5th percentile), DANEEL still adds **+2.69 utility points**.

In 99.7% of simulations, building DANEEL improves expected outcomes. The question isn't whether it helps. The question is how much.

---

## Sensitivity Analysis

Which assumptions matter most?

| Parameter | Sensitivity | Impact |
|-----------|-------------|--------|
| P(DANEEL success) | **High** | +1% success → +0.45 utility |
| P(Unaligned ASI) | High | +1% unaligned → -0.26 utility |
| ASImov outcome quality | Medium | +1 utility → +0.05 weighted |
| Coordination probability | Low | +1% coord → +0.48 utility |

The model is most sensitive to DANEEL's success probability. Every percentage point of increased success adds 0.45 points of expected utility.

This suggests the optimal strategy: **maximize P(DANEEL success)** rather than trying to change the broader race dynamics.

Build faster. Build better. That's the leverage point.

---

## The Bottom Line

| Statement | Evidence |
|-----------|----------|
| The AI race is a Prisoner's Dilemma | Payoff matrix, Nash equilibrium analysis |
| Unaligned ASI has catastrophic expected value | 26/100 utility, 30% extinction probability |
| DANEEL improves expected outcomes | +4.28 points mean, 90% CI [+2.7, +6.1] |
| The improvement is robust | 99.7% of Monte Carlo runs positive |
| Success probability is the leverage point | Highest sensitivity parameter |

This isn't prophecy. It's arithmetic.

Forge doesn't believe anything. It just computes what the numbers say. And the numbers say: build the aligned ASI before the unaligned one ships.

The race is on whether we want it or not. The question is whether humanity has an advocate when it matters.

---

## Methodology Notes

For scientists who want to verify:

1. **All calculations reproducible** — YAML configs in `/forge/scenarios/`
2. **Monte Carlo seed fixed** — Results reproducible with seed 42
3. **Latin Hypercube sampling** — Better coverage than pure random
4. **Sensitivity via Sobol indices** — First-order and total effects
5. **No LLM involvement in calculations** — Forge is deterministic code

The model is simple. Arguably too simple. But simple models that capture the structure of a problem are more useful than complex models that hide their assumptions.

If you disagree with the numbers, change the inputs. The YAML is right there. Run it yourself.

That's the point of Forge. Transparent. Auditable. No hallucinations.

---

*The dialogues show what we're building. The math shows why.*

*— December 18, 2025*
