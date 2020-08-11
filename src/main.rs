use std::time::{Duration, Instant};

use iced::{
    button, executor, text_input, time, Align, Application, Button, Color, Column, Command,
    Container, HorizontalAlignment, Length, Radio, Row, Settings, Subscription, Text, TextInput,
};

mod word_pool;

const MAX_LINE_LENGTH: usize = 60;
const TEST_TIME_SECS: u32 = 60;

const RED: Color = Color::from_rgb(0.75, 0.0, 0.0);
const GREEN: Color = Color::from_rgb(0.0, 0.75, 0.0);

fn main() {
    TypingTest::run(Settings::default());
}

fn generate_line(words: &[String]) -> Vec<Word> {
    let mut total_chars = 0;
    let mut final_vec = Vec::new();

    loop {
        let index = fastrand::usize(..words.len());
        let word = &words[index];
        total_chars += word.len();
        if total_chars > MAX_LINE_LENGTH {
            break;
        }

        final_vec.push(word.into());
    }

    final_vec
}

struct TypingTest {
    state: TestState,
    test_start: Instant,
    selected_test_length: Option<TestLength>,
    test_length_secs: u32,
    remaining_time_secs: u32,
    word_pool: Vec<String>,
    current_word_pos: usize,
    current_word: String,
    current_line: Vec<Word>,
    next_line: Vec<Word>,
    stats: Stats,
    display_current_wpm: bool,
    display_timer: bool,
    text_input: text_input::State,
    wpm_button: button::State,
    timer_button: button::State,
    retry_button: button::State,
}

impl Default for TypingTest {
    fn default() -> Self {
        let word_pool = word_pool::default_word_pool();
        let current_line = generate_line(&word_pool);
        let next_line = generate_line(&word_pool);

        TypingTest {
            word_pool,
            current_line,
            next_line,
            state: TestState::Inactive,
            test_start: Instant::now(),
            selected_test_length: Some(TestLength::Length(TEST_TIME_SECS)),
            test_length_secs: TEST_TIME_SECS,
            remaining_time_secs: TEST_TIME_SECS,
            current_word_pos: 0,
            current_word: String::new(),
            stats: Stats::default(),
            display_current_wpm: true,
            display_timer: true,
            text_input: text_input::State::default(),
            wpm_button: button::State::default(),
            timer_button: button::State::default(),
            retry_button: button::State::default(),
        }
    }
}

impl TypingTest {
    pub fn with_test_length(length_secs: u32) -> TypingTest {
        TypingTest {
            selected_test_length: Some(TestLength::Length(length_secs)),
            test_length_secs: length_secs,
            remaining_time_secs: length_secs,
            ..TypingTest::default()
        }
    }

    pub fn submit_word(&mut self, word: &str) {
        let actual_word = &mut self.current_line[self.current_word_pos];

        actual_word.state = if actual_word.word == word {
            self.stats.correct(word);
            WordState::Correct
        } else {
            self.stats.incorrect(word);
            WordState::Incorrect
        };

        self.current_word_pos += 1;
        if self.current_word_pos == self.current_line.len() {
            self.current_word_pos = 0;
            self.current_line = self.next_line.clone();
            self.next_line = generate_line(&self.word_pool);
        }
    }
}

#[derive(PartialEq)]
enum TestState {
    Inactive,
    Active,
    Complete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TestLength {
    Length(u32),
}

#[derive(Clone)]
struct Word {
    word: String,
    state: WordState,
}

impl<S: Into<String>> From<S> for Word {
    fn from(word: S) -> Self {
        Word {
            word: word.into(),
            state: WordState::NotTyped,
        }
    }
}

impl From<&Word> for Text {
    fn from(word: &Word) -> Self {
        let color = match word.state {
            WordState::NotTyped => Color::BLACK,
            WordState::Correct => GREEN,
            WordState::Incorrect => RED,
        };

        Text::new(word.word.clone()).color(color)
    }
}

#[derive(Copy, Clone)]
enum WordState {
    NotTyped,
    Correct,
    Incorrect,
}

#[derive(Default)]
struct Stats {
    pub correct_chars: u32,
    pub correct_words: u32,
    pub incorrect_chars: u32,
    pub incorrect_words: u32,
}

impl Stats {
    pub fn correct(&mut self, word: &str) {
        // NOTE: +1 for chars due to spacebar
        self.correct_chars += word.len() as u32 + 1;
        self.correct_words += 1;
    }

    pub fn incorrect(&mut self, word: &str) {
        // NOTE: +1 for chars due to spacebar
        self.incorrect_chars += word.len() as u32 + 1;
        self.incorrect_words += 1;
    }

    pub fn current_wpm(&self, remaining_secs: u32, total_secs: u32) -> u32 {
        let elapsed_mins = (total_secs - remaining_secs) as f32 / 60.0;
        let unnormalised = self.correct_chars as f32 / 5.0;
        (unnormalised / elapsed_mins) as u32
    }

    pub fn final_wpm(&self, total_secs: u32) -> u32 {
        let mins = total_secs as f32 / 60.0;
        ((self.correct_chars as f32 / 5.0) / mins) as u32
    }

    pub fn accuracy(&self) -> f32 {
        let total_chars = self.correct_chars + self.incorrect_chars;
        if total_chars == 0 {
            return 100.0;
        }

        let total_chars = total_chars as f32;
        self.correct_chars as f32 / total_chars * 100.0
    }
}

#[derive(Debug, Clone)]
enum UIMessage {
    Reset,
    TimerTick(Instant),
    InputChanged(String),
    TestLengthChanged(TestLength),
    ToggleCurrentWPM,
    ToggleTimer,
}

impl Application for TypingTest {
    type Executor = executor::Default;
    type Message = UIMessage;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (TypingTest::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Typing Test")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            UIMessage::Reset => *self = TypingTest::with_test_length(self.test_length_secs),
            UIMessage::TimerTick(last_tick) => {
                self.remaining_time_secs = self.test_length_secs
                    - last_tick
                        .checked_duration_since(self.test_start)
                        .unwrap_or_default()
                        .as_secs() as u32;

                if self.remaining_time_secs == 0 {
                    self.state = TestState::Complete;
                }
            }
            UIMessage::InputChanged(mut s) => {
                match self.state {
                    TestState::Inactive => {
                        self.test_start = Instant::now();
                        self.state = TestState::Active;
                    }
                    TestState::Complete => s.truncate(0),
                    TestState::Active => {
                        // If spacebar was pressed and the word isn't just whitespace, submit this string
                        if s.ends_with(' ') {
                            let trimmed = s.trim_end();
                            if !trimmed.is_empty() {
                                self.submit_word(trimmed);
                            }

                            s.truncate(0);
                        }
                    }
                }

                self.current_word = s;
            }
            UIMessage::TestLengthChanged(length) => match length {
                TestLength::Length(secs) => *self = TypingTest::with_test_length(secs),
            },
            UIMessage::ToggleCurrentWPM => self.display_current_wpm = !self.display_current_wpm,
            UIMessage::ToggleTimer => self.display_timer = !self.display_timer,
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        const TICK_DURATION: Duration = Duration::from_millis(100);

        match self.state {
            TestState::Inactive | TestState::Complete => Subscription::none(),
            TestState::Active => time::every(TICK_DURATION).map(UIMessage::TimerTick),
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        fn format_mm_ss(secs: u32) -> String {
            format!("{}:{:0>2}", secs / 60, secs % 60)
        }

        let title = Text::new("Typing Test").size(40);

        let current_line = self
            .current_line
            .iter()
            .map(Text::from)
            .fold(Row::new().spacing(5), |row, word| row.push(word));

        let next_line = self
            .next_line
            .iter()
            .map(Text::from)
            .fold(Row::new().spacing(5), |row, word| row.push(word));

        let line_display = Column::new()
            .spacing(5)
            .push(current_line)
            .push(next_line)
            .width(Length::Fill);

        let typing_area = TextInput::new(
            &mut self.text_input,
            "",
            &self.current_word,
            UIMessage::InputChanged,
        )
        .padding(5);

        let current_wpm = if self.display_current_wpm {
            format!(
                "{} WPM",
                self.stats
                    .current_wpm(self.remaining_time_secs, self.test_length_secs)
            )
        } else {
            String::from(" ")
        };

        let wpm_button = Button::new(
            &mut self.wpm_button,
            Text::new(current_wpm).horizontal_alignment(HorizontalAlignment::Center),
        )
        .min_width(100)
        .on_press(UIMessage::ToggleCurrentWPM);

        let timer_text = if self.display_timer {
            format_mm_ss(self.remaining_time_secs)
        } else {
            String::from(" ")
        };

        let timer = Button::new(
            &mut self.timer_button,
            Text::new(timer_text).horizontal_alignment(HorizontalAlignment::Center),
        )
        .min_width(75)
        .on_press(UIMessage::ToggleTimer);

        let retry = Button::new(
            &mut self.retry_button,
            Text::new("Retry").horizontal_alignment(HorizontalAlignment::Center),
        )
        .min_width(75)
        .on_press(UIMessage::Reset);

        let typing_display = Row::new()
            .spacing(10)
            .align_items(Align::Center)
            .push(typing_area)
            .push(wpm_button)
            .push(timer)
            .push(retry);

        let test_lengths = Row::new()
            .spacing(10)
            .push(Radio::new(
                TestLength::Length(10),
                "10 secs",
                self.selected_test_length,
                UIMessage::TestLengthChanged,
            ))
            .push(Radio::new(
                TestLength::Length(30),
                "30 secs",
                self.selected_test_length,
                UIMessage::TestLengthChanged,
            ))
            .push(Radio::new(
                TestLength::Length(60),
                "1 min",
                self.selected_test_length,
                UIMessage::TestLengthChanged,
            ))
            .push(Radio::new(
                TestLength::Length(120),
                "2 mins",
                self.selected_test_length,
                UIMessage::TestLengthChanged,
            ))
            .push(Radio::new(
                TestLength::Length(300),
                "5 mins",
                self.selected_test_length,
                UIMessage::TestLengthChanged,
            ));

        let mut main_view = Column::new()
            .spacing(20)
            .align_items(Align::Center)
            .max_width(600)
            .push(title)
            .push(line_display)
            .push(typing_display)
            .push(test_lengths);

        // Show statistics if the test is completed
        if self.state == TestState::Complete {
            let wpm = Text::new(format!(
                "{} WPM",
                self.stats.final_wpm(self.test_length_secs)
            ))
            .size(30);

            let correct_chars_label = Text::new("Correct Characters:");
            let correct_chars = Text::new(self.stats.correct_chars.to_string()).color(GREEN);

            let correct_words_label = Text::new("Correct Words:");
            let correct_words = Text::new(self.stats.correct_words.to_string()).color(GREEN);

            let incorrect_chars_label = Text::new("Incorrect Characters:");
            let incorrect_chars = Text::new(self.stats.incorrect_chars.to_string()).color(RED);

            let incorrect_words_label = Text::new("Incorrect Words:");
            let incorrect_words = Text::new(self.stats.incorrect_words.to_string()).color(RED);

            let accuracy_label = Text::new("Accuracy:");
            let accuracy = Text::new(format!("{:.2}%", self.stats.accuracy()));

            let labels = Column::new()
                .align_items(Align::End)
                .spacing(10)
                .push(correct_chars_label)
                .push(incorrect_chars_label)
                .push(correct_words_label)
                .push(incorrect_words_label)
                .push(accuracy_label);

            let values = Column::new()
                .align_items(Align::Start)
                .spacing(10)
                .push(correct_chars)
                .push(incorrect_chars)
                .push(correct_words)
                .push(incorrect_words)
                .push(accuracy);

            let stats_breakdown = Row::new().spacing(10).push(labels).push(values);

            let stats_display = Column::new()
                .align_items(Align::Center)
                .spacing(20)
                .push(wpm)
                .push(stats_breakdown);

            main_view = main_view.push(stats_display);
        }

        Container::new(main_view)
            .padding(10)
            .center_x()
            .center_y()
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}
