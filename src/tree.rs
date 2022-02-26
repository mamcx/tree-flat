//! High-performance Tree Wrangling, the APL Way
//!
//! https://aplwiki.com/wiki/Aaron_Hsu

use crate::iter::IntoIter;
use std::fmt::{write, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId(pub(crate) usize);

impl Display for NodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write! {f, "NodeId({})", self.0}
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeRef<'a, T: 'a> {
    /// Node ID.
    pub id: NodeId,
    /// Data.
    pub data: &'a T,
}

impl<T: Display> Display for NodeRef<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write! {f, "{}", self.data}
    }
}

#[derive(Debug, Clone)]
pub struct Node<'a, T: 'a> {
    /// Node ID.
    id: NodeId,
    /// Tree containing the node.
    tree: &'a Tree<T>,
}

#[derive(Debug)]
pub struct NodeMut<'a, T: 'a> {
    /// Node ID.
    id: NodeId,
    /// Tree containing the node.
    tree: &'a mut Tree<T>,
}

impl<'a, T: 'a> NodeMut<'a, T> {
    pub fn deep(&self) -> usize {
        self.tree.deep[self.id.0 - 1] + 1
    }

    pub fn push(&mut self, data: T) -> NodeMut<T> {
        let deep = self.deep();
        let id = self.tree._add(data, deep);
        NodeMut {
            id,
            tree: self.tree,
        }
    }

    pub fn push_many(&mut self, data: &[T]) -> NodeMut<T>
    where
        T: Clone,
    {
        let deep = self.deep();
        let id = self.tree._add_many(data, deep);
        NodeMut {
            id,
            tree: self.tree,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Jump {
    start: usize,
    len: usize,
}

/// Vec-backed, *flattened*, Tree.
///
/// Always contains at least a root node.
#[derive(Debug, Clone)]
pub struct Tree<T> {
    pub(crate) data: Vec<T>,
    pub(crate) deep: Vec<usize>,
    pub(crate) jump: Vec<Jump>,
}

impl<T> Tree<T> {
    pub fn new(root: T) -> Self {
        Self::with_capacity(root, 1)
    }

    pub fn with_capacity(root: T, capacity: usize) -> Self {
        let mut t = Tree {
            data: Vec::with_capacity(capacity),
            deep: Vec::with_capacity(capacity),
            jump: Vec::with_capacity(1),
        };
        t._add(root, 0);
        t
    }

    pub fn deep(&self, deep: usize) -> Option<NodeId> {
        if let Some(pos) = self.jump.get(deep) {
            Some(NodeId(self.deep[pos.start + pos.len - 1]))
        } else {
            None
        }
    }

    pub fn get(&self, node: NodeId) -> Option<&T> {
        let idx = node.0;
        self.data.get(idx)
    }

    fn _slice(&self, deep: usize) -> &[T] {
        if deep < self.jump.len() {
            let jump = self.jump[deep];

            &self.data[jump.start..(jump.start + jump.len)]
        } else {
            &[]
        }
    }

    fn _add_jump(&mut self, deep: usize) -> NodeId {
        let last = self.data.len();

        if deep >= self.jump.len() {
            self.jump.push(Jump {
                start: last - 1,
                len: 1,
            })
        } else {
            self.jump[deep].len += 1;
        }

        NodeId(last)
    }

    fn _add(&mut self, data: T, deep: usize) -> NodeId {
        self.data.push(data);
        self.deep.push(deep);

        self._add_jump(deep)
    }

    fn _add_many(&mut self, data: &[T], deep: usize) -> NodeId
    where
        T: Clone,
    {
        let mut deeps = vec![deep; data.len()];
        self.data.extend_from_slice(data);
        self.deep.append(&mut deeps);

        self._add_jump(deep)
    }

    pub fn node(&mut self, id: NodeId) -> Option<Node<T>> {
        if id.0 < self.data.len() {
            Some(Node { id, tree: self })
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

    pub fn print(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    where
        T: Display,
    {
        let last = self.data.len() - 1;
        for (pos, x) in self.data.iter().enumerate() {
            let branch = if pos == last {
                "└──"
            } else {
                if pos == 0 {
                    "."
                } else {
                    "├──"
                }
            };

            writeln!(f, "{}{}{}", " ".repeat(self.deep[pos]), branch, x)?;
        }
        Ok(())
    }
}

impl<T: Display> Display for Tree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.print(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ego_tree::Tree as ETree;
    // use std::fs::ReadDir;
    // use std::path::PathBuf;
    // use std::{fs, io};

    #[test]
    fn ego_create() {
        let mut tree = ETree::new(0);
        let mut root = tree.root_mut();

        for i in 1..10 {
            root.append(i);
        }

        println!("{tree:?}");
    }

    #[test]
    fn tree() {
        let mut tree = Tree::with_capacity(1, 5);

        let mut root = tree.root_mut();

        let mut child = root.push(2);
        child.push(21);
        child.push(22);
        root.push(3);

        dbg!(&tree);
        dbg!(&tree._slice(0));
        dbg!(&tree._slice(1));
        dbg!(&tree._slice(10));

        println!("{tree}");

        for n in &tree {
            println!("{n}");
        }
    }
    //
    // fn walk_dir(tree: &mut Tree<String>, parent: NodeId, of: ReadDir) -> io::Result<()> {
    //     dbg!(parent);
    //     for entry in of {
    //         let entry = entry?;
    //         let path = entry.path();
    //         let metadata = fs::metadata(&path)?;
    //         if metadata.is_dir() {
    //             let root = tree.push(parent, path.to_str().unwrap().into());
    //             dbg!("dir", root, parent);
    //             walk_dir(tree, root, fs::read_dir(path)?)?;
    //         } else {
    //             dbg!("file");
    //             tree.push(parent, path.to_str().unwrap().into());
    //         }
    //     }
    //
    //     Ok(())
    // }
    //
    // #[test]
    // fn files() -> io::Result<()> {
    //     let mut tree = Tree::with_capacity(3);
    //
    //     let p: PathBuf = "/Users/mamcx/Proyectos/basura/eldiro/crates/ast".into();
    //     let root = tree.root_mut(p.to_str().unwrap().into());
    //     walk_dir(&mut tree, root, fs::read_dir(p.clone())?)?;
    //     dbg!(&tree.deep);
    //     println!("{tree}");
    //
    //     Ok(())
    // }
}
