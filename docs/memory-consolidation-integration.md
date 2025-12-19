# Memory Consolidation Integration Guide

This guide explains how memory consolidation to Qdrant is wired up in DANEEL.

## Architecture Overview

```
Cognitive Loop (Anchor Stage)
       ↓
   Thought produced?
       ↓
   Salience > threshold?
       ↓
   Convert to Memory
       ↓
   Store to Qdrant (async, non-blocking)
```

## Components

### 1. CognitiveLoop (`src/core/cognitive_loop.rs`)

The cognitive loop now includes:

- **MemoryDb client**: Optional `Arc<MemoryDb>` for sharing across tasks
- **Consolidation threshold**: Default 0.7, configurable
- **consolidate_memory()**: Async method called in Anchor stage
- **thought_to_memory()**: Converts Thought → Memory

### 2. MemoryDb (`src/memory_db/mod.rs`)

The Qdrant client with:

- **connect_and_init()**: Convenience method for quick setup
- **store_memory()**: Persist memory with 768-dim vector
- **init_collections()**: Creates memories/episodes/identity collections

## Integration Steps

### Step 1: Start Qdrant

```bash
docker compose up -d qdrant
```

Qdrant will be available at `http://localhost:6334` (gRPC) and `http://localhost:6333` (HTTP/UI).

### Step 2: Initialize MemoryDb

```rust
use daneel::memory_db::MemoryDb;
use std::sync::Arc;

// Connect and initialize collections
let memory_db = MemoryDb::connect_and_init("http://localhost:6334").await?;
let memory_db = Arc::new(memory_db);
```

### Step 3: Wire into CognitiveLoop

```rust
use daneel::core::cognitive_loop::CognitiveLoop;
use daneel::config::CognitiveConfig;

// Create cognitive loop
let mut cognitive_loop = CognitiveLoop::with_config(CognitiveConfig::human());

// Inject memory database
cognitive_loop.set_memory_db(Arc::clone(&memory_db));

// Optional: Adjust threshold
cognitive_loop.set_consolidation_threshold(0.6);
```

### Step 4: Run the Loop

```rust
cognitive_loop.start();

loop {
    let result = cognitive_loop.run_cycle().await;

    // High-salience thoughts are automatically consolidated
    // to Qdrant in the background (non-blocking)
}
```

## How It Works

### Anchor Stage (Memory Encoding)

In the Anchor stage of the cognitive loop:

1. **Check if thought was produced**: `if let Some(thought) = &thought_produced`
2. **Calculate composite salience**: Weighted sum of importance, novelty, relevance, valence, connection
3. **Threshold check**: Only store if `salience > consolidation_threshold`
4. **Convert to Memory**: Create Memory struct with emotional state, source, etc.
5. **Generate vector**: Currently dummy 768-dim zeros (TODO: real embeddings)
6. **Spawn async task**: `tokio::spawn` to avoid blocking the cognitive loop
7. **Store to Qdrant**: `memory_db.store_memory(&memory, &vector)`

### Error Handling

- Connection failures are logged but don't crash the loop
- Storage errors are logged to tracing
- The cognitive loop continues running even if Qdrant is unavailable

## Configuration

### Consolidation Threshold

```rust
// Default: 0.7 (high salience only)
cognitive_loop.set_consolidation_threshold(0.7);

// More permissive: 0.5 (store medium-salience thoughts)
cognitive_loop.set_consolidation_threshold(0.5);

// Very selective: 0.9 (only exceptional thoughts)
cognitive_loop.set_consolidation_threshold(0.9);
```

### Qdrant URL

```rust
// Default: localhost
let db = MemoryDb::connect_and_init("http://localhost:6334").await?;

// Remote Qdrant
let db = MemoryDb::connect_and_init("http://qdrant.example.com:6334").await?;
```

## Collections

Three Qdrant collections are created on initialization:

1. **memories**: Individual memory records with 768-dim context vectors
2. **episodes**: Event boundaries (Door Syndrome segmentation)
3. **identity**: Timmy's persistent self-concept (singleton)

All use cosine distance for similarity search.

## Vector Embeddings

**Current**: Dummy 768-dim zero vectors

**TODO**: Generate real embeddings using:
- sentence-transformers/all-mpnet-base-v2 (768-dim)
- Or custom embedding model
- Via LLM integration (Phase 2)

## Monitoring

### Tracing

Memory consolidation emits debug logs:

```
DEBUG memory_id=... salience=0.85 "Memory consolidated to Qdrant"
DEBUG thought_id=... salience=0.55 threshold=0.7 "Thought below consolidation threshold - not storing"
ERROR memory_id=... error=... "Failed to consolidate memory to Qdrant"
```

### Metrics

Check storage stats:

```rust
let memory_count = memory_db.memory_count().await?;
let episode_count = memory_db.episode_count().await?;
println!("Stored: {} memories, {} episodes", memory_count, episode_count);
```

## Example

See `examples/memory_consolidation.rs` for a complete working example:

```bash
cargo run --example memory_consolidation
```

## Testing

```bash
# Unit tests
cargo test memory_db

# Integration test (requires Qdrant)
docker compose up -d qdrant
cargo test --test memory_integration -- --ignored
```

## Future Work

- [ ] Real vector embeddings (currently dummy zeros)
- [ ] Episode boundary detection
- [ ] Association tracking (Hebbian co-activation)
- [ ] Sleep consolidation replay
- [ ] Identity persistence
- [ ] Memory pruning/forgetting

## References

- ADR-021: Memory Database Architecture
- ADR-022: Qdrant Integration
- ADR-023: Sleep Consolidation
- TMI Paper: Memory Consolidation (Gatilho da Memória)
