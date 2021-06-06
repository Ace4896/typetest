use std::{
    cmp::Ordering,
    time::{Duration, Instant},
};

/// Stores statistics for a typing test.
pub struct TestStats {
    test_start: Instant,
    checkpoints: Vec<TestCheckpoint>,
    missed_words: Vec<MissedWord>,

    correct_chars: u64,
    incorrect_chars: u64,
    correct_words: u64,
    incorrect_words: u64,
}

/// Represents a statistics checkpoint in a typing test.
///
/// Eventually, this will be used for graphing WPM.
pub struct TestCheckpoint {
    pub elapsed: Duration,

    pub correct_chars: u64,
    pub incorrect_chars: u64,
    pub correct_words: u64,
    pub incorrect_words: u64,
}

/// Represents a missed word in a typing test.
#[derive(PartialEq)]
pub struct MissedWord {
    pub expected: String,
    pub actual: String,
}

impl TestStats {
    /// Creates a new instance of test statistics.
    pub fn new() -> Self {
        Self {
            test_start: Instant::now(),
            checkpoints: Vec::new(),
            missed_words: Vec::new(),

            correct_chars: 0,
            incorrect_chars: 0,
            correct_words: 0,
            incorrect_words: 0,
        }
    }

    /// Gets the list of missed words (in the order they were typed).
    pub fn get_missed_words(&self) -> &[MissedWord] {
        &self.missed_words
    }

    /// Gets the latest statistics checkpoint.
    pub fn get_latest_checkpoint(&self) -> Option<&TestCheckpoint> {
        self.checkpoints.last()
    }

    /// Starts the next test.
    pub fn next_test(&mut self) {
        self.test_start = Instant::now();
        self.checkpoints.clear();
        self.missed_words.clear();

        self.correct_chars = 0;
        self.incorrect_chars = 0;
        self.correct_words = 0;
        self.incorrect_words = 0;
    }

    /// Submits a word for the current test, returning whether it was correct or not.
    pub fn submit_word(&mut self, expected: &str, actual: &str) -> bool {
        if expected.is_empty() {
            return false;
        }

        if expected == actual {
            // NOTE: +1 to chars due to spacebar
            self.correct_chars += expected.len() as u64 + 1;
            self.correct_words += 1;

            true
        } else {
            self.incorrect_words += 1;

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

            self.missed_words.push(MissedWord::new(expected, actual));

            false
        }
    }

    /// Saves a statistics checkpoint the test.
    pub fn checkpoint(&mut self) {
        let checkpoint = TestCheckpoint {
            elapsed: Instant::now().duration_since(self.test_start),

            correct_chars: self.correct_chars,
            incorrect_chars: self.incorrect_chars,
            correct_words: self.correct_words,
            incorrect_words: self.incorrect_words,
        };

        self.checkpoints.push(checkpoint);
    }
}

impl TestCheckpoint {
    /// Calculates the accuracy for this checkpoint.
    pub fn accuracy(&self) -> f32 {
        (self.correct_chars as f32 / (self.correct_chars + self.incorrect_chars) as f32) * 100.0
    }

    /// Calculates the effective WPM for this checkpoint.
    /// Uses 1 WPM = 5 CPM for this calculation.
    pub fn effective_wpm(&self) -> u64 {
        (self.correct_chars as f32 / 5.0 / self.elapsed.as_secs_f32() * 60.0) as u64
    }

    /// Calculates the raw WPM for this checkpoint.
    /// Uses 1 WPM = 5 CPM for this calculation.
    pub fn raw_wpm(&self) -> u64 {
        ((self.correct_chars + self.incorrect_chars) as f32 / 5.0 / self.elapsed.as_secs_f32()
            * 60.0) as u64
    }
}

impl MissedWord {
    /// Creates a new instance of a missed word.
    pub fn new(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self {
            expected: expected.into(),
            actual: actual.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::*;

    mod test_stats {
        use super::*;

        #[test]
        fn submit_word_with_empty_expected_word_returns_false() {
            let mut stats = TestStats::new();
            let is_correct = stats.submit_word("", "");

            assert!(!is_correct);
        }

        #[test]
        fn submit_word_with_correct_word_updates_correct_stats() {
            let mut stats = TestStats::new();
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
            let mut stats = TestStats::new();

            let is_correct = stats.submit_word(expected, actual);

            assert!(!is_correct);

            assert_eq!(correct_chars, stats.correct_chars);
            assert_eq!(incorrect_chars, stats.incorrect_chars);

            assert_eq!(0, stats.correct_words);
            assert_eq!(1, stats.incorrect_words);

            assert!(stats
                .get_missed_words()
                .contains(&MissedWord::new(expected, actual)));
        }
    }

    mod test_checkpoint {
        use super::*;

        #[rstest(
            correct_chars,
            incorrect_chars,
            expected_accuracy,
            case(10, 0, 100.0),
            case(10, 10, 50.0),
            case(5, 20, 20.0)
        )]
        fn accuracy(correct_chars: u64, incorrect_chars: u64, expected_accuracy: f32) {
            let checkpoint = TestCheckpoint {
                elapsed: Duration::from_secs(0),

                correct_chars,
                incorrect_chars,
                correct_words: 0,
                incorrect_words: 0,
            };

            assert!((expected_accuracy - checkpoint.accuracy()) < 0.01);
        }

        #[rstest(
            elapsed,
            correct_chars,
            incorrect_chars,
            expected_wpm,
            case(Duration::from_secs(60), 100, 80, 20),
            case(Duration::from_secs(30), 100, 80, 40)
        )]
        fn effective_wpm(
            elapsed: Duration,
            correct_chars: u64,
            incorrect_chars: u64,
            expected_wpm: u64,
        ) {
            let checkpoint = TestCheckpoint {
                elapsed,

                correct_chars,
                incorrect_chars,
                correct_words: 0,
                incorrect_words: 0,
            };

            assert_eq!(expected_wpm, checkpoint.effective_wpm());
        }

        #[rstest(
            elapsed,
            correct_chars,
            incorrect_chars,
            expected_wpm,
            case(Duration::from_secs(60), 100, 80, 36),
            case(Duration::from_secs(30), 100, 80, 72)
        )]
        fn raw_wpm(elapsed: Duration, correct_chars: u64, incorrect_chars: u64, expected_wpm: u64) {
            let checkpoint = TestCheckpoint {
                elapsed,

                correct_chars,
                incorrect_chars,
                correct_words: 0,
                incorrect_words: 0,
            };

            assert_eq!(expected_wpm, checkpoint.raw_wpm());
        }
    }
}
