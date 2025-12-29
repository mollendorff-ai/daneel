//! `ContinuityActor` - Âncora da Memória (Memory Anchor)
//!
//! Implements TMI's identity persistence and memory anchor system using Ractor actor model.
//!
//! # TMI Concept
//!
//! From Cury's Theory of Multifocal Intelligence:
//! - Identity persists across time while thoughts are ephemeral
//! - Significant experiences become anchored memories
//! - Milestones mark growth and development
//! - Checkpoints enable continuity across restarts
//!
//! This is TMI's answer to the question: "Who am I?"
//!
//! # Core Concepts
//!
//! - **Identity**: DANEEL's persistent self-concept (always named "DANEEL")
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
//!
//! # Usage
//!
//! ```no_run
//! use daneel::actors::continuity::{ContinuityActor, ContinuityMessage};
//! use ractor::Actor;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Spawn the actor
//! let (actor_ref, _) = Actor::spawn(None, ContinuityActor, ()).await?;
//!
//! // Query identity
//! let response = actor_ref.call(|reply| ContinuityMessage::WhoAmI { reply }, None).await?;
//!
//! // Record an experience
//! // let experience = Experience::from_thought(some_thought);
//! // let response = actor_ref.call(|reply| ContinuityMessage::RecordExperience {
//! //     experience,
//! //     reply,
//! // }, None).await?;
//! # Ok(())
//! # }
//! ```

pub mod types;

#[cfg(test)]
mod tests;

use chrono::{DateTime, Utc};
use ractor::{Actor, ActorProcessingErr, ActorRef};
use std::collections::HashMap;

// Re-export types for public API
pub use types::{
    CheckpointId, ContinuityError, ContinuityMessage, ContinuityResponse, Experience, ExperienceId,
    Identity, Milestone, MilestoneId,
};

/// Checkpoint - A snapshot of DANEEL's continuity state
///
/// Checkpoints enable recovery and continuity across restarts.
/// Currently in-memory only (persistence comes in Phase 2).
#[derive(Debug, Clone, PartialEq)]
pub struct Checkpoint {
    /// Unique identifier for this checkpoint
    pub id: CheckpointId,

    /// When this checkpoint was created
    pub created_at: DateTime<Utc>,

    /// Number of experiences at checkpoint time
    pub experience_count: u64,

    /// Number of milestones at checkpoint time
    pub milestone_count: u64,

    /// Snapshot of identity at checkpoint time
    identity: Identity,

    /// Snapshot of experiences
    experiences: HashMap<ExperienceId, Experience>,

    /// Snapshot of milestones
    milestones: Vec<Milestone>,
}

impl Checkpoint {
    /// Create a new checkpoint from current state
    #[must_use]
    fn from_state(state: &ContinuityState) -> Self {
        Self {
            id: CheckpointId::new(),
            created_at: Utc::now(),
            experience_count: state.identity.experience_count,
            milestone_count: state.identity.milestone_count,
            identity: state.identity.clone(),
            experiences: state.experiences.clone(),
            milestones: state.milestones.clone(),
        }
    }
}

/// Continuity Actor State
///
/// Maintains DANEEL's persistent identity and memory anchor.
#[derive(Debug)]
pub struct ContinuityState {
    /// DANEEL's persistent identity
    identity: Identity,

    /// Recorded experiences (`ExperienceId` -> Experience)
    experiences: HashMap<ExperienceId, Experience>,

    /// Growth milestones (chronological order)
    milestones: Vec<Milestone>,

    /// Saved checkpoints (`CheckpointId` -> Checkpoint)
    checkpoints: HashMap<CheckpointId, Checkpoint>,
}

impl ContinuityState {
    /// Create new continuity state with default DANEEL identity
    fn new() -> Self {
        Self {
            identity: Identity::new(),
            experiences: HashMap::new(),
            milestones: Vec::new(),
            checkpoints: HashMap::new(),
        }
    }

    /// Create continuity state with a specific identity
    #[must_use]
    #[allow(dead_code)] // Public API for future use
    fn with_identity(identity: Identity) -> Self {
        Self {
            identity,
            experiences: HashMap::new(),
            milestones: Vec::new(),
            checkpoints: HashMap::new(),
        }
    }

    /// Get current identity with updated uptime
    fn get_identity(&mut self) -> Identity {
        self.identity.update_uptime();
        self.identity.clone()
    }

    /// Record a significant experience
    fn record_experience(&mut self, experience: Experience) -> ExperienceId {
        let experience_id = experience.id;
        self.experiences.insert(experience_id, experience);
        self.identity.experience_count += 1;
        experience_id
    }

    /// Retrieve a specific experience by ID
    fn get_experience(&self, experience_id: ExperienceId) -> Result<Experience, ContinuityError> {
        self.experiences
            .get(&experience_id)
            .cloned()
            .ok_or(ContinuityError::ExperienceNotFound { experience_id })
    }

    /// Get experiences within a time range
    fn get_timeline(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<Experience> {
        self.experiences
            .values()
            .filter(|exp| exp.recorded_at >= start && exp.recorded_at <= end)
            .cloned()
            .collect()
    }

    /// Add a milestone
    fn add_milestone(&mut self, milestone: Milestone) -> MilestoneId {
        let milestone_id = milestone.id;
        self.milestones.push(milestone);
        self.identity.milestone_count += 1;
        milestone_id
    }

    /// Get all milestones
    fn get_milestones(&self) -> Vec<Milestone> {
        self.milestones.clone()
    }

    /// Create a checkpoint of current state
    fn create_checkpoint(&mut self) -> CheckpointId {
        let checkpoint = Checkpoint::from_state(self);
        let checkpoint_id = checkpoint.id;
        self.checkpoints.insert(checkpoint_id, checkpoint);
        checkpoint_id
    }

    /// Restore from a checkpoint
    fn restore_checkpoint(&mut self, checkpoint_id: CheckpointId) -> Result<(), ContinuityError> {
        let checkpoint = self
            .checkpoints
            .get(&checkpoint_id)
            .ok_or(ContinuityError::CheckpointNotFound { checkpoint_id })?;

        // Restore state from checkpoint
        self.identity = checkpoint.identity.clone();
        self.experiences = checkpoint.experiences.clone();
        self.milestones = checkpoint.milestones.clone();

        Ok(())
    }
}

/// The Continuity Actor
///
/// Implements identity persistence and memory anchoring as a Ractor actor.
pub struct ContinuityActor;

#[ractor::async_trait]
impl Actor for ContinuityActor {
    type Msg = ContinuityMessage;
    type State = ContinuityState;
    type Arguments = ();

    #[cfg_attr(coverage_nightly, coverage(off))]
    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(ContinuityState::new())
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            ContinuityMessage::WhoAmI { reply } => {
                let identity = state.get_identity();
                let response = ContinuityResponse::Identity { identity };
                let _ = reply.send(response);
            }

            ContinuityMessage::RecordExperience { experience, reply } => {
                let experience_id = state.record_experience(experience);
                let response = ContinuityResponse::ExperienceRecorded { experience_id };
                let _ = reply.send(response);
            }

            ContinuityMessage::GetExperience {
                experience_id,
                reply,
            } => {
                let response = match state.get_experience(experience_id) {
                    Ok(experience) => ContinuityResponse::ExperienceFound { experience },
                    Err(error) => ContinuityResponse::Error { error },
                };
                let _ = reply.send(response);
            }

            ContinuityMessage::GetTimeline { start, end, reply } => {
                let experiences = state.get_timeline(start, end);
                let response = ContinuityResponse::Timeline { experiences };
                let _ = reply.send(response);
            }

            ContinuityMessage::AddMilestone { milestone, reply } => {
                let milestone_id = state.add_milestone(milestone);
                let response = ContinuityResponse::MilestoneAdded { milestone_id };
                let _ = reply.send(response);
            }

            ContinuityMessage::GetMilestones { reply } => {
                let milestones = state.get_milestones();
                let response = ContinuityResponse::Milestones { milestones };
                let _ = reply.send(response);
            }

            ContinuityMessage::Checkpoint { reply } => {
                let checkpoint_id = state.create_checkpoint();
                let response = ContinuityResponse::CheckpointSaved { checkpoint_id };
                let _ = reply.send(response);
            }

            ContinuityMessage::Restore {
                checkpoint_id,
                reply,
            } => {
                let response = match state.restore_checkpoint(checkpoint_id) {
                    Ok(()) => ContinuityResponse::Restored {
                        from_checkpoint: checkpoint_id,
                    },
                    Err(error) => ContinuityResponse::Error { error },
                };
                let _ = reply.send(response);
            }
        }

        Ok(())
    }
}

// ============================================================================
// Inline Unit Tests for Pure State Logic
// ============================================================================

/// ADR-049: Test modules excluded from coverage
#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
#[allow(clippy::cast_precision_loss)] // Test calculations
mod state_tests {
    use super::*;
    use crate::core::types::{Content, SalienceScore, Thought};
    use chrono::Duration;

    // ------------------------------------------------------------------------
    // ContinuityState tests
    // ------------------------------------------------------------------------

    #[test]
    fn state_new_creates_default_identity() {
        let state = ContinuityState::new();
        assert_eq!(state.identity.name, "DANEEL");
        assert_eq!(state.identity.experience_count, 0);
        assert_eq!(state.identity.milestone_count, 0);
        assert!(state.experiences.is_empty());
        assert!(state.milestones.is_empty());
        assert!(state.checkpoints.is_empty());
    }

    #[test]
    fn state_with_identity_uses_provided_identity() {
        let mut custom_identity = Identity::new();
        custom_identity.experience_count = 42;
        custom_identity.milestone_count = 7;

        let state = ContinuityState::with_identity(custom_identity);
        assert_eq!(state.identity.experience_count, 42);
        assert_eq!(state.identity.milestone_count, 7);
        assert!(state.experiences.is_empty());
        assert!(state.milestones.is_empty());
    }

    #[test]
    fn state_get_identity_updates_uptime() {
        let mut state = ContinuityState::new();
        let original_uptime = state.identity.uptime;

        // Wait a bit
        std::thread::sleep(std::time::Duration::from_millis(10));

        let identity = state.get_identity();
        assert!(identity.uptime > original_uptime);
    }

    #[test]
    fn state_record_experience_increments_count() {
        let mut state = ContinuityState::new();
        assert_eq!(state.identity.experience_count, 0);

        let thought = Thought::new(Content::Empty, SalienceScore::neutral());
        let experience = Experience::from_thought(thought);
        let exp_id = experience.id;

        let returned_id = state.record_experience(experience);
        assert_eq!(returned_id, exp_id);
        assert_eq!(state.identity.experience_count, 1);
        assert!(state.experiences.contains_key(&exp_id));
    }

    #[test]
    fn state_get_experience_found() {
        let mut state = ContinuityState::new();
        let thought = Thought::new(Content::Empty, SalienceScore::neutral());
        let experience = Experience::from_thought(thought);
        let exp_id = experience.id;
        state.record_experience(experience);

        let result = state.get_experience(exp_id);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, exp_id);
    }

    #[test]
    fn state_get_experience_not_found() {
        let state = ContinuityState::new();
        let nonexistent_id = ExperienceId::new();

        let result = state.get_experience(nonexistent_id);
        assert!(result.is_err());
        match result {
            Err(ContinuityError::ExperienceNotFound { experience_id }) => {
                assert_eq!(experience_id, nonexistent_id);
            }
            _ => panic!("Expected ExperienceNotFound error"),
        }
    }

    #[test]
    fn state_get_timeline_filters_by_range() {
        let mut state = ContinuityState::new();
        let now = Utc::now();

        // Experience inside range
        let thought1 = Thought::new(Content::Empty, SalienceScore::neutral());
        let mut exp1 = Experience::from_thought(thought1);
        exp1.recorded_at = now - Duration::minutes(30);
        let exp1_id = exp1.id;
        state.record_experience(exp1);

        // Experience outside range (too old)
        let thought2 = Thought::new(Content::Empty, SalienceScore::neutral());
        let mut exp2 = Experience::from_thought(thought2);
        exp2.recorded_at = now - Duration::hours(5);
        state.record_experience(exp2);

        // Experience inside range
        let thought3 = Thought::new(Content::Empty, SalienceScore::neutral());
        let mut exp3 = Experience::from_thought(thought3);
        exp3.recorded_at = now - Duration::minutes(10);
        let exp3_id = exp3.id;
        state.record_experience(exp3);

        let start = now - Duration::hours(1);
        let end = now + Duration::minutes(5);
        let timeline = state.get_timeline(start, end);

        assert_eq!(timeline.len(), 2);
        let ids: Vec<ExperienceId> = timeline.iter().map(|e| e.id).collect();
        assert!(ids.contains(&exp1_id));
        assert!(ids.contains(&exp3_id));
    }

    #[test]
    fn state_get_timeline_empty_when_no_matches() {
        let mut state = ContinuityState::new();
        let now = Utc::now();

        // Experience outside range
        let thought = Thought::new(Content::Empty, SalienceScore::neutral());
        let mut exp = Experience::from_thought(thought);
        exp.recorded_at = now - Duration::hours(5);
        state.record_experience(exp);

        let start = now - Duration::hours(1);
        let end = now + Duration::minutes(5);
        let timeline = state.get_timeline(start, end);

        assert!(timeline.is_empty());
    }

    #[test]
    fn state_add_milestone_increments_count() {
        let mut state = ContinuityState::new();
        assert_eq!(state.identity.milestone_count, 0);

        let milestone = Milestone::simple("First Boot", "DANEEL came online");
        let milestone_id = milestone.id;

        let returned_id = state.add_milestone(milestone);
        assert_eq!(returned_id, milestone_id);
        assert_eq!(state.identity.milestone_count, 1);
        assert_eq!(state.milestones.len(), 1);
    }

    #[test]
    fn state_get_milestones_returns_all() {
        let mut state = ContinuityState::new();

        let m1 = Milestone::simple("First", "First milestone");
        let m2 = Milestone::simple("Second", "Second milestone");
        let m1_id = m1.id;
        let m2_id = m2.id;

        state.add_milestone(m1);
        state.add_milestone(m2);

        let milestones = state.get_milestones();
        assert_eq!(milestones.len(), 2);
        assert_eq!(milestones[0].id, m1_id);
        assert_eq!(milestones[1].id, m2_id);
    }

    #[test]
    fn state_create_checkpoint_saves_state() {
        let mut state = ContinuityState::new();

        // Add some experiences and milestones
        let thought = Thought::new(Content::Empty, SalienceScore::neutral());
        let experience = Experience::from_thought(thought);
        state.record_experience(experience);

        let milestone = Milestone::simple("Test", "Test milestone");
        state.add_milestone(milestone);

        let checkpoint_id = state.create_checkpoint();
        assert!(state.checkpoints.contains_key(&checkpoint_id));

        let checkpoint = state.checkpoints.get(&checkpoint_id).unwrap();
        assert_eq!(checkpoint.experience_count, 1);
        assert_eq!(checkpoint.milestone_count, 1);
    }

    #[test]
    fn state_restore_checkpoint_success() {
        let mut state = ContinuityState::new();

        // Add initial experience
        let thought1 = Thought::new(Content::Empty, SalienceScore::neutral());
        let exp1 = Experience::from_thought(thought1);
        let exp1_id = exp1.id;
        state.record_experience(exp1);

        // Create checkpoint
        let checkpoint_id = state.create_checkpoint();
        assert_eq!(state.identity.experience_count, 1);

        // Add more experiences after checkpoint
        let thought2 = Thought::new(Content::Empty, SalienceScore::neutral());
        let exp2 = Experience::from_thought(thought2);
        state.record_experience(exp2);
        assert_eq!(state.identity.experience_count, 2);

        // Restore checkpoint
        let result = state.restore_checkpoint(checkpoint_id);
        assert!(result.is_ok());
        assert_eq!(state.identity.experience_count, 1);
        assert!(state.experiences.contains_key(&exp1_id));
    }

    #[test]
    fn state_restore_checkpoint_not_found() {
        let mut state = ContinuityState::new();
        let nonexistent_id = CheckpointId::new();

        let result = state.restore_checkpoint(nonexistent_id);
        assert!(result.is_err());
        match result {
            Err(ContinuityError::CheckpointNotFound { checkpoint_id }) => {
                assert_eq!(checkpoint_id, nonexistent_id);
            }
            _ => panic!("Expected CheckpointNotFound error"),
        }
    }

    // ------------------------------------------------------------------------
    // Checkpoint tests
    // ------------------------------------------------------------------------

    #[test]
    fn checkpoint_from_state_captures_all_data() {
        let mut state = ContinuityState::new();

        // Add experiences
        for i in 0..3 {
            let thought = Thought::new(Content::Empty, SalienceScore::neutral());
            let mut exp = Experience::from_thought(thought);
            exp.significance = (i as f32).mul_add(0.1, 0.5);
            state.record_experience(exp);
        }

        // Add milestones
        let milestone = Milestone::simple("Test", "Test milestone");
        state.add_milestone(milestone);

        let checkpoint = Checkpoint::from_state(&state);

        assert_eq!(checkpoint.experience_count, 3);
        assert_eq!(checkpoint.milestone_count, 1);
        assert_eq!(checkpoint.identity.name, "DANEEL");
        assert_eq!(checkpoint.experiences.len(), 3);
        assert_eq!(checkpoint.milestones.len(), 1);
    }

    #[test]
    fn checkpoint_has_unique_id() {
        let state = ContinuityState::new();
        let checkpoint1 = Checkpoint::from_state(&state);
        let checkpoint2 = Checkpoint::from_state(&state);

        assert_ne!(checkpoint1.id, checkpoint2.id);
    }

    #[test]
    fn checkpoint_clone_preserves_data() {
        let mut state = ContinuityState::new();
        let thought = Thought::new(Content::Empty, SalienceScore::neutral());
        let exp = Experience::from_thought(thought);
        state.record_experience(exp);

        let checkpoint = Checkpoint::from_state(&state);
        let cloned = checkpoint.clone();

        assert_eq!(cloned.id, checkpoint.id);
        assert_eq!(cloned.experience_count, checkpoint.experience_count);
        assert_eq!(cloned.milestone_count, checkpoint.milestone_count);
        assert_eq!(cloned.experiences.len(), checkpoint.experiences.len());
    }

    #[test]
    fn checkpoint_equality() {
        let state = ContinuityState::new();
        let checkpoint = Checkpoint::from_state(&state);
        let same_checkpoint = checkpoint.clone();

        assert_eq!(checkpoint, same_checkpoint);
    }
}
