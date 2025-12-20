//! Memory Windows Widget
//!
//! Shows: 9 slots visualization (TMI min=3, max=9)
//! Visualizes bounded working memory.

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
        .title(" MEMORY WINDOWS ")
        .title_style(Style::default().fg(colors::PRIMARY).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::DIM));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Build window slots visualization
    let mut slots = Vec::new();
    for (i, window) in app.memory_windows.iter().enumerate() {
        let (icon, color) = if window.active {
            ("██", colors::PRIMARY)
        } else {
            ("░░", colors::DIM)
        };

        slots.push(Span::styled(
            format!("[{}] ", i + 1),
            Style::default().fg(colors::DIM),
        ));
        slots.push(Span::styled(icon, Style::default().fg(color)));
        slots.push(Span::raw("  "));
    }

    let active_count = app.active_window_count();
    let status_color = if active_count >= 7 {
        colors::WARNING
    } else if active_count >= 3 {
        colors::SUCCESS
    } else {
        colors::DANGER
    };

    let total_memories = app.memory_count + app.unconscious_count;

    let lines = vec![
        Line::from(slots),
        Line::from(vec![
            Span::styled("Active: ", Style::default().fg(colors::DIM)),
            Span::styled(
                format!("{}/9", active_count),
                Style::default().fg(status_color).bold(),
            ),
            Span::styled("  │  ", Style::default().fg(colors::DIM)),
            Span::styled("Conscious: ", Style::default().fg(colors::DIM)),
            Span::styled(
                format!("{}", app.memory_count),
                Style::default().fg(colors::SUCCESS),
            ),
            Span::styled("  Unconscious: ", Style::default().fg(colors::DIM)),
            Span::styled(
                format!("{}", app.unconscious_count),
                Style::default().fg(colors::SECONDARY),
            ),
            Span::styled("  Total: ", Style::default().fg(colors::DIM)),
            Span::styled(
                format!("{}", total_memories),
                Style::default().fg(colors::FOREGROUND),
            ),
        ]),
    ];

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}
