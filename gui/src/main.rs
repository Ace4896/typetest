use iced::Application;
use views::{
    results::{ResultsMessage, ResultsState},
    settings::{SettingsMessage, SettingsState},
    typing_test::{TypingTestMessage, TypingTestState},
    View,
};

mod views;

/// Top-level Iced application.
pub struct App {
    current_view: View,

    typing_test_state: TypingTestState,
    results_state: ResultsState,
    settings_state: SettingsState,
}

/// Top-level message for the application.
#[derive(Clone, Debug)]
pub enum AppMessage {
    // View Messages
    TypingTest(TypingTestMessage),
    Results(ResultsMessage),
    Settings(SettingsMessage),
}

fn main() -> Result<(), iced::Error> {
    App::run(iced::Settings::default())
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = AppMessage;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let app = App {
            current_view: View::TypingTest,

            typing_test_state: TypingTestState::new(),
            results_state: ResultsState::new(),
            settings_state: SettingsState::new(),
        };

        (app, iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("TypeTest")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> iced::Command<Self::Message> {
        match message {
            AppMessage::TypingTest(message) => self
                .typing_test_state
                .update(message)
                .map(AppMessage::TypingTest),
            AppMessage::Results(message) => {
                self.results_state.update(message).map(AppMessage::Results)
            }
            AppMessage::Settings(message) => self
                .settings_state
                .update(message)
                .map(AppMessage::Settings),
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        todo!()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        todo!()
    }
}
