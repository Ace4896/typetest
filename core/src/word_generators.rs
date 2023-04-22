pub mod random;

/// Represents the different statuses a word could be in during a typing test.
#[derive(Clone, Copy)]
pub enum WordStatus {
    NotTyped,
    Correct,
    Incorrect,
}

/// Represents a word that is intended to be displayed during a typing test.
pub struct TestWord {
    pub word: String,
    pub status: WordStatus,
}

impl TestWord {
    pub fn new(word: impl Into<String>) -> Self {
        TestWord {
            word: word.into(),
            status: WordStatus::NotTyped,
        }
    }
}

/// Common trait for all word generators.
pub trait WordGenerator {
    /// Fills a line with words, respecting the maximum number of characters.
    ///
    /// Requires `&mut self` since some word generators may need to update their state,
    /// e.g. a word generator that replicates a passage of text.
    fn fill_line(&mut self, line: &mut Vec<TestWord>, max_chars: usize);

    /// Prepares this word generator for a redo of the same test.
    fn redo(&mut self);

    /// Prepares this word generator for the next test.
    fn next_test(&mut self);
}
