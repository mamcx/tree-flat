// use std::fs::ReadDir;
// use std::path::PathBuf;
// use std::{fs, io};

use crate::tree::{NodeId, Tree};
use ego_tree::Tree as ETree;

fn build() -> Tree<i32> {
    let mut tree = Tree::with_capacity(0, 5);

    let mut root = tree.root_mut();
    root.push(1).push(2);

    let mut child3 = root.push(3);
    child3.push(4).push(5);
    child3.push(6);

    let mut child7 = root.push(7);
    let mut child8 = child7.push(8);

    child8.push(9);
    child8.push(10);

    let mut child11 = child7.push(11);
    child11.push(12);
    child11.push(13);

    child7.push(14);
    tree
}

#[test]
fn ego_create() {
    let mut tree = ETree::new(0);
    let mut root = tree.root_mut();

    for i in 1..10 {
        root.append(i);
    }

    println!("{tree:?}");
}

#[test]
fn create() {
    let tree = build();
    assert_eq!(tree.len(), 15);
    assert_eq!(tree.data.len(), 15);
    assert_eq!(tree.deep.len(), 15);
    assert_eq!(tree.parent.len(), 15);
}

#[test]
fn folder_mimic() {
    let mut tree = Tree::with_capacity("Users", 5);

    let mut root = tree.root_mut();

    let mut child = root.push("jhon_doe");
    child.push("file1.rs");
    child.push("file2.rs");
    let mut child = root.push("jane_doe");
    child.push("cat.jpg");

    assert_eq!(
        tree.as_data(),
        ["Users", "jhon_doe", "file1.rs", "file2.rs", "jane_doe", "cat.jpg",]
    );
    assert_eq!(tree.as_deep(), [0, 1, 2, 2, 1, 2,]);
}

//Confirm the data is iterated in pre-order (ie: as inserted)
#[test]
fn iter() {
    let tree = build();
    let mut data = Vec::with_capacity(tree.len());

    for x in tree.into_iter() {
        data.push(*x.data);
    }

    assert_eq!(data, tree.data);
}

#[test]
fn childs() {
    let tree = build();
    let child_of = 1;
    let parent = NodeId(child_of);

    let node = tree.node(parent).unwrap();

    for x in node.iter_childrens() {
        dbg!(x);
    }
}

//
// fn walk_dir(tree: &mut Tree<String>, parent: NodeId, of: ReadDir) -> io::Result<()> {
//     dbg!(parent);
//     for entry in of {
//         let entry = entry?;
//         let path = entry.path();
//         let metadata = fs::metadata(&path)?;
//         if metadata.is_dir() {
//             let root = tree.push(parent, path.to_str().unwrap().into());
//             dbg!("dir", root, parent);
//             walk_dir(tree, root, fs::read_dir(path)?)?;
//         } else {
//             dbg!("file");
//             tree.push(parent, path.to_str().unwrap().into());
//         }
//     }
//
//     Ok(())
// }
//
// #[test]
// fn files() -> io::Result<()> {
//     let mut tree = Tree::with_capacity(3);
//
//     let p: PathBuf = "/Users/mamcx/Proyectos/basura/eldiro/crates/ast".into();
//     let root = tree.root_mut(p.to_str().unwrap().into());
//     walk_dir(&mut tree, root, fs::read_dir(p.clone())?)?;
//     dbg!(&tree.deep);
//     println!("{tree}");
//
//     Ok(())
// }
