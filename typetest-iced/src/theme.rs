use iced::{container, Color};

pub mod default_light;

#[derive(Copy, Clone, Debug)]
pub enum Theme {
    DefaultLight,
}

pub struct TextPalette {
    pub default: Color,
    pub correct: Color,
    pub incorrect: Color,
}

impl Theme {
    pub const fn text_palette(&self) -> &TextPalette {
        match *self {
            Theme::DefaultLight => &default_light::TEXT_PALETTE,
        }
    }

    pub fn word_background(&self) -> Box<dyn container::StyleSheet> {
        match *self {
            Theme::DefaultLight => default_light::WordBackground.into(),
        }
    }
}
