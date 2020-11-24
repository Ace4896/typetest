use std::time::{Duration, Instant};

use iced::{
    button, text_input, time, Align, Button, Command, Element, Row, Subscription, Text, TextInput,
};

use typetest_core::{
    stats::TestStatistics,
    word_gen::{random::RandomWordGenerator, WordGenerator},
};

use crate::AppMessage;

/// Represents the possible messages that could be sent during a typing test.
#[derive(Clone, Debug)]
pub(crate) enum TypingTestMessage {
    TimerTick(Instant),
    InputChanged(String),
    ToggleWPMDisplay,
    ToggleTimerDisplay,
    Retry,
    NextTest,
}

/// Represents the different statuses a typing test could be in.
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum TypingTestStatus {
    NotStarted,
    Started,
    Finished,
}

/// Represents the current state for a typing test.
pub(crate) struct TypingTestState {
    word_gen: Box<dyn WordGenerator>,
    pub(crate) current_stats: TestStatistics,
    pub(crate) status: TypingTestStatus,

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
}

impl TypingTestState {
    pub(crate) fn new() -> TypingTestState {
        TypingTestState {
            word_gen: Box::new(RandomWordGenerator::default()),
            current_stats: TestStatistics::default(),
            status: TypingTestStatus::NotStarted,

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
        }
    }

    /// Handles updates for the typing test screen.
    pub(crate) fn update(&mut self, message: TypingTestMessage) -> Command<AppMessage> {
        match message {
            TypingTestMessage::TimerTick(i) => {
                if self.status != TypingTestStatus::Started {
                    return Command::none();
                }

                let diff = i - self.test_start;
                self.remaining_seconds = self.test_length_seconds - diff.as_secs();

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

                if s.ends_with(' ') {
                    let trimmed = s.trim();
                    if !trimmed.is_empty() {
                        // TODO: Word list
                        dbg!(trimmed);
                        self.current_stats.submit_word(trimmed, trimmed);
                    }

                    s.clear();
                }

                self.current_input = s;
            }
            TypingTestMessage::ToggleWPMDisplay => self.show_wpm = !self.show_wpm,
            TypingTestMessage::ToggleTimerDisplay => self.show_timer = !self.show_timer,
            TypingTestMessage::Retry => {
                self.status = TypingTestStatus::NotStarted;
                self.word_gen.prepare_for_retry();
                self.current_stats.reset();
                self.current_input.clear();
                self.remaining_seconds = self.test_length_seconds;
            }
            TypingTestMessage::NextTest => {
                self.status = TypingTestStatus::NotStarted;
                self.word_gen.prepare_for_next_test();
                self.current_stats.reset();
                self.current_input.clear();
                self.remaining_seconds = self.test_length_seconds;
            }
        }

        Command::none()
    }

    /// Creates the view for the current `TypingTestState`.
    pub(crate) fn view(&mut self) -> Element<AppMessage> {
        if self.status == TypingTestStatus::Finished {
            self.results_widget()
        } else {
            self.typing_test_widget()
        }
    }

    /// Handles subscriptions for the typing test screen.
    pub(crate) fn subscription(&self) -> Subscription<AppMessage> {
        const TICK_DURATION: Duration = Duration::from_millis(100);

        match self.status {
            TypingTestStatus::NotStarted | TypingTestStatus::Finished => Subscription::none(),
            TypingTestStatus::Started => time::every(TICK_DURATION)
                .map(|i| AppMessage::TypingTest(TypingTestMessage::TimerTick(i))),
        }
    }

    /// Builds the typing test widget.
    fn typing_test_widget(&mut self) -> Element<AppMessage> {
        let input_box = TextInput::new(
            &mut self.input_box,
            "",
            &self.current_input,
            |s| AppMessage::TypingTest(TypingTestMessage::InputChanged(s)),
        );

        let wpm_text = if self.show_wpm {
            format!("{} WPM", self.current_stats.effective_wpm)
        } else {
            String::from(" ")
        };

        let wpm_button = Button::new(&mut self.wpm_button, Text::new(wpm_text))
            .min_width(100)
            .on_press(AppMessage::TypingTest(
                TypingTestMessage::ToggleTimerDisplay,
            ));

        let timer_text = if self.show_timer {
            format!(
                "{:0>2}:{:0>2}",
                self.remaining_seconds / 60,
                self.remaining_seconds % 60
            )
        } else {
            String::from(" ")
        };

        let timer_button = Button::new(&mut self.timer_button, Text::new(timer_text))
            .min_width(100)
            .on_press(AppMessage::TypingTest(TypingTestMessage::ToggleWPMDisplay));

        let redo_button = Button::new(&mut self.redo_button, Text::new("Redo"))
            .min_width(100)
            .on_press(AppMessage::TypingTest(TypingTestMessage::NextTest));

        let typing_area = Row::new()
            .spacing(10)
            .align_items(Align::Center)
            .push(input_box)
            .push(wpm_button)
            .push(timer_button)
            .push(redo_button);

        typing_area.into()
    }

    /// Builds the results widget.
    fn results_widget(&mut self) -> Element<AppMessage> {
        Text::new("Results").into()
    }
}
