//! Identity Panel Widget
//!
//! Shows: Name, Uptime, Thought count, Thoughts/hr, Lifetime count (ADR-034)

use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui::app::App;
use crate::tui::colors;

/// Format a number with comma separators for readability (e.g., 1234567 -> "1,234,567")
fn format_with_commas(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::with_capacity(s.len() + s.len() / 3);
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result
}

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" IDENTITY ")
        .title_style(Style::default().fg(colors::PRIMARY).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::DIM));

    // Format lifetime count with comma separators for readability
    let lifetime_str = format_with_commas(app.lifetime_thought_count);

    let lines = vec![
        Line::from(vec![
            Span::styled("Name: ", Style::default().fg(colors::DIM)),
            Span::styled("Timmy", Style::default().fg(colors::PRIMARY).bold()),
            Span::styled("  Uptime: ", Style::default().fg(colors::DIM)),
            Span::styled(app.uptime_string(), Style::default().fg(colors::FOREGROUND)),
        ]),
        Line::from(vec![
            Span::styled("Thoughts: ", Style::default().fg(colors::DIM)),
            Span::styled(
                format!("{}", app.thought_count),
                Style::default().fg(colors::FOREGROUND),
            ),
            Span::styled("  Lifetime: ", Style::default().fg(colors::DIM)),
            Span::styled(lifetime_str, Style::default().fg(colors::SUCCESS).bold()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Memories: ", Style::default().fg(colors::DIM)),
            Span::styled(
                format!("{}", app.memory_count),
                Style::default().fg(colors::SUCCESS),
            ),
            Span::styled(" ↑", Style::default().fg(colors::SUCCESS)),
            Span::styled("  Rate: ", Style::default().fg(colors::DIM)),
            Span::styled(
                format!("{:.0}/hr", app.thoughts_per_hour),
                Style::default().fg(colors::FOREGROUND),
            ),
        ]),
        Line::from(vec![
            Span::styled("Unconscious: ", Style::default().fg(colors::DIM)),
            Span::styled(
                format!("{}", app.unconscious_count),
                Style::default().fg(colors::SECONDARY),
            ),
            Span::styled(" ↓", Style::default().fg(colors::SECONDARY)),
            Span::styled("  Dreams: ", Style::default().fg(colors::DIM)),
            Span::styled(
                format!("{}", app.dream_cycles),
                Style::default().fg(colors::WARNING),
            ),
            if app.last_dream_strengthened > 0 {
                Span::styled(
                    format!(" (+{})", app.last_dream_strengthened),
                    Style::default().fg(colors::WARNING),
                )
            } else {
                Span::raw("")
            },
        ]),
        // TUI-VIS-4: Cumulative dream strengthening stats
        Line::from(vec![
            Span::styled("Total Strengthened: ", Style::default().fg(colors::DIM)),
            Span::styled(
                format!("{}", app.cumulative_dream_strengthened),
                Style::default().fg(colors::SUCCESS).bold(),
            ),
            Span::styled("  Efficiency: ", Style::default().fg(colors::DIM)),
            {
                // Calculate dream efficiency ratio
                let efficiency = if app.cumulative_dream_candidates > 0 {
                    (app.cumulative_dream_strengthened as f32
                        / app.cumulative_dream_candidates as f32)
                        * 100.0
                } else {
                    0.0
                };
                Span::styled(
                    format!("{:.1}%", efficiency),
                    Style::default().fg(colors::PRIMARY).bold(),
                )
            },
        ]),
        Line::from(vec![
            Span::styled("Resurfacing: ", Style::default().fg(colors::DIM)),
            // Show count with glow effect when active
            if app.is_resurfacing_active() {
                Span::styled(
                    format!("{} ↑↓", app.resurfacing_count),
                    Style::default().fg(colors::HIGHLIGHT).bold(),
                )
            } else if app.resurfacing_count > 0 {
                Span::styled(
                    format!("{} ↑↓", app.resurfacing_count),
                    Style::default().fg(colors::SECONDARY),
                )
            } else {
                Span::styled("0 ↑↓", Style::default().fg(colors::DIM))
            },
        ]),
    ];

    let paragraph = Paragraph::new(lines).block(block);

    frame.render_widget(paragraph, area);
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // TUI-VIS-4: Dream Efficiency Display Tests
    // =========================================================================

    #[test]
    fn efficiency_calculation_zero_candidates() {
        let app = App::new();
        let efficiency = if app.cumulative_dream_candidates > 0 {
            (app.cumulative_dream_strengthened as f32 / app.cumulative_dream_candidates as f32) * 100.0
        } else {
            0.0
        };
        assert_eq!(efficiency, 0.0);
    }

    #[test]
    fn efficiency_calculation_with_values() {
        let mut app = App::new();
        app.cumulative_dream_strengthened = 45;
        app.cumulative_dream_candidates = 100;

        let efficiency = (app.cumulative_dream_strengthened as f32 / app.cumulative_dream_candidates as f32) * 100.0;
        assert!((efficiency - 45.0).abs() < 0.01);
    }

    #[test]
    fn efficiency_formatting() {
        let mut app = App::new();
        app.cumulative_dream_strengthened = 123;
        app.cumulative_dream_candidates = 456;

        let efficiency = (app.cumulative_dream_strengthened as f32 / app.cumulative_dream_candidates as f32) * 100.0;
        let formatted = format!("{:.1}%", efficiency);

        // Should format to 1 decimal place
        assert!(formatted.contains('.'));
        assert!(formatted.ends_with('%'));

        // Verify value is approximately 27.0%
        let expected = 123.0 / 456.0 * 100.0;
        assert!((efficiency - expected).abs() < 0.01);
    }

    #[test]
    fn efficiency_high_values() {
        let mut app = App::new();
        app.cumulative_dream_strengthened = 999_999;
        app.cumulative_dream_candidates = 1_000_000;

        let efficiency = (app.cumulative_dream_strengthened as f32 / app.cumulative_dream_candidates as f32) * 100.0;
        assert!((efficiency - 99.9999).abs() < 0.01);
    }

    #[test]
    fn efficiency_perfect_score() {
        let mut app = App::new();
        app.cumulative_dream_strengthened = 100;
        app.cumulative_dream_candidates = 100;

        let efficiency = (app.cumulative_dream_strengthened as f32 / app.cumulative_dream_candidates as f32) * 100.0;
        assert!((efficiency - 100.0).abs() < 0.01);
    }

    #[test]
    fn format_with_commas_simple() {
        assert_eq!(format_with_commas(123), "123");
        assert_eq!(format_with_commas(1234), "1,234");
        assert_eq!(format_with_commas(12345), "12,345");
        assert_eq!(format_with_commas(123456), "123,456");
        assert_eq!(format_with_commas(1234567), "1,234,567");
    }

    #[test]
    fn format_with_commas_edge_cases() {
        assert_eq!(format_with_commas(0), "0");
        assert_eq!(format_with_commas(1), "1");
        assert_eq!(format_with_commas(999), "999");
        assert_eq!(format_with_commas(1000), "1,000");
    }

    #[test]
    fn format_with_commas_large_numbers() {
        assert_eq!(format_with_commas(1_000_000), "1,000,000");
        assert_eq!(format_with_commas(1_234_567_890), "1,234,567,890");
    }
}
