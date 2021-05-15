use iced::{pick_list, Align, Column, Command, Element, PickList, Row, Text};

use typetest_iced_themes::{theme::Theme, AppTheme};

use super::SettingsMessage;

/// Represents a message specific to the global settings view.
#[derive(Clone, Debug)]
pub enum GlobalSettingsMessage {
    ThemeChanged(AppTheme),
}

impl From<GlobalSettingsMessage> for SettingsMessage {
    #[inline]
    fn from(m: GlobalSettingsMessage) -> Self {
        match m {
            GlobalSettingsMessage::ThemeChanged(t) => SettingsMessage::ThemeChanged(t),
        }
    }
}

/// Represents the state for any global settings.
pub struct GlobalSettingsState {
    pub current_theme: AppTheme,
    theme_pick_list: pick_list::State<AppTheme>,
}

impl GlobalSettingsState {
    pub fn new() -> GlobalSettingsState {
        GlobalSettingsState {
            current_theme: AppTheme::default(),
            theme_pick_list: pick_list::State::default(),
        }
    }

    /// Updates the global settings widget.
    pub fn update(&mut self, message: GlobalSettingsMessage) -> Command<GlobalSettingsMessage> {
        match message {
            GlobalSettingsMessage::ThemeChanged(t) => self.current_theme = t,
        }

        Command::none()
    }

    /// Builds the global settings widget.
    pub fn view<'a>(&'a mut self, theme: &'a Box<dyn Theme>) -> Element<'a, GlobalSettingsMessage> {
        let title = Text::new("Global Settings").size(28);

        let theme_label = Text::new("Theme:");
        let theme_pick_list = PickList::new(
            &mut self.theme_pick_list,
            &AppTheme::ALL_THEMES[..],
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
