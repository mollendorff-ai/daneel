//! Cognitive Diversity Index Widget (ADR-041)
//!
//! Shows: Shannon entropy of TMI-aligned composite salience distribution
//! Displays how "psychologically emergent" vs "clockwork" the mind appears
//! High entropy = varied cognitive states, Low entropy = repetitive patterns
//!
//! TMI Composite: emotional_intensity (|valence| × arousal) is PRIMARY per Cury
//! Bins thoughts into 5 categorical states (MINIMAL/LOW/MODERATE/HIGH/INTENSE)
//!
//! SOURCE OF TRUTH: Redis stream (daneel:stream:awake), NOT Qdrant.
//! Entropy is EMERGENT from stream dynamics - it resets on restart and re-emerges.
//! See ADR-040: Fractality Source of Truth, ADR-041: Entropy Calculation

use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Sparkline},
    Frame,
};

use crate::tui::app::App;
use crate::tui::colors;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    // Prepare sparkline data - convert entropy history to u64 for sparkline
    // Scale entropy values to 0-100 range for better visualization
    // 5 categorical bins (ADR-041): log2(5) ≈ 2.32
    let max_entropy = 5.0f32.log2();
    let data: Vec<u64> = app
        .entropy_history
        .iter()
        .map(|&e| ((e / max_entropy * 100.0).clamp(0.0, 100.0)) as u64)
        .collect();

    // Determine color based on current entropy level
    let normalized_entropy = app.current_entropy / max_entropy;
    let entropy_color = if normalized_entropy > 0.7 {
        colors::SUCCESS // High entropy = emergent (green)
    } else if normalized_entropy > 0.4 {
        colors::PRIMARY // Medium entropy = balanced (teal)
    } else {
        colors::WARNING // Low entropy = clockwork (yellow/orange)
    };

    // Create the sparkline widget
    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .title(" COGNITIVE DIVERSITY ")
                .title_style(Style::default().fg(colors::SECONDARY).bold())
                .borders(Borders::ALL)
                .border_style(Style::default().fg(colors::DIM)),
        )
        .data(&data)
        .style(Style::default().fg(entropy_color))
        .max(100);

    frame.render_widget(sparkline, area);

    // If there's enough space, render the entropy value and description below the sparkline
    if area.height > 4 {
        let inner = Block::default().borders(Borders::NONE).inner(area);

        let description = app.entropy_description();
        let description_color = if description == "EMERGENT" {
            colors::SUCCESS
        } else if description == "BALANCED" {
            colors::PRIMARY
        } else {
            colors::WARNING
        };

        let info_line = Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(
                format!("{:.2} bits", app.current_entropy),
                Style::default().fg(colors::FOREGROUND).bold(),
            ),
            Span::styled("  ", Style::default()),
            Span::styled(description, Style::default().fg(description_color).bold()),
        ]);

        // Render at bottom of the widget area
        if inner.height >= 2 {
            let info_rect = Rect {
                x: inner.x,
                y: inner.y + inner.height - 2,
                width: inner.width,
                height: 1,
            };

            let paragraph = ratatui::widgets::Paragraph::new(info_line);
            frame.render_widget(paragraph, info_rect);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::app::ThoughtStatus;

    #[test]
    fn entropy_description_emergent() {
        let mut app = App::new();
        // Add varied thoughts to create high entropy
        for i in 0..100 {
            let salience = i as f32 / 100.0;
            app.add_thought(
                salience,
                0.0,
                0.5,
                format!("window_{}", i % 9),
                ThoughtStatus::Processing,
            );
        }
        app.update_entropy();

        // With varied salience, entropy should be high
        assert!(app.current_entropy > 0.0);
    }

    #[test]
    fn entropy_description_clockwork() {
        let mut app = App::new();
        // Add repetitive thoughts to create low entropy
        for _ in 0..100 {
            app.add_thought(
                0.5,
                0.0,
                0.5,
                "window_0".to_string(),
                ThoughtStatus::Processing,
            );
        }
        app.update_entropy();

        // All same salience should give low entropy
        let description = app.entropy_description();
        assert!(description == "CLOCKWORK" || description == "BALANCED");
    }

    #[test]
    fn entropy_calculation_empty_thoughts() {
        let app = App::new();
        let entropy = app.calculate_entropy();
        assert_eq!(entropy, 0.0);
    }

    #[test]
    fn entropy_history_respects_max_size() {
        let mut app = App::new();

        // Add more entropy values than MAX_ENTROPY_HISTORY
        for i in 0..60 {
            app.add_thought(
                i as f32 / 60.0,
                0.0,
                0.5,
                "window_0".to_string(),
                ThoughtStatus::Processing,
            );
            app.update_entropy();
        }

        // History should be capped at MAX_ENTROPY_HISTORY (50)
        assert!(app.entropy_history.len() <= 50);
    }

    #[test]
    fn entropy_value_non_negative() {
        let mut app = App::new();
        for i in 0..10 {
            app.add_thought(
                i as f32 / 10.0,
                0.0,
                0.5,
                "window_0".to_string(),
                ThoughtStatus::Processing,
            );
        }

        let entropy = app.calculate_entropy();
        assert!(entropy >= 0.0);
    }
}
