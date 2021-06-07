use iced::{button, scrollable, Align, Button, Column, HorizontalAlignment, Row, Scrollable, Text};
use typetest_core::stats::TestStats;
use typetest_themes::ApplicationTheme;

use super::Action;

/// Represents the state for the results view.
#[derive(Debug)]
pub struct ResultsState {
    // TODO: Instead of storing test length, store the settings used
    test_length_seconds: u64,
    stats: TestStats,

    show_missed_words: bool,

    retry_button: button::State,
    next_test_button: button::State,
    toggle_missed_button: button::State,
    missed_scrollable: scrollable::State,
}

/// Represents the messages used by the results view.
#[derive(Clone, Debug)]
pub enum ResultsMessage {
    ToggleMissedWords,
    Action(Action),
}

impl ResultsState {
    pub fn new() -> Self {
        Self {
            test_length_seconds: 60,
            stats: TestStats::new(),

            show_missed_words: false,

            retry_button: button::State::new(),
            next_test_button: button::State::new(),
            toggle_missed_button: button::State::new(),
            missed_scrollable: scrollable::State::new(),
        }
    }

    pub fn update(&mut self, message: ResultsMessage) -> iced::Command<ResultsMessage> {
        match message {
            ResultsMessage::ToggleMissedWords => self.show_missed_words = !self.show_missed_words,
            _ => {}
        }

        iced::Command::none()
    }

    pub fn view(&mut self, theme: &Box<dyn ApplicationTheme>) -> iced::Element<ResultsMessage> {
        let word_palette = theme.word_palette();
        let checkpoint = self
            .stats
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
        let test_length = Text::new(format_time_mm_ss(checkpoint.elapsed.as_secs()));

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
            &mut self.next_test_button,
            Text::new("Next Test").horizontal_alignment(HorizontalAlignment::Center),
        )
        .min_width(100)
        .style(theme)
        .on_press(ResultsMessage::Action(Action::NextTest));

        let retry_button = Button::new(
            &mut self.retry_button,
            Text::new("Retry").horizontal_alignment(HorizontalAlignment::Center),
        )
        .min_width(100)
        .style(theme)
        .on_press(ResultsMessage::Action(Action::RetryTest));

        let toggle_missed_button = {
            let tmp = Button::new(
                &mut self.toggle_missed_button,
                Text::new("Toggle Missed Words").horizontal_alignment(HorizontalAlignment::Center),
            )
            .style(theme);

            if self.stats.get_missed_words().is_empty() {
                tmp
            } else {
                tmp.on_press(ResultsMessage::ToggleMissedWords)
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
            let missed_words = self.stats.get_missed_words();
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
                    Scrollable::new(&mut self.missed_scrollable)
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

    pub fn update_stats(&mut self, stats: TestStats) {
        self.stats = stats;
    }
}

/// Formats the provided number of seconds into the mm:ss format.
#[inline]
fn format_time_mm_ss(seconds: u64) -> String {
    format!("{:0>2}:{:0>2}", seconds / 60, seconds % 60)
}
