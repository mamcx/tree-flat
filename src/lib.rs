//! # TreeFlat is the simplest way to build & traverse a pre-order Tree for Rust.
//!
//! If you build a [tree::Tree] *in pre-order*, and display *in pre-order*,
//! this is the tree for you.
//!
//! **No extra fluff**, just a simple & performant one-trick pony.
//!
//! Note: The tree depends in the build order, so is not possible to re-order the tree
//! (changing parents or levels) in different order. So, for example, you can't add
//! a branch later to one in the *middle* (only can add *after* the end...).
//!
//! ## How it works
//!
//! Instead of creating an [tree::Tree] of [node::Node] pointers, nested enums, or nested `Arena`-based `ids`,
//! it just stores the representation of a [tree::Tree] like:
//!
//! ```bash
//! . Users
//! ├── jhon_doe
//! ├   ├── file1.rs
//! ├   ├── file2.rs
//! ├── jane_doe
//! └──── cat.jpg
//! ```
//!
//! ... flattened in pre-order on 3 [Vec], that store the data, the deep/level and the parent:
//!
//! | DATA:  | Users | jhon_doe | file1.rs | file2.rs | jane_doe | cat.jpg |
//! |--------|-------|----------|----------|----------|----------|---------|
//! | DEEP:  | 0     | 1        | 2        | 2        | 1        | 2       |
//! | PARENT:| 0     | 0        | 1        | 1        | 0        | 4       |
//!
//! This allows for the performance of [Vec], on the most common operations
//! (critically: Push items + Iterate), and very efficient iterations of
//! [node::Node::parents]/[node::Node::childrens]/[node::Node::siblings], because
//! it just traverse the flat vectors.
//!
//! - - - - - -
//!
//! Inspired by the talk:
//!
//! > “High-performance Tree Wrangling, the APL Way”
//! > -- <cite> [Aaron Hsu - APL Wiki](https://aplwiki.com/wiki/Aaron_Hsu)  

/// Flat-tree iterators
pub mod iter;
/// Flat-tree nodes
pub mod node;
#[cfg(test)]
mod tests;
/// Flat-tree implementation
pub mod tree;

pub mod prelude {
    pub use crate::iter;
    pub use crate::node::{Node, NodeId, NodeMut};
    pub use crate::tree;
    pub use crate::tree::Tree;
}
