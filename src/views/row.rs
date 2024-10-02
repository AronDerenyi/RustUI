use std::rc::Rc;

use crate::{
    core::{Constraints, ContentBuilder, Context, Layout},
    View,
};
use macroquad::math::Vec2;

pub struct Row {
    spacing: f32,
    content: ContentBuilder,
}

impl Row {
    pub fn new(content: ContentBuilder) -> Self {
        Self {
            spacing: 0.0,
            content,
        }
    }

    pub fn spacing(self, spacing: f32) -> Self {
        Self { spacing, ..self }
    }
}

impl View for Row {
    fn get_children(&self, _ctx: &mut Context) -> Box<[Rc<dyn View>]> {
        self.content.build()
    }

    fn get_constraints(&self, child_constraints: &[Constraints]) -> Constraints {
        let mut constraints = Constraints {
            size: Vec2::default(),
        };
        for child_constraint in child_constraints {
            constraints.size = Vec2::new(
                constraints.size.x + child_constraint.size.x,
                constraints.size.y.max(child_constraint.size.y),
            );
        }
        constraints.size.x += self.spacing * (child_constraints.len() as f32 - 1.0).max(0.0);
        constraints
    }

    fn get_children_layouts(
        &self,
        layout: Layout,
        child_constraints: &[Constraints],
    ) -> Box<[Layout]> {
        let mut x = layout.position.x;
        let y = layout.position.y;
        let mut layouts = Vec::new();
        for child_constraint in child_constraints.iter() {
            let layout = Layout {
                position: Vec2::new(x, y),
                size: child_constraint.size,
            };
            layouts.push(layout);
            x += child_constraint.size.x + self.spacing;
        }
        layouts.into()
    }
}
