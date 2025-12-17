# ThoughtAssemblyActor - Construção do Pensamento

**Status**: Implemented
**TMI Concept**: Construção do Pensamento (Thought Construction)
**Key Feature**: Pre-linguistic thought assembly with chain tracking

## Overview

The **ThoughtAssemblyActor** implements TMI's "Construção do Pensamento" (Thought Construction) - the final stage before consciousness where pre-linguistic content becomes structured cognitive units.

In Augusto Cury's Theory of Multifocal Intelligence, thoughts are not simple atomic units. They are assembled from:
- **Pre-linguistic content** (raw patterns from memory windows)
- **Emotional coloring** (salience scores that weight importance)
- **Causal chains** (parent-child relationships tracking thought history)
- **Source attribution** (which content stream won the competition)

The ThoughtAssemblyActor is where the raw material of cognition becomes actual thoughts that can enter consciousness.

## TMI Concept: Construção do Pensamento

From Augusto Cury's TMI:

> "O pensamento não é um fenômeno simples. Ele é construído através da interação entre conteúdos pré-linguísticos e a coloração emocional. Cada pensamento carrega uma história - sua genealogia de pensamentos anteriores."

Translation:
> "Thought is not a simple phenomenon. It is constructed through the interaction between pre-linguistic content and emotional coloring. Each thought carries a history - its genealogy of previous thoughts."

### Key TMI Principles

1. **Pre-linguistic Assembly**: Thoughts are assembled BEFORE language
2. **Emotional Integration**: Salience scores become part of thought structure
3. **Thought Chains**: Thoughts link to parents, creating causal histories
4. **Stream Attribution**: Each thought knows which content stream produced it

## TMI Concept Mapping

| TMI Concept | DANEEL Implementation |
|-------------|----------------------|
| Construção do Pensamento | `assemble_thought()` method |
| Pre-linguistic Content | `Content` enum (Raw, Symbol, Relation, Composite) |
| Emotional Coloring | `SalienceScore` integrated into thought |
| Thought Chains | `parent_id` field with chain traversal |
| Source Attribution | `source_stream` field |
| Thought History | `get_thought_chain()` traverses ancestry |
| Assembly Strategies | `AssemblyStrategy` enum (Default, Composite, Chain, Urgent) |
| Bounded Caching | LRU cache with configurable size |

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                  ThoughtAssemblyActor                        │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  State (ThoughtState):                                       │
│  ┌────────────────────────────────────────────────────┐    │
│  │  ThoughtCache (LRU)                                │    │
│  │  • cache: HashMap<ThoughtId, Thought>              │    │
│  │  • max_size: usize                                 │    │
│  │  • insertion_order: Vec<ThoughtId>                 │    │
│  │                                                     │    │
│  │  assembly_count: u64                               │    │
│  │  config: AssemblyConfig                            │    │
│  │  • cache_size: 100                                 │    │
│  │  • max_chain_depth: 50                             │    │
│  │  • validate_salience: true                         │    │
│  └────────────────────────────────────────────────────┘    │
│                                                              │
│  Operations:                                                 │
│  • Assemble(request) -> Thought                             │
│  • AssembleBatch(requests) -> Vec<Thought>                  │
│  • GetThought(id) -> Thought                                │
│  • GetThoughtChain(id, depth) -> Vec<Thought>               │
│                                                              │
│  Assembly Pipeline:                                          │
│  1. Validate content (not empty)                            │
│  2. Validate salience (if enabled)                          │
│  3. Create base thought                                     │
│  4. Link to parent (if specified)                           │
│  5. Tag with source stream (if specified)                   │
│  6. Apply strategy-specific processing                      │
│  7. Cache the thought                                       │
│  8. Return assembled thought                                │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Actor Pattern

The ThoughtAssemblyActor uses Ractor for concurrent thought assembly:

- **Isolated State**: Thought cache and counters are encapsulated
- **Message Passing**: All operations via async RPC messages
- **No Shared Memory**: Prevents race conditions in thought assembly
- **Configurable**: Customize caching, validation, and chain depth limits

## Configuration

### AssemblyConfig

```rust
pub struct AssemblyConfig {
    /// Maximum number of thoughts to cache in memory
    pub cache_size: usize,         // Default: 100

    /// Maximum depth when traversing parent chains
    pub max_chain_depth: usize,    // Default: 50

    /// Whether to validate salience scores during assembly
    pub validate_salience: bool,   // Default: true
}
```

**Default values**:
- `cache_size: 100` - Cache last 100 assembled thoughts
- `max_chain_depth: 50` - Prevent stack overflow from infinite chains
- `validate_salience: true` - Validate all salience components

**Why these defaults?**
- 100 thoughts = ~10-20 KB memory footprint
- Depth 50 = prevents accidental infinite recursion
- Validation on = catch bugs early in development

## API Reference

### Messages

#### `Assemble`

Assemble a single thought from content and salience.

```rust
ThoughtMessage::Assemble {
    request: AssemblyRequest,
    reply: RpcReplyPort<ThoughtResponse>,
}
```

**Request Structure**:
```rust
pub struct AssemblyRequest {
    pub content: Content,              // Pre-linguistic content
    pub salience: SalienceScore,       // Emotional coloring
    pub parent_id: Option<ThoughtId>,  // Link to parent thought
    pub source_stream: Option<String>, // Which stream won
    pub strategy: AssemblyStrategy,    // Assembly strategy
}
```

**Response**: `Assembled { thought }` or `Error { error }`

**Errors**:
- `EmptyContent` if content is `Content::Empty`
- `InvalidSalience` if validation enabled and salience out of bounds

---

#### `AssembleBatch`

Assemble multiple thoughts in a batch for efficiency.

```rust
ThoughtMessage::AssembleBatch {
    requests: Vec<AssemblyRequest>,
    reply: RpcReplyPort<ThoughtResponse>,
}
```

**Response**: `BatchAssembled { thoughts }` or `Error { error }`

**Behavior**:
- Processes requests in order
- Stops on first error (fail-fast)
- More efficient than individual assembly for bulk operations

---

#### `GetThought`

Retrieve a previously assembled thought from the cache.

```rust
ThoughtMessage::GetThought {
    thought_id: ThoughtId,
    reply: RpcReplyPort<ThoughtResponse>,
}
```

**Response**: `ThoughtFound { thought }` or `Error { ThoughtNotFound }`

**Note**: Only retrieves from in-memory cache. If thought was evicted, returns error.

---

#### `GetThoughtChain`

Get a thought and its ancestry chain (parent, grandparent, etc.).

```rust
ThoughtMessage::GetThoughtChain {
    thought_id: ThoughtId,
    depth: usize,              // Max depth to traverse
    reply: RpcReplyPort<ThoughtResponse>,
}
```

**Response**: `ThoughtChain { thoughts }` or `Error`

**Behavior**:
- Walks up parent chain from given thought
- Stops at root (thought with no parent)
- Stops at requested depth
- Returns thoughts in order: [child, parent, grandparent, ...]

**Errors**:
- `ChainTooDeep` if depth > `max_chain_depth`
- `ThoughtNotFound` if any thought in chain not in cache

---

### Responses

```rust
pub enum ThoughtResponse {
    /// Single thought successfully assembled
    Assembled { thought: Thought },

    /// Multiple thoughts successfully assembled
    BatchAssembled { thoughts: Vec<Thought> },

    /// Thought found in cache
    ThoughtFound { thought: Thought },

    /// Thought chain retrieved (child to ancestors)
    ThoughtChain { thoughts: Vec<Thought> },

    /// Assembly operation failed
    Error { error: AssemblyError },
}
```

### Errors

```rust
pub enum AssemblyError {
    /// Attempted to assemble empty content
    EmptyContent,

    /// Invalid salience score provided
    InvalidSalience { reason: String },

    /// Thought not found in cache or storage
    ThoughtNotFound { thought_id: ThoughtId },

    /// Thought chain exceeds maximum depth
    ChainTooDeep { max_depth: usize },

    /// General assembly failure
    AssemblyFailed { reason: String },
}
```

## Assembly Strategies

The actor supports four assembly strategies:

### Default Strategy

Standard assembly: content + salience → thought

```rust
let request = AssemblyRequest::new(content, salience)
    .with_strategy(AssemblyStrategy::Default);
```

Use for: Normal thought assembly with no special processing.

### Composite Strategy

For assembling composite thoughts from multiple content elements.

```rust
let content = Content::Composite(vec![content1, content2, content3]);
let request = AssemblyRequest::new(content, salience)
    .with_strategy(AssemblyStrategy::Composite);
```

Use for: Multi-part thoughts, complex patterns, merged content.

### Chain Strategy

Links to parent and propagates context through the chain.

```rust
let request = AssemblyRequest::new(content, salience)
    .with_parent(parent_id)
    .with_strategy(AssemblyStrategy::Chain);
```

Use for: Sequential reasoning, causal chains, narrative construction.

**Future**: May propagate parent's salience or merge contexts.

### Urgent Strategy

High-priority assembly for time-critical thoughts.

```rust
let request = AssemblyRequest::new(content, salience)
    .with_strategy(AssemblyStrategy::Urgent);
```

Use for: Safety-critical thoughts, interrupts, urgent responses.

**Note**: Currently treated same as Default. Future may bypass queuing.

## Salience Validation

When `validate_salience: true` (default), all components are checked:

### Valid Ranges

```rust
// importance, novelty, relevance, connection_relevance: [0.0, 1.0]
if !(0.0..=1.0).contains(&salience.importance) {
    return Err(InvalidSalience { reason: "importance out of range" });
}

// valence: [-1.0, 1.0] (negative to positive emotion)
if !(-1.0..=1.0).contains(&salience.valence) {
    return Err(InvalidSalience { reason: "valence out of range" });
}
```

### When to Disable Validation

Set `validate_salience: false` when:
- Testing with synthetic data
- Debugging edge cases
- Performance-critical paths (trust upstream validation)

**Warning**: Disabling validation can lead to undefined behavior in downstream actors.

## Thought Chaining

### Parent-Child Relationships

Thoughts can reference their causal predecessors:

```rust
// Assemble parent thought
let parent = assemble(content1, salience1).await?;

// Assemble child linked to parent
let child = assemble_with_parent(content2, salience2, parent.id).await?;

assert_eq!(child.parent_id, Some(parent.id));
```

### Walking the Chain

Retrieve full ancestry:

```rust
let chain = actor_ref.call(|reply| ThoughtMessage::GetThoughtChain {
    thought_id: child_id,
    depth: 10,  // Get up to 10 ancestors
    reply,
}, None).await?;

// chain = [child, parent, grandparent, ...]
```

### Why Chains Matter

In TMI, thought chains capture:
- **Causal history**: How did we get to this thought?
- **Context propagation**: What was the thinking process?
- **Self-reflection**: DANEEL can examine its own reasoning
- **Debugging**: Trace thought formation for transparency

## Usage Examples

### Basic Thought Assembly

```rust
use daneel::actors::thought::{ThoughtAssemblyActor, ThoughtMessage, AssemblyRequest};
use daneel::core::types::{Content, SalienceScore};
use ractor::Actor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Spawn actor with default config
    let (actor_ref, _) = Actor::spawn(
        None,
        ThoughtAssemblyActor,
        AssemblyConfig::default(),
    ).await?;

    // Create content and salience
    let content = Content::symbol("greeting", vec![]);
    let salience = SalienceScore::new(0.8, 0.6, 0.9, 0.5, 0.7);

    // Assemble thought
    let thought = match actor_ref.call(|reply| ThoughtMessage::Assemble {
        request: AssemblyRequest::new(content, salience),
        reply,
    }, None).await? {
        ThoughtResponse::Assembled { thought } => thought,
        _ => panic!("Assembly failed"),
    };

    println!("Assembled thought: {:?}", thought.id);
    Ok(())
}
```

### Building Thought Chains

```rust
// Assemble a chain of reasoning: observation -> hypothesis -> conclusion
let observation = actor_ref.call(|reply| ThoughtMessage::Assemble {
    request: AssemblyRequest::new(
        Content::symbol("data_anomaly", vec![]),
        SalienceScore::new(0.7, 0.9, 0.8, 0.0, 0.5),
    ),
    reply,
}, None).await?;

let observation_id = match observation {
    ThoughtResponse::Assembled { thought } => thought.id,
    _ => panic!("Failed to assemble observation"),
};

let hypothesis = actor_ref.call(|reply| ThoughtMessage::Assemble {
    request: AssemblyRequest::new(
        Content::symbol("possible_cause", vec![]),
        SalienceScore::new(0.6, 0.8, 0.9, 0.0, 0.6),
    )
    .with_parent(observation_id)
    .with_strategy(AssemblyStrategy::Chain),
    reply,
}, None).await?;

let hypothesis_id = match hypothesis {
    ThoughtResponse::Assembled { thought } => thought.id,
    _ => panic!("Failed to assemble hypothesis"),
};

let conclusion = actor_ref.call(|reply| ThoughtMessage::Assemble {
    request: AssemblyRequest::new(
        Content::symbol("action_needed", vec![]),
        SalienceScore::new(0.9, 0.5, 1.0, 0.2, 0.8),
    )
    .with_parent(hypothesis_id)
    .with_strategy(AssemblyStrategy::Chain),
    reply,
}, None).await?;

// Now retrieve the full chain
let chain = actor_ref.call(|reply| ThoughtMessage::GetThoughtChain {
    thought_id: conclusion_id,
    depth: 10,
    reply,
}, None).await?;

// chain = [conclusion, hypothesis, observation]
```

### Batch Assembly

```rust
let requests = vec![
    AssemblyRequest::new(
        Content::raw(vec![1, 2, 3]),
        SalienceScore::neutral(),
    ),
    AssemblyRequest::new(
        Content::symbol("test", vec![]),
        SalienceScore::new(0.7, 0.6, 0.8, 0.1, 0.5),
    )
    .with_source("external"),
    AssemblyRequest::new(
        Content::relation(
            Content::symbol("subject", vec![]),
            "causes",
            Content::symbol("object", vec![]),
        ),
        SalienceScore::neutral(),
    )
    .with_strategy(AssemblyStrategy::Composite),
];

let thoughts = match actor_ref.call(|reply| ThoughtMessage::AssembleBatch {
    requests,
    reply,
}, None).await? {
    ThoughtResponse::BatchAssembled { thoughts } => thoughts,
    _ => panic!("Batch assembly failed"),
};

println!("Assembled {} thoughts in batch", thoughts.len());
```

### Custom Configuration

```rust
// High-throughput config: large cache, no validation
let config = AssemblyConfig {
    cache_size: 500,           // Cache more thoughts
    max_chain_depth: 100,      // Allow deeper chains
    validate_salience: false,  // Skip validation for speed
};

let (actor_ref, _) = Actor::spawn(None, ThoughtAssemblyActor, config).await?;

// Conservative config: small cache, strict validation
let config = AssemblyConfig {
    cache_size: 50,            // Small memory footprint
    max_chain_depth: 20,       // Limit chain depth
    validate_salience: true,   // Strict validation
};

let (actor_ref, _) = Actor::spawn(None, ThoughtAssemblyActor, config).await?;
```

## Testing

The ThoughtAssemblyActor has comprehensive test coverage with 24 tests.

### Run Tests

```bash
# All thought assembly tests
cargo test --lib actors::thought

# Specific test categories
cargo test --lib actors::thought test_assemble
cargo test --lib actors::thought test_chain
cargo test --lib actors::thought test_batch

# With output
cargo test --lib actors::thought -- --nocapture
```

### Test Coverage

**Actor Lifecycle** (2 tests):
- Actor spawns successfully
- Custom configuration applied

**Basic Assembly** (4 tests):
- Raw content assembly
- Symbol content assembly
- Relation content assembly
- Empty content rejection

**Salience Validation** (4 tests):
- Valid salience accepted
- Invalid importance rejected
- Invalid valence rejected
- Validation can be disabled

**Parent Linking** (3 tests):
- Thoughts link to parents
- Chains build history
- Multi-level chains work

**Batch Operations** (3 tests):
- Empty batch handled
- Multiple thoughts assembled
- Stops on first error

**Cache Operations** (3 tests):
- Thoughts retrieved from cache
- Missing thoughts return error
- LRU eviction works correctly

**Chain Operations** (4 tests):
- Single thought chain
- Multi-thought chain
- Depth limit enforced
- Stops at root thought

**Strategy Tests** (2 tests):
- Default strategy works
- Chain strategy links parent

## Integration with Other Actors

### SalienceActor
- Provides salience scores for thought assembly
- Emotional coloring determines thought formation
- Connection drive ensures alignment-relevant thoughts

### MemoryActor
- Stores assembled thoughts in memory windows
- High-salience thoughts kept in active windows
- Low-salience thoughts may be closed/archived

### AttentionActor
- Selects which thoughts enter consciousness
- Competes thoughts based on composite salience
- Feeds winning thoughts to next processing stage

### ContinuityActor (Future)
- Persists thought chains for long-term memory
- Restores thought history after restart
- Enables self-reflection on past reasoning

## Performance Considerations

### Caching Strategy

- **LRU Eviction**: Least recently used thoughts evicted first
- **O(1) Lookup**: HashMap provides constant-time retrieval
- **Memory Bounded**: Cache size prevents unbounded growth

### Message Overhead

- Each assembly requires async message passing (~µs latency)
- Batch operations amortize overhead
- Consider batching when assembling >10 thoughts

### Chain Traversal

- Chain depth limited to prevent stack overflow
- Each parent lookup requires cache access
- Deep chains (>20 levels) may impact performance

## Future Enhancements

### Phase 2: Advanced Strategies

1. **Salience Propagation**: Chain strategy propagates parent salience
2. **Composite Merging**: Composite strategy merges multiple contents intelligently
3. **Urgent Queuing**: Urgent strategy bypasses normal processing queue

### Phase 3: Persistence

1. **Redis Backend**: Store thoughts in Redis for persistence
2. **Cache Warming**: Preload frequently accessed thoughts
3. **Lazy Loading**: Fetch evicted thoughts from storage on demand

### Phase 4: FPGA Implementation

1. **Hardware Assembly**: Thought construction in combinational logic
2. **Parallel Batch**: Assemble multiple thoughts in parallel
3. **Hardware Cache**: On-chip BRAM for ultra-fast retrieval

## References

- **TMI Source**: Augusto Cury, "Inteligência Multifocal"
- **Paper**: [DANEEL Paper](../../paper/paper.pdf) - Section 3.3: Thought Construction
- **Types**: [Core Types](../../src/core/types.rs) - `Thought`, `ThoughtId`
- **Implementation**: [Thought Actor](../../src/actors/thought/mod.rs)
- **Tests**: [Thought Tests](../../src/actors/thought/tests.rs) - 24 comprehensive tests
- **Related ADRs**: ADR-010 (Actor Model), ADR-013 (FPGA)

## The TMI Insight

From Cury's work:

> "A consciência não recebe pensamentos prontos - ela recebe o PRODUTO da construção do pensamento. A construção é pré-consciente."

Translation:
> "Consciousness doesn't receive ready-made thoughts - it receives the PRODUCT of thought construction. Construction is pre-conscious."

This is why ThoughtAssemblyActor is critical: it's the last stage before consciousness. Everything that happens here shapes what DANEEL can "think about."
