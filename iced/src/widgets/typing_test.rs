use std::{
    mem,
    time::{Duration, Instant},
};

use iced::{
    button, scrollable, text_input, time, Align, Button, Column, Command, Container, Element,
    HorizontalAlignment, Row, Scrollable, Subscription, Text, TextInput,
};

use typetest_core::{
    stats::TestStats,
    word_generators::{random::InfiniteWordGenerator, DisplayedWord, WordGenerator, WordStatus},
};

use typetest_iced_themes::{theme::Theme, AppTheme};

use crate::{AppMessage, GlobalMessage};

const MAX_CHARS: usize = 65;

/// Represents the possible messages that could be sent during a typing test.
#[derive(Clone, Debug)]
pub enum TypingTestMessage {
    TimerTick(Instant),
    InputChanged(String),
    ToggleWPMDisplay,
    ToggleTimerDisplay,
    ToggleMissedWords,
    Retry,
    NextTest,
}

impl From<TypingTestMessage> for AppMessage {
    #[inline]
    fn from(message: TypingTestMessage) -> AppMessage {
        AppMessage::TypingTest(message)
    }
}

impl TypingTestMessage {
    #[inline]
    fn timer_tick(i: Instant) -> AppMessage {
        TypingTestMessage::TimerTick(i).into()
    }

    #[inline]
    fn input_changed(s: String) -> AppMessage {
        TypingTestMessage::InputChanged(s).into()
    }
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
    pub current_stats: TestStats,
    pub status: TypingTestStatus,

    current_pos: usize,
    current_line: Vec<DisplayedWord>,
    next_line: Vec<DisplayedWord>,

    current_input: String,
    test_start: Instant,
    test_length_seconds: u64,
    last_test_length_seconds: u64,
    remaining_seconds: u64,

    show_wpm: bool,
    show_timer: bool,

    show_missed_words: bool,

    // Widget States
    input_box: text_input::State,
    wpm_button: button::State,
    timer_button: button::State,
    redo_button: button::State,

    results_retry_button: button::State,
    results_next_test_button: button::State,
    results_toggle_missed_button: button::State,
    results_missed_scrollable: scrollable::State,
}

/// Formats the provided number of seconds into the mm:ss format.
#[inline]
fn format_time_mm_ss(seconds: u64) -> String {
    format!("{:0>2}:{:0>2}", seconds / 60, seconds % 60)
}

/// Converts a [DisplayedWord] to an [iced::Text].
fn word_to_iced_text(word: &DisplayedWord, theme: &Box<dyn Theme>) -> Text {
    let theme = theme.displayed_word().word_palette();
    let color = match word.status {
        WordStatus::NotTyped => theme.default,
        WordStatus::Correct => theme.correct,
        WordStatus::Incorrect => theme.incorrect,
    };

    Text::new(&word.word)
        .color(color)
        .font(AppTheme::monospace_font())
}

/// Converts a list of [DisplayedWord]s into an [iced::Row] of [iced::Text]s.
fn words_to_displayed_row<'a>(
    words: &'a [DisplayedWord],
    current_pos: usize,
    theme: &'a Box<dyn Theme>,
) -> Row<'a, AppMessage> {
    let word_theme = theme.displayed_word();

    words
        .iter()
        .enumerate()
        .map(|(pos, w)| -> Element<_> {
            let text = word_to_iced_text(w, theme);
            if current_pos == pos {
                Container::new(text)
                    .style(word_theme.word_background())
                    .into()
            } else {
                text.into()
            }
        })
        .fold(Row::new().spacing(0), |row, w| {
            row.push(w)
                .push(Text::new(" ").font(AppTheme::monospace_font()))
        })
        .into()
}

impl TypingTestState {
    pub fn new(test_length: u64) -> TypingTestState {
        let mut word_gen = Box::new(InfiniteWordGenerator::default());
        let mut current_line = Vec::new();
        let mut next_line = Vec::new();

        word_gen.fill_line(&mut current_line, MAX_CHARS);
        word_gen.fill_line(&mut next_line, MAX_CHARS);

        TypingTestState {
            word_gen,
            current_line,
            next_line,

            current_stats: TestStats::new(),
            status: TypingTestStatus::NotStarted,

            current_pos: 0,

            current_input: String::new(),
            test_start: Instant::now(),
            test_length_seconds: test_length,
            last_test_length_seconds: test_length,
            remaining_seconds: test_length,

            show_wpm: true,
            show_timer: true,

            show_missed_words: false,

            input_box: text_input::State::new(),
            wpm_button: button::State::new(),
            timer_button: button::State::new(),
            redo_button: button::State::new(),

            results_retry_button: button::State::new(),
            results_next_test_button: button::State::new(),
            results_toggle_missed_button: button::State::new(),
            results_missed_scrollable: scrollable::State::new(),
        }
    }

    /// Handles any global updates which may be related to the typing test screen.
    pub fn global_update(&mut self, message: GlobalMessage) -> Command<AppMessage> {
        match message {
            GlobalMessage::TimeLengthChanged(s) => {
                self.test_length_seconds = s;

                // Reset any tests if we're not on the results screen
                if self.status != TypingTestStatus::Finished {
                    self.reset_test_state(false);
                }
            }
        }

        Command::none()
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
                    self.current_stats.checkpoint();
                    self.remaining_seconds = new_remaining;
                }

                if self.remaining_seconds == 0 {
                    self.status = TypingTestStatus::Finished;
                    self.last_test_length_seconds = self.test_length_seconds;
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
                    self.current_stats.next_test();
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
                            self.word_gen.fill_line(&mut self.next_line, MAX_CHARS);

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
            TypingTestMessage::ToggleMissedWords => {
                self.show_missed_words = !self.show_missed_words
            }
            TypingTestMessage::Retry => self.reset_test_state(false),
            TypingTestMessage::NextTest => self.reset_test_state(true),
        }

        Command::none()
    }

    /// Creates the view for the current `TypingTestState`.
    pub fn view<'a>(&'a mut self, theme: &'a Box<dyn Theme>) -> Element<'a, AppMessage> {
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
            TypingTestStatus::Started => {
                time::every(TICK_DURATION).map(TypingTestMessage::timer_tick)
            }
        }
    }

    /// Resets the state of the typing test.
    fn reset_test_state(&mut self, new_test: bool) {
        if new_test {
            self.word_gen.next_test();
        } else {
            self.word_gen.redo();
        }

        self.status = TypingTestStatus::NotStarted;
        self.current_stats.next_test();
        self.current_pos = 0;
        self.current_input.clear();
        self.remaining_seconds = self.test_length_seconds;
        self.show_missed_words = false;

        self.word_gen.fill_line(&mut self.current_line, MAX_CHARS);
        self.word_gen.fill_line(&mut self.next_line, MAX_CHARS);
    }

    /// Builds the typing test widget.
    fn typing_test_widget<'a>(&'a mut self, theme: &'a Box<dyn Theme>) -> Element<'a, AppMessage> {
        let current_line = words_to_displayed_row(&self.current_line, self.current_pos, theme);
        let next_line = words_to_displayed_row(&self.next_line, self.next_line.len(), theme);

        let line_display = Column::new()
            .spacing(5)
            .push(current_line)
            .push(next_line)
            .max_width(600);

        let input_box = TextInput::new(
            &mut self.input_box,
            "",
            &self.current_input,
            TypingTestMessage::input_changed,
        )
        .padding(5)
        .style(theme);

        let wpm_text = if self.show_wpm {
            let wpm = self
                .current_stats
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
        .on_press(TypingTestMessage::ToggleWPMDisplay.into());

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
        .on_press(TypingTestMessage::ToggleTimerDisplay.into());

        let redo_button = Button::new(
            &mut self.redo_button,
            Text::new("Redo").horizontal_alignment(HorizontalAlignment::Center),
        )
        .min_width(80)
        .style(theme)
        .on_press(TypingTestMessage::NextTest.into());

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
    fn results_widget(&mut self, theme: &Box<dyn Theme>) -> Element<AppMessage> {
        let word_palette = theme.displayed_word().word_palette();
        let checkpoint = self
            .current_stats
            .get_latest_checkpoint()
            .expect("No test results to display!");
        let wpm = Text::new(format!("{} WPM", checkpoint.effective_wpm())).size(30);

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

        let raw_wpm = Text::new(format!("{} WPM", checkpoint.raw_wpm()));

        let correct_chars =
            Text::new(checkpoint.correct_chars.to_string()).color(word_palette.correct);
        let incorrect_chars =
            Text::new(checkpoint.incorrect_chars.to_string()).color(word_palette.incorrect);

        let correct_words =
            Text::new(checkpoint.correct_words.to_string()).color(word_palette.correct);
        let incorrect_words =
            Text::new(checkpoint.incorrect_words.to_string()).color(word_palette.incorrect);

        let accuracy = Text::new(format!("{:.2}%", checkpoint.accuracy()));
        let test_length = Text::new(format_time_mm_ss(self.last_test_length_seconds));

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
        .on_press(TypingTestMessage::NextTest.into());

        let retry_button = Button::new(
            &mut self.results_retry_button,
            Text::new("Retry").horizontal_alignment(HorizontalAlignment::Center),
        )
        .min_width(100)
        .style(theme)
        .on_press(TypingTestMessage::Retry.into());

        let toggle_missed_button = {
            let tmp = Button::new(
                &mut self.results_toggle_missed_button,
                Text::new("Toggle Missed Words").horizontal_alignment(HorizontalAlignment::Center),
            )
            .style(theme);

            if self.current_stats.get_missed_words().is_empty() {
                tmp
            } else {
                tmp.on_press(TypingTestMessage::ToggleMissedWords.into())
            }
        };

        let controls = Row::new()
            .align_items(Align::Center)
            .spacing(10)
            .push(next_test_button)
            .push(retry_button)
            .push(toggle_missed_button);

        let mut results = Column::new()
            .align_items(Align::Center)
            .spacing(20)
            .push(wpm)
            .push(stats_grid);

        if self.show_missed_words {
            let missed_words = self.current_stats.get_missed_words();
            if !missed_words.is_empty() {
                // TODO: Grid widget?
                let (missed_words, freq) = missed_words.iter().fold(
                    (
                        Column::new().align_items(Align::End),
                        Column::new().align_items(Align::Start),
                    ),
                    |(expected_col, actual_col), missed_word| {
                        (
                            expected_col.push(Text::new(&missed_word.expected)),
                            actual_col.push(Text::new(&missed_word.actual)),
                        )
                    },
                );

                let missed_words_grid = Row::new().spacing(10).push(missed_words).push(freq);

                results = results.push(
                    Scrollable::new(&mut self.results_missed_scrollable)
                        .align_items(Align::Center)
                        .padding(20)
                        .spacing(10)
                        .style(theme)
                        .max_height(200)
                        .push(Text::new("Missed Words").size(28))
                        .push(missed_words_grid),
                );
            }
        }

        results.push(controls).into()
    }
}