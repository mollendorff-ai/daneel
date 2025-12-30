//! Stream competition tracking for App
//!
//! Methods for tracking and visualizing attention competition across windows.

use std::time::{Duration, Instant};

use super::App;

impl App {
    /// Update stream competition for a window
    pub fn update_stream_competition(&mut self, window: &str, salience: f32) {
        let idx = match window {
            "trigger" => Some(0),
            "autoflow" => Some(1),
            "attention" => Some(2),
            "assembly" => Some(3),
            "anchor" => Some(4),
            "memory" => Some(5),
            "reasoning" => Some(6),
            "emotion" => Some(7),
            "sensory" => Some(8),
            _ => window
                .strip_prefix("window_")
                .and_then(|s| s.parse::<usize>().ok())
                .filter(|&i| i < 9),
        };

        if let Some(idx) = idx {
            let alpha = 0.3;
            self.stream_competition.activity[idx] =
                alpha * salience + (1.0 - alpha) * self.stream_competition.activity[idx];
        }
    }

    /// Decay stream competition activity over time
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn decay_stream_competition(&mut self, delta: Duration) {
        let decay_rate: f32 = 0.95;
        let decay = decay_rate.powf(delta.as_secs_f32());

        for activity in &mut self.stream_competition.activity {
            *activity *= decay;
        }

        if self.stream_competition.last_update.elapsed() >= Duration::from_secs(1) {
            for (i, history) in self.stream_competition.history.iter_mut().enumerate() {
                history.push(self.stream_competition.activity[i]);
                if history.len() > 20 {
                    history.remove(0);
                }
            }

            self.stream_competition.dominant_stream = self
                .stream_competition
                .activity
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .map_or(0, |(idx, _)| idx);

            self.stream_competition.last_update = Instant::now();
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
#[allow(clippy::float_cmp, clippy::cast_precision_loss)]
mod tests {
    use super::*;
    use crate::tui::app::{StreamCompetition, ThoughtStatus};

    // =========================================================================
    // StreamCompetition Tests
    // =========================================================================

    #[test]
    fn stream_competition_default_initialization() {
        let competition = StreamCompetition::default();

        // All activity should start at 0.0
        assert_eq!(competition.activity.len(), 9);
        assert!(competition.activity.iter().all(|&a| a == 0.0));

        // All history arrays should be empty
        assert_eq!(competition.history.len(), 9);
        assert!(competition.history.iter().all(std::vec::Vec::is_empty));

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

        // Update window_0 with high salience (legacy format)
        app.update_stream_competition("window_0", 0.8);

        // Activity should be increased (uses EMA with alpha=0.3)
        // First update: 0.3 * 0.8 + 0.7 * 0.0 = 0.24
        assert!(app.stream_competition.activity[0] > 0.0);
        assert!(app.stream_competition.activity[0] <= 0.8);
    }

    #[test]
    fn update_stream_competition_stage_names() {
        let mut app = App::new();

        // Test all stage names map correctly (HOTFIX Dec 20 2025)
        let stages = [
            ("trigger", 0),
            ("autoflow", 1),
            ("attention", 2),
            ("assembly", 3),
            ("anchor", 4),
            ("memory", 5),
            ("reasoning", 6),
            ("emotion", 7),
            ("sensory", 8),
        ];

        for (stage, expected_idx) in stages {
            app.update_stream_competition(stage, 0.9);
            assert!(
                app.stream_competition.activity[expected_idx] > 0.0,
                "Stage '{stage}' should map to index {expected_idx}"
            );
        }
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

    // =========================================================================
    // Decay Tests
    // =========================================================================

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
        let expected = 0.95_f32.powi(1);
        assert!((app.stream_competition.activity[0] - expected).abs() < 0.01);
    }

    #[test]
    fn decay_stream_competition_longer_duration() {
        let mut app = App::new();
        app.stream_competition.activity[0] = 1.0;

        // Decay with 2 seconds
        app.decay_stream_competition(Duration::from_secs(2));

        // Should be 0.95^2 ≈ 0.9025
        let expected = 0.95_f32.powi(2);
        assert!((app.stream_competition.activity[0] - expected).abs() < 0.01);
    }

    #[test]
    fn decay_stream_competition_updates_history() {
        let mut app = App::new();
        app.stream_competition.activity[0] = 0.8;
        app.stream_competition.activity[5] = 0.3;

        // Set last_update to more than 1 second ago to trigger history update
        app.stream_competition.last_update =
            Instant::now().checked_sub(Duration::from_secs(2)).unwrap();

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
            app.stream_competition.last_update =
                Instant::now().checked_sub(Duration::from_secs(2)).unwrap();
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
        app.stream_competition.last_update =
            Instant::now().checked_sub(Duration::from_secs(2)).unwrap();
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
        app.stream_competition.last_update =
            Instant::now().checked_sub(Duration::from_secs(2)).unwrap();
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
        app.stream_competition.last_update =
            Instant::now().checked_sub(Duration::from_secs(2)).unwrap();
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
        app.add_thought(
            0.9,
            0.5,
            0.7,
            "window_0".to_string(),
            ThoughtStatus::Salient,
        );
        app.add_thought(
            0.8,
            0.3,
            0.6,
            "window_0".to_string(),
            ThoughtStatus::Processing,
        );
        app.add_thought(
            0.7,
            0.0,
            0.5,
            "window_3".to_string(),
            ThoughtStatus::Salient,
        );
        app.add_thought(
            0.6,
            -0.2,
            0.4,
            "window_5".to_string(),
            ThoughtStatus::Processing,
        );

        // Window 0 should have highest activity (two thoughts)
        assert!(app.stream_competition.activity[0] > app.stream_competition.activity[3]);
        assert!(app.stream_competition.activity[0] > app.stream_competition.activity[5]);

        // Windows 3 and 5 should have some activity
        assert!(app.stream_competition.activity[3] > 0.0);
        assert!(app.stream_competition.activity[5] > 0.0);

        // Apply decay
        app.stream_competition.last_update =
            Instant::now().checked_sub(Duration::from_secs(2)).unwrap();
        app.decay_stream_competition(Duration::from_secs(1));

        // Window 0 should still be dominant
        assert_eq!(app.stream_competition.dominant_stream, 0);

        // History should be populated
        assert!(!app.stream_competition.history[0].is_empty());
    }
}
