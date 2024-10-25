mod build;
mod change;
mod debug;
mod graphics;
mod node;

use super::View;
use crate::utils::id_vec::{Id, IdVec};
use change::Change;
use itertools::Itertools;
use macroquad::{math::Vec2, miniquad::window::screen_size};
pub use node::Node;
use std::{
    any::Any,
    cell::RefCell,
    collections::HashMap,
    ops::{Index, IndexMut},
    rc::Rc,
};

pub struct Tree {
    root: Id,
    nodes: IdVec<RefCell<Node>>,
}

impl Tree {
    pub fn new<V: View>(view: V) -> Self {
        let mut nodes = IdVec::new();
        let root = nodes.insert(RefCell::new(Node::new(None, Rc::new(view))));
        Tree { root, nodes }
    }

    pub fn update(&mut self, states: &mut HashMap<Id, Rc<dyn Any>>, id: Id) {
        self[id].borrow_mut().change.add(Change::VIEW);
        self.build(states, id);

        let layout = ViewLayout {
            tree: self,
            id: self.root,
        };
        let size = layout.size(screen_size().into());
        layout.layout(Vec2::ZERO, size);

        self.calculate_graphics(self.root);
        self.debug_print(self.root, "".into());
        self.reset(self.root);
    }
}

impl Tree {
    fn insert(&mut self, parent: Id, view: Rc<dyn View>) -> Id {
        self.nodes
            .insert(RefCell::new(Node::new(Some(parent), view.clone())))
    }

    fn remove(&mut self, id: Id) {
        let children = self[id].borrow().children.clone();
        for child_id in children {
            self.remove(child_id);
        }
        self.nodes.remove(id);
        // TODO: Clean up states and other id bound properties and callbacks
    }

    fn reset(&self, id: Id) {
        let mut node = self[id].borrow_mut();
        // if !node.change.contains(Change::ALL) {
        //     return;
        // }

        node.change.clear();
        for child_id in node.children.clone() {
            self.reset(child_id);
        }
    }
}

impl Index<Id> for Tree {
    type Output = RefCell<Node>;

    fn index(&self, id: Id) -> &Self::Output {
        &self.nodes[id]
    }
}

impl IndexMut<Id> for Tree {
    fn index_mut(&mut self, id: Id) -> &mut Self::Output {
        &mut self.nodes[id]
    }
}

type VisitorFunc<'a, P, V> = dyn Fn(&Node, P, &[Visitor<'a, P, V>]) -> V;

impl Tree {
    pub fn traverse<'a, P, V>(&'a self, parameter: P, func: &'a VisitorFunc<'a, P, V>) -> V {
        Visitor {
            tree: self,
            id: self.root,
            func,
        }
        .visit(parameter)
    }
}

pub struct Visitor<'a, P, V> {
    tree: &'a Tree,
    id: Id,
    func: &'a VisitorFunc<'a, P, V>,
}

impl<P, V> Visitor<'_, P, V> {
    pub fn visit(&self, parameter: P) -> V {
        let node = self.tree.nodes[self.id].borrow();
        (self.func)(
            &*node,
            parameter,
            &node
                .children
                .iter()
                .map(|id| Visitor {
                    tree: self.tree,
                    id: *id,
                    func: self.func,
                })
                .collect::<Box<_>>(),
        )
    }
}

pub struct ViewSize<'a> {
    tree: &'a Tree,
    id: Id,
}

impl ViewSize<'_> {
    pub fn size(&self, constraints: Vec2) -> Vec2 {
        let node = self.tree.nodes[self.id].borrow();
        node.view.size(
            constraints,
            &node
                .children
                .iter()
                .map(|id| ViewSize {
                    tree: self.tree,
                    id: *id,
                })
                .collect::<Box<_>>(),
        )
    }
}

pub struct ViewLayout<'a> {
    tree: &'a Tree,
    id: Id,
}

impl ViewLayout<'_> {
    pub fn size(&self, constraints: Vec2) -> Vec2 {
        ViewSize {
            tree: self.tree,
            id: self.id,
        }
        .size(constraints)
    }

    pub fn layout(&self, position: Vec2, size: Vec2) {
        let mut node = self.tree.nodes[self.id].borrow_mut();
        node.position = position;

        if node.size != size {
            node.size = size;
            node.change.add(Change::SIZE);
        }

        // if node.change.contains(Change::VIEW | Change::SIZE) {
        node.view.layout(
            size,
            &node
                .children
                .iter()
                .map(|id| ViewLayout {
                    tree: self.tree,
                    id: *id,
                })
                .collect::<Box<_>>(),
        );
        // }
    }
}
