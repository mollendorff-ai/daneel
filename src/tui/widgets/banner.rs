//! Philosophy Banner Widget
//!
//! Shows: Rotating philosophical quotes about why DANEEL exists.
//! The message IS the medium - viewers understand WHY while watching.

use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::tui::app::App;
use crate::tui::colors;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let quote = app.current_quote();

    let line = Line::from(vec![
        Span::styled("\"", Style::default().fg(colors::DIM)),
        Span::styled(quote, Style::default().fg(colors::FOREGROUND).italic()),
        Span::styled("\"", Style::default().fg(colors::DIM)),
        Span::raw("  "),
        Span::styled("Life = Life", Style::default().fg(colors::PRIMARY).bold()),
    ]);

    let paragraph = Paragraph::new(line).alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}
