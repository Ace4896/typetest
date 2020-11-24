use iced::Color;

pub(crate) mod default;

#[derive(Copy, Clone, Debug)]
pub(crate) struct ColorPalette {
    pub(crate) text_default: Color,
    pub(crate) correct: Color,
    pub(crate) incorrect: Color,
}

pub(crate) trait TypeTestTheme {
    fn color_palette(&self) -> &ColorPalette;
}
