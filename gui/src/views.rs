use typetest_core::stats::TestStats;
use typetest_themes::Theme;

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

/// Represents an application-wide action which can be signalled from a view.
#[derive(Clone, Debug)]
pub enum Action {
    ChangeTheme(Theme),
    ChangeView(View),

    // TODO: This should be some kind of "typing settings changed" structure, not an individual one
    ChangeTimeLength(u64),

    DisplayResults(TestStats),
    RetryTest,
    NextTest,
}
