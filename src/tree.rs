#![allow(dead_code)]

use crate::iter::{IntoIter, TreeIter};
use crate::node::NodeMut;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};

use crate::prelude::*;

/// Vec-backed, *flattened in pre-order*, Tree.
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
        t.push_with_level(root, 0, 0.into());
        t
    }

    /// Push a node into the tree
    ///
    /// #WARNING
    ///
    /// This assumes you are pushing in pre-order!
    pub fn push_with_level(&mut self, data: T, level: usize, parent: NodeId) -> NodeId {
        let parent = parent.to_index();
        //let parent = if parent == 0 { 0 } else { parent - 1 };

        self.data.push(data);
        self.level.push(level);
        self.parent.push(parent);

        (self.data.len() - 1).into()
    }

    pub(crate) fn _make_node(&self, id: NodeId) -> Node<T> {
        Node {
            id,
            data: &self.data[id.to_index()],
            tree: self,
        }
    }

    pub(crate) fn _make_node_mut(&mut self, id: NodeId) -> NodeMut<T> {
        NodeMut {
            id,
            data: &mut self.data[id.to_index()],
        }
    }

    pub(crate) fn _make_tree_mut(&mut self, id: NodeId, parent: NodeId) -> TreeMut<T> {
        TreeMut {
            id,
            parent,
            tree: self,
        }
    }

    /// Get a mutable [TreeMut<T>] handle of the root, so you can push children
    ///
    /// This always success
    pub fn tree_root_mut(&mut self) -> TreeMut<T> {
        self._make_tree_mut(0.into(), 0.into())
    }

    /// Get a mutable [TreeMut<T>] from his [NodeId], so you can push children
    pub fn tree_node_mut(&mut self, id: NodeId) -> Option<TreeMut<T>> {
        if id.to_index() < self.data.len() {
            Some(self._make_tree_mut(id, 0.into()))
        } else {
            None
        }
    }

    /// Get the [Node<T>] from his [NodeId]
    pub fn node(&self, id: NodeId) -> Option<Node<T>> {
        if id.to_index() < self.data.len() {
            Some(self._make_node(id))
        } else {
            None
        }
    }

    /// Get the root [Node<T>]
    pub fn root(&self) -> Node<T> {
        self._make_node(0.into())
    }

    /// Get a mutable [NodeMut<T>] from his [NodeId].
    pub fn node_mut(&mut self, id: NodeId) -> Option<NodeMut<T>> {
        if id.to_index() < self.data.len() {
            Some(self._make_node_mut(id))
        } else {
            None
        }
    }

    /// Get a mutable [NodeMut<T>] handle of the root.
    ///
    /// This always success
    pub fn root_mut(&mut self) -> NodeMut<'_, T> {
        self._make_node_mut(0.into())
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> TreeIter<'_, T> {
        TreeIter { pos: 0, tree: self }
    }
    pub fn into_iter(&self) -> IntoIter<T> {
        IntoIter { tree: self }
    }

    /// A slice view of the internal data
    pub fn as_data(&self) -> &[T] {
        &self.data
    }
    /// A slice view of the internal data
    pub fn as_data_mut(&mut self) -> &mut [T] {
        self.data.as_mut_slice()
    }

    /// A slice view of the internal level
    pub fn as_level(&self) -> &[usize] {
        &self.level
    }

    /// Get the level from a [NodeId]
    pub fn get_level(&self, of: NodeId) -> usize {
        if of.to_index() == 0 {
            0
        } else {
            self.level[of.to_index()]
        }
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
