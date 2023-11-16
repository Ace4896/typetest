use iced::{widget, Application, Command, Element, Subscription};

mod controls;

pub struct App {}

#[derive(Debug, Clone)]
pub enum Message {}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (App {}, Command::none())
    }

    fn title(&self) -> String {
        "TypeTest".to_string()
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::Dark
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        controls::circle::Circle::new(50.0).into()
    }
}
