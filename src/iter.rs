use crate::tree::*;
use std::fmt::Debug;

pub struct TreeIter<'a, T> {
    pos: usize,
    tree: &'a Tree<T>,
}

impl<'a, T: Debug> Iterator for TreeIter<'a, T> {
    type Item = Node<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let id = NodeId(self.pos);
        self.pos += 1;
        if self.pos < self.tree.len() {
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
            tree: &self.tree,
        }
    }
}

impl<'a, T: Debug> IntoIterator for &'a Tree<T> {
    type Item = Node<'a, T>;
    type IntoIter = TreeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        TreeIter {
            pos: 0,
            tree: &self,
        }
    }
}
