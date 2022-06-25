use std::fmt::Debug;

use crate::prelude::*;

pub struct TreeIter<'a, T> {
    pub(crate) pos: usize,
    pub(crate) tree: &'a Tree<T>,
}

impl<'a, T: Debug> Iterator for TreeIter<'a, T> {
    type Item = Node<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let id = NodeId::from_index(self.pos);
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
        if self.node.to_index() > 0 {
            self.node = NodeId::from_index(self.parent);
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
        let range = &tree.parent[parent.to_index() + 1..];
        //dbg!(range);
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
            let idx = self.parent.to_index();
            let level_parent = self.tree.level[idx];
            let node = NodeId::from_index(self.pos + idx);
            let level_child = self.tree.level[node.to_index()];
            //dbg!(self.pos, level_parent, node, level_child);
            self.pos += 1;

            if level_child > level_parent {
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
                self.tree.level[start..]
                    .iter()
                    .enumerate()
                    .find_map(|(pos, level)| {
                        let idx = self.pos + pos;
                        if *level == self.level && self.node.to_index() != idx {
                            Some(idx)
                        } else {
                            None
                        }
                    })
            {
                self.pos = pos + 1;
                Some(self.tree._make_node(NodeId::from_index(pos)))
            } else {
                None
            }
        } else {
            None
        }
    }
}
