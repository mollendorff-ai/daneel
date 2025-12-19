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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn salience_color_low() {
        assert_eq!(salience_color(0.0), DIM);
        assert_eq!(salience_color(0.1), DIM);
        assert_eq!(salience_color(0.29), DIM);
    }

    #[test]
    fn salience_color_medium() {
        assert_eq!(salience_color(0.3), FOREGROUND);
        assert_eq!(salience_color(0.5), FOREGROUND);
        assert_eq!(salience_color(0.69), FOREGROUND);
    }

    #[test]
    fn salience_color_high() {
        assert_eq!(salience_color(0.7), PRIMARY);
        assert_eq!(salience_color(0.8), PRIMARY);
        assert_eq!(salience_color(0.89), PRIMARY);
    }

    #[test]
    fn salience_color_critical() {
        assert_eq!(salience_color(0.9), HIGHLIGHT);
        assert_eq!(salience_color(0.95), HIGHLIGHT);
        assert_eq!(salience_color(1.0), HIGHLIGHT);
    }

    #[test]
    fn color_constants_are_rgb() {
        // All our colors should be RGB type
        assert!(matches!(BACKGROUND, Color::Rgb(_, _, _)));
        assert!(matches!(FOREGROUND, Color::Rgb(_, _, _)));
        assert!(matches!(PRIMARY, Color::Rgb(_, _, _)));
        assert!(matches!(SECONDARY, Color::Rgb(_, _, _)));
        assert!(matches!(SUCCESS, Color::Rgb(_, _, _)));
        assert!(matches!(WARNING, Color::Rgb(_, _, _)));
        assert!(matches!(DANGER, Color::Rgb(_, _, _)));
        assert!(matches!(DIM, Color::Rgb(_, _, _)));
        assert!(matches!(HIGHLIGHT, Color::Rgb(_, _, _)));
    }

    #[test]
    fn primary_is_teal() {
        // DANEEL brand color is teal-ish
        if let Color::Rgb(r, g, b) = PRIMARY {
            assert!(g > r, "Green should be dominant in teal");
            assert!(g > b || (g as i16 - b as i16).abs() < 50, "Green should be close to or greater than blue");
        }
    }

    #[test]
    fn danger_is_red() {
        if let Color::Rgb(r, g, b) = DANGER {
            assert!(r > g, "Red should be dominant in danger");
            assert!(r > b, "Red should be dominant in danger");
        }
    }

    #[test]
    fn success_is_green() {
        if let Color::Rgb(r, g, b) = SUCCESS {
            assert!(g > r, "Green should be dominant in success");
            assert!(g > b, "Green should be dominant in success");
        }
    }
}
