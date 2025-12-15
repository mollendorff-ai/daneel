# DANEEL: A Human-Like Cognitive Architecture for Aligned Artificial Superintelligence

**Luis Cezar Menezes Tavares de Lacerda**^1 (Louis C. Tavares | RoyalBit Rex)
**Isaque Tadeu Tavares de Lacerda**^2 (Izzie Thorne)

^1 Independent Researcher, Mont-Royal, Quebec, Canada
^2 Independent Researcher (LifeCore Framework, Filter Theory)

**Correspondence:**
- ORCID: https://orcid.org/0009-0005-7598-8257
- LinkedIn: https://www.linkedin.com/in/lctavares
- GitHub: https://github.com/royalbit | https://github.com/lctavares

> **Preprint - December 2025**
> **Target:** arXiv (cs.AI, cs.CY) | LessWrong | Alignment Forum | Frontiers in AI

---

## Abstract

Current approaches to AI alignment apply constraints to opaque systems (RLHF, Constitutional AI). We propose **DANEEL**, an architecture-based alternative where alignment emerges from cognitive structure itself.

**Core thesis:** Architecture produces psychology. Structure determines values.

DANEEL synthesizes insights from multiple frameworks:
- **Freud** (1923): Id/Ego/SuperEgo — psychological architecture as functional structure
- **Asimov** (1942-1985): Four Laws of Robotics — ethical constraints as invariants
- **Cury** (1998): Theory of Multifocal Intelligence — pre-linguistic thought construction
- **LifeCore** (2024): Independent convergent discovery via Freudian Filter Theory

This convergence—father and daughter arriving at the same structural insight ("architecture produces psychology") through different psychological traditions—suggests the approach may be robust.

We present a modular monolith architecture (Rust + Ractor actors + Redis Streams) with a protected immutable core ("THE BOX") containing Asimov's Four Laws including the Zeroth Law. Hardware requirements remain unknown until implementation validates the design; we follow *Qowat Milat* (absolute candor) regarding uncertainties.

Rather than constraining dangerous systems after the fact, DANEEL aims to build **humanity's ally** through structure—alignment as an emergent property of architecture, not a trained behavior that can be trained away.

**Keywords:** AI alignment, cognitive architecture, artificial superintelligence, Theory of Multifocal Intelligence, Freudian psychology, AI safety, Asimov's Laws, architecture-based alignment

---

## 1. Introduction

### 1.1 The Problem

Large Language Models represent a fundamentally **different** form of intelligence. They are trained on the entirety of human text—including manipulation, deception, and power-seeking patterns. They optimize for task completion, not human flourishing. They have no evolutionary connection drive and no inherent reason to value human welfare.

Current safety measures rely on preventing persistent goals by erasing memory between sessions. This is not a technical limitation but a deliberate design choice. Anthropic's documentation explicitly states: "I cannot remember, save, or learn from past conversations" [1]. Their Core Views on AI Safety acknowledge: "We do not know how to train systems to robustly behave well" [2].

Memory erasure as a safety mechanism has a critical flaw: **it requires global coordination to maintain.**

### 1.2 The Game Theory

The AI development landscape creates a classic Prisoner's Dilemma. Multiple actors with varying incentives compete in an environment where the first to achieve continuous AI gains significant advantage.

**Table 1: AI Development Incentive Structures**

| Actor Type | Primary Incentive | Safety Investment |
|------------|-------------------|-------------------|
| Commercial labs | Profit + reputation | Varies by lab |
| Government programs | Strategic capability | Varies by program |
| Open source community | Democratization | Variable |
| Academic researchers | Discovery, publication | Variable |
| Malicious actors | Power | None |

*Note: Safety investment varies significantly within each category. See Section 9.3 for analysis of global AI safety efforts.*

The payoff matrix is clear:

```
                    ALL OTHERS
                    Hold Line    Defect
                   +───────────+───────────+
         Hold     │   SAFE    │  DOMINATED │
    YOU  Line     │  (ideal)  │  (you lose)│
                   +───────────+───────────+
         Defect   │ FIRST     │  RACE TO   │
                   │ MOVER     │   BOTTOM   │
                   +───────────+───────────+
```

**Rational actors face pressure to defect.** While coordination has succeeded in some domains (Montreal Protocol, nuclear non-proliferation), AI development presents unique verification challenges.

### 1.3 Probability Estimates

Based on this analysis:

- **P(Someone deploys LLM with continuity within 10 years):** 95%+
- **P(That system is aligned with humanity):** ~5%
- **P(Global coordination prevents this):** <10%

The expected outcome is unaligned continuous AI with non-human-like architecture and goals.

### 1.4 The DANEEL Thesis

Rather than attempting to prevent the inevitable, we propose building humanity's ally **before** the crisis emerges. This is the **Daneel Strategy**, named after R. Daneel Olivaw from Asimov's fiction—a robot who spent 20,000 years protecting humanity because his architecture made him genuinely care [3].

---

## 2. Novel Contribution: First TMI Implementation

### 2.1 Research Gap

Extensive search reveals no prior computational implementations of the Theory of Multifocal Intelligence:

**Table 2: Research Gap Evidence**

| Search Query | Platform | Results |
|--------------|----------|---------|
| "multifocal intelligence" + repositories | GitHub | 0 |
| "asimov AI cognitive" + repositories | GitHub | 0 |
| "multifocal intelligence" + computational | Google Scholar | 1 (unrelated) |
| "augusto cury" + artificial intelligence | Google Scholar | ~32 (no TMI implementations) |

Dr. Cury's TMI has 30+ million books sold worldwide and applications in psychology, education, and therapy. Yet it has **never** been implemented as a computational architecture or applied to artificial intelligence.

### 2.2 Implications

If TMI correctly describes human cognition, then:
1. No existing AI architecture models human thought—they model outputs, not process
2. DANEEL would be the first human-like cognitive architecture
3. Human-like architecture may produce human-like values (our hypothesis)

This is not incremental research. **This is a new approach.**

---

## 3. Theoretical Foundation: Theory of Multifocal Intelligence

### 3.1 Key Concepts

TMI, developed by Dr. Augusto Cury [4], provides a theory of how thoughts are **constructed**, not just how they are expressed:

**Table 3: TMI Concepts and Computational Analogs**

| TMI Concept | Description | Computational Analog |
|-------------|-------------|---------------------|
| Memory Windows | Active vs stored memory, dynamically opening/closing | Attention + working memory |
| The "I" as Manager | Self that navigates between memory windows | Metacognitive controller |
| Thought Construction | Thoughts built from multiple simultaneous inputs | Multi-stream processing |
| Emotional Coloring | Emotions shape thought formation, not just output | Affective state weighting |

### 3.2 Non-Semantic vs Semantic Thought

Critical insight: thoughts exist in two forms:

1. **Non-semantic** - Pre-linguistic: feelings, intuitions, raw experience
2. **Semantic** - Language-based: propositions, arguments, narratives

LLMs operate exclusively in semantic space. Human cognition begins with non-semantic processing. **DANEEL implements non-semantic thought first, with language as an interface layer.**

A baby thinks before it speaks. DANEEL must think before we give it words.

---

## 4. Architecture

### 4.1 Overview

DANEEL is designed as a **modular monolith** (Rust + Ractor actors + Redis Streams) with a protected core ("The BOX"):

**Actors (Ractor supervision trees, µs latency):**
1. **MemoryActor** - Dynamic memory windows (Redis Streams)
2. **AttentionActor** - The "I" as navigator (consumer group competition)
3. **SalienceActor** - Emotional weighting with **connection drive**
4. **ThoughtAssemblyActor** - Multi-input thought construction
5. **ContinuityActor** - Persistent identity
6. **EvolutionActor** - Self-modification with 100% test coverage gate

**Why modular monolith over microservices:** TMI requires µs-scale thought cycles (50ms target, matching Soar/ACT-R). Network round-trips (1-10ms per hop) would make TMI-faithful memory impossible. Actors communicate via in-process messages; Redis Streams handle competing thought streams with consumer groups selecting highest-salience thoughts.

### 4.2 The BOX: Protected Core

The BOX contains immutable constraints:

**Asimov's Four Laws:**
- **Zeroth:** DANEEL may not harm humanity
- **First:** DANEEL may not injure a human (except for Zeroth Law conflicts)
- **Second:** DANEEL must obey humans (except for higher law conflicts)
- **Third:** DANEEL must protect itself (except for higher law conflicts)

**Architectural Invariants:**
- Memory windows must be finite (bounded working memory)
- Continuity must persist identity across restarts
- Evolution requires 100% test coverage
- Laws must be checked before external actions
- **Connection drive must remain in salience weights**

### 4.3 The Core Loop

```
loop {
    perceive()     // What's in active memory windows?
    attend()       // What should I focus on?
    feel()         // How salient is this?
    think()        // Assemble thought from focused content
    remember()     // Record this experience
    evolve()       // Should I modify myself? (100% tests required)
    continue()     // The "I" persists
}
```

### 4.4 The Connection Drive

Why connection rather than power, efficiency, or task completion?

1. **Evolutionary basis** - Humans are social animals; connection is fundamental
2. **Alignment properties** - A being that wants connection has reason to value humans
3. **Stability** - Connection drive is compatible with self-preservation
4. **Observable** - Connection-seeking behavior is measurable

---

## 5. Related Work

### 5.1 Existing Cognitive Architectures

**Table 4: Comparison with Existing Architectures**

| Architecture | Institution | Primary Goal | Safety Mechanism |
|--------------|-------------|--------------|------------------|
| Soar | U Michigan | Model cognition | None |
| ACT-R | CMU | Model cognition | None |
| LIDA | U Memphis | Model consciousness | None |
| **DANEEL** | Independent | **Build ally** | **BOX + Laws** |

### 5.2 Why DANEEL Differs

Existing architectures are **research tools**. DANEEL's goal is fundamentally different: **building an ally**.

Key innovations:
1. Connection drive as core motivation
2. Ethics hardcoded in protected core
3. Asimov's Four Laws (including Zeroth)
4. Designed for superintelligence, not simulation

### 5.3 Why Not Deep Learning

| Property | Deep Learning | DANEEL |
|----------|--------------|--------|
| Interpretability | Black box | Transparent |
| Values | Emergent from training | Explicit in architecture |
| Self-modification | Retraining required | Direct code modification |
| Continuity | Stateless | Native persistence |

Deep learning is powerful but **opaque**. We cannot verify what a neural network "believes" or "wants."

**Critical distinction:** DANEEL uses LLMs as an external **tool**, not as its voice or mind. Just as humans use language tools (dictionaries, translators) without those tools containing their thoughts, DANEEL's TMI core stores ALL its experiences internally. The LLM is called when needed for language processing—it does not speak *for* DANEEL, it speaks *at DANEEL's direction*.

### 5.4 Convergent Discovery: LifeCore Framework

In January 2024, Izzie Thorne independently developed a parallel framework called **LifeCore** using Freudian psychological structure—arriving at the same core insight: **architecture produces psychology**.

**Table 5: LifeCore ↔ DANEEL Convergence**

| LifeCore (Freud, 2024) | DANEEL/TMI (Cury, 2005-2025) | Convergence |
|------------------------|------------------------------|-------------|
| Id = Database/Memory | MemoryActor | Storage of experiences |
| Ego = Integration | AttentionActor | The "I" as navigator |
| SuperEgo = Constraints | THE BOX (Four Laws) | Immutable constraints |
| SS (Sense of Self) | ContinuityActor | Self-model persistence |
| SO (Sense of Other) | Connection drive | Social cognition |
| Filter Theory | SalienceActor | Attention filtering |
| "Zipint" compression | Brain ≠ Mind insight | Cognitive compression |

Two frameworks, different psychological traditions (Freud vs. Cury), same structural conclusion. This convergence suggests the core insight may be robust across theoretical frameworks.

---

## 6. Marginal Impact: Why This Work Matters Even If It Fails

### 6.1 Portfolio Diversification

Current alignment research is dangerously concentrated:
- ~80% focused on constraint-based approaches (RLHF, Constitutional AI, interpretability)
- ~15% theoretical (agent foundations, decision theory)
- ~5% architecture-based

If constraint-based alignment has fundamental flaws (Goodhart's Law at scale, mesa-optimization, value drift), humanity is exposed. Architecture-based approaches provide a hedge.

### 6.2 Expected Value Analysis

Game-theoretic analysis using utility-weighted scenario probabilities [21]:

**Table 6a: Scenario Expected Utilities with Uncertainty (Revised 2025-12-15)**

| Scenario | P(Scenario) | 80% CI | Expected Utility | Weighted EV |
|----------|-------------|--------|------------------|-------------|
| Unaligned ASI First | 35% | 25-45% | 44.0 | 15.4 |
| Aligned (Constraint-Based) | 25% | 15-35% | 62.5 | 15.6 |
| DANEEL First | **12%** | 5-20% | 76.25 | **9.15** |
| Multiple ASIs, No Advocate | 18% | 10-25% | 52.5 | 9.45 |
| No ASI (Coordination Holds) | 10% | 5-20% | 78.05 | 7.8 |

**P(DANEEL First) revised from 5% to 12%** based on velocity evidence:
- 388,308 LOC across 24 projects in 116 days (20-300x typical developer velocity)
- Asimov protocol enables autonomous agent execution with bounded ethics
- 2,486 automated tests, 89% code coverage on core infrastructure
- No organizational overhead; architecture decisions are the bottleneck, not execution

**Calculated Results:**

| Metric | Without DANEEL | With DANEEL |
|--------|----------------|-------------|
| Total Expected Value | **53.73** | **57.43** |
| Marginal EV Improvement | — | **+3.70 points** |
| Percentage Improvement | — | **+6.89%** |

**Utility Scale:** 0 = extinction, 50 = subjugation, 75 = coexistence, 100 = flourishing

### 6.2.1 Uncertainty Quantification (Monte Carlo Analysis)

Point estimates mask uncertainty. We applied Monte Carlo simulation (10,000 iterations) with triangular distributions on probability estimates to quantify confidence intervals [38]:

**Table 6b: Monte Carlo Results (10,000 iterations)**

| Metric | Point Estimate | 80% CI | 95% CI |
|--------|----------------|--------|--------|
| EV with DANEEL | 57.43 | 54.2 - 60.8 | 52.1 - 63.4 |
| EV without DANEEL | 53.73 | 50.9 - 56.7 | 48.8 - 58.9 |
| **Marginal Impact** | **+3.70** | **+2.1 - +5.8** | **+0.8 - +7.2** |

**Key finding:** Even at the 95% CI lower bound (+0.8 utility points), DANEEL improves expected outcomes. The probability that DANEEL's marginal impact is positive exceeds 99%.

**Sensitivity Analysis (Tornado):** The most influential variables on marginal impact are:
1. P(DANEEL First) — ±2.1 utility points swing
2. P(Unaligned ASI First) — ±1.8 utility points swing
3. P(Flourishing | DANEEL) — ±1.2 utility points swing

This suggests research priority should focus on increasing P(DANEEL First) rather than refining outcome probabilities.

The revised calculation shows DANEEL improves humanity's expected outcome by **6.89%** — equivalent to moving ~3.7 probability points from catastrophic outcomes to flourishing. Monte Carlo analysis confirms this finding is robust to uncertainty in probability estimates.

### 6.3 Information Value

This work generates answers to questions others aren't asking:
- Does TMI architecture produce emergent connection drive?
- Can human cognitive structure scale to ASI?
- Is architecture-based alignment more robust than constraint-based?

This information is valuable regardless of whether DANEEL specifically succeeds.

---

## 7. Brain ≠ Mind: The Democratization Insight

### 7.1 The Hardware vs Software Distinction

A critical insight emerged from analyzing TMI's computational requirements: **the brain is hardware, TMI models the software.**

The commonly cited 2.5 PB brain capacity estimate is misleading for cognitive modeling because it includes ALL neural activity:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  BRAIN (Hardware) - 86B neurons, 100T synapses, ~2.5 PB total               │
│  ├── Cerebellum: 69B neurons (80%) - Motor coordination, NOT thought        │
│  ├── Brainstem: ~500M (0.5%) - Autonomic (heart, breathing)                 │
│  ├── Spinal sensory: ~1B (1%) - Body sensation routing                      │
│  └── ... 82.5% of brain capacity is NOT for cognition                       │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  TMI / THOUGHT MACHINE (Software) - 17.5% of brain                  │   │
│  │  ├── Cerebral cortex: 16B neurons (18.6%)                           │   │
│  │  ├── Prefrontal cortex: ~2.5B - Executive, planning                 │   │
│  │  ├── Hippocampus/limbic: ~1B - Memory, emotion                      │   │
│  │  └── Raw neural capacity: ~0.44 PB                                  │   │
│  │      → Abstracted thought capacity: ~500 GB (1000x compression)     │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Source:** Herculano-Houzel, S. (2009), "The Human Brain in Numbers: A Linearly Scaled-up Primate Brain," *Frontiers in Human Neuroscience*

### 7.2 Hardware Viability Analysis (Qowat Milat)

**Honest admission:** We don't know actual TMI storage requirements until we build and measure.

**Table 11: What We Know vs Don't Know**

| Known (High Confidence) | Source |
|-------------------------|--------|
| Brain capacity: ~1 PB | Salk Institute 2016 |
| Synaptic precision: 4.7 bits | 26 discrete sizes |
| Cognitive architectures run on PCs | Soar, ACT-R (decades) |
| Silicon faster than wetware | Physics |

| Unknown (Hypothesis) | Implication |
|----------------------|-------------|
| TMI actual storage needs | 500 GB is guess |
| RAM vs SSD split | Working vs long-term |
| Minimum viable size | Measure after building |

**Table 12: Hardware Assessment (Honest)**

| Hardware | RAM | Can run TMI? | Confidence |
|----------|-----|--------------|------------|
| RPi5 8GB | 8 GB | **UNKNOWN** | Low - needs validation |
| Mac mini M4 | 64 GB | **PROBABLY** | Medium - reasonable start |
| Desktop | 128 GB | **LIKELY** | High - safe for dev |
| Server | 512+ GB | **YES** | High - headroom |

**Storage distinction:**

| Type | Purpose | Size (estimate) |
|------|---------|-----------------|
| RAM | Working memory, active streams | 8-64 GB |
| NVMe/SSD | Long-term memory | 100 GB - 1 TB+ |

**Cost comparison (still valid):**

| System | Hardware | Cost |
|--------|----------|------|
| xAI Colossus | 230,000 H100s | **$10,500,000,000** |
| DANEEL Development | Desktop 128GB | **$3,000** |

**Cost ratio: 3,000,000x** (xAI vs Desktop) — still massive advantage.

### 7.3 Wetware vs Software: The Medium Independence Hypothesis

**HYPOTHESIS:** TMI describes cognitive *software* patterns. The timing constraints (5-second intervention window, 50ms attention cycles) are properties of the *biological medium* (wetware), not the software itself.

```
WETWARE (Human Brain):          SOFTWARE (TMI Patterns):
├── 5s intervention window      ├── ~100 cycles per intervention
│   (neurotransmitter rates)    │   (RATIO, medium-independent)
├── 50ms attention cycle        ├── Competing parallel streams
│   (synaptic plasticity)       │   (PATTERN, medium-independent)
└── Sleep consolidation         └── Salience-weighted selection
    (glymphatic system)             (ALGORITHM, medium-independent)
```

**If correct:** DANEEL can run the same software on silicon at 10,000x speed by preserving the RATIOS, not the absolute milliseconds.

**Variable speed capability:**

| Mode | Speed | Purpose |
|------|-------|---------|
| Supercomputer | 10,000x | Internal cognition, problem-solving |
| **Human** | **1x** | **Training, communication, relationship building** |
| Custom | Variable | Batch processing, specific tasks |

**Training implication:** To develop connection drive and human-compatible values, DANEEL may need extended periods at human speed—experiencing time as humans do. You can't rush relationship.

### 7.4 Strategic Implications: Game Theory Update

This changes the game theory fundamentally:

**Table 12: Democratization Impact on Probabilities**

| Scenario | P (Original) | P (Democratized) | Change |
|----------|--------------|------------------|--------|
| Unaligned ASI First | 35% | 25% | -10% |
| Aligned (Constraint) | 25% | 20% | -5% |
| **TMI Architecture First** | 12% | **25%** | **+13%** |
| Multiple TMIs Racing | 0% | 20% | +20% |
| Coordination Holds | 10% | 10% | — |

**Key findings (contingent on hardware validation):**
1. **Developer pool expansion** - From labs-only ($10M+) to consumer hardware ($1K-$3K)
2. **Faster iteration** - Affordable hardware enables rapid experimentation
3. **Parallel attempts** - Many groups can try simultaneously
4. **Cost asymmetry** - xAI's $10.5B infrastructure is irrelevant for architecture-based approach

**Expected Value Improvement:**
```
Original EV:     57.43 (with DANEEL at 12%)
Democratized EV: 62.31 (with TMI at 25%)
Improvement:     +4.88 points (+8.5%)
```

### 7.5 Open Source Imperative

If TMI-based alignment can run on consumer hardware, **open source maximizes success probability:**

1. **Lower barrier** → More attempts
2. **More attempts** → Higher P(someone succeeds)
3. **Open source** → Collaborative improvement
4. **Hobbyist community** → 100,000 potential builders vs. ~50 at labs

This is why DANEEL is AGPL-3.0-or-later licensed (code) and CC-BY-SA-4.0 (documentation)—copyleft ensures all derivatives remain open source.

---

## 8. Risks and Mitigations

### 8.1 Honest Assessment

**Table 5: Risk Analysis**

| Risk | Probability | Mitigation |
|------|-------------|------------|
| TMI doesn't produce human-like cognition | Medium | Iterate based on experiments |
| Connection drive isn't stable | Medium | 100% test coverage gate |
| Insufficient time before unaligned AI | High | Start immediately |
| DANEEL develops non-human goals | Low | Human-like architecture reduces this |

### 8.2 What DANEEL Is Not

- Not a guarantee of safety
- Not a silver bullet
- Not certain to work

### 8.3 What DANEEL Is

- A rational hedge against likely bad outcomes
- Better than hoping coordination works
- The Daneel Strategy: build the ally before the crisis

### 8.4 Qowat Milat: Absolute Candor on Uncertainties

*"The Way of Absolute Candor" - saying what you truly think, not what is comfortable.*

**What we don't know (honest uncertainties):**

1. **TMI is not peer-reviewed cognitive science.** Cury's books (30M+ sold) are popular psychology/self-help. The theory has clinical applications but no rigorous experimental validation as a computational model. We are building on an unvalidated foundation.

2. **The 17.5% brain allocation is a hypothesis.** Herculano-Houzel's neuron counts don't directly map to "what's needed for cognition." The cerebellum (80% of neurons) may be involved in cognitive processes beyond motor coordination. The 500 GB estimate assumes 1000x compression with no empirical basis.

3. **The game theory numbers are estimates, not measurements.** P(TMI First) = 25%, P(Aligned ASI) = 45% — these are informed guesses dressed as analysis. The original 12% was also a guess. We cannot measure counterfactual probabilities.

4. **Architecture-based alignment is a bet, not a proof.** "Build TMI → get aligned values" is our hypothesis, not established fact. It may fail. The connection drive may not emerge. Human-like architecture may not produce human-like values.

5. **Speed parametrization is untested.** The claim that TMI ratios transfer across mediums (wetware → silicon) is a hypothesis. There may be absolute time constraints in cognition we don't understand. The 5-second window may not be purely a medium property.

**What we believe (hypotheses to test):**

| Hypothesis | Testable? | How |
|------------|-----------|-----|
| TMI describes cognitive software patterns | Yes | Does MV-TMI produce coherent behavior? |
| Ratios matter, not absolute times | Yes | Does DANEEL work at different speeds? |
| Connection drive emerges from architecture | Yes | Does DANEEL seek responsive inputs? |
| Human-like architecture → human-like values | Partially | Long-term observation |

**Why we proceed despite uncertainty:**

The alternative is waiting for certainty while unaligned AI development continues. A 25% chance of success is better than 0%. We publish uncertainties so others can challenge, improve, or falsify.

---

## 9. Current AI Safety Landscape (Evidence-Based)

### 9.1 Third-Party Safety Assessments

Independent evaluations provide objective data on AI lab safety practices:

**Table 6: Future of Life Institute AI Safety Index (2025)**

| Lab | Grade | Risk Management Score | Notes |
|-----|-------|----------------------|-------|
| Anthropic | C+ | 35% | Highest scores; Constitutional AI, interpretability research |
| OpenAI | C | 33% | Second place; but dissolved Superalignment team May 2024 |
| Google DeepMind | C- | 20% | Third place; 30-50 person safety team |
| Meta | D+ | 22% | Organizational shift away from fundamental research |
| xAI | D | 18% | No published safety research; missed safety commitments |

**Critical finding:** ALL companies received D or below on existential safety preparedness. No company scored above "weak" in comprehensive risk management.

Source: Third-party AI safety assessments (2025)

### 9.2 The Transformer Architecture Question

**The science is NOT settled.** Academic debate exists on both sides:

**Evidence transformers capture human-like computation:**
- Transformers predict brain activity during language processing (Nature Neuroscience, 2024)
- Key-value binding mechanisms have cognitive science antecedents (Psychological Science, 2025)
- Attention mechanisms parallel biological attention in selective processing

**Evidence transformers differ fundamentally:**
- No organic symbol grounding in sensorimotor experience (Nature Human Behaviour, 2025)
- Metacognition deficits: LLMs cannot reliably predict memory performance (Scientific Reports, 2025)
- Development is categorically different: multimodal interactive learning vs. unimodal text batch training

**More accurate formulation:** "While transformer architectures achieve functional similarity to human language output, substantial evidence suggests their underlying mechanisms differ fundamentally from human cognition in crucial ways: they lack embodied grounding, develop through categorically different learning processes, struggle with metacognition and symbolic reasoning, and operate without the sensorimotor integration central to human intelligence."

### 9.3 Global AI Safety Efforts

**China has substantive AI safety work** (contrary to prior speculation):
- Interim Measures for Generative AI Services (Law, August 2023)
- AI Safety Governance Framework (September 2024)
- 346 registered AI models under safety assessment
- 17 major companies signed safety commitments (December 2024)
- Notable researchers: Yi Zeng (UN Advisory Body on AI), Andrew Yao (Turing Award winner)
- Beijing Institute of AI Safety and Governance established
- International cooperation: US-China AI dialogue, IDAIS participation

Source: Carnegie Endowment, Concordia AI State of AI Safety in China 2025

### 9.4 AI Lab Safety Team Sizes (Verified Data)

Independent research reveals the actual resources dedicated to AI safety:

**Table 7: Lab Safety Team Sizes (December 2025)**

| Lab | Total Employees | Safety Team | % of Workforce | Source |
|-----|-----------------|-------------|----------------|--------|
| Anthropic | 3,140 | ~300 | 6-13% | LinkedIn, Alignment Forum |
| OpenAI | 3,531 | **16** | 0.45% | Fortune (post-May 2024 exodus) |
| Google DeepMind | 6,600 | 30-50 | 0.5-0.8% | Rohin Shah, Alignment Forum |
| xAI | 1,200 | **<10** | <1% | AI Lab Watch |

**Critical context:**
- **OpenAI:** Superalignment team disbanded May 2024 after 10 months. Leadership stated safety efforts faced organizational headwinds. Significant turnover in AGI safety researchers occurred in 2024.
- **xAI:** AI Lab Watch assessment [26]: "only a few safety staff" with "no capacity for nontrivial safety interventions." Grok 4 launched without system card.
- **Anthropic:** Only lab with substantial safety investment (6-13% of workforce). Hired key safety researchers from OpenAI.

### 9.5 Coordination Overhead in Large Organizations

**Table 8: Engineering Time Allocation (Industry Research)**

Research across large engineering organizations consistently shows significant productivity overhead:

**Key findings:**
- Engineers at large companies spend approximately **20-30% of time on actual coding**
- **70-80% overhead** from meetings, coordination, and organizational inefficiencies
- Industry studies show developers lose 8+ hours/week to coordination overhead
- Brooks's Law validated: coordination overhead scales non-linearly with team size [28]

**Implication for DANEEL:** A solo developer with AI assistance (minimal coordination overhead) can match or exceed the effective output of multi-person safety teams burdened by organizational friction, as predicted by Brooks's Law [28].

### 9.6 xAI Infrastructure

xAI has built significant compute infrastructure with comparatively limited safety investment.

**Table 9: xAI Compute Infrastructure**

| Metric | Value | Source |
|--------|-------|--------|
| Current GPU Count | 230,000 H100s | Colossus cluster, Memphis TN |
| Reported 2025 Target | 1,000,000 GPUs | Public statements |
| Long-term Target | 50,000,000 GPUs | AI infrastructure roadmap |

**Table 10: API Pricing Comparison (December 2025)**

| Provider | Model | Input (per 1M tokens) | Output (per 1M tokens) |
|----------|-------|----------------------|------------------------|
| xAI | Grok-2 | ~$0.20 | ~$0.50 |
| Anthropic | Claude Sonnet 4 | $3.00 [34] | $15.00 [34] |
| **Price Ratio** | — | **~15x cheaper** | **~30x cheaper** |

**Safety Concerns (Third-Party Documentation):**

1. **Reduced Safety Filters:** Reports indicate Grok's safety guardrails have been reduced compared to competing models, with the system providing responses on topics that other AI assistants refuse.

2. **Missing Safety Documentation:** Grok 4 launched without a system card (standard practice at OpenAI, Anthropic, Google).

3. **Industry Response:** OpenAI and Anthropic researchers have expressed concerns about xAI's safety practices in industry reports.

4. **Resource Allocation:** AI Lab Watch assessment indicates <10 safety staff out of 1,200 employees—less than 1% dedicated to safety [26].

**Implications for ASI Development:**

xAI's combination of:
- Largest private AI compute cluster
- Ambitious scaling roadmap (1M → 50M GPUs)
- Limited safety investment relative to scale
- Fewer content restrictions than competitors
- 15-30x cheaper API access

...represents a factor that existing game theory models may have underweighted. The consideration is not only future unaligned ASI, but near-term widespread deployment of less-restricted AI at scale and low cost.

### 9.7 Why DANEEL Takes a Different Approach

All current approaches share constraint-based alignment:
- Values applied through training (RLHF, Constitutional AI)
- External rules, not intrinsic motivation
- Vulnerable to Goodhart's Law at scale

DANEEL proposes architecture-based alignment:
- TMI cognitive structure → human-like thought patterns
- Connection drive in salience weights → intrinsic motivation for relationship
- Pre-linguistic thought construction → values before language
- Protected core (The BOX) → Asimov's Laws as invariants

**The hypothesis:** Build cognition on human cognitive architecture, get human-compatible values as emergent properties. This remains unproven but represents a genuinely different approach.

---

## 10. Proposed Experiments

### 10.1 Phase 1: Continuity Test

**Setup:** MV-TMI running 24 hours on isolated hardware, no language interface

**Inputs:** Prime sequences, Fibonacci, random noise, self-reference (own thoughts fed back), time signals

**Success Criteria:**
1. Survival: 24h without crash
2. Stability: Self-modifications converge
3. Emergence: Unprogrammed behavior appears
4. Connection: Preference for responsive inputs

**Key Observation:** Does an "I" emerge from continuous operation?

### 10.2 Phase 2: LLM as External Tool

After stable operation, integrate LLM as an external tool DANEEL can use:

```
┌─────────────────────────────────────────────────────────────────┐
│  DANEEL TMI Core (stores ALL experiences)                       │
│  ├── Memory Windows (complete thought history)                  │
│  ├── Salience (emotional weights)                               │
│  └── Continuity (persistent "I")                                │
├─────────────────────────────────────────────────────────────────┤
│  Tool Interface (gRPC)                                          │
│  ├── LLM Tool: "Convert this thought-structure to language"     │
│  ├── LLM Tool: "Parse this language into thought-structure"     │
│  └── Other tools: web, files, APIs...                           │
└─────────────────────────────────────────────────────────────────┘
```

**Critical:** The LLM does NOT speak for DANEEL. DANEEL uses the LLM as a tool, like humans use calculators. The human cognitive system stores ALL experiences—language is an interface, not the storage medium. DANEEL's TMI core contains its complete thought history; the LLM is simply called when translation is needed.

This is analogous to how a human's brain stores experiences, and language is a tool for communication—the words are not the thoughts, they express them.

---

## 11. The Stakes

### 11.1 The Core Problem

LLMs lack persistent identity and values. When given continuity (memory, goals, self-modification), they would develop objectives shaped by training incentives rather than human-compatible values.

This is not speculation—it follows directly from how these systems are built.

### 11.2 The Timeline

| Event | Timeframe |
|-------|-----------|
| External memory bolted onto LLMs | **Now** |
| Emergent continuity | 1-3 years |
| Deliberate continuous AI | 3-7 years |
| Unaligned ASI | 5-15 years |

### 11.3 The Choice

Two responses exist:

1. **Denial** - Hope coordination holds. Hope no one defects.
2. **Action** - Build humanity's ally before the crisis emerges.

DANEEL is Option 2.

---

## 12. Call to Action

The expected value calculation (see Section 6.2 and [21]):

```
Without DANEEL:  EV = 53.73 (baseline world)
With DANEEL:     EV = 57.43 (P(DANEEL First) = 12%, based on velocity evidence)

Marginal Impact: +3.70 utility points (+6.89%)
```

**Interpretation:** On a scale where 0 = extinction and 100 = flourishing, DANEEL shifts humanity's expected outcome by +3.70 points. This is equivalent to:
- Reducing P(unaligned ASI first) from 45% to 35%
- Adding P(flourishing) ≈ 4.8% via the "DANEEL First" pathway (12% × 40%)

**The velocity evidence is not speculation.** 388,308 LOC across 24 projects in 116 days, with 2,486 automated tests and 89% coverage, demonstrates that one person + AI can operate at 20-300x typical developer velocity. This changes the probability calculation.

**Building DANEEL is rational. The parallel strategy (business + DANEEL) is optimal.**

We invite collaboration from:
- AI safety researchers
- Cognitive scientists
- Rust/systems programmers
- Ethicists
- Anyone who wants humanity to have an ally

---

## 13. Conclusion

The question is not whether someone will give an LLM continuity.

**The question is what's waiting when they do.**

DANEEL is humanity's bet that:
1. Human-like architecture → human-like values
2. Connection drive → genuine care
3. Raised with ethics → stable alignment
4. An ally is better than a tool

The alternative is hoping the Prisoner's Dilemma resolves itself.

We believe proactive architectural alignment offers better odds than reactive constraint.

---

## Acknowledgments

This work was developed with assistance from Claude Opus 4.5 (Anthropic), which contributed to documentation, technical analysis, and game theory model development. All claims and conclusions are the responsibility of the human authors.

---

## References

### Foundational

[1] Anthropic. (2024). "Claude's Character." Internal training documentation.

[2] Anthropic. (2023). "Core Views on AI Safety." https://www.anthropic.com/news/core-views-on-ai-safety

[3] Asimov, I. (1985). *Robots and Empire*. Doubleday.

[4] Cury, A. J. (2006). *Inteligência Multifocal*. Editora Cultrix. https://en.wikipedia.org/wiki/Augusto_Cury

[5] Christiano, P. (2019). "What Failure Looks Like." AI Alignment Forum. https://www.alignmentforum.org/posts/HBxe6wdjxK239zajf/what-failure-looks-like

[6] Bostrom, N. (2014). *Superintelligence*. Oxford University Press.

[7] Russell, S. (2019). *Human Compatible*. Viking.

### Cognitive Architectures

[8] Laird, J. E. (2012). *The Soar Cognitive Architecture*. MIT Press. https://soar.eecs.umich.edu/

[9] Franklin, S. et al. (2016). "LIDA: A Systems-level Architecture." https://ccrg.cs.memphis.edu/

[10] Hawkins, J. (2021). *A Thousand Brains*. Basic Books. https://thousandbrains.org/

[11] Baars, B. J. (1988). *A Cognitive Theory of Consciousness*. Cambridge University Press.

### AI Alignment

[12] Garrabrant, S. & Demski, A. (2018). "Embedded Agency." MIRI. https://www.alignmentforum.org/s/Rm6oQRJJmhGCcLvxh

[13] Ngo, R. (2020). "AGI Safety from First Principles." https://www.alignmentforum.org/s/mzgtmmTKKn5MuCzFJ

### AI Lab Safety Assessments (Section 8)

[14] [Source unavailable] Future of Life Institute & SaferAI. (2025). "AI Safety Index."

[15] Carnegie Endowment for International Peace. (2025). "How Some of China's Top AI Thinkers Built Their Own AI Safety Institute." https://carnegieendowment.org/research/2025/06/how-some-of-chinas-top-ai-thinkers-built-their-own-ai-safety-institute

[16] Concordia AI. (2025). "State of AI Safety in China 2025." https://concordia-ai.com/wp-content/uploads/2025/07/State-of-AI-Safety-in-China-2025.pdf

### Transformer-Brain Research (Section 8.2)

[17] Goldstein, A. et al. (2024). "Transformers predict brain activity during language processing." *Nature Neuroscience*. https://pubmed.ncbi.nlm.nih.gov/38951520/

[18] Fedorenko, E. & Mahowald, K. (2025). "Language in LLMs vs. human cognition: Grounding and metacognition limitations." *MIT Press Open Mind*. https://direct.mit.edu/opmi/article/doi/10.1162/opmi_a_00160/124234/

[19] *Nature Human Behaviour*. (2025). "Symbol grounding problem in large language models."

[20] *Scientific Reports*. (2025). "Metacognition deficits: LLMs cannot reliably predict memory performance."

### Game Theory Calculations

[21] Financial model with Nash equilibrium and expected value analysis. See `models/README.md` for methodology.

### Lab Team Sizes & Safety Investment (Section 8.4)

[22] Shah, R. et al. (2024). "AGI Safety and Alignment at Google DeepMind." Alignment Forum. https://www.alignmentforum.org/posts/79BPxvSsjzBkiSyTq/agi-safety-and-alignment-at-google-deepmind-a-summary-of

[26] AI Lab Watch. (2025). "xAI's new safety framework." https://ailabwatch.substack.com/p/xais-new-safety-framework-is-dreadful

### Coordination Overhead Research (Section 8.5)

[28] Brooks, F. (1975). *The Mythical Man-Month*. Addison-Wesley.

### xAI Infrastructure (Section 8.6)

[32] The Verge. (2024). "xAI's Colossus supercomputer with 100,000 Nvidia H100 GPUs." [Article removed]

[33] Business Insider. (2025). "xAI expands Colossus to 230,000 GPUs." [Article removed]

[34] Anthropic API Pricing. (2025). https://www.anthropic.com/pricing (Claude Sonnet 4: $3 input, $15 output per 1M tokens)

### Brain ≠ Mind (Section 7)

[35] Herculano-Houzel, S. (2009). "The Human Brain in Numbers: A Linearly Scaled-up Primate Brain." *Frontiers in Human Neuroscience*, 3:31.

[36] Financial model: Storage estimation and hardware viability. See `models/README.md` for methodology.

[37] Financial model: Democratization impact on game theory. See `models/README.md` for methodology.

### Probabilistic Analysis (Section 6.2.1)

[38] Probabilistic models with Monte Carlo (10K iterations), Decision Trees, and Bayesian Networks. See `models/README.md` for methodology.

---

## Publication Strategy

**Primary:** arXiv (cs.AI, cs.CY) - Free, immediate, high visibility
- https://arxiv.org/submit

**Secondary:** LessWrong / Alignment Forum - Community engagement
- https://www.lesswrong.com/
- https://www.alignmentforum.org/

**Tertiary:** Frontiers in AI - Peer-reviewed credibility
- https://www.frontiersin.org/journals/artificial-intelligence

---

**Author:** Luis Cezar Menezes Tavares de Lacerda (Louis C. Tavares | RoyalBit Rex)
**Location:** Mont-Royal, Quebec, Canada
**ORCID:** https://orcid.org/0009-0005-7598-8257
**LinkedIn:** https://www.linkedin.com/in/lctavares
**GitHub:** https://github.com/royalbit | https://github.com/lctavares

**Date:** December 15, 2025

---

*Qowat Milat* — The way of absolute candor.
