//! AttentionActor Types
//!
//! Types for TMI's "O Eu" (The 'I') - the attention mechanism that selects
//! between competing memory windows.
//!
//! # The Attention Mechanism
//!
//! In TMI, attention is the navigator between memory windows. Multiple windows
//! may be open simultaneously (sensory input, episodic memory, working memory),
//! but only ONE can have focus at a time. The attention mechanism implements
//! competitive selection: the window with highest salience wins.
//!
//! This is "O Eu" - the sense of "I" that emerges from the selection process.
//! There is no homunculus, just a competitive algorithm. Yet from this simple
//! mechanism emerges the experience of directed attention.

use crate::core::types::WindowId;
use chrono::{DateTime, Duration, Utc};
use ractor::RpcReplyPort;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Messages that can be sent to the AttentionActor
#[derive(Debug)]
pub enum AttentionMessage {
    /// Trigger one attention cycle
    ///
    /// The attention actor will evaluate all open windows and focus on
    /// the one with highest salience. This is the core competitive selection.
    Cycle {
        /// Reply port for the cycle result
        reply: RpcReplyPort<AttentionResponse>,
    },

    /// Focus on a specific window
    ///
    /// Override competitive selection and force focus on a particular window.
    /// This is useful for external control or testing.
    Focus {
        /// Window to focus on
        window_id: WindowId,
        /// Reply port for confirmation
        reply: RpcReplyPort<AttentionResponse>,
    },

    /// Shift attention to a new window
    ///
    /// Similar to Focus, but explicitly tracks the shift from current focus.
    /// This is the conscious "switching" action.
    Shift {
        /// Window to shift attention to
        to: WindowId,
        /// Reply port for the shift result
        reply: RpcReplyPort<AttentionResponse>,
    },

    /// Get the current focus
    ///
    /// Query which window (if any) currently has attention.
    GetFocus {
        /// Reply port for current focus
        reply: RpcReplyPort<AttentionResponse>,
    },

    /// Get the attention map
    ///
    /// Query the salience scores for all windows being tracked.
    /// This shows the "competition" - what's vying for attention.
    GetAttentionMap {
        /// Reply port for attention map
        reply: RpcReplyPort<AttentionResponse>,
    },
}

/// Responses from the AttentionActor
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttentionResponse {
    /// Attention cycle completed
    CycleComplete {
        /// Window that won focus (None if no windows available)
        focused: Option<WindowId>,
        /// Salience score of the focused window
        salience: f32,
    },

    /// Focus was set on a window
    FocusSet {
        /// The window that now has focus
        window_id: WindowId,
    },

    /// Focus was shifted from one window to another
    FocusShifted {
        /// Previous focus (None if no previous focus)
        from: Option<WindowId>,
        /// New focus
        to: WindowId,
    },

    /// Current focus state
    CurrentFocus {
        /// Currently focused window (None if no focus)
        window_id: Option<WindowId>,
    },

    /// Attention map (salience scores for all windows)
    AttentionMap {
        /// Map of window IDs to their salience scores
        scores: HashMap<WindowId, f32>,
    },

    /// An error occurred
    Error {
        /// The error that occurred
        error: AttentionError,
    },
}

impl AttentionResponse {
    /// Create a cycle complete response
    #[must_use]
    pub const fn cycle_complete(focused: Option<WindowId>, salience: f32) -> Self {
        Self::CycleComplete { focused, salience }
    }

    /// Create a focus set response
    #[must_use]
    pub const fn focus_set(window_id: WindowId) -> Self {
        Self::FocusSet { window_id }
    }

    /// Create a focus shifted response
    #[must_use]
    pub const fn focus_shifted(from: Option<WindowId>, to: WindowId) -> Self {
        Self::FocusShifted { from, to }
    }

    /// Create a current focus response
    #[must_use]
    pub const fn current_focus(window_id: Option<WindowId>) -> Self {
        Self::CurrentFocus { window_id }
    }

    /// Create an attention map response
    #[must_use]
    pub fn attention_map(scores: HashMap<WindowId, f32>) -> Self {
        Self::AttentionMap { scores }
    }

    /// Create an error response
    #[must_use]
    pub fn error(error: AttentionError) -> Self {
        Self::Error { error }
    }
}

/// Errors that can occur in attention operations
#[derive(Debug, Clone, Error, PartialEq, Serialize, Deserialize)]
pub enum AttentionError {
    /// Requested window does not exist
    #[error("Window not found: {window_id}")]
    WindowNotFound {
        /// The window ID that was not found
        window_id: WindowId,
    },

    /// No windows available for attention
    #[error("No windows available for attention selection")]
    NoWindowsAvailable,

    /// Attention cycle failed
    #[error("Attention cycle failed: {reason}")]
    CycleFailed {
        /// Reason for the failure
        reason: String,
    },
}

/// The current focus state
///
/// Tracks which window currently has attention, how long it has been focused,
/// and when the last shift occurred.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FocusState {
    /// Currently focused window (None if no focus)
    pub current_focus: Option<WindowId>,

    /// How long the current focus has been held
    pub focus_duration: Duration,

    /// When the last shift occurred
    pub last_shift: DateTime<Utc>,
}

impl FocusState {
    /// Create a new focus state with no focus
    #[must_use]
    pub fn new() -> Self {
        Self {
            current_focus: None,
            focus_duration: Duration::zero(),
            last_shift: Utc::now(),
        }
    }

    /// Focus on a window
    ///
    /// Updates the focus state to track the new window and resets timing.
    pub fn focus_on(&mut self, window_id: WindowId) {
        let now = Utc::now();

        // If we're shifting focus, record the shift time
        if self.current_focus.is_some() {
            self.last_shift = now;
        }

        self.current_focus = Some(window_id);
        self.focus_duration = Duration::zero();
    }

    /// Clear the current focus
    ///
    /// Releases attention from any window. This represents the "unfocused" state
    /// where attention is free to be captured by the next salient stimulus.
    pub fn clear_focus(&mut self) {
        self.current_focus = None;
        self.focus_duration = Duration::zero();
        self.last_shift = Utc::now();
    }

    /// Update the focus duration
    ///
    /// Call this periodically to track how long attention has been held.
    /// Useful for detecting "stuck" attention or implementing attention decay.
    pub fn update_duration(&mut self, elapsed: Duration) {
        self.focus_duration += elapsed;
    }

    /// Check if currently focused
    #[must_use]
    pub const fn is_focused(&self) -> bool {
        self.current_focus.is_some()
    }

    /// Get the currently focused window
    #[must_use]
    pub const fn focused_window(&self) -> Option<WindowId> {
        self.current_focus
    }
}

impl Default for FocusState {
    fn default() -> Self {
        Self::new()
    }
}

/// Attention map - tracks salience for all windows
///
/// This is the "competition space" where windows compete for attention.
/// The window with highest salience wins focus (in competitive selection).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttentionMap {
    /// Salience scores for each window
    scores: HashMap<WindowId, f32>,
}

impl AttentionMap {
    /// Create a new empty attention map
    #[must_use]
    pub fn new() -> Self {
        Self {
            scores: HashMap::new(),
        }
    }

    /// Update the salience score for a window
    ///
    /// If the window is new, it's added to the map. If it already exists,
    /// its score is updated.
    pub fn update(&mut self, window_id: WindowId, salience: f32) {
        self.scores.insert(window_id, salience);
    }

    /// Remove a window from the attention map
    pub fn remove(&mut self, window_id: &WindowId) {
        self.scores.remove(window_id);
    }

    /// Get the salience score for a window
    #[must_use]
    pub fn get(&self, window_id: &WindowId) -> Option<f32> {
        self.scores.get(window_id).copied()
    }

    /// Find the window with highest salience
    ///
    /// This is the core of competitive selection: argmax(salience).
    /// Returns None if no windows are being tracked.
    #[must_use]
    pub fn highest_salience(&self) -> Option<(WindowId, f32)> {
        self.scores
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(id, &score)| (*id, score))
    }

    /// Get all windows above a salience threshold
    ///
    /// Returns windows that are "salient enough" to compete for attention.
    /// Useful for implementing attention filtering or multi-focus scenarios.
    #[must_use]
    pub fn above_threshold(&self, threshold: f32) -> Vec<(WindowId, f32)> {
        self.scores
            .iter()
            .filter(|(_, &score)| score >= threshold)
            .map(|(id, &score)| (*id, score))
            .collect()
    }

    /// Get all window scores
    #[must_use]
    pub fn all_scores(&self) -> &HashMap<WindowId, f32> {
        &self.scores
    }

    /// Get the number of windows being tracked
    #[must_use]
    pub fn len(&self) -> usize {
        self.scores.len()
    }

    /// Check if the attention map is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.scores.is_empty()
    }

    /// Clear all window scores
    pub fn clear(&mut self) {
        self.scores.clear();
    }
}

impl Default for AttentionMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_state_starts_unfocused() {
        let state = FocusState::new();
        assert!(!state.is_focused());
        assert_eq!(state.focused_window(), None);
    }

    #[test]
    fn focus_state_can_focus() {
        let mut state = FocusState::new();
        let window_id = WindowId::new();

        state.focus_on(window_id);

        assert!(state.is_focused());
        assert_eq!(state.focused_window(), Some(window_id));
    }

    #[test]
    fn focus_state_can_clear() {
        let mut state = FocusState::new();
        let window_id = WindowId::new();

        state.focus_on(window_id);
        state.clear_focus();

        assert!(!state.is_focused());
        assert_eq!(state.focused_window(), None);
    }

    #[test]
    fn focus_state_tracks_duration() {
        let mut state = FocusState::new();
        let window_id = WindowId::new();

        state.focus_on(window_id);
        assert_eq!(state.focus_duration, Duration::zero());

        state.update_duration(Duration::milliseconds(100));
        assert_eq!(state.focus_duration, Duration::milliseconds(100));

        state.update_duration(Duration::milliseconds(50));
        assert_eq!(state.focus_duration, Duration::milliseconds(150));
    }

    #[test]
    fn attention_map_starts_empty() {
        let map = AttentionMap::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
        assert_eq!(map.highest_salience(), None);
    }

    #[test]
    fn attention_map_can_update() {
        let mut map = AttentionMap::new();
        let window_id = WindowId::new();

        map.update(window_id, 0.8);

        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&window_id), Some(0.8));
    }

    #[test]
    fn attention_map_finds_highest_salience() {
        let mut map = AttentionMap::new();
        let window1 = WindowId::new();
        let window2 = WindowId::new();
        let window3 = WindowId::new();

        map.update(window1, 0.5);
        map.update(window2, 0.9);
        map.update(window3, 0.3);

        let (highest_id, highest_score) = map.highest_salience().unwrap();
        assert_eq!(highest_id, window2);
        assert_eq!(highest_score, 0.9);
    }

    #[test]
    fn attention_map_filters_by_threshold() {
        let mut map = AttentionMap::new();
        let window1 = WindowId::new();
        let window2 = WindowId::new();
        let window3 = WindowId::new();

        map.update(window1, 0.5);
        map.update(window2, 0.9);
        map.update(window3, 0.3);

        let above = map.above_threshold(0.4);
        assert_eq!(above.len(), 2);

        // Check that both windows above threshold are present
        let ids: Vec<WindowId> = above.iter().map(|(id, _)| *id).collect();
        assert!(ids.contains(&window1));
        assert!(ids.contains(&window2));
        assert!(!ids.contains(&window3));
    }

    #[test]
    fn attention_map_can_remove() {
        let mut map = AttentionMap::new();
        let window_id = WindowId::new();

        map.update(window_id, 0.8);
        assert_eq!(map.len(), 1);

        map.remove(&window_id);
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn attention_map_can_clear() {
        let mut map = AttentionMap::new();
        map.update(WindowId::new(), 0.5);
        map.update(WindowId::new(), 0.7);
        map.update(WindowId::new(), 0.3);

        assert_eq!(map.len(), 3);

        map.clear();

        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn attention_response_constructors() {
        let window_id = WindowId::new();

        let response = AttentionResponse::cycle_complete(Some(window_id), 0.8);
        assert!(matches!(response, AttentionResponse::CycleComplete { .. }));

        let response = AttentionResponse::focus_set(window_id);
        assert!(matches!(response, AttentionResponse::FocusSet { .. }));

        let response = AttentionResponse::focus_shifted(None, window_id);
        assert!(matches!(response, AttentionResponse::FocusShifted { .. }));

        let response = AttentionResponse::current_focus(Some(window_id));
        assert!(matches!(response, AttentionResponse::CurrentFocus { .. }));

        let mut scores = HashMap::new();
        scores.insert(window_id, 0.8);
        let response = AttentionResponse::attention_map(scores);
        assert!(matches!(response, AttentionResponse::AttentionMap { .. }));
    }

    #[test]
    fn attention_error_types() {
        let window_id = WindowId::new();

        let error = AttentionError::WindowNotFound { window_id };
        assert!(format!("{}", error).contains("Window not found"));

        let error = AttentionError::NoWindowsAvailable;
        assert!(format!("{}", error).contains("No windows available"));

        let error = AttentionError::CycleFailed {
            reason: "test".to_string(),
        };
        assert!(format!("{}", error).contains("test"));
    }

    #[test]
    fn focus_state_shift_updates_timing() {
        let mut state = FocusState::new();
        let window1 = WindowId::new();
        let window2 = WindowId::new();

        // Focus on first window
        state.focus_on(window1);
        let first_shift = state.last_shift;

        // Wait a bit (simulate time passing)
        std::thread::sleep(std::time::Duration::from_millis(10));

        // Focus on second window
        state.focus_on(window2);
        let second_shift = state.last_shift;

        // last_shift should have updated
        assert!(second_shift > first_shift);
        assert_eq!(state.focused_window(), Some(window2));
    }
}
