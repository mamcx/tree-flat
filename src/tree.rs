#![allow(dead_code)]

use crate::iter::IntoIter;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};

use crate::prelude::*;

/// Vec-backed, *flattened*, Tree.
///
/// Always contains at least a root node.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tree<T> {
    pub(crate) data: Vec<T>,
    pub(crate) level: Vec<usize>,
    pub(crate) parent: Vec<usize>,
}

impl<T: Debug> Tree<T> {
    /// Create a new [Tree] with the specified value
    pub fn new(root: T) -> Self {
        Self::with_capacity(root, 1)
    }

    /// Create a new [Tree] with the specified value & set the capacity of the internal vectors
    pub fn with_capacity(root: T, capacity: usize) -> Self {
        let mut t = Tree {
            data: Vec::with_capacity(capacity),
            level: Vec::with_capacity(capacity),
            parent: Vec::with_capacity(capacity),
        };
        t.push_with_level(root, 0, NodeId(0));
        t
    }

    /// Push a node into the tree
    ///
    /// #WARNING
    ///
    /// This assumes you are pushing in pre-order!
    pub fn push_with_level(&mut self, data: T, level: usize, parent: NodeId) -> NodeId {
        let parent = if parent.0 == 0 { 0 } else { parent.0 - 1 };

        self.data.push(data);
        self.level.push(level);
        self.parent.push(parent);

        NodeId(self.data.len())
    }

    pub(crate) fn _make_node(&self, id: NodeId) -> Node<T> {
        Node {
            id,
            data: &self.data[id.0],
            tree: self,
        }
    }

    pub(crate) fn _make_node_mut(&mut self, id: NodeId) -> NodeMut<T> {
        NodeMut { id, tree: self }
    }

    /// Get the [Node<T>] from his [NodeId]
    pub fn node(&self, id: NodeId) -> Option<Node<T>> {
        if id.0 < self.data.len() {
            Some(self._make_node(id))
        } else {
            None
        }
    }

    /// Get a mutable [NodeMut<T>] from his [NodeId], so you can push children
    pub fn node_mut(&mut self, id: NodeId) -> Option<NodeMut<T>> {
        if id.0 < self.data.len() {
            Some(self._make_node_mut(id))
        } else {
            None
        }
    }

    /// Get a mutable [NodeMut<T>] handle of the root, so you can push children
    ///
    /// This always success
    pub fn root_mut(&mut self) -> NodeMut<T> {
        self._make_node_mut(NodeId(1))
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn into_iter(&self) -> IntoIter<T> {
        IntoIter { tree: self }
    }

    /// A slice view of the internal data
    pub fn as_data(&self) -> &[T] {
        &self.data
    }
    /// A slice view of the internal level
    pub fn as_level(&self) -> &[usize] {
        &self.level
    }
    /// A slice view of the internal parents
    pub fn as_parents(&self) -> &[usize] {
        &self.parent
    }

    /// Consume tree and move-out the data
    pub fn to_data(self) -> Vec<T> {
        self.data
    }

    /// Pretty-print the tree
    pub fn print(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    where
        T: Display,
    {
        let last = self.data.len() - 1;
        for (pos, x) in self.data.iter().enumerate() {
            let mut branch = if pos == 0 {
                "."
            } else if pos == last {
                "└──"
            } else {
                "├──"
            }
            .to_string();

            let level = self.level[pos];
            let mut col = String::with_capacity(level * 2);
            for _i in 1..level {
                match pos.cmp(&last) {
                    Ordering::Greater => branch.push_str(&*"──".repeat(level)),
                    Ordering::Less => col.push_str("├   "),
                    Ordering::Equal => branch.push_str("──"),
                }
            }
            writeln!(f, "{}{} {}", col, branch, x)?;
        }
        Ok(())
    }
}

impl<T: Debug + Display> Display for Tree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.print(f)
    }
}
