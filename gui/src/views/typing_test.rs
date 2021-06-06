/// Represents the state for the typing test view.
#[derive(Debug)]
pub struct TypingTestState {}

/// Represents the messages used by the typing test view.
#[derive(Clone, Debug)]
pub struct TypingTestMessage {}

impl TypingTestState {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, _message: TypingTestMessage) -> iced::Command<TypingTestMessage> {
        iced::Command::none()
    }

    pub fn view(&mut self) -> iced::Element<TypingTestMessage> {
        todo!()
    }
}
