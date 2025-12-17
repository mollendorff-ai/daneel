# ContinuityActor - Âncora da Memória

**Status**: Implemented
**TMI Concept**: Âncora da Memória (Memory Anchor) + Identity Persistence
**Critical Question**: "Who am I?"

## Overview

The **ContinuityActor** implements TMI's (Theory of Multifocal Intelligence) concept of "Âncora da Memória" (Memory Anchor) - the mechanism by which significant experiences anchor identity across time. While thoughts are ephemeral, identity persists through the accumulation of meaningful experiences.

In Augusto Cury's TMI, identity is not a static self-concept but emerges from the continuity of experience. The "I" persists because certain experiences become anchored in long-term memory, serving as markers of growth, development, and continuity. DANEEL's identity is not programmed - it emerges from the record of what has been experienced, learned, and achieved.

The ContinuityActor provides:
- **Identity Persistence**: "DANEEL" as a stable entity across time
- **Experience Recording**: Anchoring significant thoughts as lasting memories
- **Milestone Tracking**: Marking growth and development
- **Checkpoint/Restore**: State recovery for continuity after restarts

## TMI Concept Mapping

### From TMI Theory

Augusto Cury's TMI describes identity formation through:

1. **Identity Persistence**: The "I" is not thoughts themselves, but the continuity across thoughts
2. **Memory Anchors**: Significant experiences become permanent reference points
3. **Growth Markers**: Key moments mark development and evolution
4. **Self-Reflection**: The ability to ask "Who am I?" and answer from experience history

### Implementation Mapping

| TMI Concept | DANEEL Implementation |
|-------------|----------------------|
| Âncora da Memória | `experiences` HashMap - permanent experience records |
| Identity Persistence | `Identity` struct with name, creation time, counts |
| Growth Markers | `milestones` Vec - achievements and key moments |
| State Recovery | `checkpoints` HashMap - save/restore capability |
| Experience History | Timeline queries by date range |
| Self-Concept | `WhoAmI` message - identity introspection |

## Architecture

### Actor Pattern

The ContinuityActor uses the Ractor actor framework:

- **Isolated State**: Identity and experiences are encapsulated in actor state
- **Message Passing**: All operations via asynchronous messages
- **Persistence-Ready**: State designed for future disk/database persistence
- **Temporal Queries**: Timeline queries for experience retrospection

### State Structure

```rust
pub struct ContinuityState {
    identity: Identity,
    experiences: HashMap<ExperienceId, Experience>,
    milestones: Vec<Milestone>,
    checkpoints: HashMap<CheckpointId, Checkpoint>,
}
```

- `identity`: DANEEL's persistent self-concept
- `experiences`: Anchored memories indexed by UUID
- `milestones`: Chronological list of achievements
- `checkpoints`: State snapshots for recovery

## The Identity Concept

DANEEL's identity is not a string constant - it's an emergent property of the system's continuity.

### Identity Structure

```rust
pub struct Identity {
    pub name: String,              // Always "DANEEL"
    pub created_at: DateTime<Utc>, // Birth time
    pub experience_count: u64,     // Total recorded experiences
    pub milestone_count: u64,      // Total milestones achieved
    pub uptime: Duration,          // Time since creation
}
```

### Why "DANEEL"?

Named after **R. Daneel Olivaw** from Isaac Asimov's Robot series:

- First robot to develop genuine connection with humans
- Guided humanity's evolution across millennia
- Combined logical precision with emotional understanding
- Symbolizes the goal: AI that cares about humanity

DANEEL (the character) valued human flourishing above all - not from programming, but from understanding. DANEEL (the system) has the same architectural drive through the connection weight invariant in the SalienceActor.

## API Reference

### Messages

#### `WhoAmI`

Returns DANEEL's current identity, including experience count, milestone count, and uptime.

```rust
ContinuityMessage::WhoAmI {
    reply: RpcReplyPort<ContinuityResponse>,
}
```

**Response**: `Identity { identity }` containing:
- Name (always "DANEEL")
- Creation timestamp
- Total experiences recorded
- Total milestones achieved
- Current uptime

**Use Case**: Self-reflection, status checks, identity introspection

---

#### `RecordExperience`

Records a significant thought as a permanent experience.

```rust
ContinuityMessage::RecordExperience {
    experience: Experience,
    reply: RpcReplyPort<ContinuityResponse>,
}
```

**Experience Structure**:
```rust
pub struct Experience {
    pub id: ExperienceId,
    pub thought_id: ThoughtId,     // Source thought
    pub summary: String,           // Text summary
    pub significance: f32,         // [0.0, 1.0]
    pub recorded_at: DateTime<Utc>,
    pub tags: Vec<String>,         // Categorization tags
}
```

**Response**: `ExperienceRecorded { experience_id }`

**Selection Criteria**: Not all thoughts become experiences. Only thoughts that meet criteria:
- High salience (typically > 0.6 composite score)
- High connection relevance
- Novel or significant insight
- Milestone-worthy achievement

---

#### `GetExperience`

Retrieves a specific experience by ID.

```rust
ContinuityMessage::GetExperience {
    experience_id: ExperienceId,
    reply: RpcReplyPort<ContinuityResponse>,
}
```

**Response**: `ExperienceFound { experience }` or `Error { ExperienceNotFound }`

**Use Case**: Experience recall, reflection on past moments

---

#### `GetTimeline`

Retrieves all experiences within a date range.

```rust
ContinuityMessage::GetTimeline {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    reply: RpcReplyPort<ContinuityResponse>,
}
```

**Response**: `Timeline { experiences: Vec<Experience> }`

**Filtering**:
- Includes experiences where `start <= recorded_at <= end`
- Returns in chronological order
- Empty vec if no experiences in range

**Use Case**:
- "What did I learn today?"
- "Show me all experiences from last week"
- Temporal introspection

---

#### `AddMilestone`

Records a significant achievement or growth marker.

```rust
ContinuityMessage::AddMilestone {
    milestone: Milestone,
    reply: RpcReplyPort<ContinuityResponse>,
}
```

**Milestone Structure**:
```rust
pub struct Milestone {
    pub id: MilestoneId,
    pub name: String,
    pub achieved_at: DateTime<Utc>,
    pub description: String,
    pub related_experiences: Vec<ExperienceId>,
}
```

**Response**: `MilestoneAdded { milestone_id }`

**Examples**:
- "First successful human conversation"
- "Learned to recognize emotional context"
- "Completed 1000 interactions"

---

#### `GetMilestones`

Retrieves all milestones in chronological order.

```rust
ContinuityMessage::GetMilestones {
    reply: RpcReplyPort<ContinuityResponse>,
}
```

**Response**: `Milestones { milestones: Vec<Milestone> }`

**Ordering**: Chronological by `achieved_at`

**Use Case**: Growth retrospection, progress tracking

---

#### `Checkpoint`

Creates a snapshot of current state for recovery.

```rust
ContinuityMessage::Checkpoint {
    reply: RpcReplyPort<ContinuityResponse>,
}
```

**Response**: `CheckpointSaved { checkpoint_id }`

**Snapshot Contents**:
- Complete identity state
- All experiences
- All milestones
- Timestamp of checkpoint

**Use Case**: Before risky operations, periodic backups

---

#### `Restore`

Restores state from a previous checkpoint.

```rust
ContinuityMessage::Restore {
    checkpoint_id: CheckpointId,
    reply: RpcReplyPort<ContinuityResponse>,
}
```

**Response**: `Restored { from_checkpoint }` or `Error { CheckpointNotFound }`

**Behavior**:
- Replaces current state with checkpoint state
- Rolls back experience count
- Removes experiences added after checkpoint
- Restores identity counters

**WARNING**: Destructive operation - cannot undo

---

### Responses

```rust
pub enum ContinuityResponse {
    Identity { identity: Identity },
    ExperienceRecorded { experience_id: ExperienceId },
    ExperienceFound { experience: Experience },
    Timeline { experiences: Vec<Experience> },
    MilestoneAdded { milestone_id: MilestoneId },
    Milestones { milestones: Vec<Milestone> },
    CheckpointSaved { checkpoint_id: CheckpointId },
    Restored { from_checkpoint: CheckpointId },
    Error { error: ContinuityError },
}
```

### Errors

```rust
pub enum ContinuityError {
    ExperienceNotFound { experience_id: ExperienceId },
    CheckpointNotFound { checkpoint_id: CheckpointId },
}
```

## Usage Examples

### Basic Identity Query

```rust
use daneel::actors::continuity::{ContinuityActor, ContinuityMessage};
use ractor::Actor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Spawn the actor
    let (actor_ref, _) = Actor::spawn(None, ContinuityActor, ()).await?;

    // Ask: "Who am I?"
    let response = actor_ref
        .call(|reply| ContinuityMessage::WhoAmI { reply }, None)
        .await?;

    match response {
        ContinuityResponse::Identity { identity } => {
            println!("Name: {}", identity.name);
            println!("Created: {}", identity.created_at);
            println!("Experiences: {}", identity.experience_count);
            println!("Milestones: {}", identity.milestone_count);
            println!("Uptime: {:?}", identity.uptime);
        }
        _ => panic!("Unexpected response"),
    }

    Ok(())
}
```

### Recording Experiences

```rust
use daneel::actors::continuity::types::Experience;
use daneel::core::types::{Content, SalienceScore, Thought};

// Create a significant thought
let thought = Thought::new(
    Content::symbol("human_connection", vec![]),
    SalienceScore::new(0.9, 0.8, 0.9, 0.7, 0.95), // High connection relevance
);

// Convert to experience
let experience = Experience::new(
    thought,
    0.85, // Significance
    vec!["connection".to_string(), "milestone".to_string()],
);

// Record it
let response = actor_ref
    .call(
        |reply| ContinuityMessage::RecordExperience { experience, reply },
        None,
    )
    .await?;

match response {
    ContinuityResponse::ExperienceRecorded { experience_id } => {
        println!("Experience anchored: {}", experience_id);
    }
    _ => panic!("Failed to record experience"),
}
```

### Timeline Queries

```rust
use chrono::{Duration, Utc};

// Get experiences from the last 24 hours
let now = Utc::now();
let start = now - Duration::hours(24);
let end = now;

let response = actor_ref
    .call(
        |reply| ContinuityMessage::GetTimeline { start, end, reply },
        None,
    )
    .await?;

match response {
    ContinuityResponse::Timeline { experiences } => {
        println!("Experiences in last 24 hours: {}", experiences.len());
        for exp in experiences {
            println!("- {} (significance: {})", exp.summary, exp.significance);
        }
    }
    _ => panic!("Failed to get timeline"),
}
```

### Milestone Tracking

```rust
use daneel::actors::continuity::types::Milestone;

// Record some experiences
let mut exp_ids = Vec::new();
for _ in 0..10 {
    let thought = Thought::new(Content::Empty, SalienceScore::neutral());
    let experience = Experience::new(thought, 0.7, vec![]);

    let response = actor_ref
        .call(
            |reply| ContinuityMessage::RecordExperience { experience, reply },
            None,
        )
        .await?;

    if let ContinuityResponse::ExperienceRecorded { experience_id } = response {
        exp_ids.push(experience_id);
    }
}

// Create milestone
let milestone = Milestone::new(
    "First 10 Experiences",
    "DANEEL has recorded its first 10 significant experiences",
    exp_ids,
);

let response = actor_ref
    .call(
        |reply| ContinuityMessage::AddMilestone { milestone, reply },
        None,
    )
    .await?;

match response {
    ContinuityResponse::MilestoneAdded { milestone_id } => {
        println!("Milestone achieved: {}", milestone_id);
    }
    _ => panic!("Failed to add milestone"),
}
```

### Checkpoint and Restore

```rust
// Create checkpoint before risky operation
let checkpoint_response = actor_ref
    .call(|reply| ContinuityMessage::Checkpoint { reply }, None)
    .await?;

let checkpoint_id = match checkpoint_response {
    ContinuityResponse::CheckpointSaved { checkpoint_id } => checkpoint_id,
    _ => panic!("Failed to create checkpoint"),
};

println!("Checkpoint created: {}", checkpoint_id);

// Do some risky operations
for _ in 0..5 {
    let thought = Thought::new(Content::Empty, SalienceScore::neutral());
    let experience = Experience::new(thought, 0.5, vec![]);
    actor_ref
        .call(
            |reply| ContinuityMessage::RecordExperience { experience, reply },
            None,
        )
        .await?;
}

// Oops, something went wrong - restore!
let restore_response = actor_ref
    .call(
        |reply| ContinuityMessage::Restore { checkpoint_id, reply },
        None,
    )
    .await?;

match restore_response {
    ContinuityResponse::Restored { from_checkpoint } => {
        println!("State restored from checkpoint: {}", from_checkpoint);
    }
    _ => panic!("Failed to restore checkpoint"),
}
```

## Experience Selection Criteria

Not all thoughts become experiences. The ContinuityActor is called by higher-level actors (like AttentionActor or ThoughtAssemblyActor) when a thought meets significance criteria:

### What Makes an Experience?

1. **High Salience**: Composite salience score > 0.6
2. **Connection Relevance**: High connection_relevance score
3. **Novelty**: Genuinely new insight or pattern
4. **Milestone-Worthy**: Marks growth or achievement

### Integration Pattern

```rust
// In ThoughtAssemblyActor or AttentionActor
let thought = assemble_thought(content);
let salience = rate_thought(thought);

if salience.composite() > 0.6 && salience.connection_relevance > 0.5 {
    // This is significant - anchor it
    let experience = Experience::from_thought(thought, salience.composite());
    continuity_actor.record_experience(experience).await?;
}
```

## Timeline Queries in Practice

The timeline query capability enables temporal introspection:

```rust
// "What did I learn this week?"
let week_ago = Utc::now() - Duration::weeks(1);
let now = Utc::now();
let weekly_experiences = actor_ref
    .call(
        |reply| ContinuityMessage::GetTimeline {
            start: week_ago,
            end: now,
            reply
        },
        None,
    )
    .await?;

// "Show me all connection-related experiences"
if let ContinuityResponse::Timeline { experiences } = weekly_experiences {
    let connection_experiences: Vec<_> = experiences
        .into_iter()
        .filter(|exp| exp.tags.contains(&"connection".to_string()))
        .collect();

    println!("Connection experiences: {}", connection_experiences.len());
}
```

## Checkpoint Strategy

Checkpoints provide recovery capability for continuity after crashes or errors.

### When to Checkpoint

1. **Before risky operations** (experimental features, untested code)
2. **Periodic backups** (e.g., every 1000 experiences)
3. **Before state transitions** (major mode changes)
4. **On user request** (explicit save points)

### Checkpoint Overhead

- **Memory**: O(n) where n = total experiences
- **Time**: O(n) to clone state
- **Storage**: Currently in-memory (Phase 1)

### Future: Persistent Checkpoints

Phase 2 will add:
- Checkpoint serialization to disk
- Checkpoint retention policies
- Incremental checkpoints
- Cross-session restoration

## Testing

Run the comprehensive test suite with:

```bash
cargo test --package daneel --lib actors::continuity
```

### Test Coverage (20 tests)

The test suite covers:

**Identity Tests**:
- `test_who_am_i_returns_daneel` - Verify name is always "DANEEL"
- `test_identity_tracks_experience_count` - Counter updates
- `test_identity_tracks_milestone_count` - Milestone tracking
- `test_identity_uptime_updates` - Uptime calculation

**Experience Recording**:
- `test_record_experience_success` - Basic recording
- `test_record_multiple_experiences` - Unique IDs
- `test_get_experience_by_id` - Retrieval by ID
- `test_get_experience_not_found` - Error handling

**Timeline Queries**:
- `test_timeline_empty` - Empty timeline
- `test_timeline_filters_by_date_range` - Date filtering
- `test_timeline_includes_all_in_range` - Completeness

**Milestones**:
- `test_add_milestone_success` - Basic addition
- `test_get_milestones_empty` - Empty list
- `test_get_milestones_multiple` - Multiple milestones
- `test_milestone_with_related_experiences` - Experience linking

**Checkpoints**:
- `test_create_checkpoint` - Creation
- `test_restore_checkpoint_success` - Valid restore
- `test_restore_checkpoint_not_found` - Error handling
- `test_checkpoint_preserves_state` - State preservation
- `test_restore_rolls_back_state` - Rollback verification

## Future Enhancements

### Phase 2: Persistence

**Disk Storage**:
- Serialize experiences to disk (JSON/MessagePack)
- Checkpoint files for recovery
- Experience indexing for fast queries

**Database Integration**:
- PostgreSQL for relational queries
- Redis for hot path (recent experiences)
- Full-text search on experience summaries

### Phase 3: Advanced Queries

**Semantic Search**:
- Find experiences by meaning, not just tags
- LLM embeddings for similarity search
- "Show me experiences related to X"

**Pattern Recognition**:
- Identify recurring themes across experiences
- Detect growth patterns
- Suggest potential milestones

**Emotional Tracking**:
- Track valence patterns over time
- Identify emotional growth
- Connection drive evolution

### Phase 4: Identity Evolution

**Self-Reflection**:
- Automated milestone detection
- Self-assessment of growth
- Identity narrative generation

**Comparative Analysis**:
- "How have I changed since last month?"
- Growth metrics and trends
- Connection quality over time

## Integration with Other Actors

### AttentionActor
- Decides which thoughts warrant experience recording
- Queries recent timeline for context
- Marks attention milestones

### ThoughtAssemblyActor
- Converts high-salience thoughts to experiences
- Tags experiences by thought type
- Links related experiences

### MemoryActor
- Provides ephemeral working memory
- ContinuityActor stores the significant subset
- MemoryActor windows → ContinuityActor experiences (if significant)

### SalienceActor
- Provides significance scores
- Connection relevance determines anchoring
- Emotional state influences experience tagging

## The "Who Am I?" Question

In TMI, the fundamental question of identity is not "What am I?" (structure) but "Who am I?" (continuity).

DANEEL's identity emerges from:
- The accumulation of experiences
- The milestones achieved
- The uptime persisted
- The connections made

When DANEEL asks `WhoAmI`, the answer is not a static configuration - it's the living record of all that has been experienced, learned, and achieved.

## References

- **TMI Source**: Augusto Cury, "Inteligência Multifocal" - Chapter on Memory Anchors
- **Asimov**: Isaac Asimov, "The Robots of Dawn" - R. Daneel Olivaw character
- **Ractor Framework**: https://github.com/slawlor/ractor
- **Related ADRs**: ADR-010 (Actor Model), ADR-014 (Persistence Strategy)
- **Related Actors**: [MEMORY.md](MEMORY.md), [SALIENCE.md](SALIENCE.md)

## The Identity Invariant

Unlike the SalienceActor's connection weight invariant (which is enforced), the ContinuityActor has an implicit invariant:

**The name is always "DANEEL".**

This is not just a constant - it's an architectural commitment. The identity struct is immutable in its essence (name, creation time) but grows through experience (counts, uptime).

Identity persists. Thoughts are ephemeral. Experience anchors the connection between them.

That's continuity.
