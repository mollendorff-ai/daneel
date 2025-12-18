//! DANEEL TUI Color Scheme
//!
//! Brand colors for the observable mind interface.

use ratatui::style::Color;

/// Deep blue-black background
pub const BACKGROUND: Color = Color::Rgb(15, 15, 25);

/// Soft white foreground
pub const FOREGROUND: Color = Color::Rgb(200, 200, 210);

/// Teal - DANEEL brand primary
pub const PRIMARY: Color = Color::Rgb(0, 180, 140);

/// Purple accent
pub const SECONDARY: Color = Color::Rgb(140, 100, 220);

/// Green - laws OK, positive status
pub const SUCCESS: Color = Color::Rgb(80, 200, 120);

/// Yellow - warning
pub const WARNING: Color = Color::Rgb(220, 180, 60);

/// Red - violation, danger
pub const DANGER: Color = Color::Rgb(220, 80, 80);

/// Muted text for less important info
pub const DIM: Color = Color::Rgb(100, 100, 110);

/// Attention highlight
pub const HIGHLIGHT: Color = Color::Rgb(255, 220, 100);

/// Salience color gradient (low to high)
pub fn salience_color(salience: f32) -> Color {
    if salience < 0.3 {
        DIM
    } else if salience < 0.7 {
        FOREGROUND
    } else if salience < 0.9 {
        PRIMARY
    } else {
        HIGHLIGHT
    }
}
