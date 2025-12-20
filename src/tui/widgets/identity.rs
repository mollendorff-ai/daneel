//! Identity Panel Widget
//!
//! Shows: Name, Uptime, Thought count, Thoughts/hr

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
        .title(" IDENTITY ")
        .title_style(Style::default().fg(colors::PRIMARY).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::DIM));

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
            Span::styled("  Rate: ", Style::default().fg(colors::DIM)),
            Span::styled(
                format!("{:.0}/hr", app.thoughts_per_hour),
                Style::default().fg(colors::FOREGROUND),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Memories: ", Style::default().fg(colors::DIM)),
            Span::styled(
                format!("{}", app.memory_count),
                Style::default().fg(colors::SUCCESS),
            ),
            Span::styled(" ↑", Style::default().fg(colors::SUCCESS)),
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
