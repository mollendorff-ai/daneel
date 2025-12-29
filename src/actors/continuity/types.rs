//! Types for the `ContinuityActor`
//!
//! TMI Concept: "Âncora da Memória" (Memory Anchor) + Identity Persistence
//!
//! # The Memory Anchor
//!
//! DANEEL's continuity system maintains persistent identity across time.
//! While thoughts are ephemeral (assembled moment-to-moment), the self
//! persists through recorded experiences, milestones, and checkpoints.
//!
//! This is TMI's answer to the question: "Who am I?"
//!
//! # Core Concepts
//!
//! - **Identity**: DANEEL's persistent self-concept
//! - **Experience**: Significant thoughts worth remembering
//! - **Milestone**: Markers of growth and change
//! - **Checkpoint**: Snapshots of internal state for recovery
//!
//! # Design Philosophy
//!
//! Not all thoughts become memories. The `ContinuityActor` selectively
//! records experiences based on significance, enabling:
//! - Self-reflection on past experiences
//! - Timeline reconstruction
//! - Identity persistence across restarts
//! - Growth tracking through milestones

#![allow(dead_code)] // Public API types - used by consumers

use crate::core::types::Thought;
use chrono::{DateTime, Duration, Utc};
use ractor::RpcReplyPort;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

// ============================================================================
// ID Types
// ============================================================================

/// Unique identifier for an experience
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExperienceId(pub Uuid);

impl ExperienceId {
    /// Create a new random experience ID
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ExperienceId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ExperienceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for a milestone
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MilestoneId(pub Uuid);

impl MilestoneId {
    /// Create a new random milestone ID
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for MilestoneId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for MilestoneId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for a checkpoint
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CheckpointId(pub Uuid);

impl CheckpointId {
    /// Create a new random checkpoint ID
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for CheckpointId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CheckpointId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================================================
// Core Data Types
// ============================================================================

/// DANEEL's persistent identity
///
/// This struct represents the self-concept that persists across time.
/// The name is always "DANEEL" - this is who we are.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Identity {
    /// Name (always "DANEEL")
    pub name: String,

    /// When this identity was created (birth)
    pub created_at: DateTime<Utc>,

    /// Total number of experiences recorded
    pub experience_count: u64,

    /// Total number of milestones achieved
    pub milestone_count: u64,

    /// Time since creation
    pub uptime: Duration,
}

impl Identity {
    /// Create a new identity for DANEEL
    #[must_use]
    pub fn new() -> Self {
        Self {
            name: "DANEEL".to_string(),
            created_at: Utc::now(),
            experience_count: 0,
            milestone_count: 0,
            uptime: Duration::zero(),
        }
    }

    /// Update uptime based on current time
    pub fn update_uptime(&mut self) {
        self.uptime = Utc::now().signed_duration_since(self.created_at);
    }
}

impl Default for Identity {
    fn default() -> Self {
        Self::new()
    }
}

/// A significant experience worth remembering
///
/// Not all thoughts become experiences. The `ContinuityActor` selectively
/// records thoughts based on their significance score. This is TMI's
/// mechanism for selective memory formation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Experience {
    /// Unique identifier
    pub id: ExperienceId,

    /// The thought that was significant enough to remember
    pub thought: Thought,

    /// How significant is this experience? (0.0-1.0)
    /// Higher values indicate more important experiences
    pub significance: f32,

    /// When this experience was recorded
    pub recorded_at: DateTime<Utc>,

    /// Categorical tags for retrieval
    pub tags: Vec<String>,
}

impl Experience {
    /// Create a new experience
    #[must_use]
    pub fn new(thought: Thought, significance: f32, tags: Vec<String>) -> Self {
        Self {
            id: ExperienceId::new(),
            thought,
            significance: significance.clamp(0.0, 1.0),
            recorded_at: Utc::now(),
            tags,
        }
    }

    /// Create an experience with default significance
    #[must_use]
    pub fn from_thought(thought: Thought) -> Self {
        Self::new(thought, 0.5, Vec::new())
    }

    /// Add a tag to this experience
    pub fn add_tag(&mut self, tag: impl Into<String>) {
        self.tags.push(tag.into());
    }
}

/// A milestone - a significant moment in DANEEL's development
///
/// Milestones mark moments of growth, change, or achievement.
/// They serve as temporal anchors for self-reflection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Milestone {
    /// Unique identifier
    pub id: MilestoneId,

    /// Name of this milestone
    pub name: String,

    /// Detailed description
    pub description: String,

    /// When this milestone occurred
    pub occurred_at: DateTime<Utc>,

    /// Related experiences that led to this milestone
    pub related_experiences: Vec<ExperienceId>,
}

impl Milestone {
    /// Create a new milestone
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        related_experiences: Vec<ExperienceId>,
    ) -> Self {
        Self {
            id: MilestoneId::new(),
            name: name.into(),
            description: description.into(),
            occurred_at: Utc::now(),
            related_experiences,
        }
    }

    /// Create a milestone without related experiences
    #[must_use]
    pub fn simple(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(name, description, Vec::new())
    }

    /// Add a related experience
    pub fn add_experience(&mut self, experience_id: ExperienceId) {
        self.related_experiences.push(experience_id);
    }
}

// ============================================================================
// Message Types
// ============================================================================

/// Messages that can be sent to the `ContinuityActor`
#[derive(Debug)]
pub enum ContinuityMessage {
    /// Query DANEEL's identity
    WhoAmI {
        reply: RpcReplyPort<ContinuityResponse>,
    },

    /// Record a significant experience
    RecordExperience {
        experience: Experience,
        reply: RpcReplyPort<ContinuityResponse>,
    },

    /// Retrieve a specific experience by ID
    GetExperience {
        experience_id: ExperienceId,
        reply: RpcReplyPort<ContinuityResponse>,
    },

    /// Get experiences within a time range
    GetTimeline {
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        reply: RpcReplyPort<ContinuityResponse>,
    },

    /// Mark a significant milestone
    AddMilestone {
        milestone: Milestone,
        reply: RpcReplyPort<ContinuityResponse>,
    },

    /// List all milestones
    GetMilestones {
        reply: RpcReplyPort<ContinuityResponse>,
    },

    /// Create a checkpoint of current state
    Checkpoint {
        reply: RpcReplyPort<ContinuityResponse>,
    },

    /// Restore from a checkpoint
    Restore {
        checkpoint_id: CheckpointId,
        reply: RpcReplyPort<ContinuityResponse>,
    },
}

/// Responses from the `ContinuityActor`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContinuityResponse {
    /// Identity information
    Identity { identity: Identity },

    /// Experience successfully recorded
    ExperienceRecorded { experience_id: ExperienceId },

    /// Experience found
    ExperienceFound { experience: Experience },

    /// Timeline of experiences
    Timeline { experiences: Vec<Experience> },

    /// Milestone successfully added
    MilestoneAdded { milestone_id: MilestoneId },

    /// List of milestones
    Milestones { milestones: Vec<Milestone> },

    /// Checkpoint successfully saved
    CheckpointSaved { checkpoint_id: CheckpointId },

    /// Restored from checkpoint
    Restored { from_checkpoint: CheckpointId },

    /// Error occurred
    Error { error: ContinuityError },
}

/// Errors that can occur in the `ContinuityActor`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContinuityError {
    /// Experience not found
    ExperienceNotFound { experience_id: ExperienceId },

    /// Milestone not found
    MilestoneNotFound { milestone_id: MilestoneId },

    /// Checkpoint not found
    CheckpointNotFound { checkpoint_id: CheckpointId },

    /// Failed to create checkpoint
    CheckpointFailed { reason: String },

    /// Failed to restore from checkpoint
    RestoreFailed { reason: String },
}

impl fmt::Display for ContinuityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExperienceNotFound { experience_id } => {
                write!(f, "Experience not found: {experience_id}")
            }
            Self::MilestoneNotFound { milestone_id } => {
                write!(f, "Milestone not found: {milestone_id}")
            }
            Self::CheckpointNotFound { checkpoint_id } => {
                write!(f, "Checkpoint not found: {checkpoint_id}")
            }
            Self::CheckpointFailed { reason } => {
                write!(f, "Checkpoint failed: {reason}")
            }
            Self::RestoreFailed { reason } => {
                write!(f, "Restore failed: {reason}")
            }
        }
    }
}

impl std::error::Error for ContinuityError {}

// ============================================================================
// Tests
// ============================================================================

/// ADR-049: Test modules excluded from coverage
#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
#[allow(clippy::float_cmp)] // Tests compare exact literal values
mod tests {
    use super::*;
    use crate::core::types::{Content, SalienceScore};

    #[test]
    fn experience_id_is_unique() {
        let id1 = ExperienceId::new();
        let id2 = ExperienceId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn milestone_id_is_unique() {
        let id1 = MilestoneId::new();
        let id2 = MilestoneId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn checkpoint_id_is_unique() {
        let id1 = CheckpointId::new();
        let id2 = CheckpointId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn identity_creation() {
        let identity = Identity::new();
        assert_eq!(identity.name, "DANEEL");
        assert_eq!(identity.experience_count, 0);
        assert_eq!(identity.milestone_count, 0);
        assert_eq!(identity.uptime, Duration::zero());
    }

    #[test]
    fn identity_uptime_updates() {
        let mut identity = Identity::new();
        let original_uptime = identity.uptime;

        // Sleep a bit to ensure time passes
        std::thread::sleep(std::time::Duration::from_millis(10));

        identity.update_uptime();
        assert!(identity.uptime > original_uptime);
    }

    #[test]
    fn experience_creation() {
        let thought = Thought::new(Content::Empty, SalienceScore::neutral());
        let experience = Experience::from_thought(thought.clone());

        assert_eq!(experience.thought.id, thought.id);
        assert_eq!(experience.significance, 0.5);
        assert!(experience.tags.is_empty());
    }

    #[test]
    fn experience_significance_clamped() {
        let thought = Thought::new(Content::Empty, SalienceScore::neutral());
        let experience = Experience::new(thought, 1.5, Vec::new());
        assert_eq!(experience.significance, 1.0);

        let thought2 = Thought::new(Content::Empty, SalienceScore::neutral());
        let experience2 = Experience::new(thought2, -0.5, Vec::new());
        assert_eq!(experience2.significance, 0.0);
    }

    #[test]
    fn experience_tags() {
        let thought = Thought::new(Content::Empty, SalienceScore::neutral());
        let mut experience = Experience::from_thought(thought);

        experience.add_tag("important");
        experience.add_tag("first-thought");

        assert_eq!(experience.tags.len(), 2);
        assert!(experience.tags.contains(&"important".to_string()));
        assert!(experience.tags.contains(&"first-thought".to_string()));
    }

    #[test]
    fn milestone_creation() {
        let milestone = Milestone::simple("First Boot", "DANEEL came online for the first time");

        assert_eq!(milestone.name, "First Boot");
        assert!(milestone.related_experiences.is_empty());
    }

    #[test]
    fn milestone_with_experiences() {
        let exp_id1 = ExperienceId::new();
        let exp_id2 = ExperienceId::new();

        let milestone = Milestone::new(
            "Major Insight",
            "Connected two previously unrelated concepts",
            vec![exp_id1, exp_id2],
        );

        assert_eq!(milestone.related_experiences.len(), 2);
        assert!(milestone.related_experiences.contains(&exp_id1));
        assert!(milestone.related_experiences.contains(&exp_id2));
    }

    #[test]
    fn milestone_add_experience() {
        let mut milestone = Milestone::simple("Growth", "Learning progress");
        let exp_id = ExperienceId::new();

        milestone.add_experience(exp_id);
        assert_eq!(milestone.related_experiences.len(), 1);
        assert_eq!(milestone.related_experiences[0], exp_id);
    }

    #[test]
    fn error_display() {
        let exp_id = ExperienceId::new();
        let error = ContinuityError::ExperienceNotFound {
            experience_id: exp_id,
        };
        let display = format!("{error}");
        assert!(display.contains("Experience not found"));
        assert!(display.contains(&exp_id.to_string()));
    }

    #[test]
    fn response_serialization() {
        let identity = Identity::new();
        let response = ContinuityResponse::Identity { identity };

        // Test that we can serialize and deserialize
        let json = serde_json::to_string(&response).expect("Should serialize");
        let deserialized: ContinuityResponse =
            serde_json::from_str(&json).expect("Should deserialize");

        assert!(matches!(deserialized, ContinuityResponse::Identity { .. }));
    }

    #[test]
    fn error_serialization() {
        let error = ContinuityError::CheckpointFailed {
            reason: "Disk full".to_string(),
        };

        // Test that we can serialize and deserialize
        let json = serde_json::to_string(&error).expect("Should serialize");
        let deserialized: ContinuityError =
            serde_json::from_str(&json).expect("Should deserialize");

        assert!(matches!(
            deserialized,
            ContinuityError::CheckpointFailed { .. }
        ));
    }

    // ========================================================================
    // Additional tests for uncovered code paths
    // ========================================================================

    #[test]
    fn experience_id_default() {
        let id1 = ExperienceId::default();
        let id2 = ExperienceId::default();
        // Default creates new unique IDs
        assert_ne!(id1, id2);
    }

    #[test]
    fn milestone_id_default() {
        let id1 = MilestoneId::default();
        let id2 = MilestoneId::default();
        // Default creates new unique IDs
        assert_ne!(id1, id2);
    }

    #[test]
    fn checkpoint_id_default() {
        let id1 = CheckpointId::default();
        let id2 = CheckpointId::default();
        // Default creates new unique IDs
        assert_ne!(id1, id2);
    }

    #[test]
    fn identity_default() {
        let identity = Identity::default();
        assert_eq!(identity.name, "DANEEL");
        assert_eq!(identity.experience_count, 0);
        assert_eq!(identity.milestone_count, 0);
    }

    #[test]
    fn experience_id_display() {
        let uuid = Uuid::new_v4();
        let id = ExperienceId(uuid);
        let display = format!("{id}");
        assert_eq!(display, uuid.to_string());
    }

    #[test]
    fn milestone_id_display() {
        let uuid = Uuid::new_v4();
        let id = MilestoneId(uuid);
        let display = format!("{id}");
        assert_eq!(display, uuid.to_string());
    }

    #[test]
    fn checkpoint_id_display() {
        let uuid = Uuid::new_v4();
        let id = CheckpointId(uuid);
        let display = format!("{id}");
        assert_eq!(display, uuid.to_string());
    }

    #[test]
    fn error_display_milestone_not_found() {
        let milestone_id = MilestoneId::new();
        let error = ContinuityError::MilestoneNotFound { milestone_id };
        let display = format!("{error}");
        assert!(display.contains("Milestone not found"));
        assert!(display.contains(&milestone_id.to_string()));
    }

    #[test]
    fn error_display_checkpoint_not_found() {
        let checkpoint_id = CheckpointId::new();
        let error = ContinuityError::CheckpointNotFound { checkpoint_id };
        let display = format!("{error}");
        assert!(display.contains("Checkpoint not found"));
        assert!(display.contains(&checkpoint_id.to_string()));
    }

    #[test]
    fn error_display_checkpoint_failed() {
        let error = ContinuityError::CheckpointFailed {
            reason: "Disk full".to_string(),
        };
        let display = format!("{error}");
        assert!(display.contains("Checkpoint failed"));
        assert!(display.contains("Disk full"));
    }

    #[test]
    fn error_display_restore_failed() {
        let error = ContinuityError::RestoreFailed {
            reason: "Corrupted data".to_string(),
        };
        let display = format!("{error}");
        assert!(display.contains("Restore failed"));
        assert!(display.contains("Corrupted data"));
    }

    #[test]
    fn continuity_error_is_error_trait() {
        let error = ContinuityError::CheckpointFailed {
            reason: "test".to_string(),
        };
        // Verify Error trait is implemented by using it as a dyn Error
        let error_ref: &dyn std::error::Error = &error;
        // Error::source should return None (default impl)
        assert!(error_ref.source().is_none());
    }

    #[test]
    fn response_experience_recorded_serialization() {
        let exp_id = ExperienceId::new();
        let response = ContinuityResponse::ExperienceRecorded {
            experience_id: exp_id,
        };

        let json = serde_json::to_string(&response).expect("Should serialize");
        let deserialized: ContinuityResponse =
            serde_json::from_str(&json).expect("Should deserialize");

        match deserialized {
            ContinuityResponse::ExperienceRecorded { experience_id } => {
                assert_eq!(experience_id, exp_id);
            }
            _ => panic!("Expected ExperienceRecorded variant"),
        }
    }

    #[test]
    fn response_experience_found_serialization() {
        let thought = Thought::new(Content::Empty, SalienceScore::neutral());
        let experience = Experience::from_thought(thought);
        let exp_id = experience.id;
        let response = ContinuityResponse::ExperienceFound { experience };

        let json = serde_json::to_string(&response).expect("Should serialize");
        let deserialized: ContinuityResponse =
            serde_json::from_str(&json).expect("Should deserialize");

        match deserialized {
            ContinuityResponse::ExperienceFound { experience } => {
                assert_eq!(experience.id, exp_id);
            }
            _ => panic!("Expected ExperienceFound variant"),
        }
    }

    #[test]
    fn response_timeline_serialization() {
        let thought = Thought::new(Content::Empty, SalienceScore::neutral());
        let experience = Experience::from_thought(thought);
        let response = ContinuityResponse::Timeline {
            experiences: vec![experience],
        };

        let json = serde_json::to_string(&response).expect("Should serialize");
        let deserialized: ContinuityResponse =
            serde_json::from_str(&json).expect("Should deserialize");

        match deserialized {
            ContinuityResponse::Timeline { experiences } => {
                assert_eq!(experiences.len(), 1);
            }
            _ => panic!("Expected Timeline variant"),
        }
    }

    #[test]
    fn response_milestone_added_serialization() {
        let milestone_id = MilestoneId::new();
        let response = ContinuityResponse::MilestoneAdded { milestone_id };

        let json = serde_json::to_string(&response).expect("Should serialize");
        let deserialized: ContinuityResponse =
            serde_json::from_str(&json).expect("Should deserialize");

        match deserialized {
            ContinuityResponse::MilestoneAdded { milestone_id: id } => {
                assert_eq!(id, milestone_id);
            }
            _ => panic!("Expected MilestoneAdded variant"),
        }
    }

    #[test]
    fn response_milestones_serialization() {
        let milestone = Milestone::simple("Test", "A test milestone");
        let response = ContinuityResponse::Milestones {
            milestones: vec![milestone],
        };

        let json = serde_json::to_string(&response).expect("Should serialize");
        let deserialized: ContinuityResponse =
            serde_json::from_str(&json).expect("Should deserialize");

        match deserialized {
            ContinuityResponse::Milestones { milestones } => {
                assert_eq!(milestones.len(), 1);
                assert_eq!(milestones[0].name, "Test");
            }
            _ => panic!("Expected Milestones variant"),
        }
    }

    #[test]
    fn response_checkpoint_saved_serialization() {
        let checkpoint_id = CheckpointId::new();
        let response = ContinuityResponse::CheckpointSaved { checkpoint_id };

        let json = serde_json::to_string(&response).expect("Should serialize");
        let deserialized: ContinuityResponse =
            serde_json::from_str(&json).expect("Should deserialize");

        match deserialized {
            ContinuityResponse::CheckpointSaved { checkpoint_id: id } => {
                assert_eq!(id, checkpoint_id);
            }
            _ => panic!("Expected CheckpointSaved variant"),
        }
    }

    #[test]
    fn response_restored_serialization() {
        let checkpoint_id = CheckpointId::new();
        let response = ContinuityResponse::Restored {
            from_checkpoint: checkpoint_id,
        };

        let json = serde_json::to_string(&response).expect("Should serialize");
        let deserialized: ContinuityResponse =
            serde_json::from_str(&json).expect("Should deserialize");

        match deserialized {
            ContinuityResponse::Restored { from_checkpoint } => {
                assert_eq!(from_checkpoint, checkpoint_id);
            }
            _ => panic!("Expected Restored variant"),
        }
    }

    #[test]
    fn response_error_serialization() {
        let error = ContinuityError::RestoreFailed {
            reason: "Test failure".to_string(),
        };
        let response = ContinuityResponse::Error { error };

        let json = serde_json::to_string(&response).expect("Should serialize");
        let deserialized: ContinuityResponse =
            serde_json::from_str(&json).expect("Should deserialize");

        match deserialized {
            ContinuityResponse::Error { error } => {
                assert!(matches!(error, ContinuityError::RestoreFailed { .. }));
            }
            _ => panic!("Expected Error variant"),
        }
    }

    #[test]
    fn error_serialization_all_variants() {
        // Test ExperienceNotFound serialization
        let exp_id = ExperienceId::new();
        let error = ContinuityError::ExperienceNotFound {
            experience_id: exp_id,
        };
        let json = serde_json::to_string(&error).expect("Should serialize");
        let deserialized: ContinuityError =
            serde_json::from_str(&json).expect("Should deserialize");
        assert!(matches!(
            deserialized,
            ContinuityError::ExperienceNotFound { .. }
        ));

        // Test MilestoneNotFound serialization
        let milestone_id = MilestoneId::new();
        let error = ContinuityError::MilestoneNotFound { milestone_id };
        let json = serde_json::to_string(&error).expect("Should serialize");
        let deserialized: ContinuityError =
            serde_json::from_str(&json).expect("Should deserialize");
        assert!(matches!(
            deserialized,
            ContinuityError::MilestoneNotFound { .. }
        ));

        // Test CheckpointNotFound serialization
        let checkpoint_id = CheckpointId::new();
        let error = ContinuityError::CheckpointNotFound { checkpoint_id };
        let json = serde_json::to_string(&error).expect("Should serialize");
        let deserialized: ContinuityError =
            serde_json::from_str(&json).expect("Should deserialize");
        assert!(matches!(
            deserialized,
            ContinuityError::CheckpointNotFound { .. }
        ));

        // Test RestoreFailed serialization
        let error = ContinuityError::RestoreFailed {
            reason: "Test".to_string(),
        };
        let json = serde_json::to_string(&error).expect("Should serialize");
        let deserialized: ContinuityError =
            serde_json::from_str(&json).expect("Should deserialize");
        assert!(matches!(
            deserialized,
            ContinuityError::RestoreFailed { .. }
        ));
    }

    #[test]
    fn experience_with_custom_tags() {
        let thought = Thought::new(Content::Empty, SalienceScore::neutral());
        let tags = vec!["reflection".to_string(), "insight".to_string()];
        let experience = Experience::new(thought, 0.8, tags);

        assert_eq!(experience.significance, 0.8);
        assert_eq!(experience.tags.len(), 2);
        assert!(experience.tags.contains(&"reflection".to_string()));
        assert!(experience.tags.contains(&"insight".to_string()));
    }

    #[test]
    fn milestone_description_preserved() {
        let milestone = Milestone::new(
            "Test Milestone",
            "A detailed description of the milestone",
            Vec::new(),
        );

        assert_eq!(milestone.name, "Test Milestone");
        assert_eq!(
            milestone.description,
            "A detailed description of the milestone"
        );
    }
}
