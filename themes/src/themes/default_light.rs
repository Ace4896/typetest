use iced_core::{Background, Color};
use iced_style::container;

use crate::{ApplicationTheme, WordPalette};

const WORD_PALETTE: WordPalette = WordPalette {
    default: Color::BLACK,
    correct: Color::from_rgb(0.0, 0.75, 0.0),
    incorrect: Color::from_rgb(0.75, 0.0, 0.0),
};

#[derive(Clone, Copy, Debug)]
pub struct DefaultLight;

impl ApplicationTheme for DefaultLight {
    fn button(&self) -> Box<dyn iced_style::button::StyleSheet> {
        Default::default()
    }

    fn container(&self) -> Box<dyn iced_style::container::StyleSheet> {
        Default::default()
    }

    fn pick_list(&self) -> Box<dyn iced_style::pick_list::StyleSheet> {
        Default::default()
    }

    fn radio(&self) -> Box<dyn iced_style::radio::StyleSheet> {
        Default::default()
    }

    fn scrollable(&self) -> Box<dyn iced_style::scrollable::StyleSheet> {
        Default::default()
    }

    fn text_input(&self) -> Box<dyn iced_style::text_input::StyleSheet> {
        Default::default()
    }

    fn word_palette(&self) -> WordPalette {
        WORD_PALETTE
    }

    fn word_background(&self) -> Box<dyn iced_style::container::StyleSheet> {
        WordBackground.into()
    }
}

pub struct WordBackground;
impl container::StyleSheet for WordBackground {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.25))),
            border_radius: 2.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            text_color: None,
        }
    }
}
