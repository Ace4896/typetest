use iced::{executor, widget, Application, Command, Element, Subscription, Theme};

pub struct App {}

#[derive(Clone, Debug)]
pub enum AppMessage {}

impl Application for App {
    type Executor = executor::Default;
    type Message = AppMessage;
    type Theme = Theme;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (App {}, Command::none())
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
            .font(iced::Font::MONOSPACE)
            .into()
    }
}
