use std::time::Instant;

use iced::{button, text_input, Align, Button, Row, Text, TextInput};

use typetest_core::{stats::TestStatistics, word_gen::WordGenerator};

use crate::AppMessage;

/// Represents the current state for a typing test.
pub(crate) struct TypingTestState {
    word_gen: Box<dyn WordGenerator>,
    pub(crate) current_stats: TestStatistics,
    pub(crate) status: TypingTestStatus,

    current_input: String,
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
    pub(crate) fn update(&mut self, message: &TypingTestMessage) {
        match message {
            TypingTestMessage::TimerTick(_) => todo!(),
            TypingTestMessage::InputChanged(_) => todo!(),
            TypingTestMessage::StatusChanged(_) => todo!(),
            TypingTestMessage::ToggleWPMDisplay => self.show_wpm = !self.show_wpm,
            TypingTestMessage::ToggleTimerDisplay => self.show_timer = !self.show_timer,
            TypingTestMessage::Retry => {
                self.current_input.clear();
                self.remaining_seconds = 60;    // TODO: Configurable version of this
                
            },
            TypingTestMessage::NextTest => todo!(),
        }
    }
}

/// Represents the possible messages that could be sent during a typing test.
#[derive(Clone)]
pub(crate) enum TypingTestMessage {
    TimerTick(Instant),
    InputChanged(String),
    StatusChanged(TypingTestStatus),
    ToggleWPMDisplay,
    ToggleTimerDisplay,
    Retry,
    NextTest,
}

/// Represents the different statuses a typing test could be in.
#[derive(Copy, Clone, PartialEq)]
pub(crate) enum TypingTestStatus {
    NotStarted,
    Started,
    Finished,
}

/// Builds the typing test widget.
pub(crate) fn typing_test_widget(test_state: &mut TypingTestState) -> iced::Element<AppMessage> {
    let input_box = TextInput::new(
        &mut test_state.input_box,
        "",
        &test_state.current_input,
        |s| AppMessage::TypingTest(TypingTestMessage::InputChanged(s)),
    );

    let wpm_text = if test_state.show_wpm {
        format!("{} WPM", test_state.current_stats.effective_wpm)
    } else {
        String::from(" ")
    };

    let wpm_button = Button::new(&mut test_state.wpm_button, Text::new(wpm_text))
        .min_width(100)
        .on_press(AppMessage::TypingTest(
            TypingTestMessage::ToggleTimerDisplay,
        ));

    let timer_text = if test_state.show_timer {
        format!(
            "{:0>2}:{:0>2}",
            test_state.remaining_seconds / 60,
            test_state.remaining_seconds % 60
        )
    } else {
        String::from(" ")
    };

    let timer_button = Button::new(&mut test_state.timer_button, Text::new(timer_text))
        .min_width(100)
        .on_press(AppMessage::TypingTest(TypingTestMessage::ToggleWPMDisplay));

    let redo_button = Button::new(&mut test_state.redo_button, Text::new("Redo"))
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
