use std::fmt::Display;

use iced_core::Font;
use iced_style::{button, container, pick_list, radio, scrollable, text_input};

pub mod themes;

/// The Noto Sans Mono font.
const NOTO_SANS_MONO: Font = Font::External {
    name: "Noto Sans Mono",
    bytes: include_bytes!("../../fonts/NotoSansMono/NotoSansMono-Regular.ttf"),
};

/// Represents the available themes in the application.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    DefaultDark,
    DefaultLight,
}

impl Theme {
    /// An array of all the themes in this application.
    pub const ALL_THEMES: [Theme; 2] = [Theme::DefaultDark, Theme::DefaultLight];

    /// Gets the monospace font to use for the word display.
    pub const fn monospace_font() -> Font {
        NOTO_SANS_MONO
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::DefaultDark
    }
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Theme::DefaultDark => f.write_str("Default (Dark)"),
            Theme::DefaultLight => f.write_str("Default (Light)"),
        }
    }
}

/// Trait that needs to be implemented for any themes in the application.
/// Once implemented, the theme can be used for any widgets used by the GUI.
pub trait ApplicationTheme {
    // Built-In Widgets
    fn button(&self) -> Box<dyn button::StyleSheet>;
    fn container(&self) -> Box<dyn container::StyleSheet>;
    fn pick_list(&self) -> Box<dyn pick_list::StyleSheet>;
    fn radio(&self) -> Box<dyn radio::StyleSheet>;
    fn scrollable(&self) -> Box<dyn scrollable::StyleSheet>;
    fn text_input(&self) -> Box<dyn text_input::StyleSheet>;
}

impl From<Theme> for Box<dyn ApplicationTheme> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::DefaultDark => Box::new(themes::default_dark::DefaultDark),
            Theme::DefaultLight => Box::new(themes::default_light::DefaultLight),
        }
    }
}

impl From<&Box<dyn ApplicationTheme>> for Box<dyn button::StyleSheet> {
    #[inline]
    fn from(theme: &Box<dyn ApplicationTheme>) -> Self {
        theme.button()
    }
}

impl From<&Box<dyn ApplicationTheme>> for Box<dyn container::StyleSheet> {
    #[inline]
    fn from(theme: &Box<dyn ApplicationTheme>) -> Self {
        theme.container()
    }
}

impl From<&Box<dyn ApplicationTheme>> for Box<dyn pick_list::StyleSheet> {
    #[inline]
    fn from(theme: &Box<dyn ApplicationTheme>) -> Self {
        theme.pick_list()
    }
}

impl From<&Box<dyn ApplicationTheme>> for Box<dyn radio::StyleSheet> {
    #[inline]
    fn from(theme: &Box<dyn ApplicationTheme>) -> Self {
        theme.radio()
    }
}

impl From<&Box<dyn ApplicationTheme>> for Box<dyn scrollable::StyleSheet> {
    #[inline]
    fn from(theme: &Box<dyn ApplicationTheme>) -> Self {
        theme.scrollable()
    }
}

impl From<&Box<dyn ApplicationTheme>> for Box<dyn text_input::StyleSheet> {
    #[inline]
    fn from(theme: &Box<dyn ApplicationTheme>) -> Self {
        theme.text_input()
    }
}
