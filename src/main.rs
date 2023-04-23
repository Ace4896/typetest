#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use iced::{widget, Application, Command, Element};

use typetest_core::word_generators::WordStatus;
use typetest_themes::{TestWordColour, WordHighlight, MONOSPACE_FONT};

/// Top-level Iced application.
pub struct App {}

/// Top-level message for the application.
#[derive(Clone, Debug)]
pub enum AppMessage {}

fn main() -> Result<(), iced::Error> {
    App::run(iced::Settings::default())
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = AppMessage;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self {}, Command::none())
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
        let theme = self.theme();

        iced::widget::column!(
            widget::text("default")
                .font(MONOSPACE_FONT)
                .style(theme.test_word_colour(&WordStatus::NotTyped)),
            widget::text("correct")
                .font(MONOSPACE_FONT)
                .style(theme.test_word_colour(&WordStatus::Correct)),
            widget::text("incorrect")
                .font(MONOSPACE_FONT)
                .style(theme.test_word_colour(&WordStatus::Incorrect)),
            widget::container(
                widget::text("default with background")
                    .font(MONOSPACE_FONT)
                    .style(theme.test_word_colour(&WordStatus::NotTyped))
            )
            .style(WordHighlight),
            widget::container(
                widget::text("incorrect with background")
                    .font(MONOSPACE_FONT)
                    .style(theme.test_word_colour(&WordStatus::Incorrect))
            )
            .style(WordHighlight)
        )
        .into()
    }
}
