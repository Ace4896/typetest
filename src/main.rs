#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use iced::{Align, Application, Column, Container, Length, Text};
use log::{error, LevelFilter};
use typetest_themes::ApplicationTheme;
use views::{
    results::{ResultsMessage, ResultsState},
    settings::{SettingsMessage, SettingsState},
    typing_test::{TypingTestMessage, TypingTestState},
    Action, View,
};

mod config;
mod views;
mod widgets;

/// Top-level Iced application.
pub struct App {
    current_view: View,
    current_theme: Box<dyn ApplicationTheme>,

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

fn main() -> anyhow::Result<()> {
    #[cfg(not(target_arch = "wasm32"))]
    env_logger::builder().filter_level(LevelFilter::Info).init();

    match config::load_config() {
        Ok(app_config) => App::run(iced::Settings::with_flags(app_config))?,
        Err(e) => error!("Could not start TypeTest! {}", e),
    }

    Ok(())
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = AppMessage;
    type Flags = crate::config::Config;

    fn new(config: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let app = App {
            current_view: View::TypingTest,
            current_theme: config.global.theme.into(),

            typing_test_state: TypingTestState::new(&config),
            results_state: ResultsState::new(),
            settings_state: SettingsState::new(&config),
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
            AppMessage::TypingTest(message) => {
                if let TypingTestMessage::Action(action) = &message {
                    self.handle_action(action);
                }

                self.typing_test_state
                    .update(message)
                    .map(AppMessage::TypingTest)
            }
            AppMessage::Results(message) => {
                if let ResultsMessage::Action(action) = &message {
                    self.handle_action(action);
                }

                self.results_state.update(message).map(AppMessage::Results)
            }
            AppMessage::Settings(message) => {
                if let SettingsMessage::Action(action) = &message {
                    self.handle_action(action);
                }

                self.settings_state
                    .update(message)
                    .map(AppMessage::Settings)
            }
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let title = Text::new("TypeTest").size(40);
        let inner_view: iced::Element<_> = match self.current_view {
            View::TypingTest => self
                .typing_test_state
                .view(&self.current_theme)
                .map(AppMessage::TypingTest),
            View::Results => self
                .results_state
                .view(&self.current_theme)
                .map(AppMessage::Results),
            View::Settings => self
                .settings_state
                .view(&self.current_theme)
                .map(AppMessage::Settings),
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

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        if let View::TypingTest = self.current_view {
            self.typing_test_state
                .subscription()
                .map(AppMessage::TypingTest)
        } else {
            iced::Subscription::none()
        }
    }
}

impl App {
    /// Handles any application-wide actions signalled by the views.
    fn handle_action(&mut self, action: &Action) {
        match action {
            Action::ChangeTheme(theme) => self.current_theme = (*theme).into(),
            Action::ChangeView(view) => self.current_view = *view,
            Action::ChangeTimeLength(time) => self.typing_test_state.update_time_length(*time),

            Action::DisplayResults(stats) => {
                self.results_state.update_stats(stats.clone());
                self.current_view = View::Results;
            }

            Action::RetryTest => {
                self.typing_test_state.reset_test_state(false);
                self.current_view = View::TypingTest;
            }

            Action::NextTest => {
                self.typing_test_state.reset_test_state(true);
                self.current_view = View::TypingTest;
            }
        }
    }
}
