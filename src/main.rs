#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use iced::{widget, Application, Command, Element};

use typetest_core::word_generators::WordStatus;
use typetest_themes::{TestWordColour, WordHighlight, MONOSPACE_FONT};

/// Top-level Iced application.
pub struct App {
    app_settings: AppSettings,

    // View-specific state - should be moved once views are setup
    selected_theme: Option<iced::Theme>
}

// Top-level application settings.
pub struct AppSettings {
    current_theme: iced::Theme,
}

/// Top-level message for the application.
#[derive(Clone, Debug)]
pub enum AppMessage {
    ThemeChanged(iced::Theme)
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
            Self {
                app_settings: AppSettings {
                    current_theme: iced::Theme::Dark,
                },

                selected_theme: Some(iced::Theme::Dark),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("TypeTest")
    }

    fn theme(&self) -> Self::Theme {
        self.app_settings.current_theme.clone()
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let theme = self.theme();

        // TODO: It looks like I need a custom theme enum anyway, because pick_list requires the Eq trait to be implemented
        let theme_picklist = widget::pick_list(&[iced::Theme::Light, iced::Theme::Dark], self.selected_theme, AppMessage::ThemeChanged);

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
