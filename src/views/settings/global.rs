use iced::{pick_list, Align, Column, Command, Element, PickList, Row, Text};
use typetest_themes::{ApplicationTheme, Theme};

use crate::views::Action;

use super::SettingsMessage;

/// Represents the state for the global settings view.
#[derive(Debug)]
pub struct GlobalSettingsState {
    current_theme: Theme,
    theme_pick_list: pick_list::State<Theme>,
}

#[derive(Clone, Debug)]
pub enum GlobalSettingsMessage {
    ThemeChanged(Theme),
    Action(Action),
}

impl From<GlobalSettingsMessage> for SettingsMessage {
    #[inline]
    fn from(message: GlobalSettingsMessage) -> Self {
        if let GlobalSettingsMessage::Action(a) = message {
            SettingsMessage::Action(a)
        } else {
            SettingsMessage::GlobalSettings(message)
        }
    }
}

impl GlobalSettingsState {
    pub fn new() -> GlobalSettingsState {
        GlobalSettingsState {
            current_theme: Theme::default(),
            theme_pick_list: pick_list::State::default(),
        }
    }

    /// Updates the global settings widget.
    pub fn update(&mut self, message: GlobalSettingsMessage) -> Command<GlobalSettingsMessage> {
        match message {
            GlobalSettingsMessage::ThemeChanged(t) => {
                self.current_theme = t;
                return Command::perform(async move { t }, |t| {
                    GlobalSettingsMessage::Action(Action::ChangeTheme(t))
                });
            }
            _ => {}
        }

        Command::none()
    }

    /// Builds the global settings widget.
    pub fn view(&mut self, theme: &Box<dyn ApplicationTheme>) -> Element<GlobalSettingsMessage> {
        let title = Text::new("Global Settings").size(28);

        let theme_label = Text::new("Theme:");
        let theme_pick_list = PickList::new(
            &mut self.theme_pick_list,
            &Theme::ALL_THEMES[..],
            Some(self.current_theme),
            GlobalSettingsMessage::ThemeChanged,
        )
        .style(theme);

        let theme_selector = Row::new()
            .align_items(Align::Center)
            .spacing(10)
            .push(theme_label)
            .push(theme_pick_list);

        Column::new()
            .spacing(10)
            .push(title)
            .push(theme_selector)
            .into()
    }
}
