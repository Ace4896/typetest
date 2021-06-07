use typetest_themes::ApplicationTheme;

/// Represents the state for the results view.
#[derive(Debug)]
pub struct ResultsState {}

/// Represents the messages used by the results view.
#[derive(Clone, Debug)]
pub enum ResultsMessage {}

impl ResultsState {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, _message: ResultsMessage) -> iced::Command<ResultsMessage> {
        iced::Command::none()
    }

    pub fn view(&mut self, _theme: &Box<dyn ApplicationTheme>) -> iced::Element<ResultsMessage> {
        todo!()
    }
}
