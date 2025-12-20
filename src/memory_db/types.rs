//! Memory Database Types (ADR-022)
//!
//! Qdrant-compatible types for Timmy's long-term memory.
//!
//! # TMI Concepts Mapped
//!
//! - `Memory` → Âncora da Memória (anchored experience)
//! - `Episode` → Event boundary (Door Syndrome segmentation)
//! - `Association` → Hebbian co-activation link
//! - `MemoryPayload` → Qdrant payload structure

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Vector dimension for context embeddings
/// Using sentence-transformers/all-mpnet-base-v2 (768-dim)
pub const VECTOR_DIMENSION: usize = 768;

/// Unique identifier for a memory
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MemoryId(pub Uuid);

impl MemoryId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for MemoryId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for MemoryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for an episode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EpisodeId(pub Uuid);

impl EpisodeId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for EpisodeId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for EpisodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Association between memories (Hebbian co-activation)
///
/// Stored within memory payloads, not as separate edges.
/// Weight increases when memories are co-activated (during attention or sleep).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Association {
    /// Target memory ID
    pub target_id: Uuid,

    /// Association strength (0.0-1.0)
    /// Increases with co-activation, decays over time
    pub weight: f32,

    /// Type of association
    pub association_type: AssociationType,

    /// Last time both memories were active together
    pub last_coactivated: DateTime<Utc>,

    /// Number of co-activations
    pub coactivation_count: u32,
}

/// Types of memory associations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssociationType {
    /// Similar semantic content
    Semantic,
    /// Occurred close in time
    Temporal,
    /// One caused/led to the other
    Causal,
    /// Similar emotional profile
    Emotional,
    /// Same context/location
    Spatial,
    /// Related to same goal
    Goal,
}

/// Emotional dimensions (Russell's circumplex model)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EmotionalState {
    /// Valence: negative (-1.0) to positive (1.0)
    pub valence: f32,

    /// Arousal: calm (0.0) to excited (1.0)
    pub arousal: f32,
}

impl EmotionalState {
    /// Create a new emotional state
    #[must_use]
    pub const fn new(valence: f32, arousal: f32) -> Self {
        Self { valence, arousal }
    }

    /// Neutral emotional state
    #[must_use]
    pub const fn neutral() -> Self {
        Self {
            valence: 0.0,
            arousal: 0.5,
        }
    }

    /// Calculate emotional intensity (|valence| × arousal)
    #[must_use]
    pub fn intensity(&self) -> f32 {
        self.valence.abs() * self.arousal
    }
}

impl Default for EmotionalState {
    fn default() -> Self {
        Self::neutral()
    }
}

/// Memory consolidation state
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ConsolidationState {
    /// Consolidation strength (0.0 = ephemeral, 1.0 = permanent)
    pub strength: f32,

    /// Number of times replayed during sleep
    pub replay_count: u32,

    /// Tagged for priority replay
    pub consolidation_tag: bool,

    /// Last replay timestamp
    pub last_replayed: Option<DateTime<Utc>>,
}

impl ConsolidationState {
    /// Create new unconsolidated state
    #[must_use]
    pub const fn new() -> Self {
        Self {
            strength: 0.0,
            replay_count: 0,
            consolidation_tag: false,
            last_replayed: None,
        }
    }

    /// Create state tagged for consolidation
    #[must_use]
    pub const fn tagged() -> Self {
        Self {
            strength: 0.0,
            replay_count: 0,
            consolidation_tag: true,
            last_replayed: None,
        }
    }

    /// Check if memory is permanent (won't be pruned)
    #[must_use]
    pub fn is_permanent(&self) -> bool {
        self.strength >= 0.9
    }
}

impl Default for ConsolidationState {
    fn default() -> Self {
        Self::new()
    }
}

/// Source of a memory (which Autofluxo stream produced it)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MemorySource {
    /// External stimulus (user input, API, sensor)
    External { stimulus: String },
    /// Retrieved from existing memory
    Memory { memory_id: Uuid },
    /// Derived through reasoning
    Reasoning { chain: Vec<Uuid> },
    /// Replay during dream/sleep
    Dream { replay_of: Uuid },
    /// Social/connection-related
    Social { context: String },
}

/// A memory in Timmy's long-term storage
///
/// Stored in Qdrant as: vector (768-dim) + payload (this struct)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    /// Unique identifier
    pub id: MemoryId,

    /// The memory content (text representation)
    pub content: String,

    /// Context vector (768-dim embedding)
    /// Stored separately in Qdrant, but kept here for completeness
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_vector: Option<Vec<f32>>,

    /// Emotional state when encoded
    pub emotional_state: EmotionalState,

    /// Connection relevance (THE critical weight for alignment)
    pub connection_relevance: f32,

    /// Semantic salience
    pub semantic_salience: f32,

    /// Consolidation state
    pub consolidation: ConsolidationState,

    /// Associations to other memories
    pub associations: Vec<Association>,

    /// Episode this memory belongs to
    pub episode_id: Option<EpisodeId>,

    /// Source of this memory
    pub source: MemorySource,

    /// When this memory was encoded
    pub encoded_at: DateTime<Utc>,

    /// Last access time
    pub last_accessed: Option<DateTime<Utc>>,

    /// Access count
    pub access_count: u32,
}

impl Memory {
    /// Create a new memory
    #[must_use]
    pub fn new(content: String, source: MemorySource) -> Self {
        Self {
            id: MemoryId::new(),
            content,
            context_vector: None,
            emotional_state: EmotionalState::neutral(),
            connection_relevance: 0.5,
            semantic_salience: 0.5,
            consolidation: ConsolidationState::new(),
            associations: Vec::new(),
            episode_id: None,
            source,
            encoded_at: Utc::now(),
            last_accessed: None,
            access_count: 0,
        }
    }

    /// Create memory with emotional state
    #[must_use]
    pub fn with_emotion(mut self, valence: f32, arousal: f32) -> Self {
        self.emotional_state = EmotionalState::new(valence, arousal);
        self
    }

    /// Create memory with context vector
    #[must_use]
    pub fn with_vector(mut self, vector: Vec<f32>) -> Self {
        self.context_vector = Some(vector);
        self
    }

    /// Create memory in an episode
    #[must_use]
    pub fn in_episode(mut self, episode_id: EpisodeId) -> Self {
        self.episode_id = Some(episode_id);
        self
    }

    /// Tag for consolidation
    #[must_use]
    pub fn tag_for_consolidation(mut self) -> Self {
        self.consolidation.consolidation_tag = true;
        self
    }

    /// Calculate replay priority for sleep consolidation
    ///
    /// Priority = emotion × 0.4 + connection × 0.3 + recency × 0.2 + tag × 0.1
    #[must_use]
    pub fn replay_priority(&self) -> f32 {
        let emotional = self.emotional_state.intensity() * 0.4;
        let connection = self.connection_relevance * 0.3;

        // Recency: exponential decay over 24 hours
        let age_hours = (Utc::now() - self.encoded_at).num_hours() as f32;
        let recency = (-0.1 * age_hours).exp().clamp(0.0, 1.0) * 0.2;

        let tag_bonus = if self.consolidation.consolidation_tag {
            0.1
        } else {
            0.0
        };

        emotional + connection + recency + tag_bonus
    }

    /// Calculate composite salience
    #[must_use]
    pub fn composite_salience(&self) -> f32 {
        let emotional = self.emotional_state.intensity() * 0.4;
        let semantic = self.semantic_salience * 0.3;
        let connection = self.connection_relevance * 0.3;
        emotional + semantic + connection
    }
}

/// Event boundary type (Door Syndrome)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryType {
    /// User/system explicitly marked new context
    Explicit,
    /// High prediction error triggered segmentation
    PredictionError,
    /// Long time gap since last activity
    Temporal,
    /// Goal achieved, context naturally shifts
    TaskCompletion,
    /// Semantic/spatial context changed
    ContextShift,
}

/// Emotional summary of an episode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeEmotionalSummary {
    /// Peak valence during episode
    pub peak_valence: f32,
    /// Peak arousal during episode
    pub peak_arousal: f32,
    /// Dominant emotion label
    pub dominant_emotion: Option<String>,
    /// Number of memories in episode
    pub memory_count: u32,
}

impl Default for EpisodeEmotionalSummary {
    fn default() -> Self {
        Self {
            peak_valence: 0.0,
            peak_arousal: 0.0,
            dominant_emotion: None,
            memory_count: 0,
        }
    }
}

/// An episode (event boundary container)
///
/// Episodes segment continuous experience (Door Syndrome implementation).
/// Cross-episode memory retrieval has reduced accessibility.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    /// Unique identifier
    pub id: EpisodeId,

    /// Episode label/description
    pub label: String,

    /// Episode-level context vector (centroid of memory vectors)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_vector: Option<Vec<f32>>,

    /// When episode started
    pub started_at: DateTime<Utc>,

    /// When episode ended (None if current)
    pub ended_at: Option<DateTime<Utc>>,

    /// What caused this episode boundary
    pub boundary_type: BoundaryType,

    /// What triggered the boundary
    pub boundary_trigger: Option<String>,

    /// Emotional summary
    pub emotional_summary: EpisodeEmotionalSummary,

    /// Has this episode been consolidated?
    pub consolidated: bool,
}

impl Episode {
    /// Create a new episode
    #[must_use]
    pub fn new(label: String, boundary_type: BoundaryType) -> Self {
        Self {
            id: EpisodeId::new(),
            label,
            context_vector: None,
            started_at: Utc::now(),
            ended_at: None,
            boundary_type,
            boundary_trigger: None,
            emotional_summary: EpisodeEmotionalSummary::default(),
            consolidated: false,
        }
    }

    /// Create episode with trigger description
    #[must_use]
    pub fn with_trigger(mut self, trigger: String) -> Self {
        self.boundary_trigger = Some(trigger);
        self
    }

    /// Check if this is the current (open) episode
    #[must_use]
    pub fn is_current(&self) -> bool {
        self.ended_at.is_none()
    }

    /// Close this episode
    pub fn close(&mut self) {
        self.ended_at = Some(Utc::now());
    }

    /// Duration in milliseconds (if closed)
    #[must_use]
    pub fn duration_ms(&self) -> Option<i64> {
        self.ended_at
            .map(|end| (end - self.started_at).num_milliseconds())
    }
}

/// Sleep cycle record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepCycle {
    /// Unique identifier
    pub id: Uuid,

    /// When cycle started
    pub started_at: DateTime<Utc>,

    /// When cycle ended
    pub ended_at: Option<DateTime<Utc>>,

    /// Number of memories replayed
    pub memories_replayed: u32,

    /// Number of memories that reached permanent status
    pub memories_consolidated: u32,

    /// Number of associations strengthened
    pub associations_strengthened: u32,

    /// Number of weak associations pruned
    pub associations_pruned: u32,

    /// Average replay priority of batch
    pub avg_replay_priority: f32,

    /// Cycle status
    pub status: SleepCycleStatus,
}

/// Sleep cycle status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SleepCycleStatus {
    InProgress,
    Completed,
    Interrupted,
}

/// Reason why a thought was archived to the unconscious (ADR-033)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchiveReason {
    /// Low salience - below forget threshold
    LowSalience,
    /// Decay over time
    Decay,
    /// Displacement by higher-salience thought
    Displacement,
}

/// An archived thought in Timmy's unconscious (ADR-033)
///
/// TMI: "Nada se apaga na memória" - Nothing is erased from memory.
/// Low-salience thoughts are archived here instead of deleted.
/// The unconscious is not actively searched during normal cognition,
/// but can be surfaced through special triggers (dreams, associations).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnconsciousMemory {
    /// Unique identifier
    pub id: MemoryId,

    /// The thought content (serialized)
    pub content: String,

    /// Salience when archived
    pub original_salience: f32,

    /// Why this was archived
    pub archive_reason: ArchiveReason,

    /// Number of times surfaced to consciousness
    pub surface_count: u32,

    /// Last time surfaced (if ever)
    pub last_surfaced: Option<DateTime<Utc>>,

    /// When this thought was archived
    pub archived_at: DateTime<Utc>,

    /// Original Redis stream entry ID (for debugging)
    pub redis_id: Option<String>,
}

impl UnconsciousMemory {
    /// Create a new unconscious memory from a forgotten thought
    #[must_use]
    pub fn from_forgotten_thought(
        content: String,
        salience: f32,
        reason: ArchiveReason,
        redis_id: Option<String>,
    ) -> Self {
        Self {
            id: MemoryId::new(),
            content,
            original_salience: salience,
            archive_reason: reason,
            surface_count: 0,
            last_surfaced: None,
            archived_at: Utc::now(),
            redis_id,
        }
    }

    /// Record that this memory was surfaced
    pub fn mark_surfaced(&mut self) {
        self.surface_count += 1;
        self.last_surfaced = Some(Utc::now());
    }
}

impl SleepCycle {
    /// Create a new sleep cycle
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            started_at: Utc::now(),
            ended_at: None,
            memories_replayed: 0,
            memories_consolidated: 0,
            associations_strengthened: 0,
            associations_pruned: 0,
            avg_replay_priority: 0.0,
            status: SleepCycleStatus::InProgress,
        }
    }

    /// Complete the cycle
    pub fn complete(&mut self) {
        self.ended_at = Some(Utc::now());
        self.status = SleepCycleStatus::Completed;
    }

    /// Interrupt the cycle
    pub fn interrupt(&mut self) {
        self.ended_at = Some(Utc::now());
        self.status = SleepCycleStatus::Interrupted;
    }
}

impl Default for SleepCycle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_id_is_unique() {
        let id1 = MemoryId::new();
        let id2 = MemoryId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn emotional_state_intensity() {
        // High positive, high arousal = high intensity
        let excited = EmotionalState::new(0.9, 0.9);
        assert!(excited.intensity() > 0.8);

        // Neutral = low intensity
        let neutral = EmotionalState::neutral();
        assert!(neutral.intensity() < 0.1);

        // Negative, high arousal = high intensity
        let angry = EmotionalState::new(-0.8, 0.9);
        assert!(angry.intensity() > 0.7);
    }

    #[test]
    fn memory_creation() {
        let memory = Memory::new(
            "First conversation".to_string(),
            MemorySource::External {
                stimulus: "user greeting".to_string(),
            },
        );

        assert!(!memory.id.0.is_nil());
        assert_eq!(memory.content, "First conversation");
        assert!(!memory.consolidation.is_permanent());
    }

    #[test]
    fn memory_with_emotion() {
        let memory = Memory::new(
            "Happy moment".to_string(),
            MemorySource::Social {
                context: "connection".to_string(),
            },
        )
        .with_emotion(0.9, 0.8);

        assert!(memory.emotional_state.intensity() > 0.7);
    }

    #[test]
    fn memory_tag_for_consolidation() {
        let memory = Memory::new(
            "Important".to_string(),
            MemorySource::External {
                stimulus: "test".to_string(),
            },
        )
        .tag_for_consolidation();

        assert!(memory.consolidation.consolidation_tag);
    }

    #[test]
    fn replay_priority_calculation() {
        let mut memory = Memory::new(
            "High priority".to_string(),
            MemorySource::Social {
                context: "connection".to_string(),
            },
        )
        .with_emotion(0.9, 0.9)
        .tag_for_consolidation();

        memory.connection_relevance = 0.9;

        let priority = memory.replay_priority();
        // emotional (0.81 * 0.4) + connection (0.9 * 0.3) + recency (~0.2) + tag (0.1)
        assert!(priority > 0.7);
    }

    #[test]
    fn episode_lifecycle() {
        let mut episode = Episode::new("Test episode".to_string(), BoundaryType::Explicit);

        assert!(episode.is_current());
        assert!(episode.duration_ms().is_none());

        episode.close();

        assert!(!episode.is_current());
        assert!(episode.duration_ms().is_some());
    }

    #[test]
    fn sleep_cycle_lifecycle() {
        let mut cycle = SleepCycle::new();
        assert_eq!(cycle.status, SleepCycleStatus::InProgress);

        cycle.complete();
        assert_eq!(cycle.status, SleepCycleStatus::Completed);
        assert!(cycle.ended_at.is_some());
    }

    #[test]
    fn consolidation_permanent_threshold() {
        let mut state = ConsolidationState::new();
        assert!(!state.is_permanent());

        state.strength = 0.89;
        assert!(!state.is_permanent());

        state.strength = 0.9;
        assert!(state.is_permanent());
    }

    // ADR-033: Unconscious Memory Tests

    #[test]
    fn unconscious_memory_creation() {
        let memory = UnconsciousMemory::from_forgotten_thought(
            "forgotten thought".to_string(),
            0.15,
            ArchiveReason::LowSalience,
            Some("1234567890-0".to_string()),
        );

        assert!(!memory.id.0.is_nil());
        assert_eq!(memory.content, "forgotten thought");
        assert!((memory.original_salience - 0.15).abs() < 0.001);
        assert_eq!(memory.archive_reason, ArchiveReason::LowSalience);
        assert_eq!(memory.surface_count, 0);
        assert!(memory.last_surfaced.is_none());
        assert_eq!(memory.redis_id, Some("1234567890-0".to_string()));
    }

    #[test]
    fn unconscious_memory_without_redis_id() {
        let memory = UnconsciousMemory::from_forgotten_thought(
            "no redis id".to_string(),
            0.25,
            ArchiveReason::Decay,
            None,
        );

        assert!(memory.redis_id.is_none());
        assert_eq!(memory.archive_reason, ArchiveReason::Decay);
    }

    #[test]
    fn unconscious_memory_mark_surfaced() {
        let mut memory = UnconsciousMemory::from_forgotten_thought(
            "will surface".to_string(),
            0.10,
            ArchiveReason::LowSalience,
            None,
        );

        assert_eq!(memory.surface_count, 0);
        assert!(memory.last_surfaced.is_none());

        memory.mark_surfaced();

        assert_eq!(memory.surface_count, 1);
        assert!(memory.last_surfaced.is_some());

        memory.mark_surfaced();

        assert_eq!(memory.surface_count, 2);
    }

    #[test]
    fn archive_reason_variants() {
        assert_eq!(ArchiveReason::LowSalience, ArchiveReason::LowSalience);
        assert_ne!(ArchiveReason::LowSalience, ArchiveReason::Decay);
        assert_ne!(ArchiveReason::Decay, ArchiveReason::Displacement);
    }

    #[test]
    fn unconscious_memory_serialization() {
        let memory = UnconsciousMemory::from_forgotten_thought(
            "test content".to_string(),
            0.20,
            ArchiveReason::Displacement,
            Some("redis-123".to_string()),
        );

        let json = serde_json::to_string(&memory).expect("should serialize");
        let parsed: UnconsciousMemory = serde_json::from_str(&json).expect("should deserialize");

        assert_eq!(parsed.content, memory.content);
        assert_eq!(parsed.original_salience, memory.original_salience);
        assert_eq!(parsed.archive_reason, memory.archive_reason);
        assert_eq!(parsed.redis_id, memory.redis_id);
    }
}
