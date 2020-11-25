use iced::Color;

use super::{ColorPalette, TypeTestTheme};

/// The default theme for the application.
pub struct DefaultTheme {
    color_palette: ColorPalette,
}

impl DefaultTheme {
    pub const fn new() -> DefaultTheme {
        DefaultTheme {
            color_palette: ColorPalette {
                text_default: Color::BLACK,
                correct: Color::from_rgb(0.0, 0.75, 0.0),
                incorrect: Color::from_rgb(0.75, 0.0, 0.0),
            },
        }
    }
}

impl TypeTestTheme for DefaultTheme {
    fn color_palette(&self) -> &ColorPalette {
        &self.color_palette
    }
}
