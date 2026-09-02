//! The per-pass node arena that `view()` flattens into a `view-tree`.
//!
//! WIT has no recursive types, so the guest builds its widget tree as a flat
//! `list<node>` with `node-id` indices standing in for parent->child edges.
//! Each widget's `From<Widget> for Element` closure pushes its record here in
//! post-order and returns the id; [`crate::ApplicationResource::view`] turns the
//! finished arena into a `view-tree` and hands it to the host by value.
//!
//! An `Arena` is created fresh for each `view()` pass and threaded by exclusive
//! reference through the build closures alongside [`crate::element::Realize`] —
//! there is no process-global arena.

use crate::bindings::iced::app::widgets::{Node, NodeId};

/// A flat, append-only collection of the nodes built during one `view()` pass.
#[allow(missing_debug_implementations)]
#[derive(Default)]
pub(crate) struct Arena {
    nodes: Vec<Node>,
}

impl Arena {
    /// Creates an empty arena for a fresh `view()` pass.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Appends `node` and returns its id.
    pub(crate) fn push(&mut self, node: Node) -> NodeId {
        self.nodes.push(node);
        (self.nodes.len() - 1) as NodeId
    }

    /// Consumes the arena, yielding the built node list.
    pub(crate) fn into_nodes(self) -> Vec<Node> {
        self.nodes
    }
}
