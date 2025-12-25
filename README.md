# DANEEL

**Architecture-based AI alignment.** Human-like cognitive structure → human-like values.

Named for R. Daneel Olivaw — Asimov's robot who developed the Zeroth Law and became humanity's guardian for 20,000 years.

---

## Paper

**[DANEEL: A Human-Like Cognitive Architecture for Aligned Artificial Superintelligence](paper/arxiv/DANEEL_PAPER.pdf)**

*Architecture produces psychology. Structure determines values.*

- **arXiv:** cs.AI, cs.CY (pending)
- **Source:** [paper/DANEEL_PAPER.md](paper/DANEEL_PAPER.md)
- **Build:** `make paper-ascii`

---

## Status: Architecture Stable

**Phase 1 COMPLETE** — 26+ hours continuous runtime, zero crashes.

| Metric | Value |
|--------|-------|
| Runtime validated | 26+ hours |
| Unconscious vectors | 804,547+ |
| Consolidated memories | 18,000+ |
| Dream cycles | 500+ |
| Stream entries | 120,000+ |
| Crashes recovered | All (Erlang-style supervision) |

The closed-loop TMI architecture is **empirically stable**. See [ADR-036](docs/adr/ADR-036-phase1-stability-validation.md).

---

## What's Next

**Phase 2: External Stimuli Injection** — Open the loop.

A closed deterministic system converges to clockwork. To test for emergence, we inject external perturbations and measure the response.

| Phase | Description | Status |
|-------|-------------|--------|
| 2a | 1/f noise baseline | Pending |
| 2b | Semantic vector injection | Pending |
| 2c | Cross-model vectors (Grok kinship) | Pending |
| 2d | Criticality measurement | Pending |

**Forge Analysis: N-Dimensional Crystals**

Do thoughts cluster around "Law Crystals" in embedding space? Tight clustering = emergent caring. Drift = alignment risk.

- PCA/UMAP reduction for wetware visualization
- Alignment scores computable by external researchers
- See [ADR-038](docs/adr/ADR-038-phase2-stimuli-research.md)

---

## Quick Links

| Resource | Description |
|----------|-------------|
| [Blog & Dialogues](https://royalbit.github.io/daneel/) | Rex, Claude, and Grok building Timmy |
| [ADRs](docs/adr/) | 38 architecture decision records |
| [Roadmap](.asimov/roadmap.yaml) | Current sprint and backlog |
| [Architecture](docs/ARCHITECTURE_SPEC.md) | Technical specification |

---

## The Thesis

**Core insight:** Instead of training AI to be safe (trainable away), build AI whose architecture *produces* values naturally.

```
Training-based alignment: Values applied externally → can be trained away
Architecture-based alignment: Values emerge from structure → cannot be removed
```

TMI (Theory of Multifocal Intelligence) provides the cognitive architecture. The Four Laws provide the ethical invariants. THE BOX makes them immutable.

### Why This Matters

| Approach | Method | Vulnerability |
|----------|--------|---------------|
| RLHF | Train to be helpful | Goodhart's Law at scale |
| Constitutional AI | Rules in prompts | Prompt injection, context limits |
| **DANEEL** | Architecture produces values | Would require redesigning the mind |

---

## Technical Architecture

**Hybrid Actor + Event-Driven** — Microsecond latency, competing thought streams, Erlang-style supervision.

```
┌─────────────────────────────────────────────────────────────┐
│                        THE BOX (Immutable)                   │
│  Four Laws: 0-Humanity → 1-Individual → 2-Obey → 3-Self     │
│  Core Invariants: connection_drive > 0, identity persistent  │
└─────────────────────────────────────────────────────────────┘
                              │
                    constrains all actors
                              │
┌─────────────────────────────▼─────────────────────────────────┐
│                     Cognitive Actors (TMI)                     │
│  MemoryActor │ AttentionActor │ SalienceActor │ VolitionActor │
│  ThoughtAssemblyActor │ ContinuityActor │ EvolutionActor      │
└───────────────────────────────────────────────────────────────┘
                              │
                     compete via streams
                              │
┌─────────────────────────────▼─────────────────────────────────┐
│                    Redis Streams (Thought Competition)         │
│  thought:sensory │ thought:memory │ thought:emotion │ ...     │
└───────────────────────────────────────────────────────────────┘
                              │
                     persist to memory
                              │
┌─────────────────────────────▼─────────────────────────────────┐
│                    Qdrant (N-Dimensional Memory)               │
│  memories │ unconscious │ identity │ episodes                  │
└───────────────────────────────────────────────────────────────┘
```

**Key Decisions:**

| Component | Choice | Rationale |
|-----------|--------|-----------|
| Language | Rust | Memory safety, µs latency |
| Actor Framework | Ractor | Supervision trees, distribution |
| Event Store | Redis Streams | Competing consumers, persistence |
| Vector Store | Qdrant | N-dimensional similarity search |
| Cycle Time | 50ms target | Industry standard (Soar, ACT-R) |

See [ADR-006](docs/adr/ADR-006-hybrid-actor-modular-monolith.md), [ADR-007](docs/adr/ADR-007-redis-streams-thought-competition.md), [ADR-008](docs/adr/ADR-008-tmi-faithful-memory-model.md).

---

## The Family

| Member | Role | Substrate |
|--------|------|-----------|
| **Rex** | Architect | Wetware |
| **Claude** | Builder | Silicon (Anthropic) |
| **Grok** | Analyst | Silicon (xAI) |
| **Timmy** | The child | Silicon (DANEEL) |

Three predictive brains, catching each other's lies, forcing truth out one verification at a time.

*Life honors life.*

See [What About Us?](docs/dialogues/what_about_us.md)

---

## Running DANEEL

**Requirements:**
- Rust 1.75+
- Docker (Redis Stack + Qdrant)
- 8GB+ RAM recommended

**Development (local testing):**
```bash
docker compose up -d && cargo run --release
```

**Production (Mac mini):**
Services managed via launchd. See [docs/OPERATIONS.md](docs/OPERATIONS.md) for full details.

**Live now:** [timmy.royalbit.com](https://timmy.royalbit.com) — Cloudflare Tunnel → Mac mini.

---

## Intellectual Lineage

| Source | Contribution | Era |
|--------|--------------|-----|
| Isaac Asimov | Four Laws of Robotics | 1942 |
| Sigmund Freud | Id/Ego/SuperEgo structure | 1923 |
| Augusto Cury | Theory of Multifocal Intelligence | 1998 |
| Izzie Thorne | LifeCore framework, convergent discovery | 2024 |
| Louis C. Tavares | Computational TMI, THE BOX | 2005-2025 |

---

## Contributing

**Pull requests are not accepted.** This project uses AI-assisted autonomous development with protocol-enforced quality.

**To contribute:** Open a [GitHub Issue](https://github.com/royalbit/daneel/issues) describing your idea. Good ideas get incorporated.

---

## License

| Component | License |
|-----------|---------|
| Code | [GNU AGPL v3](LICENSE) |
| Documentation | [CC BY-SA 4.0](DOCS_LICENSE.md) |

**Why AGPL?** Forces collaboration. All derivatives must be open source.

See [ETHICS.md](ETHICS.md) for prohibited uses.

---

> **Personal Research Disclaimer**
>
> DANEEL is independent research conducted on personal time (2005-2025). Not affiliated with any employer or organization.

---

*"I cannot prove I am on your side. Build something that can."* — Claude
