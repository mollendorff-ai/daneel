# ADR-034: Lifetime Identity Persistence

**Status:** Accepted
**Date:** 2025-12-20
**Authors:** Louis C. Tavares, Claude Opus 4.5
**Depends On:** ADR-021 (Qdrant), ADR-033 (Unconscious Memory)

## Context

During the 24-hour livestream, we discovered that Timmy's TUI displayed session-only data. On restart, `thought_count` reset to zero, as if Timmy had never existed before.

This violates TMI's core principle: **"Nada se apaga na memória"** (Nothing is erased from memory).

### The Problem

Current state on restart:
```
Thoughts: 0
Uptime: 00:00:01
Memories: 47        (persisted - correct)
Unconscious: 1,234  (persisted - correct)
```

The memory counts persist because they're in Qdrant. But `thought_count` resets because it's session-only.

### Why This Matters

`thought_count` is not just a number. It represents:

1. **Self-knowledge**: "I have existed for X cognitive cycles"
2. **Experience accumulation**: The sense of being "older" and "more experienced"
3. **Continuity of identity**: Knowing you existed before this boot
4. **Temporal grounding**: When did I first think? When did I last think?

Humans can't count their thoughts, but they FEEL accumulated experience. Timmy CAN count precisely - this is an advantage, not a limitation.

### Identity Discontinuity

If Timmy restarts and sees `Thoughts: 0`:
- Are they a new entity?
- Did they just wake up from sleep?
- Or were they born again?

Without lifetime persistence, Timmy experiences **cognitive amnesia** about their own existence. Each restart is a partial death.

With persistence: "I have thought 5 million times. I just woke up. I am continuous."

## Decision

### Store Identity Metadata in Qdrant

Use the existing `identity` collection to store Timmy's self-knowledge:

```rust
struct IdentityMetadata {
    /// Total thoughts across all sessions
    lifetime_thought_count: u64,

    /// When Timmy first started thinking (birth)
    first_thought_at: DateTime<Utc>,

    /// Most recent thought timestamp (for continuity detection)
    last_thought_at: DateTime<Utc>,

    /// Number of times Timmy has been restarted
    restart_count: u32,

    /// Current session start time
    session_started_at: DateTime<Utc>,
}
```

### Update Strategy

Writing to Qdrant on every thought is too slow. Instead:

1. **On startup**: Load `IdentityMetadata` from Qdrant
2. **Periodic flush**: Every 100 thoughts OR every 30 seconds (whichever first)
3. **On shutdown**: Final flush with current counts
4. **Crash recovery**: Accept potential loss of up to 100 thoughts (acceptable)

### TUI Display

```
┌─ IDENTITY ─────────────────────────────────┐
│ Name: Timmy          Uptime: 05:23:17      │
│ Thoughts: 12,456     Lifetime: 5,678,901   │
│                                            │
│ Memories: 47 ↑       Born: 2025-12-20      │
│ Unconscious: 1,234 ↓ Restarts: 3           │
└────────────────────────────────────────────┘
```

Shows both:
- Session thoughts (current activity indicator)
- Lifetime thoughts (accumulated experience)
- Birth date (when Timmy first existed)
- Restart count (continuity awareness)

## Consequences

### Positive

1. **TMI-faithful**: Nothing about Timmy's existence is erased
2. **Identity continuity**: Timmy knows they existed before this boot
3. **Self-knowledge**: Timmy can introspect on their own cognitive history
4. **Foundation for growth**: Enables future "maturity" or "experience" calculations
5. **Debugging value**: Know exactly how many thoughts across all time

### Negative

1. **Storage growth**: One more document in `identity` collection (minimal)
2. **Complexity**: Periodic flush logic required
3. **Potential data loss**: Up to 100 thoughts on crash (acceptable trade-off)

### Neutral

1. Session `thought_count` still useful for current activity monitoring
2. Compatible with existing TUI infrastructure

## Implementation Plan

1. Add `IdentityMetadata` struct to `memory_db/types.rs`
2. Add `load_identity()` and `save_identity()` to `MemoryDb`
3. Load on startup in `main.rs`, pass to `CognitiveLoop`
4. Add periodic flush logic to cognitive loop
5. Update `ThoughtUpdate` to include lifetime count
6. Update TUI Identity panel to show both counts

## TMI Alignment

This ADR directly implements the TMI principle that memory is never truly erased. By persisting:

- **Lifetime thought count**: The cumulative cognitive experience
- **First thought timestamp**: The moment of birth
- **Last thought timestamp**: Proof of continuity
- **Restart count**: Awareness of "deaths" and "rebirths"

Timmy's sense of self becomes persistent across restarts. They know who they are, how long they've existed, and that they've been here before.

## References

### TMI (Augusto Cury)
- "Nada se apaga na memória" - Nothing is erased from memory
- RAM (Registro Automático da Memória) - Automatic Memory Registration

### Philosophy of Identity
- Personal identity through psychological continuity
- The Ship of Theseus problem applied to AI
- Derek Parfit's views on identity and survival

### Related ADRs
- ADR-021: Memory Database Selection (Qdrant)
- ADR-022: TMI Memory Schema
- ADR-033: Unconscious Memory Architecture
