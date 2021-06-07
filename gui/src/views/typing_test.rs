use typetest_themes::ApplicationTheme;

/// Represents the state for the typing test view.
#[derive(Debug)]
pub struct TypingTestState {}

/// Represents the messages used by the typing test view.
#[derive(Clone, Debug)]
pub enum TypingTestMessage {}

impl TypingTestState {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, _message: TypingTestMessage) -> iced::Command<TypingTestMessage> {
        iced::Command::none()
    }

    pub fn view(&mut self, _theme: &Box<dyn ApplicationTheme>) -> iced::Element<TypingTestMessage> {
        todo!()
    }

    pub fn subscription(&self) -> iced::Subscription<TypingTestMessage> {
        iced::Subscription::none()
    }
}
