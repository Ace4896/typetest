use typetest_themes::{ApplicationTheme, Theme};

use super::Action;

/// Represents the state for the settings view.
#[derive(Debug)]
pub struct SettingsState {
    selected_theme: Theme,
}

/// Represents the messages used by the settings view.
#[derive(Clone, Debug)]
pub enum SettingsMessage {
    ThemeChanged(Theme),
    Action(Action),
}

impl SettingsState {
    pub fn new() -> Self {
        Self {
            selected_theme: Theme::DefaultDark,
        }
    }

    pub fn update(&mut self, message: SettingsMessage) -> iced::Command<SettingsMessage> {
        match message {
            SettingsMessage::ThemeChanged(theme) => {
                self.selected_theme = theme;
                return iced::Command::perform(async move { theme }, |theme| {
                    SettingsMessage::ThemeChanged(theme)
                });
            }
            _ => {}
        }

        iced::Command::none()
    }

    pub fn view(&mut self, _theme: &Box<dyn ApplicationTheme>) -> iced::Element<SettingsMessage> {
        todo!()
    }
}
