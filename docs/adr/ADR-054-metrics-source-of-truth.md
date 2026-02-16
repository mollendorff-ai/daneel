# ADR-054: Metrics Source of Truth

**Status:** Accepted
**Date:** 2026-01-01
**Author:** Rex (Möllendorff) + Claude Opus 4.5
**Updates:** ADR-041 (Entropy Calculation Standardization)
**Related:** ADR-053 (Headless Default, Web Observatory)

---

## Context

ADR-041 standardized entropy calculation with a TMI-aligned formula. However, implementation diverged:

### Current State (Broken)

| Metric | TUI (app/entropy.rs) | API (handlers.rs) | Issue |
|--------|---------------------|-------------------|-------|
| **EMERGENT threshold** | > 0.6 | > 0.65 | Different! |
| **BALANCED threshold** | > 0.3 | > 0.35 | Different! |
| **CV scaling** | / 1.0 | / 2.0 | 2x difference |
| **Burst ratio** | / 4.0 | / 14.0 | 3.5x difference |
| **Data source** | In-memory VecDeque | Redis stream | Divergent |

**Result:** TUI and daneel-web show different values for the same moment.

### Data Flow (Current - Broken)

```
Redis Stream
     │
     ├──► TUI App State (in-memory) ──► TUI Display
     │         └── calculate_entropy() [DIFFERENT THRESHOLDS]
     │
     └──► API handlers.rs ──► daneel-web
               └── compute_entropy() [DIFFERENT THRESHOLDS]
```

### Additional Issue

daneel-web queries Redis/Qdrant directly for some metrics (identity, system stats) instead of using the daneel API. This creates:
- Triple query load (TUI + API + daneel-web all query Redis)
- Potential inconsistency if queries happen at different times

## Decision

**Single source of truth: daneel API calculates, clients display.**

### Architecture (Fixed)

```
Redis Stream / Qdrant
         │
         ▼
┌─────────────────────┐
│   daneel core       │
│   src/core/metrics/ │  ◄── Single calculation
│   - entropy.rs      │
│   - fractality.rs   │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│   daneel API        │
│   /extended_metrics │  ◄── Single endpoint
└──────────┬──────────┘
           │
     ┌─────┴─────┐
     ▼           ▼
┌─────────┐ ┌─────────────┐
│ Scripts │ │ daneel-web  │  ◄── Display only
│ (curl)  │ │ (no calc)   │
└─────────┘ └─────────────┘
```

### Unified Thresholds

Per ADR-041, using the API values (more conservative):

| State | Threshold | Meaning |
|-------|-----------|---------|
| EMERGENT | > 0.65 | High cognitive diversity |
| BALANCED | > 0.35 | Healthy middle ground |
| CLOCKWORK | ≤ 0.35 | Low diversity (routine) |

### Unified Fractality Calculation

```rust
// Single formula in src/core/metrics/fractality.rs
pub fn calculate_fractality(inter_arrival_times: &[Duration]) -> f32 {
    let cv = coefficient_of_variation(times);
    let burst_ratio = calculate_burst_ratio(times);

    let cv_component = (cv / 2.0).clamp(0.0, 1.0);      // Standardized
    let burst_component = (burst_ratio / 14.0).clamp(0.0, 1.0);  // Standardized

    0.6 * cv_component + 0.4 * burst_component
}
```

## Implementation

### Phase 1: Consolidate Calculation

Create `src/core/metrics/` module:
```
src/core/metrics/
├── mod.rs           # pub use entropy, fractality
├── entropy.rs       # calculate_entropy() - single implementation
├── fractality.rs    # calculate_fractality() - single implementation
└── thresholds.rs    # EMERGENT_THRESHOLD, BALANCED_THRESHOLD constants
```

### Phase 2: API Uses Core Metrics

Update `src/api/handlers.rs`:
```rust
use crate::core::metrics::{calculate_entropy, calculate_fractality};

async fn extended_metrics() -> Json<ExtendedMetrics> {
    let entropy = calculate_entropy(&stream_data);
    let fractality = calculate_fractality(&timestamps);
    // ...
}
```

### Phase 3: Remove TUI Calculation

With ADR-053 (TUI deprecation), the divergent TUI calculation is removed entirely.

### Phase 4: daneel-web Fetches Only

Update daneel-web to:
1. Remove direct Redis queries for metrics
2. Fetch everything from `/extended_metrics` endpoint
3. Display only, no calculation

```rust
// daneel-web/src/main.rs
async fn fetch_metrics() -> DashboardMetrics {
    // BEFORE: Query Redis + Qdrant directly
    // AFTER: Single API call
    let response = client.get("http://localhost:3030/extended_metrics").await?;
    response.json().await
}
```

## Consequences

### Positive

- **No divergence** - Same values everywhere
- **Single maintenance** - Fix bugs in one place
- **Reduced load** - One query path, not three
- **Testable** - Core metrics module has unit tests
- **Consistent UX** - Users see same numbers in all interfaces

### Negative

- **API dependency** - daneel-web requires daneel API running
- **Latency** - API call vs direct Redis (negligible, ~1ms)

### Mitigations

- daneel-web health check verifies API availability
- Cache API response for 100-200ms (already done via polling interval)

## Success Criteria

1. `src/core/metrics/` module exists with tests
2. API uses core metrics (no duplicate calculation)
3. TUI removed (ADR-053)
4. daneel-web fetches from API only
5. All thresholds match (0.65/0.35)
6. Fractality uses consistent scaling (cv/2.0, burst/14.0)

---

## References

- ADR-041: Entropy Calculation Standardization (updated by this ADR)
- ADR-053: Headless Default, Web Observatory (companion ADR)
- TMI Salience Calibration: ADR-032

---

*"One source of truth. No divergence. No surprises."*
