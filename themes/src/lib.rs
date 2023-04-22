use iced_core::{Background, Color, Font};
use iced_style::{container, Theme};
use typetest_core::word_generators::WordStatus;

/// The monospace font to use in the application (Noto Sans Mono).
pub const MONOSPACE_FONT: Font = Font::External {
    name: "Noto Sans Mono",
    bytes: include_bytes!("../fonts/NotoSansMono/NotoSansMono-Regular.ttf"),
};

pub trait TypingTestStyleSheet {
    fn default_text(&self) -> Color;
    fn correct_text(&self) -> Color;
    fn incorrect_text(&self) -> Color;

    fn word_background(&self) -> container::Appearance;

    fn monospace_font(&self) -> Font {
        MONOSPACE_FONT
    }

    fn text_colour(&self, word_status: WordStatus) -> Color {
        match word_status {
            WordStatus::NotTyped => self.default_text(),
            WordStatus::Correct => self.correct_text(),
            WordStatus::Incorrect => self.incorrect_text(),
        }
    }
}

impl TypingTestStyleSheet for Theme {
    fn default_text(&self) -> Color {
        self.palette().text
    }

    fn correct_text(&self) -> Color {
        // TODO: Not sure if this is always readable
        self.extended_palette().success.strong.color
    }

    fn incorrect_text(&self) -> Color {
        // TODO: Not sure if this is always readable
        self.extended_palette().danger.strong.color
    }

    // TODO: Looks like this is meant to return a container stylesheet instead
    fn word_background(&self) -> container::Appearance {
        // Light Palette:
        // background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.25))),
        // border_radius: 2.0,
        // border_width: 0.0,
        // border_color: Color::TRANSPARENT,
        // text_color: None,

        // Dark Palette:
        // background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.75))),
        // border_radius: 2.0,
        // border_width: 0.0,
        // border_color: Color::TRANSPARENT,
        // text_color: None,

        container::Appearance {
            text_color: None,
            background: Some(Background::Color(
                self.extended_palette().background.strong.color,
            )),
            border_radius: 2.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }
}
