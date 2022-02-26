use crate::tree::*;

pub struct TreeIter<'a, T> {
    pos: usize,
    tree: &'a Tree<T>,
}

impl<'a, T> Iterator for TreeIter<'a, T> {
    type Item = Node<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos += 1;
        let id = NodeId(self.pos);
        if let Some(data) = self.tree.get(id) {
            Some(Node {
                id,
                data,
                tree: &self.tree,
            })
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

impl<'a, T> IntoIterator for IntoIter<'a, T> {
    type Item = Node<'a, T>;
    type IntoIter = TreeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        TreeIter {
            pos: 0,
            tree: &self.tree,
        }
    }
}

impl<'a, T> IntoIterator for &'a Tree<T> {
    type Item = Node<'a, T>;
    type IntoIter = TreeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        TreeIter {
            pos: 0,
            tree: &self,
        }
    }
}
