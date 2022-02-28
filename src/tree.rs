#![allow(dead_code)]
use crate::iter::{ChildrenIter, IntoIter, ParentIter};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId(pub(crate) usize);

impl Display for NodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write! {f, "NodeId({})", self.0}
    }
}

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
    pub fn deep(&self) -> usize {
        self.tree.deep[self.id.0]
    }

    pub fn iter_parents(&self) -> ParentIter<'_, T> {
        ParentIter {
            parent: self.tree.parent[self.id.0],
            node: self.id,
            tree: &self.tree,
        }
    }

    pub fn iter_childrens(&self) -> ChildrenIter<'_, T> {
        ChildrenIter::new(self.id, self.tree)
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

#[derive(Debug)]
pub struct NodeMut<'a, T: 'a> {
    /// Node ID.
    id: NodeId,
    /// Tree containing the node.
    tree: &'a mut Tree<T>,
}

impl<'a, T: Debug + 'a> NodeMut<'a, T> {
    pub fn deep(&self) -> usize {
        self.tree.deep[self.id.0 - 1] + 1
    }

    pub fn push(&mut self, data: T) -> NodeMut<T>
    where
        T: Debug,
    {
        let deep = self.deep();

        let id = self.tree._add(data, deep, self.id);
        NodeMut {
            id,
            tree: self.tree,
        }
    }

    // pub fn push_many(&mut self, data: &[T]) -> NodeMut<T>
    // where
    //     T: Clone,
    // {
    //     let deep = self.deep();
    //     let id = self.tree._add_many(data, deep);
    //     NodeMut {
    //         id,
    //         tree: self.tree,
    //     }
    // }
}

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

    pub fn get(&self, node: NodeId) -> Option<&T> {
        let idx = node.0;
        self.data.get(idx)
    }

    fn _add(&mut self, data: T, deep: usize, parent: NodeId) -> NodeId {
        let parent = if parent.0 == 0 { 0 } else { parent.0 - 1 };

        self.data.push(data);
        self.deep.push(deep);
        self.parent.push(parent);

        NodeId(self.data.len())
    }

    // fn _add_many(&mut self, data: &[T], deep: usize) -> NodeId
    // where
    //     T: Clone,
    // {
    //     let mut deeps = vec![deep; data.len()];
    //     self.data.extend_from_slice(data);
    //     self.deep.append(&mut deeps);
    //
    //     self._add_jump(deep)
    // }

    fn parent(&self, _id: Node<T>) -> Option<Node<T>> {
        unimplemented!()
    }

    pub(crate) fn is_child(&self, parent: NodeId, of: NodeId) -> bool {
        //dbg!(parent, of);
        if parent == of {
            return true;
        }
        if parent > of {
            return false;
        }
        if let Some(p) = self.parent.get(of.0) {
            //dbg!(parent, p);
            if parent.0 == *p {
                true
            } else {
                if *p == 0 {
                    false
                } else {
                    self.is_child(parent, NodeId(*p - 1))
                }
            }
        } else {
            false
        }
    }

    pub(crate) fn _make_node(&self, id: NodeId) -> Node<T> {
        Node {
            id,
            data: &self.data[id.0],
            tree: self,
        }
    }

    pub fn node(&self, id: NodeId) -> Option<Node<T>> {
        if id.0 < self.data.len() {
            Some(self._make_node(id))
        } else {
            None
        }
    }

    pub fn root_mut(&mut self) -> NodeMut<T> {
        NodeMut {
            id: NodeId(1),
            tree: self,
        }
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
