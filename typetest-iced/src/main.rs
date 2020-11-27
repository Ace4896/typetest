#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod theme;

mod widgets;
use theme::Theme;

use iced::{
    button, executor, Align, Application, Button, Column, Command, Container, Element,
    HorizontalAlignment, Length, Settings, Subscription, Text,
};

use widgets::typing_test::{TypingTestMessage, TypingTestState};

/// Represents the different pages in the application.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Page {
    TypingTest,
    Settings,
}

/// Top-level enum for the messages that can be sent in this application.
#[derive(Clone, Debug)]
pub enum AppMessage {
    Navigate(Page),
    TypingTest(TypingTestMessage),
}

/// Represents the main state of the application.
pub struct TypeTestApp {
    current_page: Page,
    current_theme: Theme,

    // Widget States
    typing_test_state: TypingTestState,
    settings_button: button::State,
}

impl TypeTestApp {
    fn new() -> TypeTestApp {
        TypeTestApp {
            current_page: Page::TypingTest,
            current_theme: Theme::DefaultDark,

            typing_test_state: TypingTestState::new(),
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

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            AppMessage::Navigate(page) => {
                self.current_page = page;
                Command::none()
            }
            AppMessage::TypingTest(m) => self.typing_test_state.update(m),
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let title = Text::new("TypeTest").size(40);

        let inner_view: Element<_> = match self.current_page {
            Page::TypingTest => {
                let settings_button = Button::new(
                    &mut self.settings_button,
                    Text::new("Settings").horizontal_alignment(HorizontalAlignment::Center),
                )
                .min_width(100)
                .style(self.current_theme)
                .on_press(AppMessage::Navigate(Page::Settings));

                Column::new()
                    .align_items(Align::Center)
                    .spacing(20)
                    .push(self.typing_test_state.view(self.current_theme))
                    .push(settings_button)
                    .into()
            }
            page => Text::new(format!("Unknown Page {:?}", page)).into(),
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
            .style(self.current_theme)
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
