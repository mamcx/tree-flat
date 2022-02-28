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
        let (pos, range) = if parent.0 == 0 {
            (1, tree.parent.as_slice())
        } else {
            (0, &tree.parent[(parent.0 + 1)..])
        };
        ChildrenIter {
            pos,
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
            if self.parent.0 == 0 {
                self.pos += 1;
                return Some(self.tree._make_node(NodeId(self.pos - 2)));
            }
            for p in &self.range[self.pos..] {
                //dbg!(p, self.parent.0);
                let p = *p;
                self.pos += 1;
                if p == 0 {
                    break;
                }
                if p < self.parent.0 {
                    continue;
                }
                let node = NodeId(p);
                if self.tree.is_child(self.parent, node) {
                    return Some(self.tree._make_node(NodeId(self.pos + self.parent.0)));
                }
            }
        }
        None
    }
}
