#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use iced::{Application, Command, Element, widget};

/// Top-level Iced application.
pub struct App {
}

/// Top-level message for the application.
#[derive(Clone, Debug)]
pub enum AppMessage {
}

fn main() -> Result<(), iced::Error> {
    App::run(iced::Settings::default())
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = AppMessage;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {},
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("TypeTest")
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::Dark
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        widget::text("TypeTest").into()
    }
}
