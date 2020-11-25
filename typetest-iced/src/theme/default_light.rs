use iced::{button, container, Background, Color, Vector};

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

pub struct Button;
impl button::StyleSheet for Button {
    // Button stylesheet adapted from the styling example
    // https://github.com/hecrj/iced/blob/master/examples/styling
    fn active(&self) -> button::Style {
        button::Style {
            background: Color::from_rgb(0.11, 0.42, 0.87).into(),
            border_radius: 5.0,
            shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: Color::WHITE,
            shadow_offset: Vector::new(1.0, 2.0),
            ..self.active()
        }
    }
}
