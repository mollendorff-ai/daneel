//! TUI Layout Composition
//!
//! Composes all widgets into the DANEEL dashboard layout.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders},
    Frame,
};

use crate::tui::app::App;
use crate::tui::colors;
use crate::tui::widgets;

/// Render the complete DANEEL TUI
#[cfg_attr(coverage_nightly, coverage(off))]
pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Main container
    let main_block = Block::default()
        .title(" DANEEL/Timmy - The Observable Mind ")
        .title_style(Style::default().fg(colors::PRIMARY))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::DIM));

    let inner = main_block.inner(area);
    frame.render_widget(main_block, area);

    // Main layout: top panels, thought stream, competition + memory, banner
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10), // Top panels (Identity + THE BOX)
            Constraint::Min(10),    // Thought stream
            Constraint::Length(13), // Stream competition panel
            Constraint::Length(5),  // Memory windows
            Constraint::Length(2),  // Philosophy banner
        ])
        .split(inner);

    // Top row: Identity (left), THE BOX (center), Entropy (right)
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // Identity
            Constraint::Percentage(50), // THE BOX
            Constraint::Percentage(20), // Entropy
        ])
        .split(main_chunks[0]);

    // Middle row: Thought stream (left) and Veto log (right)
    let thought_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70), // Thought stream
            Constraint::Percentage(30), // Veto log
        ])
        .split(main_chunks[1]);

    // Competition + Fractality row
    let analysis_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(55), // Stream competition
            Constraint::Percentage(45), // Pulse fractality
        ])
        .split(main_chunks[2]);

    // Render all widgets
    widgets::identity::render(frame, top_chunks[0], app);
    widgets::the_box::render(frame, top_chunks[1], app);
    widgets::entropy::render(frame, top_chunks[2], app);
    widgets::thoughts::render(frame, thought_chunks[0], app);
    widgets::veto::render(frame, thought_chunks[1], app);
    widgets::competition::render(frame, analysis_chunks[0], app);
    widgets::fractality::render(frame, analysis_chunks[1], app);
    widgets::memory::render(frame, main_chunks[3], app);
    widgets::banner::render(frame, main_chunks[4], app);

    // Help overlay (if active)
    if app.show_help {
        widgets::help::render(frame, area);
    }
}

/// Calculate layout for streaming at 1920x1080
#[allow(dead_code)]
#[cfg_attr(coverage_nightly, coverage(off))]
#[must_use]
pub fn streaming_layout(area: Rect) -> Vec<Rect> {
    // Optimized for readability on mobile/stream viewers
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(12), // Larger top panels
            Constraint::Min(15),    // Thought stream
            Constraint::Length(6),  // Memory windows
            Constraint::Length(3),  // Banner
        ])
        .split(area)
        .to_vec()
}
