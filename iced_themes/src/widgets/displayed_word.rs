use iced::{container, Color};

/// A set of rules that dictate the style of a displayed word.
pub trait StyleSheet {
    fn word_palette(&self) -> WordPalette;
    fn word_background(&self) -> Box<dyn container::StyleSheet>;
}

/// Represents the different colours for each displayed word.
pub struct WordPalette {
    pub default: Color,
    pub correct: Color,
    pub incorrect: Color,
}

impl<T> From<T> for Box<dyn StyleSheet>
where
    T: 'static + StyleSheet,
{
    fn from(style: T) -> Self {
        Box::new(style)
    }
}
