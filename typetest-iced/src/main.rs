#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod widgets;
use widgets::{results::ResultsState, typing_test::{TypingTestMessage, TypingTestState}};

use std::{
    mem,
    time::{Duration, Instant},
};

use iced::{
    button, executor, text_input, time, Align, Application, Button, Color, Column, Command,
    Container, HorizontalAlignment, Length, Radio, Row, Settings, Subscription, Text, TextInput,
};

use typetest_core::{
    stats::TestStatistics,
    word_gen::{random::RandomWordGenerator, DisplayedWord, WordGenerator},
};

/// Represents the different pages in the application.
enum Page {
    TypingTest(TypingTestState),
    Results(ResultsState),
}

/// Represents the main state of the application.
struct TypeTestApp {
    current_page: Page,
}

/// Top-level enum for the messages that can be sent in this application.
#[derive(Clone)]
enum AppMessage {
    TypingTest(TypingTestMessage),
}

impl From<TypingTestMessage> for AppMessage {
    fn from(message: TypingTestMessage) -> Self {
        AppMessage::TypingTest(message)
    }
}

fn main() -> Result<(), iced::Error> {
    todo!()
}
