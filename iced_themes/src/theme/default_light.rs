use iced::{container, Background, Color};

use crate::widgets::displayed_word::{self, WordPalette};

const WORD_PALETTE: WordPalette = WordPalette {
    default: Color::BLACK,
    correct: Color::from_rgb(0.0, 0.75, 0.0),
    incorrect: Color::from_rgb(0.75, 0.0, 0.0),
};

/// The default light theme used by iced.
pub struct DefaultLight;
impl super::Theme for DefaultLight {
    fn button(&self) -> Box<dyn iced::button::StyleSheet> {
        Default::default()
    }

    fn container(&self) -> Box<dyn container::StyleSheet> {
        Default::default()
    }

    fn pick_list(&self) -> Box<dyn iced::pick_list::StyleSheet> {
        Default::default()
    }

    fn radio(&self) -> Box<dyn iced::radio::StyleSheet> {
        Default::default()
    }

    fn scrollable(&self) -> Box<dyn iced::scrollable::StyleSheet> {
        Default::default()
    }

    fn text_input(&self) -> Box<dyn iced::text_input::StyleSheet> {
        Default::default()
    }

    fn displayed_word(&self) -> Box<dyn displayed_word::StyleSheet> {
        DisplayedWord.into()
    }
}

pub struct DisplayedWord;
impl displayed_word::StyleSheet for DisplayedWord {
    fn word_palette(&self) -> WordPalette {
        WORD_PALETTE
    }

    fn word_background(&self) -> Box<dyn container::StyleSheet> {
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
