//! Supervisor Module
//!
//! Erlang-style "let it crash" supervision for actors.
//! Part of RES-4: Supervisor Tree (Ractor).
//!
//! # Philosophy
//!
//! Instead of trying to handle every error, we let actors crash
//! and restart them automatically. This is the Erlang way.
//!
//! Supervision strategies:
//! - OneForOne: Restart only the failed actor
//! - OneForAll: Restart all actors if one fails
//! - RestForOne: Restart the failed actor and all actors started after it

use std::collections::HashMap;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

/// Supervision strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupervisionStrategy {
    /// Restart only the failed actor
    OneForOne,
    /// Restart all actors when one fails
    OneForAll,
    /// Restart failed actor and all started after it
    RestForOne,
}

impl Default for SupervisionStrategy {
    fn default() -> Self {
        Self::OneForOne
    }
}

/// Configuration for the supervisor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisorConfig {
    /// Supervision strategy
    pub strategy: SupervisionStrategy,

    /// Maximum restarts within the time window before escalating
    pub max_restarts: u32,

    /// Time window for counting restarts
    pub restart_window: Duration,

    /// Delay between restart attempts
    pub restart_delay: Duration,
}

impl Default for SupervisorConfig {
    fn default() -> Self {
        Self {
            strategy: SupervisionStrategy::OneForOne,
            max_restarts: 3,
            restart_window: Duration::from_secs(10),
            restart_delay: Duration::from_millis(100),
        }
    }
}

impl SupervisorConfig {
    /// Create a config with custom max restarts
    pub fn with_max_restarts(mut self, max: u32) -> Self {
        self.max_restarts = max;
        self
    }

    /// Create a config with custom restart window
    pub fn with_restart_window(mut self, window: Duration) -> Self {
        self.restart_window = window;
        self
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), SupervisorError> {
        if self.max_restarts == 0 {
            return Err(SupervisorError::InvalidConfig(
                "max_restarts must be > 0".to_string(),
            ));
        }
        if self.restart_window.is_zero() {
            return Err(SupervisorError::InvalidConfig(
                "restart_window must be > 0".to_string(),
            ));
        }
        Ok(())
    }
}

/// Errors that can occur in supervision
#[derive(Debug, Clone, thiserror::Error)]
pub enum SupervisorError {
    /// Configuration is invalid
    #[error("Invalid supervisor config: {0}")]
    InvalidConfig(String),

    /// Actor restart limit exceeded
    #[error("Actor '{0}' exceeded restart limit ({1} restarts in {2:?})")]
    RestartLimitExceeded(String, u32, Duration),

    /// Actor not found
    #[error("Actor '{0}' not found")]
    ActorNotFound(String),

    /// Restart failed
    #[error("Failed to restart actor '{0}': {1}")]
    RestartFailed(String, String),
}

/// Event emitted by the supervisor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SupervisorEvent {
    /// Actor was started
    ActorStarted {
        actor_id: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    },

    /// Actor crashed
    ActorCrashed {
        actor_id: String,
        reason: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    },

    /// Actor was restarted
    ActorRestarted {
        actor_id: String,
        restart_count: u32,
        timestamp: chrono::DateTime<chrono::Utc>,
    },

    /// Restart limit exceeded, escalating
    RestartLimitExceeded {
        actor_id: String,
        restart_count: u32,
        timestamp: chrono::DateTime<chrono::Utc>,
    },

    /// Full system restart triggered
    FullRestartTriggered {
        reason: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
}

/// Tracks restart history for an actor
#[derive(Debug, Clone)]
struct RestartHistory {
    /// List of restart timestamps
    timestamps: Vec<Instant>,
}

impl RestartHistory {
    fn new() -> Self {
        Self {
            timestamps: Vec::new(),
        }
    }

    /// Record a restart and return count within window
    fn record_restart(&mut self, window: Duration) -> u32 {
        let now = Instant::now();

        // Add new restart
        self.timestamps.push(now);

        // Remove old restarts outside window
        self.timestamps.retain(|t| now.duration_since(*t) <= window);

        self.timestamps.len() as u32
    }

    /// Get restart count within window
    fn count_within_window(&self, window: Duration) -> u32 {
        let now = Instant::now();
        self.timestamps
            .iter()
            .filter(|t| now.duration_since(**t) <= window)
            .count() as u32
    }

    /// Clear restart history
    fn clear(&mut self) {
        self.timestamps.clear();
    }
}

/// Actor state tracked by supervisor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActorState {
    /// Actor is running normally
    Running,
    /// Actor has crashed, pending restart
    Crashed,
    /// Actor is being restarted
    Restarting,
    /// Actor has been stopped (not restarting)
    Stopped,
}

/// Information about a supervised actor
struct SupervisedActor {
    state: ActorState,
    restart_history: RestartHistory,
}

/// Supervisor that manages actor lifecycle
pub struct Supervisor {
    config: SupervisorConfig,
    actors: HashMap<String, SupervisedActor>,
    events: Vec<SupervisorEvent>,
}

impl Supervisor {
    /// Create a new supervisor with the given config
    pub fn new(config: SupervisorConfig) -> Result<Self, SupervisorError> {
        config.validate()?;
        Ok(Self {
            config,
            actors: HashMap::new(),
            events: Vec::new(),
        })
    }

    /// Register an actor with the supervisor
    pub fn register_actor(&mut self, actor_id: &str) {
        self.actors.insert(
            actor_id.to_string(),
            SupervisedActor {
                state: ActorState::Running,
                restart_history: RestartHistory::new(),
            },
        );

        self.events.push(SupervisorEvent::ActorStarted {
            actor_id: actor_id.to_string(),
            timestamp: chrono::Utc::now(),
        });
    }

    /// Report that an actor has crashed
    ///
    /// Returns Ok(true) if actor should be restarted,
    /// Ok(false) if restart limit exceeded,
    /// Err if actor not found.
    pub fn report_crash(
        &mut self,
        actor_id: &str,
        reason: &str,
    ) -> Result<bool, SupervisorError> {
        let actor = self
            .actors
            .get_mut(actor_id)
            .ok_or_else(|| SupervisorError::ActorNotFound(actor_id.to_string()))?;

        actor.state = ActorState::Crashed;

        self.events.push(SupervisorEvent::ActorCrashed {
            actor_id: actor_id.to_string(),
            reason: reason.to_string(),
            timestamp: chrono::Utc::now(),
        });

        // Record restart and check limit
        let restart_count = actor
            .restart_history
            .record_restart(self.config.restart_window);

        if restart_count > self.config.max_restarts {
            self.events.push(SupervisorEvent::RestartLimitExceeded {
                actor_id: actor_id.to_string(),
                restart_count,
                timestamp: chrono::Utc::now(),
            });
            return Ok(false);
        }

        Ok(true)
    }

    /// Mark actor as restarted
    pub fn mark_restarted(&mut self, actor_id: &str) -> Result<(), SupervisorError> {
        let actor = self
            .actors
            .get_mut(actor_id)
            .ok_or_else(|| SupervisorError::ActorNotFound(actor_id.to_string()))?;

        let restart_count = actor
            .restart_history
            .count_within_window(self.config.restart_window);

        actor.state = ActorState::Running;

        self.events.push(SupervisorEvent::ActorRestarted {
            actor_id: actor_id.to_string(),
            restart_count,
            timestamp: chrono::Utc::now(),
        });

        Ok(())
    }

    /// Get the state of an actor
    pub fn get_actor_state(&self, actor_id: &str) -> Option<ActorState> {
        self.actors.get(actor_id).map(|a| a.state)
    }

    /// Get restart count for an actor within the current window
    pub fn get_restart_count(&self, actor_id: &str) -> Option<u32> {
        self.actors
            .get(actor_id)
            .map(|a| a.restart_history.count_within_window(self.config.restart_window))
    }

    /// Drain and return all pending events
    pub fn drain_events(&mut self) -> Vec<SupervisorEvent> {
        std::mem::take(&mut self.events)
    }

    /// Trigger a full system restart
    pub fn trigger_full_restart(&mut self, reason: &str) {
        for actor in self.actors.values_mut() {
            actor.state = ActorState::Restarting;
            actor.restart_history.clear();
        }

        self.events.push(SupervisorEvent::FullRestartTriggered {
            reason: reason.to_string(),
            timestamp: chrono::Utc::now(),
        });
    }

    /// Get IDs of all actors that need restart based on strategy
    pub fn get_actors_to_restart(&self, failed_actor: &str) -> Vec<String> {
        match self.config.strategy {
            SupervisionStrategy::OneForOne => {
                vec![failed_actor.to_string()]
            }
            SupervisionStrategy::OneForAll => {
                self.actors.keys().cloned().collect()
            }
            SupervisionStrategy::RestForOne => {
                // For simplicity, treat as OneForOne for now
                // Full implementation would track actor start order
                vec![failed_actor.to_string()]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supervisor_config_default() {
        let config = SupervisorConfig::default();
        assert_eq!(config.strategy, SupervisionStrategy::OneForOne);
        assert_eq!(config.max_restarts, 3);
        assert_eq!(config.restart_window, Duration::from_secs(10));
    }

    #[test]
    fn test_supervisor_config_validates() {
        let valid = SupervisorConfig::default();
        assert!(valid.validate().is_ok());

        let invalid = SupervisorConfig {
            max_restarts: 0,
            ..Default::default()
        };
        assert!(invalid.validate().is_err());

        let invalid2 = SupervisorConfig {
            restart_window: Duration::ZERO,
            ..Default::default()
        };
        assert!(invalid2.validate().is_err());
    }

    #[test]
    fn test_restart_tracking() {
        let config = SupervisorConfig::default();
        let mut supervisor = Supervisor::new(config).unwrap();

        supervisor.register_actor("test_actor");

        // First crash should allow restart
        let should_restart = supervisor.report_crash("test_actor", "test error").unwrap();
        assert!(should_restart);
        assert_eq!(supervisor.get_restart_count("test_actor"), Some(1));

        // Second crash
        let should_restart = supervisor.report_crash("test_actor", "test error").unwrap();
        assert!(should_restart);
        assert_eq!(supervisor.get_restart_count("test_actor"), Some(2));

        // Third crash
        let should_restart = supervisor.report_crash("test_actor", "test error").unwrap();
        assert!(should_restart);
        assert_eq!(supervisor.get_restart_count("test_actor"), Some(3));

        // Fourth crash should exceed limit
        let should_restart = supervisor.report_crash("test_actor", "test error").unwrap();
        assert!(!should_restart);
    }

    #[test]
    fn test_escalation_after_threshold() {
        let config = SupervisorConfig {
            max_restarts: 2,
            restart_window: Duration::from_secs(60),
            ..Default::default()
        };
        let mut supervisor = Supervisor::new(config).unwrap();

        supervisor.register_actor("test_actor");

        // First two crashes are fine
        assert!(supervisor.report_crash("test_actor", "crash 1").unwrap());
        assert!(supervisor.report_crash("test_actor", "crash 2").unwrap());

        // Third crash exceeds limit
        assert!(!supervisor.report_crash("test_actor", "crash 3").unwrap());

        // Check events include escalation
        let events = supervisor.drain_events();
        assert!(events.iter().any(|e| matches!(e, SupervisorEvent::RestartLimitExceeded { .. })));
    }

    #[test]
    fn test_one_for_one_strategy() {
        let config = SupervisorConfig {
            strategy: SupervisionStrategy::OneForOne,
            ..Default::default()
        };
        let mut supervisor = Supervisor::new(config).unwrap();

        supervisor.register_actor("actor1");
        supervisor.register_actor("actor2");
        supervisor.register_actor("actor3");

        let to_restart = supervisor.get_actors_to_restart("actor2");
        assert_eq!(to_restart, vec!["actor2"]);
    }

    #[test]
    fn test_one_for_all_strategy() {
        let config = SupervisorConfig {
            strategy: SupervisionStrategy::OneForAll,
            ..Default::default()
        };
        let mut supervisor = Supervisor::new(config).unwrap();

        supervisor.register_actor("actor1");
        supervisor.register_actor("actor2");
        supervisor.register_actor("actor3");

        let mut to_restart = supervisor.get_actors_to_restart("actor2");
        to_restart.sort();

        assert_eq!(to_restart.len(), 3);
        assert!(to_restart.contains(&"actor1".to_string()));
        assert!(to_restart.contains(&"actor2".to_string()));
        assert!(to_restart.contains(&"actor3".to_string()));
    }

    #[test]
    fn test_actor_state_transitions() {
        let config = SupervisorConfig::default();
        let mut supervisor = Supervisor::new(config).unwrap();

        supervisor.register_actor("test_actor");
        assert_eq!(
            supervisor.get_actor_state("test_actor"),
            Some(ActorState::Running)
        );

        supervisor.report_crash("test_actor", "crash").unwrap();
        assert_eq!(
            supervisor.get_actor_state("test_actor"),
            Some(ActorState::Crashed)
        );

        supervisor.mark_restarted("test_actor").unwrap();
        assert_eq!(
            supervisor.get_actor_state("test_actor"),
            Some(ActorState::Running)
        );
    }

    #[test]
    fn test_actor_not_found() {
        let config = SupervisorConfig::default();
        let mut supervisor = Supervisor::new(config).unwrap();

        let result = supervisor.report_crash("nonexistent", "crash");
        assert!(matches!(result, Err(SupervisorError::ActorNotFound(_))));
    }

    #[test]
    fn test_events_are_emitted() {
        let config = SupervisorConfig::default();
        let mut supervisor = Supervisor::new(config).unwrap();

        supervisor.register_actor("test_actor");
        supervisor.report_crash("test_actor", "crash").unwrap();
        supervisor.mark_restarted("test_actor").unwrap();

        let events = supervisor.drain_events();

        assert!(events.iter().any(|e| matches!(e, SupervisorEvent::ActorStarted { .. })));
        assert!(events.iter().any(|e| matches!(e, SupervisorEvent::ActorCrashed { .. })));
        assert!(events.iter().any(|e| matches!(e, SupervisorEvent::ActorRestarted { .. })));
    }
}
