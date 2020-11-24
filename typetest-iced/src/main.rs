#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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

/// Represents the current state for a typing test.
struct TypingTestState {} // TODO: widget states, other info that needs to be stored here

/// Represents the current state for a results page.
struct ResultsState {} // TODO: widget states, other info that needs to be stored here

/// Represents the main state of the application.
struct TypeTest {
    current_page: Page,
    word_gen: Box<dyn WordGenerator>,
    current_stats: TestStatistics,
}

fn main() -> Result<(), iced::Error> {
    todo!()
}
