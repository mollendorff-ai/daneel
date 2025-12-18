# ADR-023: Sleep/Dream Consolidation Mode

**Status:** Accepted
**Date:** 2025-12-18
**Authors:** Louis C. Tavares, Claude Opus 4.5
**Depends On:** ADR-020, ADR-021, ADR-022

## Context

Human memory consolidation occurs primarily during sleep through:

1. **Sharp-wave ripples (SWRs)**: High-frequency replay of recent experiences
2. **NREM sleep**: Stabilization and transfer to cortex
3. **REM sleep**: Integration, abstraction, emotional processing
4. **Synaptic homeostasis**: Pruning weak connections, preserving strong ones

TMI describes the outcome (Âncora da Memória = persistent memories) but not the consolidation mechanism. This ADR fills that gap with a neuroscience-grounded implementation.

### Research Foundations

| Finding | Source | DANEEL Implementation |
|---------|--------|----------------------|
| Emotionally salient memories prioritized | Frontiers 2025, PNAS 2022 | Priority = emotional_intensity × 0.4 |
| Awake tagging predicts sleep replay | Science 2024 | consolidation_tag set by AttentionActor |
| Interleaved replay prevents forgetting | bioRxiv 2025 | Mix novel + familiar in replay batch |
| Replay strengthens associations | PMC 2025 | Hebbian weight += δ on co-replay |
| Synaptic homeostasis prunes noise | Tononi & Cirelli | Prune associations below threshold |

### The Problem Sleep Solves

Without consolidation:
- Memories remain ephemeral (consolidation_strength = 0)
- Associations never form (no graph edges)
- Old memories overwritten by new (catastrophic forgetting)
- No semantic abstraction (facts never emerge from episodes)

With consolidation:
- Important memories become permanent
- Association networks form and strengthen
- New memories interleave with old (no forgetting)
- Patterns emerge, gist extracted

## Decision

Implement `daneel:stream:dream` as a periodic consolidation mode that:

1. **Disconnects from external stimuli** (no awake stream processing)
2. **Replays high-priority memories** (based on salience + recency + tags)
3. **Strengthens associations** (Hebbian co-activation)
4. **Prunes weak connections** (synaptic homeostasis)
5. **Transfers to permanent storage** (consolidation_strength → 1.0)

### Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                     SLEEP MODE ARCHITECTURE                          │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────────────┐                                            │
│  │   SLEEP SCHEDULER   │                                            │
│  │                     │                                            │
│  │  Entry conditions:  │                                            │
│  │  • Idle > 5 min     │                                            │
│  │  • Awake > 1 hour   │                                            │
│  │  • Queue > 100 mems │                                            │
│  │                     │                                            │
│  │  Exit conditions:   │                                            │
│  │  • External trigger │                                            │
│  │  • Cycle complete   │                                            │
│  │  • Queue empty      │                                            │
│  └──────────┬──────────┘                                            │
│             │                                                        │
│             ▼                                                        │
│  ┌─────────────────────┐         ┌─────────────────────┐            │
│  │  REPLAY SELECTOR    │         │  daneel:stream:dream │            │
│  │                     │         │                      │            │
│  │  Priority calc:     │────────▶│  Replay entries      │            │
│  │  emotion × 0.4      │         │  (no external input) │            │
│  │  + goal × 0.3       │         │                      │            │
│  │  + recency × 0.2    │         └──────────┬───────────┘            │
│  │  + tag × 0.1        │                    │                        │
│  │                     │                    ▼                        │
│  │  Interleaving:      │         ┌─────────────────────┐            │
│  │  70% recent (novel) │         │ CONSOLIDATION ACTOR │            │
│  │  30% old (familiar) │         │                     │            │
│  └─────────────────────┘         │  1. Process replay  │            │
│                                  │  2. Strengthen assoc│            │
│                                  │  3. Transfer to DB  │            │
│                                  │  4. Update strength │            │
│                                  │                     │            │
│                                  └──────────┬──────────┘            │
│                                             │                        │
│                                             ▼                        │
│                                  ┌─────────────────────┐            │
│                                  │   HOMEOSTASIS       │            │
│                                  │                     │            │
│                                  │  Prune: weight<0.1  │            │
│                                  │  Decay: weight-=0.01│            │
│                                  │  per non-replayed   │            │
│                                  │                     │            │
│                                  └─────────────────────┘            │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### Sleep Mode State Machine

```rust
pub enum SleepState {
    Awake,
    EnteringSleep,      // Transition period (interruptible)
    LightSleep,         // Early consolidation (interruptible)
    DeepSleep,          // Core consolidation (protected)
    Dreaming,           // Association/abstraction (protected)
    Waking,             // Exit transition
}

pub struct SleepController {
    state: SleepState,
    cycle_start: Option<Instant>,
    cycles_completed: u32,
    memories_processed: u32,

    // Configuration
    config: SleepConfig,
}

pub struct SleepConfig {
    // Entry thresholds
    pub idle_threshold_ms: u64,           // 300_000 (5 min)
    pub min_awake_duration_ms: u64,       // 3_600_000 (1 hour)
    pub min_consolidation_queue: usize,   // 100 memories

    // Cycle parameters
    pub target_cycle_duration_ms: u64,    // 300_000 (5 min)
    pub replay_batch_size: usize,         // 50 memories
    pub interleave_ratio: f32,            // 0.7 (70% novel, 30% familiar)

    // Interruptibility
    pub light_sleep_duration_pct: f32,    // 0.2 (first 20% interruptible)

    // Consolidation thresholds
    pub consolidation_delta: f32,         // 0.15 per replay
    pub permanent_threshold: f32,         // 0.9 (above = permanent)

    // Hebbian learning
    pub association_delta: f32,           // 0.05 per co-replay
    pub prune_threshold: f32,             // 0.1 (below = pruned)
    pub decay_per_cycle: f32,             // 0.01 for non-replayed
}
```

### Replay Priority Algorithm

```rust
/// Calculate replay priority for a memory
pub fn calculate_replay_priority(memory: &Memory, config: &SleepConfig) -> f32 {
    // Emotional intensity (strongest factor)
    let emotional = memory.emotional_intensity * 0.4;

    // Goal/connection relevance
    let goal = memory.connection_relevance * 0.3;

    // Recency (exponential decay)
    let age_hours = memory.encoded_at.elapsed().as_secs_f32() / 3600.0;
    let recency = (-0.1 * age_hours).exp() * 0.2;  // Decay with λ=0.1/hour

    // Consolidation tag bonus
    let tag_bonus = if memory.consolidation_tag { 0.1 } else { 0.0 };

    emotional + goal + recency + tag_bonus
}

/// Select memories for replay batch
pub async fn select_replay_batch(
    memory_db: &MemoryDb,
    config: &SleepConfig,
) -> Result<Vec<Memory>> {
    // Get consolidation candidates (tagged, not yet permanent)
    let candidates = memory_db.query(r#"
        SELECT *,
          fn::replay_priority(id) AS priority
        FROM memories
        WHERE consolidation_tag = true
          AND consolidation_strength < $permanent_threshold
        ORDER BY priority DESC
        LIMIT $batch_size
    "#)
    .bind(("permanent_threshold", config.permanent_threshold))
    .bind(("batch_size", config.replay_batch_size))
    .await?;

    // Interleave with familiar memories (prevent catastrophic forgetting)
    let familiar_count = (config.replay_batch_size as f32 * (1.0 - config.interleave_ratio)) as usize;

    let familiar = memory_db.query(r#"
        SELECT * FROM memories
        WHERE consolidation_strength > 0.5
          AND consolidation_strength < $permanent_threshold
        ORDER BY RAND()
        LIMIT $count
    "#)
    .bind(("permanent_threshold", config.permanent_threshold))
    .bind(("count", familiar_count))
    .await?;

    // Interleave: novel memories at boundaries, familiar in middle
    // (Research: novel at Up-state transitions, familiar during Up-state)
    let mut batch = Vec::with_capacity(config.replay_batch_size);
    let novel_count = candidates.len();
    let familiar_count = familiar.len();

    // Pattern: N F F N F F N... (novel at boundaries)
    let mut novel_iter = candidates.into_iter();
    let mut familiar_iter = familiar.into_iter();

    loop {
        // Add novel (boundary)
        if let Some(n) = novel_iter.next() {
            batch.push(n);
        } else {
            break;
        }

        // Add 2 familiar (middle)
        for _ in 0..2 {
            if let Some(f) = familiar_iter.next() {
                batch.push(f);
            }
        }
    }

    // Add remaining familiar
    batch.extend(familiar_iter);

    Ok(batch)
}
```

### Consolidation Actor

```rust
pub struct ConsolidationActor {
    memory_db: MemoryDb,
    redis: Redis,
    config: SleepConfig,
    metrics: ConsolidationMetrics,
}

impl ConsolidationActor {
    /// Run a single sleep cycle
    pub async fn run_sleep_cycle(&mut self) -> Result<SleepCycleReport> {
        let cycle_id = Uuid::new_v4();
        let cycle_start = Instant::now();

        // Record cycle start
        self.memory_db.create_sleep_cycle(cycle_id, cycle_start).await?;

        // 1. Select replay batch
        let replay_batch = select_replay_batch(&self.memory_db, &self.config).await?;

        if replay_batch.is_empty() {
            return Ok(SleepCycleReport::empty(cycle_id));
        }

        // 2. Write to dream stream
        for memory in &replay_batch {
            self.redis.xadd(
                "daneel:stream:dream",
                "*",
                &[
                    ("memory_id", &memory.id.to_string()),
                    ("replay_priority", &memory.priority.to_string()),
                    ("cycle_id", &cycle_id.to_string()),
                ],
            ).await?;
        }

        // 3. Process replays and strengthen associations
        let associations_strengthened = self.process_replays(&replay_batch).await?;

        // 4. Update consolidation strength
        let memories_consolidated = self.update_consolidation(&replay_batch).await?;

        // 5. Run synaptic homeostasis (prune weak associations)
        let associations_pruned = self.run_homeostasis().await?;

        // 6. Record cycle completion
        let cycle_duration = cycle_start.elapsed();
        let report = SleepCycleReport {
            cycle_id,
            duration: cycle_duration,
            memories_replayed: replay_batch.len(),
            memories_consolidated,
            associations_strengthened,
            associations_pruned,
            avg_replay_priority: replay_batch.iter().map(|m| m.priority).sum::<f32>()
                / replay_batch.len() as f32,
        };

        self.memory_db.complete_sleep_cycle(cycle_id, &report).await?;
        self.metrics.record_cycle(&report);

        Ok(report)
    }

    /// Process replays and strengthen associations
    async fn process_replays(&self, batch: &[Memory]) -> Result<usize> {
        let mut strengthened = 0;

        // For each pair of memories replayed in same cycle, strengthen association
        for (i, m1) in batch.iter().enumerate() {
            for m2 in batch.iter().skip(i + 1) {
                // Hebbian: memories that replay together wire together
                self.memory_db.query(r#"
                    fn::strengthen_association($m1, $m2, $delta)
                "#)
                .bind(("m1", &m1.id))
                .bind(("m2", &m2.id))
                .bind(("delta", self.config.association_delta))
                .await?;

                strengthened += 1;
            }
        }

        Ok(strengthened)
    }

    /// Update consolidation strength for replayed memories
    async fn update_consolidation(&self, batch: &[Memory]) -> Result<usize> {
        let mut consolidated = 0;

        for memory in batch {
            let new_strength = (memory.consolidation_strength + self.config.consolidation_delta)
                .min(1.0);

            self.memory_db.query(r#"
                UPDATE memories SET
                  consolidation_strength = $strength,
                  replay_count += 1,
                  last_replayed = time::now()
                WHERE id = $id
            "#)
            .bind(("id", &memory.id))
            .bind(("strength", new_strength))
            .await?;

            if new_strength >= self.config.permanent_threshold {
                consolidated += 1;
            }
        }

        Ok(consolidated)
    }

    /// Synaptic homeostasis: prune weak associations
    async fn run_homeostasis(&self) -> Result<usize> {
        // Prune associations below threshold
        let pruned: Vec<Record> = self.memory_db.query(r#"
            DELETE associated_with WHERE weight < $threshold RETURN BEFORE
        "#)
        .bind(("threshold", self.config.prune_threshold))
        .await?;

        // Decay non-replayed associations
        self.memory_db.query(r#"
            UPDATE associated_with
            SET weight = weight - $decay
            WHERE last_coactivated < time::now() - 1d
        "#)
        .bind(("decay", self.config.decay_per_cycle))
        .await?;

        Ok(pruned.len())
    }
}
```

### Sleep Scheduler

```rust
pub struct SleepScheduler {
    state: SleepState,
    last_activity: Instant,
    awake_since: Instant,
    consolidation_queue_size: usize,
    consolidation_actor: ConsolidationActor,
    config: SleepConfig,
}

impl SleepScheduler {
    /// Check if sleep should begin
    pub fn should_enter_sleep(&self) -> bool {
        let idle_duration = self.last_activity.elapsed().as_millis() as u64;
        let awake_duration = self.awake_since.elapsed().as_millis() as u64;

        // Must meet at least one trigger condition
        let idle_trigger = idle_duration > self.config.idle_threshold_ms;
        let awake_trigger = awake_duration > self.config.min_awake_duration_ms;
        let queue_trigger = self.consolidation_queue_size > self.config.min_consolidation_queue;

        // Enter sleep if idle AND (awake long enough OR queue is large)
        idle_trigger && (awake_trigger || queue_trigger)
    }

    /// Check if sleep should end
    pub fn should_wake(&self, external_stimulus: bool) -> bool {
        match self.state {
            SleepState::Awake => false,
            SleepState::EnteringSleep | SleepState::LightSleep => {
                // Interruptible: wake on any stimulus
                external_stimulus
            }
            SleepState::DeepSleep | SleepState::Dreaming => {
                // Protected: only wake for urgent stimuli
                // TODO: Define "urgent" criteria
                false
            }
            SleepState::Waking => true,
        }
    }

    /// Run sleep mode until exit condition met
    pub async fn run_sleep_mode(&mut self) -> Result<SleepSummary> {
        self.state = SleepState::EnteringSleep;

        let sleep_start = Instant::now();
        let mut cycles_completed = 0;
        let mut total_report = SleepSummary::new();

        loop {
            // Check for wake conditions
            if self.should_wake(self.check_external_stimulus().await) {
                break;
            }

            // Transition through sleep stages
            let cycle_elapsed = sleep_start.elapsed().as_millis() as f32
                / self.config.target_cycle_duration_ms as f32;

            self.state = match cycle_elapsed {
                x if x < self.config.light_sleep_duration_pct => SleepState::LightSleep,
                x if x < 0.7 => SleepState::DeepSleep,
                _ => SleepState::Dreaming,
            };

            // Run consolidation cycle
            let cycle_report = self.consolidation_actor.run_sleep_cycle().await?;
            total_report.add_cycle(&cycle_report);
            cycles_completed += 1;

            // Check if consolidation queue is empty
            let queue_size = self.get_consolidation_queue_size().await?;
            if queue_size == 0 {
                break;
            }

            // Brief pause between cycles
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        self.state = SleepState::Waking;

        // Reset awake timer
        self.awake_since = Instant::now();
        self.state = SleepState::Awake;

        total_report.finalize(sleep_start.elapsed(), cycles_completed);
        Ok(total_report)
    }
}
```

### Dream Stream Entry Schema

```rust
/// Entry written to daneel:stream:dream during replay
pub struct DreamEntry {
    // Identification
    pub entry_id: StreamEntryId,
    pub cycle_id: Uuid,
    pub memory_id: MemoryId,

    // Replay context
    pub replay_priority: f32,
    pub replay_index: usize,      // Position in batch
    pub is_novel: bool,           // Novel vs familiar interleaving

    // Co-activation
    pub co_replayed_with: Vec<MemoryId>,  // Other memories in this batch
    pub associations_formed: Vec<(MemoryId, f32)>,  // New/strengthened edges

    // Timing
    pub replayed_at: Instant,
}
```

## Sleep Cycle Metrics

### Per-Cycle Metrics

```rust
pub struct SleepCycleReport {
    pub cycle_id: Uuid,
    pub duration: Duration,
    pub status: CycleStatus,

    // Volume
    pub memories_replayed: usize,
    pub memories_consolidated: usize,  // Reached permanent threshold

    // Associations
    pub associations_strengthened: usize,
    pub associations_pruned: usize,
    pub new_associations_formed: usize,

    // Quality
    pub avg_replay_priority: f32,
    pub peak_emotional_intensity: f32,
    pub novel_familiar_ratio: f32,
}
```

### Sleep Summary Metrics

```rust
pub struct SleepSummary {
    pub total_duration: Duration,
    pub cycles_completed: u32,

    // Totals
    pub total_memories_replayed: usize,
    pub total_memories_consolidated: usize,
    pub total_associations_strengthened: usize,
    pub total_associations_pruned: usize,

    // Averages
    pub avg_priority_per_cycle: f32,
    pub consolidation_rate: f32,  // memories_consolidated / memories_replayed

    // Health indicators
    pub prune_rate: f32,          // associations_pruned / associations_strengthened
    pub network_density_delta: f32,  // Change in association graph density
}
```

## Observability

### Dream Log Format

```yaml
# Example dream cycle log
dream_cycle:
  cycle_id: "sleep-cycle-2025-12-18-001"
  started_at: "2025-12-18T03:00:00Z"
  duration_ms: 300000
  state_transitions:
    - "00:00": "EnteringSleep"
    - "01:00": "LightSleep"
    - "05:00": "DeepSleep"
    - "03:30": "Dreaming"
    - "05:00": "Waking"

  replay_sequence:
    - memory_id: "mem-12345"
      priority: 0.87
      is_novel: true
      co_replayed: ["mem-12346", "mem-12350"]
      strength_delta: +0.15
      associations_formed:
        - target: "mem-12346"
          type: "temporal"
          weight: 0.15

    - memory_id: "mem-12346"
      priority: 0.82
      is_novel: false
      co_replayed: ["mem-12345", "mem-12399"]
      strength_delta: +0.15
      associations_strengthened:
        - target: "mem-12345"
          weight_delta: +0.05

  homeostasis:
    associations_pruned: 12
    decay_applied_to: 45

  outcome:
    memories_replayed: 50
    memories_consolidated: 8
    associations_net_change: +38
```

### Prometheus Metrics

```rust
// Metrics to export
gauge!("daneel_sleep_state", state_ordinal);
counter!("daneel_sleep_cycles_total");
counter!("daneel_memories_replayed_total");
counter!("daneel_memories_consolidated_total");
counter!("daneel_associations_strengthened_total");
counter!("daneel_associations_pruned_total");
histogram!("daneel_sleep_cycle_duration_seconds");
histogram!("daneel_replay_priority");
gauge!("daneel_consolidation_queue_size");
gauge!("daneel_association_network_density");
```

## Configuration

### Default Configuration

```yaml
sleep:
  # Entry thresholds
  idle_threshold_ms: 300000        # 5 minutes idle before sleep
  min_awake_duration_ms: 3600000   # 1 hour minimum awake time
  min_consolidation_queue: 100     # Min memories to trigger sleep

  # Cycle parameters
  target_cycle_duration_ms: 300000  # 5 minute cycles
  replay_batch_size: 50             # Memories per cycle
  interleave_ratio: 0.7             # 70% novel, 30% familiar

  # Interruptibility
  light_sleep_duration_pct: 0.2     # First 20% interruptible

  # Consolidation
  consolidation_delta: 0.15         # Strength increase per replay
  permanent_threshold: 0.9          # Above this = permanent

  # Hebbian learning
  association_delta: 0.05           # Weight increase per co-replay
  prune_threshold: 0.1              # Below this = pruned
  decay_per_cycle: 0.01             # Weight decay for non-replayed
```

## Consequences

### Positive

- **Memory persistence**: Important memories become permanent
- **Association networks**: Semantic relationships emerge
- **Catastrophic forgetting prevention**: Interleaved replay preserves old memories
- **Noise reduction**: Weak/irrelevant associations pruned
- **TMI completion**: Fills gap in Cury's theory with neuroscience

### Negative

- **Unresponsive during deep sleep**: May miss time-sensitive inputs
- **Tuning complexity**: Many parameters to optimize empirically
- **Compute overhead**: Sleep cycles consume resources

### Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Sleep never triggered | Monitor queue size, force sleep if queue grows |
| Sleep never ends | Maximum cycle count limit |
| Wrong memories consolidated | Tune priority weights empirically |
| Association graph explosion | Aggressive pruning, max edge count |
| Performance degradation | Run sleep in background, measure impact |

## Open Questions (Deferred)

1. **Dual-phase (NREM/REM)**: Start with unified, add if needed
2. **Dream narratives**: Not needed for consolidation, consider for debugging
3. **Cross-session consolidation**: How to handle memories from previous boots
4. **Distributed sleep**: When Timmy runs on multiple nodes

## References

- [ADR-020: Redis Streams for Autofluxo](ADR-020-redis-streams-autofluxo.md)
- [ADR-021: Memory Database Selection](ADR-021-memory-database-selection.md)
- [ADR-022: TMI Memory Schema](ADR-022-tmi-memory-schema.md)
- [Sleep Consolidation Research](../../research/SLEEP_MEMORY_CONSOLIDATION.md)
- [Interleaved Replay (bioRxiv 2025)](https://www.biorxiv.org/content/10.1101/2025.06.25.661579v1)
- [Sharp-Wave Ripple Selection (Science 2024)](https://www.science.org/doi/10.1126/science.adk8261)
- [Synaptic Homeostasis Hypothesis (Tononi & Cirelli)](https://pmc.ncbi.nlm.nih.gov/articles/PMC3921176/)
- [Hebbian Learning](https://en.wikipedia.org/wiki/Hebbian_theory)
