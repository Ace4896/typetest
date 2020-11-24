#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod widgets;
use widgets::typing_test::{TypingTestMessage, TypingTestState};

use std::{
    mem,
    time::{Duration, Instant},
};

use iced::{
    button, executor, text_input, time, Align, Application, Button, Color, Column, Command,
    Container, HorizontalAlignment, Length, Radio, Row, Settings, Subscription, Text, TextInput,
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
    typing_test_state: TypingTestState,
}

impl TypeTestApp {
    fn new() -> TypeTestApp {
        TypeTestApp {
            current_page: Page::TypingTest,
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
            AppMessage::Navigate(_) => Command::none(), // TODO: Page navigation
            AppMessage::TypingTest(m) => self.typing_test_state.update(m),
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        match self.current_page {
            Page::TypingTest => self.typing_test_state.view(),
            _ => Text::new("Unknown Page").into(),
        }
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
