//! Event handling for App
//!
//! Keyboard events and periodic update methods.

use std::time::{Duration, Instant};

use super::{App, PHILOSOPHY_QUOTES};

impl App {
    /// Update the pulse animation phase
    pub fn update_pulse(&mut self, delta: Duration) {
        self.the_box.pulse_phase += delta.as_secs_f32();
        if self.the_box.pulse_phase > 1.0 {
            self.the_box.pulse_phase -= 1.0;
        }
    }

    /// Update the philosophy quote if enough time has passed
    pub fn update_quote(&mut self) {
        if self.last_quote_change.elapsed() > Duration::from_secs(30) {
            self.quote_index = (self.quote_index + 1) % PHILOSOPHY_QUOTES.len();
            self.last_quote_change = Instant::now();
        }
    }

    /// Get the current philosophy quote
    #[must_use]
    pub fn current_quote(&self) -> &'static str {
        PHILOSOPHY_QUOTES[self.quote_index]
    }

    /// Handle a keyboard event
    pub const fn handle_key(&mut self, key: crossterm::event::KeyCode) {
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
#[cfg_attr(coverage_nightly, coverage(off))]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use crossterm::event::KeyCode;

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

    #[test]
    fn update_quote_does_not_change_before_30_seconds() {
        let mut app = App::new();
        let initial_index = app.quote_index;

        // Immediately call update_quote (much less than 30 seconds)
        app.update_quote();

        // Quote index should not change
        assert_eq!(app.quote_index, initial_index);
    }

    #[test]
    fn update_quote_changes_after_30_seconds() {
        let mut app = App::new();
        let initial_index = app.quote_index;

        // Set last_quote_change to more than 30 seconds ago
        app.last_quote_change = Instant::now().checked_sub(Duration::from_secs(31)).unwrap();

        app.update_quote();

        // Quote index should have advanced
        assert_eq!(
            app.quote_index,
            (initial_index + 1) % PHILOSOPHY_QUOTES.len()
        );
    }

    #[test]
    fn update_quote_wraps_around() {
        let mut app = App::new();

        // Set to last quote
        app.quote_index = PHILOSOPHY_QUOTES.len() - 1;
        app.last_quote_change = Instant::now().checked_sub(Duration::from_secs(31)).unwrap();

        app.update_quote();

        // Should wrap to 0
        assert_eq!(app.quote_index, 0);
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
}
