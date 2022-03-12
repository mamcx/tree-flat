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
//! └────── cat.jpg
//! ```
//!
//! ... flattened in pre-order on 3 [Vec], that store the data, the level/deep and the parent:
//!
//! | DATA:  | Users | jhon_doe | file1.rs | file2.rs | jane_doe | cat.jpg |
//! |--------|-------|----------|----------|----------|----------|---------|
//! | LEVEL:  | 0     | 1        | 2        | 2        | 1        | 2       |
//! | PARENT:| 0     | 0        | 1        | 1        | 0        | 4       |
//!
//! This allows for the performance of [Vec], on the most common operations
//! (critically: Push items + Iterate), and very efficient iterations of
//! [node::Node::parents]/[node::Node::children]/[node::Node::siblings], because
//! it just traverse the flat vectors.
//!
//! The iterators exploit this observations:
//!
//! * The children are at the right/up of the parent
//! * The parents are at the left/down of the children
//! * The siblings are all that share the same level
//!
//! # Examples
//! ```
//! use tree_flat::prelude::*;
//!
//! let mut tree = Tree::with_capacity("Users", 6);
//!
//! let mut root = tree.root_mut();
//!
//! let mut child = root.push("jhon_doe");
//! child.push("file1.rs");
//! child.push("file2.rs");
//!
//! let mut child = root.push("jane_doe");
//! child.push("cat.jpg");
//!
//! //The data is backed by vectors and arena-like ids on them:
//! assert_eq!(
//!    tree.as_data(),
//!    ["Users", "jhon_doe", "file1.rs", "file2.rs", "jane_doe", "cat.jpg",]
//! );
//! assert_eq!(tree.as_level(), [0, 1, 2, 2, 1, 2,]);
//! assert_eq!(tree.as_parents(), [0, 0, 1, 1, 0, 4,]);
//! //Pretty print the tree
//! println!("{}", tree);
//!
//! //Iterations is as inserted:
//! for f in &tree {
//!   dbg!(f);
//! }
//!
//! ```
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
/// Import this module for easy access to the Flat-tree
pub mod prelude {
    pub use crate::iter;
    pub use crate::node::{Node, NodeId, NodeMut};
    pub use crate::tree;
    pub use crate::tree::Tree;
}
