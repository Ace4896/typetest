pub mod random;

/// Represents the different statuses a word could be in during a typing test.
pub enum WordStatus {
    NotTyped,
    Correct,
    Incorrect,
}

/// Represents a word that is intended to be displayed during a typing test.
pub struct DisplayedWord {
    pub word: String,
    pub status: WordStatus,
}

impl DisplayedWord {
    pub fn new(word: impl Into<String>) -> Self {
        DisplayedWord {
            word: word.into(),
            status: WordStatus::NotTyped,
        }
    }
}

/// Trait for a word generator.
pub trait WordGenerator {
    /// Returns whether this generator is finite or not.
    ///
    /// An example of a finite word generator would be one that replicates a passage of text.
    fn is_finite(&self) -> bool;

    /// Returns the number of remaining words (if this generator is finite).
    fn remaining_words(&self) -> Option<usize>;

    /// Prepares this word generator for a retry at the same test.
    ///
    /// For random word generators, this re-initialises the RNG with the same seed.
    fn prepare_for_retry(&mut self);

    /// Prepares this word generator for the next test.
    ///
    /// For random word generators, this re-initialises the RNG with a different seed.
    fn prepare_for_next_test(&mut self);

    /// Fills the provided vector with the specified maximum number of characters.
    ///
    /// Requires `&mut self` since some word generators may need to update their state,
    /// e.g. a word generator that replicates a passage of text.
    fn fill_words(&mut self, vec: &mut Vec<DisplayedWord>, max_chars: usize);
}
