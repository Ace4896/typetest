use iced::{pick_list, Align, Column, Command, Element, PickList, Row, Text};
use typetest_themes::ApplicationTheme;

use crate::{config::RandomGeneratorSettings, views::Action};

use super::SettingsMessage;

/// Represents a message specific to the random generator settings view.
#[derive(Clone, Debug)]
pub enum RandomGeneratorMessage {
    TimeLengthChanged(u64),
    Action(Action),
}

impl From<RandomGeneratorMessage> for SettingsMessage {
    #[inline]
    fn from(message: RandomGeneratorMessage) -> Self {
        if let RandomGeneratorMessage::Action(a) = message {
            SettingsMessage::Action(a)
        } else {
            SettingsMessage::RandomGenerator(message)
        }
    }
}

#[derive(Debug)]
pub struct RandomGeneratorState {
    time_length_seconds: u64,
    time_length_pick_list: pick_list::State<u64>,
}

impl RandomGeneratorState {
    pub fn new(settings: &RandomGeneratorSettings) -> RandomGeneratorState {
        RandomGeneratorState {
            time_length_seconds: settings.time_length_seconds,
            time_length_pick_list: pick_list::State::default(),
        }
    }

    pub fn update(&mut self, message: RandomGeneratorMessage) -> Command<RandomGeneratorMessage> {
        match message {
            RandomGeneratorMessage::TimeLengthChanged(time) => {
                self.time_length_seconds = time;
                Command::perform(async move { time }, |time| {
                    RandomGeneratorMessage::Action(Action::ChangeTimeLength(time))
                })
            }
            _ => Command::none(),
        }
    }

    /// Builds the widget for random generator settings.
    pub fn view(&mut self, theme: &Box<dyn ApplicationTheme>) -> Element<RandomGeneratorMessage> {
        const TIME_OPTIONS: [u64; 5] = [10, 30, 60, 120, 300];

        let title = Text::new("Random Generator Settings").size(28);

        let time_length_label = Text::new("Test Length (Time):");
        let time_length_pick_list = PickList::new(
            &mut self.time_length_pick_list,
            &TIME_OPTIONS[..],
            Some(self.time_length_seconds),
            RandomGeneratorMessage::TimeLengthChanged,
        )
        .style(theme);

        let time_length = Row::new()
            .align_items(Align::Center)
            .spacing(10)
            .push(time_length_label)
            .push(time_length_pick_list);

        Column::new()
            .spacing(10)
            .push(title)
            .push(time_length)
            .into()
    }
}
