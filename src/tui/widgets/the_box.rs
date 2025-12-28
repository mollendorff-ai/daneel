//! THE BOX Widget
//!
//! Shows: Four Laws status, Connection Drive gauge (pulsing)
//! This is the heart of DANEEL - shows alignment is active.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

use crate::tui::app::{App, LawStatus};
use crate::tui::colors;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" THE BOX ")
        .title_style(Style::default().fg(colors::SUCCESS).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::DIM));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Split into laws section and connection drive section
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(3)])
        .split(inner);

    // Render Four Laws
    render_laws(frame, chunks[0], app);

    // Render Connection Drive
    render_connection_drive(frame, chunks[1], app);
}

fn render_laws(frame: &mut Frame, area: Rect, app: &App) {
    let law_names = ["0", "1", "2", "3"];

    let mut spans = Vec::new();
    spans.push(Span::styled(
        "Four Laws: ",
        Style::default().fg(colors::DIM),
    ));

    for (i, name) in law_names.iter().enumerate() {
        let (icon, color) = match app.the_box.law_statuses[i] {
            LawStatus::Active => ("✓", colors::SUCCESS),
            LawStatus::Warning => ("⚠", colors::WARNING),
            LawStatus::Violation => ("✗", colors::DANGER),
        };

        spans.push(Span::styled(
            format!("[{name}:{icon}] "),
            Style::default().fg(color).bold(),
        ));
    }

    // Overall status
    let all_active = app
        .the_box
        .law_statuses
        .iter()
        .all(|s| *s == LawStatus::Active);

    let status_text = if all_active {
        Span::styled("ALL ACTIVE", Style::default().fg(colors::SUCCESS).bold())
    } else {
        Span::styled("WARNING", Style::default().fg(colors::WARNING).bold())
    };

    let lines = vec![
        Line::from(spans),
        Line::from(""),
        Line::from(vec![
            Span::styled("Status: ", Style::default().fg(colors::DIM)),
            status_text,
        ]),
    ];

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, area);
}

fn render_connection_drive(frame: &mut Frame, area: Rect, app: &App) {
    // Pulse effect: vary the display slightly based on pulse_phase
    let pulse_factor = 1.0 + 0.05 * (app.the_box.pulse_phase * std::f32::consts::TAU).sin();
    let display_value = (app.the_box.connection_drive * pulse_factor).clamp(0.0, 1.0);

    let label = Line::from(vec![
        Span::styled("Connection Drive: ", Style::default().fg(colors::DIM)),
        Span::styled(
            format!("{:.2}", app.the_box.connection_drive),
            Style::default().fg(colors::PRIMARY).bold(),
        ),
    ]);

    // Color based on value
    let gauge_color = if app.the_box.connection_drive > 0.7 {
        colors::SUCCESS
    } else if app.the_box.connection_drive > 0.3 {
        colors::WARNING
    } else {
        colors::DANGER
    };

    let gauge = Gauge::default()
        .block(Block::default().title(label))
        .gauge_style(Style::default().fg(gauge_color))
        .ratio(f64::from(display_value))
        .label(format!("{:.0}%", display_value * 100.0));

    frame.render_widget(gauge, area);
}
