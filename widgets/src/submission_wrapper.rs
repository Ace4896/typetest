use iced_native::{
    event::Status,
    keyboard::{self, KeyCode, Modifiers},
    widget::TextInput,
    Element, Event, Widget,
};

/// A wrapper widget for [`TextInput`] which emits the following messages for these keybinds:
/// 
/// - Submit:
///   - Spacebar
///   - Enter
/// - Redo:
///   - Ctrl + R (or Cmd + R on Mac OS)
///   - F5
pub struct SubmissionWrapper<'a, Message, Renderer>
where
    Renderer: iced_native::text::Renderer,
    Renderer::Theme: iced_style::text_input::StyleSheet,
{
    current_value: &'a str,
    text_input: TextInput<'a, Message, Renderer>,

    pressed_keybind: bool,
    on_submit: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_redo: Option<Box<dyn Fn() -> Message + 'a>>,
}

impl<'a, Message, Renderer> SubmissionWrapper<'a, Message, Renderer>
where
    Renderer: iced_native::text::Renderer,
    Renderer::Theme: iced_style::text_input::StyleSheet,
{
    /// Creates a new [`SubmissionWrapper`] with the specified [`TextInput`] and current text value.
    pub fn new(text_input: TextInput<'a, Message, Renderer>, current_value: &'a str) -> Self {
        Self {
            text_input,
            current_value,

            pressed_keybind: false,
            on_submit: None,
            on_redo: None,
        }
    }

    /// Specifies the message callback to use when a submit keybind is input.
    pub fn on_submit<F>(mut self, callback: F) -> Self
    where
        F: 'a + Fn(String) -> Message,
    {
        self.on_submit = Some(Box::new(callback));
        self
    }

    /// Specifies the message callback to use when a redo keybind is input.
    pub fn on_redo<F>(mut self, callback: F) -> Self
    where
        F: 'a + Fn() -> Message,
    {
        self.on_redo = Some(Box::new(callback));
        self
    }

    fn is_submit(&self, key_code: KeyCode, modifiers: Modifiers) -> bool {
        (key_code == KeyCode::Space && modifiers.is_empty())
            || (key_code == KeyCode::Enter && modifiers.is_empty())
    }

    fn is_redo(&self, key_code: KeyCode, modifiers: Modifiers) -> bool {
        (key_code == KeyCode::F5 && modifiers.is_empty())
            || (key_code == KeyCode::R && modifiers.command())
    }
}

impl<'a, Message, Renderer> From<SubmissionWrapper<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced_native::text::Renderer,
    Renderer::Theme: iced_style::text_input::StyleSheet,
{
    fn from(text_input: SubmissionWrapper<'a, Message, Renderer>) -> Self {
        Element::new(text_input)
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for SubmissionWrapper<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced_native::text::Renderer,
    Renderer::Theme: iced_style::text_input::StyleSheet,
{
    fn on_event(
        &mut self,
        state: &mut iced_native::widget::Tree,
        event: Event,
        layout: iced_native::Layout<'_>,
        cursor_position: iced_native::Point,
        renderer: &Renderer,
        clipboard: &mut dyn iced_native::Clipboard,
        shell: &mut iced_native::Shell<'_, Message>,
    ) -> Status {
        // In iced 0.9, keyboard events are sent in this order when a character key is pressed:
        //
        // 1. KeyPressed
        // 2. CharacterReceived
        // 3. KeyReleased
        //
        // The idea we only forward events to the underlying text input when no keybinds are pressed.

        match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key_code,
                modifiers,
            }) => {
                if let Some(on_submit) = &self.on_submit {
                    if self.is_submit(key_code, modifiers) {
                        self.pressed_keybind = true;
                        shell.publish(on_submit(self.current_value.to_string()));
                    }
                }

                if let Some(on_redo) = &self.on_redo {
                    if self.is_redo(key_code, modifiers) {
                        self.pressed_keybind = true;
                        shell.publish(on_redo());
                    }
                }
            }

            Event::Keyboard(keyboard::Event::KeyReleased {
                key_code,
                modifiers,
            }) => {
                if (self.on_submit.is_some() && self.is_submit(key_code, modifiers))
                    || (self.on_redo.is_some() && self.is_redo(key_code, modifiers))
                {
                    self.pressed_keybind = false;
                }
            }

            _ => {}
        }

        if self.pressed_keybind {
            Status::Captured
        } else {
            self.text_input.on_event(
                state,
                event,
                layout,
                cursor_position,
                renderer,
                clipboard,
                shell,
            )
        }
    }

    fn width(&self) -> iced_native::Length {
        (&self.text_input as &dyn Widget<Message, Renderer>).width()
    }

    fn height(&self) -> iced_native::Length {
        (&self.text_input as &dyn Widget<Message, Renderer>).height()
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
        state: &iced_native::widget::Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as iced_native::Renderer>::Theme,
        style: &iced_native::renderer::Style,
        layout: iced_native::Layout<'_>,
        cursor_position: iced_native::Point,
        viewport: &iced_native::Rectangle,
    ) {
        (&self.text_input as &dyn Widget<Message, Renderer>).draw(
            state,
            renderer,
            theme,
            style,
            layout,
            cursor_position,
            viewport,
        )
    }

    fn tag(&self) -> iced_native::widget::tree::Tag {
        self.text_input.tag()
    }

    fn state(&self) -> iced_native::widget::tree::State {
        self.text_input.state()
    }

    fn children(&self) -> Vec<iced_native::widget::Tree> {
        self.text_input.children()
    }

    fn diff(&self, tree: &mut iced_native::widget::Tree) {
        self.text_input.diff(tree)
    }

    fn operate(
        &self,
        state: &mut iced_native::widget::Tree,
        layout: iced_native::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced_native::widget::Operation<Message>,
    ) {
        self.text_input.operate(state, layout, renderer, operation)
    }

    fn mouse_interaction(
        &self,
        state: &iced_native::widget::Tree,
        layout: iced_native::Layout<'_>,
        cursor_position: iced_native::Point,
        viewport: &iced_native::Rectangle,
        renderer: &Renderer,
    ) -> iced_native::mouse::Interaction {
        self.text_input
            .mouse_interaction(state, layout, cursor_position, viewport, renderer)
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut iced_native::widget::Tree,
        layout: iced_native::Layout<'_>,
        renderer: &Renderer,
    ) -> Option<iced_native::overlay::Element<'b, Message, Renderer>> {
        self.text_input.overlay(state, layout, renderer)
    }
}
