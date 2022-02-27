#![allow(dead_code)]
//! # TreeFlat is the simplest way to build & traverse a pre-order Tree.
//!
//! If you build a [Tree] *in pre-order*, and display *in pre-order*,
//! this is the tree for you.
//!
//! **No extra fluff**, just a simple & performant one-trick pony.
//!
//! ## How it is made
//!
//! Instead of create a [Tree] of pointers, nested enums or `Arena`-based `ids`,
//! it just store the representation of a [Tree] like:
//!
//! ```bash
//! .Users
//! ├──jhon_doe
//! | ├── file1.rs
//! | ├── file2.rs
//! ├──jane_doe
//! └── cat.jpg
//! ```
//!
//! .. flattened in pre-order on 2 [Vec], that store the data and the deep:
//!
//! | DATA:| Users | jhon_doe | file1.rs | file2.rs | jane_doe | cat.jpg |
//! |------|-------|----------|----------|----------|----------|---------|
//! | DEEP:| 0     | 1        | 2        | 2        | 1        | 2       |
//!
//! This allows for the performance of [Vec], on the most common operations
//! (critically: Push items + Iterate).
//!
//! - - - - - -
//!
//! Inspired by the talk:
//!
//! > “High-performance Tree Wrangling, the APL Way”
//! > -- <cite> [Aaron Hsu - APL Wiki](https://aplwiki.com/wiki/Aaron_Hsu)  
//!

use crate::iter::IntoIter;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId(pub(crate) usize);

impl Display for NodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write! {f, "NodeId({})", self.0}
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Path(Vec<usize>);

impl Path {
    pub fn grow(&mut self, len: usize) {
        self.0.push(len);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Jump {
    paths: Vec<Path>,
}

impl Jump {
    pub fn new() -> Self {
        Jump { paths: vec![] }
    }
}

impl Display for Jump {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", '[')?;
        for j in &self.paths {
            write!(f, "{}", '[')?;
            for k in &j.0 {
                write!(f, "{}, ", k)?;
            }
            write!(f, "{}", ']')?;
        }
        write!(f, "{}", ']')?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node<'a, T: 'a> {
    /// Node ID.
    pub id: NodeId,
    /// Data.
    pub data: &'a T,
    /// Tree containing the node.
    pub(crate) tree: &'a Tree<T>,
}

impl<T> Node<'_, T> {
    pub fn deep(&self) -> usize {
        self.tree.deep[self.id.0]
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
    pub(crate) jump: Jump,
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
            jump: Jump::new(),
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

        let idx = self.data.len();
        let p = self._add_jump(deep);
        p.grow(parent);

        NodeId(idx)
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

    pub(crate) fn _make_node(&self, id: NodeId) -> Node<T> {
        Node {
            id,
            data: &self.data[id.0],
            tree: self,
        }
    }

    pub fn node(&mut self, id: NodeId) -> Option<Node<T>> {
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
    fn tree2() {
        let mut tree = Tree::with_capacity(0, 5);

        let mut root = tree.root_mut();
        root.push(1).push(2);

        let mut child = root.push(3);
        child.push(4).push(5);
        child.push(6);

        println!("{tree}");
        println!("{:?}", &tree.deep);
        println!("{:?}", &tree.data);
        println!("{:?}", &tree.parent);
        println!("{}", &tree.jump);
        //
        // let mut levels = vec![];
        // for (pos, (x, deep)) in tree.data.iter().zip(tree.deep.iter()).enumerate() {
        //     let deep = *deep;
        //     if deep > levels.len() {
        //         for x in levels.len()..deep {
        //             levels.push(vec![]);
        //         }
        //     }
        //     if deep > 0 {
        //         levels[deep - 1].push(pos);
        //     }
        // }
        //
        // dbg!(levels);
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
        println!("{tree}");

        for n in &tree {
            println!("{n}");
        }
        //
        // let mut tree = Tree::with_capacity("Users", 5);
        //
        // let mut root = tree.root_mut();
        //
        // let mut child = root.push("jhon_doe");
        // child.push("file1.rs");
        // child.push("file2.rs");
        // let mut child = root.push("jane_doe");
        // child.push("cat.jpg");
        //
        // dbg!(tree.as_data());
        // dbg!(tree.as_deep());
        // println!("{tree}");
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
