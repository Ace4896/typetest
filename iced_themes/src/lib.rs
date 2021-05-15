use std::fmt::Display;

use iced_core::Font;
use theme::Theme;

pub mod theme;
pub mod widgets;

/// The Noto Sans Mono font.
const NOTO_SANS_MONO: Font = Font::External {
    name: "Noto Sans Mono",
    bytes: include_bytes!("../../fonts/NotoSansMono/NotoSansMono-Regular.ttf"),
};

/// Represents a theme in the application.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AppTheme {
    DefaultDark,
    DefaultLight,
}

impl AppTheme {
    /// An array of all the themes in this application.
    pub const ALL_THEMES: [AppTheme; 2] = [AppTheme::DefaultDark, AppTheme::DefaultLight];

    /// Gets the monospace font to use for the word display.
    pub const fn monospace_font() -> Font {
        NOTO_SANS_MONO
    }
}

impl Default for AppTheme {
    fn default() -> Self {
        AppTheme::DefaultDark
    }
}

impl Display for AppTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            AppTheme::DefaultDark => f.write_str("Default (Dark)"),
            AppTheme::DefaultLight => f.write_str("Default (Light)"),
        }
    }
}

impl From<AppTheme> for Box<dyn Theme> {
    fn from(app_theme: AppTheme) -> Self {
        match app_theme {
            AppTheme::DefaultDark => Box::new(theme::default_dark::DefaultDark),
            AppTheme::DefaultLight => Box::new(theme::default_light::DefaultLight),
        }
    }
}
