mod global;
mod random_generator;

use iced::{
    button, scrollable, Align, Button, Column, HorizontalAlignment, Length, Scrollable, Text,
};
use typetest_themes::ApplicationTheme;

use self::{
    global::{GlobalSettingsMessage, GlobalSettingsState},
    random_generator::{RandomGeneratorMessage, RandomGeneratorState},
};

use super::{Action, View};

/// Represents the state for the settings view.
#[derive(Debug)]
pub struct SettingsState {
    global_settings: GlobalSettingsState,
    random_generator: RandomGeneratorState,

    // Other Widgets
    scroll: scrollable::State,
    back_button: button::State,
}

/// Represents the messages used by the settings view.
#[derive(Clone, Debug)]
pub enum SettingsMessage {
    GlobalSettings(GlobalSettingsMessage),
    RandomGenerator(RandomGeneratorMessage),
    Action(Action),
}

impl SettingsState {
    pub fn new() -> Self {
        Self {
            global_settings: GlobalSettingsState::new(),
            random_generator: RandomGeneratorState::new(),

            scroll: scrollable::State::new(),
            back_button: button::State::new(),
        }
    }

    pub fn update(&mut self, message: SettingsMessage) -> iced::Command<SettingsMessage> {
        match message {
            SettingsMessage::GlobalSettings(m) => {
                self.global_settings.update(m).map(SettingsMessage::from)
            }
            SettingsMessage::RandomGenerator(m) => {
                self.random_generator.update(m).map(SettingsMessage::from)
            }
            _ => iced::Command::none(),
        }
    }

    pub fn view(&mut self, theme: &Box<dyn ApplicationTheme>) -> iced::Element<SettingsMessage> {
        let back_button = Button::new(
            &mut self.back_button,
            Text::new("Back").horizontal_alignment(HorizontalAlignment::Center),
        )
        .min_width(100)
        .style(theme)
        .on_press(SettingsMessage::Action(Action::ChangeView(
            View::TypingTest,
        )));

        let main_content = Scrollable::new(&mut self.scroll)
            .align_items(Align::Start)
            .spacing(20)
            .height(Length::Fill)
            .width(Length::Fill)
            .style(theme)
            .push(self.global_settings.view(theme).map(SettingsMessage::from))
            .push(self.random_generator.view(theme).map(SettingsMessage::from));

        Column::new()
            .align_items(Align::Center)
            .spacing(10)
            .max_height(500)
            .max_width(400)
            .push(main_content)
            .push(back_button)
            .into()
    }
}
