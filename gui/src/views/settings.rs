/// Represents the state for the settings view.
#[derive(Debug)]
pub struct SettingsState {}

/// Represents the messages used by the settings view.
#[derive(Clone, Debug)]
pub struct SettingsMessage {}

impl SettingsState {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, _message: SettingsMessage) -> iced::Command<SettingsMessage> {
        iced::Command::none()
    }

    pub fn view(&mut self) -> iced::Element<SettingsMessage> {
        todo!()
    }
}
