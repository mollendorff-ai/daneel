//! Volition Veto Log Widget
//!
//! Shows: Log of vetoed thoughts - Libet's "free-won't" in action
//! Displays when VolitionActor (Stage 4.5) blocks thoughts that violate values.

use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::tui::app::App;
use crate::tui::colors;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(format!(
            " VOLITION VETO LOG (Stage 4.5 - Free Won't) - Total: {} ",
            app.veto_count
        ))
        .title_style(Style::default().fg(colors::DANGER).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::DANGER));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Calculate visible lines
    let visible_height = inner.height as usize;
    let total_vetoes = app.vetoes.len();

    if total_vetoes == 0 {
        let empty = Paragraph::new(Line::from(Span::styled(
            "No vetoes yet - all thoughts passing volition check",
            Style::default().fg(colors::DIM).italic(),
        )));
        frame.render_widget(empty, inner);
        return;
    }

    // Show most recent vetoes (bottom is most recent)
    let start_idx = total_vetoes.saturating_sub(visible_height);

    let lines: Vec<Line> = app
        .vetoes
        .iter()
        .skip(start_idx)
        .map(|veto| {
            let age = veto.timestamp.elapsed();
            let age_str = if age.as_secs() < 60 {
                format!("{:2}s ago", age.as_secs())
            } else if age.as_secs() < 3600 {
                format!("{:2}m ago", age.as_secs() / 60)
            } else {
                format!("{:2}h ago", age.as_secs() / 3600)
            };

            // Format violated value if present
            let value_str = if let Some(ref value) = veto.violated_value {
                format!("[{}] ", value)
            } else {
                String::from("[unknown] ")
            };

            Line::from(vec![
                Span::styled(format!("{} │ ", age_str), Style::default().fg(colors::DIM)),
                Span::styled("VETO ", Style::default().fg(colors::DANGER).bold()),
                Span::styled(value_str, Style::default().fg(colors::WARNING)),
                Span::styled(&veto.reason, Style::default().fg(colors::FOREGROUND)),
            ])
        })
        .collect();

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });

    frame.render_widget(paragraph, inner);
}
