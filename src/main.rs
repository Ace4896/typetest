use std::time::{Duration, Instant};

use iced::{
    button, executor, text_input, time, Align, Application, Button, Color, Column, Command,
    Container, Length, Row, Settings, Subscription, Text, TextInput,
};

use rand::prelude::*;

mod word_pool;

const MAX_LINE_LENGTH: usize = 50;
const TEST_TIME_SECS: u32 = 60;

fn main() {
    TypingTest::run(Settings::default());
}

fn generate_line(words: &[String]) -> Vec<Word> {
    let mut total_chars = 0;
    let mut final_vec = Vec::new();

    loop {
        let word = words.choose(&mut thread_rng()).unwrap();
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
    remaining_time_secs: u32,
    word_pool: Vec<String>,
    current_word_pos: usize,
    current_word: String,
    current_line: Vec<Word>,
    next_line: Vec<Word>,
    stats: Stats,
    text_input: text_input::State,
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
            remaining_time_secs: TEST_TIME_SECS,
            current_word_pos: 0,
            current_word: String::new(),
            stats: Stats::default(),
            text_input: text_input::State::default(),
            retry_button: button::State::default(),
        }
    }
}

impl TypingTest {
    pub fn submit_word(&mut self, word: &str) {
        let actual_word = &mut self.current_line[self.current_word_pos];
        let char_count = word.len() as u32;

        actual_word.state = if actual_word.word == word {
            self.stats.correct_chars += char_count;
            self.stats.correct_words += 1;
            WordState::Correct
        } else {
            self.stats.incorrect_chars += char_count;
            self.stats.incorrect_words += 1;
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
        const RED: Color = Color::from_rgb(1.0, 0.0, 0.0);
        const GREEN: Color = Color::from_rgb(0.0, 1.0, 0.0);

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
    pub fn final_wpm(&self) -> u32 {
        self.correct_chars / 5
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
            UIMessage::Reset => *self = TypingTest::default(),
            UIMessage::TimerTick(last_tick) => {
                self.remaining_time_secs = TEST_TIME_SECS
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
                            if !s.trim_end().is_empty() {
                                self.submit_word(s.trim_end());
                            }

                            s.truncate(0);
                        }
                    }
                }

                self.current_word = s;
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        const TICK_DURATION: Duration = Duration::from_millis(10);

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

        let line_display = Column::new().spacing(5).push(current_line).push(next_line);

        let typing_area = TextInput::new(
            &mut self.text_input,
            "Type Here!",
            &self.current_word,
            UIMessage::InputChanged,
        )
        .padding(5)
        .width(Length::Units(200));

        let timer = Text::new(format_mm_ss(self.remaining_time_secs));
        let retry =
            Button::new(&mut self.retry_button, Text::new("Retry")).on_press(UIMessage::Reset);

        let typing_display = Row::new()
            .spacing(10)
            .align_items(Align::Center)
            .push(typing_area)
            .push(timer)
            .push(retry);

        let mut main_view = Column::new()
            .spacing(20)
            .align_items(Align::Center)
            .push(title)
            .push(line_display)
            .push(typing_display);

        // Show statistics if the test is completed
        // if self.state == TestState::Complete {
        let wpm = Text::new(format!("{} WPM", self.stats.final_wpm()));

        let correct_display = Text::new(format!(
            "Correct Words: {} ({ } Characters)",
            self.stats.correct_words, self.stats.correct_chars
        ));

        let incorrect_display = Text::new(format!(
            "Incorrect Words: {} ({ } Characters)",
            self.stats.incorrect_words, self.stats.incorrect_chars
        ));

        let accuracy_display = Text::new(format!("Accuracy: {:.2}%", self.stats.accuracy()));

        let stats_display = Column::new()
            .push(wpm)
            .push(correct_display)
            .push(incorrect_display)
            .push(accuracy_display);

        main_view = main_view.push(stats_display);
        // }

        Container::new(main_view)
            .padding(10)
            .center_x()
            .center_y()
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}
