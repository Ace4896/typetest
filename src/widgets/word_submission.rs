use iced::Point;
use iced_native::{
    event, keyboard, text_input, Clipboard, Element, Event, Layout, TextInput, Widget,
};
use keyboard::{KeyCode, Modifiers};

/// Native-only wrapper for word submission. It provides the following functionality:
/// - Intercept spacebar inputs, sending a message indicating that the current word is being submitted.
/// - Redo the test when one of the following hotkeys is used:
///   - Ctrl + R (or Cmd + R on Mac OS)
///   - F5
pub struct SubmissionWrapper<'a, Message, Renderer>
where
    Renderer: text_input::Renderer,
{
    text_input: TextInput<'a, Message, Renderer>,
    on_submit_word: Message,
    on_redo: Message,
}

impl<'a, Message, Renderer> SubmissionWrapper<'a, Message, Renderer>
where
    Renderer: text_input::Renderer,
{
    /// Creates a new [`SubmissionWrapper`].
    pub fn new(
        text_input: TextInput<'a, Message, Renderer>,
        on_submit_word: Message,
        on_redo: Message,
    ) -> Self {
        Self {
            text_input,
            on_submit_word,
            on_redo,
        }
    }
}

impl<'a, Message, Renderer> From<SubmissionWrapper<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + text_input::Renderer,
{
    fn from(widget: SubmissionWrapper<'a, Message, Renderer>) -> Self {
        Element::new(widget)
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for SubmissionWrapper<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: text_input::Renderer,
{
    fn width(&self) -> iced::Length {
        Widget::<Message, Renderer>::width(&self.text_input)
    }

    fn height(&self) -> iced::Length {
        Widget::<Message, Renderer>::height(&self.text_input)
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        self.text_input.layout(renderer, limits)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        viewport: &iced::Rectangle,
    ) -> Renderer::Output {
        Widget::<Message, Renderer>::draw(
            &self.text_input,
            renderer,
            defaults,
            layout,
            cursor_position,
            viewport,
        )
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        Widget::<Message, Renderer>::hash_layout(&self.text_input, state)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        messages: &mut Vec<Message>,
    ) -> event::Status {
        match event {
            Event::Keyboard(
                keyboard::Event::KeyPressed {
                    key_code: KeyCode::R,
                    modifiers: Modifiers { control: true, .. },
                }
                | keyboard::Event::KeyPressed {
                    key_code: KeyCode::F5,
                    ..
                },
            ) => {
                messages.push(self.on_redo.clone());
                event::Status::Captured
            }
            Event::Keyboard(keyboard::Event::CharacterReceived(' ')) => {
                messages.push(self.on_submit_word.clone());
                event::Status::Captured
            }
            _ => Widget::<Message, Renderer>::on_event(
                &mut self.text_input,
                event,
                layout,
                cursor_position,
                renderer,
                clipboard,
                messages,
            ),
        }
    }
}
