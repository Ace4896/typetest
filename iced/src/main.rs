#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod widgets;

use iced::{
    button, executor, Align, Application, Button, Column, Command, Container, Element,
    HorizontalAlignment, Length, Settings, Subscription, Text,
};

use typetest_iced_themes::{theme::Theme, AppTheme};
use widgets::{
    settings::{SettingsMessage, SettingsState},
    typing_test::{TypingTestMessage, TypingTestState, TypingTestStatus},
};

/// Represents the different pages in the application.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Page {
    TypingTest,
    Settings,
}

/// Top-level enum for the messages that can be sent in this application.
#[derive(Clone, Debug)]
pub enum AppMessage {
    Global(GlobalMessage),
    Navigate(Page),
    TypingTest(TypingTestMessage),
    Settings(SettingsMessage),
}

/// Represents a message that is to be sent to all top-level widgets.
#[derive(Clone, Debug)]
pub enum GlobalMessage {
    TimeLengthChanged(u64),
}

impl From<GlobalMessage> for AppMessage {
    #[inline]
    fn from(g: GlobalMessage) -> AppMessage {
        AppMessage::Global(g)
    }
}

impl GlobalMessage {
    #[inline]
    pub fn time_length_changed(s: u64) -> AppMessage {
        GlobalMessage::TimeLengthChanged(s).into()
    }
}

/// Represents the main state of the application.
pub struct TypeTestApp {
    current_page: Page,
    current_theme: Box<dyn Theme>,

    // Widget States
    typing_test_state: TypingTestState,
    settings_button: button::State,

    settings_state: SettingsState,
}

impl TypeTestApp {
    fn new() -> TypeTestApp {
        let settings_state = SettingsState::new();
        let random_time_length = settings_state.random_time_length();

        TypeTestApp {
            settings_state,

            current_page: Page::TypingTest,
            current_theme: AppTheme::default().into(),

            typing_test_state: TypingTestState::new(random_time_length),
            settings_button: button::State::new(),
        }
    }
}

impl Application for TypeTestApp {
    type Executor = executor::Default;
    type Message = AppMessage;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (TypeTestApp::new(), Command::none())
    }

    fn title(&self) -> String {
        match self.current_page {
            Page::TypingTest => String::from("TypeTest"),
            Page::Settings => String::from("TypeTest - Settings"),
        }
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> Command<Self::Message> {
        match message {
            AppMessage::Global(g) => {
                let typing_test_cmd = self.typing_test_state.global_update(g.clone());
                let settings_cmd = self.settings_state.global_update(g.clone());
                Command::batch(vec![typing_test_cmd, settings_cmd])
            }
            AppMessage::Navigate(page) => {
                self.current_page = page;
                Command::none()
            }
            AppMessage::TypingTest(message) => self.typing_test_state.update(message),
            AppMessage::Settings(message) => {
                // Propagate settings changes to other widgets
                match message {
                    SettingsMessage::ThemeChanged(app_theme) => {
                        self.current_theme = app_theme.into()
                    }
                    SettingsMessage::TimeLengthChanged(t) => {
                        // TODO: Remove global update altogether
                        self.typing_test_state
                            .global_update(GlobalMessage::TimeLengthChanged(t));
                    }
                    _ => {}
                }

                self.settings_state.update(message).map(AppMessage::from)
            }
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let title = Text::new("TypeTest").size(40);
        let inner_view: Element<_> = match self.current_page {
            Page::TypingTest => {
                let settings_button = {
                    let tmp = Button::new(
                        &mut self.settings_button,
                        Text::new("Settings").horizontal_alignment(HorizontalAlignment::Center),
                    )
                    .min_width(100)
                    .style(&self.current_theme);

                    if self.typing_test_state.status != TypingTestStatus::Started {
                        tmp.on_press(AppMessage::Navigate(Page::Settings))
                    } else {
                        tmp
                    }
                };

                Column::new()
                    .align_items(Align::Center)
                    .spacing(20)
                    .push(self.typing_test_state.view(&self.current_theme))
                    .push(settings_button)
                    .into()
            }
            Page::Settings => self
                .settings_state
                .view(&self.current_theme)
                .map(AppMessage::from),
        };

        let inner_container = Container::new(inner_view)
            .padding(10)
            .height(Length::Fill)
            .width(Length::Fill)
            .align_x(Align::Center)
            .align_y(Align::Center);

        let main_view = Column::new()
            .align_items(Align::Center)
            .height(Length::Fill)
            .push(title)
            .push(inner_container);

        Container::new(main_view)
            .padding(20)
            .height(Length::Fill)
            .width(Length::Fill)
            .style(&self.current_theme)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        match self.current_page {
            Page::TypingTest => self.typing_test_state.subscription(),
            _ => Subscription::none(),
        }
    }
}

fn main() -> Result<(), iced::Error> {
    TypeTestApp::run(Settings::default())
}
