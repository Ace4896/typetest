use iced::{
    button, pick_list, scrollable, Align, Button, Column, Command, Element, HorizontalAlignment,
    Length, PickList, Row, Scrollable, Text,
};

use crate::{
    theme::{self, Theme},
    AppMessage, GlobalMessage, Page,
};

/// Represents a message specific to the settings widget.
#[derive(Clone, Debug)]
pub enum SettingsMessage {
    ThemeChanged(Theme),
}

impl From<SettingsMessage> for AppMessage {
    #[inline]
    fn from(message: SettingsMessage) -> AppMessage {
        AppMessage::Settings(message)
    }
}

impl SettingsMessage {
    #[inline]
    fn theme_changed(theme: Theme) -> AppMessage {
        SettingsMessage::ThemeChanged(theme).into()
    }
}

/// Represents the state for any global settings.
pub struct GlobalSettingsState {
    pub current_theme: Theme,

    theme_pick_list: pick_list::State<Theme>,
}

impl GlobalSettingsState {
    pub fn new() -> GlobalSettingsState {
        GlobalSettingsState {
            current_theme: Theme::DefaultDark,

            theme_pick_list: pick_list::State::default(),
        }
    }

    /// Builds the global settings widget.
    fn global_settings(&mut self) -> Element<AppMessage> {
        let title = Text::new("Global Settings").size(28);

        let theme_label = Text::new("Theme:");
        let theme_pick_list = PickList::new(
            &mut self.theme_pick_list,
            &theme::ALL_THEMES[..],
            Some(self.current_theme),
            SettingsMessage::theme_changed,
        )
        .style(self.current_theme);

        let theme_selector = Row::new()
            .align_items(Align::Center)
            .spacing(10)
            .push(theme_label)
            .push(theme_pick_list);

        Column::new()
            .spacing(10)
            .push(title)
            .push(theme_selector)
            .into()
    }
}

pub struct RandomGeneratorState {
    pub time_length_seconds: u64,

    // Widget State
    time_length_pick_list: pick_list::State<u64>,
}

impl RandomGeneratorState {
    pub fn new() -> RandomGeneratorState {
        RandomGeneratorState {
            time_length_seconds: 60,

            time_length_pick_list: pick_list::State::default(),
        }
    }

    // TODO: Other options?
    /// Builds the widget for random generator settings.
    fn random_generator_settings(&mut self, theme: Theme) -> Element<AppMessage> {
        const TIME_OPTIONS: [u64; 5] = [10, 30, 60, 120, 300];

        let title = Text::new("Random Generator Settings").size(28);

        let time_length_label = Text::new("Test Length (Time):");
        let time_length_pick_list = PickList::new(
            &mut self.time_length_pick_list,
            &TIME_OPTIONS[..],
            Some(self.time_length_seconds),
            GlobalMessage::time_length_changed,
        )
        .style(theme);

        let time_length = Row::new()
            .align_items(Align::Center)
            .spacing(10)
            .push(time_length_label)
            .push(time_length_pick_list);

        Column::new()
            .spacing(10)
            .push(title)
            .push(time_length)
            .into()
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

    /// Gets the current theme.
    pub fn current_theme(&self) -> Theme {
        self.global_settings_state.current_theme
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
    pub fn update(&mut self, message: SettingsMessage) -> Command<AppMessage> {
        match message {
            SettingsMessage::ThemeChanged(t) => self.global_settings_state.current_theme = t,
        }

        Command::none()
    }

    /// Builds the top-level view for all settings.
    pub fn view(&mut self) -> Element<AppMessage> {
        let current_theme = self.current_theme();

        let back_button = Button::new(
            &mut self.back_button,
            Text::new("Back").horizontal_alignment(HorizontalAlignment::Center),
        )
        .min_width(100)
        .style(current_theme)
        .on_press(AppMessage::Navigate(Page::TypingTest));

        let main_content = Scrollable::new(&mut self.scroll_state)
            .align_items(Align::Start)
            .spacing(20)
            .height(Length::Fill)
            .width(Length::Fill)
            .style(current_theme)
            .push(self.global_settings_state.global_settings())
            .push(
                self.random_generator_state
                    .random_generator_settings(current_theme),
            );

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
