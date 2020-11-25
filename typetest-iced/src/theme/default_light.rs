use iced::{container, Background, Color};

use super::TextPalette;

pub const TEXT_PALETTE: TextPalette = TextPalette {
    default: Color::BLACK,
    correct: Color::from_rgb(0.0, 0.75, 0.0),
    incorrect: Color::from_rgb(0.75, 0.0, 0.0),
};

pub struct WordBackground;

impl container::StyleSheet for WordBackground {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.25))),
            border_radius: 2.0,
            ..Default::default()
        }
    }
}
