use iced::{button, Button, HorizontalAlignment, Text};
use typetest_themes::ApplicationTheme;

use super::{Action, View};

/// Represents the state for the typing test view.
#[derive(Debug)]
pub struct TypingTestState {
    settings_button: button::State,
}

/// Represents the messages used by the typing test view.
#[derive(Clone, Debug)]
pub enum TypingTestMessage {
    Action(Action),
}

impl TypingTestState {
    pub fn new() -> Self {
        Self {
            settings_button: button::State::new(),
        }
    }

    pub fn update(&mut self, message: TypingTestMessage) -> iced::Command<TypingTestMessage> {
        match message {
            _ => {}
        }

        iced::Command::none()
    }

    pub fn view(&mut self, theme: &Box<dyn ApplicationTheme>) -> iced::Element<TypingTestMessage> {
        let settings_button = Button::new(
            &mut self.settings_button,
            Text::new("Settings").horizontal_alignment(HorizontalAlignment::Center),
        )
        .on_press(TypingTestMessage::Action(Action::ChangeView(
            View::Settings,
        )))
        .style(theme);

        settings_button.into()
    }

    pub fn subscription(&self) -> iced::Subscription<TypingTestMessage> {
        iced::Subscription::none()
    }
}
