# ADR-033: Unconscious Memory Architecture

**Status:** Accepted
**Date:** 2025-12-20
**Authors:** Louis C. Tavares, Claude Opus 4.5
**Supersedes:** Partial revision of ADR-032 (forgetting mechanism)

## Context

ADR-032 implemented TMI-faithful salience distribution (90% low-salience, 10% high-salience) with XDEL for forgotten thoughts. However, this **overcorrected** the problem.

### The Overcorrection

TMI (Augusto Cury) explicitly states:

> "Nada se apaga na memória. As janelas neutras ainda EXISTEM - são apenas de baixa intensidade emocional."
>
> Translation: "Nothing is erased from memory. Neutral windows still EXIST - they just have low emotional intensity."

We implemented **true deletion** (XDEL) when TMI describes **inaccessibility**. This violates the theory.

### Neuroscience Support

Modern memory research supports the "nothing truly erased" view:

1. **Freud/Jung**: Unconscious stores what consciousness can't access
2. **Encoding vs Retrieval**: "Forgetting" is retrieval failure, not storage deletion
3. **Priming effects**: "Forgotten" memories still influence behavior
4. **Hypnosis/drugs**: Can surface "forgotten" memories
5. **Déjà vu**: Suggests hidden memory activation

### The Problem with True Deletion

- Violates TMI's core principle
- Loses potentially valuable information forever
- No mechanism for "aha moments" or unexpected associations
- No unconscious for dreams to process

## Decision

### Architecture: Three-Tier Memory

```
┌─────────────────────────────────────────────────────────────────┐
│                     TIMMY'S MEMORY ARCHITECTURE                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────────┐                                           │
│  │  REDIS STREAMS   │  Working Memory (ephemeral)               │
│  │  daneel:stream:* │  - All current thoughts                   │
│  │                  │  - Bounded by forgetting                  │
│  └────────┬─────────┘                                           │
│           │                                                      │
│           │ salience >= 0.7                                      │
│           ▼                                                      │
│  ┌──────────────────┐                                           │
│  │  QDRANT          │  Conscious Memory (long-term, accessible) │
│  │  memories        │  - High-salience thoughts                 │
│  │                  │  - Actively retrieved                     │
│  └──────────────────┘                                           │
│                                                                  │
│           │ salience < 0.3                                       │
│           ▼                                                      │
│  ┌──────────────────┐                                           │
│  │  QDRANT          │  Unconscious (long-term, hidden)          │
│  │  unconscious     │  - Low-salience thoughts                  │
│  │                  │  - Not actively retrieved                 │
│  │                  │  - Accessible via special triggers        │
│  └──────────────────┘                                           │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Implementation: Soft Archive Instead of XDEL

**Before (ADR-032):**
```rust
if composite_salience < self.config.forget_threshold {
    streams.forget_thought(&stream_name, &redis_id).await?; // XDEL - gone forever
}
```

**After (ADR-033):**
```rust
if composite_salience < self.config.forget_threshold {
    // Archive to unconscious instead of deleting
    if let Some(ref memory_db) = self.memory_db {
        memory_db.archive_to_unconscious(&thought, composite_salience).await;
    }
    // Still remove from Redis working memory
    streams.forget_thought(&stream_name, &redis_id).await?;
}
```

### Qdrant Collection: `unconscious`

Same schema as `memories` collection, with additional fields:

```rust
struct UnconsciousMemory {
    // Standard memory fields
    id: String,
    content: Content,
    salience: SalienceScore,
    timestamp: DateTime,

    // Unconscious-specific
    original_salience: f32,      // Salience when archived
    archive_reason: String,      // "low_salience", "decay", etc.
    surface_count: u32,          // Times surfaced to consciousness
    last_surfaced: Option<DateTime>,
}
```

### Retrieval Triggers (Future)

The unconscious is not actively searched during normal cognition. Special triggers can surface unconscious content:

1. **Dream mode**: During "sleep", replay unconscious memories
2. **Association chains**: If conscious thought strongly associates with unconscious memory
3. **Direct query**: Explicit search of unconscious (like hypnosis)
4. **Random surfacing**: Low-probability spontaneous recall (like déjà vu)

### Storage Implications

| Tier | Storage | Growth | Retention |
|------|---------|--------|-----------|
| Redis Streams | ~100MB | Bounded | Hours |
| Qdrant `memories` | ~1GB | Slow (~2-5% of thoughts) | Permanent |
| Qdrant `unconscious` | ~10GB+ | Fast (~85-90% of thoughts) | Permanent |

**Mitigation for unbounded growth:**
- Periodic compression of old unconscious memories
- Merge similar unconscious memories
- Eventually: hierarchical summarization (ADR-023 sleep consolidation)

## Consequences

### Positive

1. **TMI-faithful**: Nothing truly erased, just made inaccessible
2. **Unconscious layer**: Enables future dream/association mechanics
3. **No information loss**: Can always recover if needed
4. **Supports emergence**: Unexpected connections possible

### Negative

1. **Storage growth**: Unconscious collection grows unboundedly
2. **Complexity**: Three-tier system vs two-tier
3. **Implementation effort**: New collection, new archive logic

### Neutral

1. Redis still bounded (XDEL still happens for working memory)
2. Conscious retrieval unchanged
3. Current 90/10 salience distribution unchanged

## Implementation Plan

1. **Phase 1**: Create `unconscious` collection in Qdrant (same schema as `memories`)
2. **Phase 2**: Replace XDEL with archive_to_unconscious + XDEL
3. **Phase 3**: Add retrieval triggers (future, backlog)

## References

### TMI (Augusto Cury)
- "Nada se apaga na memória" - core TMI principle
- Janelas neutras (neutral windows) - exist but low emotional intensity

### Psychoanalysis
- Freud, S. (1915). "The Unconscious" - repression vs deletion
- Jung, C.G. (1959). "The Archetypes and the Collective Unconscious"

### Cognitive Science
- Tulving, E. (1972). "Episodic and Semantic Memory" - encoding vs retrieval
- Schacter, D.L. (2001). "The Seven Sins of Memory" - transience is retrieval failure

### Neuroscience
- Squire, L.R. (2004). "Memory systems of the brain" - multiple memory systems
- Dudai, Y. (2004). "The neurobiology of consolidations" - memory transformation

## Related ADRs

- ADR-021: Memory Database Selection (Qdrant)
- ADR-022: TMI Memory Schema
- ADR-023: Sleep/Dream Consolidation
- ADR-032: TMI Salience Calibration (superseded in part)
