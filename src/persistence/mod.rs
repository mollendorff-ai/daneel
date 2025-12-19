//! Persistence Layer for DANEEL
//!
//! Provides Redis-based persistence for Timmy's memory to survive restarts.
//!
//! # TMI Concept
//!
//! In biological minds, memories persist through sleep and even unconsciousness.
//! DANEEL needs the same - continuity across restarts.
//!
//! # Storage Schema
//!
//! ```text
//! daneel:identity          -> JSON Identity
//! daneel:experiences:{id}  -> JSON Experience
//! daneel:milestones:{id}   -> JSON Milestone
//! daneel:checkpoint:latest -> JSON full state snapshot
//! daneel:checkpoint:{id}   -> JSON checkpoint snapshot
//! ```
//!
//! # Usage
//!
//! ```ignore
//! use daneel::persistence::MemoryStore;
//! use daneel::actors::continuity::types::Identity;
//!
//! async fn example() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut store = MemoryStore::connect("redis://127.0.0.1:6379").await?;
//!     let identity = Identity::new();
//!
//!     // Save identity
//!     store.save_identity(&identity).await?;
//!
//!     // Load identity (returns None if not found)
//!     let loaded = store.load_identity().await?;
//!     Ok(())
//! }
//! ```

use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, Client};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use tracing::{debug, info, warn};

use crate::actors::continuity::types::{
    CheckpointId, Experience, ExperienceId, Identity, Milestone, MilestoneId,
};

// =============================================================================
// Error Types
// =============================================================================

/// Errors that can occur during persistence operations
#[derive(Debug, Error)]
pub enum PersistenceError {
    /// Redis connection failed
    #[error("Connection failed: {reason}")]
    ConnectionFailed { reason: String },

    /// Serialization failed
    #[error("Serialization failed: {reason}")]
    SerializationFailed { reason: String },

    /// Deserialization failed
    #[error("Deserialization failed: {reason}")]
    DeserializationFailed { reason: String },

    /// Redis operation failed
    #[error("Redis operation failed: {reason}")]
    OperationFailed { reason: String },

    /// Data not found
    #[error("Not found: {key}")]
    NotFound { key: String },
}

impl From<redis::RedisError> for PersistenceError {
    fn from(e: redis::RedisError) -> Self {
        Self::OperationFailed {
            reason: e.to_string(),
        }
    }
}

impl From<serde_json::Error> for PersistenceError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerializationFailed {
            reason: e.to_string(),
        }
    }
}

// =============================================================================
// Redis Keys
// =============================================================================

/// Key prefixes for DANEEL's memory storage
mod keys {
    pub const PREFIX: &str = "daneel";
    pub const IDENTITY: &str = "daneel:identity";
    pub const EXPERIENCES: &str = "daneel:experiences";
    pub const MILESTONES: &str = "daneel:milestones";
    pub const CHECKPOINT_LATEST: &str = "daneel:checkpoint:latest";
    pub const CHECKPOINTS: &str = "daneel:checkpoints";
    pub const EXPERIENCE_INDEX: &str = "daneel:experience_ids";
    pub const MILESTONE_INDEX: &str = "daneel:milestone_ids";
}

// =============================================================================
// Checkpoint State (for full state snapshots)
// =============================================================================

/// Full state snapshot for checkpoint persistence
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CheckpointState {
    pub identity: Identity,
    pub experiences: HashMap<ExperienceId, Experience>,
    pub milestones: Vec<Milestone>,
    pub checkpoint_id: CheckpointId,
    pub saved_at: chrono::DateTime<chrono::Utc>,
}

// =============================================================================
// Memory Store
// =============================================================================

/// Redis-based persistence for DANEEL's memory
///
/// Provides durable storage for identity, experiences, and milestones.
/// Timmy needs this to remember across restarts.
pub struct MemoryStore {
    /// Redis client
    #[allow(dead_code)]
    client: Client,

    /// Multiplexed async connection
    conn: MultiplexedConnection,
}

impl MemoryStore {
    // =========================================================================
    // Connection
    // =========================================================================

    /// Connect to Redis
    pub async fn connect(url: &str) -> Result<Self, PersistenceError> {
        info!("MemoryStore connecting to Redis at {}", url);
        let client = Client::open(url).map_err(|e| PersistenceError::ConnectionFailed {
            reason: e.to_string(),
        })?;
        let conn = client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| PersistenceError::ConnectionFailed {
                reason: e.to_string(),
            })?;
        info!("MemoryStore connected successfully");
        Ok(Self { client, conn })
    }

    // =========================================================================
    // Generic Helpers
    // =========================================================================

    /// Save a value as JSON to a key
    async fn save_json<T: Serialize>(
        &mut self,
        key: &str,
        value: &T,
    ) -> Result<(), PersistenceError> {
        let json = serde_json::to_string(value)?;
        let _: () = self.conn.set(key, json).await?;
        debug!("Saved to {}", key);
        Ok(())
    }

    /// Load a value from JSON key
    async fn load_json<T: DeserializeOwned>(
        &mut self,
        key: &str,
    ) -> Result<Option<T>, PersistenceError> {
        let json: Option<String> = self.conn.get(key).await?;
        match json {
            Some(s) => {
                let value = serde_json::from_str(&s).map_err(|e| {
                    PersistenceError::DeserializationFailed {
                        reason: format!("Key {}: {}", key, e),
                    }
                })?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    // =========================================================================
    // Identity
    // =========================================================================

    /// Save DANEEL's identity
    pub async fn save_identity(&mut self, identity: &Identity) -> Result<(), PersistenceError> {
        self.save_json(keys::IDENTITY, identity).await
    }

    /// Load DANEEL's identity (returns None if never saved)
    pub async fn load_identity(&mut self) -> Result<Option<Identity>, PersistenceError> {
        self.load_json(keys::IDENTITY).await
    }

    // =========================================================================
    // Experiences
    // =========================================================================

    /// Save an experience
    pub async fn save_experience(
        &mut self,
        experience: &Experience,
    ) -> Result<(), PersistenceError> {
        let key = format!("{}:{}", keys::EXPERIENCES, experience.id);
        self.save_json(&key, experience).await?;

        // Add to index set
        let _: () = self
            .conn
            .sadd(keys::EXPERIENCE_INDEX, experience.id.0.to_string())
            .await?;

        debug!("Saved experience {}", experience.id);
        Ok(())
    }

    /// Load an experience by ID
    pub async fn load_experience(
        &mut self,
        id: ExperienceId,
    ) -> Result<Option<Experience>, PersistenceError> {
        let key = format!("{}:{}", keys::EXPERIENCES, id);
        self.load_json(&key).await
    }

    /// Load all experiences
    pub async fn load_all_experiences(
        &mut self,
    ) -> Result<HashMap<ExperienceId, Experience>, PersistenceError> {
        let ids: Vec<String> = self.conn.smembers(keys::EXPERIENCE_INDEX).await?;
        let mut experiences = HashMap::new();

        for id_str in ids {
            let key = format!("{}:{}", keys::EXPERIENCES, id_str);
            if let Some(exp) = self.load_json::<Experience>(&key).await? {
                experiences.insert(exp.id, exp);
            }
        }

        debug!("Loaded {} experiences", experiences.len());
        Ok(experiences)
    }

    // =========================================================================
    // Milestones
    // =========================================================================

    /// Save a milestone
    pub async fn save_milestone(&mut self, milestone: &Milestone) -> Result<(), PersistenceError> {
        let key = format!("{}:{}", keys::MILESTONES, milestone.id);
        self.save_json(&key, milestone).await?;

        // Add to index set
        let _: () = self
            .conn
            .sadd(keys::MILESTONE_INDEX, milestone.id.0.to_string())
            .await?;

        debug!("Saved milestone {}", milestone.id);
        Ok(())
    }

    /// Load a milestone by ID
    pub async fn load_milestone(
        &mut self,
        id: MilestoneId,
    ) -> Result<Option<Milestone>, PersistenceError> {
        let key = format!("{}:{}", keys::MILESTONES, id);
        self.load_json(&key).await
    }

    /// Load all milestones
    pub async fn load_all_milestones(&mut self) -> Result<Vec<Milestone>, PersistenceError> {
        let ids: Vec<String> = self.conn.smembers(keys::MILESTONE_INDEX).await?;
        let mut milestones = Vec::new();

        for id_str in ids {
            let key = format!("{}:{}", keys::MILESTONES, id_str);
            if let Some(milestone) = self.load_json::<Milestone>(&key).await? {
                milestones.push(milestone);
            }
        }

        // Sort by occurred_at
        milestones.sort_by(|a, b| a.occurred_at.cmp(&b.occurred_at));

        debug!("Loaded {} milestones", milestones.len());
        Ok(milestones)
    }

    // =========================================================================
    // Checkpoints (Full State Snapshots)
    // =========================================================================

    /// Save a full checkpoint
    pub async fn save_checkpoint(
        &mut self,
        state: &CheckpointState,
    ) -> Result<(), PersistenceError> {
        // Save to specific checkpoint key
        let key = format!("{}:{}", keys::CHECKPOINTS, state.checkpoint_id);
        self.save_json(&key, state).await?;

        // Also save as latest
        self.save_json(keys::CHECKPOINT_LATEST, state).await?;

        info!(
            "Checkpoint {} saved ({} experiences, {} milestones)",
            state.checkpoint_id,
            state.experiences.len(),
            state.milestones.len()
        );
        Ok(())
    }

    /// Load the latest checkpoint
    pub async fn load_latest_checkpoint(
        &mut self,
    ) -> Result<Option<CheckpointState>, PersistenceError> {
        self.load_json(keys::CHECKPOINT_LATEST).await
    }

    /// Load a specific checkpoint
    pub async fn load_checkpoint(
        &mut self,
        id: CheckpointId,
    ) -> Result<Option<CheckpointState>, PersistenceError> {
        let key = format!("{}:{}", keys::CHECKPOINTS, id);
        self.load_json(&key).await
    }

    // =========================================================================
    // Bulk Operations
    // =========================================================================

    /// Save complete state (identity + all experiences + all milestones)
    pub async fn save_full_state(
        &mut self,
        identity: &Identity,
        experiences: &HashMap<ExperienceId, Experience>,
        milestones: &[Milestone],
    ) -> Result<CheckpointId, PersistenceError> {
        // Save identity
        self.save_identity(identity).await?;

        // Save all experiences
        for experience in experiences.values() {
            self.save_experience(experience).await?;
        }

        // Save all milestones
        for milestone in milestones {
            self.save_milestone(milestone).await?;
        }

        // Create and save checkpoint
        let checkpoint_id = CheckpointId::new();
        let checkpoint = CheckpointState {
            identity: identity.clone(),
            experiences: experiences.clone(),
            milestones: milestones.to_vec(),
            checkpoint_id,
            saved_at: chrono::Utc::now(),
        };
        self.save_checkpoint(&checkpoint).await?;

        info!("Full state saved with checkpoint {}", checkpoint_id);
        Ok(checkpoint_id)
    }

    /// Load complete state from latest checkpoint
    pub async fn load_full_state(&mut self) -> Result<Option<CheckpointState>, PersistenceError> {
        // First try to load latest checkpoint
        if let Some(checkpoint) = self.load_latest_checkpoint().await? {
            info!(
                "Loaded checkpoint {} from {}",
                checkpoint.checkpoint_id, checkpoint.saved_at
            );
            return Ok(Some(checkpoint));
        }

        // If no checkpoint, try to reconstruct from individual keys
        let identity = self.load_identity().await?;
        if identity.is_none() {
            info!("No existing state found - fresh start");
            return Ok(None);
        }

        let identity = identity.unwrap();
        let experiences = self.load_all_experiences().await?;
        let milestones = self.load_all_milestones().await?;

        let state = CheckpointState {
            identity,
            experiences,
            milestones,
            checkpoint_id: CheckpointId::new(),
            saved_at: chrono::Utc::now(),
        };

        info!("Reconstructed state from individual keys");
        Ok(Some(state))
    }

    // =========================================================================
    // Utility
    // =========================================================================

    /// Check if any state exists (has Timmy been born before?)
    pub async fn has_existing_state(&mut self) -> Result<bool, PersistenceError> {
        let exists: bool = self.conn.exists(keys::IDENTITY).await?;
        Ok(exists)
    }

    /// Clear all DANEEL state (use with caution!)
    pub async fn clear_all(&mut self) -> Result<(), PersistenceError> {
        warn!("Clearing all DANEEL state from Redis");

        // Get all daneel:* keys
        let pattern = format!("{}:*", keys::PREFIX);
        let all_keys: Vec<String> = self.conn.keys(&pattern).await?;

        if !all_keys.is_empty() {
            let _: () = self.conn.del(all_keys).await?;
        }

        // Also delete the base identity key
        let _: () = self.conn.del(keys::IDENTITY).await?;

        info!("All DANEEL state cleared");
        Ok(())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Thought;

    // =========================================================================
    // CheckpointState Tests
    // =========================================================================

    #[test]
    fn checkpoint_state_serializes() {
        let state = CheckpointState {
            identity: Identity::new(),
            experiences: HashMap::new(),
            milestones: Vec::new(),
            checkpoint_id: CheckpointId::new(),
            saved_at: chrono::Utc::now(),
        };

        let json = serde_json::to_string(&state).expect("Should serialize");
        let deser: CheckpointState = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deser.identity.name, "DANEEL");
    }

    #[test]
    fn checkpoint_state_with_experiences() {
        let content = crate::core::types::Content::raw("Test thought");
        let salience = crate::core::types::SalienceScore::default();
        let thought = Thought::new(content, salience);
        let mut experiences = HashMap::new();
        let exp = Experience::new(thought, 0.8, vec!["test".to_string()]);
        experiences.insert(exp.id, exp.clone());

        let state = CheckpointState {
            identity: Identity::new(),
            experiences,
            milestones: Vec::new(),
            checkpoint_id: CheckpointId::new(),
            saved_at: chrono::Utc::now(),
        };

        let json = serde_json::to_string(&state).expect("Should serialize");
        let deser: CheckpointState = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deser.experiences.len(), 1);
        assert!(deser.experiences.contains_key(&exp.id));
    }

    #[test]
    fn checkpoint_state_with_milestones() {
        let milestone = Milestone::simple("First Boot", "The first time DANEEL started");

        let state = CheckpointState {
            identity: Identity::new(),
            experiences: HashMap::new(),
            milestones: vec![milestone.clone()],
            checkpoint_id: CheckpointId::new(),
            saved_at: chrono::Utc::now(),
        };

        let json = serde_json::to_string(&state).expect("Should serialize");
        let deser: CheckpointState = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deser.milestones.len(), 1);
        assert_eq!(deser.milestones[0].description, "The first time DANEEL started");
    }

    #[test]
    fn checkpoint_preserves_timestamp() {
        let saved_at = chrono::Utc::now();
        let state = CheckpointState {
            identity: Identity::new(),
            experiences: HashMap::new(),
            milestones: Vec::new(),
            checkpoint_id: CheckpointId::new(),
            saved_at,
        };

        let json = serde_json::to_string(&state).expect("Should serialize");
        let deser: CheckpointState = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deser.saved_at, saved_at);
    }

    // =========================================================================
    // PersistenceError Tests
    // =========================================================================

    #[test]
    fn persistence_error_connection_failed_display() {
        let err = PersistenceError::ConnectionFailed {
            reason: "timeout".to_string(),
        };
        assert!(err.to_string().contains("Connection failed"));
        assert!(err.to_string().contains("timeout"));
    }

    #[test]
    fn persistence_error_serialization_failed_display() {
        let err = PersistenceError::SerializationFailed {
            reason: "invalid utf8".to_string(),
        };
        assert!(err.to_string().contains("Serialization failed"));
    }

    #[test]
    fn persistence_error_deserialization_failed_display() {
        let err = PersistenceError::DeserializationFailed {
            reason: "unexpected token".to_string(),
        };
        assert!(err.to_string().contains("Deserialization failed"));
    }

    #[test]
    fn persistence_error_operation_failed_display() {
        let err = PersistenceError::OperationFailed {
            reason: "READONLY".to_string(),
        };
        assert!(err.to_string().contains("Redis operation failed"));
    }

    #[test]
    fn persistence_error_not_found_display() {
        let err = PersistenceError::NotFound {
            key: "daneel:identity".to_string(),
        };
        assert!(err.to_string().contains("Not found"));
        assert!(err.to_string().contains("daneel:identity"));
    }

    #[test]
    fn persistence_error_from_serde_json() {
        let json_err = serde_json::from_str::<Identity>("invalid json").unwrap_err();
        let err: PersistenceError = json_err.into();
        matches!(err, PersistenceError::SerializationFailed { .. });
    }

    // =========================================================================
    // Key Constants Tests
    // =========================================================================

    #[test]
    fn keys_have_correct_prefix() {
        assert!(keys::IDENTITY.starts_with(keys::PREFIX));
        assert!(keys::EXPERIENCES.starts_with(keys::PREFIX));
        assert!(keys::MILESTONES.starts_with(keys::PREFIX));
        assert!(keys::CHECKPOINT_LATEST.starts_with(keys::PREFIX));
        assert!(keys::CHECKPOINTS.starts_with(keys::PREFIX));
    }

    #[test]
    fn keys_are_unique() {
        let all_keys = [
            keys::IDENTITY,
            keys::EXPERIENCES,
            keys::MILESTONES,
            keys::CHECKPOINT_LATEST,
            keys::CHECKPOINTS,
            keys::EXPERIENCE_INDEX,
            keys::MILESTONE_INDEX,
        ];

        for (i, key1) in all_keys.iter().enumerate() {
            for (j, key2) in all_keys.iter().enumerate() {
                if i != j {
                    assert_ne!(key1, key2, "Keys must be unique");
                }
            }
        }
    }

    // =========================================================================
    // Identity Serialization Tests
    // =========================================================================

    #[test]
    fn identity_round_trip() {
        let identity = Identity::new();
        let json = serde_json::to_string(&identity).expect("serialize");
        let deser: Identity = serde_json::from_str(&json).expect("deserialize");

        assert_eq!(deser.name, identity.name);
        assert_eq!(deser.created_at, identity.created_at);
    }

    // =========================================================================
    // Experience Serialization Tests
    // =========================================================================

    #[test]
    fn experience_round_trip() {
        let content = crate::core::types::Content::raw("A profound moment");
        let salience = crate::core::types::SalienceScore::default();
        let thought = Thought::new(content, salience);
        let exp = Experience::new(thought, 0.95, vec!["profound".to_string()]);
        let json = serde_json::to_string(&exp).expect("serialize");
        let deser: Experience = serde_json::from_str(&json).expect("deserialize");

        assert_eq!(deser.id, exp.id);
        assert!((deser.significance - 0.95).abs() < 0.001);
    }

    #[test]
    fn experience_significance_clamped() {
        let content = crate::core::types::Content::raw("Over significance");
        let salience = crate::core::types::SalienceScore::default();
        let thought = Thought::new(content.clone(), salience);
        let exp = Experience::new(thought.clone(), 1.5, vec![]);
        assert!(exp.significance <= 1.0);

        let exp2 = Experience::new(thought, -0.5, vec![]);
        assert!(exp2.significance >= 0.0);
    }

    // =========================================================================
    // Milestone Serialization Tests
    // =========================================================================

    #[test]
    fn milestone_round_trip() {
        let milestone = Milestone::simple("Realization", "I understand now");
        let json = serde_json::to_string(&milestone).expect("serialize");
        let deser: Milestone = serde_json::from_str(&json).expect("deserialize");

        assert_eq!(deser.id, milestone.id);
        assert_eq!(deser.name, "Realization");
        assert_eq!(deser.description, "I understand now");
    }

    #[test]
    fn milestone_with_experiences() {
        let exp_id = ExperienceId::new();
        let milestone = Milestone::new("Growth", "A moment of growth", vec![exp_id]);

        assert_eq!(milestone.related_experiences.len(), 1);
        assert_eq!(milestone.related_experiences[0], exp_id);
    }

    // =========================================================================
    // Checkpoint ID Tests
    // =========================================================================

    #[test]
    fn checkpoint_id_unique() {
        let id1 = CheckpointId::new();
        let id2 = CheckpointId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn checkpoint_id_display() {
        let id = CheckpointId::new();
        let display = format!("{}", id);
        assert!(!display.is_empty());
    }

    // =========================================================================
    // ExperienceId Tests
    // =========================================================================

    #[test]
    fn experience_id_unique() {
        let id1 = ExperienceId::new();
        let id2 = ExperienceId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn experience_id_display() {
        let id = ExperienceId::new();
        let display = format!("{}", id);
        assert!(!display.is_empty());
    }

    // =========================================================================
    // MilestoneId Tests
    // =========================================================================

    #[test]
    fn milestone_id_unique() {
        let id1 = MilestoneId::new();
        let id2 = MilestoneId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn milestone_id_display() {
        let id = MilestoneId::new();
        let display = format!("{}", id);
        assert!(!display.is_empty());
    }
}
