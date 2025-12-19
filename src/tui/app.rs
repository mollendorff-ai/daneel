//! TUI Application State
//!
//! Holds all data that the TUI displays. Updated by the cognitive loop,
//! read by the TUI renderer.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Maximum thoughts to keep in the visible stream
const MAX_THOUGHTS: usize = 100;

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

/// A single thought entry for the stream display
#[derive(Clone, Debug)]
pub struct ThoughtEntry {
    pub timestamp: Instant,
    pub salience: f32,
    pub window: String,
    pub status: ThoughtStatus,
}

/// Status of a thought in the cognitive pipeline
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum ThoughtStatus {
    Processing,
    Salient,
    MemoryWrite,
    Anchored,
    Dismissed,
}

impl ThoughtStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Processing => "PROCESSING",
            Self::Salient => "SALIENT",
            Self::MemoryWrite => "MEMORY WRITE",
            Self::Anchored => "ANCHORED",
            Self::Dismissed => "DISMISSED",
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
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            thought_count: 0,
            thoughts_per_hour: 0.0,
            the_box: TheBoxState::default(),
            thoughts: VecDeque::with_capacity(MAX_THOUGHTS),
            memory_windows: std::array::from_fn(|i| MemoryWindow {
                active: i < 5, // Start with 5 active windows
                age: Duration::ZERO,
                label: format!("window_{i}"),
            }),
            quote_index: 0,
            last_quote_change: now,
            stream_paused: false,
            scroll_offset: 0,
            should_quit: false,
            show_help: false,
        }
    }

    /// Get uptime as a formatted string
    pub fn uptime_string(&self) -> String {
        let elapsed = self.start_time.elapsed();
        let hours = elapsed.as_secs() / 3600;
        let minutes = (elapsed.as_secs() % 3600) / 60;
        let seconds = elapsed.as_secs() % 60;
        format!("{hours:02}:{minutes:02}:{seconds:02}")
    }

    /// Get active memory window count
    pub fn active_window_count(&self) -> usize {
        self.memory_windows.iter().filter(|w| w.active).count()
    }

    /// Add a new thought to the stream
    pub fn add_thought(&mut self, salience: f32, window: String, status: ThoughtStatus) {
        if self.thoughts.len() >= MAX_THOUGHTS {
            self.thoughts.pop_front();
        }
        self.thoughts.push_back(ThoughtEntry {
            timestamp: Instant::now(),
            salience,
            window,
            status,
        });
        self.thought_count += 1;

        // Update thoughts/hour (simple moving average)
        let elapsed_hours = self.start_time.elapsed().as_secs_f32() / 3600.0;
        if elapsed_hours > 0.0 {
            self.thoughts_per_hour = self.thought_count as f32 / elapsed_hours;
        }
    }

    /// Update pulse animation (call every frame)
    pub fn update_pulse(&mut self, delta: Duration) {
        // Pulse at ~1Hz
        self.the_box.pulse_phase += delta.as_secs_f32();
        if self.the_box.pulse_phase > 1.0 {
            self.the_box.pulse_phase -= 1.0;
        }
    }

    /// Update philosophy quote if enough time has passed
    pub fn update_quote(&mut self) {
        if self.last_quote_change.elapsed() > Duration::from_secs(30) {
            self.quote_index = (self.quote_index + 1) % PHILOSOPHY_QUOTES.len();
            self.last_quote_change = Instant::now();
        }
    }

    /// Get current philosophy quote
    pub fn current_quote(&self) -> &'static str {
        PHILOSOPHY_QUOTES[self.quote_index]
    }

    /// Handle keyboard input
    pub fn handle_key(&mut self, key: crossterm::event::KeyCode) {
        use crossterm::event::KeyCode;
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('p') => self.stream_paused = !self.stream_paused,
            KeyCode::Char('?') => self.show_help = !self.show_help,
            KeyCode::Up if self.stream_paused => {
                self.scroll_offset = self.scroll_offset.saturating_add(1);
            }
            KeyCode::Down if self.stream_paused => {
                self.scroll_offset = self.scroll_offset.saturating_sub(1);
            }
            KeyCode::Esc => {
                self.show_help = false;
                self.stream_paused = false;
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyCode;

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
        let parts: Vec<&str> = uptime.split(':').collect();
        assert_eq!(parts.len(), 3);
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
    // Add Thought Tests
    // =========================================================================

    #[test]
    fn add_thought_increments_count() {
        let mut app = App::new();
        assert_eq!(app.thought_count, 0);

        app.add_thought(0.5, "window_0".to_string(), ThoughtStatus::Processing);
        assert_eq!(app.thought_count, 1);

        app.add_thought(0.8, "window_1".to_string(), ThoughtStatus::Salient);
        assert_eq!(app.thought_count, 2);
    }

    #[test]
    fn add_thought_adds_to_queue() {
        let mut app = App::new();
        app.add_thought(0.5, "window_0".to_string(), ThoughtStatus::Processing);

        assert_eq!(app.thoughts.len(), 1);
        assert_eq!(app.thoughts[0].window, "window_0");
    }

    #[test]
    fn add_thought_respects_max_limit() {
        let mut app = App::new();

        // Add MAX_THOUGHTS + 10 thoughts
        for i in 0..110 {
            app.add_thought(0.5, format!("window_{}", i), ThoughtStatus::Processing);
        }

        // Queue should be capped at MAX_THOUGHTS (100)
        assert_eq!(app.thoughts.len(), MAX_THOUGHTS);
        // First thought should be window_10 (first 10 were evicted)
        assert_eq!(app.thoughts[0].window, "window_10");
    }

    #[test]
    fn add_thought_updates_thoughts_per_hour() {
        let mut app = App::new();
        app.add_thought(0.5, "test".to_string(), ThoughtStatus::Processing);

        // After first thought, thoughts_per_hour should be calculated
        // (will be high because elapsed time is tiny)
        assert!(app.thoughts_per_hour > 0.0);
    }

    // =========================================================================
    // Pulse Update Tests
    // =========================================================================

    #[test]
    fn update_pulse_increments_phase() {
        let mut app = App::new();
        let initial_phase = app.the_box.pulse_phase;

        app.update_pulse(Duration::from_millis(100));
        assert!(app.the_box.pulse_phase > initial_phase);
    }

    #[test]
    fn update_pulse_wraps_at_one() {
        let mut app = App::new();
        app.the_box.pulse_phase = 0.95;

        app.update_pulse(Duration::from_millis(100)); // Should push over 1.0
        assert!(app.the_box.pulse_phase < 1.0); // Should have wrapped
    }

    // =========================================================================
    // Quote Tests
    // =========================================================================

    #[test]
    fn current_quote_returns_valid_string() {
        let app = App::new();
        let quote = app.current_quote();
        assert!(!quote.is_empty());
    }

    #[test]
    fn philosophy_quotes_not_empty() {
        assert!(!PHILOSOPHY_QUOTES.is_empty());
    }

    #[test]
    fn quote_index_in_bounds() {
        let mut app = App::new();
        for i in 0..PHILOSOPHY_QUOTES.len() {
            app.quote_index = i;
            let _ = app.current_quote(); // Should not panic
        }
    }

    // =========================================================================
    // Keyboard Handling Tests
    // =========================================================================

    #[test]
    fn handle_key_q_quits() {
        let mut app = App::new();
        assert!(!app.should_quit);

        app.handle_key(KeyCode::Char('q'));
        assert!(app.should_quit);
    }

    #[test]
    fn handle_key_p_toggles_pause() {
        let mut app = App::new();
        assert!(!app.stream_paused);

        app.handle_key(KeyCode::Char('p'));
        assert!(app.stream_paused);

        app.handle_key(KeyCode::Char('p'));
        assert!(!app.stream_paused);
    }

    #[test]
    fn handle_key_question_toggles_help() {
        let mut app = App::new();
        assert!(!app.show_help);

        app.handle_key(KeyCode::Char('?'));
        assert!(app.show_help);

        app.handle_key(KeyCode::Char('?'));
        assert!(!app.show_help);
    }

    #[test]
    fn handle_key_up_when_paused_scrolls() {
        let mut app = App::new();
        app.stream_paused = true;
        app.scroll_offset = 0;

        app.handle_key(KeyCode::Up);
        assert_eq!(app.scroll_offset, 1);

        app.handle_key(KeyCode::Up);
        assert_eq!(app.scroll_offset, 2);
    }

    #[test]
    fn handle_key_down_when_paused_scrolls() {
        let mut app = App::new();
        app.stream_paused = true;
        app.scroll_offset = 5;

        app.handle_key(KeyCode::Down);
        assert_eq!(app.scroll_offset, 4);
    }

    #[test]
    fn handle_key_down_saturates_at_zero() {
        let mut app = App::new();
        app.stream_paused = true;
        app.scroll_offset = 0;

        app.handle_key(KeyCode::Down);
        assert_eq!(app.scroll_offset, 0); // Should not go negative
    }

    #[test]
    fn handle_key_arrows_ignored_when_not_paused() {
        let mut app = App::new();
        app.scroll_offset = 5;

        app.handle_key(KeyCode::Up);
        assert_eq!(app.scroll_offset, 5); // Unchanged

        app.handle_key(KeyCode::Down);
        assert_eq!(app.scroll_offset, 5); // Unchanged
    }

    #[test]
    fn handle_key_esc_clears_states() {
        let mut app = App::new();
        app.show_help = true;
        app.stream_paused = true;

        app.handle_key(KeyCode::Esc);
        assert!(!app.show_help);
        assert!(!app.stream_paused);
    }

    #[test]
    fn handle_key_unknown_does_nothing() {
        let mut app = App::new();
        let thought_count = app.thought_count;
        let paused = app.stream_paused;

        app.handle_key(KeyCode::Char('x')); // Random key

        assert_eq!(app.thought_count, thought_count);
        assert_eq!(app.stream_paused, paused);
    }

    // =========================================================================
    // LawStatus Tests
    // =========================================================================

    #[test]
    fn law_status_equality() {
        assert_eq!(LawStatus::Active, LawStatus::Active);
        assert_ne!(LawStatus::Active, LawStatus::Warning);
        assert_ne!(LawStatus::Warning, LawStatus::Violation);
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
