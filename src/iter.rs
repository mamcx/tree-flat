use std::fmt::Debug;

use crate::prelude::*;

pub struct TreeIter<'a, T> {
    pos: usize,
    tree: &'a Tree<T>,
}

impl<'a, T: Debug> Iterator for TreeIter<'a, T> {
    type Item = Node<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let id = NodeId(self.pos);
        self.pos += 1;
        if self.pos <= self.tree.len() {
            Some(self.tree._make_node(id))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.tree.len()))
    }
}

pub struct IntoIter<'a, T> {
    pub(crate) tree: &'a Tree<T>,
}

impl<'a, T: Debug> IntoIterator for IntoIter<'a, T> {
    type Item = Node<'a, T>;
    type IntoIter = TreeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        TreeIter {
            pos: 0,
            tree: self.tree,
        }
    }
}

impl<'a, T: Debug> IntoIterator for &'a Tree<T> {
    type Item = Node<'a, T>;
    type IntoIter = TreeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        TreeIter { pos: 0, tree: self }
    }
}

#[derive(Debug)]
pub struct ParentIter<'a, T> {
    pub(crate) parent: usize,
    pub(crate) node: NodeId,
    pub(crate) tree: &'a Tree<T>,
}

impl<'a, T: Debug> Iterator for ParentIter<'a, T> {
    type Item = Node<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        // dbg!(self.pos, self.parent, self.node.0);
        if self.node.0 > 0 {
            self.node = NodeId(self.parent);
            self.parent = self.tree.parent[self.parent];
            Some(self.tree._make_node(self.node))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct ChildrenIter<'a, T> {
    pub(crate) pos: usize,
    pub(crate) parent: NodeId,
    pub(crate) range: &'a [usize],
    pub(crate) tree: &'a Tree<T>,
}

impl<'a, T> ChildrenIter<'a, T> {
    pub fn new(parent: NodeId, tree: &'a Tree<T>) -> Self {
        let range = &tree.parent[(parent.0 + 1)..];
        dbg!(range);
        ChildrenIter {
            pos: 1,
            parent,
            range,
            tree,
        }
    }
}

impl<'a, T: Debug> Iterator for ChildrenIter<'a, T> {
    type Item = Node<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        //dbg!(self.pos, self.range.len());
        if self.pos <= self.range.len() {
            let level = self.tree.deep[self.parent.0];
            let node = NodeId(self.pos + self.parent.0);
            let level_child = self.tree.deep[node.0];
            //dbg!(self.pos, level, node.0, level_child);
            self.pos += 1;

            if level_child > level {
                Some(self.tree._make_node(node))
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct SiblingsIter<'a, T> {
    pub(crate) pos: usize,
    pub(crate) level: usize,
    pub(crate) node: NodeId,
    pub(crate) tree: &'a Tree<T>,
}

impl<'a, T: Debug> Iterator for SiblingsIter<'a, T> {
    type Item = Node<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        //dbg!(self.pos, self.range.len());
        if self.pos <= self.tree.len() {
            let start = self.pos;
            if let Some(pos) =
                self.tree.deep[start..]
                    .iter()
                    .enumerate()
                    .find_map(|(pos, level)| {
                        let idx = self.pos + pos;
                        if *level == self.level && self.node.0 != idx {
                            Some(idx)
                        } else {
                            None
                        }
                    })
            {
                self.pos = pos + 1;
                Some(self.tree._make_node(NodeId(pos)))
            } else {
                None
            }
        } else {
            None
        }
    }
}
