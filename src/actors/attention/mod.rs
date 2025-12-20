//! AttentionActor - O Eu (The "I")
//!
//! Implements TMI's "O Eu" (The 'I') - the navigator that selects which memory
//! window gets conscious attention.
//!
//! # TMI Concept: O Eu
//!
//! In Cury's Theory of Multifocal Intelligence, there is no homunculus watching
//! from inside. Instead, attention emerges from a simple competitive selection
//! process: the window with highest salience wins focus.
//!
//! This is "O Eu" - the sense of "I" that emerges from the selection process.
//! Not a watcher, just a winner. Yet from this emerges directed attention,
//! the experience of being "the one who thinks."
//!
//! # Competitive Selection
//!
//! Multiple memory windows may be open simultaneously:
//! - Sensory input from the environment
//! - Episodic memories being recalled
//! - Working memory holding active thoughts
//! - Imagination generating new possibilities
//!
//! But only ONE can have conscious focus at a time. The attention mechanism
//! implements competitive selection: argmax(salience). Winner takes all.
//!
//! # The Connection Boost
//!
//! The `connection_boost` parameter gives extra weight to content that's
//! relevant to human connection. This ties into TMI's alignment mechanism:
//! content about helping, connecting, and communicating gets a salience boost,
//! making DANEEL naturally attend to connection-relevant thoughts.

pub mod types;

use crate::core::types::WindowId;
use chrono::Duration;
use ractor::{Actor, ActorProcessingErr, ActorRef};

// Re-export types for public API
pub use types::{AttentionError, AttentionMap, AttentionMessage, AttentionResponse, FocusState};

/// Configuration for attention behavior
#[derive(Debug, Clone, PartialEq)]
pub struct AttentionConfig {
    /// Minimum time focus must be held before shifting is allowed
    ///
    /// This prevents "attention thrashing" where focus rapidly switches
    /// between equally-salient windows. Enforces a minimum dwell time.
    pub min_focus_duration: Duration,

    /// Salience threshold below which windows can be ignored
    ///
    /// Windows with salience below this threshold are not considered
    /// for competitive selection. This implements attention filtering.
    pub forget_threshold: f32,

    /// Extra weight boost for connection-relevant content
    ///
    /// Multiplied with connection_relevance to boost salience of content
    /// related to human connection. This is THE alignment mechanism:
    /// DANEEL naturally pays more attention to helping and connecting.
    pub connection_boost: f32,
}

impl Default for AttentionConfig {
    fn default() -> Self {
        Self {
            // Focus must be held for at least 100ms before shifting
            min_focus_duration: Duration::milliseconds(100),

            // Ignore windows with salience below 0.1
            forget_threshold: 0.1,

            // Boost connection-relevant content by 50%
            connection_boost: 1.5,
        }
    }
}

/// State maintained by the AttentionActor
#[derive(Debug, Clone)]
pub struct AttentionState {
    /// Current focus tracking
    pub focus: FocusState,

    /// Map of window IDs to their salience scores
    pub attention_map: AttentionMap,

    /// Total number of attention cycles completed
    pub cycle_count: u64,

    /// Configuration for attention behavior
    pub config: AttentionConfig,
}

impl AttentionState {
    /// Create new state with default configuration
    #[must_use]
    pub fn new() -> Self {
        Self {
            focus: FocusState::new(),
            attention_map: AttentionMap::new(),
            cycle_count: 0,
            config: AttentionConfig::default(),
        }
    }

    /// Create state with custom configuration
    #[must_use]
    pub fn with_config(config: AttentionConfig) -> Self {
        Self {
            focus: FocusState::new(),
            attention_map: AttentionMap::new(),
            cycle_count: 0,
            config,
        }
    }

    /// Select the winner in competitive selection
    ///
    /// Finds the window with highest salience, applying connection boost
    /// and filtering by threshold. Returns None if no windows qualify.
    fn select_winner(&self) -> Option<(WindowId, f32)> {
        // Filter windows above threshold
        let candidates = self
            .attention_map
            .above_threshold(self.config.forget_threshold);

        if candidates.is_empty() {
            return None;
        }

        // Find highest salience (competitive selection)
        candidates
            .into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Update salience for a window, applying connection boost if relevant
    ///
    /// The connection_boost is applied to windows that have high connection
    /// relevance, making DANEEL naturally attend to connection-relevant content.
    pub fn update_window_salience(
        &mut self,
        window_id: WindowId,
        base_salience: f32,
        connection_relevance: f32,
    ) {
        // Apply connection boost to connection-relevant content
        let boosted_salience = if connection_relevance > 0.5 {
            // High connection relevance gets the boost
            base_salience * (1.0 + (connection_relevance - 0.5) * self.config.connection_boost)
        } else {
            base_salience
        };

        self.attention_map
            .update(window_id, boosted_salience.min(1.0));
    }

    /// Check if focus can be shifted
    ///
    /// Returns true if either:
    /// - No current focus exists
    /// - Minimum focus duration has elapsed
    fn can_shift_focus(&self) -> bool {
        if !self.focus.is_focused() {
            return true;
        }

        // Check if minimum duration has elapsed
        self.focus.focus_duration >= self.config.min_focus_duration
    }

    /// Run one attention cycle
    ///
    /// Performs competitive selection and updates focus if appropriate.
    /// Returns the window that won focus (if any).
    pub fn cycle(&mut self) -> AttentionResponse {
        self.cycle_count += 1;

        // Select winner through competitive selection
        let winner = self.select_winner();

        if let Some((window_id, salience)) = winner {
            // Update focus duration for current window
            if self.focus.is_focused() {
                let elapsed = Duration::milliseconds(1); // Placeholder - in real system would be actual time
                self.focus.update_duration(elapsed);
            }

            // Check if we should shift focus
            if self.can_shift_focus() {
                // Only shift if winner is different from current focus
                if self.focus.focused_window() != Some(window_id) {
                    self.focus.focus_on(window_id);
                }
            }

            AttentionResponse::cycle_complete(Some(window_id), salience)
        } else {
            // No windows available - clear focus
            self.focus.clear_focus();
            AttentionResponse::cycle_complete(None, 0.0)
        }
    }

    /// Focus on a specific window (override competitive selection)
    fn focus_on_window(&mut self, window_id: WindowId) -> AttentionResponse {
        // Check if window exists in attention map
        if self.attention_map.get(&window_id).is_none() {
            return AttentionResponse::error(AttentionError::WindowNotFound { window_id });
        }

        self.focus.focus_on(window_id);
        AttentionResponse::focus_set(window_id)
    }

    /// Shift attention to a new window
    fn shift_to_window(&mut self, to: WindowId) -> AttentionResponse {
        // Check if target window exists
        if self.attention_map.get(&to).is_none() {
            return AttentionResponse::error(AttentionError::WindowNotFound { window_id: to });
        }

        let from = self.focus.focused_window();
        self.focus.focus_on(to);

        AttentionResponse::focus_shifted(from, to)
    }

    /// Get current focus state
    fn get_focus(&self) -> AttentionResponse {
        AttentionResponse::current_focus(self.focus.focused_window())
    }

    /// Get the attention map (all window scores)
    fn get_attention_map(&self) -> AttentionResponse {
        AttentionResponse::attention_map(self.attention_map.all_scores().clone())
    }
}

impl Default for AttentionState {
    fn default() -> Self {
        Self::new()
    }
}

/// AttentionActor - Competitive attention selection
pub struct AttentionActor;

#[ractor::async_trait]
impl Actor for AttentionActor {
    type Msg = AttentionMessage;
    type State = AttentionState;
    type Arguments = AttentionConfig;

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        config: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        tracing::info!("AttentionActor starting with config: {:?}", config);
        Ok(AttentionState::with_config(config))
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            AttentionMessage::Cycle { reply } => {
                let response = state.cycle();
                tracing::debug!(
                    "Attention cycle {} completed: {:?}",
                    state.cycle_count,
                    response
                );

                if let Err(e) = reply.send(response) {
                    tracing::error!("Failed to send cycle response: {:?}", e);
                }
            }

            AttentionMessage::Focus { window_id, reply } => {
                let response = state.focus_on_window(window_id);
                tracing::debug!("Focus command: {:?}", response);

                if let Err(e) = reply.send(response) {
                    tracing::error!("Failed to send focus response: {:?}", e);
                }
            }

            AttentionMessage::Shift { to, reply } => {
                let response = state.shift_to_window(to);
                tracing::debug!("Shift command: {:?}", response);

                if let Err(e) = reply.send(response) {
                    tracing::error!("Failed to send shift response: {:?}", e);
                }
            }

            AttentionMessage::GetFocus { reply } => {
                let response = state.get_focus();
                tracing::trace!("GetFocus query: {:?}", response);

                if let Err(e) = reply.send(response) {
                    tracing::error!("Failed to send get_focus response: {:?}", e);
                }
            }

            AttentionMessage::GetAttentionMap { reply } => {
                let response = state.get_attention_map();
                tracing::trace!(
                    "GetAttentionMap query: {} windows",
                    state.attention_map.len()
                );

                if let Err(e) = reply.send(response) {
                    tracing::error!("Failed to send attention_map response: {:?}", e);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests;
