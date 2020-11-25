use iced::Color;

use super::{TextPalette, Theme};

/// The default light theme for the application.
pub const DEFAULT_LIGHT: Theme = Theme {
    text: TextPalette {
        default: Color::BLACK,
        correct: Color::from_rgb(0.0, 0.75, 0.0),
        incorrect: Color::from_rgb(0.75, 0.0, 0.0),
    }
};
