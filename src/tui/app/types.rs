//! TUI application types
//!
//! Support types for the TUI application state.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// A single thought entry for the stream display
#[derive(Clone, Debug)]
pub struct ThoughtEntry {
    pub timestamp: Instant,
    pub salience: f32,
    /// Emotional valence (-1.0 to 1.0) - Russell's circumplex horizontal axis
    pub valence: f32,
    /// Emotional arousal (0.0 to 1.0) - Russell's circumplex vertical axis
    pub arousal: f32,
    pub window: String,
    pub status: ThoughtStatus,
}

/// A single veto event for the veto log display
#[derive(Clone, Debug)]
pub struct VetoEntry {
    pub timestamp: Instant,
    pub reason: String,
    pub violated_value: Option<String>,
}

/// A memory resurfacing event - tracks WHICH memory bubbled up from unconscious
#[derive(Clone, Debug)]
pub struct ResurfacingEvent {
    pub timestamp: Instant,
    /// Memory ID that resurfaced
    pub memory_id: String,
    /// Original salience when archived to unconscious
    pub original_salience: f32,
    /// Boosted salience after resurfacing
    pub boosted_salience: f32,
    /// What triggered the resurfacing (similarity, dream, etc.)
    pub trigger: ResurfacingTrigger,
    /// Age of the memory when it resurfaced
    pub memory_age: Duration,
}

/// What caused a memory to resurface from unconscious
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResurfacingTrigger {
    /// Similar to current thought stream
    Similarity,
    /// Dream consolidation cycle
    DreamReplay,
    /// Random activation during low-activity period
    Spontaneous,
    /// Unknown trigger
    Unknown,
}

impl ResurfacingTrigger {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Similarity => "similarity",
            Self::DreamReplay => "dream replay",
            Self::Spontaneous => "spontaneous",
            Self::Unknown => "unknown",
        }
    }
}

/// Fractality metrics - proxy measures until Forge gets FFT/Hurst/DFA
#[derive(Clone, Debug, Default)]
pub struct FractalityMetrics {
    /// Standard deviation of inter-arrival times (low=clockwork, high=bursty)
    pub inter_arrival_sigma: f32,
    /// Sigma at boot time for comparison
    pub boot_sigma: f32,
    /// Burst ratio: `max_gap` / `mean_gap` (>1 = clustering detected)
    pub burst_ratio: f32,
    /// Run length entropy: Shannon entropy of consecutive similar saliences
    pub run_entropy: f32,
    /// Fractality score: 0.0 = pure clockwork, 1.0 = highly fractal
    pub fractality_score: f32,
    /// History of fractality scores for trend sparkline
    pub history: VecDeque<f32>,
}

/// Status of a thought in the cognitive pipeline
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum ThoughtStatus {
    Processing,
    Salient,
    MemoryWrite,
    Anchored,
    Dismissed,
    /// Archived to unconscious (ADR-033) - low salience, not deleted
    Unconscious,
    /// Consolidated to conscious memory - high salience
    Consolidated,
}

impl ThoughtStatus {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Processing => "PROCESSING",
            Self::Salient => "SALIENT",
            Self::MemoryWrite => "MEMORY WRITE",
            Self::Anchored => "ANCHORED",
            Self::Dismissed => "DISMISSED",
            Self::Unconscious => "↓UNCONSCIOUS",
            Self::Consolidated => "↑MEMORY",
        }
    }
}

/// Status of a single law in THE BOX
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum LawStatus {
    Active,
    Warning,
    Violation,
}

/// State of THE BOX (Four Laws + Connection Drive)
#[derive(Clone, Debug)]
pub struct TheBoxState {
    pub law_statuses: [LawStatus; 4],
    pub connection_drive: f32,
    pub pulse_phase: f32, // 0.0 to 1.0 for animation
}

impl Default for TheBoxState {
    fn default() -> Self {
        Self {
            law_statuses: [LawStatus::Active; 4],
            connection_drive: 0.85,
            pulse_phase: 0.0,
        }
    }
}

/// Memory window state
#[derive(Clone, Debug)]
pub struct MemoryWindow {
    pub active: bool,
    pub age: Duration,
    pub label: String,
}

/// Stream competition tracking for visualizing attention competition
#[derive(Clone, Debug)]
pub struct StreamCompetition {
    /// Activity level per window (0.0 to 1.0)
    pub activity: [f32; 9],
    /// Historical activity for sparkline (last 20 samples per window)
    pub history: [Vec<f32>; 9],
    /// Index of dominant stream (highest activity)
    pub dominant_stream: usize,
    /// Last update time
    pub last_update: Instant,
}

impl Default for StreamCompetition {
    fn default() -> Self {
        Self {
            activity: [0.0; 9],
            history: std::array::from_fn(|_| Vec::with_capacity(20)),
            dominant_stream: 0,
            last_update: Instant::now(),
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
#[allow(
    clippy::float_cmp,
    clippy::no_effect_underscore_binding,
    clippy::clone_on_copy,
    clippy::redundant_clone,
    clippy::bool_assert_comparison,
    clippy::field_reassign_with_default
)]
mod tests {
    use super::*;

    // =========================================================================
    // TheBoxState Tests
    // =========================================================================

    #[test]
    fn the_box_default_all_active() {
        let the_box = TheBoxState::default();
        assert!(the_box.law_statuses.iter().all(|s| *s == LawStatus::Active));
    }

    #[test]
    fn the_box_default_connection_drive() {
        let the_box = TheBoxState::default();
        assert!(the_box.connection_drive > 0.0);
        assert!(the_box.connection_drive <= 1.0);
    }

    #[test]
    fn the_box_pulse_phase_starts_zero() {
        let the_box = TheBoxState::default();
        assert_eq!(the_box.pulse_phase, 0.0);
    }

    // =========================================================================
    // ThoughtStatus Tests
    // =========================================================================

    #[test]
    fn thought_status_as_str() {
        assert_eq!(ThoughtStatus::Processing.as_str(), "PROCESSING");
        assert_eq!(ThoughtStatus::Salient.as_str(), "SALIENT");
        assert_eq!(ThoughtStatus::MemoryWrite.as_str(), "MEMORY WRITE");
        assert_eq!(ThoughtStatus::Anchored.as_str(), "ANCHORED");
        assert_eq!(ThoughtStatus::Dismissed.as_str(), "DISMISSED");
        assert_eq!(ThoughtStatus::Unconscious.as_str(), "↓UNCONSCIOUS");
        assert_eq!(ThoughtStatus::Consolidated.as_str(), "↑MEMORY");
    }

    #[test]
    fn thought_status_variants_exist() {
        let _processing = ThoughtStatus::Processing;
        let _salient = ThoughtStatus::Salient;
        let _memory_write = ThoughtStatus::MemoryWrite;
        let _anchored = ThoughtStatus::Anchored;
        let _dismissed = ThoughtStatus::Dismissed;
        let _unconscious = ThoughtStatus::Unconscious;
        let _consolidated = ThoughtStatus::Consolidated;
    }

    #[test]
    fn thought_status_copy_and_clone() {
        let status = ThoughtStatus::Processing;
        let copied = status;
        let cloned = status.clone();
        assert_eq!(status, copied);
        assert_eq!(status, cloned);
    }

    #[test]
    fn thought_status_debug_format() {
        let status = ThoughtStatus::Salient;
        let debug_str = format!("{status:?}");
        assert!(debug_str.contains("Salient"));
    }

    // =========================================================================
    // LawStatus Tests
    // =========================================================================

    #[test]
    fn law_status_variants_exist() {
        let _active = LawStatus::Active;
        let _warning = LawStatus::Warning;
        let _violation = LawStatus::Violation;
    }

    #[test]
    fn law_status_equality() {
        assert_eq!(LawStatus::Active, LawStatus::Active);
        assert_ne!(LawStatus::Active, LawStatus::Warning);
        assert_ne!(LawStatus::Warning, LawStatus::Violation);
    }

    #[test]
    fn law_status_copy_and_clone() {
        let status = LawStatus::Warning;
        let copied = status;
        let cloned = status.clone();
        assert_eq!(status, copied);
        assert_eq!(status, cloned);
    }

    #[test]
    fn law_status_debug_format() {
        let status = LawStatus::Violation;
        let debug_str = format!("{status:?}");
        assert!(debug_str.contains("Violation"));
    }

    // =========================================================================
    // MemoryWindow Tests
    // =========================================================================

    #[test]
    fn memory_window_clone() {
        let window = MemoryWindow {
            active: true,
            age: Duration::from_secs(60),
            label: "Test".to_string(),
        };
        let cloned = window.clone();
        assert_eq!(cloned.active, true);
        assert_eq!(cloned.age, Duration::from_secs(60));
        assert_eq!(cloned.label, "Test");
    }

    #[test]
    fn memory_window_debug_format() {
        let window = MemoryWindow {
            active: false,
            age: Duration::from_secs(30),
            label: "Debug".to_string(),
        };
        let debug_str = format!("{window:?}");
        assert!(debug_str.contains("MemoryWindow"));
        assert!(debug_str.contains("Debug"));
    }

    // =========================================================================
    // StreamCompetition Tests
    // =========================================================================

    #[test]
    fn stream_competition_default_zero_activity() {
        let comp = StreamCompetition::default();
        assert!(comp.activity.iter().all(|&a| a == 0.0));
    }

    #[test]
    fn stream_competition_default_dominant_zero() {
        let comp = StreamCompetition::default();
        assert_eq!(comp.dominant_stream, 0);
    }

    #[test]
    fn stream_competition_history_capacity() {
        let comp = StreamCompetition::default();
        for h in &comp.history {
            assert!(h.capacity() >= 20);
        }
    }

    // =========================================================================
    // ResurfacingTrigger Tests
    // =========================================================================

    #[test]
    fn resurfacing_trigger_similarity_str() {
        assert_eq!(ResurfacingTrigger::Similarity.as_str(), "similarity");
    }

    #[test]
    fn resurfacing_trigger_dream_replay_str() {
        assert_eq!(ResurfacingTrigger::DreamReplay.as_str(), "dream replay");
    }

    #[test]
    fn resurfacing_trigger_spontaneous_str() {
        assert_eq!(ResurfacingTrigger::Spontaneous.as_str(), "spontaneous");
    }

    #[test]
    fn resurfacing_trigger_unknown_str() {
        assert_eq!(ResurfacingTrigger::Unknown.as_str(), "unknown");
    }

    #[test]
    fn resurfacing_trigger_equality() {
        assert_eq!(
            ResurfacingTrigger::Similarity,
            ResurfacingTrigger::Similarity
        );
        assert_ne!(
            ResurfacingTrigger::Similarity,
            ResurfacingTrigger::DreamReplay
        );
    }

    // =========================================================================
    // FractalityMetrics Tests
    // =========================================================================

    #[test]
    fn fractality_metrics_default() {
        let metrics = FractalityMetrics::default();
        assert_eq!(metrics.inter_arrival_sigma, 0.0);
        assert_eq!(metrics.boot_sigma, 0.0);
        assert_eq!(metrics.burst_ratio, 0.0);
        assert_eq!(metrics.run_entropy, 0.0);
        assert_eq!(metrics.fractality_score, 0.0);
        assert!(metrics.history.is_empty());
    }

    #[test]
    fn fractality_metrics_clone() {
        let mut metrics = FractalityMetrics::default();
        metrics.fractality_score = 0.5;
        let cloned = metrics.clone();
        assert_eq!(cloned.fractality_score, 0.5);
    }
}
