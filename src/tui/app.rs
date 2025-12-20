//! TUI Application State
//!
//! Holds all data that the TUI displays. Updated by the cognitive loop,
//! read by the TUI renderer.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Maximum thoughts to keep in the visible stream
const MAX_THOUGHTS: usize = 100;

/// Number of recent entropy values to track for sparkline
const MAX_ENTROPY_HISTORY: usize = 50;

/// Number of bins for salience distribution when calculating entropy
const ENTROPY_BINS: usize = 10;

/// Maximum vetoes to keep in the visible log
const MAX_VETOES: usize = 50;

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
    pub fn as_str(&self) -> &'static str {
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

    /// Unconscious memory count (Qdrant unconscious collection) - ADR-033
    pub unconscious_count: u64,

    /// Lifetime thought count across all sessions (ADR-034)
    pub lifetime_thought_count: u64,

    /// Total dream consolidation cycles (ADR-023)
    pub dream_cycles: u64,

    /// Memories strengthened in last dream
    pub last_dream_strengthened: usize,

    /// Total memories strengthened across ALL dreams (cumulative)
    /// TUI-VIS-4: Cumulative Dream Strengthening
    pub cumulative_dream_strengthened: u64,

    /// Total candidates evaluated across ALL dreams
    /// TUI-VIS-4: For efficiency tracking (strengthened / candidates)
    pub cumulative_dream_candidates: u64,

    /// Rolling history of Shannon entropy values (for sparkline)
    pub entropy_history: VecDeque<f32>,

    /// Current entropy value (cached from last calculation)
    pub current_entropy: f32,

    /// Veto log (most recent at end) - Libet's "free-won't" in action
    pub vetoes: VecDeque<VetoEntry>,

    /// Total veto count this session
    pub veto_count: u64,

    /// Count of recently resurfaced memories (last 60 seconds)
    pub resurfacing_count: usize,

    /// Timestamp of last resurfacing event (for glow animation)
    pub last_resurfacing: Option<Instant>,

    /// Timestamps of recent resurfacing events (for count tracking)
    resurfacing_events: VecDeque<Instant>,
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
    pub fn add_thought(
        &mut self,
        salience: f32,
        valence: f32,
        arousal: f32,
        window: String,
        status: ThoughtStatus,
    ) {
        if self.thoughts.len() >= MAX_THOUGHTS {
            self.thoughts.pop_front();
        }

        // Update stream competition metrics
        self.update_stream_competition(&window, salience);

        self.thoughts.push_back(ThoughtEntry {
            timestamp: Instant::now(),
            salience,
            valence,
            arousal,
            window,
            status,
        });
        self.thought_count += 1;

        // Track resurfacing events (Consolidated status = memories bubbling up from unconscious)
        if status == ThoughtStatus::Consolidated {
            let now = Instant::now();
            self.last_resurfacing = Some(now);
            self.resurfacing_events.push_back(now);
        }

        // Update thoughts/hour (simple moving average)
        let elapsed_hours = self.start_time.elapsed().as_secs_f32() / 3600.0;
        if elapsed_hours > 0.0 {
            self.thoughts_per_hour = self.thought_count as f32 / elapsed_hours;
        }

        // Update entropy every 5 thoughts
        if self.thought_count % 5 == 0 {
            self.update_entropy();
        }
    }

    /// Add a new veto event to the log
    pub fn add_veto(&mut self, reason: String, violated_value: Option<String>) {
        if self.vetoes.len() >= MAX_VETOES {
            self.vetoes.pop_front();
        }
        self.vetoes.push_back(VetoEntry {
            timestamp: Instant::now(),
            reason,
            violated_value,
        });
        self.veto_count += 1;
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

    /// Update resurfacing count by removing events older than 60 seconds
    pub fn update_resurfacing(&mut self) {
        let cutoff = Instant::now() - Duration::from_secs(60);

        // Remove old events
        while let Some(&timestamp) = self.resurfacing_events.front() {
            if timestamp < cutoff {
                self.resurfacing_events.pop_front();
            } else {
                break;
            }
        }

        // Update count
        self.resurfacing_count = self.resurfacing_events.len();
    }

    /// Check if resurfacing is currently active (happened in last 2 seconds) for glow effect
    pub fn is_resurfacing_active(&self) -> bool {
        if let Some(last) = self.last_resurfacing {
            last.elapsed() < Duration::from_secs(2)
        } else {
            false
        }
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

    /// Update stream competition metrics based on recent thought activity
    pub fn update_stream_competition(&mut self, window: &str, salience: f32) {
        // Extract window index from "window_N" format
        if let Some(idx_str) = window.strip_prefix("window_") {
            if let Ok(idx) = idx_str.parse::<usize>() {
                if idx < 9 {
                    // Boost this window's activity based on salience
                    // Use exponential moving average for smooth transitions
                    let alpha = 0.3; // Smoothing factor
                    self.stream_competition.activity[idx] =
                        alpha * salience + (1.0 - alpha) * self.stream_competition.activity[idx];
                }
            }
        }
    }

    /// Decay stream competition activity over time (call periodically)
    pub fn decay_stream_competition(&mut self, delta: Duration) {
        let decay_rate: f32 = 0.95; // Decay 5% per update
        let decay = decay_rate.powf(delta.as_secs_f32());

        for activity in &mut self.stream_competition.activity {
            *activity *= decay;
        }

        // Update history snapshots every second
        if self.stream_competition.last_update.elapsed() >= Duration::from_secs(1) {
            for (i, history) in self.stream_competition.history.iter_mut().enumerate() {
                history.push(self.stream_competition.activity[i]);
                if history.len() > 20 {
                    history.remove(0);
                }
            }

            // Update dominant stream
            self.stream_competition.dominant_stream = self
                .stream_competition
                .activity
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(idx, _)| idx)
                .unwrap_or(0);

            self.stream_competition.last_update = Instant::now();
        }
    }

    /// Calculate Shannon entropy from recent thought salience distribution
    ///
    /// Bins salience values into ENTROPY_BINS buckets and computes:
    /// H = -Σ(p_i * log2(p_i))
    ///
    /// Higher entropy = more varied/emergent thinking
    /// Lower entropy = more repetitive/mechanical patterns
    ///
    /// Returns entropy in bits (0.0 to log2(ENTROPY_BINS))
    pub fn calculate_entropy(&self) -> f32 {
        if self.thoughts.is_empty() {
            return 0.0;
        }

        // Count salience values in each bin
        let mut bins = [0u32; ENTROPY_BINS];
        for thought in &self.thoughts {
            let salience = thought.salience.clamp(0.0, 1.0);
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let bin_idx = ((salience * ENTROPY_BINS as f32).floor() as usize).min(ENTROPY_BINS - 1);
            bins[bin_idx] += 1;
        }

        // Calculate probabilities and Shannon entropy
        let total = self.thoughts.len() as f32;
        let mut entropy = 0.0f32;

        for &count in &bins {
            if count > 0 {
                let p = count as f32 / total;
                entropy -= p * p.log2();
            }
        }

        entropy
    }

    /// Update entropy history with current entropy value
    ///
    /// Should be called periodically (e.g., every few thoughts) to track
    /// how entropy evolves over time.
    pub fn update_entropy(&mut self) {
        let entropy = self.calculate_entropy();
        self.current_entropy = entropy;

        if self.entropy_history.len() >= MAX_ENTROPY_HISTORY {
            self.entropy_history.pop_front();
        }
        self.entropy_history.push_back(entropy);
    }

    /// Get entropy description: "EMERGENT", "BALANCED", or "CLOCKWORK"
    pub fn entropy_description(&self) -> &'static str {
        // Max possible entropy for our bin count
        let max_entropy = (ENTROPY_BINS as f32).log2();
        let normalized = self.current_entropy / max_entropy;

        if normalized > 0.7 {
            "EMERGENT"
        } else if normalized > 0.4 {
            "BALANCED"
        } else {
            "CLOCKWORK"
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
        assert_eq!(ThoughtStatus::Unconscious.as_str(), "↓UNCONSCIOUS");
        assert_eq!(ThoughtStatus::Consolidated.as_str(), "↑MEMORY");
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

        app.add_thought(
            0.5,
            0.0,
            0.5,
            "window_0".to_string(),
            ThoughtStatus::Processing,
        );
        assert_eq!(app.thought_count, 1);

        app.add_thought(
            0.8,
            0.3,
            0.7,
            "window_1".to_string(),
            ThoughtStatus::Salient,
        );
        assert_eq!(app.thought_count, 2);
    }

    #[test]
    fn add_thought_adds_to_queue() {
        let mut app = App::new();
        app.add_thought(
            0.5,
            0.0,
            0.5,
            "window_0".to_string(),
            ThoughtStatus::Processing,
        );

        assert_eq!(app.thoughts.len(), 1);
        assert_eq!(app.thoughts[0].window, "window_0");
    }

    #[test]
    fn add_thought_respects_max_limit() {
        let mut app = App::new();

        // Add MAX_THOUGHTS + 10 thoughts
        for i in 0..110 {
            app.add_thought(
                0.5,
                0.0,
                0.5,
                format!("window_{}", i),
                ThoughtStatus::Processing,
            );
        }

        // Queue should be capped at MAX_THOUGHTS (100)
        assert_eq!(app.thoughts.len(), MAX_THOUGHTS);
        // First thought should be window_10 (first 10 were evicted)
        assert_eq!(app.thoughts[0].window, "window_10");
    }

    #[test]
    fn add_thought_updates_thoughts_per_hour() {
        let mut app = App::new();
        app.add_thought(0.5, 0.0, 0.5, "test".to_string(), ThoughtStatus::Processing);

        // After first thought, thoughts_per_hour should be calculated
        // (will be high because elapsed time is tiny)
        assert!(app.thoughts_per_hour > 0.0);
    }

    #[test]
    fn add_thought_stores_emotion_values() {
        let mut app = App::new();
        app.add_thought(0.5, 0.7, 0.8, "test".to_string(), ThoughtStatus::Processing);

        assert_eq!(app.thoughts[0].valence, 0.7);
        assert_eq!(app.thoughts[0].arousal, 0.8);
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
    #[allow(clippy::const_is_empty)]
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

    // =========================================================================
    // Resurfacing Tests (TUI-VIS-3)
    // =========================================================================

    #[test]
    fn resurfacing_count_starts_zero() {
        let app = App::new();
        assert_eq!(app.resurfacing_count, 0);
        assert!(app.last_resurfacing.is_none());
    }

    #[test]
    fn consolidated_thought_tracks_resurfacing() {
        let mut app = App::new();

        app.add_thought(
            0.9,
            0.5,
            0.8,
            "test".to_string(),
            ThoughtStatus::Consolidated,
        );

        assert!(app.last_resurfacing.is_some());
        assert_eq!(app.resurfacing_events.len(), 1);
    }

    #[test]
    fn non_consolidated_thought_ignores_resurfacing() {
        let mut app = App::new();

        app.add_thought(0.5, 0.0, 0.5, "test".to_string(), ThoughtStatus::Processing);

        assert!(app.last_resurfacing.is_none());
        assert_eq!(app.resurfacing_events.len(), 0);
    }

    #[test]
    fn update_resurfacing_counts_recent_events() {
        let mut app = App::new();

        // Add 3 consolidated thoughts
        for _ in 0..3 {
            app.add_thought(
                0.9,
                0.5,
                0.8,
                "test".to_string(),
                ThoughtStatus::Consolidated,
            );
        }

        app.update_resurfacing();
        assert_eq!(app.resurfacing_count, 3);
    }

    #[test]
    fn is_resurfacing_active_true_when_recent() {
        let mut app = App::new();

        app.add_thought(
            0.9,
            0.5,
            0.8,
            "test".to_string(),
            ThoughtStatus::Consolidated,
        );

        assert!(app.is_resurfacing_active());
    }

    #[test]
    fn is_resurfacing_active_false_when_no_events() {
        let app = App::new();
        assert!(!app.is_resurfacing_active());
    }

    #[test]
    fn multiple_consolidated_thoughts_tracked() {
        let mut app = App::new();

        // Add multiple resurfacing events
        for i in 0..5 {
            app.add_thought(
                0.9,
                0.5,
                0.8,
                format!("test_{i}"),
                ThoughtStatus::Consolidated,
            );
        }

        app.update_resurfacing();
        assert_eq!(app.resurfacing_count, 5);
        assert_eq!(app.resurfacing_events.len(), 5);
    }

    // =========================================================================
    // Entropy Tests
    // =========================================================================

    #[test]
    fn calculate_entropy_empty_thoughts() {
        let app = App::new();
        let entropy = app.calculate_entropy();
        assert_eq!(entropy, 0.0);
    }

    #[test]
    fn calculate_entropy_uniform_distribution() {
        let mut app = App::new();
        // Add thoughts with uniform salience distribution
        for i in 0..100 {
            let salience = (i as f32 / 100.0);
            app.add_thought(
                salience,
                0.0,
                0.5,
                format!("window_{}", i % 9),
                ThoughtStatus::Processing,
            );
        }

        let entropy = app.calculate_entropy();
        // Uniform distribution should have high entropy
        assert!(entropy > 2.0); // Should be close to log2(10) ≈ 3.32
    }

    #[test]
    fn calculate_entropy_single_value() {
        let mut app = App::new();
        // All thoughts with same salience = zero entropy
        for _ in 0..50 {
            app.add_thought(
                0.5,
                0.0,
                0.5,
                "window_0".to_string(),
                ThoughtStatus::Processing,
            );
        }

        let entropy = app.calculate_entropy();
        // Single value should give zero entropy
        assert!(entropy < 0.01);
    }

    #[test]
    #[test]
    fn update_entropy_adds_to_history() {
        let mut app = App::new();

        // Add 3 thoughts (not enough to trigger automatic update at 5)
        for i in 0..3 {
            app.add_thought(
                (i as f32 / 10.0),
                0.0,
                0.5,
                "window_0".to_string(),
                ThoughtStatus::Processing,
            );
        }

        // Should not have auto-updated yet
        assert_eq!(app.entropy_history.len(), 0);

        // Manually update
        app.update_entropy();
        assert_eq!(app.entropy_history.len(), 1);

        app.update_entropy();
        assert_eq!(app.entropy_history.len(), 2);
    }

    #[test]
    fn update_entropy_respects_max_size() {
        let mut app = App::new();

        // Add thoughts
        for i in 0..10 {
            app.add_thought(
                (i as f32 / 10.0),
                0.0,
                0.5,
                "window_0".to_string(),
                ThoughtStatus::Processing,
            );
        }

        // Update entropy more times than MAX_ENTROPY_HISTORY
        for _ in 0..60 {
            app.update_entropy();
        }

        assert!(app.entropy_history.len() <= MAX_ENTROPY_HISTORY);
    }

    #[test]
    fn entropy_description_high() {
        let mut app = App::new();
        // Simulate high entropy
        app.current_entropy = 2.5; // High value relative to max ~3.32
        let desc = app.entropy_description();
        assert_eq!(desc, "EMERGENT");
    }

    #[test]
    fn entropy_description_medium() {
        let mut app = App::new();
        app.current_entropy = 1.5; // Medium value
        let desc = app.entropy_description();
        assert_eq!(desc, "BALANCED");
    }

    #[test]
    fn entropy_description_low() {
        let mut app = App::new();
        app.current_entropy = 0.5; // Low value
        let desc = app.entropy_description();
        assert_eq!(desc, "CLOCKWORK");
    }

    #[test]
    fn add_thought_updates_entropy_periodically() {
        let mut app = App::new();

        // Add 4 thoughts - shouldn't update entropy yet
        for _ in 0..4 {
            app.add_thought(
                0.5,
                0.0,
                0.5,
                "window_0".to_string(),
                ThoughtStatus::Processing,
            );
        }
        assert_eq!(app.entropy_history.len(), 0);

        // 5th thought should trigger entropy update
        app.add_thought(
            0.6,
            0.0,
            0.5,
            "window_0".to_string(),
            ThoughtStatus::Processing,
        );
        assert_eq!(app.entropy_history.len(), 1);

        // Next 4 thoughts shouldn't update
        for _ in 0..4 {
            app.add_thought(
                0.7,
                0.0,
                0.5,
                "window_0".to_string(),
                ThoughtStatus::Processing,
            );
        }
        assert_eq!(app.entropy_history.len(), 1);

        // 10th thought should trigger another update
        app.add_thought(
            0.8,
            0.0,
            0.5,
            "window_0".to_string(),
            ThoughtStatus::Processing,
        );
        assert_eq!(app.entropy_history.len(), 2);
    }

    // =========================================================================
    // TUI-VIS-4: Cumulative Dream Strengthening Tests
    // =========================================================================

    #[test]
    fn cumulative_dream_strengthened_initializes_to_zero() {
        let app = App::new();
        assert_eq!(app.cumulative_dream_strengthened, 0);
    }

    #[test]
    fn cumulative_dream_candidates_initializes_to_zero() {
        let app = App::new();
        assert_eq!(app.cumulative_dream_candidates, 0);
    }

    #[test]
    fn cumulative_dream_values_persist_in_app() {
        let mut app = App::new();

        // Simulate updating values
        app.cumulative_dream_strengthened = 42;
        app.cumulative_dream_candidates = 100;

        assert_eq!(app.cumulative_dream_strengthened, 42);
        assert_eq!(app.cumulative_dream_candidates, 100);

        // Verify values persist across clones
        let cloned_app = app.clone();
        assert_eq!(cloned_app.cumulative_dream_strengthened, 42);
        assert_eq!(cloned_app.cumulative_dream_candidates, 100);
    }

    #[test]
    fn cumulative_dream_efficiency_calculation() {
        let mut app = App::new();

        // Test zero case
        let efficiency = if app.cumulative_dream_candidates > 0 {
            (app.cumulative_dream_strengthened as f32 / app.cumulative_dream_candidates as f32) * 100.0
        } else {
            0.0
        };
        assert_eq!(efficiency, 0.0);

        // Test 50% efficiency
        app.cumulative_dream_strengthened = 50;
        app.cumulative_dream_candidates = 100;
        let efficiency = (app.cumulative_dream_strengthened as f32 / app.cumulative_dream_candidates as f32) * 100.0;
        assert!((efficiency - 50.0).abs() < 0.01);

        // Test 100% efficiency
        app.cumulative_dream_strengthened = 100;
        app.cumulative_dream_candidates = 100;
        let efficiency = (app.cumulative_dream_strengthened as f32 / app.cumulative_dream_candidates as f32) * 100.0;
        assert!((efficiency - 100.0).abs() < 0.01);

        // Test fractional efficiency
        app.cumulative_dream_strengthened = 33;
        app.cumulative_dream_candidates = 100;
        let efficiency = (app.cumulative_dream_strengthened as f32 / app.cumulative_dream_candidates as f32) * 100.0;
        assert!((efficiency - 33.0).abs() < 0.01);
    }

    #[test]
    fn cumulative_dream_values_accumulate() {
        let mut app = App::new();

        // Simulate multiple dream cycles
        app.cumulative_dream_strengthened += 10;
        app.cumulative_dream_candidates += 20;
        assert_eq!(app.cumulative_dream_strengthened, 10);
        assert_eq!(app.cumulative_dream_candidates, 20);

        app.cumulative_dream_strengthened += 15;
        app.cumulative_dream_candidates += 30;
        assert_eq!(app.cumulative_dream_strengthened, 25);
        assert_eq!(app.cumulative_dream_candidates, 50);

        // Verify efficiency after accumulation
        let efficiency = (app.cumulative_dream_strengthened as f32 / app.cumulative_dream_candidates as f32) * 100.0;
        assert!((efficiency - 50.0).abs() < 0.01);
    }

    // =========================================================================
    // TUI-VIS-6: Volition Veto Log Tests
    // =========================================================================

    #[test]
    fn veto_log_starts_empty() {
        let app = App::new();
        assert_eq!(app.vetoes.len(), 0);
        assert_eq!(app.veto_count, 0);
    }

    #[test]
    fn veto_entry_creation_with_reason_and_value() {
        let veto = VetoEntry {
            timestamp: Instant::now(),
            reason: "Violates core value".to_string(),
            violated_value: Some("honesty".to_string()),
        };

        assert_eq!(veto.reason, "Violates core value");
        assert_eq!(veto.violated_value, Some("honesty".to_string()));
    }

    #[test]
    fn veto_entry_creation_without_violated_value() {
        let veto = VetoEntry {
            timestamp: Instant::now(),
            reason: "Unknown violation".to_string(),
            violated_value: None,
        };

        assert_eq!(veto.reason, "Unknown violation");
        assert!(veto.violated_value.is_none());
    }

    #[test]
    fn add_veto_increments_count() {
        let mut app = App::new();
        assert_eq!(app.veto_count, 0);

        app.add_veto(
            "Test veto reason".to_string(),
            Some("integrity".to_string()),
        );
        assert_eq!(app.veto_count, 1);

        app.add_veto("Another veto".to_string(), None);
        assert_eq!(app.veto_count, 2);
    }

    #[test]
    fn add_veto_adds_entry_to_queue() {
        let mut app = App::new();

        app.add_veto(
            "Dishonest thought detected".to_string(),
            Some("honesty".to_string()),
        );

        assert_eq!(app.vetoes.len(), 1);
        assert_eq!(app.vetoes[0].reason, "Dishonest thought detected");
        assert_eq!(app.vetoes[0].violated_value, Some("honesty".to_string()));
    }

    #[test]
    fn add_veto_respects_max_size() {
        let mut app = App::new();

        // Add MAX_VETOES + 10 entries
        for i in 0..60 {
            app.add_veto(format!("Veto {}", i), Some(format!("value_{}", i)));
        }

        // Queue should be capped at MAX_VETOES (50)
        assert_eq!(app.vetoes.len(), MAX_VETOES);

        // First entry should be veto 10 (first 10 were evicted)
        assert_eq!(app.vetoes[0].reason, "Veto 10");
        assert_eq!(app.vetoes[0].violated_value, Some("value_10".to_string()));

        // Last entry should be veto 59
        assert_eq!(app.vetoes[49].reason, "Veto 59");

        // Veto count should still track all 60 vetoes
        assert_eq!(app.veto_count, 60);
    }

    #[test]
    fn add_veto_maintains_chronological_order() {
        let mut app = App::new();

        // Add vetoes in sequence
        for i in 0..5 {
            app.add_veto(format!("Veto {}", i), None);
        }

        // Verify chronological order (oldest to newest)
        for i in 0..5 {
            assert_eq!(app.vetoes[i].reason, format!("Veto {}", i));
        }
    }

    #[test]
    fn add_veto_with_various_value_formats() {
        let mut app = App::new();

        // Veto with value
        app.add_veto("Test 1".to_string(), Some("honesty".to_string()));
        assert_eq!(app.vetoes[0].violated_value, Some("honesty".to_string()));

        // Veto without value
        app.add_veto("Test 2".to_string(), None);
        assert!(app.vetoes[1].violated_value.is_none());

        // Veto with complex value name
        app.add_veto(
            "Test 3".to_string(),
            Some("life honours life".to_string()),
        );
        assert_eq!(
            app.vetoes[2].violated_value,
            Some("life honours life".to_string())
        );
    }

    #[test]
    fn veto_timestamp_is_recent() {
        use std::time::Duration;

        let mut app = App::new();
        let before = Instant::now();

        app.add_veto("Test veto".to_string(), None);

        let after = Instant::now();
        let veto_time = app.vetoes[0].timestamp;

        // Veto timestamp should be between before and after
        assert!(veto_time >= before);
        assert!(veto_time <= after);

        // Should be very recent (less than 100ms ago)
        assert!(veto_time.elapsed() < Duration::from_millis(100));
    }

    #[test]
    fn multiple_vetoes_tracked_separately() {
        let mut app = App::new();

        app.add_veto("First veto".to_string(), Some("honesty".to_string()));
        app.add_veto("Second veto".to_string(), Some("integrity".to_string()));
        app.add_veto("Third veto".to_string(), None);

        assert_eq!(app.vetoes.len(), 3);
        assert_eq!(app.veto_count, 3);

        // Each veto should have distinct data
        assert_eq!(app.vetoes[0].reason, "First veto");
        assert_eq!(app.vetoes[1].reason, "Second veto");
        assert_eq!(app.vetoes[2].reason, "Third veto");

        assert_eq!(app.vetoes[0].violated_value, Some("honesty".to_string()));
        assert_eq!(
            app.vetoes[1].violated_value,
            Some("integrity".to_string())
        );
        assert!(app.vetoes[2].violated_value.is_none());
    }

    #[test]
    fn veto_entry_is_cloneable() {
        let veto = VetoEntry {
            timestamp: Instant::now(),
            reason: "Test reason".to_string(),
            violated_value: Some("test_value".to_string()),
        };

        let cloned = veto.clone();

        assert_eq!(cloned.reason, veto.reason);
        assert_eq!(cloned.violated_value, veto.violated_value);
    }

    #[test]
    fn veto_display_data_structure() {
        let mut app = App::new();

        // Add a veto with violated value
        app.add_veto(
            "Thought conflicts with stated values".to_string(),
            Some("transparency".to_string()),
        );

        // Verify display data is properly structured
        let veto = &app.vetoes[0];
        assert_eq!(veto.reason, "Thought conflicts with stated values");
        assert_eq!(veto.violated_value, Some("transparency".to_string()));

        // Format check: violated value should be presentable
        let value_str = if let Some(ref value) = veto.violated_value {
            format!("[{}] ", value)
        } else {
            String::from("[unknown] ")
        };
        assert_eq!(value_str, "[transparency] ");
    }

    #[test]
    fn veto_display_without_value() {
        let mut app = App::new();

        app.add_veto("Generic violation".to_string(), None);

        let veto = &app.vetoes[0];
        let value_str = if let Some(ref value) = veto.violated_value {
            format!("[{}] ", value)
        } else {
            String::from("[unknown] ")
        };
        assert_eq!(value_str, "[unknown] ");
    }

    // =========================================================================
    // StreamCompetition Tests (TUI-VIS-5)
    // =========================================================================

    #[test]
    fn stream_competition_default_initialization() {
        let competition = StreamCompetition::default();

        // All activity should start at 0.0
        assert_eq!(competition.activity.len(), 9);
        assert!(competition.activity.iter().all(|&a| a == 0.0));

        // All history arrays should be empty
        assert_eq!(competition.history.len(), 9);
        assert!(competition.history.iter().all(|h| h.is_empty()));

        // Dominant stream should start at 0
        assert_eq!(competition.dominant_stream, 0);
    }

    #[test]
    fn stream_competition_via_app_initialization() {
        let app = App::new();

        // Verify stream_competition is properly initialized
        assert_eq!(app.stream_competition.activity.len(), 9);
        assert!(app.stream_competition.activity.iter().all(|&a| a == 0.0));
        assert_eq!(app.stream_competition.dominant_stream, 0);
    }

    #[test]
    fn update_stream_competition_valid_window() {
        let mut app = App::new();

        // Update window_0 with high salience
        app.update_stream_competition("window_0", 0.8);

        // Activity should be increased (uses EMA with alpha=0.3)
        // First update: 0.3 * 0.8 + 0.7 * 0.0 = 0.24
        assert!(app.stream_competition.activity[0] > 0.0);
        assert!(app.stream_competition.activity[0] <= 0.8);
    }

    #[test]
    fn update_stream_competition_multiple_windows() {
        let mut app = App::new();

        // Update different windows
        app.update_stream_competition("window_0", 0.8);
        app.update_stream_competition("window_3", 0.6);
        app.update_stream_competition("window_8", 0.4);

        // Check that correct windows were updated
        assert!(app.stream_competition.activity[0] > 0.0);
        assert!(app.stream_competition.activity[3] > 0.0);
        assert!(app.stream_competition.activity[8] > 0.0);

        // Other windows should still be 0
        assert_eq!(app.stream_competition.activity[1], 0.0);
        assert_eq!(app.stream_competition.activity[2], 0.0);
    }

    #[test]
    fn update_stream_competition_exponential_moving_average() {
        let mut app = App::new();

        // First update
        app.update_stream_competition("window_0", 1.0);
        let activity_1 = app.stream_competition.activity[0];

        // Second update with same value
        app.update_stream_competition("window_0", 1.0);
        let activity_2 = app.stream_competition.activity[0];

        // Activity should increase (approaching 1.0 via EMA)
        assert!(activity_2 > activity_1);
        assert!(activity_2 <= 1.0);
    }

    #[test]
    fn update_stream_competition_invalid_window_name() {
        let mut app = App::new();

        // Invalid window names should not panic
        app.update_stream_competition("invalid", 0.8);
        app.update_stream_competition("window_", 0.8);
        app.update_stream_competition("window_abc", 0.8);
        app.update_stream_competition("notawindow_5", 0.8);

        // All activities should still be 0
        assert!(app.stream_competition.activity.iter().all(|&a| a == 0.0));
    }

    #[test]
    fn update_stream_competition_out_of_bounds_index() {
        let mut app = App::new();

        // Window indices beyond 8 should not panic
        app.update_stream_competition("window_9", 0.8);
        app.update_stream_competition("window_10", 0.8);
        app.update_stream_competition("window_100", 0.8);

        // All activities should still be 0
        assert!(app.stream_competition.activity.iter().all(|&a| a == 0.0));
    }

    #[test]
    fn add_thought_updates_stream_competition() {
        let mut app = App::new();

        // Adding thought should automatically update stream competition
        app.add_thought(
            0.9,
            0.0,
            0.5,
            "window_2".to_string(),
            ThoughtStatus::Salient,
        );

        // Window 2 should have activity
        assert!(app.stream_competition.activity[2] > 0.0);
    }

    #[test]
    fn decay_stream_competition_reduces_activity() {
        let mut app = App::new();

        // Set up some activity
        app.stream_competition.activity[0] = 1.0;
        app.stream_competition.activity[3] = 0.5;

        // Apply decay
        app.decay_stream_competition(Duration::from_secs(1));

        // Activities should be reduced
        assert!(app.stream_competition.activity[0] < 1.0);
        assert!(app.stream_competition.activity[3] < 0.5);
    }

    #[test]
    fn decay_stream_competition_decay_rate() {
        let mut app = App::new();
        app.stream_competition.activity[0] = 1.0;

        // Decay with 1 second
        app.decay_stream_competition(Duration::from_secs(1));

        // Should be 0.95^1 = 0.95
        let expected = 0.95_f32.powf(1.0);
        assert!((app.stream_competition.activity[0] - expected).abs() < 0.01);
    }

    #[test]
    fn decay_stream_competition_longer_duration() {
        let mut app = App::new();
        app.stream_competition.activity[0] = 1.0;

        // Decay with 2 seconds
        app.decay_stream_competition(Duration::from_secs(2));

        // Should be 0.95^2 ≈ 0.9025
        let expected = 0.95_f32.powf(2.0);
        assert!((app.stream_competition.activity[0] - expected).abs() < 0.01);
    }

    #[test]
    fn decay_stream_competition_updates_history() {
        let mut app = App::new();
        app.stream_competition.activity[0] = 0.8;
        app.stream_competition.activity[5] = 0.3;

        // Set last_update to more than 1 second ago to trigger history update
        app.stream_competition.last_update = Instant::now() - Duration::from_secs(2);

        // Decay should update history
        app.decay_stream_competition(Duration::from_millis(100));

        // History should have entries now
        assert_eq!(app.stream_competition.history[0].len(), 1);
        assert_eq!(app.stream_competition.history[5].len(), 1);
    }

    #[test]
    fn decay_stream_competition_limits_history_length() {
        let mut app = App::new();
        app.stream_competition.activity[0] = 0.5;

        // Force history updates by repeatedly setting last_update in the past
        for _ in 0..25 {
            app.stream_competition.last_update = Instant::now() - Duration::from_secs(2);
            app.decay_stream_competition(Duration::from_millis(100));
        }

        // History should be capped at 20 entries
        assert!(app.stream_competition.history[0].len() <= 20);
    }

    #[test]
    fn decay_stream_competition_detects_dominant_stream() {
        let mut app = App::new();

        // Set different activity levels
        app.stream_competition.activity[0] = 0.3;
        app.stream_competition.activity[2] = 0.9; // Highest
        app.stream_competition.activity[5] = 0.5;

        // Trigger dominant stream update
        app.stream_competition.last_update = Instant::now() - Duration::from_secs(2);
        app.decay_stream_competition(Duration::from_millis(100));

        // Window 2 should be dominant
        assert_eq!(app.stream_competition.dominant_stream, 2);
    }

    #[test]
    fn decay_stream_competition_dominant_stream_tie() {
        let mut app = App::new();

        // Set equal activity levels
        app.stream_competition.activity[3] = 0.8;
        app.stream_competition.activity[7] = 0.8;

        // Trigger dominant stream update
        app.stream_competition.last_update = Instant::now() - Duration::from_secs(2);
        app.decay_stream_competition(Duration::from_millis(100));

        // Should pick last one with max value (index 7) due to max_by behavior
        assert_eq!(app.stream_competition.dominant_stream, 7);
    }

    #[test]
    fn decay_stream_competition_all_zero_activity() {
        let mut app = App::new();

        // All activities at 0
        for activity in &mut app.stream_competition.activity {
            *activity = 0.0;
        }

        // Trigger dominant stream update
        app.stream_competition.last_update = Instant::now() - Duration::from_secs(2);
        app.decay_stream_competition(Duration::from_millis(100));

        // Should pick last window (index 8) when all are equal due to max_by behavior
        assert_eq!(app.stream_competition.dominant_stream, 8);
    }

    #[test]
    fn stream_competition_activity_bar_representation() {
        let mut app = App::new();

        // Set up 9 windows with varying activity
        for i in 0..9 {
            app.stream_competition.activity[i] = (i as f32) / 10.0;
        }

        // Count active streams (activity > 0.1)
        let active = app
            .stream_competition
            .activity
            .iter()
            .filter(|&&a| a > 0.1)
            .count();

        // Windows 2-8 should be active (0.2-0.8 range)
        assert_eq!(active, 7);
    }

    #[test]
    fn stream_competition_integration_test() {
        let mut app = App::new();

        // Simulate thought stream with different windows
        app.add_thought(0.9, 0.5, 0.7, "window_0".to_string(), ThoughtStatus::Salient);
        app.add_thought(0.8, 0.3, 0.6, "window_0".to_string(), ThoughtStatus::Processing);
        app.add_thought(0.7, 0.0, 0.5, "window_3".to_string(), ThoughtStatus::Salient);
        app.add_thought(0.6, -0.2, 0.4, "window_5".to_string(), ThoughtStatus::Processing);

        // Window 0 should have highest activity (two thoughts)
        assert!(app.stream_competition.activity[0] > app.stream_competition.activity[3]);
        assert!(app.stream_competition.activity[0] > app.stream_competition.activity[5]);

        // Windows 3 and 5 should have some activity
        assert!(app.stream_competition.activity[3] > 0.0);
        assert!(app.stream_competition.activity[5] > 0.0);

        // Apply decay
        app.stream_competition.last_update = Instant::now() - Duration::from_secs(2);
        app.decay_stream_competition(Duration::from_secs(1));

        // Window 0 should still be dominant
        assert_eq!(app.stream_competition.dominant_stream, 0);

        // History should be populated
        assert!(!app.stream_competition.history[0].is_empty());
    }
}
