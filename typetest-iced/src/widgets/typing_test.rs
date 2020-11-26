use std::{
    mem,
    time::{Duration, Instant},
};

use iced::{
    button, text_input, time, Align, Button, Column, Command, Container, Element,
    HorizontalAlignment, Row, Subscription, Text, TextInput,
};

use typetest_core::{
    stats::TestStatistics,
    word_gen::{random::RandomWordGenerator, DisplayedWord, WordGenerator, WordStatus},
};

use crate::{
    theme::{TextPalette, Theme},
    AppMessage,
};

const MAX_CHARS: usize = 50;

/// Represents the possible messages that could be sent during a typing test.
#[derive(Clone, Debug)]
pub enum TypingTestMessage {
    TimerTick(Instant),
    InputChanged(String),
    ToggleWPMDisplay,
    ToggleTimerDisplay,
    Retry,
    NextTest,
}

/// Represents the different statuses a typing test could be in.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TypingTestStatus {
    NotStarted,
    Started,
    Finished,
}

/// Represents the current state for a typing test.
pub struct TypingTestState {
    word_gen: Box<dyn WordGenerator>,
    pub current_stats: TestStatistics,
    pub status: TypingTestStatus,

    current_pos: usize,
    current_line: Vec<DisplayedWord>,
    next_line: Vec<DisplayedWord>,

    current_input: String,
    test_start: Instant,
    test_length_seconds: u64,
    remaining_seconds: u64,

    show_wpm: bool,
    show_timer: bool,

    // Widget States
    input_box: text_input::State,
    wpm_button: button::State,
    timer_button: button::State,
    redo_button: button::State,

    results_retry_button: button::State,
    results_next_test_button: button::State,
}

/// Formats the provided number of seconds into the mm:ss format.
#[inline]
fn format_time_mm_ss(seconds: u64) -> String {
    format!("{:0>2}:{:0>2}", seconds / 60, seconds % 60)
}

impl TypingTestState {
    pub fn new() -> TypingTestState {
        let mut word_gen = Box::new(RandomWordGenerator::default());
        let mut current_line = Vec::new();
        let mut next_line = Vec::new();

        word_gen.fill_words(&mut current_line, MAX_CHARS);
        word_gen.fill_words(&mut next_line, MAX_CHARS);

        TypingTestState {
            word_gen,
            current_line,
            next_line,

            current_stats: TestStatistics::default(),
            status: TypingTestStatus::NotStarted,

            current_pos: 0,

            current_input: String::new(),
            test_start: Instant::now(),
            test_length_seconds: 60,
            remaining_seconds: 60,

            show_wpm: true,
            show_timer: true,

            input_box: text_input::State::new(),
            wpm_button: button::State::new(),
            timer_button: button::State::new(),
            redo_button: button::State::new(),

            results_retry_button: button::State::new(),
            results_next_test_button: button::State::new(),
        }
    }

    /// Handles updates for the typing test screen.
    pub fn update(&mut self, message: TypingTestMessage) -> Command<AppMessage> {
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
                    self.current_stats
                        .update_wpm(self.remaining_seconds - new_remaining);
                    self.remaining_seconds = new_remaining;
                }

                if self.remaining_seconds == 0 {
                    self.status = TypingTestStatus::Finished;
                }
            }
            TypingTestMessage::InputChanged(mut s) => {
                if self.status == TypingTestStatus::Finished {
                    return Command::none();
                }

                // Begin the test if it hasn't already
                if self.status == TypingTestStatus::NotStarted {
                    self.status = TypingTestStatus::Started;
                    self.test_start = Instant::now();
                }

                // If it ends in a space, prepare to submit the word
                if s.ends_with(' ') {
                    let trimmed = s.trim();
                    if !trimmed.is_empty() {
                        let is_correct = self
                            .current_stats
                            .submit_word(&self.current_line[self.current_pos].word, trimmed);

                        self.current_line[self.current_pos].status = if is_correct {
                            WordStatus::Correct
                        } else {
                            WordStatus::Incorrect
                        };

                        if self.current_pos >= self.current_line.len() - 1 {
                            self.current_pos = 0;
                            mem::swap(&mut self.current_line, &mut self.next_line);
                            self.word_gen.fill_words(&mut self.next_line, MAX_CHARS);

                            // If we're using a finite word generator and the next line is empty, test is done
                            if self.next_line.is_empty() {
                                self.status = TypingTestStatus::Finished;
                            }
                        } else {
                            self.current_pos += 1;
                        }
                    }

                    s.clear();
                } else {
                    // If it does not end in a space, just check if the word is correct so far
                    self.current_line[self.current_pos].status =
                        if self.current_line[self.current_pos].word.starts_with(&s) {
                            WordStatus::NotTyped
                        } else {
                            WordStatus::Incorrect
                        };
                }

                self.current_input = s;
            }
            TypingTestMessage::ToggleWPMDisplay => self.show_wpm = !self.show_wpm,
            TypingTestMessage::ToggleTimerDisplay => self.show_timer = !self.show_timer,
            TypingTestMessage::Retry => {
                self.status = TypingTestStatus::NotStarted;
                self.word_gen.prepare_for_retry();
                self.current_stats.reset();
                self.current_pos = 0;
                self.current_input.clear();
                self.remaining_seconds = self.test_length_seconds;

                self.word_gen.fill_words(&mut self.current_line, MAX_CHARS);
                self.word_gen.fill_words(&mut self.next_line, MAX_CHARS);
            }
            TypingTestMessage::NextTest => {
                self.status = TypingTestStatus::NotStarted;
                self.word_gen.prepare_for_next_test();
                self.current_stats.reset();
                self.current_pos = 0;
                self.current_input.clear();
                self.remaining_seconds = self.test_length_seconds;

                self.word_gen.fill_words(&mut self.current_line, MAX_CHARS);
                self.word_gen.fill_words(&mut self.next_line, MAX_CHARS);
            }
        }

        Command::none()
    }

    /// Creates the view for the current `TypingTestState`.
    pub fn view(&mut self, theme: Theme) -> Element<AppMessage> {
        if self.status == TypingTestStatus::Finished {
            self.results_widget(theme)
        } else {
            self.typing_test_widget(theme)
        }
    }

    /// Handles subscriptions for the typing test screen.
    pub fn subscription(&self) -> Subscription<AppMessage> {
        const TICK_DURATION: Duration = Duration::from_secs(1);

        match self.status {
            TypingTestStatus::NotStarted | TypingTestStatus::Finished => Subscription::none(),
            TypingTestStatus::Started => time::every(TICK_DURATION)
                .map(|i| AppMessage::TypingTest(TypingTestMessage::TimerTick(i))),
        }
    }

    /// Builds the typing test widget.
    fn typing_test_widget(&mut self, theme: Theme) -> Element<AppMessage> {
        fn word_to_iced_text(word: &DisplayedWord, text_palette: &TextPalette) -> Text {
            let color = match word.status {
                WordStatus::NotTyped => text_palette.default,
                WordStatus::Correct => text_palette.correct,
                WordStatus::Incorrect => text_palette.incorrect,
            };

            Text::new(&word.word).color(color).font(Theme::monospace())
        }

        fn words_to_displayed_row(
            words: &[DisplayedWord],
            current_pos: usize,
            theme: Theme,
        ) -> Row<AppMessage> {
            let text_palette = theme.text_palette();
            words
                .iter()
                .enumerate()
                .map(|(pos, w)| -> Element<_> {
                    let text = word_to_iced_text(w, text_palette);
                    if current_pos == pos {
                        Container::new(text).style(theme.word_background()).into()
                    } else {
                        text.into()
                    }
                })
                .fold(Row::new().spacing(0), |row, w| {
                    row.push(w).push(Text::new(" ").font(Theme::monospace()))
                })
                .into()
        }

        let current_line = words_to_displayed_row(&self.current_line, self.current_pos, theme);
        let next_line = words_to_displayed_row(&self.next_line, self.next_line.len(), theme);

        let line_display = Column::new()
            .spacing(5)
            .push(current_line)
            .push(next_line)
            .max_width(600);

        let input_box = TextInput::new(&mut self.input_box, "", &self.current_input, |s| {
            AppMessage::TypingTest(TypingTestMessage::InputChanged(s))
        })
        .padding(5)
        .style(theme);

        let wpm_text = if self.show_wpm {
            format!("{} WPM", self.current_stats.effective_wpm)
        } else {
            String::from(" ")
        };

        let wpm_button = Button::new(
            &mut self.wpm_button,
            Text::new(wpm_text).horizontal_alignment(HorizontalAlignment::Center),
        )
        .min_width(100)
        .style(theme)
        .on_press(AppMessage::TypingTest(TypingTestMessage::ToggleWPMDisplay));

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
        .on_press(AppMessage::TypingTest(
            TypingTestMessage::ToggleTimerDisplay,
        ));

        let redo_button = Button::new(
            &mut self.redo_button,
            Text::new("Redo").horizontal_alignment(HorizontalAlignment::Center),
        )
        .min_width(80)
        .style(theme)
        .on_press(AppMessage::TypingTest(TypingTestMessage::NextTest));

        let typing_area = Row::new()
            .spacing(10)
            .align_items(Align::Center)
            .max_width(600)
            .push(input_box)
            .push(wpm_button)
            .push(timer_button)
            .push(redo_button);

        Column::new()
            .spacing(20)
            .push(line_display)
            .push(typing_area)
            .into()
    }

    /// Builds the results widget.
    fn results_widget(&mut self, theme: Theme) -> Element<AppMessage> {
        let text_palette = theme.text_palette();

        let wpm = Text::new(format!("{} WPM", self.current_stats.effective_wpm)).size(30);

        // Labels
        const LABEL_SPACING: u16 = 10;
        let labels = Column::new()
            .align_items(Align::End)
            .spacing(LABEL_SPACING)
            .push(Text::new("Raw WPM:"))
            .push(Text::new("Correct Characters:"))
            .push(Text::new("Incorrect Characters:"))
            .push(Text::new("Correct Words:"))
            .push(Text::new("Incorrect Words:"))
            .push(Text::new("Accuracy:"))
            .push(Text::new("Test Length:"));

        let raw_wpm = Text::new(format!("{} WPM", self.current_stats.raw_wpm));

        let correct_chars =
            Text::new(self.current_stats.correct_chars.to_string()).color(text_palette.correct);
        let incorrect_chars =
            Text::new(self.current_stats.incorrect_chars.to_string()).color(text_palette.incorrect);

        let correct_words =
            Text::new(self.current_stats.correct_words.to_string()).color(text_palette.correct);
        let incorrect_words =
            Text::new(self.current_stats.incorrect_words.to_string()).color(text_palette.incorrect);

        let accuracy = Text::new(format!("{:.2}%", self.current_stats.accuracy()));
        let test_length = Text::new(format_time_mm_ss(self.test_length_seconds));

        let values = Column::new()
            .align_items(Align::Start)
            .spacing(LABEL_SPACING)
            .push(raw_wpm)
            .push(correct_chars)
            .push(incorrect_chars)
            .push(correct_words)
            .push(incorrect_words)
            .push(accuracy)
            .push(test_length);

        let stats_grid = Row::new().spacing(10).push(labels).push(values);

        let next_test_button = Button::new(
            &mut self.results_next_test_button,
            Text::new("Next Test").horizontal_alignment(HorizontalAlignment::Center),
        )
        .min_width(100)
        .style(theme)
        .on_press(AppMessage::TypingTest(TypingTestMessage::NextTest));

        let retry_button = Button::new(
            &mut self.results_retry_button,
            Text::new("Retry").horizontal_alignment(HorizontalAlignment::Center),
        )
        .min_width(100)
        .style(theme)
        .on_press(AppMessage::TypingTest(TypingTestMessage::Retry));

        let controls = Row::new()
            .align_items(Align::Center)
            .spacing(10)
            .push(next_test_button)
            .push(retry_button);

        Column::new()
            .align_items(Align::Center)
            .spacing(20)
            .push(wpm)
            .push(stats_grid)
            .push(controls)
            .into()
    }
}
