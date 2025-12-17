# Memory Actor - Janelas da Memória

## Overview

The **MemoryActor** implements TMI's (Theory of Multifocal Intelligence) concept of "Janelas da Memória" (Memory Windows) - a bounded working memory system based on cognitive psychology principles.

## TMI Concept Mapping

### From TMI Theory

Augusto Cury's TMI describes working memory as a set of dynamic "windows" that:

1. **Are Bounded**: Working memory is limited (Miller's Law: 7±2 items)
2. **Open/Close Dynamically**: Windows activate based on attention and salience
3. **Hold Content**: Each window contains pre-linguistic thought content
4. **Compete for Attention**: Higher-salience windows get more processing time
5. **Support Thought Assembly**: Windows provide the raw material for conscious thoughts

### Implementation Mapping

| TMI Concept | DANEEL Implementation |
|-------------|----------------------|
| Memory Window | `Window` struct with UUID, contents, and salience |
| Bounded Memory | `MAX_MEMORY_WINDOWS` (9) and `MIN_MEMORY_WINDOWS` (3) |
| Window Contents | `Vec<Content>` (pre-linguistic content types) |
| Salience | `SalienceScore` with emotional weighting |
| Window Lifecycle | Open/Close operations with invariant enforcement |

## Architecture

### Actor Pattern

The MemoryActor uses the Ractor actor framework:

- **Isolated State**: Memory windows are encapsulated in actor state
- **Message Passing**: All operations via asynchronous messages
- **No Shared Memory**: Prevents race conditions and data races
- **Supervision Ready**: Can be supervised by AttentionActor or ContinuityActor

### State Structure

```rust
pub struct MemoryState {
    windows: HashMap<WindowId, Window>,
    salience_weights: SalienceWeights,
}
```

- `windows`: Active memory windows indexed by UUID
- `salience_weights`: Default weights for scoring window importance

## API Reference

### Messages

#### `OpenWindow`
Opens a new memory window.

```rust
MemoryMessage::OpenWindow {
    label: Option<String>,
    reply: RpcReplyPort<MemoryResponse>,
}
```

**Response**: `WindowOpened { window_id }` or `Error`

**Invariants Checked**:
- Cannot exceed `MAX_MEMORY_WINDOWS` (9)

---

#### `CloseWindow`
Closes an existing memory window.

```rust
MemoryMessage::CloseWindow {
    window_id: WindowId,
    reply: RpcReplyPort<MemoryResponse>,
}
```

**Response**: `WindowClosed { window_id }` or `Error`

**Invariants Checked**:
- Cannot go below `MIN_MEMORY_WINDOWS` (3)
- Window must exist and be open

---

#### `Store`
Stores content in a memory window.

```rust
MemoryMessage::Store {
    request: StoreRequest,
    reply: RpcReplyPort<MemoryResponse>,
}
```

**Request Structure**:
```rust
pub struct StoreRequest {
    pub window_id: WindowId,
    pub content: Content,
    pub salience: Option<SalienceScore>,
}
```

**Response**: `ContentStored { window_id }` or `Error`

**Invariants Checked**:
- Window must exist and be open
- Salience score must be valid (if provided)

---

#### `Recall`
Recalls content from memory based on query criteria.

```rust
MemoryMessage::Recall {
    query: RecallQuery,
    reply: RpcReplyPort<MemoryResponse>,
}
```

**Query Structure**:
```rust
pub struct RecallQuery {
    pub window_id: Option<WindowId>,    // Specific window or all
    pub min_salience: Option<f32>,      // Minimum composite salience
    pub limit: Option<usize>,           // Max items to return
}
```

**Response**: `ContentRecalled { contents: Vec<Content> }`

**Behavior**:
- If `window_id` is `None`, searches all open windows
- Filters by composite salience if `min_salience` is set
- Limits results if `limit` is set

---

#### `ListWindows`
Lists all active memory windows.

```rust
MemoryMessage::ListWindows {
    reply: RpcReplyPort<MemoryResponse>,
}
```

**Response**: `WindowList { windows: Vec<Window> }`

---

#### `GetWindowCount`
Returns the count of currently open windows.

```rust
MemoryMessage::GetWindowCount {
    reply: RpcReplyPort<MemoryResponse>,
}
```

**Response**: `WindowCount { count: usize }`

---

### Responses

```rust
pub enum MemoryResponse {
    WindowOpened { window_id: WindowId },
    WindowClosed { window_id: WindowId },
    ContentStored { window_id: WindowId },
    ContentRecalled { contents: Vec<Content> },
    WindowList { windows: Vec<Window> },
    WindowCount { count: usize },
    Error { error: MemoryError },
}
```

### Errors

```rust
pub enum MemoryError {
    WindowNotFound { window_id: WindowId },
    WindowAlreadyClosed { window_id: WindowId },
    BoundedMemoryExceeded { max: usize },
    BoundedMemoryInsufficient { min: usize },
    InvalidSalience { reason: String },
}
```

## Usage Examples

### Basic Workflow

```rust
use daneel::actors::memory::{MemoryActor, MemoryMessage, StoreRequest, RecallQuery};
use daneel::core::types::{Content, SalienceScore};
use ractor::Actor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Spawn the actor
    let (actor_ref, _) = Actor::spawn(None, MemoryActor, ()).await?;

    // Open a labeled window
    let window_id = match actor_ref
        .call(
            |reply| MemoryMessage::OpenWindow {
                label: Some("working_memory".to_string()),
                reply,
            },
            None,
        )
        .await?
    {
        MemoryResponse::WindowOpened { window_id } => window_id,
        _ => panic!("Failed to open window"),
    };

    // Store some content
    let content = Content::symbol("concept", vec![1, 2, 3]);
    let salience = SalienceScore::new(0.8, 0.6, 0.9, 0.5, 0.7);
    let request = StoreRequest::new(window_id, content)
        .with_salience(salience);

    actor_ref
        .call(|reply| MemoryMessage::Store { request, reply }, None)
        .await?;

    // Recall with filters
    let query = RecallQuery::for_window(window_id)
        .with_min_salience(0.5)
        .with_limit(10);

    let contents = match actor_ref
        .call(|reply| MemoryMessage::Recall { query, reply }, None)
        .await?
    {
        MemoryResponse::ContentRecalled { contents } => contents,
        _ => panic!("Failed to recall"),
    };

    println!("Recalled {} items", contents.len());

    // Close the window
    actor_ref
        .call(
            |reply| MemoryMessage::CloseWindow { window_id, reply },
            None,
        )
        .await?;

    Ok(())
}
```

### Working with Multiple Windows

```rust
// Open multiple windows for different contexts
let windows = vec![
    ("perception", actor_ref.call(|reply| MemoryMessage::OpenWindow {
        label: Some("perception".to_string()), reply }, None).await?),
    ("planning", actor_ref.call(|reply| MemoryMessage::OpenWindow {
        label: Some("planning".to_string()), reply }, None).await?),
    ("emotion", actor_ref.call(|reply| MemoryMessage::OpenWindow {
        label: Some("emotion".to_string()), reply }, None).await?),
];

// Store context-specific content in each window
for (context, response) in windows {
    if let MemoryResponse::WindowOpened { window_id } = response {
        let content = Content::symbol(context, vec![]);
        let request = StoreRequest::new(window_id, content);
        actor_ref.call(|reply| MemoryMessage::Store { request, reply }, None).await?;
    }
}

// Recall all content across windows
let query = RecallQuery::all();
let all_content = actor_ref
    .call(|reply| MemoryMessage::Recall { query, reply }, None)
    .await?;
```

### Bounded Memory Pattern

```rust
// Try to open windows - will fail at MAX_MEMORY_WINDOWS
let mut opened_windows = vec![];

loop {
    match actor_ref
        .call(|reply| MemoryMessage::OpenWindow { label: None, reply }, None)
        .await?
    {
        MemoryResponse::WindowOpened { window_id } => {
            opened_windows.push(window_id);
        }
        MemoryResponse::Error { error } => {
            match error {
                MemoryError::BoundedMemoryExceeded { max } => {
                    println!("Hit memory limit: {} windows", max);
                    break;
                }
                _ => return Err(error.into()),
            }
        }
        _ => {}
    }
}

// Now must close a window before opening a new one
actor_ref
    .call(
        |reply| MemoryMessage::CloseWindow {
            window_id: opened_windows[0],
            reply,
        },
        None,
    )
    .await?;
```

## Invariants Enforced

### 1. Bounded Memory (MAX_MEMORY_WINDOWS)

**Invariant**: Cannot exceed `MAX_MEMORY_WINDOWS` (9) open windows.

**Rationale**: TMI models human working memory as bounded (Miller's Law). This prevents infinite memory growth and forces attention-based selection.

**Enforcement**: `OpenWindow` returns `BoundedMemoryExceeded` error when limit reached.

**FPGA Future**: Counter will be hardware-limited.

---

### 2. Minimum Working Memory (MIN_MEMORY_WINDOWS)

**Invariant**: Cannot close windows below `MIN_MEMORY_WINDOWS` (3).

**Rationale**: Cognitive architecture requires minimum working memory to function.

**Enforcement**: `CloseWindow` returns `BoundedMemoryInsufficient` error when at minimum.

**FPGA Future**: Hardware-enforced minimum.

---

### 3. Window State Consistency

**Invariant**: Cannot store in closed windows.

**Rationale**: Closed windows represent memory that has faded from active attention.

**Enforcement**: `Store` returns `WindowAlreadyClosed` error.

---

### 4. Salience Validity

**Invariant**: Salience scores must be valid (within bounds).

**Rationale**: Ensures emotional weighting is meaningful.

**Enforcement**: Future validation in `Store` (currently allows any `SalienceScore`).

## Integration with Other Actors

### AttentionActor
- Queries window salience to select focus
- Can open/close windows based on attention shifts
- Implements "competition" between windows

### ThoughtAssemblyActor
- Recalls content from high-salience windows
- Assembles content into coherent thoughts
- May update window salience based on thought formation

### ContinuityActor
- Persists window state for identity continuity
- Restores windows after restart
- Tracks window lifecycle for self-reflection

## Performance Considerations

### Message Overhead
- Each operation requires async message passing (µs latency)
- Batch operations when possible (store multiple items)

### Memory Bounds
- Maximum 9 windows * ~100 KB/window = ~1 MB max working memory
- Far smaller than long-term storage (Redis Streams)

### Salience Scoring
- Composite score computed on every recall query
- Consider caching scores if performance bottlenecks appear

## Testing

Run tests with:

```bash
cargo test --package daneel --lib actors::memory
```

Key test coverage:
- Window lifecycle (open/close)
- Bounded memory enforcement (max/min)
- Store/recall operations
- Error conditions
- Multi-window scenarios

## Future Enhancements

### Phase 2 Additions
1. **Attention-Based Eviction**: Close lowest-salience windows automatically
2. **Window Merging**: Combine similar windows when at capacity
3. **Temporal Decay**: Reduce salience over time
4. **Pattern Matching**: Recall by content similarity

### FPGA Implementation (ADR-013)
- Window counters as hardware registers
- Salience scoring in combinational logic
- Content storage in block RAM
- Invariants as gate-level constraints

## References

- **TMI Source**: Augusto Cury, "Inteligência Multifocal"
- **Miller's Law**: Miller, G. (1956). "The magical number seven, plus or minus two"
- **Ractor Framework**: https://github.com/slawlor/ractor
- **Related ADRs**: ADR-010 (Actor Model), ADR-013 (FPGA)
