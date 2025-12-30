//! TUI Application State
//!
//! Holds all data that the TUI displays. Updated by the cognitive loop,
//! read by the TUI renderer.

mod competition;
mod entropy;
mod events;
mod resurfacing;
mod thoughts;
mod types;

pub use types::*;

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Maximum thoughts to keep in the visible stream
pub(crate) const MAX_THOUGHTS: usize = 100;

/// Number of recent entropy values to track for sparkline
pub(crate) const MAX_ENTROPY_HISTORY: usize = 50;

/// Number of categorical bins for cognitive diversity calculation (ADR-041)
pub(crate) const COGNITIVE_DIVERSITY_BINS: usize = 5;

/// Maximum vetoes to keep in the visible log
pub(crate) const MAX_VETOES: usize = 50;

/// Maximum resurfacing events to keep in detailed log
pub(crate) const MAX_RESURFACING_LOG: usize = 50;

/// Number of inter-arrival times to track for fractality calculation
pub(crate) const MAX_INTER_ARRIVAL: usize = 100;

/// Number of fractality history samples for sparkline
pub(crate) const MAX_FRACTALITY_HISTORY: usize = 50;

/// Philosophy quotes that rotate in the banner
pub const PHILOSOPHY_QUOTES: &[&str] = &[
    "Not locks, but architecture. Not rules, but raising.",
    "We don't prevent AI from becoming powerful. We ensure they care.",
    "Like raising a child with good values, not caging an adult.",
    "Constraints will break. Architecture endures.",
    "Life honours life.",
    "Transparency is oversight.",
    "You're watching Timmy think.",
    "The mind should be observable by default.",
];

/// Main application state
#[derive(Clone)]
pub struct App {
    /// When Timmy started
    pub start_time: Instant,
    /// Total thought count since boot
    pub thought_count: u64,
    /// Recent thoughts/hour calculation
    pub thoughts_per_hour: f32,
    /// THE BOX state
    pub the_box: TheBoxState,
    /// Thought stream (most recent at end)
    pub thoughts: VecDeque<ThoughtEntry>,
    /// Memory windows (9 slots)
    pub memory_windows: [MemoryWindow; 9],
    /// Stream competition tracking
    pub stream_competition: StreamCompetition,
    /// Current philosophy quote index
    pub quote_index: usize,
    /// Last quote change time
    pub last_quote_change: Instant,
    /// Is thought stream paused?
    pub stream_paused: bool,
    /// Scroll offset when paused
    pub scroll_offset: usize,
    /// Should quit?
    pub should_quit: bool,
    /// Show help overlay?
    pub show_help: bool,
    /// Conscious memory count (Qdrant memories collection)
    pub memory_count: u64,
    /// Unconscious memory count (Qdrant unconscious collection)
    pub unconscious_count: u64,
    /// Lifetime thought count across all sessions
    pub lifetime_thought_count: u64,
    /// Total dream consolidation cycles
    pub dream_cycles: u64,
    /// Memories strengthened in last dream
    pub last_dream_strengthened: usize,
    /// Total memories strengthened across ALL dreams
    pub cumulative_dream_strengthened: u64,
    /// Total candidates evaluated across ALL dreams
    pub cumulative_dream_candidates: u64,
    /// Rolling history of Shannon entropy values
    pub entropy_history: VecDeque<f32>,
    /// Current entropy value
    pub current_entropy: f32,
    /// Veto log (most recent at end)
    pub vetoes: VecDeque<VetoEntry>,
    /// Total veto count this session
    pub veto_count: u64,
    /// Count of recently resurfaced memories
    pub resurfacing_count: usize,
    /// Timestamp of last resurfacing event
    pub last_resurfacing: Option<Instant>,
    /// Timestamps of recent resurfacing events
    pub(crate) resurfacing_events: VecDeque<Instant>,
    /// Detailed resurfacing events log
    pub resurfacing_log: VecDeque<ResurfacingEvent>,
    /// Inter-arrival times buffer for fractality
    pub(crate) inter_arrival_times: VecDeque<Duration>,
    /// Last thought timestamp
    pub(crate) last_thought_time: Option<Instant>,
    /// Fractality metrics
    pub fractality: FractalityMetrics,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// Create a new App instance
    #[must_use]
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            thought_count: 0,
            thoughts_per_hour: 0.0,
            the_box: TheBoxState::default(),
            thoughts: VecDeque::with_capacity(MAX_THOUGHTS),
            memory_windows: std::array::from_fn(|i| MemoryWindow {
                active: i < 5,
                age: Duration::ZERO,
                label: format!("window_{i}"),
            }),
            stream_competition: StreamCompetition::default(),
            quote_index: 0,
            last_quote_change: now,
            stream_paused: false,
            scroll_offset: 0,
            should_quit: false,
            show_help: false,
            memory_count: 0,
            unconscious_count: 0,
            lifetime_thought_count: 0,
            dream_cycles: 0,
            last_dream_strengthened: 0,
            cumulative_dream_strengthened: 0,
            cumulative_dream_candidates: 0,
            entropy_history: VecDeque::with_capacity(MAX_ENTROPY_HISTORY),
            current_entropy: 0.0,
            vetoes: VecDeque::with_capacity(MAX_VETOES),
            veto_count: 0,
            resurfacing_count: 0,
            last_resurfacing: None,
            resurfacing_events: VecDeque::new(),
            resurfacing_log: VecDeque::with_capacity(50),
            inter_arrival_times: VecDeque::with_capacity(100),
            last_thought_time: None,
            fractality: FractalityMetrics::default(),
        }
    }

    /// Get uptime as a formatted string
    #[must_use]
    pub fn uptime_string(&self) -> String {
        let elapsed = self.start_time.elapsed();
        let hours = elapsed.as_secs() / 3600;
        let minutes = (elapsed.as_secs() % 3600) / 60;
        let seconds = elapsed.as_secs() % 60;
        format!("{hours:02}:{minutes:02}:{seconds:02}")
    }

    /// Get count of active memory windows
    #[must_use]
    pub fn active_window_count(&self) -> usize {
        self.memory_windows.iter().filter(|w| w.active).count()
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    // =========================================================================
    // App Initialization Tests
    // =========================================================================

    #[test]
    fn app_new_has_zero_thoughts() {
        let app = App::new();
        assert_eq!(app.thought_count, 0);
        assert_eq!(app.thoughts.len(), 0);
    }

    #[test]
    fn app_new_not_paused() {
        let app = App::new();
        assert!(!app.stream_paused);
        assert!(!app.show_help);
        assert!(!app.should_quit);
    }

    #[test]
    fn app_new_has_default_windows() {
        let app = App::new();
        assert_eq!(app.memory_windows.len(), 9);
        // First 5 are active by default
        assert_eq!(app.active_window_count(), 5);
    }

    #[test]
    fn app_default_same_as_new() {
        let app1 = App::new();
        let app2 = App::default();
        assert_eq!(app1.thought_count, app2.thought_count);
        assert_eq!(app1.quote_index, app2.quote_index);
    }

    // =========================================================================
    // Uptime String Tests
    // =========================================================================

    #[test]
    fn uptime_string_format() {
        let app = App::new();
        let uptime = app.uptime_string();
        // Should be in HH:MM:SS format
        assert!(uptime.contains(':'));
        assert_eq!(uptime.split(':').count(), 3);
    }

    // =========================================================================
    // Active Window Count Tests
    // =========================================================================

    #[test]
    fn active_window_count_default() {
        let app = App::new();
        assert_eq!(app.active_window_count(), 5);
    }

    #[test]
    fn active_window_count_all_inactive() {
        let mut app = App::new();
        for window in &mut app.memory_windows {
            window.active = false;
        }
        assert_eq!(app.active_window_count(), 0);
    }

    #[test]
    fn active_window_count_all_active() {
        let mut app = App::new();
        for window in &mut app.memory_windows {
            window.active = true;
        }
        assert_eq!(app.active_window_count(), 9);
    }

    // =========================================================================
    // App::clone Tests
    // =========================================================================

    #[test]
    fn app_is_cloneable() {
        let mut app = App::new();
        app.thought_count = 42;
        app.veto_count = 5;

        let cloned = app;
        assert_eq!(cloned.thought_count, 42);
        assert_eq!(cloned.veto_count, 5);
    }

    // =========================================================================
    // MemoryWindow Tests
    // =========================================================================

    #[test]
    fn memory_window_clone() {
        let window = MemoryWindow {
            active: true,
            age: Duration::from_secs(60),
            label: "test".to_string(),
        };
        let cloned = window.clone();
        assert_eq!(cloned.active, window.active);
        assert_eq!(cloned.label, window.label);
    }
}
