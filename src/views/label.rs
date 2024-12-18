use crate::math::Vec2;
use crate::{
    core::{
        constraints::{Constraint, Constraints},
        context::{Context, ContextMut},
        interaction::Interaction,
        layout::Layout,
        view::View,
        view_tree::{ViewDrawer, ViewInteractor, ViewSizer},
    },
    graphics::{
        color::Color,
        painter::Painter,
        text::{Text, TextStyle},
    },
};
use std::rc::Rc;

#[derive(PartialEq)]
pub struct Label {
    text: String,
    size: f32,
    color: Color,
}

pub fn label(text: impl Into<String>) -> Label {
    Label {
        text: text.into(),
        size: 12.0,
        color: Color::BLACK,
    }
}

impl Label {
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl View for Label {
    fn build(&self, context: &mut Context) -> Vec<Rc<dyn View>> {
        let text = Text::new(
            &self.text,
            TextStyle {
                size: self.size,
                color: self.color,
            },
        );
        vec![Rc::new(TextView { text })]
    }

    fn size(&self, constraints: Constraints, children: &[ViewSizer]) -> Vec2 {
        children[0].size(constraints)
    }

    fn layout(&self, layout: Layout, children: &[ViewSizer]) -> Vec<Layout> {
        vec![Layout {
            position: Vec2::ZERO,
            size: layout.size,
        }]
    }

    fn draw(&self, layout: Layout, painter: &mut Painter, children: &[ViewDrawer]) {
        painter.translate(layout.position, |painter| {
            children[0].draw(painter);
        });
    }

    fn interact(
        &self,
        context: &mut ContextMut,
        layout: Layout,
        interaction: Interaction,
        consumed: bool,
        children: &[ViewInteractor],
    ) -> bool {
        false
    }
}

struct TextView {
    text: Text,
}

impl PartialEq for TextView {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl View for TextView {
    fn build(&self, context: &mut Context) -> Vec<Rc<dyn View>> {
        Vec::new()
    }

    fn size(&self, constraints: Constraints, children: &[ViewSizer]) -> Vec2 {
        self.text.size(match constraints.width {
            Constraint::Ideal => f32::INFINITY,
            Constraint::Min => 0.0,
            Constraint::Max => f32::INFINITY,
            Constraint::Fixed(width) => width,
        })
    }

    fn layout(&self, layout: Layout, children: &[ViewSizer]) -> Vec<Layout> {
        Vec::new()
    }

    fn draw(&self, layout: Layout, painter: &mut Painter, children: &[ViewDrawer]) {
        painter.draw_paragraph(&self.text, layout.position, layout.size.x);
    }

    fn interact(
        &self,
        context: &mut ContextMut,
        layout: Layout,
        interaction: Interaction,
        consumed: bool,
        children: &[ViewInteractor],
    ) -> bool {
        false
    }
}
