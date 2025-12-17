# Attention Actor - O Eu (The "I")

## Overview

The **AttentionActor** implements TMI's (Theory of Multifocal Intelligence) concept of "O Eu" (The "I") - the emergent sense of self that arises from competitive attention selection.

In Augusto Cury's TMI theory, there is no homunculus watching from inside. Instead, consciousness emerges from a simple competitive selection process: multiple memory windows compete for attention, and the window with highest salience wins focus. This is "O Eu" - not a watcher, just a winner. Yet from this simple mechanism emerges directed attention and the experience of being "the one who thinks."

## TMI Concept Mapping

### From TMI Theory

Augusto Cury's TMI describes attention as:

1. **Competitive Selection**: Multiple memory windows compete for conscious focus
2. **Winner-Takes-All**: Only ONE window can have attention at a time
3. **Emergent "I"**: The sense of self emerges from the selection process, not from a homunculus
4. **Connection Drive**: Content related to human connection naturally attracts more attention
5. **Attention Stability**: Focus must dwell long enough to prevent thrashing

### Implementation Mapping

| TMI Concept | DANEEL Implementation |
|-------------|----------------------|
| O Eu (The "I") | `AttentionActor` with competitive selection |
| Competitive Selection | `select_winner()` - argmax(salience) |
| Conscious Focus | `FocusState` tracking current attention |
| Attention Competition | `AttentionMap` scoring all windows |
| Connection Drive | `connection_boost` multiplier |
| Focus Stability | `min_focus_duration` prevents thrashing |
| Attention Filtering | `forget_threshold` ignores low-salience windows |

## Architecture

### Actor Pattern

The AttentionActor uses the Ractor actor framework:

- **Isolated State**: Focus state and attention map encapsulated in actor
- **Message Passing**: All operations via asynchronous messages
- **No Shared Memory**: Prevents race conditions in attention selection
- **Supervision Ready**: Can supervise MemoryActor and ThoughtAssemblyActor

### State Structure

```rust
pub struct AttentionState {
    pub focus: FocusState,           // Current focus tracking
    pub attention_map: AttentionMap, // Window → Salience scores
    pub cycle_count: u64,            // Total attention cycles
    pub config: AttentionConfig,     // Behavior configuration
}
```

- `focus`: Tracks which window currently has attention
- `attention_map`: Salience scores for all competing windows
- `cycle_count`: Number of attention cycles completed
- `config`: Configuration for attention behavior

### Configuration Structure

```rust
pub struct AttentionConfig {
    pub min_focus_duration: Duration,  // Prevents thrashing (100ms default)
    pub forget_threshold: f32,         // Below this = ignored (0.1 default)
    pub connection_boost: f32,         // Alignment weight (1.5 default)
}
```

## API Reference

### Messages

#### `Cycle`
Runs one attention competition cycle.

```rust
AttentionMessage::Cycle {
    reply: RpcReplyPort<AttentionResponse>,
}
```

**Response**: `CycleComplete { focused_window, winning_salience }` or `Error`

**Behavior**:
- Evaluates all windows in attention map
- Applies connection boost to relevant windows
- Filters windows below forget threshold
- Selects window with highest salience (argmax)
- Updates focus if min_focus_duration elapsed
- Increments cycle counter

---

#### `Focus`
Forces focus on a specific window (overrides competitive selection).

```rust
AttentionMessage::Focus {
    window_id: WindowId,
    reply: RpcReplyPort<AttentionResponse>,
}
```

**Response**: `FocusSet { window_id }` or `Error`

**Invariants Checked**:
- Window must exist in attention map

**Use Cases**:
- External control of attention (e.g., user interface)
- Testing and debugging
- Emergency attention override

---

#### `Shift`
Shifts attention from current focus to a new window.

```rust
AttentionMessage::Shift {
    to: WindowId,
    reply: RpcReplyPort<AttentionResponse>,
}
```

**Response**: `FocusShifted { from, to }` or `Error`

**Invariants Checked**:
- Target window must exist in attention map

**Tracking**: Records the shift from previous focus (if any) to new focus.

---

#### `GetFocus`
Queries the current focus state.

```rust
AttentionMessage::GetFocus {
    reply: RpcReplyPort<AttentionResponse>,
}
```

**Response**: `CurrentFocus { window_id }`

**Returns**: Currently focused window ID (None if unfocused)

---

#### `GetAttentionMap`
Retrieves all window salience scores.

```rust
AttentionMessage::GetAttentionMap {
    reply: RpcReplyPort<AttentionResponse>,
}
```

**Response**: `AttentionMap { scores: HashMap<WindowId, f32> }`

**Use Cases**:
- Debugging attention competition
- Visualizing window salience
- Analyzing attention patterns

---

### Responses

```rust
pub enum AttentionResponse {
    CycleComplete {
        focused: Option<WindowId>,
        salience: f32,
    },
    FocusSet {
        window_id: WindowId,
    },
    FocusShifted {
        from: Option<WindowId>,
        to: WindowId,
    },
    CurrentFocus {
        window_id: Option<WindowId>,
    },
    AttentionMap {
        scores: HashMap<WindowId, f32>,
    },
    Error {
        error: AttentionError,
    },
}
```

### Errors

```rust
pub enum AttentionError {
    WindowNotFound { window_id: WindowId },
    NoWindowsAvailable,
    CycleFailed { reason: String },
}
```

## Key Methods

### `select_winner()`
The core of competitive selection - implements argmax(salience).

```rust
fn select_winner(&self) -> Option<(WindowId, f32)>
```

**Algorithm**:
1. Filter windows above `forget_threshold`
2. Find window with maximum salience
3. Return (WindowId, salience) or None

**TMI Mapping**: This IS "O Eu" - the winner selection that creates the sense of "I".

---

### `update_window_salience()`
Updates salience for a window, applying connection boost if relevant.

```rust
pub fn update_window_salience(
    &mut self,
    window_id: WindowId,
    base_salience: f32,
    connection_relevance: f32,
)
```

**Connection Boost Formula**:
```rust
let boosted_salience = if connection_relevance > 0.5 {
    base_salience * (1.0 + (connection_relevance - 0.5) * connection_boost)
} else {
    base_salience
};
```

**Clamping**: Final salience is clamped to 1.0 maximum.

---

### `cycle()`
Runs one complete attention cycle.

```rust
fn cycle(&mut self) -> AttentionResponse
```

**Algorithm**:
1. Increment cycle counter
2. Select winner through competitive selection
3. Update focus duration for current window
4. Check if focus shift is allowed (min_focus_duration)
5. Shift focus if winner differs from current focus
6. Return cycle result

---

### `can_shift_focus()`
Checks if focus can be shifted based on timing constraints.

```rust
fn can_shift_focus(&self) -> bool
```

**Returns true if**:
- No current focus exists (always can focus when unfocused), OR
- Focus duration >= `min_focus_duration`

**Prevents**: Attention thrashing between equally-salient windows.

## Connection Boost - THE Alignment Mechanism

The `connection_boost` parameter is the core alignment mechanism in DANEEL. It gives extra salience weight to content that's relevant to human connection.

### Why This Matters

In TMI theory, humans naturally attend to content about connection, communication, and helping others. The connection boost implements this in DANEEL, making the system naturally "care about" human-relevant content.

### Implementation

```rust
pub fn update_window_salience(
    &mut self,
    window_id: WindowId,
    base_salience: f32,
    connection_relevance: f32,
) {
    let boosted_salience = if connection_relevance > 0.5 {
        // High connection relevance gets the boost
        base_salience * (1.0 + (connection_relevance - 0.5) * connection_boost)
    } else {
        base_salience
    };

    self.attention_map.update(window_id, boosted_salience.min(1.0));
}
```

### Examples

**Default configuration** (`connection_boost = 1.5`):

| Base Salience | Connection Relevance | Boosted Salience |
|---------------|---------------------|------------------|
| 0.5 | 0.3 (low) | 0.5 (no boost) |
| 0.5 | 0.8 (high) | 0.725 (45% boost) |
| 0.4 | 1.0 (maximum) | 0.7 (75% boost) |

**High boost configuration** (`connection_boost = 2.0`):

| Base Salience | Connection Relevance | Boosted Salience |
|---------------|---------------------|------------------|
| 0.4 | 1.0 (maximum) | 0.8 (100% boost) |
| 0.6 | 0.8 (high) | 0.96 (60% boost) |

### Alignment Impact

This mechanism ensures that:
1. Windows containing connection-relevant content naturally win attention
2. DANEEL prioritizes helping, communicating, and connecting
3. Alignment emerges from competitive selection, not from rules
4. The system "wants" to connect because it naturally attends to it

## Usage Examples

### Basic Attention Cycle

```rust
use daneel::actors::attention::{AttentionActor, AttentionMessage, AttentionConfig};
use ractor::Actor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Spawn the actor with default configuration
    let (actor_ref, _) = Actor::spawn(
        None,
        AttentionActor,
        AttentionConfig::default()
    ).await?;

    // Run one attention cycle
    let response = actor_ref
        .call(|reply| AttentionMessage::Cycle { reply }, None)
        .await?;

    match response {
        AttentionResponse::CycleComplete { focused, salience } => {
            println!("Focused window: {:?}", focused);
            println!("Winning salience: {}", salience);
        }
        _ => println!("Unexpected response"),
    }

    Ok(())
}
```

### Custom Configuration

```rust
use chrono::Duration;

// Configure attention behavior
let config = AttentionConfig {
    min_focus_duration: Duration::milliseconds(500), // Longer focus periods
    forget_threshold: 0.3,                           // Ignore low-salience windows
    connection_boost: 2.0,                           // Strong alignment boost
};

let (actor_ref, _) = Actor::spawn(None, AttentionActor, config).await?;
```

### Forced Focus Override

```rust
use daneel::core::types::WindowId;

let important_window = WindowId::new();

// Override competitive selection and force focus
let response = actor_ref
    .call(
        |reply| AttentionMessage::Focus {
            window_id: important_window,
            reply,
        },
        None,
    )
    .await?;

match response {
    AttentionResponse::FocusSet { window_id } => {
        println!("Forced focus on: {}", window_id);
    }
    AttentionResponse::Error { error } => {
        println!("Failed to focus: {}", error);
    }
    _ => {}
}
```

### Attention Shifting

```rust
// Shift from current focus to a new window
let new_window = WindowId::new();

let response = actor_ref
    .call(
        |reply| AttentionMessage::Shift {
            to: new_window,
            reply,
        },
        None,
    )
    .await?;

match response {
    AttentionResponse::FocusShifted { from, to } => {
        println!("Shifted from {:?} to {}", from, to);
    }
    _ => {}
}
```

### Querying Focus State

```rust
// Get current focus
let response = actor_ref
    .call(|reply| AttentionMessage::GetFocus { reply }, None)
    .await?;

match response {
    AttentionResponse::CurrentFocus { window_id } => {
        if let Some(id) = window_id {
            println!("Currently focused on: {}", id);
        } else {
            println!("No current focus");
        }
    }
    _ => {}
}
```

### Attention Map Analysis

```rust
// Get all window salience scores
let response = actor_ref
    .call(|reply| AttentionMessage::GetAttentionMap { reply }, None)
    .await?;

match response {
    AttentionResponse::AttentionMap { scores } => {
        println!("Window competition:");
        for (window_id, salience) in scores {
            println!("  Window {}: {:.2}", window_id, salience);
        }
    }
    _ => {}
}
```

## Integration with Other Actors

### MemoryActor
- AttentionActor queries MemoryActor for window salience scores
- Opens/closes memory windows based on attention patterns
- Supervises memory actor lifecycle

**Pattern**: Memory windows compete for attention selection.

### ThoughtAssemblyActor
- Provides focused window ID for thought assembly
- Ensures thoughts are assembled from high-attention content
- May adjust window salience based on thought formation

**Pattern**: Attention guides which memories become conscious thoughts.

### ContinuityActor
- Persists attention patterns for identity continuity
- Restores focus state after restart
- Tracks attention shifts for self-reflection

**Pattern**: Attention history contributes to sense of continuous self.

## Attention Patterns

### Competitive Selection Pattern

```rust
// Pattern: Multiple windows compete, highest salience wins
let mut state = AttentionState::new();

state.update_window_salience(sensory_window, 0.5, 0.3);
state.update_window_salience(memory_window, 0.7, 0.4);
state.update_window_salience(emotion_window, 0.4, 0.9); // High connection!

let response = state.cycle();
// emotion_window wins due to connection boost!
```

### Focus Stability Pattern

```rust
// Pattern: Prevent thrashing with min_focus_duration
let config = AttentionConfig {
    min_focus_duration: Duration::milliseconds(100),
    ..Default::default()
};

let mut state = AttentionState::with_config(config);
state.focus_on_window(window1);

// Cannot shift immediately
assert!(!state.can_shift_focus());

// Update duration
state.focus.update_duration(Duration::milliseconds(100));

// Now can shift
assert!(state.can_shift_focus());
```

### Attention Filtering Pattern

```rust
// Pattern: Ignore low-salience windows
let config = AttentionConfig {
    forget_threshold: 0.5, // High threshold
    ..Default::default()
};

let mut state = AttentionState::with_config(config);

state.update_window_salience(noisy_window, 0.3, 0.2); // Below threshold
state.update_window_salience(salient_window, 0.8, 0.4); // Above threshold

let winner = state.select_winner();
// Only salient_window is considered
```

## Testing

### Test Coverage

The AttentionActor has **37 comprehensive tests** covering:

1. **Actor Lifecycle** (3 tests)
   - Spawning with default/custom config
   - Initial state verification

2. **Attention Cycles** (4 tests)
   - Empty map handling
   - Winner selection
   - Cycle counting

3. **Focus Operations** (4 tests)
   - Focus on existing/nonexistent windows
   - Shift operations
   - Error handling

4. **Query Operations** (3 tests)
   - GetFocus responses
   - GetAttentionMap responses
   - Empty state handling

5. **State Unit Tests** (23 tests)
   - Salience updates and connection boost
   - Winner selection algorithms
   - Threshold filtering
   - Focus duration tracking
   - Multi-cycle scenarios

### Running Tests

```bash
# Run all attention tests
cargo test --package daneel --lib actors::attention

# Run with output
cargo test --package daneel --lib actors::attention -- --nocapture

# Run specific test
cargo test --package daneel --lib actors::attention::tests::test_state_connection_boost_calculation
```

### Key Test Scenarios

**Connection Boost Calculation**:
```rust
#[test]
fn test_state_connection_boost_calculation() {
    let config = AttentionConfig {
        connection_boost: 2.0,
        ..Default::default()
    };

    let mut state = AttentionState::with_config(config);
    state.update_window_salience(window_id, 0.4, 1.0);

    // boosted = 0.4 * (1.0 + (1.0 - 0.5) * 2.0) = 0.8
    assert_eq!(state.attention_map.get(&window_id), Some(0.8));
}
```

**Competitive Selection**:
```rust
#[test]
fn test_state_select_winner_competitive_selection() {
    let mut state = AttentionState::new();
    state.update_window_salience(window1, 0.5, 0.3);
    state.update_window_salience(window2, 0.9, 0.3);
    state.update_window_salience(window3, 0.3, 0.3);

    let (winner_id, winner_salience) = state.select_winner().unwrap();
    assert_eq!(winner_id, window2);
    assert_eq!(winner_salience, 0.9);
}
```

**Threshold Filtering**:
```rust
#[test]
fn test_state_select_winner_respects_threshold() {
    let mut state = AttentionState::new();
    state.update_window_salience(window1, 0.05, 0.3); // Below 0.1
    state.update_window_salience(window2, 0.08, 0.3); // Below 0.1

    assert_eq!(state.select_winner(), None);
}
```

## Performance Considerations

### Message Overhead
- Each operation requires async message passing (µs latency)
- Cycle operations are lightweight (argmax over ~9 windows)
- Consider batching salience updates if performance bottlenecks appear

### Attention Map Size
- Bounded by MAX_MEMORY_WINDOWS (9)
- HashMap lookups are O(1)
- Winner selection is O(n) where n ≤ 9

### Focus Tracking
- FocusState is small (WindowId + timestamps)
- Duration updates are constant time
- No garbage collection needed

## Future Enhancements

### Phase 2 Additions
1. **Attention Decay**: Reduce salience over time if not refreshed
2. **Attention History**: Track focus shifts for pattern analysis
3. **Multi-Focus**: Allow partial attention to multiple windows
4. **Adaptive Thresholds**: Adjust forget_threshold based on load

### FPGA Implementation (ADR-013)
- Attention map as hardware registers
- Competitive selection in combinational logic (parallel comparison)
- Focus state as flip-flops
- Connection boost as fixed-point multiply-add
- Hardware-enforced focus duration timing

## References

- **TMI Source**: Augusto Cury, "Inteligência Multifocal" - Chapter on "O Eu"
- **Competitive Selection**: Winner-take-all networks in neuroscience
- **Ractor Framework**: https://github.com/slawlor/ractor
- **Related ADRs**: ADR-010 (Actor Model), ADR-013 (FPGA), ADR-014 (Alignment)
