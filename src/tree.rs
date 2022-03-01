#![allow(dead_code)]
use crate::iter::IntoIter;
use std::fmt::{Debug, Display, Formatter};

use crate::prelude::*;

/// Vec-backed, *flattened*, Tree.
///
/// Always contains at least a root node.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tree<T> {
    pub(crate) data: Vec<T>,
    pub(crate) deep: Vec<usize>,
    pub(crate) parent: Vec<usize>,
}

impl<T: Debug> Tree<T> {
    pub fn new(root: T) -> Self {
        Self::with_capacity(root, 1)
    }

    pub fn with_capacity(root: T, capacity: usize) -> Self {
        let mut t = Tree {
            data: Vec::with_capacity(capacity),
            deep: Vec::with_capacity(capacity),
            parent: Vec::with_capacity(capacity),
        };
        t._add(root, 0, NodeId(0));
        t
    }

    pub(crate) fn _add(&mut self, data: T, deep: usize, parent: NodeId) -> NodeId {
        let parent = if parent.0 == 0 { 0 } else { parent.0 - 1 };

        self.data.push(data);
        self.deep.push(deep);
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

    pub fn node(&self, id: NodeId) -> Option<Node<T>> {
        if id.0 < self.data.len() {
            Some(self._make_node(id))
        } else {
            None
        }
    }

    pub fn node_mut(&mut self, id: NodeId) -> Option<NodeMut<T>> {
        if id.0 < self.data.len() {
            Some(self._make_node_mut(id))
        } else {
            None
        }
    }

    pub fn root_mut(&mut self) -> NodeMut<T> {
        self._make_node_mut(NodeId(1))
    }

    pub fn push_with_deep(&mut self, data: T, deep: usize, parent: NodeId) -> NodeId {
        self._add(data, deep, parent)
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

    pub fn as_data(&self) -> &[T] {
        &self.data
    }

    pub fn as_deep(&self) -> &[usize] {
        &self.deep
    }

    pub fn as_parents(&self) -> &[usize] {
        &self.parent
    }

    pub fn to_data(self) -> Vec<T> {
        self.data
    }

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

            let deep = self.deep[pos];
            let mut col = String::with_capacity(deep * 2);
            for _i in 1..deep {
                if pos < last {
                    col.push_str("├   ");
                } else {
                    branch.push_str("──");
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
