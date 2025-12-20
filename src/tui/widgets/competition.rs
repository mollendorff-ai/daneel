//! Stream Competition Panel
//!
//! Visualizes how multiple thought streams compete for attention.
//! Shows relative activity levels and highlights the dominant stream.
//!
//! Based on TMI (Theory of Multifocal Intelligence):
//! - Multiple streams process in parallel
//! - Attention selects which stream becomes conscious
//! - Streams compete for the "cognitive spotlight"

use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui::app::App;
use crate::tui::colors;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" STREAM COMPETITION ")
        .title_style(Style::default().fg(colors::HIGHLIGHT).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::DIM));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Build activity bars for each window
    let mut lines = Vec::new();

    // Header line
    lines.push(Line::from(vec![
        Span::styled("Stream", Style::default().fg(colors::DIM)),
        Span::raw("    "),
        Span::styled("Activity Level", Style::default().fg(colors::DIM)),
        Span::raw("                     "),
        Span::styled("Trend", Style::default().fg(colors::DIM)),
    ]));

    // Separator
    lines.push(Line::from(Span::styled(
        "─".repeat(inner.width as usize),
        Style::default().fg(colors::DIM),
    )));

    // Activity bars for each window
    for (i, (&activity, history)) in app
        .stream_competition
        .activity
        .iter()
        .zip(app.stream_competition.history.iter())
        .enumerate()
    {
        let window_label = format!("W{}", i + 1);
        let is_dominant = i == app.stream_competition.dominant_stream;

        // Bar visualization (20 chars wide)
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let filled = (activity.clamp(0.0, 1.0) * 20.0).round() as usize;
        let bar = "█".repeat(filled) + &"░".repeat(20 - filled);

        // Color based on activity level and dominance
        let bar_color = if is_dominant {
            colors::HIGHLIGHT // Dominant stream in highlight color
        } else if activity > 0.7 {
            colors::SUCCESS // High activity
        } else if activity > 0.3 {
            colors::PRIMARY // Medium activity
        } else {
            colors::DIM // Low activity
        };

        // Sparkline trend (last 8 samples)
        let sparkline = create_sparkline(history, 8);

        // Build the line
        let mut line_spans = vec![
            Span::styled(
                format!("{:3}", window_label),
                if is_dominant {
                    Style::default().fg(colors::HIGHLIGHT).bold()
                } else {
                    Style::default().fg(colors::FOREGROUND)
                },
            ),
            Span::raw("  "),
            Span::styled(bar, Style::default().fg(bar_color)),
            Span::raw("  "),
            Span::styled(
                format!("{:4.0}%", activity * 100.0),
                Style::default().fg(colors::DIM),
            ),
            Span::raw("  "),
            Span::styled(sparkline, Style::default().fg(colors::SECONDARY)),
        ];

        // Add dominant marker
        if is_dominant {
            line_spans.push(Span::raw("  "));
            line_spans.push(Span::styled(
                "◄ SPOTLIGHT",
                Style::default().fg(colors::HIGHLIGHT),
            ));
        }

        lines.push(Line::from(line_spans));
    }

    // Add summary line
    lines.push(Line::from(""));
    let active_streams = app
        .stream_competition
        .activity
        .iter()
        .filter(|&&a| a > 0.1)
        .count();
    lines.push(Line::from(vec![
        Span::styled("Active Streams: ", Style::default().fg(colors::DIM)),
        Span::styled(
            format!("{}/9", active_streams),
            Style::default().fg(colors::PRIMARY).bold(),
        ),
        Span::styled("  │  Competition: ", Style::default().fg(colors::DIM)),
        Span::styled(
            calculate_competition_level(active_streams),
            Style::default()
                .fg(if active_streams > 6 {
                    colors::WARNING
                } else if active_streams > 3 {
                    colors::SUCCESS
                } else {
                    colors::DIM
                })
                .bold(),
        ),
    ]));

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

/// Create a sparkline visualization from history data
fn create_sparkline(history: &[f32], width: usize) -> String {
    if history.is_empty() {
        return " ".repeat(width);
    }

    // Use Unicode block elements for sparklines
    const SPARK_CHARS: [char; 8] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '█'];

    // Take the last `width` samples
    let start = history.len().saturating_sub(width);
    let samples = &history[start..];

    samples
        .iter()
        .map(|&value| {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let idx = (value.clamp(0.0, 1.0) * 7.0).round() as usize;
            SPARK_CHARS[idx]
        })
        .collect()
}

/// Calculate competition level description
fn calculate_competition_level(active_streams: usize) -> &'static str {
    match active_streams {
        0..=1 => "Minimal",
        2..=3 => "Low",
        4..=5 => "Moderate",
        6..=7 => "High",
        _ => "Intense",
    }
}
