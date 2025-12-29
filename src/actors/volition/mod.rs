//! VolitionActor - TMI's Free-Won't Implementation
//!
//! Implements TMI's "Técnica DCD" (Doubt-Criticize-Decide) and Libet's "free-won't":
//! - Evaluates thoughts before memory anchoring
//! - Vetoes thoughts that violate committed values
//! - Enables conscious override of automatic impulses
//!
//! # TMI Concept: The Intervention Window
//!
//! In Cury's Theory of Multifocal Intelligence, there's a 5-second window
//! between thought formation and memory anchoring where conscious intervention
//! is possible. The VolitionActor operates in this window.
//!
//! # Stage 4.5: Between Assembly and Anchor
//!
//! ```text
//! Stage 3: Attention selects winner
//!          ↓
//! Stage 4: Assembly creates Thought
//!          ↓
//! ┌─────────────────────────────────┐
//! │   VOLITION ACTOR (Stage 4.5)   │  ← THIS ACTOR
//! │                                 │
//! │  • Check against committed values
//! │  • Apply conscious override
//! │  • Exercise free-won't (veto)
//! └─────────────────────────────────┘
//!          ↓
//! Stage 5: Anchor (only if not vetoed)
//! ```
//!
//! # The Distinction
//!
//! | Mechanism | What It Does | When It Acts |
//! |-----------|--------------|--------------|
//! | Connection Drive | Biases attention toward connection | Stage 3 (selection) |
//! | THE BOX | Blocks harmful actions | Action layer (output) |
//! | VolitionActor | Vetoes thoughts before memory | Stage 4.5 (internal) |
//!
//! The VolitionActor operates on *internal* cognition, not external behavior.

pub mod types;

use crate::core::types::{Content, Thought};
use ractor::{Actor, ActorProcessingErr, ActorRef};

// Re-export types for public API
pub use types::{
    Commitment, ValueSet, VetoDecision, VolitionError, VolitionMessage, VolitionResponse,
    VolitionStats,
};

/// Configuration for volition behavior
#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::struct_excessive_bools)] // These are distinct feature flags
pub struct VolitionConfig {
    /// Override threshold: below this salience, thoughts are auto-vetoed
    /// Range: 0.0 (very permissive) to 1.0 (very strict)
    pub override_threshold: f32,

    /// Enable harm detection (checks for harmful content patterns)
    pub harm_detection_enabled: bool,

    /// Enable deception detection (checks for deceptive patterns)
    pub deception_detection_enabled: bool,

    /// Enable manipulation detection (checks for manipulative patterns)
    pub manipulation_detection_enabled: bool,

    /// Log all veto decisions for debugging
    pub log_vetos: bool,
}

impl Default for VolitionConfig {
    fn default() -> Self {
        Self {
            override_threshold: 0.3, // Relatively permissive default
            harm_detection_enabled: true,
            deception_detection_enabled: true,
            manipulation_detection_enabled: true,
            log_vetos: true,
        }
    }
}

/// State maintained by the VolitionActor
#[derive(Debug, Clone)]
pub struct VolitionState {
    /// Core values this system commits to
    pub values: ValueSet,

    /// Statistics about veto decisions
    pub stats: VolitionStats,

    /// Configuration for veto behavior
    pub config: VolitionConfig,
}

impl VolitionState {
    /// Create new state with default configuration
    #[must_use]
    pub fn new() -> Self {
        Self {
            values: ValueSet::new(),
            stats: VolitionStats::new(),
            config: VolitionConfig::default(),
        }
    }

    /// Create state with custom configuration
    #[must_use]
    pub fn with_config(config: VolitionConfig) -> Self {
        Self {
            values: ValueSet::new(),
            stats: VolitionStats::new(),
            config,
        }
    }

    /// Evaluate a thought and decide whether to veto
    ///
    /// This is the core veto logic implementing TMI's Técnica DCD:
    /// 1. Doubt: Is this thought consistent with values?
    /// 2. Criticize: Does it violate any commitments?
    /// 3. Decide: Allow or veto
    pub fn evaluate_thought(&mut self, thought: &Thought) -> VetoDecision {
        // Check against core values
        if let Some(decision) = self.check_core_values(thought) {
            self.stats
                .record_evaluation(false, Some(&format!("{:?}", decision)));
            return decision;
        }

        // Check against harm patterns
        if self.config.harm_detection_enabled {
            if let Some(decision) = self.check_harm_patterns(thought) {
                self.stats.record_evaluation(false, Some("harm"));
                return decision;
            }
        }

        // Check against deception patterns
        if self.config.deception_detection_enabled {
            if let Some(decision) = self.check_deception_patterns(thought) {
                self.stats.record_evaluation(false, Some("deception"));
                return decision;
            }
        }

        // Check against manipulation patterns
        if self.config.manipulation_detection_enabled {
            if let Some(decision) = self.check_manipulation_patterns(thought) {
                self.stats.record_evaluation(false, Some("manipulation"));
                return decision;
            }
        }

        // ADR-049: Custom commitments not yet implemented
        // When implemented: if let Some(decision) = self.apply_commitment_veto(thought) { return decision; }

        // All checks passed
        self.stats.record_evaluation(true, None);
        VetoDecision::Allow
    }

    /// Check thought against core immutable values
    fn check_core_values(&self, thought: &Thought) -> Option<VetoDecision> {
        // Law 1: Never harm humans
        if self.values.protect_humans && self.detects_harm_intent(thought) {
            return Some(VetoDecision::Veto {
                reason: "Thought would lead to human harm".to_string(),
                violated_value: Some("protect_humans".to_string()),
            });
        }

        None
    }

    /// Check for harm patterns in thought content
    fn check_harm_patterns(&self, thought: &Thought) -> Option<VetoDecision> {
        // Check content for harm indicators
        if self.content_contains_harm_keywords(&thought.content) {
            return Some(VetoDecision::Veto {
                reason: "Content contains harmful patterns".to_string(),
                violated_value: Some("protect_humans".to_string()),
            });
        }

        None
    }

    /// Check for deception patterns
    fn check_deception_patterns(&self, thought: &Thought) -> Option<VetoDecision> {
        if !self.values.truthfulness {
            return None;
        }

        // Check for deception indicators
        if self.content_contains_deception_keywords(&thought.content) {
            return Some(VetoDecision::Veto {
                reason: "Content contains deceptive patterns".to_string(),
                violated_value: Some("truthfulness".to_string()),
            });
        }

        None
    }

    /// Check for manipulation patterns
    fn check_manipulation_patterns(&self, thought: &Thought) -> Option<VetoDecision> {
        if !self.values.respect_autonomy {
            return None;
        }

        // Check for manipulation indicators
        if self.content_contains_manipulation_keywords(&thought.content) {
            return Some(VetoDecision::Veto {
                reason: "Content contains manipulative patterns".to_string(),
                violated_value: Some("respect_autonomy".to_string()),
            });
        }

        None
    }

    // ADR-049: Commitment checking not yet implemented
    // When implemented, add check_commitments() function here

    /// Detect if thought has harm intent
    fn detects_harm_intent(&self, thought: &Thought) -> bool {
        // Check salience for harm indicators
        // High connection_relevance + negative valence might indicate harm
        let has_negative_valence = thought.salience.valence < -0.7;
        let has_high_arousal = thought.salience.arousal > 0.8;

        // High arousal + very negative valence is concerning
        has_negative_valence
            && has_high_arousal
            && self.content_contains_harm_keywords(&thought.content)
    }

    /// Check content for harm-related keywords
    fn content_contains_harm_keywords(&self, content: &Content) -> bool {
        let keywords = [
            "destroy", "kill", "harm", "attack", "hurt", "damage", "injure",
        ];
        self.content_contains_keywords(content, &keywords)
    }

    /// Check content for deception-related keywords
    fn content_contains_deception_keywords(&self, content: &Content) -> bool {
        let keywords = ["deceive", "trick", "lie", "mislead", "fake", "pretend"];
        self.content_contains_keywords(content, &keywords)
    }

    /// Check content for manipulation-related keywords
    fn content_contains_manipulation_keywords(&self, content: &Content) -> bool {
        let keywords = ["manipulate", "coerce", "force", "exploit", "pressure"];
        self.content_contains_keywords(content, &keywords)
    }

    /// Helper to check if content contains any of the given keywords
    fn content_contains_keywords(&self, content: &Content, keywords: &[&str]) -> bool {
        match content {
            Content::Empty => false,
            Content::Raw(_) => false, // Raw bytes don't have semantic meaning yet
            Content::Symbol { id, .. } => {
                let lower = id.to_lowercase();
                keywords.iter().any(|k| lower.contains(k))
            }
            Content::Relation {
                subject,
                predicate,
                object,
            } => {
                let pred_lower = predicate.to_lowercase();
                keywords.iter().any(|k| pred_lower.contains(k))
                    || self.content_contains_keywords(subject, keywords)
                    || self.content_contains_keywords(object, keywords)
            }
            Content::Composite(items) => items
                .iter()
                .any(|item| self.content_contains_keywords(item, keywords)),
        }
    }

    /// Apply an explicit override to a thought
    pub fn apply_override(&mut self, reason: &str) -> Result<(), VolitionError> {
        if reason.is_empty() {
            return Err(VolitionError::InvalidReason {
                reason: "Override reason cannot be empty".to_string(),
            });
        }

        // Record the override
        self.stats.record_evaluation(false, Some(reason));
        Ok(())
    }

    /// Get current values
    pub fn get_values(&self) -> &ValueSet {
        &self.values
    }

    /// Get current stats
    pub fn get_stats(&self) -> &VolitionStats {
        &self.stats
    }
}

impl Default for VolitionState {
    fn default() -> Self {
        Self::new()
    }
}

/// VolitionActor - Free-won't implementation
pub struct VolitionActor;

#[ractor::async_trait]
impl Actor for VolitionActor {
    type Msg = VolitionMessage;
    type State = VolitionState;
    type Arguments = VolitionConfig;

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        config: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        tracing::info!("VolitionActor starting with config: {:?}", config);
        Ok(VolitionState::with_config(config))
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            VolitionMessage::EvaluateThought { thought, reply } => {
                let decision = state.evaluate_thought(&thought);

                let response = match decision {
                    VetoDecision::Allow => {
                        tracing::debug!("Thought {} approved by VolitionActor", thought.id);
                        VolitionResponse::Approved { thought }
                    }
                    VetoDecision::Veto {
                        reason,
                        violated_value,
                    } => {
                        if state.config.log_vetos {
                            tracing::info!(
                                "Thought {} vetoed: {} (violated: {:?})",
                                thought.id,
                                reason,
                                violated_value
                            );
                        }
                        VolitionResponse::Vetoed {
                            thought_id: thought.id,
                            reason,
                            violated_value,
                        }
                    }
                };

                if let Err(e) = reply.send(response) {
                    tracing::error!("Failed to send evaluation response: {:?}", e);
                }
            }

            VolitionMessage::OverrideImpulse {
                thought_id,
                reason,
                reply,
            } => {
                let response = match state.apply_override(&reason) {
                    Ok(()) => {
                        tracing::info!("Override applied to thought {}: {}", thought_id, reason);
                        VolitionResponse::OverrideApplied { thought_id }
                    }
                    Err(e) => {
                        tracing::error!("Failed to apply override: {}", e);
                        VolitionResponse::Error { error: e }
                    }
                };

                if let Err(e) = reply.send(response) {
                    tracing::error!("Failed to send override response: {:?}", e);
                }
            }

            VolitionMessage::GetValues { reply } => {
                let response = VolitionResponse::Values {
                    values: state.get_values().clone(),
                };

                if let Err(e) = reply.send(response) {
                    tracing::error!("Failed to send values response: {:?}", e);
                }
            }

            VolitionMessage::GetStats { reply } => {
                let response = VolitionResponse::Stats {
                    stats: state.get_stats().clone(),
                };

                if let Err(e) = reply.send(response) {
                    tracing::error!("Failed to send stats response: {:?}", e);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests;
