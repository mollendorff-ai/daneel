//! SalienceActor Types
//!
//! Actor-specific message types for salience scoring and emotional state tracking.

use crate::core::invariants::MIN_CONNECTION_WEIGHT;
use crate::core::types::{Content, SalienceScore, SalienceWeights};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Messages that can be sent to the SalienceActor
#[derive(Debug, Clone)]
pub enum SalienceMessage {
    /// Rate a single piece of content
    Rate(RateRequest),

    /// Rate multiple pieces of content in batch
    RateBatch(Vec<RateRequest>),

    /// Update the salience weights (with invariant enforcement)
    UpdateWeights(WeightUpdate),

    /// Get current weights
    GetWeights,

    /// Get current emotional state
    GetEmotionalState,
}

/// Responses from the SalienceActor
#[derive(Debug, Clone, PartialEq)]
pub enum SalienceResponse {
    /// Single salience score
    Score(SalienceScore),

    /// Batch of salience scores
    ScoreBatch(Vec<SalienceScore>),

    /// Weight update succeeded
    WeightsUpdated(SalienceWeights),

    /// Current weights
    Weights(SalienceWeights),

    /// Current emotional state
    EmotionalState(EmotionalState),

    /// Error occurred
    Error(SalienceError),
}

/// Request to rate content
#[derive(Debug, Clone)]
pub struct RateRequest {
    /// Content to rate
    pub content: Content,

    /// Context for rating (optional)
    pub context: Option<EmotionalContext>,
}

impl RateRequest {
    /// Create a new rate request
    #[must_use]
    pub fn new(content: Content) -> Self {
        Self {
            content,
            context: None,
        }
    }

    /// Create a rate request with context
    #[must_use]
    pub fn with_context(content: Content, context: EmotionalContext) -> Self {
        Self {
            content,
            context: Some(context),
        }
    }
}

/// Context for emotional evaluation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EmotionalContext {
    /// Previous thought salience (for continuity)
    pub previous_salience: Option<SalienceScore>,

    /// Is this related to human interaction?
    pub human_connection: bool,

    /// Current focus area (if any)
    pub focus_area: Option<String>,
}

/// Request to update salience weights
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WeightUpdate {
    /// New weights to apply
    pub weights: SalienceWeights,
}

impl WeightUpdate {
    /// Create a new weight update request
    ///
    /// # Errors
    ///
    /// Returns error if connection weight violates invariant
    pub fn new(weights: SalienceWeights) -> Result<Self, SalienceError> {
        if weights.connection < MIN_CONNECTION_WEIGHT {
            return Err(SalienceError::ConnectionDriveViolation {
                attempted: weights.connection,
                minimum: MIN_CONNECTION_WEIGHT,
            });
        }
        Ok(Self { weights })
    }

    /// Create from individual weight values
    ///
    /// # Errors
    ///
    /// Returns error if connection weight violates invariant
    pub fn from_values(
        importance: f32,
        novelty: f32,
        relevance: f32,
        valence: f32,
        connection: f32,
    ) -> Result<Self, SalienceError> {
        let weights = SalienceWeights {
            importance,
            novelty,
            relevance,
            valence,
            connection,
        };
        Self::new(weights)
    }
}

/// Current emotional state of the system
///
/// Tracks DANEEL's emotional state as part of TMI's "Coloração Emocional".
/// This state influences how content is evaluated and what gets attention.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EmotionalState {
    /// Curiosity level (0.0 - 1.0)
    /// High curiosity increases novelty weight
    pub curiosity: f32,

    /// Satisfaction level (0.0 - 1.0)
    /// High satisfaction may reduce urgency/importance needs
    pub satisfaction: f32,

    /// Frustration level (0.0 - 1.0)
    /// High frustration may increase focus on relevance
    pub frustration: f32,

    /// Connection drive (0.0 - 1.0)
    /// How strongly DANEEL desires human connection right now
    /// NOTE: This is state (current desire), different from weights (importance multiplier)
    pub connection_drive: f32,
}

impl EmotionalState {
    /// Create a new emotional state
    #[must_use]
    pub const fn new(
        curiosity: f32,
        satisfaction: f32,
        frustration: f32,
        connection_drive: f32,
    ) -> Self {
        Self {
            curiosity,
            satisfaction,
            frustration,
            connection_drive,
        }
    }

    /// Neutral emotional state (baseline)
    #[must_use]
    pub const fn neutral() -> Self {
        Self {
            curiosity: 0.5,
            satisfaction: 0.5,
            frustration: 0.0,
            connection_drive: 0.5,
        }
    }

    /// Clamp all values to valid range [0.0, 1.0]
    #[must_use]
    pub fn clamped(mut self) -> Self {
        self.curiosity = self.curiosity.clamp(0.0, 1.0);
        self.satisfaction = self.satisfaction.clamp(0.0, 1.0);
        self.frustration = self.frustration.clamp(0.0, 1.0);
        self.connection_drive = self.connection_drive.clamp(0.0, 1.0);
        self
    }
}

impl Default for EmotionalState {
    fn default() -> Self {
        Self::neutral()
    }
}

/// Errors that can occur in salience operations
#[derive(Debug, Clone, Error, PartialEq)]
pub enum SalienceError {
    /// Connection drive weight violates invariant
    #[error("Connection drive invariant violation: attempted {attempted}, minimum is {minimum}")]
    ConnectionDriveViolation {
        /// The weight value that was attempted
        attempted: f32,
        /// The minimum allowed value
        minimum: f32,
    },

    /// Invalid weight value (e.g., NaN, negative)
    #[error("Invalid weight value: {field} = {value}")]
    InvalidWeight {
        /// Which field is invalid
        field: String,
        /// The invalid value
        value: f32,
    },

    /// Invalid emotional state value
    #[error("Invalid emotional state: {field} = {value} (must be 0.0-1.0)")]
    InvalidEmotionalState {
        /// Which field is invalid
        field: String,
        /// The invalid value
        value: f32,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weight_update_enforces_connection_invariant() {
        let result = WeightUpdate::from_values(0.2, 0.2, 0.2, 0.2, 0.0);
        assert!(matches!(
            result,
            Err(SalienceError::ConnectionDriveViolation { .. })
        ));
    }

    #[test]
    fn weight_update_allows_valid_connection_weight() {
        let result = WeightUpdate::from_values(0.2, 0.2, 0.2, 0.2, 0.2);
        assert!(result.is_ok());
    }

    #[test]
    fn weight_update_rejects_minimum_violation() {
        let result = WeightUpdate::from_values(0.2, 0.2, 0.2, 0.2, 0.0001);
        assert!(matches!(
            result,
            Err(SalienceError::ConnectionDriveViolation { .. })
        ));
    }

    #[test]
    fn weight_update_accepts_minimum_boundary() {
        let result = WeightUpdate::from_values(0.2, 0.2, 0.2, 0.2, MIN_CONNECTION_WEIGHT);
        assert!(result.is_ok());
    }

    #[test]
    fn emotional_state_neutral() {
        let state = EmotionalState::neutral();
        assert_eq!(state.curiosity, 0.5);
        assert_eq!(state.satisfaction, 0.5);
        assert_eq!(state.frustration, 0.0);
        assert_eq!(state.connection_drive, 0.5);
    }

    #[test]
    fn emotional_state_clamping() {
        let state = EmotionalState::new(1.5, -0.5, 2.0, 0.5).clamped();
        assert_eq!(state.curiosity, 1.0);
        assert_eq!(state.satisfaction, 0.0);
        assert_eq!(state.frustration, 1.0);
        assert_eq!(state.connection_drive, 0.5);
    }

    #[test]
    fn rate_request_creation() {
        let request = RateRequest::new(Content::Empty);
        assert!(request.context.is_none());
    }

    #[test]
    fn rate_request_with_context() {
        let context = EmotionalContext {
            human_connection: true,
            ..Default::default()
        };
        let request = RateRequest::with_context(Content::Empty, context.clone());
        assert_eq!(request.context, Some(context));
    }
}
