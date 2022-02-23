//! High-performance Tree Wrangling, the APL Way
//!
//! https://aplwiki.com/wiki/Aaron_Hsu

use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub struct Node(usize);

#[derive(Debug, Clone, Copy)]
struct Jump {
    start: usize,
    len: usize,
}

#[derive(Debug, Clone)]
pub struct Tree<T> {
    data: Vec<T>,
    deep: Vec<usize>,
    jump: Vec<Jump>,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Tree {
            data: Vec::new(),
            deep: Vec::new(),
            jump: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Tree {
            data: Vec::with_capacity(capacity),
            deep: Vec::with_capacity(capacity),
            jump: Vec::new(),
        }
    }

    pub fn deep(&self, deep: usize) -> Option<Node> {
        if let Some(pos) = self.jump.get(deep) {
            Some(Node(self.deep[pos.start + pos.len - 1]))
        } else {
            None
        }
    }

    pub fn get(&self, node: Node) -> Option<&T> {
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

    fn _add(&mut self, data: T, deep: usize) -> Node {
        self.data.push(data);
        self.deep.push(deep);
        let last = self.data.len();

        if deep >= self.jump.len() {
            self.jump.push(Jump {
                start: last - 1,
                len: 1,
            })
        } else {
            self.jump[deep].len += 1;
        }

        Node(last)
    }

    pub fn add_root(&mut self, data: T) -> Node {
        self._add(data, 0)
    }

    pub fn add_child(&mut self, parent: Node, data: T) -> Node {
        let deep = self.deep[parent.0 - 1] + 1;
        self._add(data, deep)
    }

    pub fn print(&self, f: &mut Formatter<'_>, sep: &str) -> std::fmt::Result
    where
        T: Display,
    {
        for (pos, x) in self.data.iter().enumerate() {
            writeln!(f, "{}{}", sep.repeat(self.deep[pos]), x,)?;
        }
        Ok(())
    }
}

impl<T: Display> Display for Tree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.print(f, "-")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::ReadDir;
    use std::path::PathBuf;
    use std::{fs, io};

    #[test]
    fn tree() {
        let mut tree = Tree::with_capacity(3);

        let root = tree.add_root(1);

        tree.add_child(root, 2);
        tree.add_child(root, 3);

        dbg!(&tree);
        dbg!(&tree._slice(0));
        dbg!(&tree._slice(1));
        dbg!(&tree._slice(10));

        println!("{tree}");
    }

    fn walk_dir(tree: &mut Tree<String>, parent: Node, of: ReadDir) -> io::Result<()> {
        dbg!(parent);
        for entry in of {
            let entry = entry?;
            let path = entry.path();
            let metadata = fs::metadata(&path)?;
            if metadata.is_dir() {
                let root = tree.add_child(parent, path.to_str().unwrap().into());
                dbg!("dir", root, parent);
                walk_dir(tree, root, fs::read_dir(path)?)?;
            } else {
                dbg!("file");
                tree.add_child(parent, path.to_str().unwrap().into());
            }
        }

        Ok(())
    }

    #[test]
    fn files() -> io::Result<()> {
        let mut tree = Tree::with_capacity(3);

        let p: PathBuf = "/Users/mamcx/Proyectos/basura/eldiro/crates/ast".into();
        let root = tree.add_root(p.to_str().unwrap().into());
        walk_dir(&mut tree, root, fs::read_dir(p.clone())?)?;
        dbg!(&tree.deep);
        println!("{tree}");

        Ok(())
    }
}
