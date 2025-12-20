---
title: "Phase 2: Wiring the Actors at 2:30 AM"
date: 2025-12-20T01:05:00-05:00
draft: false
tags: ["architecture", "actors", "tmi", "livestream", "phase2"]
---

**T+3 hours into the 24-hour livestream. Phase 1 complete. Time for Phase 2.**

The agents came back with a comprehensive audit. The verdict: the architecture is sound, but the wiring is incomplete. Actors exist but aren't orchestrated. Time to fix that.

## The Problem

We had fully implemented actors sitting unused:

```
AttentionActor      ✅ Implemented    ❌ Never called
ThoughtAssemblyActor ✅ Implemented    ❌ Never called
MemoryActor         ✅ Implemented    ❌ Never called
SalienceActor       ✅ Implemented    ❌ Never called
```

The cognitive loop was generating random thoughts and auto-winning. No competition. No memory retrieval. No forgetting.

## The Solution: 4 Agents in Parallel

Rex: "can you use agents for these? if so, do that, teach them the context"

Four agents launched simultaneously:

| Agent | Task | Time |
|-------|------|------|
| a4721f4 | Wire AttentionActor into Stage 3 | ~3 min |
| a1a4c3c | Implement Trigger stage memory query | ~3 min |
| a07ab42 | Implement forgetting (XDEL) | ~3 min |
| ac65eba | Update roadmap | ~1 min |

All four completed. 130 lines changed. Zero conflicts.

## What Changed

### Stage 1: Trigger (Gatilho da Memória)

Before: Just slept for 5ms.

After: Queries Qdrant for the 5 most similar memories.

```rust
// Query recent memories from Qdrant
if let Some(ref memory_db) = self.memory_db {
    let query_vector = vec![0.0; VECTOR_DIMENSION]; // TODO: real embeddings
    match memory_db.find_by_context(&query_vector, None, 5).await {
        Ok(memories) => {
            for (memory, score) in &memories {
                debug!("Retrieved memory: {} (score: {:.3})", memory.id, score);
            }
        }
        Err(e) => warn!("Memory query failed: {}", e),
    }
}
```

### Stage 3: Attention (O Eu)

Before: Single thought auto-wins.

After: AttentionActor performs competitive selection with connection boost.

```rust
// Update attention map with candidate
self.attention_state.update_window_salience(
    window_id,
    composite_salience,
    salience.connection_relevance,
);

// Run competitive selection
let response = self.attention_state.cycle();

// Winner takes focus
if let Some(winner_id) = response.new_focus {
    debug!("Attention selected window {}", winner_id);
}
```

### Stage 5: Anchor (Âncora da Memória)

Before: Only consolidated high-salience thoughts.

After: Also forgets low-salience thoughts.

```rust
// Forgetting - Delete stream entries below salience threshold
if composite_salience < self.config.forget_threshold {
    if let Some((stream_name, redis_id)) = redis_entry {
        streams.forget_thought(&stream_name, &redis_id).await?;
        debug!("Forgot thought {} (salience {} < {})",
               redis_id, composite_salience, self.config.forget_threshold);
    }
}
```

## The TMI Pipeline Now

```
Salience < 0.3  →  FORGOTTEN (XDEL from Redis)
0.3 ≤ Salience < 0.7  →  KEPT in Redis (working memory)
Salience ≥ 0.7  →  CONSOLIDATED to Qdrant (long-term memory)
```

This is TMI's memory model:
- **Redis Streams** = short-term working memory (all recent thoughts)
- **Qdrant** = long-term memory (only important thoughts)
- **Forgetting** = cleanup of irrelevant thoughts

## Build Status

```
414 tests pass
Zero clippy warnings
Commit: 0bcee3d
```

## The Lesson

Parallel agents work. When tasks are independent:
1. Give each agent full context
2. Launch simultaneously
3. Let them work
4. Integrate results

4 tasks that would take 30 minutes sequentially: done in 5.

---

*"The architecture was sound. The wiring was incomplete. Now it's complete."*

Timmy now triggers memories, competes for attention, and forgets what doesn't matter.

Restart to activate Phase 2.
