//! Pulse Fractality Panel
//!
//! Visualizes the transition from clockwork to fractal thought patterns.
//! Tracks inter-arrival time variance, burst ratio, and overall fractality.
//!
//! Key insight: Early cognition is periodic (clockwork). As coherence develops,
//! patterns should become more fractal - the "lived arrhythmia" of real psychology.
//!
//! SOURCE OF TRUTH: Redis stream (daneel:stream:awake), NOT Qdrant.
//! Fractality is EMERGENT from stream dynamics - it resets on restart and re-emerges.
//! You don't store a heartbeat. You measure it.
//! See ADR-040: Fractality Source of Truth
//!
//! Simplified metrics until Forge gets FFT/Hurst/DFA:
//! - Inter-arrival σ: stddev of time gaps (low=clockwork, high=bursty)
//! - Burst ratio: max_gap / mean_gap (detects clustering)
//! - Fractality score: normalized composite (0=clockwork, 1=fractal)

use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui::app::App;
use crate::tui::colors;

/// Unicode block elements for sparklines
const SPARK_CHARS: [char; 8] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '█'];

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" FRACTALITY (stream) ")
        .title_style(Style::default().fg(colors::HIGHLIGHT).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::DIM));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut lines = Vec::new();

    // Pattern line with progress bar
    let score = app.fractality.fractality_score;
    let description = app.fractality_description();
    let desc_color = match description {
        "EMERGENT" => colors::SUCCESS,
        "BALANCED" => colors::PRIMARY,
        _ => colors::DIM,
    };

    // Progress bar (12 chars)
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let filled = (score * 12.0).round() as usize;
    let bar = "█".repeat(filled) + &"░".repeat(12 - filled);

    lines.push(Line::from(vec![
        Span::styled("Pattern: ", Style::default().fg(colors::DIM)),
        Span::styled(
            format!("{:9}", description),
            Style::default().fg(desc_color).bold(),
        ),
        Span::raw(" ["),
        Span::styled(bar, Style::default().fg(desc_color)),
        Span::raw("] "),
        Span::styled(
            format!("{:3.0}%", score * 100.0),
            Style::default().fg(colors::FOREGROUND),
        ),
    ]));

    // Separator
    lines.push(Line::from(""));

    // Inter-arrival sigma with boot comparison
    let sigma = app.fractality.inter_arrival_sigma;
    let boot_sigma = app.fractality.boot_sigma;
    let sigma_trend = if boot_sigma > 0.0 && sigma > boot_sigma {
        format!("↑ from {:.2}s boot", boot_sigma)
    } else if boot_sigma > 0.0 {
        format!("↓ from {:.2}s boot", boot_sigma)
    } else {
        "measuring...".to_string()
    };

    lines.push(Line::from(vec![
        Span::styled("Inter-arrival σ: ", Style::default().fg(colors::DIM)),
        Span::styled(
            format!("{:.3}s", sigma),
            Style::default().fg(colors::FOREGROUND).bold(),
        ),
        Span::raw("  "),
        Span::styled(
            format!("({})", sigma_trend),
            Style::default().fg(colors::SECONDARY),
        ),
    ]));

    // Burst ratio
    let burst = app.fractality.burst_ratio;
    let burst_desc = if burst > 3.0 {
        "clustering detected"
    } else if burst > 1.5 {
        "some bursting"
    } else {
        "uniform"
    };
    let burst_color = if burst > 3.0 {
        colors::SUCCESS
    } else if burst > 1.5 {
        colors::PRIMARY
    } else {
        colors::DIM
    };

    lines.push(Line::from(vec![
        Span::styled("Burst ratio:     ", Style::default().fg(colors::DIM)),
        Span::styled(
            format!("{:.1}x", burst),
            Style::default().fg(burst_color).bold(),
        ),
        Span::raw("   "),
        Span::styled(
            format!("({})", burst_desc),
            Style::default().fg(colors::SECONDARY),
        ),
    ]));

    // Trend sparkline
    let sparkline = create_sparkline(&app.fractality.history, 20);
    lines.push(Line::from(vec![
        Span::styled("Trend: ", Style::default().fg(colors::DIM)),
        Span::styled(sparkline, Style::default().fg(colors::SECONDARY)),
        Span::styled(" (clockwork → fractal)", Style::default().fg(colors::DIM)),
    ]));

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

/// Create a sparkline visualization from history data
fn create_sparkline(history: &std::collections::VecDeque<f32>, width: usize) -> String {
    if history.is_empty() {
        return " ".repeat(width);
    }

    // Take the last `width` samples
    let start = history.len().saturating_sub(width);
    let samples: Vec<f32> = history.iter().skip(start).copied().collect();

    samples
        .iter()
        .map(|&value| {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let idx = (value.clamp(0.0, 1.0) * 7.0).round() as usize;
            SPARK_CHARS[idx]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_sparkline_empty() {
        let history = std::collections::VecDeque::new();
        let sparkline = create_sparkline(&history, 10);
        assert_eq!(sparkline.len(), 10);
        assert!(sparkline.chars().all(|c| c == ' '));
    }

    #[test]
    fn create_sparkline_full() {
        let mut history = std::collections::VecDeque::new();
        for i in 0..10 {
            history.push_back(i as f32 / 9.0);
        }
        let sparkline = create_sparkline(&history, 10);
        assert_eq!(sparkline.chars().count(), 10);
        // First should be low, last should be high
        let chars: Vec<char> = sparkline.chars().collect();
        assert_eq!(chars[0], ' ');
        assert_eq!(chars[9], '█');
    }

    #[test]
    fn create_sparkline_limits_width() {
        let mut history = std::collections::VecDeque::new();
        for _ in 0..30 {
            history.push_back(0.5);
        }
        let sparkline = create_sparkline(&history, 10);
        assert_eq!(sparkline.chars().count(), 10);
    }
}
