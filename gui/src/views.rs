pub mod results;
pub mod settings;
pub mod typing_test;

/// Represents the different views in the application.
#[derive(Clone, Copy, Debug)]
pub enum View {
    TypingTest,
    Results,
    Settings,
}
