use std::fmt::Display;

use iced::{button, container, pick_list, radio, scrollable, text_input, Color, Font};

pub mod default_dark;
pub mod default_light;

const NOTO_SANS_MONO: Font = Font::External {
    name: "Noto Sans Mono",
    bytes: include_bytes!("../fonts/NotoSansMono/NotoSansMono-Regular.ttf"),
};

/// An array of all the themes in this application.
pub const ALL_THEMES: [Theme; 2] = [Theme::DefaultDark, Theme::DefaultLight];

/// Represents various colors used for text in this application.
pub struct TextPalette {
    pub default: Color,
    pub correct: Color,
    pub incorrect: Color,
}

/// Represents a theme in this application.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Theme {
    DefaultDark,
    DefaultLight,
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Theme::DefaultLight => f.write_str("Default (Light)"),
            Theme::DefaultDark => f.write_str("Default (Dark)"),
        }
    }
}

impl From<Theme> for String {
    fn from(theme: Theme) -> String {
        theme.to_string()
    }
}

impl Theme {
    pub const fn monospace() -> Font {
        NOTO_SANS_MONO
    }

    pub const fn text_palette(&self) -> &TextPalette {
        match *self {
            Theme::DefaultLight => &default_light::TEXT_PALETTE,
            Theme::DefaultDark => &default_dark::TEXT_PALETTE,
        }
    }

    pub fn word_background(&self) -> Box<dyn container::StyleSheet> {
        match *self {
            Theme::DefaultLight => default_light::WordBackground.into(),
            Theme::DefaultDark => default_dark::WordBackground.into(),
        }
    }
}

impl From<Theme> for Box<dyn button::StyleSheet> {
    fn from(t: Theme) -> Self {
        match t {
            Theme::DefaultLight => default_light::Button.into(),
            Theme::DefaultDark => default_dark::Button.into(),
        }
    }
}

impl From<Theme> for Box<dyn container::StyleSheet> {
    fn from(t: Theme) -> Self {
        match t {
            Theme::DefaultLight => Default::default(),
            Theme::DefaultDark => default_dark::Container.into(),
        }
    }
}

impl From<Theme> for Box<dyn pick_list::StyleSheet> {
    fn from(t: Theme) -> Self {
        match t {
            Theme::DefaultLight => Default::default(),
            Theme::DefaultDark => default_dark::PickList.into(),
        }
    }
}

impl From<Theme> for Box<dyn radio::StyleSheet> {
    fn from(t: Theme) -> Self {
        match t {
            Theme::DefaultLight => Default::default(),
            Theme::DefaultDark => default_dark::Radio.into(),
        }
    }
}

impl From<Theme> for Box<dyn scrollable::StyleSheet> {
    fn from(t: Theme) -> Self {
        match t {
            Theme::DefaultLight => Default::default(),
            Theme::DefaultDark => default_dark::Scrollable.into(),
        }
    }
}

impl From<Theme> for Box<dyn text_input::StyleSheet> {
    fn from(t: Theme) -> Self {
        match t {
            Theme::DefaultLight => Default::default(),
            Theme::DefaultDark => default_dark::TextInput.into(),
        }
    }
}
