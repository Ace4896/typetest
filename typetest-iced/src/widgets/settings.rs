use iced::{
    button, scrollable, Align, Button, Column, Command, Container, Element, HorizontalAlignment,
    Length, Scrollable, Text,
};

use crate::{theme::Theme, AppMessage, Page};

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

pub struct SettingsState {
    pub current_theme: Theme,

    // Widget States
    scroll_state: scrollable::State,
    back_button: button::State,
}

impl SettingsState {
    pub fn new() -> SettingsState {
        SettingsState {
            current_theme: Theme::DefaultDark,

            scroll_state: scrollable::State::new(),
            back_button: button::State::new(),
        }
    }

    pub fn update(&mut self, message: SettingsMessage) -> Command<AppMessage> {
        match message {
            SettingsMessage::ThemeChanged(t) => self.current_theme = t,
        }

        Command::none()
    }

    pub fn view(&mut self) -> Element<AppMessage> {
        let title = Text::new("Settings").size(32);

        let back_button = Button::new(
            &mut self.back_button,
            Text::new("Back").horizontal_alignment(HorizontalAlignment::Center),
        )
        .min_width(100)
        .style(self.current_theme)
        .on_press(AppMessage::Navigate(Page::TypingTest));

        let main_content = Scrollable::new(&mut self.scroll_state)
            .align_items(Align::Center)
            .height(Length::Fill)
            .width(Length::Fill)
            .style(self.current_theme)
            .push(Text::new("Main Content"));

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
