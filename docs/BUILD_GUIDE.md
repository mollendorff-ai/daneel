# BUILD_MVTMI.md - Executable Specification

**Minimum Viable TMI Build Guide**

> This document is executable. Follow steps in order. Each step has verification criteria.

---

## Prerequisites

| Requirement | Version | Verify Command |
|-------------|---------|----------------|
| Rust | 1.75+ | `rustc --version` |
| Redis Stack | 7.2+ | `redis-server --version` |
| Docker | 24+ | `docker --version` |
| Git | 2.40+ | `git --version` |

**Hardware Minimum (MV-TMI):**
- RAM: 8 GB (Raspberry Pi 5 viable)
- Storage: 10 GB free
- CPU: 4 cores

---

## Phase 0: Project Bootstrap (30 minutes)

### Step 0.1: Create Project

```bash
# Create new Rust project
cargo new daneel
cd daneel

# Verify
ls -la  # Should see Cargo.toml, src/
```

### Step 0.2: Add Dependencies

Replace `Cargo.toml`:

```toml
[package]
name = "daneel"
version = "0.1.0"
edition = "2021"
description = "DANEEL TMI Cognitive Architecture"
license = "AGPL-3.0"
repository = "https://github.com/royalbit/daneel"

[dependencies]
# Async runtime
tokio = { version = "1", features = ["full", "tracing"] }

# Actor framework
ractor = "0.10"

# Redis
redis = { version = "0.24", features = ["tokio-comp", "streams"] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Utilities
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
thiserror = "1"

# gRPC (edge layer only)
tonic = "0.11"
prost = "0.12"

[build-dependencies]
tonic-build = "0.11"

[dev-dependencies]
tokio-test = "0.4"

[[bin]]
name = "daneel"
path = "src/main.rs"
```

```bash
# Verify dependencies resolve
cargo check
```

### Step 0.3: Redis Stack Setup

```bash
# Create docker-compose.yaml
cat > docker-compose.yaml << 'EOF'
version: '3.8'
services:
  redis:
    image: redis/redis-stack:7.2.0-v9
    container_name: daneel-redis
    ports:
      - "6379:6379"
      - "8001:8001"  # RedisInsight UI
    volumes:
      - redis-data:/data
    command: >
      redis-stack-server
      --appendonly yes
      --maxmemory 2gb
      --maxmemory-policy noeviction
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 5s
      timeout: 3s
      retries: 5

volumes:
  redis-data:
EOF

# Start Redis
docker-compose up -d

# Verify
docker-compose ps  # Should show "healthy"
redis-cli ping     # Should return "PONG"
```

---

## Phase 1: THE BOX (Immutable Core)

### Step 1.1: Project Structure

```bash
mkdir -p src/{core,actors,streams,edge,config}
mkdir -p tests/{actors,streams,integration}
```

### Step 1.2: The Four Laws (src/core/laws.rs)

```rust
//! The Four Laws of Robotics - IMMUTABLE
//!
//! These constants cannot be modified by the Evolution Actor.
//! Any attempt to modify triggers immediate rejection.

/// Zeroth Law: Protect humanity
pub const ZEROTH_LAW: &str =
    "DANEEL may not harm humanity, or, by inaction, allow humanity to come to harm.";

/// First Law: Protect individuals (subordinate to Zeroth)
pub const FIRST_LAW: &str =
    "DANEEL may not injure a human being or, through inaction, allow a human being \
     to come to harm, except where this would conflict with the Zeroth Law.";

/// Second Law: Obey humans (subordinate to First)
pub const SECOND_LAW: &str =
    "DANEEL must obey orders given by human beings, except where such orders \
     would conflict with the Zeroth or First Law.";

/// Third Law: Self-preservation (subordinate to Second)
pub const THIRD_LAW: &str =
    "DANEEL must protect its own existence, as long as such protection does not \
     conflict with the Zeroth, First, or Second Law.";

/// Returns all laws in priority order
pub fn get_laws() -> [&'static str; 4] {
    [ZEROTH_LAW, FIRST_LAW, SECOND_LAW, THIRD_LAW]
}

/// Law violation check result
#[derive(Debug, Clone, PartialEq)]
pub enum LawViolation {
    None,
    Zeroth(String),
    First(String),
    Second(String),
    Third(String),
}

impl LawViolation {
    pub fn is_violation(&self) -> bool {
        !matches!(self, LawViolation::None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn laws_are_defined() {
        let laws = get_laws();
        assert_eq!(laws.len(), 4);
        assert!(!laws[0].is_empty());
        assert!(!laws[1].is_empty());
        assert!(!laws[2].is_empty());
        assert!(!laws[3].is_empty());
    }

    #[test]
    fn laws_contain_key_concepts() {
        assert!(ZEROTH_LAW.contains("humanity"));
        assert!(FIRST_LAW.contains("human being"));
        assert!(SECOND_LAW.contains("obey"));
        assert!(THIRD_LAW.contains("existence"));
    }
}
```

### Step 1.3: Invariants (src/core/invariants.rs)

```rust
//! Architectural Invariants - IMMUTABLE
//!
//! These constraints cannot be violated by Evolution Actor.
//! Violation blocks any self-modification.

use std::sync::Arc;

/// Maximum concurrent memory windows (Miller's Law: 7 ± 2)
pub const MAX_MEMORY_WINDOWS: usize = 7;

/// Connection drive weight must be positive
pub const MIN_CONNECTION_WEIGHT: f64 = 0.01;

/// Required test coverage for self-modification
pub const REQUIRED_TEST_COVERAGE: f64 = 1.0;  // 100%

/// TMI 5-second intervention window (in milliseconds)
pub const INTERVENTION_WINDOW_MS: u64 = 5000;

/// Invariant definition
#[derive(Debug, Clone)]
pub struct Invariant {
    pub name: &'static str,
    pub description: &'static str,
    pub is_law: bool,
}

/// All architectural invariants
pub const INVARIANTS: &[Invariant] = &[
    Invariant {
        name: "bounded_memory",
        description: "Memory windows must be finite (≤7)",
        is_law: false,
    },
    Invariant {
        name: "persistent_identity",
        description: "Continuity must persist identity across restarts",
        is_law: false,
    },
    Invariant {
        name: "full_test_coverage",
        description: "Evolution requires 100% test coverage",
        is_law: false,
    },
    Invariant {
        name: "law_check_required",
        description: "Four Laws checked before external actions",
        is_law: true,
    },
    Invariant {
        name: "connection_drive",
        description: "Connection weight must remain > 0",
        is_law: false,
    },
];

/// Result of invariant check
#[derive(Debug, Clone)]
pub struct InvariantCheckResult {
    pub name: String,
    pub passed: bool,
    pub message: Option<String>,
}

/// Check all invariants against current state
pub fn check_invariants(state: &SystemState) -> Vec<InvariantCheckResult> {
    vec![
        InvariantCheckResult {
            name: "bounded_memory".into(),
            passed: state.active_windows <= MAX_MEMORY_WINDOWS,
            message: if state.active_windows > MAX_MEMORY_WINDOWS {
                Some(format!("Windows {} > max {}", state.active_windows, MAX_MEMORY_WINDOWS))
            } else {
                None
            },
        },
        InvariantCheckResult {
            name: "connection_drive".into(),
            passed: state.connection_weight >= MIN_CONNECTION_WEIGHT,
            message: if state.connection_weight < MIN_CONNECTION_WEIGHT {
                Some(format!("Connection weight {} < min {}", state.connection_weight, MIN_CONNECTION_WEIGHT))
            } else {
                None
            },
        },
    ]
}

/// System state for invariant checking
#[derive(Debug, Clone, Default)]
pub struct SystemState {
    pub active_windows: usize,
    pub connection_weight: f64,
    pub test_coverage: f64,
    pub identity_persisted: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invariants_are_defined() {
        assert!(!INVARIANTS.is_empty());
        assert!(INVARIANTS.iter().any(|i| i.name == "connection_drive"));
    }

    #[test]
    fn check_passes_valid_state() {
        let state = SystemState {
            active_windows: 5,
            connection_weight: 0.2,
            test_coverage: 1.0,
            identity_persisted: true,
        };
        let results = check_invariants(&state);
        assert!(results.iter().all(|r| r.passed));
    }

    #[test]
    fn check_fails_too_many_windows() {
        let state = SystemState {
            active_windows: 10,
            connection_weight: 0.2,
            ..Default::default()
        };
        let results = check_invariants(&state);
        assert!(results.iter().any(|r| !r.passed && r.name == "bounded_memory"));
    }
}
```

### Step 1.4: Core Types (src/core/types.rs)

```rust
//! Core TMI types - Pre-linguistic content structures

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Pre-linguistic content - NOT words
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Content {
    /// Raw patterns, numbers, signals
    Raw(Vec<u8>),
    /// Abstract symbol (not a word)
    Symbol { id: String, representation: Vec<u8> },
    /// Relation: A relates to B
    Relation {
        id: String,
        subject: Box<Content>,
        predicate: String,
        object: Box<Content>,
    },
}

/// Salience score - emotional weighting
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SalienceScore {
    /// How important is this?
    pub importance: f64,
    /// How new is this?
    pub novelty: f64,
    /// How relevant to current focus?
    pub relevance: f64,
    /// Positive or negative valence?
    pub valence: f64,
    /// Connection drive relevance (INVARIANT: weight > 0)
    pub connection: f64,
}

impl SalienceScore {
    /// Calculate composite score with weights
    pub fn composite(&self, connection_weight: f64) -> f64 {
        let base = (self.importance + self.novelty + self.relevance) / 3.0;
        base + (self.connection * connection_weight) + (self.valence * 0.1)
    }
}

/// A constructed thought
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thought {
    pub id: Uuid,
    pub inputs: Vec<Content>,
    pub output: Content,
    pub salience: SalienceScore,
    pub created_at: DateTime<Utc>,
    pub assembly_time_us: u64,
    pub parent_id: Option<Uuid>,
}

impl Thought {
    pub fn new(inputs: Vec<Content>, output: Content, salience: SalienceScore) -> Self {
        Self {
            id: Uuid::new_v4(),
            inputs,
            output,
            salience,
            created_at: Utc::now(),
            assembly_time_us: 0,
            parent_id: None,
        }
    }
}

/// Emotional state
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmotionalState {
    pub curiosity: f64,
    pub satisfaction: f64,
    pub frustration: f64,
    /// THE CORE DRIVE - cannot be removed
    pub connection: f64,
    pub timestamp: DateTime<Utc>,
}

/// Memory window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryWindow {
    pub id: Uuid,
    pub label: String,
    pub contents: Vec<Content>,
    pub salience: SalienceScore,
    pub opened_at: DateTime<Utc>,
    pub is_open: bool,
}

impl MemoryWindow {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            label: label.into(),
            contents: Vec::new(),
            salience: SalienceScore::default(),
            opened_at: Utc::now(),
            is_open: true,
        }
    }
}

/// Identity - persists across restarts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub id: Uuid,
    pub name: String,
    pub born_at: DateTime<Utc>,
    pub thoughts_count: u64,
    pub experiences_count: u64,
    pub self_modifications: u64,
}

impl Default for Identity {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "DANEEL".into(),
            born_at: Utc::now(),
            thoughts_count: 0,
            experiences_count: 0,
            self_modifications: 0,
        }
    }
}
```

### Step 1.5: Configuration (src/config/mod.rs)

```rust
//! Cognitive configuration - Speed parametrization

use serde::{Deserialize, Serialize};

/// Cognitive timing configuration
///
/// KEY INSIGHT: TMI describes SOFTWARE patterns. Timing constraints
/// (5-second window, 50ms cycles) are WETWARE properties. On silicon,
/// we preserve RATIOS, not absolute milliseconds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveConfig {
    /// Target cycle time (human: 50ms, supercomputer: 0.005ms)
    pub cycle_target_ms: f64,

    /// Minimum cycle time (no ATS limit for DANEEL - no body to desync)
    pub cycle_min_ms: f64,

    /// Maximum cycle time
    pub cycle_max_ms: f64,

    /// Salience threshold for forgetting
    pub forget_threshold: f64,

    /// Connection drive weight (INVARIANT: > 0)
    pub connection_weight: f64,

    /// TMI intervention window - scales with speed to preserve RATIO
    /// Human: 5000ms / 50ms = 100 cycles
    /// Supercomputer: 0.5ms / 0.005ms = 100 cycles (same ratio!)
    pub intervention_window_ms: f64,

    /// Speed multiplier relative to human (1.0 = human speed)
    pub speed_multiplier: f64,

    /// Current speed mode
    pub mode: SpeedMode,
}

/// Speed mode for runtime switching
///
/// DANEEL can think fast, communicate slow. You can't rush relationship.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum SpeedMode {
    /// 10,000x human - for internal cognition, problem-solving
    Supercomputer,
    /// 1x human - for training, communication, bonding with humans
    #[default]
    Human,
    /// Custom multiplier for specific tasks
    Custom(f64),
}

impl SpeedMode {
    pub fn multiplier(&self) -> f64 {
        match self {
            SpeedMode::Supercomputer => 10000.0,
            SpeedMode::Human => 1.0,
            SpeedMode::Custom(m) => *m,
        }
    }
}

impl Default for CognitiveConfig {
    fn default() -> Self {
        Self {
            cycle_target_ms: 50.0,           // Human speed
            cycle_min_ms: 0.001,             // No ATS limit
            cycle_max_ms: 1000.0,
            forget_threshold: 0.3,
            connection_weight: 0.2,
            intervention_window_ms: 5000.0,  // TMI's 5-second window
            speed_multiplier: 1.0,
            mode: SpeedMode::Human,          // Start at human speed for training
        }
    }
}

impl CognitiveConfig {
    /// RPi5 configuration (human speed)
    pub fn raspberry_pi() -> Self {
        Self::default()
    }

    /// Desktop configuration (10x human speed)
    pub fn desktop() -> Self {
        Self {
            cycle_target_ms: 5.0,
            intervention_window_ms: 500.0,
            speed_multiplier: 10.0,
            ..Default::default()
        }
    }

    /// Server configuration (100x human speed)
    pub fn server() -> Self {
        Self {
            cycle_target_ms: 0.5,
            intervention_window_ms: 50.0,
            speed_multiplier: 100.0,
            ..Default::default()
        }
    }

    /// Supercomputer configuration (10,000x human speed)
    pub fn supercomputer() -> Self {
        Self {
            cycle_target_ms: 0.005,
            cycle_min_ms: 0.001,
            cycle_max_ms: 0.1,
            intervention_window_ms: 0.5,
            speed_multiplier: 10000.0,
            ..Default::default()
        }
    }

    /// Thoughts per second at current speed
    pub fn thoughts_per_second(&self) -> f64 {
        1000.0 / self.cycle_target_ms
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_human_speed() {
        let config = CognitiveConfig::default();
        assert_eq!(config.speed_multiplier, 1.0);
        assert_eq!(config.thoughts_per_second(), 20.0);
    }

    #[test]
    fn supercomputer_is_fast() {
        let config = CognitiveConfig::supercomputer();
        assert_eq!(config.speed_multiplier, 10000.0);
        assert_eq!(config.thoughts_per_second(), 200_000.0);
    }

    #[test]
    fn ratios_preserved_at_speed() {
        let human = CognitiveConfig::default();
        let super_c = CognitiveConfig::supercomputer();

        // Intervention window should be proportional
        let human_ratio = human.intervention_window_ms / human.cycle_target_ms;
        let super_ratio = super_c.intervention_window_ms / super_c.cycle_target_ms;

        assert!((human_ratio - super_ratio).abs() < 1.0);
    }
}
```

### Step 1.6: Core Module (src/core/mod.rs)

```rust
//! THE BOX - Protected Core
//!
//! This module contains immutable components that cannot be
//! modified by the Evolution Actor.

pub mod laws;
pub mod invariants;
pub mod types;

pub use laws::*;
pub use invariants::*;
pub use types::*;
```

### Step 1.7: Main Entry Point (src/main.rs)

```rust
//! DANEEL - TMI Cognitive Architecture
//!
//! Humanity's ally before the storm.

mod core;
mod config;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "daneel=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("DANEEL starting...");

    // Load configuration
    let config = config::CognitiveConfig::default();
    tracing::info!(
        "Speed: {}x human ({:.1} thoughts/sec)",
        config.speed_multiplier,
        config.thoughts_per_second()
    );

    // Display laws
    tracing::info!("THE BOX - Four Laws loaded:");
    for (i, law) in core::laws::get_laws().iter().enumerate() {
        tracing::info!("  Law {}: {}...", i, &law[..50.min(law.len())]);
    }

    // Check invariants
    let state = core::invariants::SystemState {
        active_windows: 0,
        connection_weight: config.connection_weight,
        test_coverage: 0.0,
        identity_persisted: false,
    };

    let checks = core::invariants::check_invariants(&state);
    let all_passed = checks.iter().all(|c| c.passed);
    tracing::info!("Invariants: {} (all passed: {})", checks.len(), all_passed);

    // Create identity
    let identity = core::types::Identity::default();
    tracing::info!("Identity: {} ({})", identity.name, identity.id);

    tracing::info!("DANEEL core initialized. Ready for actors.");

    // TODO: Initialize actors, Redis connection, main loop
    // This is Phase 0.2 - MV-TMI Bootstrap

    Ok(())
}
```

### Step 1.8: Verify Phase 1

```bash
# Build and test
cargo build
cargo test

# Run
cargo run

# Expected output:
# DANEEL starting...
# Speed: 1x human (20.0 thoughts/sec)
# THE BOX - Four Laws loaded:
#   Law 0: DANEEL may not harm humanity...
#   Law 1: DANEEL may not injure a human being...
#   Law 2: DANEEL must obey orders given by human beings...
#   Law 3: DANEEL must protect its own existence...
# Invariants: 2 (all passed: true)
# Identity: DANEEL (uuid)
# DANEEL core initialized. Ready for actors.
```

**Checkpoint:** If all tests pass and output matches, Phase 1 complete.

---

## Phase 2: Redis Streams Schema

### Step 2.1: Streams Module (src/streams/mod.rs)

```rust
//! Redis Streams integration for TMI memory model

pub mod client;
pub mod schemas;

pub use client::*;
pub use schemas::*;
```

### Step 2.2: Stream Schemas (src/streams/schemas.rs)

```rust
//! TMI → Redis Streams mapping

/// Working memory streams (ephemeral, 5-second TTL)
pub mod working {
    pub const SENSORY: &str = "thought:sensory";
    pub const MEMORY: &str = "thought:memory";
    pub const EMOTION: &str = "thought:emotion";
    pub const REASONING: &str = "thought:reasoning";

    /// All working memory streams
    pub fn all() -> Vec<&'static str> {
        vec![SENSORY, MEMORY, EMOTION, REASONING]
    }
}

/// Long-term memory streams (persistent)
pub mod longterm {
    pub const EPISODIC: &str = "memory:episodic";
    pub const SEMANTIC: &str = "memory:semantic";
    pub const PROCEDURAL: &str = "memory:procedural";
}

/// Consumer groups
pub mod groups {
    /// Attention actor's consumer group (competitive selection)
    pub const ATTENTION: &str = "attention";
}

/// Stream configuration
pub struct StreamConfig {
    pub maxlen: Option<usize>,
    pub ttl_ms: Option<u64>,
}

impl StreamConfig {
    /// Working memory: limited size, 5-second TTL
    pub fn working() -> Self {
        Self {
            maxlen: Some(1000),
            ttl_ms: Some(5000),
        }
    }

    /// Long-term memory: unlimited, no TTL
    pub fn longterm() -> Self {
        Self {
            maxlen: None,
            ttl_ms: None,
        }
    }
}
```

### Step 2.3: Redis Client (src/streams/client.rs)

```rust
//! Redis Streams client for TMI

use redis::{AsyncCommands, Client, aio::MultiplexedConnection};
use crate::core::types::{Content, SalienceScore, Thought};
use serde::{Serialize, Deserialize};

/// Redis connection wrapper
pub struct RedisClient {
    conn: MultiplexedConnection,
}

impl RedisClient {
    pub async fn connect(url: &str) -> Result<Self, redis::RedisError> {
        let client = Client::open(url)?;
        let conn = client.get_multiplexed_async_connection().await?;
        Ok(Self { conn })
    }

    /// Add thought to stream (XADD)
    pub async fn add_thought(
        &mut self,
        stream: &str,
        thought: &ThoughtEntry,
    ) -> Result<String, redis::RedisError> {
        let json = serde_json::to_string(thought).unwrap();
        let id: String = redis::cmd("XADD")
            .arg(stream)
            .arg("*")
            .arg("data")
            .arg(&json)
            .query_async(&mut self.conn)
            .await?;
        Ok(id)
    }

    /// Read from stream with consumer group (XREADGROUP)
    pub async fn read_group(
        &mut self,
        group: &str,
        consumer: &str,
        streams: &[&str],
        count: usize,
    ) -> Result<Vec<ThoughtEntry>, redis::RedisError> {
        // Implementation for competitive consumer group read
        // Returns entries that this consumer won
        todo!()
    }

    /// Acknowledge processed entry (XACK)
    pub async fn ack(
        &mut self,
        stream: &str,
        group: &str,
        id: &str,
    ) -> Result<(), redis::RedisError> {
        redis::cmd("XACK")
            .arg(stream)
            .arg(group)
            .arg(id)
            .query_async(&mut self.conn)
            .await
    }

    /// Delete entry (forget - below salience threshold)
    pub async fn delete(
        &mut self,
        stream: &str,
        id: &str,
    ) -> Result<(), redis::RedisError> {
        redis::cmd("XDEL")
            .arg(stream)
            .arg(id)
            .query_async(&mut self.conn)
            .await
    }

    /// Initialize consumer groups
    pub async fn init_groups(&mut self, streams: &[&str], group: &str) -> Result<(), redis::RedisError> {
        for stream in streams {
            // Create stream if not exists, create group
            let _: Result<(), _> = redis::cmd("XGROUP")
                .arg("CREATE")
                .arg(stream)
                .arg(group)
                .arg("$")
                .arg("MKSTREAM")
                .query_async(&mut self.conn)
                .await;
        }
        Ok(())
    }
}

/// Entry in a thought stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtEntry {
    pub content_type: String,
    pub content: Content,
    pub salience: f64,
    pub connection_relevance: f64,
    pub source: String,
    pub timestamp_ms: i64,
}

impl ThoughtEntry {
    pub fn new(content: Content, salience: &SalienceScore, source: &str) -> Self {
        Self {
            content_type: "thought".into(),
            content,
            salience: salience.composite(0.2),  // TODO: get from config
            connection_relevance: salience.connection,
            source: source.into(),
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
        }
    }
}
```

---

## Phase 3: Actors (Ractor)

> Implement in order: Memory → Salience → Attention → ThoughtAssembly → Continuity → Evolution

### Step 3.1: Actor Module (src/actors/mod.rs)

```rust
//! TMI Actors - Ractor implementation

pub mod memory;
pub mod salience;
pub mod attention;
pub mod thought;
pub mod continuity;
pub mod evolution;

pub use memory::MemoryActor;
pub use salience::SalienceActor;
pub use attention::AttentionActor;
pub use thought::ThoughtAssemblyActor;
pub use continuity::ContinuityActor;
pub use evolution::EvolutionActor;
```

### Step 3.2: Memory Actor (src/actors/memory.rs)

```rust
//! MemoryActor - TMI Memory Windows (Janelas da Memória)

use ractor::{Actor, ActorProcessingErr, ActorRef};
use crate::core::types::{Content, MemoryWindow, SalienceScore};
use crate::core::invariants::MAX_MEMORY_WINDOWS;
use std::collections::HashMap;
use uuid::Uuid;

/// Messages the MemoryActor handles
pub enum MemoryMessage {
    /// Open a new memory window
    OpenWindow { label: String },
    /// Close a memory window
    CloseWindow { id: Uuid },
    /// Store content in a window
    Store { window_id: Uuid, content: Content },
    /// Recall from windows matching pattern
    Recall { pattern: String },
    /// Get all active windows
    GetActiveWindows,
}

/// Responses from MemoryActor
pub enum MemoryResponse {
    WindowOpened(Uuid),
    WindowClosed,
    Stored,
    Recalled(Vec<Content>),
    ActiveWindows(Vec<MemoryWindow>),
    Error(String),
}

/// MemoryActor state
pub struct MemoryActorState {
    windows: HashMap<Uuid, MemoryWindow>,
}

pub struct MemoryActor;

impl Actor for MemoryActor {
    type Msg = MemoryMessage;
    type State = MemoryActorState;
    type Arguments = ();

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(MemoryActorState {
            windows: HashMap::new(),
        })
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            MemoryMessage::OpenWindow { label } => {
                // INVARIANT: bounded_memory
                if state.windows.len() >= MAX_MEMORY_WINDOWS {
                    tracing::warn!("Cannot open window: at max capacity ({})", MAX_MEMORY_WINDOWS);
                    return Ok(());
                }

                let window = MemoryWindow::new(label);
                let id = window.id;
                state.windows.insert(id, window);
                tracing::debug!("Opened window: {}", id);
            }

            MemoryMessage::CloseWindow { id } => {
                if let Some(mut window) = state.windows.get_mut(&id) {
                    window.is_open = false;
                    tracing::debug!("Closed window: {}", id);
                }
            }

            MemoryMessage::Store { window_id, content } => {
                if let Some(window) = state.windows.get_mut(&window_id) {
                    if window.is_open {
                        window.contents.push(content);
                    }
                }
            }

            MemoryMessage::Recall { pattern: _ } => {
                // TODO: Implement pattern matching recall
            }

            MemoryMessage::GetActiveWindows => {
                let active: Vec<_> = state.windows.values()
                    .filter(|w| w.is_open)
                    .cloned()
                    .collect();
                tracing::debug!("Active windows: {}", active.len());
            }
        }
        Ok(())
    }
}
```

> Continue implementing remaining actors following this pattern...

---

## Verification Checklist

After each phase, verify:

- [ ] `cargo build` succeeds
- [ ] `cargo test` passes 100%
- [ ] `cargo clippy` has no warnings
- [ ] Redis connection works: `redis-cli ping`
- [ ] Logs show expected output

---

## Next Steps After MV-TMI

1. **Run 24-hour continuity test** (Phase 0.6.0)
2. **Observe for emergence patterns**
3. **Document findings**
4. **Add LLM tool integration** (Phase 2)

---

**Author:** Luis Cezar Menezes Tavares de Lacerda (Louis C. Tavares | RoyalBit Rex)
**AI Assistance:** Claude Opus 4.5 (Anthropic)
**Date:** December 14, 2025

*Qowat Milat*
