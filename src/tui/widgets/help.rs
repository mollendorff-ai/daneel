//! Help Overlay Widget
//!
//! Shows keyboard controls when user presses '?'

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::tui::colors;

pub fn render(frame: &mut Frame, area: Rect) {
    // Center a box in the middle of the screen
    let popup_area = centered_rect(50, 50, area);

    // Clear the background
    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(" KEYBOARD CONTROLS ")
        .title_style(Style::default().fg(colors::PRIMARY).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::PRIMARY))
        .style(Style::default().bg(colors::BACKGROUND));

    let controls = vec![
        ("q", "Quit"),
        ("p", "Pause/resume thought stream"),
        ("↑/↓", "Scroll when paused"),
        ("?", "Toggle this help"),
        ("Esc", "Close help / resume"),
        ("", ""),
        ("─────", "── LEGEND ──────────────"),
        ("↑MEMORY", "Consolidated to conscious"),
        ("↓UNCON", "Archived to unconscious"),
        ("██████", "Salience bar (higher=more)"),
    ];

    let lines: Vec<Line> = controls
        .into_iter()
        .map(|(key, desc)| {
            Line::from(vec![
                Span::styled(
                    format!("  {key:6}"),
                    Style::default().fg(colors::HIGHLIGHT).bold(),
                ),
                Span::styled(format!("  {desc}"), Style::default().fg(colors::FOREGROUND)),
            ])
        })
        .collect();

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, popup_area);
}

/// Helper to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
