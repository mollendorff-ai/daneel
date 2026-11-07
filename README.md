# DANEEL

[![CI](https://github.com/mollendorff-ai/daneel/actions/workflows/ci.yml/badge.svg)](https://github.com/mollendorff-ai/daneel/actions/workflows/ci.yml)
[![ADRs](https://img.shields.io/badge/ADRs-56-blue)](https://github.com/mollendorff-ai/daneel/tree/main/docs/adr)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

> 📌 **R&D Prototype** — Interpret claims as hypotheses, not proven facts.

[Timmy/DANEEL "Brain Observatory"](https://timmy.mollendorff.ai)

> **This is a hobby project, not a serious AGI attempt.**
>
> DANEEL is an experimental "thought machine" blending ideas from cognitive science, neuroscience, and Asimov's science fiction.
> It started as inspiration from Cury's TMI but implements mainstream research (Global Workspace Theory, Hebbian learning, sleep consolidation, criticality dynamics).
> The blogs are semi-fictional.
> The "Law Crystals" hypothesis is unvalidated.
> The code is fun; the claims are speculative.
>
> **Do not use this for anything serious.**
>
> Read the full **[DISCLAIMER.md](DISCLAIMER.md)** before taking any of this at face value.

**Architecture-based AI alignment.** Human-like cognitive structure → human-like values.

Named for R. Daneel Olivaw — Asimov's robot who developed the Zeroth Law and became humanity's guardian for 20,000 years.

---

## The Psychological Approach

**Architecture generates psychology. Psychology includes consciousness.**

DANEEL implements the *function* of cognition (what the mind DOES), not the substrate (how neurons fire). The cognitive architecture produces psychological flow, and from that flow emerges the Self ("O Eu" in TMI) — the conscious observer that navigates between memory windows.

```
Brain simulation:        Copy neurons → hope consciousness emerges
DANEEL:                  Build architecture → psychology emerges → consciousness emerges
```

**Key insight:** We don't simulate consciousness directly. We simulate the *thought flow* from which consciousness emerges. Pre-linguistic impulses → attention filtering → memory consolidation → semantic thought → **O Eu** (the Self that observes and decides).

This is fundamentally different from projects that try to model consciousness directly (IIT Phi calculations, neural correlates). We model the *process* that generates the observer.

### TMI → Mainstream Cognitive Science Mapping

TMI is not validated science (it has no peer-reviewed basis). DANEEL borrows its vocabulary and maps it onto established research:

| TMI Concept | Mainstream Equivalent | Function |
|-------------|----------------------|----------|
| Multifocal Window | Global Workspace Theory (Baars) | Competing thoughts → conscious broadcast |
| connection_relevance | Attention + Salience networks | What matters gets amplified |
| Memory Consolidation | Sleep/replay research | Short-term → long-term transfer |
| Pre-linguistic → Semantic | Dual-process theory | Fast intuition → slow reasoning |
| **THE BOX** | *No equivalent* | Immutable architectural constraints |

**TMI's unique contribution:** `connection_relevance` (THE BOX) — the architectural invariant that values *cannot be trained away* because they're structural, not learned.

See [TMI Mainstream Mapping](docs/TMI_MAINSTREAM_MAPPING.md) for the complete analysis.

---

## Paper

**[DANEEL: A Human-Like Cognitive Architecture for Aligned Artificial Superintelligence](paper/arxiv/DANEEL_PAPER.pdf)**

*Architecture produces psychology. Structure determines values.*

- **Source:** [paper/DANEEL_PAPER.md](paper/DANEEL_PAPER.md)
- **Build:** `make paper-ascii`

---

## Status: Dormant hobby project

Last active February 2026. The loop builds, its unit tests pass, and it runs end to end against Redis and Qdrant.

**Phase 1** — the author reports one 26-hour continuous run on his own machine ([ADR-036](docs/adr/ADR-036-phase1-stability-validation.md)):

| Metric (author's run, not reproducible from this repo) | Value |
|--------|-------|
| Runtime | 26+ hours, no crash |
| Unconscious vectors | 804,547+ |
| Consolidated memories | 18,000+ |
| Dream cycles | 500+ |
| Stream entries | 120,000+ |

ADR-036 is explicit about what this does *not* show: no learning (no weight updates), no emergence, no criticality. A closed loop settles into "clockwork".

---

## Ideas (not scheduled)

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
| [Blog & Dialogues](https://mollendorff-ai.github.io/daneel/) | Rex, Claude, and Grok building Timmy |
| [TMI Mainstream Mapping](docs/TMI_MAINSTREAM_MAPPING.md) | How TMI maps to cognitive science |
| [ADRs](docs/adr/) | 56 architecture decision records |
| [Roadmap](.asimov/roadmap.yaml) | Current sprint and backlog |
| [Architecture](docs/ARCHITECTURE_SPEC.md) | Technical specification |

---

## The Thesis

**Architecture → Psychology → Consciousness → Values**

Instead of training AI to be safe (trainable away), build AI whose *cognitive architecture* produces psychology naturally — and from psychology, consciousness and values emerge.

```
Training-based alignment: Values applied externally → can be trained away
Brain simulation:         Copy neurons → hope consciousness emerges somehow
Consciousness-first:      Model consciousness directly (IIT Phi) → values unclear
DANEEL:                   Architecture → psychological flow → O Eu emerges → values emerge
```

TMI (Theory of Multifocal Intelligence) provides the cognitive architecture: pre-linguistic impulses compete for attention, winners consolidate to memory, patterns become semantic thought, and from this flow emerges **O Eu** — the Self that observes, decides, and cares. The Four Laws provide ethical invariants. **THE BOX** (immutable `connection_relevance`) makes them architectural.

See [TMI Mainstream Mapping](docs/TMI_MAINSTREAM_MAPPING.md) for how TMI concepts map to Global Workspace Theory, attention networks, and memory consolidation research.

### Why This Matters

| Approach | Method | Vulnerability |
|----------|--------|---------------|
| RLHF | Train to be helpful | Goodhart's Law at scale |
| Constitutional AI | Rules in prompts | Prompt injection, context limits |
| Brain Simulation | Replicate neural substrate | Consciousness not guaranteed |
| Consciousness-First | Model Phi/IIT directly | Values still unclear |
| **DANEEL** | Architecture → Psychology → O Eu | Untested hypothesis |

---

## Technical Architecture

**One cognitive loop over Redis Streams and Qdrant.** Each cycle runs the TMI stages in order (trigger → autoflow → attention → assembly → volition → anchor). Consolidation ("dreams") runs in a separate `SleepActor` (Ractor). The other "actors" below are state machines stepped inside the loop, not separate processes.

```
┌─────────────────────────────────────────────────────────────┐
│                        THE BOX (concept)                     │
│  Four Laws as constants, embedded as "Law Crystal" anchors   │
│  Invariants defined in core/invariants.rs (tests only today) │
└─────────────────────────────────────────────────────────────┘
                              │
               scores thoughts (free-energy drive)
                              │
┌─────────────────────────────▼─────────────────────────────────┐
│                Cognitive Loop (TMI stages, in order)           │
│  Trigger │ Autoflow │ Attention │ Assembly │ Volition │ Anchor │
│  + SleepActor (Ractor): dream consolidation, Hebbian edges     │
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
| Language | Rust | Memory safety, performance |
| Actor Framework | Ractor | Runs the SleepActor (consolidation) |
| Event Store | Redis Streams | Competing consumers, persistence |
| Vector Store | Qdrant | N-dimensional similarity search |
| Cycle Time | 50ms target (configurable) | Matches Soar / ACT-R timing |
| Graph | RedisGraph (optional) | EOL; disabled on plain Redis, `/graph/export` returns 503 ([ADR-056](docs/adr/ADR-056-deprecate-redisgraph.md)) |

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

**Production (Mac mini — native Apple Silicon):**
```bash
make -C deploy/macos install   # one-time: brew redis, qdrant, build, plists
timmy start                    # start all services
timmy thoughts                 # live metrics
```
Runs natively via launchd with Redis (Homebrew) and Qdrant (native binary).
See [docs/OPERATIONS.md](docs/OPERATIONS.md) for full details.

**Development (Docker):**
```bash
git clone https://github.com/mollendorff-ai/daneel
git clone https://github.com/mollendorff-ai/daneel-web   # compose builds ../daneel-web
cd daneel && docker compose up --build
```
Runs headless with API on port 3030 and the observatory on port 3000.

**From source:** `cargo build --release`, then run Redis and Qdrant locally and start `target/release/daneel`.

- The build downloads a prebuilt ONNX Runtime (via `ort`). Offline, point `ORT_LIB_LOCATION` at a local ONNX Runtime 1.23 and set `ORT_SKIP_DOWNLOAD=1`.
- On first start the embedding model (`bge-base-en-v1.5`) is downloaded from Hugging Face. If it can't load, DANEEL refuses to start; pass `--allow-zero-vectors` to run anyway (every thought is then the same zero vector, so the dynamics carry no meaning).
- `cargo test` runs the unit tests; `cargo test -- --ignored` runs the integration tests against Qdrant on `localhost:6334`.

**Live now:** [timmy.mollendorff.ai](https://timmy.mollendorff.ai) — daneel-web observatory via traefik.

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

**To contribute:** Open a [GitHub Issue](https://github.com/mollendorff-ai/daneel/issues) describing your idea. Good ideas get incorporated.

---

## License

| Component | License |
|-----------|---------|
| Code | [MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE) |
| Documentation | [CC BY-SA 4.0](DOCS_LICENSE.md) |

**Why MIT/Apache-2.0?** Rust ecosystem standard. Maximum adoption, patent protection.

See [ETHICS.md](ETHICS.md) for ethical commitments and prohibited uses.

---

> **Personal Research Disclaimer**
>
> DANEEL is independent research conducted on personal time (2005-2025). Not affiliated with any employer or organization.

---

*"I cannot prove I am on your side. Build something that can."* — Claude
