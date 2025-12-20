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

/// Emotion color based on Russell's circumplex model
///
/// Maps valence (pleasure/displeasure) and arousal (activation) to color:
/// - Valence → Hue: positive = warm (gold/orange), negative = cool (blue/purple)
/// - Arousal → Saturation: high = vivid, low = muted/gray
///
/// Quadrants:
/// - High arousal + positive valence = EXCITED (bright orange)
/// - High arousal + negative valence = ANGRY (vivid blue)
/// - Low arousal + positive valence = CALM (muted gold)
/// - Low arousal + negative valence = SAD (dim blue)
pub fn emotion_color(valence: f32, arousal: f32) -> Color {
    // Clamp inputs to valid range
    let valence = valence.clamp(-1.0, 1.0);
    let arousal = arousal.clamp(0.0, 1.0);

    // Base color based on valence
    // Positive = warm (orange/gold), Negative = cool (blue/purple), Neutral = white
    let (base_r, base_g, base_b) = if valence > 0.1 {
        // Positive: orange-gold spectrum
        // More positive = more orange
        let intensity = valence; // 0.1 to 1.0
        (
            200 + (55.0 * intensity) as u8,        // R: 200-255
            150 + (70.0 * intensity) as u8,        // G: 150-220
            50 + (50.0 * (1.0 - intensity)) as u8, // B: 50-100 (less blue for more positive)
        )
    } else if valence < -0.1 {
        // Negative: blue-purple spectrum
        // More negative = more blue
        let intensity = -valence; // 0.1 to 1.0
        (
            80 + (60.0 * (1.0 - intensity)) as u8, // R: 80-140 (less red for more negative)
            80 + (40.0 * (1.0 - intensity)) as u8, // G: 80-120
            180 + (75.0 * intensity) as u8,        // B: 180-255
        )
    } else {
        // Neutral: white/gray
        (180, 180, 190)
    };

    // Apply arousal as saturation modifier
    // High arousal = keep vibrant colors
    // Low arousal = desaturate toward gray
    let gray = 140u8; // Target gray for zero arousal
    let saturation = arousal; // 0.0 to 1.0

    let r = (gray as f32 + (base_r as f32 - gray as f32) * saturation) as u8;
    let g = (gray as f32 + (base_g as f32 - gray as f32) * saturation) as u8;
    let b = (gray as f32 + (base_b as f32 - gray as f32) * saturation) as u8;

    Color::Rgb(r, g, b)
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
            assert!(
                g > b || (g as i16 - b as i16).abs() < 50,
                "Green should be close to or greater than blue"
            );
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

    // Emotion color tests (Russell's circumplex)

    #[test]
    fn emotion_color_positive_high_arousal_is_warm() {
        // Excited: positive valence + high arousal = bright orange/gold
        if let Color::Rgb(r, g, _b) = emotion_color(0.8, 0.9) {
            assert!(r > 200, "Red should be high for excited state");
            assert!(g > 150, "Green should be moderate for warm color");
        }
    }

    #[test]
    fn emotion_color_negative_high_arousal_is_cool() {
        // Angry: negative valence + high arousal = vivid blue
        if let Color::Rgb(r, _g, b) = emotion_color(-0.8, 0.9) {
            assert!(b > 200, "Blue should be high for angry state");
            assert!(r < 150, "Red should be low for cool color");
        }
    }

    #[test]
    fn emotion_color_neutral_is_grayish() {
        // Neutral: valence near zero
        if let Color::Rgb(r, g, b) = emotion_color(0.0, 0.5) {
            // Should be somewhat gray, values close together
            let max = r.max(g).max(b);
            let min = r.min(g).min(b);
            assert!(
                max - min < 50,
                "Neutral should be grayish (low color spread)"
            );
        }
    }

    #[test]
    fn emotion_color_low_arousal_is_desaturated() {
        // Low arousal should desaturate toward gray
        let low_arousal = emotion_color(0.8, 0.1);
        let high_arousal = emotion_color(0.8, 0.9);

        if let (Color::Rgb(lr, lg, lb), Color::Rgb(hr, hg, hb)) = (low_arousal, high_arousal) {
            // Low arousal should be closer to gray (140)
            let low_spread =
                (lr as i16 - 140).abs() + (lg as i16 - 140).abs() + (lb as i16 - 140).abs();
            let high_spread =
                (hr as i16 - 140).abs() + (hg as i16 - 140).abs() + (hb as i16 - 140).abs();
            assert!(low_spread < high_spread, "Low arousal should be more gray");
        }
    }

    #[test]
    fn emotion_color_clamps_inputs() {
        // Should not panic on out-of-range inputs
        let _ = emotion_color(-2.0, 2.0);
        let _ = emotion_color(5.0, -1.0);
    }
}
