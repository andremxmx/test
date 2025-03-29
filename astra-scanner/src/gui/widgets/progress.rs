use iced::{
    advanced::{
        layout, renderer, widget::Tree, Layout, Widget,
    },
    mouse,
    Color, Element, Length, Rectangle, Size,
};

use crate::gui::style::{ACCENT_BLUE, ACCENT_GREEN, ACCENT_RED};

pub struct ProgressBar {
    value: f32,
    width: Length,
    height: f32,
}

impl ProgressBar {
    pub fn new(value: f32) -> Self {
        Self {
            value: value.max(0.0).min(1.0),
            width: Length::Fill,
            height: 18.0,
        }
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: impl Into<f32>) -> Self {
        self.height = height.into();
        self
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for ProgressBar
where
    Renderer: renderer::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        Length::Fixed(self.height)
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width).height(Length::Fixed(self.height));
        let size = Size::new(limits.max().width, self.height);
        layout::Node::new(size)
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        _theme: &<Renderer as iced::advanced::Renderer>::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        
        // Draw background
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border_radius: [4.0, 4.0, 4.0, 4.0].into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            Color::from_rgb(0.2, 0.2, 0.2),
        );
        
        // Draw progress bar
        if self.value > 0.0 {
            let color = if self.value < 0.3 {
                ACCENT_RED
            } else if self.value < 0.7 {
                ACCENT_BLUE
            } else {
                ACCENT_GREEN
            };
            
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        width: bounds.width * self.value,
                        ..bounds
                    },
                    border_radius: [4.0, 4.0, 4.0, 4.0].into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
                color,
            );
        }
    }
}

impl<'a, Message, Renderer> From<ProgressBar>
    for Element<'a, Message, Renderer>
where
    Renderer: renderer::Renderer + 'a,
{
    fn from(progress_bar: ProgressBar) -> Self {
        Element::new(progress_bar)
    }
}

pub fn progress(value: f32) -> ProgressBar {
    ProgressBar::new(value)
} 