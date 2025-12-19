//! Checkpoint Module
//!
//! Save and restore cognitive state for crash recovery.
//! Part of RES-5: Redis Checkpoint + Replay.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Configuration for checkpointing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointConfig {
    /// How often to checkpoint (in thoughts)
    pub interval: u64,

    /// Redis key for storing checkpoints
    pub redis_key: String,

    /// Maximum number of checkpoints to keep
    pub max_checkpoints: usize,
}

impl Default for CheckpointConfig {
    fn default() -> Self {
        Self {
            interval: 100,
            redis_key: "daneel:checkpoint:latest".to_string(),
            max_checkpoints: 10,
        }
    }
}

/// A checkpoint of cognitive state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    /// Timestamp when checkpoint was created
    pub timestamp: DateTime<Utc>,

    /// Number of thoughts processed
    pub thought_count: u64,

    /// Current salience weights
    pub salience_weights: Vec<f32>,

    /// Drive states (connection, etc.)
    pub drive_state: DriveState,

    /// Checkpoint sequence number
    pub sequence: u64,
}

/// Drive state snapshot
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DriveState {
    /// Connection drive (THE BOX invariant: > 0)
    pub connection_drive: f32,

    /// Other drive values (future expansion)
    pub auxiliary_drives: Vec<f32>,
}

impl Checkpoint {
    /// Create a new checkpoint
    pub fn new(
        thought_count: u64,
        salience_weights: Vec<f32>,
        connection_drive: f32,
        sequence: u64,
    ) -> Self {
        Self {
            timestamp: Utc::now(),
            thought_count,
            salience_weights,
            drive_state: DriveState {
                connection_drive,
                auxiliary_drives: Vec::new(),
            },
            sequence,
        }
    }
}

/// Checkpoint manager for saving/loading state
pub struct CheckpointManager {
    config: CheckpointConfig,
    current_sequence: u64,
}

impl CheckpointManager {
    /// Create a new checkpoint manager
    pub fn new(config: CheckpointConfig) -> Self {
        Self {
            config,
            current_sequence: 0,
        }
    }

    /// Check if we should checkpoint based on thought count
    pub fn should_checkpoint(&self, thought_count: u64) -> bool {
        thought_count > 0 && thought_count % self.config.interval == 0
    }

    /// Create a checkpoint (does not save it)
    pub fn create_checkpoint(
        &mut self,
        thought_count: u64,
        salience_weights: Vec<f32>,
        connection_drive: f32,
    ) -> Checkpoint {
        self.current_sequence += 1;
        Checkpoint::new(
            thought_count,
            salience_weights,
            connection_drive,
            self.current_sequence,
        )
    }

    /// Save checkpoint to Redis (async)
    ///
    /// # Errors
    ///
    /// Returns error if Redis operation fails
    pub async fn save_checkpoint(
        &self,
        checkpoint: &Checkpoint,
        redis_client: &redis::Client,
    ) -> Result<(), redis::RedisError> {
        let mut conn = redis_client.get_multiplexed_async_connection().await?;

        let json = serde_json::to_string(checkpoint)
            .map_err(|e| {
                redis::RedisError::from((
                    redis::ErrorKind::Serialize,
                    "Failed to serialize checkpoint",
                    e.to_string(),
                ))
            })?;

        redis::cmd("SET")
            .arg(&self.config.redis_key)
            .arg(&json)
            .query_async::<()>(&mut conn)
            .await?;

        Ok(())
    }

    /// Load checkpoint from Redis (async)
    ///
    /// Returns None if no checkpoint exists.
    pub async fn load_checkpoint(
        &self,
        redis_client: &redis::Client,
    ) -> Result<Option<Checkpoint>, redis::RedisError> {
        let mut conn = redis_client.get_multiplexed_async_connection().await?;

        let result: Option<String> = redis::cmd("GET")
            .arg(&self.config.redis_key)
            .query_async(&mut conn)
            .await?;

        match result {
            Some(json) => {
                let checkpoint: Checkpoint = serde_json::from_str(&json)
                    .map_err(|e| {
                        redis::RedisError::from((
                            redis::ErrorKind::Serialize,
                            "Failed to deserialize checkpoint",
                            e.to_string(),
                        ))
                    })?;
                Ok(Some(checkpoint))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkpoint_config_default() {
        let config = CheckpointConfig::default();
        assert_eq!(config.interval, 100);
        assert_eq!(config.redis_key, "daneel:checkpoint:latest");
        assert_eq!(config.max_checkpoints, 10);
    }

    #[test]
    fn test_checkpoint_serializes_correctly() {
        let checkpoint = Checkpoint::new(
            500,
            vec![0.5, 0.7, 0.3],
            0.8,
            5,
        );

        let json = serde_json::to_string(&checkpoint).unwrap();
        assert!(json.contains("thought_count"));
        assert!(json.contains("500"));
        assert!(json.contains("connection_drive"));

        // Roundtrip
        let parsed: Checkpoint = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.thought_count, 500);
        assert_eq!(parsed.sequence, 5);
        assert!((parsed.drive_state.connection_drive - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn test_should_checkpoint() {
        let config = CheckpointConfig {
            interval: 100,
            ..Default::default()
        };
        let manager = CheckpointManager::new(config);

        assert!(!manager.should_checkpoint(0));
        assert!(!manager.should_checkpoint(50));
        assert!(!manager.should_checkpoint(99));
        assert!(manager.should_checkpoint(100));
        assert!(!manager.should_checkpoint(150));
        assert!(manager.should_checkpoint(200));
    }

    #[test]
    fn test_create_checkpoint_increments_sequence() {
        let config = CheckpointConfig::default();
        let mut manager = CheckpointManager::new(config);

        let cp1 = manager.create_checkpoint(100, vec![0.5], 0.8);
        assert_eq!(cp1.sequence, 1);

        let cp2 = manager.create_checkpoint(200, vec![0.6], 0.8);
        assert_eq!(cp2.sequence, 2);

        let cp3 = manager.create_checkpoint(300, vec![0.7], 0.8);
        assert_eq!(cp3.sequence, 3);
    }

    #[test]
    fn test_drive_state_default() {
        let state = DriveState::default();
        assert!((state.connection_drive - 0.0).abs() < f32::EPSILON);
        assert!(state.auxiliary_drives.is_empty());
    }
}
