//! ContinuityActor - Âncora da Memória (Memory Anchor)
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
//! Not all thoughts become memories. The ContinuityActor selectively
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

// TODO: Create tests module
// #[cfg(test)]
// mod tests;

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

    /// Recorded experiences (ExperienceId -> Experience)
    experiences: HashMap<ExperienceId, Experience>,

    /// Growth milestones (chronological order)
    milestones: Vec<Milestone>,

    /// Saved checkpoints (CheckpointId -> Checkpoint)
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

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(ContinuityState::new())
    }

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
