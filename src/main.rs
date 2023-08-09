#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use iced::{executor, widget, Application, Command, Element, Settings, Subscription, Theme};

mod fonts;
mod views;
mod widgets;

fn main() -> iced::Result {
    App::run(Settings {
        default_font: fonts::DEFAULT,
        ..Default::default()
    })
}

pub struct App {}

#[derive(Clone, Debug)]
pub enum AppMessage {
    LoadFont(Option<iced::font::Error>),
}

impl Application for App {
    type Executor = executor::Default;
    type Message = AppMessage;
    type Theme = Theme;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            App {},
            crate::fonts::load_command().map(|r| AppMessage::LoadFont(r.err())),
        )
    }

    fn title(&self) -> String {
        String::from("Iced App")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn view(&self) -> Element<Self::Message> {
        widget::text("Hello world!")
            .font(crate::fonts::MONOSPACE)
            .into()
    }
}
