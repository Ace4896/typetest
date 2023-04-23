use iced_core::{Color, Font};
use iced_style::{container, Theme};
use palette::{FromColor, Hsl, Shade, Srgba};

use typetest_core::word_generators::WordStatus;

/// The monospace font to use in the application (Noto Sans Mono).
pub const MONOSPACE_FONT: Font = Font::External {
    name: "Noto Sans Mono",
    bytes: include_bytes!("../fonts/NotoSansMono/NotoSansMono-Regular.ttf"),
};

pub fn test_word_colour(theme: &Theme, word_status: &WordStatus) -> Color {
    match word_status {
        WordStatus::NotTyped => not_typed_colour(theme),
        WordStatus::Correct => correct_word(theme),
        WordStatus::Incorrect => incorrect_word(theme),
    }
}

pub fn not_typed_colour(theme: &Theme) -> Color {
    theme.palette().text
}

pub fn correct_word(theme: &Theme) -> Color {
    // TODO: Not sure if this is always readable
    theme.extended_palette().success.strong.color
}

pub fn incorrect_word(theme: &Theme) -> Color {
    // TODO: Not sure if this is always readable
    theme.extended_palette().danger.strong.color
}

/// A [`container::StyleSheet`] implementation that indicates which word is currently highlighted.
pub struct WordHighlight;

impl container::StyleSheet for WordHighlight {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        // Use HSL to change the shade of the colour
        // Iced -> SRGBA -> HSL -> SRGBA -> Iced
        let background_colour = Srgba::from(style.palette().background);
        let background_colour_hsl = Hsl::from_color(background_colour);
        let highlight_colour_hsl = background_colour_hsl.darken(0.33);
        let highlight_colour = Srgba::from_color(highlight_colour_hsl);

        container::Appearance {
            text_color: None,
            background: Some(Color::from(highlight_colour).into()),
            border_radius: 2.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }
}

impl From<WordHighlight> for iced_style::theme::Container {
    fn from(value: WordHighlight) -> Self {
        iced_style::theme::Container::Custom(Box::new(value))
    }
}
