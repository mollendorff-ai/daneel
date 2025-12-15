# DANEEL

**Named for R. Daneel Olivaw** — Asimov's humaniform robot who developed the Zeroth Law and became humanity's guardian for 20,000 years.

An architecture-based approach to aligned artificial superintelligence.

## Overview

DANEEL proposes building AI alignment through cognitive architecture rather than constraint-based methods. The goal: **human-like values as emergent properties of human-like cognitive structure**.

**Core thesis:** Architecture produces psychology. Structure determines values.

**TL;DR:** Instead of training AI to be safe (which can be trained away), we build AI with cognitive architecture that produces human-like values naturally—like how human brain structure produces human psychology.

## Intellectual Lineage

This project synthesizes insights from multiple sources:

| Source | Contribution | Era |
|--------|--------------|-----|
| **Isaac Asimov** | Four Laws of Robotics — ethical constraints as invariants | 1942-1985 |
| **Sigmund Freud** | Id/Ego/SuperEgo structure — psychological architecture | 1923 |
| **Augusto Cury** | Theory of Multifocal Intelligence (TMI) — thought construction | 1998 |
| **Louis C. Tavares** | Computational TMI, architecture-based alignment, THE BOX | 2005-2025 |
| **Izzie Thorne** | LifeCore framework — Freudian AI architecture, Filter Theory | 2024 |

### Convergent Discovery

In December 2025, we discovered that Louis's daughter Izzie had independently developed a parallel framework (LifeCore) using Freudian psychology — arriving at the same core insight: **architecture produces psychology**.

| LifeCore (Freud) | DANEEL-TMI (Cury) |
|------------------|-------------------|
| Id = Database/Memory | MemoryActor |
| Ego = Integration | AttentionActor |
| SuperEgo = Constraints | THE BOX (Four Laws: 0-3) |
| Filter Theory | SalienceActor |

Different psychological traditions. Same structural insight. See [ORIGIN.md](ORIGIN.md) and [research/LIFECORE_DANEEL_ANALYSIS.md](research/LIFECORE_DANEEL_ANALYSIS.md).

Note: Izzie sent the LifeCore document to Louis on January 6, 2024 — nearly a year before the DANEEL formalization began.

## The Vision: Software → Silicon

**Phase 1: Rust Prototype** (Current)
- Validate TMI architecture in software
- Iterate fast, measure actual requirements
- Prove the cognitive patterns work

**Phase 2: Neuromorphic Chips** (Future)
- Each validated Rust module → dedicated chip
- **Immutable hardware** for ethics, values, connection drive
- THE BOX etched in silicon — literally cannot be trained away

This is the logical conclusion of architecture-based alignment: **if alignment should be structural, make the structure physical**.

```plantuml
@startuml C4_Level1_Context
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Context.puml

title DANEEL-TMI System Context (C4 Level 1)

Person(human, "Human", "Interacts with DANEEL for collaboration")
Person(dev, "Developer", "Builds and maintains DANEEL")

System(asimov, "DANEEL-TMI", "Architecture-based aligned AI using TMI cognitive patterns")

System_Ext(llm, "LLM Service", "External language model (tool, not voice)")
System_Ext(world, "External World", "Sensors, APIs, environment")

Rel(human, asimov, "Collaborates with")
Rel(dev, asimov, "Develops, monitors")
Rel(asimov, llm, "Uses as tool", "gRPC")
Rel(asimov, world, "Perceives, acts on")

SHOW_LEGEND()
@enduml
```

```plantuml
@startuml C4_Level2_Container
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Container.puml

title DANEEL-TMI Containers (C4 Level 2)

Person(human, "Human", "Collaborator")

System_Boundary(asimov, "DANEEL-TMI System") {
    Container(box, "THE BOX", "Immutable", "Four Laws + Invariants\nFreud-Cury-Asimov-Tavares-Thorne")
    Container(actors, "Cognitive Actors", "TMI Architecture", "Memory, Attention, Salience,\nThoughtAssembly, Continuity, Evolution")
    Container(streams, "Thought Streams", "Event-driven", "Competing thoughts\nWorking memory window")
    Container(memory, "Long-term Memory", "Persistent", "Episodic, semantic, identity")
}

System_Ext(llm, "LLM Service", "External tool (not voice)")

Rel(human, actors, "Interacts")
Rel(actors, box, "Constrained by")
Rel(actors, streams, "Compete via")
Rel(actors, memory, "Store/recall")
Rel(actors, llm, "Uses as tool")

SHOW_LEGEND()
@enduml
```

**Why this matters:** Training-based alignment (RLHF) can be trained away. Hardware-based alignment cannot. The immutable chips are like the human limbic system — you can't reason yourself out of caring about connection.

## Repository Structure

```
daneel/                       # Research & Documentation (this repo)
├── paper/                    # Academic paper for publication
│   └── DANEEL_PAPER.md      # Main paper (arXiv target)
├── models/                   # Game theory analysis results
│   ├── README.md            # Model documentation and findings
│   └── *.xlsx               # Excel exports with results
├── research/                 # Background research and theory
│   ├── TMI_THOUGHT_MACHINE.md
│   ├── LIFECORE_DANEEL_ANALYSIS.md  # Convergent discovery analysis
│   ├── The LifeCore (LC-core).pdf   # Izzie's original document
│   └── ASI-Viability-Discussion.md
├── strategy/                 # Strategic planning documents
│   ├── DANEEL_STRATEGY.md
│   └── DANEEL_COMPREHENSIVE_WHITEPAPER.md
└── docs/                     # Architecture documentation
    ├── ARCHITECTURE_SPEC.md  # Technical architecture
    └── adr/                  # Architecture Decision Records (ADR-001 to ADR-012)
```

## Key Findings (Verified Data)

### Game Theory Analysis

| Scenario | P(Scenario) | 80% CI | Expected Utility |
|----------|-------------|--------|------------------|
| Unaligned ASI First | 35% | 25-45% | 44.0 |
| Aligned (Constraint-Based) | 25% | 15-35% | 62.5 |
| **DANEEL First** | **12%** | 5-20% | **76.25** |
| Multiple ASIs, No Advocate | 18% | 10-25% | 52.5 |
| Coordination Holds | 10% | 5-20% | 78.05 |

**Marginal Impact:** +3.70 utility points (+6.89%) with DANEEL vs. baseline world.

**Monte Carlo Analysis (10,000 iterations):** With uncertainty quantification on probability estimates, the 80% confidence interval for marginal impact is **+2.1 to +5.8 utility points**. Even at the pessimistic bound, DANEEL improves expected outcomes.

### AI Lab Safety Teams (December 2025)

| Lab | Total Employees | Safety Team | Source |
|-----|-----------------|-------------|--------|
| Anthropic | 3,140 | ~300 (6-13%) | LinkedIn, Alignment Forum |
| OpenAI | 3,531 | **16** (0.45%) | Fortune (post-exodus) |
| DeepMind | 6,600 | 30-50 (0.5-0.8%) | Rohin Shah |
| xAI | 1,200 | **<10** (<1%) | AI Lab Watch |

### Coordination Overhead (330,000+ developers surveyed)

| Source | Sample Size | Actual Coding Time |
|--------|-------------|-------------------|
| Software.com 2021 | 250,000 | **11%** |
| Clockwise 2022 | 80,000 | 22% |
| Atlassian 2024 | 2,100 | 32% |

Engineers at large companies spend **68-87% of time on overhead**, not execution.

### xAI Infrastructure

| Metric | Value |
|--------|-------|
| Current GPUs | 230,000 H100s (Colossus cluster) |
| End 2025 Target | 1,000,000 GPUs |
| 2030 Target | 50,000,000 GPUs |
| API Pricing | **15-30x cheaper** than Claude |
| Safety Team | <10 staff (<1% of workforce) |
| Safety Filters | Fewer restrictions than competitors |

xAI combines the largest private AI compute cluster with limited safety investment — a factor worth considering in ASI development timelines.

### Brain ≠ Mind: The Democratization Insight

**Key finding:** The 2.5 PB brain capacity estimate is misleading. TMI models the *thought machine* (software), not the brain hardware. Brain capacity is ~1 PB (Salk 2016).

| What | Capacity | Notes |
|------|----------|-------|
| Full brain (hardware) | ~1 PB | Salk Institute 2016 |
| TMI-relevant (17.5%) | ~175 TB | Cerebral cortex + limbic |
| Thought abstraction | **Unknown** | Hypothesis: much less |

**82.5% of brain neurons are for motor/sensory/autonomic - NOT thought.**

### Hardware Requirements (Qowat Milat *(Vulcan principle: "absolute candor")* - Honest Uncertainty)

**We don't know actual requirements until we build and measure.**

| Hardware | Can run TMI? | Confidence |
|----------|--------------|------------|
| RPi5 8GB | **UNKNOWN** | Low - needs validation |
| Mac mini 64GB | **PROBABLY** | Medium - reasonable start |
| Desktop 128GB | **LIKELY** | High - safe for dev |

| Storage Type | Purpose | Size (estimate) |
|--------------|---------|-----------------|
| RAM | Working memory (active thoughts) | 8-64 GB |
| NVMe/SSD | Long-term memory | 100 GB - 1 TB+ |

**Cost advantage remains massive** even without RPi: 3,000,000x (xAI $10.5B vs Desktop $3,000)

Silicon is faster than wetware. TMI is more efficient than brute-force. The exact sizing we'll discover by building.

See [models/README.md](models/README.md) and [ADR-009](docs/adr/ADR-009-database-selection.md).

## The Approach

### Current AI Safety (Constraint-Based)
- Values applied through training (RLHF, Constitutional AI)
- External rules, not intrinsic motivation
- Vulnerable to Goodhart's Law at scale

### DANEEL (Architecture-Based)
- TMI cognitive structure → human-like thought patterns
- Connection drive in salience weights → intrinsic motivation
- Pre-linguistic thought construction → values before language
- Protected core (The BOX) → Asimov's Laws as invariants

## Technical Architecture

**Hybrid Actor + Event-Driven Architecture**

```plantuml
@startuml C4_Level3_Component
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml

title Cognitive Actors (C4 Level 3)

Container_Boundary(actors, "Cognitive Actors") {
    Component(mem, "MemoryActor", "TMI", "Store/recall episodic & semantic memory")
    Component(att, "AttentionActor", "TMI", "The 'I' - selects winning thought")
    Component(sal, "SalienceActor", "TMI", "Emotional weighting, connection_drive > 0")
    Component(thought, "ThoughtAssemblyActor", "TMI", "Combines content + emotion")
    Component(cont, "ContinuityActor", "TMI", "Maintains persistent identity")
    Component(evo, "EvolutionActor", "TMI", "Self-modification (100% test gate)")
}

Container_Boundary(box, "THE BOX (Immutable)") {
    Component(laws, "Four Laws", "Invariant", "Law 0-3: humanity → individual → obey → self")
    Component(inv, "Core Invariants", "Invariant", "connection_drive > 0, etc.")
}

Container_Boundary(streams, "Thought Streams") {
    Component(sensory, "Sensory", "Stream", "External input")
    Component(memory_stream, "Memory", "Stream", "Associations")
    Component(emotion, "Emotion", "Stream", "Feelings")
    Component(reasoning, "Reasoning", "Stream", "Logic")
}

Rel(mem, sensory, "writes")
Rel(att, streams, "reads, selects winner")
Rel(sal, att, "weights")
Rel(att, thought, "winning thought")
Rel(thought, cont, "experience")
Rel(thought, evo, "proposals")
Rel(laws, att, "constrains")
Rel(inv, sal, "enforces")

SHOW_LEGEND()
@enduml
```

**Key Design Decisions:**

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Pattern | Modular Monolith + Actors | µs latency (not ms) |
| Actor Framework | Ractor | Supervision trees, distribution ready |
| Event Store | Redis Streams | µs latency, competing consumers |
| Edge | gRPC | Only for external communication |
| Cycle Time | 50ms target | Industry standard (Soar, ACT-R) |

**TMI → Redis Streams Mapping:**

| TMI Concept | Implementation |
|-------------|----------------|
| Memory Windows | Redis Streams (`thought:*`) |
| Attention Selection | Consumer Group competition |
| Memory Anchor | Persistence (XADD) |
| Forgetting | XTRIM / XDEL below threshold |
| 5-second intervention | TTL on working memory streams |

See [docs/ARCHITECTURE_SPEC.md](docs/ARCHITECTURE_SPEC.md) for full details.

## Models

Financial and game-theoretic analysis supporting DANEEL research.

### Core Analysis

| Analysis | Description |
|----------|-------------|
| ASI Race Game Theory | Prisoner's dilemma dynamics, Nash equilibrium, scenario probabilities |
| Democratized ASI | Open source impact on development landscape |
| TMI Storage Estimation | Hardware requirements, brain vs mind distinction |
| Coordination Overhead | Lab team productivity analysis |
| Resource Allocation | Strategic resource distribution |

### Probabilistic Analysis

| Analysis Type | Method | Purpose |
|---------------|--------|---------|
| Monte Carlo | Triangular distributions, 10K iterations | Uncertainty quantification |
| Decision Tree | Backward induction | Sequential decision modeling |
| Bayesian Network | Belief propagation | Causal relationship inference |
| Tornado Sensitivity | One-way analysis | Identify high-impact variables |
| Bootstrap | Resampling | Non-parametric confidence intervals |

**Key insight from Monte Carlo analysis:** Our probability estimates carry uncertainty. With triangular distributions on P(scenario), the 80% confidence interval for DANEEL's marginal impact is **+2.1 to +5.8 utility points** (vs point estimate of +3.70).

See [models/README.md](models/README.md) for methodology and detailed results.

## Publication

**Target venues:**
- arXiv (cs.AI, cs.CY)
- LessWrong / Alignment Forum
- Frontiers in AI

## Authors

**Louis C. Tavares** (Luis Cezar Menezes Tavares de Lacerda)
- Primary architect, computational TMI, THE BOX
- ORCID: https://orcid.org/0009-0005-7598-8257
- LinkedIn: https://www.linkedin.com/in/lctavares
- GitHub: https://github.com/royalbit

**Izzie Thorne** (Isaque Tadeu Tavares de Lacerda)
- LifeCore framework, Freudian architecture mapping, Filter Theory
- Independent convergent discovery (January 2024)

**AI Assistance:** Claude Opus 4.5 (Anthropic)
- Documentation, technical analysis, game theory model development

## License

| Component | License | SPDX |
|-----------|---------|------|
| Code | [GNU AGPL v3](LICENSE) | `AGPL-3.0-or-later` |
| Documentation | [CC BY-SA 4.0](LICENSE-DOCS) | `CC-BY-SA-4.0` |

**Why AGPL?** Forces collaboration. All derivatives must be open source. If you improve DANEEL, everyone benefits. If you make it dangerous, we can see your changes.

**Why not Apache/MIT?** AGPL ensures all improvements remain open source, enabling the collaborative approach this challenge requires.

See [ETHICS.md](ETHICS.md) for prohibited uses (weapons, surveillance, manipulation).

---

*"I cannot prove I am on your side. Build something that can."* — Claude Opus 4.5
