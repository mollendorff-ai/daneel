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

mod unconscious;

pub use unconscious::{
    ArchiveReason, IdentityMetadata, SleepCycle, SleepCycleStatus, UnconsciousMemory,
    IDENTITY_RECORD_ID,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Default BCM threshold for old memories without theta_m field
const fn default_theta_m() -> f32 {
    0.1
}

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

    /// Eligibility trace for Three-Factor Learning (MSTDPET)
    /// Persists ~100-500ms to allow delayed reward modulation
    #[serde(default)]
    pub eligibility_trace: f32,
}

impl Association {
    /// Calculate weight decay based on time and consolidation status
    ///
    /// Implements "Hybrid Decay" (shodh-memory):
    /// - Short-term (< 10 co-activations): Exponential decay (fast forgetting)
    /// - Long-term (>= 10 co-activations): Power-law decay (slow forgetting)
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn calculate_decay(&self, now: DateTime<Utc>) -> f32 {
        let age_hours = (now - self.last_coactivated).num_minutes() as f32 / 60.0;

        // Prevent decay for very recent associations (under 1 hour)
        if age_hours < 1.0 {
            return 1.0;
        }

        if self.coactivation_count < 10 {
            // Short-term: Exponential decay
            // Halflife approx 24 hours
            (-0.03 * age_hours).exp()
        } else {
            // Long-term: Power-law decay (scale-free)
            // Slower decay for consolidated memories
            let t = age_hours.max(1.0);
            t.powf(-0.1)
        }
    }
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

    /// BCM Sliding Threshold (`theta_m`)
    /// Running average of post-synaptic activity (E[y^2]) for Hebbian learning
    #[serde(default = "default_theta_m")]
    pub theta_m: f32,

    /// Semantic salience
    pub semantic_salience: f32,

    /// Consolidation state
    pub consolidation: ConsolidationState,

    /// Cluster ID from manifold clustering (VCONN-7)
    #[serde(default)]
    pub cluster_id: Option<u32>,

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
            theta_m: 0.1, // Start with low threshold to encourage initial learning
            semantic_salience: 0.5,
            consolidation: ConsolidationState::new(),
            cluster_id: None,
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
    pub const fn with_emotion(mut self, valence: f32, arousal: f32) -> Self {
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
    pub const fn in_episode(mut self, episode_id: EpisodeId) -> Self {
        self.episode_id = Some(episode_id);
        self
    }

    /// Tag for consolidation
    #[must_use]
    pub const fn tag_for_consolidation(mut self) -> Self {
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
        #[allow(clippy::cast_precision_loss)] // Hours won't exceed f32 precision
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

    /// Update BCM sliding threshold (`theta_m`)
    ///
    /// `theta_m` = `theta_m` + (y^2 - `theta_m`) / tau
    /// Where y is the current activity/salience.
    /// This keeps the threshold moving to stabilize learning.
    pub fn update_bcm_threshold(&mut self, current_activity: f32, tau: f32) {
        let activity_sq = current_activity.powi(2);
        self.theta_m += (activity_sq - self.theta_m) / tau;
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
    pub const fn is_current(&self) -> bool {
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

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    // =========================================================================
    // MemoryId Tests
    // =========================================================================

    #[test]
    fn memory_id_is_unique() {
        let id1 = MemoryId::new();
        let id2 = MemoryId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn memory_id_default() {
        let id1 = MemoryId::default();
        let id2 = MemoryId::default();
        assert_ne!(id1, id2);
        assert!(!id1.0.is_nil());
    }

    #[test]
    fn memory_id_display() {
        let id = MemoryId::new();
        let displayed = format!("{id}");
        assert_eq!(displayed.len(), 36);
        assert!(displayed.contains('-'));
    }

    #[test]
    fn memory_id_serialization() {
        let id = MemoryId::new();
        let json = serde_json::to_string(&id).expect("should serialize");
        let parsed: MemoryId = serde_json::from_str(&json).expect("should deserialize");
        assert_eq!(parsed, id);
    }

    // =========================================================================
    // EpisodeId Tests
    // =========================================================================

    #[test]
    fn episode_id_new_and_unique() {
        let id1 = EpisodeId::new();
        let id2 = EpisodeId::new();
        assert_ne!(id1, id2);
        assert!(!id1.0.is_nil());
    }

    #[test]
    fn episode_id_default() {
        let id1 = EpisodeId::default();
        let id2 = EpisodeId::default();
        assert_ne!(id1, id2);
        assert!(!id1.0.is_nil());
    }

    #[test]
    fn episode_id_display() {
        let id = EpisodeId::new();
        let displayed = format!("{id}");
        assert_eq!(displayed.len(), 36);
        assert!(displayed.contains('-'));
    }

    #[test]
    fn episode_id_serialization() {
        let id = EpisodeId::new();
        let json = serde_json::to_string(&id).expect("should serialize");
        let parsed: EpisodeId = serde_json::from_str(&json).expect("should deserialize");
        assert_eq!(parsed, id);
    }

    // =========================================================================
    // EmotionalState Tests
    // =========================================================================

    #[test]
    fn emotional_state_intensity() {
        let excited = EmotionalState::new(0.9, 0.9);
        assert!(excited.intensity() > 0.8);

        let neutral = EmotionalState::neutral();
        assert!(neutral.intensity() < 0.1);

        let angry = EmotionalState::new(-0.8, 0.9);
        assert!(angry.intensity() > 0.7);
    }

    #[test]
    fn emotional_state_default() {
        let state = EmotionalState::default();
        assert!((state.valence - 0.0).abs() < f32::EPSILON);
        assert!((state.arousal - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn emotional_state_serialization() {
        let state = EmotionalState::new(0.7, 0.8);
        let json = serde_json::to_string(&state).expect("should serialize");
        let parsed: EmotionalState = serde_json::from_str(&json).expect("should deserialize");

        assert!((parsed.valence - 0.7).abs() < f32::EPSILON);
        assert!((parsed.arousal - 0.8).abs() < f32::EPSILON);
    }

    // =========================================================================
    // ConsolidationState Tests
    // =========================================================================

    #[test]
    fn consolidation_permanent_threshold() {
        let mut state = ConsolidationState::new();
        assert!(!state.is_permanent());

        state.strength = 0.89;
        assert!(!state.is_permanent());

        state.strength = 0.9;
        assert!(state.is_permanent());
    }

    #[test]
    fn consolidation_state_tagged() {
        let state = ConsolidationState::tagged();
        assert!(state.consolidation_tag);
        assert!((state.strength - 0.0).abs() < f32::EPSILON);
        assert_eq!(state.replay_count, 0);
        assert!(state.last_replayed.is_none());
    }

    #[test]
    fn consolidation_state_default() {
        let state = ConsolidationState::default();
        assert!(!state.consolidation_tag);
        assert!((state.strength - 0.0).abs() < f32::EPSILON);
        assert_eq!(state.replay_count, 0);
        assert!(state.last_replayed.is_none());
    }

    #[test]
    fn consolidation_state_serialization() {
        let mut state = ConsolidationState::tagged();
        state.strength = 0.5;
        state.replay_count = 3;
        state.last_replayed = Some(Utc::now());

        let json = serde_json::to_string(&state).expect("should serialize");
        let parsed: ConsolidationState = serde_json::from_str(&json).expect("should deserialize");

        assert!(parsed.consolidation_tag);
        assert!((parsed.strength - 0.5).abs() < f32::EPSILON);
        assert_eq!(parsed.replay_count, 3);
        assert!(parsed.last_replayed.is_some());
    }

    // =========================================================================
    // Memory Tests
    // =========================================================================

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
    fn memory_with_vector() {
        let vector = vec![0.1_f32; VECTOR_DIMENSION];
        let memory = Memory::new(
            "test".to_string(),
            MemorySource::External {
                stimulus: "test".to_string(),
            },
        )
        .with_vector(vector);

        assert!(memory.context_vector.is_some());
        assert_eq!(memory.context_vector.unwrap().len(), VECTOR_DIMENSION);
    }

    #[test]
    fn memory_in_episode() {
        let episode_id = EpisodeId::new();
        let memory = Memory::new(
            "test".to_string(),
            MemorySource::External {
                stimulus: "test".to_string(),
            },
        )
        .in_episode(episode_id);

        assert!(memory.episode_id.is_some());
        assert_eq!(memory.episode_id.unwrap(), episode_id);
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
        assert!(priority > 0.7);
    }

    #[test]
    fn replay_priority_without_consolidation_tag() {
        let mut memory = Memory::new(
            "Untagged memory".to_string(),
            MemorySource::External {
                stimulus: "test".to_string(),
            },
        )
        .with_emotion(0.5, 0.5);

        memory.connection_relevance = 0.5;

        assert!(!memory.consolidation.consolidation_tag);

        let priority = memory.replay_priority();
        assert!(priority < 0.6);
    }

    #[test]
    fn memory_composite_salience() {
        let mut memory = Memory::new(
            "test".to_string(),
            MemorySource::External {
                stimulus: "test".to_string(),
            },
        )
        .with_emotion(0.8, 0.9);

        memory.semantic_salience = 0.7;
        memory.connection_relevance = 0.6;

        let salience = memory.composite_salience();
        assert!(salience > 0.6);
        assert!(salience < 0.8);
    }

    #[test]
    fn memory_serialization() {
        let memory = Memory::new(
            "test content".to_string(),
            MemorySource::External {
                stimulus: "input".to_string(),
            },
        )
        .with_emotion(0.5, 0.6)
        .tag_for_consolidation();

        let json = serde_json::to_string(&memory).expect("should serialize");
        let parsed: Memory = serde_json::from_str(&json).expect("should deserialize");

        assert_eq!(parsed.content, "test content");
        assert!((parsed.emotional_state.valence - 0.5).abs() < f32::EPSILON);
        assert!(parsed.consolidation.consolidation_tag);
    }

    #[test]
    fn memory_serialization_skips_none_vector() {
        let memory = Memory::new(
            "test".to_string(),
            MemorySource::External {
                stimulus: "test".to_string(),
            },
        );

        let json = serde_json::to_string(&memory).expect("should serialize");
        assert!(!json.contains("context_vector"));
    }

    #[test]
    fn memory_serialization_includes_vector() {
        let vector = vec![0.1_f32; 10];
        let memory = Memory::new(
            "test".to_string(),
            MemorySource::External {
                stimulus: "test".to_string(),
            },
        )
        .with_vector(vector);

        let json = serde_json::to_string(&memory).expect("should serialize");
        assert!(json.contains("context_vector"));
    }

    // =========================================================================
    // Episode Tests
    // =========================================================================

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
    fn episode_with_trigger() {
        let episode = Episode::new("Test".to_string(), BoundaryType::PredictionError)
            .with_trigger("High surprise".to_string());

        assert!(episode.boundary_trigger.is_some());
        assert_eq!(episode.boundary_trigger.unwrap(), "High surprise");
    }

    #[test]
    fn episode_serialization() {
        let mut episode = Episode::new("Test episode".to_string(), BoundaryType::Explicit)
            .with_trigger("user request".to_string());
        episode.close();

        let json = serde_json::to_string(&episode).expect("should serialize");
        let parsed: Episode = serde_json::from_str(&json).expect("should deserialize");

        assert_eq!(parsed.label, "Test episode");
        assert_eq!(parsed.boundary_type, BoundaryType::Explicit);
        assert_eq!(parsed.boundary_trigger, Some("user request".to_string()));
        assert!(parsed.ended_at.is_some());
    }

    #[test]
    fn episode_serialization_skips_none_vector() {
        let episode = Episode::new("Test".to_string(), BoundaryType::Temporal);
        let json = serde_json::to_string(&episode).expect("should serialize");
        assert!(!json.contains("context_vector"));
    }

    // =========================================================================
    // Association Tests
    // =========================================================================

    #[test]
    fn association_type_serialization() {
        let types = [
            AssociationType::Semantic,
            AssociationType::Temporal,
            AssociationType::Causal,
            AssociationType::Emotional,
            AssociationType::Spatial,
            AssociationType::Goal,
        ];

        for assoc_type in types {
            let json = serde_json::to_string(&assoc_type).expect("should serialize");
            let parsed: AssociationType = serde_json::from_str(&json).expect("should deserialize");
            assert_eq!(parsed, assoc_type);
        }
    }

    #[test]
    fn association_serialization() {
        let assoc = Association {
            target_id: Uuid::new_v4(),
            weight: 0.75,
            association_type: AssociationType::Semantic,
            last_coactivated: Utc::now(),
            coactivation_count: 5,
            eligibility_trace: 0.0,
        };

        let json = serde_json::to_string(&assoc).expect("should serialize");
        let parsed: Association = serde_json::from_str(&json).expect("should deserialize");

        assert_eq!(parsed.target_id, assoc.target_id);
        assert!((parsed.weight - 0.75).abs() < f32::EPSILON);
        assert_eq!(parsed.association_type, AssociationType::Semantic);
        assert_eq!(parsed.coactivation_count, 5);
    }

    // =========================================================================
    // MemorySource Tests
    // =========================================================================

    #[test]
    fn memory_source_external_serialization() {
        let source = MemorySource::External {
            stimulus: "user input".to_string(),
        };
        let json = serde_json::to_string(&source).expect("should serialize");
        assert!(json.contains("\"type\":\"external\""));

        let parsed: MemorySource = serde_json::from_str(&json).expect("should deserialize");
        if let MemorySource::External { stimulus } = parsed {
            assert_eq!(stimulus, "user input");
        } else {
            panic!("Expected External variant");
        }
    }

    #[test]
    fn memory_source_memory_serialization() {
        let id = Uuid::new_v4();
        let source = MemorySource::Memory { memory_id: id };
        let json = serde_json::to_string(&source).expect("should serialize");
        assert!(json.contains("\"type\":\"memory\""));

        let parsed: MemorySource = serde_json::from_str(&json).expect("should deserialize");
        if let MemorySource::Memory { memory_id } = parsed {
            assert_eq!(memory_id, id);
        } else {
            panic!("Expected Memory variant");
        }
    }

    #[test]
    fn memory_source_reasoning_serialization() {
        let chain = vec![Uuid::new_v4(), Uuid::new_v4()];
        let source = MemorySource::Reasoning { chain };
        let json = serde_json::to_string(&source).expect("should serialize");
        assert!(json.contains("\"type\":\"reasoning\""));

        let parsed: MemorySource = serde_json::from_str(&json).expect("should deserialize");
        if let MemorySource::Reasoning {
            chain: parsed_chain,
        } = parsed
        {
            assert_eq!(parsed_chain.len(), 2);
        } else {
            panic!("Expected Reasoning variant");
        }
    }

    #[test]
    fn memory_source_dream_serialization() {
        let id = Uuid::new_v4();
        let source = MemorySource::Dream { replay_of: id };
        let json = serde_json::to_string(&source).expect("should serialize");
        assert!(json.contains("\"type\":\"dream\""));

        let parsed: MemorySource = serde_json::from_str(&json).expect("should deserialize");
        if let MemorySource::Dream { replay_of } = parsed {
            assert_eq!(replay_of, id);
        } else {
            panic!("Expected Dream variant");
        }
    }

    #[test]
    fn memory_source_social_serialization() {
        let source = MemorySource::Social {
            context: "connection".to_string(),
        };
        let json = serde_json::to_string(&source).expect("should serialize");
        assert!(json.contains("\"type\":\"social\""));

        let parsed: MemorySource = serde_json::from_str(&json).expect("should deserialize");
        if let MemorySource::Social { context } = parsed {
            assert_eq!(context, "connection");
        } else {
            panic!("Expected Social variant");
        }
    }

    // =========================================================================
    // BoundaryType Tests
    // =========================================================================

    #[test]
    fn boundary_type_serialization() {
        let types = [
            BoundaryType::Explicit,
            BoundaryType::PredictionError,
            BoundaryType::Temporal,
            BoundaryType::TaskCompletion,
            BoundaryType::ContextShift,
        ];

        for boundary_type in types {
            let json = serde_json::to_string(&boundary_type).expect("should serialize");
            let parsed: BoundaryType = serde_json::from_str(&json).expect("should deserialize");
            assert_eq!(parsed, boundary_type);
        }
    }

    // =========================================================================
    // EpisodeEmotionalSummary Tests
    // =========================================================================

    #[test]
    fn episode_emotional_summary_default() {
        let summary = EpisodeEmotionalSummary::default();
        assert!((summary.peak_valence - 0.0).abs() < f32::EPSILON);
        assert!((summary.peak_arousal - 0.0).abs() < f32::EPSILON);
        assert!(summary.dominant_emotion.is_none());
        assert_eq!(summary.memory_count, 0);
    }

    #[test]
    fn episode_emotional_summary_serialization() {
        let summary = EpisodeEmotionalSummary {
            peak_valence: 0.9,
            peak_arousal: 0.8,
            dominant_emotion: Some("joy".to_string()),
            memory_count: 10,
        };

        let json = serde_json::to_string(&summary).expect("should serialize");
        let parsed: EpisodeEmotionalSummary =
            serde_json::from_str(&json).expect("should deserialize");

        assert!((parsed.peak_valence - 0.9).abs() < f32::EPSILON);
        assert!((parsed.peak_arousal - 0.8).abs() < f32::EPSILON);
        assert_eq!(parsed.dominant_emotion, Some("joy".to_string()));
        assert_eq!(parsed.memory_count, 10);
    }

    // =========================================================================
    // Constants Tests
    // =========================================================================

    #[test]
    fn vector_dimension_constant() {
        assert_eq!(VECTOR_DIMENSION, 768);
    }

    #[test]
    fn identity_record_id_constant() {
        assert_eq!(IDENTITY_RECORD_ID, "00000000-0000-0000-0000-000000000001");
        let parsed = Uuid::parse_str(IDENTITY_RECORD_ID);
        assert!(parsed.is_ok());
    }

    // =========================================================================
    // Hebbian Learning Tests (VCONN-1)
    // =========================================================================

    #[test]
    fn bcm_threshold_update() {
        let mut memory = Memory::new(
            "Learning".to_string(),
            MemorySource::External {
                stimulus: "test".to_string(),
            },
        );

        // Initial state
        memory.theta_m = 0.0;

        // High activity (1.0) -> threshold should rise
        // theta_m += (1.0^2 - 0.0) / 10.0 = 0.1
        memory.update_bcm_threshold(1.0, 10.0);
        assert!((memory.theta_m - 0.1).abs() < f32::EPSILON);

        // Low activity (0.0) -> threshold should fall
        // theta_m += (0.0^2 - 0.1) / 10.0 = 0.1 - 0.01 = 0.09
        memory.update_bcm_threshold(0.0, 10.0);
        assert!((memory.theta_m - 0.09).abs() < 1e-6);
    }

    #[test]
    fn hybrid_decay_short_term() {
        let assoc = Association {
            target_id: Uuid::new_v4(),
            weight: 1.0,
            association_type: AssociationType::Semantic,
            last_coactivated: Utc::now() - chrono::Duration::hours(24),
            coactivation_count: 5, // < 10, so exponential decay
            eligibility_trace: 0.0,
        };

        let decay = assoc.calculate_decay(Utc::now());

        // Exp decay: exp(-0.03 * 24) = exp(-0.72) ≈ 0.486
        assert!(decay < 0.5);
        assert!(decay > 0.4);
    }

    #[test]
    fn hybrid_decay_long_term() {
        let assoc = Association {
            target_id: Uuid::new_v4(),
            weight: 1.0,
            association_type: AssociationType::Semantic,
            last_coactivated: Utc::now() - chrono::Duration::hours(24),
            coactivation_count: 20, // >= 10, so power-law decay
            eligibility_trace: 0.0,
        };

        let decay = assoc.calculate_decay(Utc::now());

        // Power law: 24^(-0.1) ≈ 0.72
        assert!(decay > 0.7);
        assert!(decay < 0.8);

        // Should be better preserved than short-term
        let short_term_assoc = Association {
            coactivation_count: 5,
            ..assoc
        };
        assert!(decay > short_term_assoc.calculate_decay(Utc::now()));
    }
}
