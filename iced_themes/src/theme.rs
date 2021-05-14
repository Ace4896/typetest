use iced::{button, container, pick_list, radio, scrollable, text_input};

use crate::widgets::displayed_word;

pub mod default_dark;
pub mod default_light;

/// Common trait for application themes.
pub trait Theme {
    // Built-In Widgets
    fn button(&self) -> Box<dyn button::StyleSheet>;
    fn container(&self) -> Box<dyn container::StyleSheet>;
    fn pick_list(&self) -> Box<dyn pick_list::StyleSheet>;
    fn radio(&self) -> Box<dyn radio::StyleSheet>;
    fn scrollable(&self) -> Box<dyn scrollable::StyleSheet>;
    fn text_input(&self) -> Box<dyn text_input::StyleSheet>;

    // Custom Widgets
    fn displayed_word(&self) -> Box<dyn displayed_word::StyleSheet>;
}

impl From<Box<dyn Theme>> for Box<dyn button::StyleSheet> {
    fn from(theme: Box<dyn Theme>) -> Self {
        theme.button()
    }
}

impl From<Box<dyn Theme>> for Box<dyn container::StyleSheet> {
    fn from(theme: Box<dyn Theme>) -> Self {
        theme.container()
    }
}

impl From<Box<dyn Theme>> for Box<dyn pick_list::StyleSheet> {
    fn from(theme: Box<dyn Theme>) -> Self {
        theme.pick_list()
    }
}

impl From<Box<dyn Theme>> for Box<dyn radio::StyleSheet> {
    fn from(theme: Box<dyn Theme>) -> Self {
        theme.radio()
    }
}

impl From<Box<dyn Theme>> for Box<dyn scrollable::StyleSheet> {
    fn from(theme: Box<dyn Theme>) -> Self {
        theme.scrollable()
    }
}

impl From<Box<dyn Theme>> for Box<dyn text_input::StyleSheet> {
    fn from(theme: Box<dyn Theme>) -> Self {
        theme.text_input()
    }
}
