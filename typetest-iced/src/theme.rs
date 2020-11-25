use iced::Color;

pub mod default;

#[derive(Copy, Clone, Debug)]
pub struct ColorPalette {
    pub text_default: Color,
    pub correct: Color,
    pub incorrect: Color,
}

pub trait TypeTestTheme {
    fn color_palette(&self) -> &ColorPalette;
}
