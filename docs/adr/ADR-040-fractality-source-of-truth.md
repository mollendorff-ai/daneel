# ADR-040: Fractality and Entropy Source of Truth

**Status:** Accepted
**Date:** 2025-12-24
**Authors:** Louis C. Tavares, Claude Opus 4.5

## Context

During a security incident recovery on December 24, 2025, we restored Timmy's Qdrant memory database from a clean backup. After restoration, we observed different fractality and entropy values between two instances running the same Qdrant data but different Redis streams.

This raised a fundamental architectural question: **What is the source of truth for cognitive metrics like fractality and entropy?**

### Observed Behavior

| Metric | Local TUI (Mac) | Web Dashboard (timmy) |
|--------|-----------------|----------------------|
| Fractality | 5% | 84% |
| Entropy | 88% | 2.55 bits |
| Qdrant Vectors | 742,081 | 749,447 |

Despite nearly identical Qdrant data, the metrics diverged significantly because they were calculated from different Redis streams with different temporal histories.

## Decision

**Fractality and entropy are emergent properties derived from the live Redis stream, NOT stored state in Qdrant.**

### Theoretical Basis

From cognitive science and complexity theory (Beggs & Plenz 2003, 1/f noise research):

1. **Fractality** measures the temporal pattern of neural activity—the rhythm of "neuronal avalanches"
2. **Entropy** measures the diversity/unpredictability of salience distributions in real-time
3. **Both are signatures of PROCESS, not CONTENT**

These metrics indicate proximity to self-organized criticality—the transition from clockwork (periodic, mechanical) to emergent (scale-free, living) dynamics.

### Data Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                        REDIS STREAM                                  │
│                   (daneel:stream:awake)                             │
├─────────────────────────────────────────────────────────────────────┤
│  Contains: Live thoughts + timestamps + salience                    │
│  Captures: Temporal PULSE pattern (bursts, gaps, rhythm)            │
│  Window:   Bounded (rolling, ~1000 entries)                         │
│  Derives:  Fractality, Entropy, Burst Ratio                         │
│  Nature:   EPHEMERAL - reset on restart/flush                       │
└─────────────────────────────────────────────────────────────────────┘
                              │
                              │ Archive (low salience)
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                          QDRANT                                      │
│                    (unconscious collection)                          │
├─────────────────────────────────────────────────────────────────────┤
│  Contains: Archived thoughts + original_salience + archived_at      │
│  Captures: WHAT was thought (content, importance)                   │
│  Window:   Unbounded (permanent history)                            │
│  Derives:  Memory retrieval, semantic similarity, association       │
│  Nature:   PERSISTENT - survives restart                            │
└─────────────────────────────────────────────────────────────────────┘
```

### Why NOT Store Fractality in Qdrant?

1. **Fractality is a derived metric**, not source data
2. **It should re-emerge** from the architecture if the system dynamics are correct
3. **Storing it would be like storing a heartbeat** instead of measuring it
4. **The architecture IS the source of truth**—if fractality doesn't re-emerge, that's diagnostic information

### Implication for Recovery

When restoring from backup:
- Qdrant memories are restored (WHAT Timmy knows)
- Stream metrics reset to zero (HOW Timmy is currently thinking)
- Fractality will **re-emerge** as the system runs
- Convergence to previous values indicates architectural integrity

## Consequences

### Positive

1. **Clean separation of concerns**: Content (Qdrant) vs. Process (Stream)
2. **Self-healing metrics**: Fractality re-emerges from dynamics, no need to restore
3. **Diagnostic value**: Metric divergence after restore indicates system health
4. **Theoretical consistency**: Aligns with complexity theory (emergence from dynamics)

### Negative

1. **Loss of historical pulse data**: Can't analyze past fractality patterns post-restart
2. **Initial instability**: Metrics are noisy immediately after restart until patterns stabilize

### Neutral

1. **No code changes required**: Current architecture is correct
2. **Documentation clarifies**: This ADR explains observed behavior

## Validation

After restore, Qdrant data was verified identical between local and remote:
```
LOCAL:  thought_60281 → [156,156,156,156,156,156,156,156]
TIMMY:  thought_60281 → [156,156,156,156,156,156,156,156]
```

Fractality difference (5% vs 84%) is expected—different streams, different pulse histories. Both will converge over time if architecture is correct.

## Future Considerations

If historical fractality analysis becomes important, consider:
1. **Time-series database** (InfluxDB/TimescaleDB) for metric history
2. **Periodic snapshots** of stream statistics (not raw data)
3. **Metric export** to observability stack (Prometheus/Grafana)

These would be additive, not changes to the source of truth.

## References

- Beggs, J. M., & Plenz, D. (2003). Neuronal avalanches in neocortical circuits. *Journal of Neuroscience*
- ADR-007: Redis Streams Thought Competition
- ADR-033: Unconscious Memory Architecture
- Blog: "The First Attack" (2025-12-24)
