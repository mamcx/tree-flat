//! # TreeFlat is the simplest way to build & traverse a pre-order Tree.
//!
//! If you build a [tree::Tree] *in pre-order*, and display *in pre-order*,
//! this is the tree for you.
//!
//! **No extra fluff**, just a simple & performant one-trick pony.

//! ## How it works

//! Instead of creating an [tree::Tree] of pointers, nested enums, or `Arena`-based `ids` it just stores the representation of a [tree::Tree] like:

//! ```bash
//! .Users
//! ├──jhon_doe
//! | ├── file1.rs
//! | ├── file2.rs
//! ├──jane_doe
//! └── cat.jpg
//! ```
//!
//! ... flattened in pre-order on 2 [Vec], that store the data and the deep:
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

pub mod iter;
#[cfg(test)]
mod tests;
pub mod tree;
