use std::time::Instant;

use iced::{Command, Element, Theme};
use typetest_core::{
    stats::TestStats,
    word_generators::{random::InfiniteWordGenerator, DisplayedWord, WordGenerator, WordStatus},
};

const LINE_WIDTH: usize = 80;

/// Represents the state for the typing test view.
pub struct TypingTestView {
    word_gen: Box<dyn WordGenerator>,
    stats: TestStats,
    status: TypingTestStatus,

    current_pos: usize,
    previous_line: Vec<DisplayedWord>,
    current_line: Vec<DisplayedWord>,
    next_line: Vec<DisplayedWord>,

    test_start: Instant,
    test_length_secs: u64,
    remaining_secs: u64,

    current_input: String,
    show_wpm: bool,
    show_timer: bool,
}

/// A message for events that can occur on the typing test screen.
#[derive(Clone, Debug)]
pub enum TypingTestMessage {
    TimerTick(Instant),
    InputChanged(String),
    WordSubmitted,
    ToggleWPM,
    ToggleTimer,
    Redo,
}

/// Represents the possible statuses that a typing test could be in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypingTestStatus {
    NotStarted,
    InProgress,
    Complete,
}

impl TypingTestView {
    /// Sets up a new `TypingTestView`.
    pub fn new() -> Self {
        let mut word_gen = Box::new(InfiniteWordGenerator::default());
        let mut current_line = Vec::new();
        let mut next_line = Vec::new();

        word_gen.fill_line(&mut current_line, LINE_WIDTH);
        word_gen.fill_line(&mut next_line, LINE_WIDTH);

        Self {
            word_gen,
            stats: TestStats::new(),
            status: TypingTestStatus::NotStarted,

            current_pos: 0,
            previous_line: Vec::new(),
            current_line,
            next_line,

            test_start: Instant::now(),
            test_length_secs: 60,
            remaining_secs: 60,

            current_input: String::new(),
            show_wpm: true,
            show_timer: true,
        }
    }

    /// Updates this `TypingTestView` from a `TypingTestMessage`.
    pub fn update(&mut self, message: impl Into<TypingTestMessage>) -> Command<TypingTestMessage> {
        match message.into() {
            TypingTestMessage::TimerTick(i) => {
                if self.status != TypingTestStatus::InProgress {
                    return Command::none();
                }

                // Determine if we need to make another stats checkpoint
                let elapsed = i.duration_since(self.test_start).as_secs();
                let new_remaining = self.test_length_secs - elapsed;

                if self.remaining_secs != new_remaining {
                    self.stats.checkpoint();
                    self.remaining_secs = new_remaining;
                }

                // If the test has finished, submit the test stats to the results screen
                if self.remaining_secs == 0 {
                    self.status = TypingTestStatus::Complete;
                    // TODO: Command that will send over the relevant action
                }
            }
            TypingTestMessage::InputChanged(t) => {
                if self.status == TypingTestStatus::Complete {
                    return Command::none();
                }

                // Begin the test if it hasn't already
                if self.status == TypingTestStatus::NotStarted {
                    self.status = TypingTestStatus::InProgress;
                    self.stats.next_test();
                    self.test_start = Instant::now();
                }

                // Update the status for the current word
                let current_word = &mut self.current_line[self.current_pos];
                current_word.status = if current_word.word.starts_with(&t) {
                    WordStatus::NotTyped
                } else {
                    WordStatus::Incorrect
                };

                self.current_input = t;
            }
            TypingTestMessage::WordSubmitted => {
                if self.status != TypingTestStatus::InProgress || self.current_input.is_empty() {
                    return Command::none();
                }

                // Submit the current input and update the displayed status
                let is_correct = self.stats.submit_word(
                    &self.current_line[self.current_pos].word,
                    &self.current_input,
                );

                self.current_input.clear();
                self.current_line[self.current_pos].status = if is_correct {
                    WordStatus::Correct
                } else {
                    WordStatus::Incorrect
                };

                // Shift the lines up by one and add the next line of input
                std::mem::swap(&mut self.previous_line, &mut self.current_line);
                std::mem::swap(&mut self.current_line, &mut self.next_line);
                self.word_gen.fill_line(&mut self.next_line, LINE_WIDTH);
            }
            TypingTestMessage::ToggleWPM => self.show_wpm = !self.show_wpm,
            TypingTestMessage::ToggleTimer => self.show_timer = !self.show_timer,
            TypingTestMessage::Redo => self.reset_test_state(false),
        }

        Command::none()
    }

    /// Renders the `TypingTestView`.
    pub fn view<'a>(&'a self) -> Element<'a, TypingTestMessage> {
        todo!()
    }

    /// Resets the current test state, either by restarting the current test or beginning a new test.
    pub fn reset_test_state(&mut self, new_test: bool) {
        if new_test {
            self.word_gen.next_test();
        } else {
            self.word_gen.redo();
        }

        self.status = TypingTestStatus::NotStarted;
        self.stats.next_test();
        self.current_pos = 0;
        self.current_input.clear();
        self.remaining_secs = self.test_length_secs;

        self.previous_line.clear();
        self.word_gen.fill_line(&mut self.current_line, LINE_WIDTH);
        self.word_gen.fill_line(&mut self.next_line, LINE_WIDTH);
    }
}
