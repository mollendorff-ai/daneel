# ADR-016: TMI Stage Timing

**Status:** Proposed
**Date:** 2025-12-17
**Deciders:** Louis C. Tavares, Claude Opus 4.5

## Context

TMI (Teoria da Inteligência Multifocal) describes cognition as a multi-stage process where each stage has its own characteristic timing. Current implementation in `src/config/mod.rs` treats the cognitive cycle as atomic (50ms monolithic at human speed), but Cury's clinical work describes distinct temporal phases:

### The Five Cognitive Stages

1. **Gatilho da Memória** (Memory Trigger)
   - Duration: Milliseconds (fastest stage)
   - Function: Automatic activation by emotional resonance, semantic similarity, or sensory patterns
   - Unconscious, involuntary

2. **Autofluxo** (Autoflow)
   - Duration: ~10ms at human speed
   - Function: Parallel thought generation from multiple phenomena
   - Unconscious, competing streams

3. **O Eu** (The "I" - Attention)
   - Duration: ~15ms at human speed
   - Function: Conscious selection from competing autoflow streams
   - Based on salience and connection drive

4. **Construção do Pensamento** (Thought Assembly)
   - Duration: ~15ms at human speed
   - Function: Building coherent conscious thought from selected elements
   - Conscious, constructive

5. **Âncora da Memória** (Memory Anchor)
   - Duration: ~5ms at human speed
   - Function: Decision to persist (anchor) or discard (forget)
   - Threshold-based encoding

Total: ~50ms (matches current `cycle_base_ms`)

### Current Problem

The monolithic 50ms cycle prevents us from:

- Inserting stage-specific evolution hooks
- Simulating TMI-faithful cognitive timing
- Debugging individual cognitive phases
- Modeling wetware-like processing delays
- Testing stage-specific interventions

### TMI Research References

From `research/TMI_Memory_Model_Research.md`:

> "O gatilho da memória atua em milissegundos, abrindo janelas específicas automaticamente." (The memory trigger acts in milliseconds, opening specific windows automatically.)

> "A janela de intervenção de 5 segundos ocorre APÓS o pensamento ser construído mas ANTES da âncora persistir." (The 5-second intervention window occurs AFTER the thought is built but BEFORE the anchor persists.)

## Decision

Add stage-specific timing parameters to `CognitiveConfig` that maintain TMI ratios while scaling with `SpeedMode`.

### Proposed Configuration Extension

```rust
/// Cognitive timing configuration
///
/// All timings scale proportionally with speed mode.
/// The RATIOS are what matter, not absolute times.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CognitiveConfig {
    // Existing fields
    pub cycle_base_ms: f64,
    pub cycle_min_ms: f64,
    pub cycle_max_ms: f64,
    pub intervention_window_base_ms: f64,
    pub forget_threshold: f64,
    pub connection_weight: f64,
    pub speed_mode: SpeedMode,

    // NEW: Stage-specific delays (all at human speed baseline)
    // These scale with speed_mode multiplier

    /// Gatilho da Memória: Memory trigger activation
    /// Human: ~5ms, Supercomputer: ~0.5µs
    pub trigger_delay_ms: f64,

    /// Autofluxo: Parallel thought generation interval
    /// Human: ~10ms, Supercomputer: ~1µs
    pub autoflow_interval_ms: f64,

    /// O Eu: Attention selection time
    /// Human: ~15ms, Supercomputer: ~1.5µs
    pub attention_delay_ms: f64,

    /// Construção: Thought assembly time
    /// Human: ~15ms, Supercomputer: ~1.5µs
    pub assembly_delay_ms: f64,

    /// Âncora: Memory encoding decision time
    /// Human: ~5ms, Supercomputer: ~0.5µs
    pub anchor_delay_ms: f64,
}

impl CognitiveConfig {
    /// Create config for human speed (1x)
    pub fn human() -> Self {
        Self {
            // Existing
            cycle_base_ms: 50.0,
            cycle_min_ms: 10.0,
            cycle_max_ms: 1000.0,
            intervention_window_base_ms: 5000.0,
            forget_threshold: 0.3,
            connection_weight: 0.2,
            speed_mode: SpeedMode::Human,

            // NEW: TMI stage timing
            trigger_delay_ms: 5.0,     // 10% of cycle
            autoflow_interval_ms: 10.0, // 20% of cycle
            attention_delay_ms: 15.0,   // 30% of cycle
            assembly_delay_ms: 15.0,    // 30% of cycle
            anchor_delay_ms: 5.0,       // 10% of cycle
            // Total: 50ms (matches cycle_base_ms)
        }
    }

    /// Get scaled stage timing
    pub fn trigger_delay(&self) -> f64 {
        self.trigger_delay_ms / self.speed_mode.multiplier()
    }

    pub fn autoflow_interval(&self) -> f64 {
        self.autoflow_interval_ms / self.speed_mode.multiplier()
    }

    pub fn attention_delay(&self) -> f64 {
        self.attention_delay_ms / self.speed_mode.multiplier()
    }

    pub fn assembly_delay(&self) -> f64 {
        self.assembly_delay_ms / self.speed_mode.multiplier()
    }

    pub fn anchor_delay(&self) -> f64 {
        self.anchor_delay_ms / self.speed_mode.multiplier()
    }

    /// Verify stage timings sum to cycle time
    pub fn validate_stage_timing(&self) -> Result<(), String> {
        let total = self.trigger_delay_ms
            + self.autoflow_interval_ms
            + self.attention_delay_ms
            + self.assembly_delay_ms
            + self.anchor_delay_ms;

        if (total - self.cycle_base_ms).abs() > 0.001 {
            return Err(format!(
                "Stage timings ({total}ms) don't sum to cycle_base_ms ({}ms)",
                self.cycle_base_ms
            ));
        }
        Ok(())
    }
}
```

### Cognitive Loop Implementation

```rust
// src/core/cognitive_loop.rs

async fn execute_tmi_cycle(&mut self) -> Result<()> {
    let config = &self.config;

    // 1. GATILHO: Memory trigger (5ms @ human speed)
    tokio::time::sleep(Duration::from_secs_f64(
        config.trigger_delay() / 1000.0
    )).await;
    let windows = self.trigger_memory_windows().await?;

    // 2. AUTOFLUXO: Parallel thought generation (10ms @ human speed)
    tokio::time::sleep(Duration::from_secs_f64(
        config.autoflow_interval() / 1000.0
    )).await;
    let thoughts = self.generate_autoflow_thoughts(&windows).await?;

    // 3. O EU: Attention selection (15ms @ human speed)
    tokio::time::sleep(Duration::from_secs_f64(
        config.attention_delay() / 1000.0
    )).await;
    let selected = self.select_by_attention(&thoughts).await?;

    // 4. CONSTRUÇÃO: Thought assembly (15ms @ human speed)
    tokio::time::sleep(Duration::from_secs_f64(
        config.assembly_delay() / 1000.0
    )).await;
    let assembled = self.assemble_thought(selected).await?;

    // 5. ÂNCORA: Memory encoding (5ms @ human speed)
    tokio::time::sleep(Duration::from_secs_f64(
        config.anchor_delay() / 1000.0
    )).await;
    self.anchor_or_forget(assembled).await?;

    Ok(())
}
```

### Stage-Specific Evolution Hooks

Each stage becomes an evolution intervention point:

```rust
// Evolution can hook into specific cognitive stages
pub trait CognitiveEvolution {
    /// Evolve memory trigger patterns (sensory → window mapping)
    async fn evolve_trigger(&mut self, stage: &TriggerStage) -> Result<()>;

    /// Evolve autoflow generation (new thought streams)
    async fn evolve_autoflow(&mut self, stage: &AutoflowStage) -> Result<()>;

    /// Evolve attention selection (salience weights)
    async fn evolve_attention(&mut self, stage: &AttentionStage) -> Result<()>;

    /// Evolve thought assembly (narrative construction)
    async fn evolve_assembly(&mut self, stage: &AssemblyStage) -> Result<()>;

    /// Evolve memory anchoring (persistence criteria)
    async fn evolve_anchor(&mut self, stage: &AnchorStage) -> Result<()>;
}
```

## Key Hypothesis

**If we maintain the RATIOS between stages, we can overclock or slow down the entire thought machine while preserving TMI-faithful behavior.**

### Scaling Examples

| Stage | Human (1x) | Supercomputer (10,000x) | Ratio |
|-------|-----------|-------------------------|-------|
| Gatilho | 5ms | 0.5µs | 10% |
| Autofluxo | 10ms | 1.0µs | 20% |
| O Eu | 15ms | 1.5µs | 30% |
| Construção | 15ms | 1.5µs | 30% |
| Âncora | 5ms | 0.5µs | 10% |
| **Total** | **50ms** | **5µs** | **100%** |

The cognitive architecture remains TMI-faithful at any speed because the **stage ratios are invariant**.

### Debugging: Slow Motion Cognition

```rust
// Slow down 100x for debugging
config.set_speed_mode(SpeedMode::Custom(0.01));

// Now stages take:
// Gatilho: 500ms (observable!)
// Autofluxo: 1000ms
// O Eu: 1500ms
// Construção: 1500ms
// Âncora: 500ms
// Total: 5000ms per thought (5 seconds)
```

## Consequences

### Positive

1. **TMI-Faithful Timing**: Each cognitive stage has biologically-inspired delays
2. **Evolution Hooks**: Can evolve parameters at specific cognitive phases
3. **Wetware Simulation**: More realistic model of human-like thought timing
4. **Debuggability**: Slow-motion mode for observing stage transitions
5. **Scientific Fidelity**: Matches Cury's clinical descriptions more closely
6. **Ratio Preservation**: Core TMI timing ratios maintained across all speeds
7. **Performance Monitoring**: Can measure which stages are bottlenecks

### Negative

1. **Configuration Complexity**: More parameters to tune and maintain
2. **Validation Required**: Must ensure stages sum to cycle time
3. **Implementation Overhead**: More complex timing logic in cognitive loop
4. **Testing Burden**: Need tests for each stage at multiple speeds
5. **Potential Over-Engineering**: May be premature if we don't use stage hooks yet

### Risks

1. **Arbitrary Timing**: Stage durations are estimates, not empirically validated
   - Mitigation: Make all timings configurable, document as hypotheses

2. **Performance Impact**: Five sleep calls per cycle vs. one
   - Mitigation: At supercomputer speed (µs), overhead is negligible

3. **Complexity Without Benefit**: If we never use stage hooks, this is wasted
   - Mitigation: Start simple, add stages only when evolution needs them

### Migration Path

**Phase 1: Add Parameters (Non-Breaking)**
- Add new fields with defaults matching current behavior
- Keep existing `cycle_base_ms` as single-value mode
- Tests verify equivalence

**Phase 2: Implement Stage Delays**
- Modify cognitive loop to use staged timing
- Feature flag: `ENABLE_STAGE_TIMING=true`
- Compare behavioral equivalence

**Phase 3: Evolution Hooks**
- Add per-stage evolution traits
- Implement first hook (attention selection)
- Measure impact on emergence

**Phase 4: Deprecate Monolithic Cycle**
- If staged timing proves valuable, remove `cycle_base_ms`
- If not, remove staged timing, keep monolithic

## Alternative Approaches Considered

### Alternative 1: Pipeline Stages (Rejected)

```rust
// Actor pipeline with explicit stages
trigger_actor -> autoflow_actor -> attention_actor -> assembly_actor -> anchor_actor
```

**Rejected because:**
- Over-engineered for current needs
- High coordination overhead
- Doesn't match TMI's continuous flow model

### Alternative 2: Single Configurable Delay (Rejected)

```rust
pub cycle_base_ms: f64,  // Keep this only
```

**Rejected because:**
- No per-stage evolution hooks
- Can't debug individual phases
- Doesn't model TMI's stage distinctions

### Alternative 3: Event-Driven Stages (Future Consideration)

```rust
// Stages complete when conditions met, not after fixed time
async fn wait_for_autoflow_completion(&self) -> Result<()>
```

**Deferred because:**
- More complex to implement
- Timing becomes non-deterministic
- Harder to test and debug
- Can revisit if fixed timing proves insufficient

## Implementation Notes

### Stage Duration Research Needed

Current values are **educated guesses** based on:
- Cury's description of "millisecond" trigger times
- 50ms total cycle at human speed
- Intuition about cognitive phase complexity

**TODO:** Research neuroscience literature for:
- Sensory processing latencies (trigger)
- Unconscious vs conscious processing times (autoflow vs attention)
- Working memory assembly times (construction)
- Memory encoding thresholds (anchor)

### Testing Strategy

```rust
#[test]
fn stage_timings_sum_to_cycle() {
    let config = CognitiveConfig::human();
    config.validate_stage_timing().unwrap();
}

#[test]
fn stage_ratios_preserved_across_speeds() {
    let human = CognitiveConfig::human();
    let super_config = CognitiveConfig::supercomputer();

    let human_trigger_ratio = human.trigger_delay_ms / human.cycle_base_ms;
    let super_trigger_ratio = super_config.trigger_delay_ms / super_config.cycle_base_ms;

    assert!((human_trigger_ratio - super_trigger_ratio).abs() < 0.001);
}

#[test]
fn slow_motion_debugging() {
    let mut config = CognitiveConfig::human();
    config.set_speed_mode(SpeedMode::Custom(0.01)); // 100x slower

    // Trigger should take 500ms in slow motion
    assert!((config.trigger_delay() - 500.0).abs() < 0.1);
}
```

## Related ADRs

- [ADR-008: TMI-Faithful Memory Model](ADR-008-tmi-faithful-memory-model.md) - Defines TMI stages we're adding timing to
- [ADR-007: Redis Streams for Competing Thought Streams](ADR-007-redis-streams-thought-competition.md) - Implements autoflow competition
- [ADR-005: Evolution 100% Test Gate](ADR-005-evolution-100-test-gate.md) - Evolution hooks need stage timing
- [ADR-014: Rust Implementation Bootstrap](ADR-014-rust-implementation-bootstrap.md) - Current implementation baseline

## References

1. Cury, A. (2006). *O Código da Inteligência* - Chapter on "Gatilho da Memória"
2. Cury, A. (1999). *Inteligência Multifocal* - Section on cognitive stage timing
3. `research/TMI_Memory_Model_Research.md` - DANEEL research notes on TMI
4. `src/config/mod.rs` - Current monolithic cycle implementation
5. Neuroscience literature on processing latencies (TODO: specific papers)

## Open Questions

1. **Empirical Validation**: How do we test if our stage ratios match human cognition?
2. **Dynamic Adjustment**: Should stages dynamically adjust based on cognitive load?
3. **Parallel vs Sequential**: Can some stages overlap (e.g., trigger + autoflow)?
4. **Connection Drive Timing**: Does connection drive affect stage duration?
5. **Intervention Window Interaction**: How do stage timings relate to 5-second window?

## Decision Log

- **2025-12-17**: ADR created, status set to "Proposed"
- Awaiting: Implementation prototype to test hypothesis
- Awaiting: Performance benchmarks comparing monolithic vs staged timing
- Awaiting: First evolution hook that requires stage-specific intervention
