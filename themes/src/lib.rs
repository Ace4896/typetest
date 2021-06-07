pub mod themes;

/// Represents the available themes in the application.
#[derive(Clone, Copy, Debug)]
pub enum Theme {
    DefaultDark,
    DefaultLight,
}

/// Trait that needs to be implemented for any themes in the application.
/// Once implemented, the theme can be used for any widgets used by the GUI.
pub trait ApplicationTheme {}

impl From<Theme> for Box<dyn ApplicationTheme> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::DefaultDark => Box::new(themes::default_dark::DefaultDark),
            Theme::DefaultLight => Box::new(themes::default_light::DefaultLight),
        }
    }
}
