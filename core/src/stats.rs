use std::{cmp::Ordering, collections::HashMap};

// TODO: Accuracy unit tests

/// Stores the following statistics for a typing test:
///
/// - No. of correct/incorrect characters
/// - No. of correct/incorrect words
/// - No. of seconds elapsed
/// - Effective WPM since last time update
/// - Raw WPM since last time update
/// - Accuracy
/// - Words which were typed incorrectly, and how often this happened
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TestStatistics {
    missed_words: HashMap<String, u64>,
    pub correct_chars: u64,
    pub incorrect_chars: u64,
    pub correct_words: u64,
    pub incorrect_words: u64,
    pub elapsed_seconds: u64,
    pub effective_wpm: u64,
    pub raw_wpm: u64,
}

impl TestStatistics {
    /// Returns a HashMap with the missed words and how many times they were incorrectly entered.
    pub fn get_missed_words(&self) -> &HashMap<String, u64> {
        &self.missed_words
    }

    /// Resets the current test statistics.
    pub fn reset(&mut self) {
        self.missed_words.clear();
        self.correct_chars = 0;
        self.incorrect_chars = 0;
        self.correct_words = 0;
        self.incorrect_words = 0;
        self.elapsed_seconds = 0;
        self.effective_wpm = 0;
        self.raw_wpm = 0;
    }

    /// Gets the current accuracy as a percentage.
    ///
    /// This isn't stored to save on calculations.
    pub fn accuracy(&self) -> f32 {
        (self.correct_chars as f32 / (self.correct_chars + self.incorrect_chars) as f32) * 100.0
    }

    /// Submits a word and compares it to the expected variant, updating the character/word statistics accordingly.
    ///
    /// WPM is not updated here to save a few calculations.
    pub fn submit_word(&mut self, expected: &str, actual: &str) -> bool {
        if expected.is_empty() {
            return false;
        }

        if expected == actual {
            // NOTE: +1 due to spacebar
            self.correct_chars += expected.len() as u64 + 1;
            self.correct_words += 1;

            true
        } else {
            self.incorrect_words += 1;
            let missed_entry = self.missed_words.entry(expected.to_string()).or_insert(0);
            *missed_entry += 1;

            // Count how many characters are correct
            expected
                .chars()
                .zip(actual.chars())
                .for_each(|(e_char, a_char)| {
                    if e_char == a_char {
                        self.correct_chars += 1;
                    } else {
                        self.incorrect_chars += 1;
                    }
                });

            // If word lengths match, then spacebar usage is correct
            // Otherwise, spacebar usage is incorrect, and we have missing/extra chars
            match expected.len().cmp(&actual.len()) {
                Ordering::Equal => self.correct_chars += 1,
                Ordering::Greater => {
                    self.incorrect_chars += 1 + (expected.len() - actual.len()) as u64
                }
                Ordering::Less => {
                    self.incorrect_chars += 1 + (actual.len() - expected.len()) as u64
                }
            }

            false
        }
    }

    /// Updates the WPM statistics based on the number of elapsed seconds.
    pub fn update_wpm(&mut self, elapsed_secs: u64) {
        if elapsed_secs == 0 {
            return;
        }

        self.elapsed_seconds += elapsed_secs;
        let effective = self.correct_chars as f32 / 5.0 / self.elapsed_seconds as f32 * 60.0;
        let raw =
            (self.correct_chars + self.incorrect_chars) as f32 / 5.0 / self.elapsed_seconds as f32
                * 60.0;
        self.effective_wpm = effective as u64;
        self.raw_wpm = raw as u64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::*;

    #[test]
    fn reset_reinitialises_correct_values() {
        let mut stats = TestStatistics::default();
        stats.submit_word("REDO", "REDO");
        stats.submit_word("REDO", "redo");
        stats.submit_word("REDO", "RE");
        stats.submit_word("REDO", "REDOOO");
        stats.update_wpm(1);

        stats.reset();

        assert_eq!(0, stats.correct_chars);
        assert_eq!(0, stats.incorrect_chars);
        assert_eq!(0, stats.correct_words);
        assert_eq!(0, stats.incorrect_words);

        assert_eq!(0, stats.elapsed_seconds);

        assert_eq!(0, stats.effective_wpm);
        assert_eq!(0, stats.raw_wpm);

        assert!(stats.get_missed_words().is_empty());
    }

    #[test]
    fn submit_word_with_empty_expected_word_returns_false() {
        let mut stats = TestStatistics::default();
        let is_correct = stats.submit_word("", "");

        assert!(!is_correct);
    }

    #[test]
    fn submit_word_with_correct_word_updates_correct_stats() {
        let mut stats = TestStatistics::default();
        let is_correct = stats.submit_word("REDO", "REDO");

        assert!(is_correct);
        assert!(stats.get_missed_words().is_empty());

        // 4 chars for REDO, 1 for spacebar
        assert_eq!(5, stats.correct_chars);
        assert_eq!(1, stats.correct_words);

        assert_eq!(0, stats.incorrect_chars);
        assert_eq!(0, stats.incorrect_words);

        assert!(stats.get_missed_words().is_empty());
    }

    #[rstest(
        expected,
        actual,
        correct_chars,
        incorrect_chars,
        case("REDO", "redo", 1, 4),
        case("REDO", "RED", 3, 2),
        case("REDO", "REDOO", 4, 2),
        case("REDO", "rEdO", 3, 2)
    )]
    fn submit_word_with_incorrect_word_updates_correct_stats(
        expected: &str,
        actual: &str,
        correct_chars: u64,
        incorrect_chars: u64,
    ) {
        let mut stats = TestStatistics::default();

        let is_correct = stats.submit_word(expected, actual);

        assert!(!is_correct);

        assert_eq!(correct_chars, stats.correct_chars);
        assert_eq!(incorrect_chars, stats.incorrect_chars);

        assert_eq!(0, stats.correct_words);
        assert_eq!(1, stats.incorrect_words);

        assert!(stats.get_missed_words().contains_key(expected));
    }

    #[test]
    fn update_wpm_with_0_secs_does_not_update_stats() {
        let mut stats = TestStatistics::default();

        stats.update_wpm(0);

        assert_eq!(0, stats.elapsed_seconds);
        assert_eq!(0, stats.effective_wpm);
        assert_eq!(0, stats.raw_wpm);
    }

    #[test]
    fn update_wpm_with_positive_secs_updates_correct_stats() {
        let mut stats = TestStatistics::default();

        // Submit the following:
        //   REDO, REDO: 5 correct, 0 incorrect
        //   REDO, redo: 1 correct, 4 incorrect
        //     REDO, RE: 2 correct, 3 incorrect
        // REDO, REDOOO: 4 correct, 3 incorrect
        //
        // Total:
        // - Correct Chars: 12
        // - Correct Words: 1
        // - Incorrect Chars: 10
        // - Incorrect Words: 3
        stats.submit_word("REDO", "REDO");
        stats.submit_word("REDO", "redo");
        stats.submit_word("REDO", "RE");
        stats.submit_word("REDO", "REDOOO");

        assert_eq!(12, stats.correct_chars);
        assert_eq!(1, stats.correct_words);
        assert_eq!(10, stats.incorrect_chars);
        assert_eq!(3, stats.incorrect_words);

        // Pretend that 2 seconds have elapsed
        // Expected Statistics:
        // - Effective WPM:
        //   12 / 5 / 2 * 60 = 72 WPM
        // - Raw WPM:
        //   (12 + 10) / 5 / 2 * 60 = 132 WPM
        stats.update_wpm(2);

        assert_eq!(72, stats.effective_wpm);
        assert_eq!(132, stats.raw_wpm);
    }
}
