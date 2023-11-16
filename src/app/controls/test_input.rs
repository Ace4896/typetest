use iced::{
    advanced::{layout, mouse, renderer, widget, Layout, Widget, text},
    Color, Element, Length, Rectangle, Size, widget::text::LineHeight,
};

pub struct TestInput<Renderer>
where
    Renderer: text::Renderer
{
    current_line: Vec<String>,

    font: Renderer::Font,
    font_size: Option<f32>,
    line_height: LineHeight,
}

impl<Message, Renderer> Widget<Message, Renderer> for TestInput<Renderer>
where
    Renderer: text::Renderer,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        iced

        let current_line = self.current_line.join(" ").to_string();
        let font_size = self.font_size.unwrap_or_else(|| renderer.default_size());

        renderer.measure(&current_line, font_size, self.line_height, self.font, )

        todo!()
    }

    fn draw(
        &self,
        state: &widget::Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as iced::advanced::Renderer>::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        todo!()
    }
}

impl<'a, Message, Renderer> From<TestInput<Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: text::Renderer,
{
    fn from(test_input: TestInput<Renderer>) -> Self {
        Self::new(test_input)
    }
}
