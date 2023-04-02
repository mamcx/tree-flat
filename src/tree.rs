#![allow(dead_code)]

use crate::iter::{IntoIter, TreeIter};
use crate::node::NodeMut;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};

use crate::prelude::*;

/// Vec-backed, *flattened in pre-order*, Tree.
///
/// Always contains at least a root node.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tree<T> {
    pub(crate) data: Vec<T>,
    pub(crate) level: Vec<usize>,
    pub(crate) parent: Vec<usize>,
}

impl<T: Debug> Tree<T> {
    /// Create a new [Tree] with the specified value
    pub fn new(root: T) -> Self {
        Self::with_capacity(root, 1)
    }

    /// Create a new [Tree] with the specified value & set the capacity of the internal vectors
    pub fn with_capacity(root: T, capacity: usize) -> Self {
        let mut t = Tree {
            data: Vec::with_capacity(capacity),
            level: Vec::with_capacity(capacity),
            parent: Vec::with_capacity(capacity),
        };
        t.push_with_level(root, 0, 0.into());
        t
    }

    /// Returns the total number of elements the tree can hold without reallocating.
    pub fn capacity(&self) -> usize {
        self.data.capacity() // Any of the three underlying vectors is good enough.
    }

    /// Reserves capacity for at least `additional` more elements to be inserted
    /// in the given `Tree<T>`. The collection may reserve more space to
    /// speculatively avoid frequent reallocations. After calling `reserve`,
    /// capacity will be greater than or equal to `self.len() + additional`.
    /// Does nothing if capacity is already sufficient.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
        self.level.reserve(additional);
        self.parent.reserve(additional);
    }

    /// Reserves the minimum capacity for at least `additional` more elements to
    /// be inserted in the given `Tree<T>`. Unlike [`reserve`], this will not
    /// deliberately over-allocate to speculatively avoid frequent allocations.
    /// After calling `reserve_exact`, capacity will be greater than or equal to
    /// `self.len() + additional`. Does nothing if the capacity is already
    /// sufficient.
    ///
    /// Note that the allocator may give the collection more space than it
    /// requests. Therefore, capacity can not be relied upon to be precisely
    /// minimal. Prefer [`reserve`] if future insertions are expected.
    ///
    /// [`reserve`]: Tree::reserve
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    pub fn reserve_exact(&mut self, additional: usize) {
        self.data.reserve_exact(additional);
        self.level.reserve_exact(additional);
        self.parent.reserve_exact(additional);
    }

    /// Tries to reserve capacity for at least `additional` more elements to be inserted
    /// in the given `Tree<T>`. The collection may reserve more space to speculatively avoid
    /// frequent reallocations. After calling `try_reserve`, capacity will be
    /// greater than or equal to `self.len() + additional` if it returns
    /// `Ok(())`. Does nothing if capacity is already sufficient. This method
    /// preserves the contents even if an error occurs.
    ///
    /// # Errors
    ///
    /// If the capacity overflows, or the allocator reports a failure, then an error
    /// is returned.
    pub fn try_reserve(
        &mut self,
        additional: usize,
    ) -> Result<(), std::collections::TryReserveError> {
        self.data.try_reserve(additional)?;
        self.level.try_reserve(additional)?;
        self.parent.try_reserve(additional)
    }

    /// Tries to reserve the minimum capacity for at least `additional`
    /// elements to be inserted in the given `Tree<T>`. Unlike [`try_reserve`],
    /// this will not deliberately over-allocate to speculatively avoid frequent
    /// allocations. After calling `try_reserve_exact`, capacity will be greater
    /// than or equal to `self.len() + additional` if it returns `Ok(())`.
    /// Does nothing if the capacity is already sufficient.
    ///
    /// Note that the allocator may give the collection more space than it
    /// requests. Therefore, capacity can not be relied upon to be precisely
    /// minimal. Prefer [`try_reserve`] if future insertions are expected.
    ///
    /// [`try_reserve`]: Tree::try_reserve
    ///
    /// # Errors
    ///
    /// If the capacity overflows, or the allocator reports a failure, then an error
    /// is returned.
    pub fn try_reserve_exact(
        &mut self,
        additional: usize,
    ) -> Result<(), std::collections::TryReserveError> {
        self.data.try_reserve_exact(additional)?;
        self.level.try_reserve_exact(additional)?;
        self.parent.try_reserve_exact(additional)
    }

    /// Shrinks the capacity of the tree as much as possible.
    ///
    /// It will drop down as close as possible to the length but the allocator
    /// may still inform the tree that there is space for a few more elements.
    pub fn shrink_to_fit(&mut self) {
        if self.capacity() > self.len() {
            self.data.shrink_to_fit();
            self.level.shrink_to_fit();
            self.parent.shrink_to_fit();
        }
    }

    /// Shrinks the capacity of the tree with a lower bound.
    ///
    /// The capacity will remain at least as large as both the length
    /// and the supplied value.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    pub fn shrink_to(&mut self, min_capacity: usize) {
        if self.capacity() > min_capacity {
            self.data.shrink_to(min_capacity);
            self.level.shrink_to(min_capacity);
            self.parent.shrink_to(min_capacity);
        }
    }

    /// Shortens the tree, keeping the first `len` elements and dropping
    /// the rest.
    ///
    /// If `len` is greater than the tree's current length, this has no
    /// effect.
    ///
    /// The [`drain`] method can emulate `truncate`, but causes the excess
    /// elements to be returned instead of dropped.
    ///
    /// Note that this method has no effect on the allocated capacity
    /// of the tree.
    ///
    /// [`drain`]: Tree::drain
    pub fn truncate(&mut self, len: usize) {
        self.data.truncate(len);
        self.level.truncate(len);
        self.parent.truncate(len);
    }

    /// Push a node into the tree
    ///
    /// #WARNING
    ///
    /// This assumes you are pushing in pre-order!
    pub fn push_with_level(&mut self, data: T, level: usize, parent: NodeId) -> NodeId {
        let parent = parent.to_index();
        //let parent = if parent == 0 { 0 } else { parent - 1 };

        self.data.push(data);
        self.level.push(level);
        self.parent.push(parent);

        (self.data.len() - 1).into()
    }

    pub(crate) fn _make_node(&self, id: NodeId) -> Node<T> {
        Node {
            id,
            data: &self.data[id.to_index()],
            tree: self,
        }
    }

    pub(crate) fn _make_node_mut(&mut self, id: NodeId) -> NodeMut<T> {
        NodeMut {
            id,
            data: &mut self.data[id.to_index()],
        }
    }

    pub(crate) fn _make_tree_mut(&mut self, id: NodeId, parent: NodeId) -> TreeMut<T> {
        TreeMut {
            id,
            parent,
            tree: self,
        }
    }

    /// Removes the last element from a tree and returns it as a triple
    /// `(data: T, level: usize, parent: NodeId)`, or [`None`] if it
    /// is empty.
    #[inline]
    pub fn pop(&mut self) -> Option<(T, usize, NodeId)> {
        if let Some(data) = self.data.pop() {
            let level = self.level.pop().unwrap();
            let parent = self.parent.pop().unwrap().into();
            Some((data, level, parent))
        } else {
            None
        }
    }

    /// Removes the specified range from the tree in bulk, returning all
    /// removed elements as an iterator. If the iterator is dropped before
    /// being fully consumed, it drops the remaining removed elements.
    ///
    /// The returned iterator keeps a mutable borrow on the tree to optimize
    /// its implementation.
    ///
    /// # Panics
    ///
    /// Panics if the starting point is greater than the end point or if
    /// the end point is greater than the length of the vector.
    ///
    /// # Leaking
    ///
    /// If the returned iterator goes out of scope without being dropped (due to
    /// [`mem::forget`], for example), the tree may have lost and leaked
    /// elements arbitrarily, including elements outside the range.
    //
    // # Implementation
    //
    // The return type may be specialized as in `std::vec::Drain`, implementing more traits.
    pub fn drain<R>(&mut self, range: R) -> impl Iterator<Item = (T, usize, NodeId)> + '_
    where
        R: std::ops::RangeBounds<usize> + Clone,
    {
        let mut data_drain = self.data.drain(range.clone());
        let mut level_drain = self.level.drain(range.clone());
        let mut parent_drain = self.parent.drain(range);
        std::iter::from_fn(move || match data_drain.next() {
            Some(data) => {
                let level = level_drain.next().unwrap();
                let parent = parent_drain.next().unwrap().into();
                Some((data, level, parent))
            }
            None => None,
        })
    }

    /// Clears the tree, removing all values.
    ///
    /// Note that this method has no effect on the allocated capacity
    /// of the tree.
    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
        self.level.clear();
        self.parent.clear();
    }

    /// Returns the number of elements in the tree, also referred to as its ‘length’.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the vector contains no elements.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get a mutable [TreeMut<T>] handle of the root, so you can push children
    ///
    /// This always success
    pub fn tree_root_mut(&mut self) -> TreeMut<T> {
        self._make_tree_mut(0.into(), 0.into())
    }

    /// Get a mutable [TreeMut<T>] from his [NodeId], so you can push children
    pub fn tree_node_mut(&mut self, id: NodeId) -> Option<TreeMut<T>> {
        if id.to_index() < self.data.len() {
            Some(self._make_tree_mut(id, 0.into()))
        } else {
            None
        }
    }

    /// Get the [Node<T>] from his [NodeId]
    pub fn node(&self, id: NodeId) -> Option<Node<T>> {
        if id.to_index() < self.data.len() {
            Some(self._make_node(id))
        } else {
            None
        }
    }

    /// Get the root [Node<T>]
    pub fn root(&self) -> Node<T> {
        self._make_node(0.into())
    }

    /// Get a mutable [NodeMut<T>] from his [NodeId].
    pub fn node_mut(&mut self, id: NodeId) -> Option<NodeMut<T>> {
        if id.to_index() < self.data.len() {
            Some(self._make_node_mut(id))
        } else {
            None
        }
    }

    /// Get a mutable [NodeMut<T>] handle of the root.
    ///
    /// This always success
    pub fn root_mut(&mut self) -> NodeMut<'_, T> {
        self._make_node_mut(0.into())
    }

    pub fn iter(&self) -> TreeIter<'_, T> {
        TreeIter { pos: 0, tree: self }
    }
    pub fn into_iter(&self) -> IntoIter<T> {
        IntoIter { tree: self }
    }

    /// A slice view of the internal data
    pub fn as_data(&self) -> &[T] {
        &self.data
    }
    /// A slice view of the internal data
    pub fn as_data_mut(&mut self) -> &mut [T] {
        self.data.as_mut_slice()
    }

    /// A slice view of the internal level
    pub fn as_level(&self) -> &[usize] {
        &self.level
    }

    /// Get the level from a [NodeId]
    pub fn get_level(&self, of: NodeId) -> usize {
        if of.to_index() == 0 {
            0
        } else {
            self.level[of.to_index()]
        }
    }

    /// A slice view of the internal parents
    pub fn as_parents(&self) -> &[usize] {
        &self.parent
    }

    /// Consume tree and move-out the data
    pub fn to_data(self) -> Vec<T> {
        self.data
    }

    /// Pretty-print the tree
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

            let level = self.level[pos];
            let mut col = String::with_capacity(level * 2);
            for _i in 1..level {
                match pos.cmp(&last) {
                    Ordering::Greater => branch.push_str(&"──".repeat(level)),
                    Ordering::Less => col.push_str("├   "),
                    Ordering::Equal => branch.push_str("──"),
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
