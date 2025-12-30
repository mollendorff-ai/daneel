//! Thought and veto management for App
//!
//! Methods for adding thoughts and vetoes to the TUI state.

use std::time::Instant;

use super::{
    App, ThoughtEntry, ThoughtStatus, VetoEntry, MAX_INTER_ARRIVAL, MAX_THOUGHTS, MAX_VETOES,
};

impl App {
    /// Add a thought to the stream
    #[allow(clippy::cast_precision_loss)]
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn add_thought(
        &mut self,
        salience: f32,
        valence: f32,
        arousal: f32,
        window: String,
        status: ThoughtStatus,
    ) {
        let now = Instant::now();

        if self.thoughts.len() >= MAX_THOUGHTS {
            self.thoughts.pop_front();
        }

        if let Some(last_time) = self.last_thought_time {
            let gap = now.duration_since(last_time);
            if self.inter_arrival_times.len() >= MAX_INTER_ARRIVAL {
                self.inter_arrival_times.pop_front();
            }
            self.inter_arrival_times.push_back(gap);
        }
        self.last_thought_time = Some(now);

        self.update_stream_competition(&window, salience);

        self.thoughts.push_back(ThoughtEntry {
            timestamp: now,
            salience,
            valence,
            arousal,
            window,
            status,
        });
        self.thought_count += 1;

        if status == ThoughtStatus::Consolidated {
            self.last_resurfacing = Some(now);
            self.resurfacing_events.push_back(now);
        }

        let elapsed_hours = self.start_time.elapsed().as_secs_f32() / 3600.0;
        if elapsed_hours > 0.0 {
            self.thoughts_per_hour = self.thought_count as f32 / elapsed_hours;
        }

        if self.thought_count.is_multiple_of(5) {
            self.update_entropy();
        }

        if self.thought_count.is_multiple_of(10) {
            self.update_fractality();
        }
    }

    /// Add a veto entry to the log
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
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

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
                format!("window_{i}"),
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
    // Veto Tests
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
            app.add_veto(format!("Veto {i}"), Some(format!("value_{i}")));
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
            app.add_veto(format!("Veto {i}"), None);
        }

        // Verify chronological order (oldest to newest)
        for i in 0..5 {
            assert_eq!(app.vetoes[i].reason, format!("Veto {i}"));
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
        app.add_veto("Test 3".to_string(), Some("life honours life".to_string()));
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
        assert_eq!(app.vetoes[1].violated_value, Some("integrity".to_string()));
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
        let value_str = veto
            .violated_value
            .as_ref()
            .map_or_else(|| String::from("[unknown] "), |value| format!("[{value}] "));
        assert_eq!(value_str, "[transparency] ");
    }

    #[test]
    fn veto_display_without_value() {
        let mut app = App::new();

        app.add_veto("Generic violation".to_string(), None);

        let veto = &app.vetoes[0];
        let value_str = veto
            .violated_value
            .as_ref()
            .map_or_else(|| String::from("[unknown] "), |value| format!("[{value}] "));
        assert_eq!(value_str, "[unknown] ");
    }

    // =========================================================================
    // ThoughtEntry Tests
    // =========================================================================

    #[test]
    fn thought_entry_is_cloneable() {
        let entry = ThoughtEntry {
            timestamp: Instant::now(),
            salience: 0.5,
            valence: 0.3,
            arousal: 0.7,
            window: "test_window".to_string(),
            status: ThoughtStatus::Processing,
        };

        let cloned = entry.clone();
        assert_eq!(cloned.salience, entry.salience);
        assert_eq!(cloned.valence, entry.valence);
        assert_eq!(cloned.arousal, entry.arousal);
        assert_eq!(cloned.window, entry.window);
        assert_eq!(cloned.status, entry.status);
    }

    // =========================================================================
    // Consolidated Thought Resurfacing Tests
    // =========================================================================

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
}
