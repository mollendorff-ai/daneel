//! ContinuityActor - Âncora da Memória + Identity
//!
//! Implements TMI's identity persistence:
//! - Maintains DANEEL's persistent self-concept across time
//! - Records significant experiences (Memory Anchor)
//! - Tracks milestones and growth
//! - Enables checkpoint/restore for continuity
//!
//! This is what gives DANEEL a persistent "I" across restarts.

pub mod types;

// Re-export types for public API
pub use types::{
    CheckpointId, ContinuityError, ContinuityMessage, ContinuityResponse, Experience, ExperienceId,
    Identity, Milestone, MilestoneId,
};
