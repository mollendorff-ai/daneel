//! Memory resurfacing tracking for App
//!
//! Methods for tracking when memories resurface from unconscious.

use std::time::{Duration, Instant};

use super::{App, ResurfacingEvent, ResurfacingTrigger, MAX_RESURFACING_LOG};

impl App {
    /// Update resurfacing count based on recent events
    ///
    /// # Panics
    ///
    /// Panics if 60-second duration subtraction from current time fails (extremely unlikely).
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn update_resurfacing(&mut self) {
        let cutoff = Instant::now().checked_sub(Duration::from_secs(60)).unwrap();

        while let Some(&timestamp) = self.resurfacing_events.front() {
            if timestamp < cutoff {
                self.resurfacing_events.pop_front();
            } else {
                break;
            }
        }

        self.resurfacing_count = self.resurfacing_events.len();
    }

    /// Check if resurfacing is currently active
    #[cfg_attr(coverage_nightly, coverage(off))]
    #[must_use]
    pub fn is_resurfacing_active(&self) -> bool {
        self.last_resurfacing
            .is_some_and(|last| last.elapsed() < Duration::from_secs(2))
    }

    /// Add a detailed resurfacing event
    pub fn add_resurfacing_event(
        &mut self,
        memory_id: String,
        original_salience: f32,
        boosted_salience: f32,
        trigger: ResurfacingTrigger,
        memory_age: Duration,
    ) {
        if self.resurfacing_log.len() >= MAX_RESURFACING_LOG {
            self.resurfacing_log.pop_front();
        }

        let now = Instant::now();
        self.resurfacing_log.push_back(ResurfacingEvent {
            timestamp: now,
            memory_id,
            original_salience,
            boosted_salience,
            trigger,
            memory_age,
        });

        self.last_resurfacing = Some(now);
        self.resurfacing_events.push_back(now);
    }

    /// Get the most recent resurfacing event
    #[must_use]
    pub fn last_resurfacing_event(&self) -> Option<&ResurfacingEvent> {
        self.resurfacing_log.back()
    }

    /// Get resurfacing events from the last N seconds
    ///
    /// # Panics
    ///
    /// Panics if `seconds` duration subtraction from `Instant::now()` fails (extremely unlikely).
    #[must_use]
    pub fn recent_resurfacing_events(&self, seconds: u64) -> Vec<&ResurfacingEvent> {
        let cutoff = Instant::now()
            .checked_sub(Duration::from_secs(seconds))
            .unwrap();
        self.resurfacing_log
            .iter()
            .filter(|e| e.timestamp >= cutoff)
            .collect()
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
#[allow(clippy::float_cmp, clippy::cast_sign_loss, clippy::cast_precision_loss)]
mod tests {
    use super::*;
    use crate::tui::app::ThoughtStatus;

    // =========================================================================
    // Resurfacing State Tests
    // =========================================================================

    #[test]
    fn resurfacing_count_starts_zero() {
        let app = App::new();
        assert_eq!(app.resurfacing_count, 0);
        assert!(app.last_resurfacing.is_none());
    }

    #[test]
    fn is_resurfacing_active_false_when_no_events() {
        let app = App::new();
        assert!(!app.is_resurfacing_active());
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

    // =========================================================================
    // Update Resurfacing Tests
    // =========================================================================

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
    fn update_resurfacing_keeps_recent_events() {
        let mut app = App::new();

        // Add a consolidated thought (which adds to resurfacing_events)
        app.add_thought(
            0.9,
            0.5,
            0.8,
            "test".to_string(),
            ThoughtStatus::Consolidated,
        );

        // Update immediately - event should be kept (not older than 60 seconds)
        app.update_resurfacing();

        // Event should still be there
        assert_eq!(app.resurfacing_count, 1);
        assert_eq!(app.resurfacing_events.len(), 1);
    }

    #[test]
    fn update_resurfacing_empty_events() {
        let mut app = App::new();

        // No events added
        app.update_resurfacing();

        assert_eq!(app.resurfacing_count, 0);
    }

    #[test]
    fn update_resurfacing_removes_old_events() {
        let mut app = App::new();

        // Manually add an old event (older than 60 seconds)
        let old_timestamp = Instant::now()
            .checked_sub(Duration::from_secs(120))
            .unwrap();
        app.resurfacing_events.push_back(old_timestamp);
        app.resurfacing_count = 1;

        // Also add a recent event
        app.resurfacing_events.push_back(Instant::now());
        app.resurfacing_count = 2;

        // Update should remove the old event but keep the recent one
        app.update_resurfacing();

        assert_eq!(app.resurfacing_count, 1);
        assert_eq!(app.resurfacing_events.len(), 1);
    }

    // =========================================================================
    // Add Resurfacing Event Tests
    // =========================================================================

    #[test]
    fn add_resurfacing_event_basic() {
        let mut app = App::new();

        app.add_resurfacing_event(
            "mem_001".to_string(),
            0.3,
            0.8,
            ResurfacingTrigger::Similarity,
            Duration::from_secs(3600),
        );

        assert_eq!(app.resurfacing_log.len(), 1);
        assert!(app.last_resurfacing.is_some());
        assert_eq!(app.resurfacing_events.len(), 1);
    }

    #[test]
    fn add_resurfacing_event_stores_correct_data() {
        let mut app = App::new();

        app.add_resurfacing_event(
            "mem_test".to_string(),
            0.2,
            0.9,
            ResurfacingTrigger::DreamReplay,
            Duration::from_secs(7200),
        );

        let event = app.resurfacing_log.back().unwrap();
        assert_eq!(event.memory_id, "mem_test");
        assert_eq!(event.original_salience, 0.2);
        assert_eq!(event.boosted_salience, 0.9);
        assert_eq!(event.trigger, ResurfacingTrigger::DreamReplay);
        assert_eq!(event.memory_age, Duration::from_secs(7200));
    }

    #[test]
    fn add_resurfacing_event_respects_max_size() {
        let mut app = App::new();

        // Add MAX_RESURFACING_LOG + 10 events
        for i in 0..60 {
            app.add_resurfacing_event(
                format!("mem_{i}"),
                0.3,
                0.8,
                ResurfacingTrigger::Spontaneous,
                Duration::from_secs(i as u64),
            );
        }

        // Log should be capped at MAX_RESURFACING_LOG (50)
        assert_eq!(app.resurfacing_log.len(), MAX_RESURFACING_LOG);

        // First entry should be mem_10 (first 10 were evicted)
        assert_eq!(app.resurfacing_log.front().unwrap().memory_id, "mem_10");
    }

    #[test]
    fn add_resurfacing_event_with_all_triggers() {
        let mut app = App::new();

        app.add_resurfacing_event(
            "mem_1".to_string(),
            0.3,
            0.8,
            ResurfacingTrigger::Similarity,
            Duration::from_secs(100),
        );
        app.add_resurfacing_event(
            "mem_2".to_string(),
            0.3,
            0.8,
            ResurfacingTrigger::DreamReplay,
            Duration::from_secs(200),
        );
        app.add_resurfacing_event(
            "mem_3".to_string(),
            0.3,
            0.8,
            ResurfacingTrigger::Spontaneous,
            Duration::from_secs(300),
        );
        app.add_resurfacing_event(
            "mem_4".to_string(),
            0.3,
            0.8,
            ResurfacingTrigger::Unknown,
            Duration::from_secs(400),
        );

        assert_eq!(app.resurfacing_log.len(), 4);

        // Verify triggers
        assert_eq!(
            app.resurfacing_log[0].trigger,
            ResurfacingTrigger::Similarity
        );
        assert_eq!(
            app.resurfacing_log[1].trigger,
            ResurfacingTrigger::DreamReplay
        );
        assert_eq!(
            app.resurfacing_log[2].trigger,
            ResurfacingTrigger::Spontaneous
        );
        assert_eq!(app.resurfacing_log[3].trigger, ResurfacingTrigger::Unknown);
    }

    // =========================================================================
    // Last Resurfacing Event Tests
    // =========================================================================

    #[test]
    fn last_resurfacing_event_returns_none_when_empty() {
        let app = App::new();
        assert!(app.last_resurfacing_event().is_none());
    }

    #[test]
    fn last_resurfacing_event_returns_most_recent() {
        let mut app = App::new();

        app.add_resurfacing_event(
            "mem_old".to_string(),
            0.3,
            0.8,
            ResurfacingTrigger::Similarity,
            Duration::from_secs(100),
        );
        app.add_resurfacing_event(
            "mem_new".to_string(),
            0.4,
            0.9,
            ResurfacingTrigger::DreamReplay,
            Duration::from_secs(200),
        );

        let last = app.last_resurfacing_event().unwrap();
        assert_eq!(last.memory_id, "mem_new");
    }

    // =========================================================================
    // Recent Resurfacing Events Tests
    // =========================================================================

    #[test]
    fn recent_resurfacing_events_returns_empty_when_no_events() {
        let app = App::new();
        let recent = app.recent_resurfacing_events(60);
        assert!(recent.is_empty());
    }

    #[test]
    fn recent_resurfacing_events_returns_all_recent() {
        let mut app = App::new();

        // Add 3 events just now
        for i in 0..3 {
            app.add_resurfacing_event(
                format!("mem_{i}"),
                0.3,
                0.8,
                ResurfacingTrigger::Similarity,
                Duration::from_secs(i as u64),
            );
        }

        let recent = app.recent_resurfacing_events(60);
        assert_eq!(recent.len(), 3);
    }

    #[test]
    fn recent_resurfacing_events_filter_by_seconds() {
        let mut app = App::new();

        // Add one event now
        app.add_resurfacing_event(
            "mem_recent".to_string(),
            0.3,
            0.8,
            ResurfacingTrigger::Similarity,
            Duration::from_secs(100),
        );

        // Events within last 1 second should include the recent one
        let recent = app.recent_resurfacing_events(1);
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].memory_id, "mem_recent");
    }

    // =========================================================================
    // ResurfacingEvent Tests
    // =========================================================================

    #[test]
    fn resurfacing_event_is_cloneable() {
        let event = ResurfacingEvent {
            timestamp: Instant::now(),
            memory_id: "test_mem".to_string(),
            original_salience: 0.3,
            boosted_salience: 0.8,
            trigger: ResurfacingTrigger::Similarity,
            memory_age: Duration::from_secs(3600),
        };

        let cloned = event.clone();
        assert_eq!(cloned.memory_id, event.memory_id);
        assert_eq!(cloned.original_salience, event.original_salience);
        assert_eq!(cloned.trigger, event.trigger);
    }

    // =========================================================================
    // Cumulative Dream Strengthening Tests (TUI-VIS-4)
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
    #[allow(clippy::cast_precision_loss)]
    fn cumulative_dream_values_persist_in_app() {
        let mut app = App::new();

        // Simulate updating values
        app.cumulative_dream_strengthened = 42;
        app.cumulative_dream_candidates = 100;

        assert_eq!(app.cumulative_dream_strengthened, 42);
        assert_eq!(app.cumulative_dream_candidates, 100);

        // Verify values persist across clones
        let cloned_app = app;
        assert_eq!(cloned_app.cumulative_dream_strengthened, 42);
        assert_eq!(cloned_app.cumulative_dream_candidates, 100);
    }

    #[test]
    #[allow(clippy::cast_precision_loss)]
    fn cumulative_dream_efficiency_calculation() {
        let mut app = App::new();

        // Test zero case
        let efficiency = if app.cumulative_dream_candidates > 0 {
            (app.cumulative_dream_strengthened as f32 / app.cumulative_dream_candidates as f32)
                * 100.0
        } else {
            0.0
        };
        assert_eq!(efficiency, 0.0);

        // Test 50% efficiency
        app.cumulative_dream_strengthened = 50;
        app.cumulative_dream_candidates = 100;
        let efficiency = (app.cumulative_dream_strengthened as f32
            / app.cumulative_dream_candidates as f32)
            * 100.0;
        assert!((efficiency - 50.0).abs() < 0.01);

        // Test 100% efficiency
        app.cumulative_dream_strengthened = 100;
        app.cumulative_dream_candidates = 100;
        let efficiency = (app.cumulative_dream_strengthened as f32
            / app.cumulative_dream_candidates as f32)
            * 100.0;
        assert!((efficiency - 100.0).abs() < 0.01);

        // Test fractional efficiency
        app.cumulative_dream_strengthened = 33;
        app.cumulative_dream_candidates = 100;
        let efficiency = (app.cumulative_dream_strengthened as f32
            / app.cumulative_dream_candidates as f32)
            * 100.0;
        assert!((efficiency - 33.0).abs() < 0.01);
    }

    #[test]
    #[allow(clippy::cast_precision_loss)]
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
        let efficiency = (app.cumulative_dream_strengthened as f32
            / app.cumulative_dream_candidates as f32)
            * 100.0;
        assert!((efficiency - 50.0).abs() < 0.01);
    }
}
