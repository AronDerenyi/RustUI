use std::rc::Rc;

use crate::core::{Constraints, Context, Layout, Shape, View, ViewBuilder};
use macroquad::color::Color;

#[derive(PartialEq)]
pub struct Background {
    color: Color,
    view: ViewBuilder,
}

pub trait Backgroundable: View + Sized {
    fn background(self, color: Color) -> Background {
        Background {
            color,
            view: ViewBuilder::from_view(self),
        }
    }
}

impl<V: View + Sized> Backgroundable for V {}

impl View for Background {
    fn build(&self, _ctx: &mut Context) -> Vec<Rc<dyn View>> {
        vec![self.view.build()]
    }

    fn calculate_constraints(&self, child_constraints: &[Constraints]) -> Constraints {
        child_constraints[0]
    }

    fn calculate_layouts(&self, layout: Layout, _child_constraints: &[Constraints]) -> Vec<Layout> {
        vec![layout]
    }

    fn draw(&self, layout: Layout) -> Box<[Shape]> {
        Box::new([Shape::Rect {
            position: layout.position,
            size: layout.size,
            fill: Some(self.color),
            stroke: None,
        }])
    }
}
