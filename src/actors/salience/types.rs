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

/// ADR-049: Test modules excluded from coverage
#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
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
    fn weight_update_new_valid() {
        let weights = SalienceWeights {
            importance: 0.2,
            novelty: 0.2,
            relevance: 0.2,
            valence: 0.2,
            connection: 0.2,
        };
        let result = WeightUpdate::new(weights);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().weights, weights);
    }

    #[test]
    fn weight_update_new_invalid() {
        let weights = SalienceWeights {
            importance: 0.2,
            novelty: 0.2,
            relevance: 0.2,
            valence: 0.2,
            connection: 0.0,
        };
        let result = WeightUpdate::new(weights);
        assert!(matches!(
            result,
            Err(SalienceError::ConnectionDriveViolation { .. })
        ));
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
    fn emotional_state_default() {
        let state = EmotionalState::default();
        let neutral = EmotionalState::neutral();
        assert_eq!(state, neutral);
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
    fn emotional_state_clamping_connection_drive() {
        let state = EmotionalState::new(0.5, 0.5, 0.5, 1.5).clamped();
        assert_eq!(state.connection_drive, 1.0);

        let state2 = EmotionalState::new(0.5, 0.5, 0.5, -0.5).clamped();
        assert_eq!(state2.connection_drive, 0.0);
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

    #[test]
    fn emotional_context_default() {
        let context = EmotionalContext::default();
        assert_eq!(context.previous_salience, None);
        assert!(!context.human_connection);
        assert_eq!(context.focus_area, None);
    }

    #[test]
    fn salience_error_connection_drive_violation_display() {
        let error = SalienceError::ConnectionDriveViolation {
            attempted: 0.05,
            minimum: 0.1,
        };
        let msg = error.to_string();
        assert!(msg.contains("0.05"));
        assert!(msg.contains("0.1"));
    }

    #[test]
    fn salience_error_invalid_weight_display() {
        let error = SalienceError::InvalidWeight {
            field: "importance".to_string(),
            value: -0.5,
        };
        let msg = error.to_string();
        assert!(msg.contains("importance"));
        assert!(msg.contains("-0.5"));
    }

    #[test]
    fn salience_error_invalid_emotional_state_display() {
        let error = SalienceError::InvalidEmotionalState {
            field: "curiosity".to_string(),
            value: 1.5,
        };
        let msg = error.to_string();
        assert!(msg.contains("curiosity"));
        assert!(msg.contains("1.5"));
    }

    #[test]
    fn salience_message_debug_and_clone() {
        let msg = SalienceMessage::Rate(RateRequest::new(Content::Empty));
        let cloned = msg.clone();
        let debug_str = format!("{:?}", cloned);
        assert!(debug_str.contains("Rate"));

        let msg_batch = SalienceMessage::RateBatch(vec![RateRequest::new(Content::Empty)]);
        let debug_batch = format!("{:?}", msg_batch.clone());
        assert!(debug_batch.contains("RateBatch"));

        let msg_update = SalienceMessage::UpdateWeights(
            WeightUpdate::from_values(0.2, 0.2, 0.2, 0.2, 0.2).unwrap(),
        );
        let debug_update = format!("{:?}", msg_update.clone());
        assert!(debug_update.contains("UpdateWeights"));

        let msg_get_weights = SalienceMessage::GetWeights;
        let debug_get_weights = format!("{:?}", msg_get_weights.clone());
        assert!(debug_get_weights.contains("GetWeights"));

        let msg_get_emotional = SalienceMessage::GetEmotionalState;
        let debug_get_emotional = format!("{:?}", msg_get_emotional.clone());
        assert!(debug_get_emotional.contains("GetEmotionalState"));
    }

    #[test]
    fn salience_response_variants() {
        let weights = SalienceWeights {
            importance: 0.2,
            novelty: 0.2,
            relevance: 0.2,
            valence: 0.2,
            connection: 0.2,
        };

        let resp_score = SalienceResponse::Score(SalienceScore::new(0.5, 0.5, 0.5, 0.5, 0.5, 0.5));
        let cloned_score = resp_score.clone();
        assert_eq!(resp_score, cloned_score);
        assert!(format!("{:?}", resp_score).contains("Score"));

        let resp_batch =
            SalienceResponse::ScoreBatch(vec![SalienceScore::new(0.5, 0.5, 0.5, 0.5, 0.5, 0.5)]);
        let cloned_batch = resp_batch.clone();
        assert_eq!(resp_batch, cloned_batch);
        assert!(format!("{:?}", resp_batch).contains("ScoreBatch"));

        let resp_updated = SalienceResponse::WeightsUpdated(weights);
        let cloned_updated = resp_updated.clone();
        assert_eq!(resp_updated, cloned_updated);
        assert!(format!("{:?}", resp_updated).contains("WeightsUpdated"));

        let resp_weights = SalienceResponse::Weights(weights);
        let cloned_weights = resp_weights.clone();
        assert_eq!(resp_weights, cloned_weights);
        assert!(format!("{:?}", resp_weights).contains("Weights("));

        let resp_emotional = SalienceResponse::EmotionalState(EmotionalState::neutral());
        let cloned_emotional = resp_emotional.clone();
        assert_eq!(resp_emotional, cloned_emotional);
        assert!(format!("{:?}", resp_emotional).contains("EmotionalState"));

        let resp_error = SalienceResponse::Error(SalienceError::InvalidWeight {
            field: "test".to_string(),
            value: 0.0,
        });
        let cloned_error = resp_error.clone();
        assert_eq!(resp_error, cloned_error);
        assert!(format!("{:?}", resp_error).contains("Error"));
    }

    #[test]
    fn rate_request_debug_and_clone() {
        let request = RateRequest::new(Content::Empty);
        let cloned = request.clone();
        let debug_str = format!("{:?}", cloned);
        assert!(debug_str.contains("RateRequest"));
    }

    #[test]
    fn emotional_context_partial_eq() {
        let ctx1 = EmotionalContext {
            previous_salience: Some(SalienceScore::new(0.5, 0.5, 0.5, 0.5, 0.5, 0.5)),
            human_connection: true,
            focus_area: Some("test".to_string()),
        };
        let ctx2 = ctx1.clone();
        assert_eq!(ctx1, ctx2);
    }

    #[test]
    fn weight_update_partial_eq() {
        let update1 = WeightUpdate::from_values(0.2, 0.2, 0.2, 0.2, 0.2).unwrap();
        let update2 = WeightUpdate::from_values(0.2, 0.2, 0.2, 0.2, 0.2).unwrap();
        assert_eq!(update1, update2);
    }

    #[test]
    fn emotional_state_copy() {
        let state = EmotionalState::neutral();
        let copied = state;
        assert_eq!(state, copied);
    }
}
