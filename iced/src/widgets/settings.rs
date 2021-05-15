use iced::{
    button, scrollable, Align, Button, Column, Command, Element, HorizontalAlignment, Length,
    Scrollable, Text,
};

use typetest_iced_themes::theme::Theme;

use crate::{AppMessage, GlobalMessage, Page};

use self::{
    global::{GlobalSettingsMessage, GlobalSettingsState},
    random_generator::{RandomGeneratorMessage, RandomGeneratorState},
};

pub mod global;
pub mod random_generator;

/// Represents a message specific to the settings widget.
#[derive(Clone, Debug)]
pub enum SettingsMessage {
    GlobalSettings(GlobalSettingsMessage),
    RandomGeneratorSettings(RandomGeneratorMessage),
}

impl From<SettingsMessage> for AppMessage {
    #[inline]
    fn from(message: SettingsMessage) -> AppMessage {
        AppMessage::Settings(message)
    }
}

/// Top-level setting state.
pub struct SettingsState {
    // Widget States
    global_settings_state: GlobalSettingsState,
    random_generator_state: RandomGeneratorState,

    scroll_state: scrollable::State,
    back_button: button::State,
}

impl SettingsState {
    pub fn new() -> SettingsState {
        SettingsState {
            global_settings_state: GlobalSettingsState::new(),
            random_generator_state: RandomGeneratorState::new(),

            scroll_state: scrollable::State::new(),
            back_button: button::State::new(),
        }
    }

    /// Gets the length of a random generator test in seconds.
    pub fn random_time_length(&self) -> u64 {
        self.random_generator_state.time_length_seconds
    }

    /// Handles any global updates which may be related to this widget.
    pub fn global_update(&mut self, message: GlobalMessage) -> Command<AppMessage> {
        match message {
            GlobalMessage::TimeLengthChanged(s) => {
                self.random_generator_state.time_length_seconds = s
            }
        }

        Command::none()
    }

    /// Handles any updates specific to this widget.
    pub fn update(&mut self, message: SettingsMessage) -> Command<SettingsMessage> {
        match message {
            SettingsMessage::GlobalSettings(m) => self
                .global_settings_state
                .update(m)
                .map(SettingsMessage::from),
            SettingsMessage::RandomGeneratorSettings(m) => self
                .random_generator_state
                .update(m)
                .map(SettingsMessage::from),
        }
    }

    /// Builds the top-level view for all settings.
    pub fn view<'a>(&'a mut self, theme: &'a Box<dyn Theme>) -> Element<'a, AppMessage> {
        let back_button = Button::new(
            &mut self.back_button,
            Text::new("Back").horizontal_alignment(HorizontalAlignment::Center),
        )
        .min_width(100)
        .style(theme)
        .on_press(AppMessage::Navigate(Page::TypingTest));

        let main_content: Element<SettingsMessage> = Scrollable::new(&mut self.scroll_state)
            .align_items(Align::Start)
            .spacing(20)
            .height(Length::Fill)
            .width(Length::Fill)
            .style(theme)
            .push(
                self.global_settings_state
                    .view(theme)
                    .map(SettingsMessage::from),
            )
            .push(
                self.random_generator_state
                    .view(theme)
                    .map(SettingsMessage::from),
            )
            .into();

        Column::new()
            .align_items(Align::Center)
            .spacing(10)
            .max_height(500)
            .max_width(400)
            .push(main_content.map(AppMessage::from))
            .push(back_button)
            .into()
    }
}
