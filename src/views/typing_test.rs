use std::time::{Duration, Instant};

use iced::{
    button, text_input, Align, Button, Column, Command, Container, Element, HorizontalAlignment,
    Row, Text, TextInput,
};
use typetest_core::{
    stats::TestStats,
    word_generators::{random::InfiniteWordGenerator, DisplayedWord, WordGenerator, WordStatus},
};
use typetest_themes::{ApplicationTheme, Theme};

use crate::widgets::word_submission::SubmissionWrapper;

use super::{Action, View};

const MAX_CHARS: usize = 80;

/// Represents the state for the typing test view.
pub struct TypingTestState {
    word_gen: Box<dyn WordGenerator>,
    stats: TestStats,
    status: TypingTestStatus,

    current_pos: usize,
    current_line: Vec<DisplayedWord>,
    next_line: Vec<DisplayedWord>,

    current_input: String,

    test_start: Instant,
    test_length_seconds: u64,
    remaining_seconds: u64,

    show_wpm: bool,
    show_timer: bool,

    input_box: text_input::State,
    wpm_button: button::State,
    timer_button: button::State,
    redo_button: button::State,
    settings_button: button::State,
}

/// Represents the messages used by the typing test view.
#[derive(Clone, Debug)]
pub enum TypingTestMessage {
    TimerTick(Instant),
    InputChanged(String),
    WordSubmitted,

    ToggleWPM,
    ToggleTimer,
    Redo,

    Action(Action),
}

/// Represents the different statuses a typing test could be in.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TypingTestStatus {
    NotStarted,
    Started,
    Finished,
}

impl TypingTestState {
    pub fn new() -> Self {
        let mut word_gen = Box::new(InfiniteWordGenerator::default());
        let mut current_line = Vec::new();
        let mut next_line = Vec::new();

        word_gen.fill_line(&mut current_line, MAX_CHARS);
        word_gen.fill_line(&mut next_line, MAX_CHARS);

        Self {
            word_gen,
            current_line,
            next_line,

            stats: TestStats::new(),
            status: TypingTestStatus::NotStarted,

            current_pos: 0,

            current_input: String::new(),

            test_start: Instant::now(),
            test_length_seconds: 60,
            remaining_seconds: 60,

            show_wpm: true,
            show_timer: true,

            input_box: text_input::State::new(),
            settings_button: button::State::new(),
            wpm_button: button::State::new(),
            timer_button: button::State::new(),
            redo_button: button::State::new(),
        }
    }

    pub fn update(&mut self, message: TypingTestMessage) -> iced::Command<TypingTestMessage> {
        match message {
            TypingTestMessage::TimerTick(i) => {
                if self.status != TypingTestStatus::Started {
                    return Command::none();
                }

                let elapsed = i
                    .checked_duration_since(self.test_start)
                    .unwrap_or_default()
                    .as_secs();

                let new_remaining = self.test_length_seconds - elapsed;
                if self.remaining_seconds != new_remaining {
                    self.stats.checkpoint();
                    self.remaining_seconds = new_remaining;
                }

                if self.remaining_seconds == 0 {
                    self.status = TypingTestStatus::Finished;
                    let stats = self.stats.clone();
                    return Command::perform(async move { stats }, |stats| {
                        TypingTestMessage::Action(Action::DisplayResults(stats))
                    });
                }
            }
            TypingTestMessage::InputChanged(s) => {
                if self.status == TypingTestStatus::Finished {
                    return Command::none();
                }

                // Begin the test if it hasn't already
                if self.status == TypingTestStatus::NotStarted {
                    self.status = TypingTestStatus::Started;
                    self.stats.next_test();
                    self.test_start = Instant::now();
                }

                // Update the status for the current word
                self.current_line[self.current_pos].status =
                    if self.current_line[self.current_pos].word.starts_with(&s) {
                        WordStatus::NotTyped
                    } else {
                        WordStatus::Incorrect
                    };

                self.current_input = s;
            }
            TypingTestMessage::WordSubmitted => {
                if self.status != TypingTestStatus::Started || self.current_input.is_empty() {
                    return Command::none();
                }

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

                if self.current_pos >= self.current_line.len() - 1 {
                    self.current_pos = 0;
                    std::mem::swap(&mut self.current_line, &mut self.next_line);
                    self.word_gen.fill_line(&mut self.next_line, MAX_CHARS);
                } else {
                    self.current_pos += 1;
                }
            }
            TypingTestMessage::ToggleWPM => self.show_wpm = !self.show_wpm,
            TypingTestMessage::ToggleTimer => self.show_timer = !self.show_timer,
            TypingTestMessage::Redo => self.reset_test_state(true),
            _ => {}
        }

        Command::none()
    }

    pub fn view<'a>(
        &'a mut self,
        theme: &'a Box<dyn ApplicationTheme>,
    ) -> iced::Element<'a, TypingTestMessage> {
        // Typing Area
        // NOTE: +1 to max chars due to tiny gap between displayed words
        let current_line = line_of_displayed_words(&self.current_line, self.current_pos, theme);
        let next_line = line_of_displayed_words(&self.next_line, self.next_line.len(), theme);
        let line_display = Column::new()
            .spacing(5)
            .push(blank_line())
            .push(current_line)
            .push(next_line);

        let input_box = TextInput::new(
            &mut self.input_box,
            "",
            &self.current_input,
            TypingTestMessage::InputChanged,
        )
        .padding(5)
        .style(theme);

        let submission_wrapper = SubmissionWrapper::new(
            input_box,
            TypingTestMessage::WordSubmitted,
            TypingTestMessage::Redo,
        );

        let wpm_text = if self.show_wpm {
            let wpm = self
                .stats
                .get_latest_checkpoint()
                .map(|checkpoint| checkpoint.effective_wpm())
                .unwrap_or_default();

            format!("{} WPM", wpm)
        } else {
            String::from(" ")
        };

        let wpm_button = Button::new(
            &mut self.wpm_button,
            Text::new(wpm_text).horizontal_alignment(HorizontalAlignment::Center),
        )
        .min_width(100)
        .style(theme)
        .on_press(TypingTestMessage::ToggleWPM);

        let timer_text = if self.show_timer {
            format_time_mm_ss(self.remaining_seconds)
        } else {
            String::from(" ")
        };

        let timer_button = Button::new(
            &mut self.timer_button,
            Text::new(timer_text).horizontal_alignment(HorizontalAlignment::Center),
        )
        .min_width(80)
        .style(theme)
        .on_press(TypingTestMessage::ToggleTimer);

        let redo_button = Button::new(
            &mut self.redo_button,
            Text::new("Redo").horizontal_alignment(HorizontalAlignment::Center),
        )
        .min_width(80)
        .style(theme)
        .on_press(TypingTestMessage::Redo);

        let typing_area = Row::new()
            .align_items(Align::Center)
            .max_width(600)
            .spacing(10)
            .push(submission_wrapper)
            .push(wpm_button)
            .push(timer_button)
            .push(redo_button);

        // Settings Button
        let settings_button = Button::new(
            &mut self.settings_button,
            Text::new("Settings").horizontal_alignment(HorizontalAlignment::Center),
        )
        .min_width(100)
        .style(theme)
        .on_press(TypingTestMessage::Action(Action::ChangeView(
            View::Settings,
        )));

        Column::new()
            .align_items(Align::Center)
            .spacing(20)
            .push(line_display)
            .push(typing_area)
            .push(settings_button)
            .push(blank_line())
            .into()
    }

    pub fn subscription(&self) -> iced::Subscription<TypingTestMessage> {
        const TICK_DURATION: Duration = Duration::from_millis(100);

        match self.status {
            TypingTestStatus::NotStarted | TypingTestStatus::Finished => iced::Subscription::none(),
            TypingTestStatus::Started => {
                iced::time::every(TICK_DURATION).map(TypingTestMessage::TimerTick)
            }
        }
    }

    pub fn update_time_length(&mut self, time: u64) {
        self.test_length_seconds = time;
        self.reset_test_state(true);
    }

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
        self.remaining_seconds = self.test_length_seconds;

        self.word_gen.fill_line(&mut self.current_line, MAX_CHARS);
        self.word_gen.fill_line(&mut self.next_line, MAX_CHARS);
    }
}

/// Formats the provided number of seconds into the mm:ss format.
#[inline]
fn format_time_mm_ss(seconds: u64) -> String {
    format!("{:0>2}:{:0>2}", seconds / 60, seconds % 60)
}

#[inline]
fn blank_line() -> Text {
    Text::new(" ".repeat(MAX_CHARS + 1))
        .font(Theme::monospace_font())
        .size(22)
}

/// Converts a [DisplayedWord] to an [iced::Text].
fn displayed_word(word: &DisplayedWord, theme: &Box<dyn ApplicationTheme>) -> Text {
    let theme = theme.word_palette();
    let color = match word.status {
        WordStatus::NotTyped => theme.default,
        WordStatus::Correct => theme.correct,
        WordStatus::Incorrect => theme.incorrect,
    };

    Text::new(&word.word)
        .color(color)
        .font(Theme::monospace_font())
        .size(22)
}

/// Converts a list of [DisplayedWord]s into a line of [iced::Text]s.
fn line_of_displayed_words<'a>(
    words: &'a [DisplayedWord],
    current_pos: usize,
    theme: &'a Box<dyn ApplicationTheme>,
) -> Row<'a, TypingTestMessage> {
    let mut elements_iter = words.iter().enumerate().map(|(pos, w)| -> Element<_> {
        let text = displayed_word(w, theme);
        if current_pos == pos {
            Container::new(text).style(theme.word_background()).into()
        } else {
            text.into()
        }
    });

    let mut line = Row::new().spacing(0);
    if let Some(element) = elements_iter.next() {
        line = line.push(element);
    }

    elements_iter.fold(line, |row, word| {
        row.push(Text::new(" ").font(Theme::monospace_font()))
            .push(word)
    })
}
