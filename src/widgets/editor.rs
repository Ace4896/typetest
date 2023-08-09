use iced::{
    advanced::{self, Widget},
    event,
    keyboard::{self, KeyCode},
    widget, Event, Theme,
};

/// A wrapper around `TextInput` which intercepts word submissions and enables certain hotkeys.
pub struct InputEditor<'a, Message, Renderer = iced::Renderer<Theme>>
where
    Renderer: advanced::text::Renderer,
    Renderer::Theme: widget::text_input::StyleSheet,
{
    text_input: widget::TextInput<'a, Message, Renderer>,

    on_submit_word: Box<dyn Fn() -> Message + 'a>,
    on_redo: Box<dyn Fn() -> Message + 'a>,
}

impl<'a, Message, Renderer> InputEditor<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: advanced::text::Renderer,
    Renderer::Theme: widget::text_input::StyleSheet,
{
    /// Creates a new `InputEditor`.
    pub fn new(
        placeholder: &str,
        value: &str,
        on_submit_word: Box<dyn Fn() -> Message + 'a>,
        on_redo: Box<dyn Fn() -> Message + 'a>,
    ) -> Self {
        Self {
            text_input: widget::text_input(placeholder, value),
            on_submit_word,
            on_redo,
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for InputEditor<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: advanced::text::Renderer,
    Renderer::Theme: widget::text_input::StyleSheet,
{
    #[inline]
    fn width(&self) -> iced::Length {
        (&self.text_input as &dyn Widget<Message, Renderer>).width()
    }

    #[inline]
    fn height(&self) -> iced::Length {
        (&self.text_input as &dyn Widget<Message, Renderer>).height()
    }

    #[inline]
    fn layout(
        &self,
        renderer: &Renderer,
        limits: &advanced::layout::Limits,
    ) -> advanced::layout::Node {
        (&self.text_input as &dyn Widget<Message, Renderer>).layout(renderer, limits)
    }

    #[inline]
    fn draw(
        &self,
        state: &advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &advanced::renderer::Style,
        layout: advanced::Layout<'_>,
        cursor: advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        (&self.text_input as &dyn Widget<Message, Renderer>)
            .draw(state, renderer, theme, style, layout, cursor, viewport)
    }

    fn on_event(
        &mut self,
        state: &mut advanced::widget::Tree,
        event: Event,
        layout: advanced::Layout<'_>,
        cursor: advanced::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn advanced::Clipboard,
        shell: &mut advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) -> event::Status {
        match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key_code,
                modifiers,
            }) if (key_code == KeyCode::R && modifiers.command()) || key_code == KeyCode::F5 => {
                shell.publish((self.on_redo)());
                event::Status::Captured
            }
            Event::Keyboard(keyboard::Event::CharacterReceived(' ')) => {
                shell.publish((self.on_submit_word)());
                event::Status::Captured
            }
            _ => Widget::<Message, Renderer>::on_event(
                &mut self.text_input,
                state,
                event,
                layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            ),
        }
    }
}
