use iced::Point;
use iced_native::{
    event, keyboard, text_input, Clipboard, Element, Event, Layout, TextInput, Widget,
};

/// Native-only wrapper for word submission.
/// It intercepts spacebar inputs and sends a message indicating that the current word is being submitted.
pub struct SubmissionWrapper<'a, Message, Renderer>
where
    Renderer: text_input::Renderer,
{
    text_input: TextInput<'a, Message, Renderer>,
    on_submit_word: Message,
}

impl<'a, Message, Renderer> SubmissionWrapper<'a, Message, Renderer>
where
    Renderer: text_input::Renderer,
{
    /// Creates a new [`SubmissionWrapper`].
    pub fn new(text_input: TextInput<'a, Message, Renderer>, on_submit_word: Message) -> Self {
        Self {
            text_input,
            on_submit_word,
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
