use std::fmt::{Debug, Display, Formatter};
use std::num::NonZeroUsize;

use crate::iter::*;
use crate::prelude::*;

/// A node ID into the internal tree.
///
/// # Important:
///
/// Is not checked that the [NodeId] was not from *another* tree.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(NonZeroUsize);

impl NodeId {
    pub fn from_index(n: usize) -> Self {
        NodeId(NonZeroUsize::new(n + 1).unwrap())
    }

    pub fn to_index(self) -> usize {
        self.0.get() - 1
    }
}

impl Display for NodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write! {f, "NodeId({})", self.0}
    }
}

impl From<usize> for NodeId {
    fn from(x: usize) -> Self {
        NodeId::from_index(x)
    }
}

impl From<NodeId> for usize {
    fn from(x: NodeId) -> Self {
        x.to_index()
    }
}

/// An immutable view of the [Self::data] in the [Tree] with their [NodeId].
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
        self.tree.level[self.id.to_index()]
    }
    pub fn parent(&self) -> usize {
        self.tree.parent[self.id.to_index()]
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

/// A mutable view of the [Self::data] in the [Tree] with their [NodeId].
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeMut<'a, T: 'a> {
    /// Node ID.
    pub id: NodeId,
    /// Data.
    pub data: &'a mut T,
}

impl<T: Debug> Debug for NodeMut<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write! {f, "{:?}:{:?}", self.id, self.data}
    }
}

impl<T: Display> Display for NodeMut<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write! {f, "{}", self.data}
    }
}

/// A mutable reference in the [Tree] of the [NodeId].
#[derive(Debug)]
pub struct TreeMut<'a, T: 'a> {
    /// Node ID.
    pub id: NodeId,
    /// Node ID of the parent.
    pub parent: NodeId,
    /// Tree containing the node.
    pub tree: &'a mut Tree<T>,
}

impl<'a, T: Debug + 'a> TreeMut<'a, T> {
    pub fn get_parent_level(&self) -> usize {
        self.tree.get_level(self.parent)
    }

    /// Create a new [Node<T>], record the parent & the loop, and continue to
    /// return [NodeMut<T>] so you can add more in a builder pattern
    pub fn push(&mut self, data: T) -> TreeMut<T>
    where
        T: Debug,
    {
        let id = self.append(data);
        self.tree._make_tree_mut(id, id)
    }

    /// Create a new [Node<T>], record the parent & the loop, and
    /// return the created [NodeId]
    pub fn append(&mut self, data: T) -> NodeId
    where
        T: Debug,
    {
        let level = self.get_parent_level() + 1;

        self.tree.push_with_level(data, level, self.parent)
    }
}
