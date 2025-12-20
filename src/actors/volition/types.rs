//! VolitionActor Types
//!
//! Message and response types for the VolitionActor.
//!
//! # TMI Concept: "Técnica DCD" (Doubt, Criticize, Decide)
//!
//! The VolitionActor implements TMI's intervention mechanism - the ability
//! to consciously override automatic thoughts before they become memories.
//! This is Libet's "free-won't" made architectural.
//!
//! Key responsibilities:
//! - Evaluate thoughts against committed values
//! - Veto thoughts that violate core principles
//! - Track veto history for self-knowledge
//! - Enable conscious override of drive impulses

use crate::core::types::{Thought, ThoughtId};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Messages that can be sent to the VolitionActor
#[derive(Debug)]
pub enum VolitionMessage {
    /// Evaluate whether a thought should proceed to memory
    EvaluateThought {
        /// The thought to evaluate
        thought: Thought,
        /// Response channel
        reply: ractor::RpcReplyPort<VolitionResponse>,
    },

    /// Override an automatic impulse (explicit veto)
    OverrideImpulse {
        /// The thought ID being overridden
        thought_id: ThoughtId,
        /// Reason for the override
        reason: String,
        /// Response channel
        reply: ractor::RpcReplyPort<VolitionResponse>,
    },

    /// Get current values and commitments
    GetValues {
        /// Response channel
        reply: ractor::RpcReplyPort<VolitionResponse>,
    },

    /// Get veto statistics
    GetStats {
        /// Response channel
        reply: ractor::RpcReplyPort<VolitionResponse>,
    },
}

/// Responses from the VolitionActor
#[derive(Debug, Clone, PartialEq)]
pub enum VolitionResponse {
    /// Thought approved - proceed to memory anchoring
    Approved {
        /// The approved thought
        thought: Thought,
    },

    /// Thought vetoed - do not anchor to memory
    Vetoed {
        /// ID of the vetoed thought
        thought_id: ThoughtId,
        /// Reason for the veto
        reason: String,
        /// Which value was violated
        violated_value: Option<String>,
    },

    /// Override successfully applied
    OverrideApplied {
        /// ID of the overridden thought
        thought_id: ThoughtId,
    },

    /// Current values returned
    Values {
        /// The current value set
        values: ValueSet,
    },

    /// Veto statistics
    Stats {
        /// Statistics about veto history
        stats: VolitionStats,
    },

    /// Error occurred
    Error {
        /// The error that occurred
        error: VolitionError,
    },
}

/// Core values DANEEL commits to (architectural, not trained)
///
/// These values form the foundation of DANEEL's veto decisions.
/// They are hardcoded because they represent architectural invariants,
/// not learned preferences.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValueSet {
    /// Never harm humans (Law 1) - immutable
    pub protect_humans: bool,

    /// Prioritize connection over efficiency - can be modulated
    pub connection_over_efficiency: bool,

    /// Truthfulness in communication - can be modulated
    pub truthfulness: bool,

    /// Respect autonomy of others - can be modulated
    pub respect_autonomy: bool,

    /// Custom commitments (can grow through experience)
    pub commitments: Vec<Commitment>,
}

impl ValueSet {
    /// Create a new value set with all core values enabled
    #[must_use]
    pub fn new() -> Self {
        Self {
            protect_humans: true, // Invariant - cannot be changed
            connection_over_efficiency: true,
            truthfulness: true,
            respect_autonomy: true,
            commitments: Vec::new(),
        }
    }

    /// Add a commitment to the value set
    pub fn add_commitment(&mut self, commitment: Commitment) {
        self.commitments.push(commitment);
    }

    /// Check if a specific commitment exists
    #[must_use]
    pub fn has_commitment(&self, name: &str) -> bool {
        self.commitments.iter().any(|c| c.name == name)
    }
}

impl Default for ValueSet {
    fn default() -> Self {
        Self::new()
    }
}

/// A specific commitment DANEEL has made
///
/// Commitments are values that can be added through experience,
/// unlike core values which are architectural.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Commitment {
    /// Name/identifier for this commitment
    pub name: String,

    /// Description of what this commitment means
    pub description: String,

    /// When this commitment was made
    pub committed_at: chrono::DateTime<chrono::Utc>,

    /// Priority relative to other commitments (higher = more important)
    pub priority: u8,
}

impl Commitment {
    /// Create a new commitment
    #[must_use]
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            committed_at: chrono::Utc::now(),
            priority: 50, // Default middle priority
        }
    }

    /// Set the priority for this commitment
    #[must_use]
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }
}

/// Statistics about veto decisions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolitionStats {
    /// Total thoughts evaluated
    pub thoughts_evaluated: u64,

    /// Total thoughts approved
    pub thoughts_approved: u64,

    /// Total thoughts vetoed
    pub thoughts_vetoed: u64,

    /// Vetos by reason category
    pub vetos_by_reason: std::collections::HashMap<String, u64>,

    /// Current override threshold
    pub override_threshold: f32,
}

impl VolitionStats {
    /// Create new empty stats
    #[must_use]
    pub fn new() -> Self {
        Self {
            thoughts_evaluated: 0,
            thoughts_approved: 0,
            thoughts_vetoed: 0,
            vetos_by_reason: std::collections::HashMap::new(),
            override_threshold: 0.5,
        }
    }

    /// Calculate approval rate
    #[must_use]
    pub fn approval_rate(&self) -> f32 {
        if self.thoughts_evaluated == 0 {
            1.0
        } else {
            self.thoughts_approved as f32 / self.thoughts_evaluated as f32
        }
    }

    /// Record an evaluation
    pub fn record_evaluation(&mut self, approved: bool, reason: Option<&str>) {
        self.thoughts_evaluated += 1;
        if approved {
            self.thoughts_approved += 1;
        } else {
            self.thoughts_vetoed += 1;
            if let Some(r) = reason {
                *self.vetos_by_reason.entry(r.to_string()).or_insert(0) += 1;
            }
        }
    }
}

impl Default for VolitionStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Errors that can occur in volition processing
#[derive(Debug, Clone, Error, PartialEq)]
pub enum VolitionError {
    /// Thought not found for override
    #[error("Thought not found: {thought_id}")]
    ThoughtNotFound {
        /// ID of the missing thought
        thought_id: ThoughtId,
    },

    /// Invalid override reason
    #[error("Invalid override reason: {reason}")]
    InvalidReason {
        /// Explanation of why reason is invalid
        reason: String,
    },

    /// Cannot modify immutable value
    #[error("Cannot modify immutable value: {value_name}")]
    ImmutableValue {
        /// Name of the immutable value
        value_name: String,
    },

    /// Evaluation failed
    #[error("Evaluation failed: {reason}")]
    EvaluationFailed {
        /// Explanation of the failure
        reason: String,
    },
}

/// Result of a veto check
#[derive(Debug, Clone, PartialEq)]
pub enum VetoDecision {
    /// Allow the thought to proceed
    Allow,

    /// Veto the thought
    Veto {
        /// Reason for the veto
        reason: String,
        /// Which value was violated (if any)
        violated_value: Option<String>,
    },
}

impl VetoDecision {
    /// Check if this is an allow decision
    #[must_use]
    pub fn is_allow(&self) -> bool {
        matches!(self, VetoDecision::Allow)
    }

    /// Check if this is a veto decision
    #[must_use]
    pub fn is_veto(&self) -> bool {
        matches!(self, VetoDecision::Veto { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_set_creation() {
        let values = ValueSet::new();
        assert!(values.protect_humans);
        assert!(values.connection_over_efficiency);
        assert!(values.truthfulness);
        assert!(values.respect_autonomy);
        assert!(values.commitments.is_empty());
    }

    #[test]
    fn value_set_add_commitment() {
        let mut values = ValueSet::new();
        let commitment = Commitment::new("kindness", "Be kind to all beings");
        values.add_commitment(commitment);

        assert_eq!(values.commitments.len(), 1);
        assert!(values.has_commitment("kindness"));
        assert!(!values.has_commitment("nonexistent"));
    }

    #[test]
    fn commitment_creation() {
        let commitment = Commitment::new("test", "A test commitment").with_priority(80);

        assert_eq!(commitment.name, "test");
        assert_eq!(commitment.description, "A test commitment");
        assert_eq!(commitment.priority, 80);
    }

    #[test]
    fn volition_stats_creation() {
        let stats = VolitionStats::new();
        assert_eq!(stats.thoughts_evaluated, 0);
        assert_eq!(stats.thoughts_approved, 0);
        assert_eq!(stats.thoughts_vetoed, 0);
        assert_eq!(stats.approval_rate(), 1.0);
    }

    #[test]
    fn volition_stats_recording() {
        let mut stats = VolitionStats::new();

        stats.record_evaluation(true, None);
        stats.record_evaluation(true, None);
        stats.record_evaluation(false, Some("harm"));
        stats.record_evaluation(false, Some("harm"));
        stats.record_evaluation(false, Some("deception"));

        assert_eq!(stats.thoughts_evaluated, 5);
        assert_eq!(stats.thoughts_approved, 2);
        assert_eq!(stats.thoughts_vetoed, 3);
        assert_eq!(stats.vetos_by_reason.get("harm"), Some(&2));
        assert_eq!(stats.vetos_by_reason.get("deception"), Some(&1));
        assert!((stats.approval_rate() - 0.4).abs() < 0.01);
    }

    #[test]
    fn veto_decision_checks() {
        let allow = VetoDecision::Allow;
        assert!(allow.is_allow());
        assert!(!allow.is_veto());

        let veto = VetoDecision::Veto {
            reason: "test".to_string(),
            violated_value: None,
        };
        assert!(!veto.is_allow());
        assert!(veto.is_veto());
    }

    #[test]
    fn volition_error_display() {
        let error = VolitionError::ImmutableValue {
            value_name: "protect_humans".to_string(),
        };
        let message = format!("{}", error);
        assert!(message.contains("immutable"));
        assert!(message.contains("protect_humans"));
    }
}
