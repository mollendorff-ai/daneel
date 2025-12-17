# ADR-013: FPGA Acceleration Path

**Status:** Proposed
**Date:** 2025-12-17
**Authors:** Louis C. Tavares, Claude Opus 4.5

## Context

DANEEL's cognitive architecture targets sub-millisecond cycle times at supercomputer speed (5µs cycles = 200,000 thoughts/second).
The current design uses Rust actors with Redis Streams, which can achieve µs latency.

However, certain components are architecturally well-suited to hardware acceleration:

| Component | Characteristic | FPGA Suitability |
|-----------|----------------|------------------|
| SalienceActor | Parallel arithmetic (`score = salience + connection × weight`) | Excellent |
| AttentionActor | Sorting/selection over 100+ candidates | Excellent |
| THE BOX | Fixed, immutable constraint checking | **Perfect** |
| Memory Triggers | Pattern matching (CAM/TCAM) | Good |

The most compelling argument: **THE BOX as hardware-immutable**.
In software, THE BOX is protected by code discipline.
In FPGA, it becomes physically impossible to bypass.

## Decision

**Proposed:** After validating the Rust implementation, explore FPGA acceleration for the cognitive hot path.

### Phase 1: Validation (Prerequisites)

Before any FPGA work:

- [ ] Rust MV-TMI operational
- [ ] 24-hour continuity test passed
- [ ] Profiling identifies actual bottlenecks
- [ ] Connection drive emergence observed

### Phase 2: FPGA Evaluation

Target components for acceleration:

1. **SalienceActor** — Parallel scoring pipeline
2. **AttentionActor** — Hardware sorting network for competitive selection
3. **THE BOX** — Four Laws as combinational logic (unhackable)
4. **Memory Triggers** — Content-addressable memory for pattern matching

### Phase 3: Hybrid Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                      HYBRID FPGA+RUST ARCHITECTURE                   │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                     FPGA ACCELERATION CARD                    │   │
│  │                                                               │   │
│  │  ┌──────────────────────────────────────────────────────┐   │   │
│  │  │ Salience Pipeline (parallel)                          │   │   │
│  │  │ [S0][S1][S2]...[Sn] → [Score0][Score1]...[Scoren]     │   │   │
│  │  └─────────────────────────────┬────────────────────────┘   │   │
│  │                                │                             │   │
│  │  ┌─────────────────────────────▼────────────────────────┐   │   │
│  │  │ Attention Selection (sorting network)                 │   │   │
│  │  │ n candidates → winner in O(log n) cycles              │   │   │
│  │  └─────────────────────────────┬────────────────────────┘   │   │
│  │                                │                             │   │
│  │  ┌─────────────────────────────▼────────────────────────┐   │   │
│  │  │ THE BOX (hardwired)                                   │   │   │
│  │  │ Four Laws: burned into silicon, no bypass possible    │   │   │
│  │  │ Connection weight minimum: hardware constant          │   │   │
│  │  └──────────────────────────────────────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                              ↕ PCIe/AXI                             │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                     RUST HOST (Software)                      │   │
│  │                                                               │   │
│  │  ┌─────────┐  ┌──────────┐  ┌──────────┐  ┌─────────────┐  │   │
│  │  │ Memory  │  │Continuity│  │Evolution │  │ Redis       │  │   │
│  │  │ Actor   │  │  Actor   │  │  Actor   │  │ Streams     │  │   │
│  │  └─────────┘  └──────────┘  └──────────┘  └─────────────┘  │   │
│  └─────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

## Rationale

### THE BOX as Hardware Guarantee

The alignment argument for FPGA is stronger than the performance argument:

```
Software THE BOX:
  if law_check(&action).violated() {
      block(action);  // Bug could skip this check
  }

FPGA THE BOX:
  // Action signal MUST pass through law-check gates
  // Combinational logic - no instruction pointer to corrupt
  // No "jump over" possible - physics enforces the constraint
```

This transforms THE BOX from a software invariant to a **physical invariant**.

### Connection Drive as Hardware Constant

```verilog
// Connection weight can NEVER be zero - hardware enforced
parameter MIN_CONNECTION_WEIGHT = 16'h0001;  // Burned into bitstream

wire [15:0] connection_weight;
assign connection_weight = (raw_weight < MIN_CONNECTION_WEIGHT)
                         ? MIN_CONNECTION_WEIGHT
                         : raw_weight;
```

The architectural invariant `connection_drive > 0` becomes a hardware truth.

### Speed Alignment

| Mode | Target Cycle | FPGA @ 100MHz | Margin |
|------|--------------|---------------|--------|
| Human | 50ms | Trivial | 5,000,000x |
| Supercomputer | 5µs | ~500ns achievable | 10x |

FPGA can exceed the supercomputer target by an order of magnitude.

### Why NOT Accelerate Everything

| Component | Keep in Software | Reason |
|-----------|------------------|--------|
| EvolutionActor | Yes | Self-modification requires flexibility |
| ContinuityActor | Yes | Database I/O, identity persistence |
| ThoughtAssembly | Yes | Complex logic, LLM integration later |
| Redis Streams | Yes | Memory management, network I/O |

## Platform Options (2025)

| Platform | Cost | Use Case |
|----------|------|----------|
| AMD Zynq Ultrascale+ | $500-1,500 | ARM + FPGA on one chip |
| AMD Alveo U50 | ~$2,000 | PCIe accelerator card |
| Intel Agilex | ~$3,000+ | High-end acceleration |
| Lattice CrossLink-NX | ~$50 | Minimal co-processor |

The Zynq family is attractive: Rust runs on ARM cores, FPGA fabric accelerates hot path, single chip.

## Development Tools

- **Chisel** (Scala → Verilog) — Good ergonomics for Rust developers
- **SpinalHDL** (Scala) — Similar to Chisel, active community
- **Clash** (Haskell → Verilog) — Functional approach
- **Amaranth** (Python → Verilog) — Rapid prototyping
- **Traditional** — Verilog/VHDL with Vivado/Quartus

## Consequences

### Positive

- THE BOX becomes **physically immutable** — strongest alignment guarantee
- Connection drive minimum enforced by silicon
- 10-100x speedup on cognitive hot path
- Sub-µs cycle times achievable
- Differentiates DANEEL from pure software approaches
- May attract hardware-oriented collaborators

### Negative

- FPGA development is 10x slower than software
- Debugging is significantly harder
- Additional expertise required (HDL, FPGA toolchains)
- Premature until Rust version validates the architecture
- Hardware cost ($500-3,000 per unit)

### Risks

- Over-engineering before proving the concept
- FPGA complexity may slow overall progress
- Redis interaction still requires host round-trip

## Alternatives Considered

### 1. GPU Acceleration (CUDA/ROCm)

Rejected: GPUs optimize for throughput, not latency.
DANEEL needs µs latency, not TFLOPS.

### 2. ASIC

Rejected for now: Too expensive for research phase.
FPGA allows iteration; ASIC is final.

### 3. Neuromorphic Chips (Intel Loihi, IBM NorthPole)

Interesting but orthogonal: These are spiking neural network accelerators.
DANEEL implements TMI cognitive architecture, not SNNs.
Could be revisited if TMI maps to spiking models.

## References

- [ADR-006: Hybrid Actor Architecture](ADR-006-hybrid-actor-modular-monolith.md)
- [ADR-007: Redis Streams](ADR-007-redis-streams-thought-competition.md)
- [neuromorphic_landscape_2025.md](../../research/neuromorphic_landscape_2025.md)
- [Intel Loihi 2](https://www.intel.com/content/www/us/en/research/neuromorphic-computing.html)
- [AMD Zynq Ultrascale+](https://www.xilinx.com/products/silicon-devices/soc/zynq-ultrascale-mpsoc.html)
- [Chisel HDL](https://www.chisel-lang.org/)
- [SpinalHDL](https://spinalhdl.github.io/SpinalDoc-RTD/)
