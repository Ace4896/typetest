use iced::{
    button, pick_list, scrollable, Align, Button, Column, Command, Element, HorizontalAlignment,
    Length, PickList, Row, Scrollable, Text,
};

use crate::{
    theme::{self, Theme},
    AppMessage, Page,
};

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

pub struct ThemeState {
    pub current_theme: Theme,

    theme_pick_list: pick_list::State<Theme>,
}

impl ThemeState {
    pub fn new() -> ThemeState {
        ThemeState {
            current_theme: Theme::DefaultDark,

            theme_pick_list: pick_list::State::default(),
        }
    }

    fn theme_selector(&mut self) -> Element<AppMessage> {
        let label = Text::new("Theme:");

        let selector = PickList::new(
            &mut self.theme_pick_list,
            &theme::ALL_THEMES[..],
            Some(self.current_theme),
            SettingsMessage::theme_changed,
        )
        .style(self.current_theme);

        Row::new()
            .align_items(Align::Center)
            .spacing(10)
            .push(label)
            .push(selector)
            .into()
    }
}

pub struct RandomGeneratorState {}

impl RandomGeneratorState {
    pub fn new() -> RandomGeneratorState {
        RandomGeneratorState {}
    }

    fn random_generator_settings(&mut self) -> Element<AppMessage> {
        Text::new("zzz").into()
    }
}

pub struct SettingsState {
    // Widget States
    theme_state: ThemeState,
    random_generator_state: RandomGeneratorState,

    scroll_state: scrollable::State,
    back_button: button::State,
}

impl SettingsState {
    pub fn new() -> SettingsState {
        SettingsState {
            theme_state: ThemeState::new(),
            random_generator_state: RandomGeneratorState::new(),

            scroll_state: scrollable::State::new(),
            back_button: button::State::new(),
        }
    }

    pub fn current_theme(&self) -> Theme {
        self.theme_state.current_theme
    }

    pub fn update(&mut self, message: SettingsMessage) -> Command<AppMessage> {
        match message {
            SettingsMessage::ThemeChanged(t) => self.theme_state.current_theme = t,
        }

        Command::none()
    }

    pub fn view(&mut self) -> Element<AppMessage> {
        let current_theme = self.current_theme();
        let title = Text::new("Settings").size(32);

        let back_button = Button::new(
            &mut self.back_button,
            Text::new("Back").horizontal_alignment(HorizontalAlignment::Center),
        )
        .min_width(100)
        .style(current_theme)
        .on_press(AppMessage::Navigate(Page::TypingTest));

        let main_content = Scrollable::new(&mut self.scroll_state)
            .align_items(Align::Center)
            .height(Length::Fill)
            .width(Length::Fill)
            .style(current_theme)
            .push(self.theme_state.theme_selector())
            .push(self.random_generator_state.random_generator_settings());

        Column::new()
            .align_items(Align::Center)
            .spacing(10)
            .max_height(500)
            .push(title)
            .push(main_content)
            .push(back_button)
            .into()
    }
}
