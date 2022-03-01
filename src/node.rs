use std::fmt::{Debug, Display, Formatter};

use crate::iter::*;
use crate::prelude::*;

/// A node ID into the internal tree.
///
/// # Important:
///
/// Is not checked the [NodeId] was not from *another* tree.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId(pub usize);

impl Display for NodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write! {f, "NodeId({})", self.0}
    }
}

/// A immutable view of the [Self::data] in the [Tree] with their [NodeId].
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node<'a, T: 'a> {
    /// Node ID.
    pub id: NodeId,
    /// Data.
    pub data: &'a T,
    /// Tree containing the node.
    pub(crate) tree: &'a Tree<T>,
}

impl<T: Debug> Node<'_, T> {
    pub fn level(&self) -> usize {
        self.tree.level[self.id.0]
    }
    pub fn parent(&self) -> usize {
        self.tree.parent[self.id.0]
    }

    /// An [Iterator] of the parents from this [Node].
    pub fn parents(&self) -> ParentIter<'_, T> {
        ParentIter {
            parent: self.parent(),
            node: self.id,
            tree: self.tree,
        }
    }

    /// An [Iterator] of the children from this [Node].
    pub fn children(&self) -> ChildrenIter<'_, T> {
        ChildrenIter::new(self.id, self.tree)
    }

    /// An [Iterator] of the siblings from this [Node].
    pub fn siblings(&self) -> SiblingsIter<'_, T> {
        SiblingsIter {
            pos: 0,
            level: self.level(),
            node: self.id,
            tree: self.tree,
        }
    }
}

impl<T: Debug> Debug for Node<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write! {f, "{:?}:{:?}", self.id, self.data}
    }
}

impl<T: Display> Display for Node<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write! {f, "{}", self.data}
    }
}

/// A mutable reference in the [Tree] of the [NodeId].
#[derive(Debug)]
pub struct NodeMut<'a, T: 'a> {
    /// Node ID.
    pub id: NodeId,
    /// Tree containing the node.
    pub(crate) tree: &'a mut Tree<T>,
}

impl<'a, T: Debug + 'a> NodeMut<'a, T> {
    pub fn level(&self) -> usize {
        self.tree.level[self.id.0 - 1] + 1
    }

    /// Create a new [Node<T>], record the parent & the loop, and continue to
    /// return [NodeMut<T>] so you can add more levelly
    pub fn push(&mut self, data: T) -> NodeMut<T>
    where
        T: Debug,
    {
        let level = self.level();

        let id = self.tree.push_with_level(data, level, self.id);
        self.tree._make_node_mut(id)
    }
}
