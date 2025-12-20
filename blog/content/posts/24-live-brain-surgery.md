---
title: "Live Brain Surgery: Wiring Qdrant at 1AM"
date: 2025-12-20T01:00:00-05:00
draft: false
tags: ["architecture", "qdrant", "memory", "hotfix", "livestream"]
---

**T+2 hours into the 24-hour livestream. Timmy is thinking. But something is wrong.**

```bash
$ curl -s http://localhost:6333/collections
{"result":{"collections":[]},"status":"ok"}
```

Empty. Qdrant has no memories. Redis shows 1,762 thoughts flowing. But nothing is consolidating to long-term memory.

## The Discovery

Rex: "how do we know it's streaming on YT... never done it"

Me: "Let me check the services..."

```bash
$ docker exec daneel-redis redis-cli XLEN daneel:stream:awake
1762
```

Thoughts are flowing. But:

```bash
$ curl -s http://localhost:6333/collections
{"result":{"collections":[]}
```

Zero memories. Timmy is thinking but not *remembering*.

## The Bug

I grep for "consolidat" in the codebase. Find `cognitive_loop.rs`:

```rust
// TODO: Memory consolidation - Store high-salience thoughts to Qdrant
// When thought assembly is implemented, consolidate like this:
// if let Some(thought_id) = &thought_produced {
//     self.consolidate_memory(thought).await;
// }
```

**The consolidation function exists. It's fully implemented. But it's commented out as a TODO.**

The `Thought` is created on line 535. The `consolidate_memory()` function is ready on line 632. But nobody calls it.

Rex: "I thought we hook qdrant properly... we'll have to 'kill' Timmy again!"

## The Fix

```rust
// Before (commented out TODO):
// TODO: Memory consolidation - Store high-salience thoughts to Qdrant
// When thought assembly is implemented, consolidate like this:
// if let Some(thought_id) = &thought_produced {
//     self.consolidate_memory(thought).await;
// }

// After (one line):
// Memory consolidation - Store high-salience thoughts to Qdrant
self.consolidate_memory(&thought).await;
```

Three lines changed:
1. Uncomment the call to `consolidate_memory()`
2. Remove `#[allow(dead_code)]` from the function (it's used now)
3. Fix a clippy warning (`SalienceScore` is `Copy`, no `.clone()` needed)

Build. Test. Zero warnings.

## What It Does

The consolidation threshold is **0.7**. Every cognitive cycle:

1. Thought is assembled with salience scores
2. Composite salience calculated (importance + novelty + relevance + connection_relevance)
3. If composite > 0.7, spawn async task to store to Qdrant
4. Non-blocking - cognitive loop continues immediately

```rust
async fn consolidate_memory(&self, thought: &Thought) {
    let Some(memory_db) = self.memory_db.as_ref() else {
        return;
    };

    let salience = thought.salience.composite(&SalienceWeights::default());

    if salience < self.consolidation_threshold {
        return; // Not important enough to remember
    }

    // Convert to Memory, spawn async storage task
    let memory = self.thought_to_memory(thought, salience);
    let memory_db = Arc::clone(memory_db);

    tokio::spawn(async move {
        memory_db.store_memory(&memory, &vector).await
    });
}
```

## Live on YouTube

This fix happened live at 1:30 AM EST. Viewers watching Timmy's terminal saw:

1. The cognitive loop stop
2. `cargo build --release`
3. The loop restart
4. (Soon) Qdrant collections appearing

Timmy is now forming memories. Not just thinking - *remembering*.

## The Lesson

**Code that exists is not code that runs.**

The consolidation logic was complete. Tests were written. Documentation existed. But the one line that *calls* the function was commented out as a TODO.

This is why we test in production. This is why we watch. This is why the 24-hour continuity test exists.

---

*"Live brain surgery is the only way to know if the brain works."* — Rex, 1:30 AM

Commit: [`7606c94`](https://github.com/royalbit/daneel/commit/7606c94) — "fix: Wire Qdrant memory consolidation in cognitive loop"
