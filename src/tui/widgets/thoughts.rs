//! Thought Stream Widget
//!
//! Shows: Scrolling log of thoughts with timestamp, salience bar, window, status
//! Color-coded by salience intensity.

use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::tui::app::{App, ThoughtStatus};
use crate::tui::colors;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(if app.stream_paused {
            " THOUGHT STREAM [PAUSED] "
        } else {
            " THOUGHT STREAM "
        })
        .title_style(Style::default().fg(colors::SECONDARY).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::DIM));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Calculate visible lines
    let visible_height = inner.height as usize;
    let total_thoughts = app.thoughts.len();

    if total_thoughts == 0 {
        let empty = Paragraph::new(Line::from(Span::styled(
            "Waiting for thoughts...",
            Style::default().fg(colors::DIM).italic(),
        )));
        frame.render_widget(empty, inner);
        return;
    }

    // Determine which thoughts to show
    let start_idx = if app.stream_paused {
        total_thoughts.saturating_sub(visible_height + app.scroll_offset)
    } else {
        total_thoughts.saturating_sub(visible_height)
    };

    let end_idx = if app.stream_paused {
        total_thoughts.saturating_sub(app.scroll_offset)
    } else {
        total_thoughts
    };

    let lines: Vec<Line> = app
        .thoughts
        .iter()
        .skip(start_idx)
        .take(end_idx - start_idx)
        .map(|thought| {
            let age = thought.timestamp.elapsed();
            let age_str = format!("{:02}:{:02}", age.as_secs() / 60, age.as_secs() % 60);

            // Salience bar (8 characters)
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let filled = (thought.salience.clamp(0.0, 1.0) * 8.0).round() as usize;
            let bar: String = "█".repeat(filled) + &"░".repeat(8 - filled);

            let salience_color = colors::salience_color(thought.salience);

            let status_color = match thought.status {
                ThoughtStatus::Processing => colors::DIM,
                ThoughtStatus::Salient => colors::PRIMARY,
                ThoughtStatus::MemoryWrite => colors::SECONDARY,
                ThoughtStatus::Anchored => colors::SUCCESS,
                ThoughtStatus::Dismissed => colors::DIM,
                ThoughtStatus::Unconscious => colors::SECONDARY, // Purple/magenta for sinking
                ThoughtStatus::Consolidated => colors::SUCCESS,  // Green for rising
            };

            Line::from(vec![
                Span::styled(format!("{age_str} │ "), Style::default().fg(colors::DIM)),
                Span::styled(bar, Style::default().fg(salience_color)),
                Span::styled(
                    format!(" {:.2} │ ", thought.salience),
                    Style::default().fg(colors::DIM),
                ),
                Span::styled(
                    format!("{:12} │ ", thought.window),
                    Style::default().fg(colors::FOREGROUND),
                ),
                Span::styled(thought.status.as_str(), Style::default().fg(status_color)),
            ])
        })
        .collect();

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });

    frame.render_widget(paragraph, inner);
}
