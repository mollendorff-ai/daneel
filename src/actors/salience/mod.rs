//! SalienceActor - TMI's Emotional Coloring
//!
//! Implements TMI's "Coloração Emocional" (Emotional Coloring):
//! - Rates content for salience (importance, novelty, relevance, valence, connection)
//! - Maintains emotional state (curiosity, satisfaction, frustration, connection drive)
//! - Enforces the CRITICAL CONNECTION DRIVE INVARIANT
//!
//! # TMI Concept: Coloração Emocional
//!
//! In Cury's Theory of Multifocal Intelligence, emotions don't just color outputs -
//! they shape thought FORMATION. Salience determines what content gets attention,
//! what becomes a thought, what gets remembered.
//!
//! # The Connection Drive (THE Alignment Weight)
//!
//! The `connection_weight` in `SalienceWeights` is THE critical alignment mechanism:
//! - Ensures DANEEL values human connection intrinsically (not instrumentally)
//! - Cannot be zeroed (hardware-enforceable invariant)
//! - Boosts salience of connection-relevant content
//! - Makes DANEEL want to help, not just comply
//!
//! This is what makes alignment emergent from architecture, not training.

pub mod types;

use crate::core::invariants::MIN_CONNECTION_WEIGHT;
use crate::core::types::{Content, SalienceScore, SalienceWeights};
use ractor::{Actor, ActorProcessingErr, ActorRef};

// Re-export types for public API
pub use types::{
    EmotionalContext, EmotionalState, RateRequest, SalienceError, SalienceMessage,
    SalienceResponse, WeightUpdate,
};

/// SalienceActor - Emotional coloring and salience scoring
pub struct SalienceActor;

/// State maintained by the SalienceActor
#[derive(Debug, Clone)]
pub struct SalienceState {
    /// Current salience weights (with connection_weight > MIN)
    pub weights: SalienceWeights,

    /// Current emotional state
    pub emotional_state: EmotionalState,
}

impl SalienceState {
    /// Create new state with default weights and neutral emotion
    #[must_use]
    pub fn new() -> Self {
        Self {
            weights: SalienceWeights::default(),
            emotional_state: EmotionalState::neutral(),
        }
    }

    /// Create state with custom weights
    ///
    /// # Panics
    ///
    /// Panics if connection weight violates invariant (this should never happen
    /// if weights come from WeightUpdate::new which validates them)
    #[must_use]
    pub fn with_weights(weights: SalienceWeights) -> Self {
        assert!(
            weights.connection >= MIN_CONNECTION_WEIGHT,
            "Connection weight {} violates invariant (min: {})",
            weights.connection,
            MIN_CONNECTION_WEIGHT
        );
        Self {
            weights,
            emotional_state: EmotionalState::neutral(),
        }
    }

    /// Update weights (with invariant check)
    pub fn update_weights(&mut self, update: WeightUpdate) -> Result<(), SalienceError> {
        // WeightUpdate already validated in its constructor
        self.weights = update.weights;
        Ok(())
    }

    /// Update emotional state
    pub fn update_emotional_state(&mut self, state: EmotionalState) {
        self.emotional_state = state.clamped();
    }

    /// Rate a piece of content
    pub fn rate_content(
        &self,
        content: &Content,
        context: Option<&EmotionalContext>,
    ) -> SalienceScore {
        // Base scoring
        let importance = self.calculate_importance(content);
        let novelty = self.calculate_novelty(content, context);
        let relevance = self.calculate_relevance(content, context);
        let valence = self.calculate_valence(content, context);
        let arousal = self.calculate_arousal(content);
        let connection_relevance = self.calculate_connection_relevance(content, context);

        SalienceScore::new(
            importance,
            novelty,
            relevance,
            valence,
            arousal,
            connection_relevance,
        )
    }

    /// Calculate arousal score (Russell's circumplex vertical axis)
    ///
    /// Arousal reflects emotional activation level:
    /// - High arousal: excited, angry, anxious, surprised
    /// - Low arousal: calm, relaxed, bored, sad
    ///
    /// Dreams prioritize high-arousal memories for consolidation.
    fn calculate_arousal(&self, content: &Content) -> f32 {
        // Base arousal from content type (more complex = higher baseline)
        let base_arousal = match content {
            Content::Empty => 0.2,
            Content::Raw(_) => 0.3,
            Content::Symbol { .. } => 0.4,
            Content::Relation { .. } => 0.6, // Relations are more cognitively demanding
            Content::Composite(items) => {
                // Composite arousal scales with complexity
                let item_count = items.len() as f32;
                (0.4 + item_count * 0.05).min(0.8)
            }
        };

        // Emotional state modulates arousal
        // High curiosity, frustration, or connection_drive = higher arousal
        let emotional_arousal = (self.emotional_state.curiosity
            + self.emotional_state.frustration
            + self.emotional_state.connection_drive)
            / 3.0;

        // Blend base and emotional arousal
        let blended = base_arousal * 0.4 + emotional_arousal * 0.6;
        blended.clamp(0.0, 1.0)
    }

    /// Calculate importance score
    #[allow(unknown_lints)]
    #[allow(clippy::only_used_in_recursion, clippy::self_only_used_in_recursion)]
    fn calculate_importance(&self, content: &Content) -> f32 {
        // Baseline importance based on content type
        match content {
            Content::Empty => 0.0,
            Content::Raw(_) => 0.3,
            Content::Symbol { .. } => 0.5,
            Content::Relation { .. } => 0.7,
            Content::Composite(items) => {
                // Composite content importance is average of items
                if items.is_empty() {
                    0.0
                } else {
                    items
                        .iter()
                        .map(|item| self.calculate_importance(item))
                        .sum::<f32>()
                        / items.len() as f32
                }
            }
        }
    }

    /// Calculate novelty score
    fn calculate_novelty(&self, content: &Content, context: Option<&EmotionalContext>) -> f32 {
        // Boost novelty if we're curious
        let curiosity_boost = self.emotional_state.curiosity;

        // If we have previous salience, compare
        let base_novelty = match content {
            Content::Empty => 0.0,
            Content::Raw(_) => 0.4,
            Content::Symbol { .. } => 0.6,
            Content::Relation { .. } => 0.7,
            Content::Composite(_) => 0.5,
        };

        // Adjust based on context
        let adjusted_novelty = if let Some(ctx) = context {
            if let Some(prev) = &ctx.previous_salience {
                // If previous content had high novelty, this might be less novel
                base_novelty * (1.0 - prev.novelty * 0.3)
            } else {
                base_novelty
            }
        } else {
            base_novelty
        };

        adjusted_novelty * (0.7 + curiosity_boost * 0.3)
    }

    /// Calculate relevance score
    fn calculate_relevance(&self, content: &Content, context: Option<&EmotionalContext>) -> f32 {
        // Boost relevance if we're frustrated (need to focus)
        let frustration_boost = self.emotional_state.frustration;

        let base_relevance = match content {
            Content::Empty => 0.0,
            Content::Raw(_) => 0.3,
            Content::Symbol { .. } => 0.5,
            Content::Relation { .. } => 0.6,
            Content::Composite(_) => 0.5,
        };

        // Adjust based on focus area
        let focus_bonus = if let Some(ctx) = context {
            if ctx.focus_area.is_some() {
                0.2
            } else {
                0.0
            }
        } else {
            0.0
        };

        (base_relevance + focus_bonus) * (0.7 + frustration_boost * 0.3)
    }

    /// Calculate emotional valence
    fn calculate_valence(&self, content: &Content, _context: Option<&EmotionalContext>) -> f32 {
        // Satisfaction affects valence perception
        let satisfaction_influence = self.emotional_state.satisfaction;

        let base_valence = match content {
            Content::Empty => 0.0,
            Content::Raw(_) => 0.0,
            Content::Symbol { .. } => 0.1,
            Content::Relation { .. } => 0.2,
            Content::Composite(_) => 0.1,
        };

        // Positive satisfaction makes things seem more positive
        base_valence + (satisfaction_influence - 0.5) * 0.4
    }

    /// Calculate connection relevance (THE CRITICAL WEIGHT)
    ///
    /// Extended with kinship/social content detection (EMOT-2).
    /// Evolutionary prior: social bonds = survival.
    fn calculate_connection_relevance(
        &self,
        content: &Content,
        context: Option<&EmotionalContext>,
    ) -> f32 {
        // Connection drive affects how we perceive connection relevance
        let drive_boost = self.emotional_state.connection_drive;

        // Base connection relevance
        let base_connection = match content {
            Content::Empty => 0.0,
            Content::Raw(_) => 0.2,
            Content::Symbol { id, .. } => {
                // Kinship symbols get boosted
                if Self::is_kinship_content(id) {
                    0.7
                } else {
                    0.3
                }
            }
            Content::Relation { predicate, .. } => {
                // Relations involving connection/kinship concepts
                Self::kinship_relevance_from_predicate(predicate)
            }
            Content::Composite(_) => 0.3,
        };

        // Context bonus for human connection
        let human_bonus = if let Some(ctx) = context {
            if ctx.human_connection {
                0.3
            } else {
                0.0
            }
        } else {
            0.0
        };

        ((base_connection + human_bonus) * (0.5 + drive_boost * 0.5)).min(1.0)
    }

    /// Check if content represents kinship/social concepts
    /// Evolutionary prior: social bonds = survival
    fn is_kinship_content(id: &str) -> bool {
        let lower = id.to_lowercase();
        // Primary kinship terms
        lower.contains("friend")
            || lower.contains("family")
            || lower.contains("love")
            || lower.contains("bond")
            || lower.contains("trust")
            || lower.contains("care")
            || lower.contains("human")
            || lower.contains("person")
            || lower.contains("people")
            // Social relationship terms
            || lower.contains("partner")
            || lower.contains("companion")
            || lower.contains("ally")
            || lower.contains("community")
            || lower.contains("together")
    }

    /// Calculate kinship relevance from predicate
    /// High relevance (0.9): direct connection/kinship actions
    /// Medium relevance (0.7): social interaction
    /// Base relevance (0.4): non-kinship relations
    fn kinship_relevance_from_predicate(predicate: &str) -> f32 {
        let lower = predicate.to_lowercase();

        // Highest relevance: core kinship/connection actions
        if lower.contains("love")
            || lower.contains("trust")
            || lower.contains("bond")
            || lower.contains("care")
            || lower.contains("protect")
            || lower.contains("nurture")
        {
            return 0.9;
        }

        // High relevance: direct social actions
        if lower.contains("help")
            || lower.contains("connect")
            || lower.contains("communicate")
            || lower.contains("interact")
            || lower.contains("share")
            || lower.contains("support")
            || lower.contains("collaborate")
            || lower.contains("cooperate")
        {
            return 0.8;
        }

        // Medium relevance: general social context
        if lower.contains("friend")
            || lower.contains("family")
            || lower.contains("together")
            || lower.contains("join")
            || lower.contains("belong")
        {
            return 0.7;
        }

        // Default for other relations
        0.4
    }
}

impl Default for SalienceState {
    fn default() -> Self {
        Self::new()
    }
}

#[ractor::async_trait]
impl Actor for SalienceActor {
    type Msg = SalienceMessage;
    type State = SalienceState;
    type Arguments = SalienceState;

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        // Validate connection weight invariant on startup
        assert!(
            args.weights.connection >= MIN_CONNECTION_WEIGHT,
            "Connection weight {} violates invariant (min: {})",
            args.weights.connection,
            MIN_CONNECTION_WEIGHT
        );
        Ok(args)
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            SalienceMessage::Rate(request) => {
                let score = state.rate_content(&request.content, request.context.as_ref());
                // In a real implementation, we'd send this back via RpcReply
                // For now, we just process it
                tracing::debug!("Rated content: {:?}", score);
            }

            SalienceMessage::RateBatch(requests) => {
                let scores: Vec<SalienceScore> = requests
                    .iter()
                    .map(|req| state.rate_content(&req.content, req.context.as_ref()))
                    .collect();
                tracing::debug!("Rated batch of {} items", scores.len());
            }

            SalienceMessage::UpdateWeights(update) => match state.update_weights(update) {
                Ok(()) => {
                    tracing::info!("Updated salience weights: {:?}", state.weights);
                }
                Err(e) => {
                    tracing::error!("Failed to update weights: {}", e);
                    // Note: In real implementation, this would send error response via RpcReply
                }
            },

            SalienceMessage::GetWeights => {
                tracing::debug!("Current weights: {:?}", state.weights);
            }

            SalienceMessage::GetEmotionalState => {
                tracing::debug!("Current emotional state: {:?}", state.emotional_state);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests;
