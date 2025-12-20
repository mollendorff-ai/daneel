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
        ]),
    ];

    let paragraph = Paragraph::new(lines).block(block);

    frame.render_widget(paragraph, area);
}
