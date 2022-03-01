# TreeFlat is the simplest way to build & traverse a pre-order Tree for Rust.

### Alpha-relase!

If you build a `Tree` *in pre-order*, and display *in pre-order*,
this is the tree for you.

**No extra fluff**, just a simple & performant one-trick pony.

Note: The tree depends in the build order, so is not possible to re-order the tree
(changing parents or levels) in different order. So, for example, you can't add
a branch later to one in the *middle* (only can add *after* the end...).

## How it works

Instead of creating an Tree of Node pointers, nested enums, or nested `Arena`-based `ids`,
it just stores the representation of a Tree like:

```bash
. Users
├── jhon_doe
├   ├── file1.rs
├   ├── file2.rs
├── jane_doe
└────── cat.jpg
```

... flattened in pre-order on 3 vectors, that store the data, the level/level & the parent:

| DATA:  | Users | jhon_doe | file1.rs | file2.rs | jane_doe | cat.jpg |
|--------|-------|----------|----------|----------|----------|---------|
| level:  | 0     | 1        | 2        | 2        | 1        | 2       |
| PARENT:| 0     | 0        | 1        | 1        | 0        | 4       |

This allows for the performance of Rust `Vec`, on the most common operations
(critically: Push items + Iterate), and very efficient iterations of
`node::Node::parents`/`node::Node::children`/`node::Node::siblings`, because
it just traverse the flat vectors.

The iterators exploit this observations:

* The children are at the right/up of the parent
* The parents are at the left/down of the children
* The siblings are all that share the same level

# Examples
```rust
use tree_flat::prelude::*;

let mut tree = Tree::with_capacity("Users", 6);

let mut root = tree.root_mut();

let mut child = root.push("jhon_doe");
child.push("file1.rs");
child.push("file2.rs");

let mut child = root.push("jane_doe");
child.push("cat.jpg");

//The data is backed by vectors and arena-like ids on them:
assert_eq!(
   tree.as_data(),
   ["Users", "jhon_doe", "file1.rs", "file2.rs", "jane_doe", "cat.jpg",]
);
assert_eq!(tree.as_level(), [0, 1, 2, 2, 1, 2,]);
assert_eq!(tree.as_parents(), [0, 0, 1, 1, 0, 4,]);
//Pretty print the tree
println!("{}", tree);

//Iterations is as inserted:
for f in &tree {
  dbg!(f);
}

```
- - - - - -

Inspired by the talk:

> “High-performance Tree Wrangling, the APL Way”
> -- <cite> [Aaron Hsu - APL Wiki](https://aplwiki.com/wiki/Aaron_Hsu)

## 🤝 Contributing

Contributions, issues, and feature requests are welcome!

## Show your support

Give a ⭐️ if you like this project! or to help make my projects a reality consider donate or sponsor with a subscription in [https://www.buymeacoffee.com/mamcx](https://www.buymeacoffee.com/mamcx).

## 📝 License

This project is dual licenced as [MIT](./LICENSE-MIT) & [APACHE](./LICENSE-APACHE).