use crate::ApplicationTheme;

#[derive(Clone, Copy, Debug)]
pub struct DefaultLight;

impl ApplicationTheme for DefaultLight {
    fn button(&self) -> Box<dyn iced_style::button::StyleSheet> {
        Default::default()
    }

    fn container(&self) -> Box<dyn iced_style::container::StyleSheet> {
        Default::default()
    }

    fn pick_list(&self) -> Box<dyn iced_style::pick_list::StyleSheet> {
        Default::default()
    }

    fn radio(&self) -> Box<dyn iced_style::radio::StyleSheet> {
        Default::default()
    }

    fn scrollable(&self) -> Box<dyn iced_style::scrollable::StyleSheet> {
        Default::default()
    }

    fn text_input(&self) -> Box<dyn iced_style::text_input::StyleSheet> {
        Default::default()
    }
}
