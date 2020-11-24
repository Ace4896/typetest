#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod theme;

mod widgets;
use theme::{default::DefaultTheme, TypeTestTheme};
use widgets::typing_test::{TypingTestMessage, TypingTestState};

use iced::{
    executor, Align, Application, Color, Column, Command, Container, Element, Length, Settings,
    Subscription, Text,
};

/// Represents the different pages in the application.
#[derive(Copy, Clone, Debug)]
enum Page {
    TypingTest,
    Settings,
}

/// Top-level enum for the messages that can be sent in this application.
#[derive(Clone, Debug)]
enum AppMessage {
    Navigate(Page),
    TypingTest(TypingTestMessage),
}

/// Represents the main state of the application.
struct TypeTestApp {
    current_page: Page,
    current_theme: Box<dyn TypeTestTheme>,
    typing_test_state: TypingTestState,
}

impl TypeTestApp {
    fn new() -> TypeTestApp {
        TypeTestApp {
            current_page: Page::TypingTest,
            current_theme: Box::new(DefaultTheme::new()),
            typing_test_state: TypingTestState::new(),
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

        let inner_view = match self.current_page {
            Page::TypingTest => self.typing_test_state.view(&self.current_theme),
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
