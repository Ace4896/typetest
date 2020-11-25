use iced::Color;

pub mod default;

#[derive(Copy, Clone, Debug)]
pub enum Themes {
    DefaultLight,
}

pub struct Theme {
    pub text: TextPalette,
}

pub struct TextPalette {
    pub default: Color,
    pub correct: Color,
    pub incorrect: Color,
}

pub const fn get_theme(theme: Themes) -> &'static Theme {
    match theme {
        Themes::DefaultLight => &default::DEFAULT_LIGHT,
    }
}
