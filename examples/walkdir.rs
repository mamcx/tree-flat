use std::ffi::OsStr;
use std::fs::ReadDir;
use std::path::PathBuf;
use std::{env, fs, io};
use walkdir::WalkDir;

use tree_flat::prelude::*;

fn path_to_str(path: PathBuf) -> String {
    path.to_str().unwrap().to_string()
}

//Ignore hiddens and target
fn ignore(s: &OsStr) -> bool {
    let path = s.to_str().unwrap_or_default();
    path == "target" || path.starts_with(".")
}

fn _walk_dir_manual(mut parent: TreeMut<String>, of: ReadDir) -> io::Result<()> {
    for entry in of {
        let entry = entry?;
        let path = path_to_str(entry.path());
        if ignore(entry.path().file_name().unwrap_or_default()) {
            continue;
        }
        let metadata = fs::metadata(&path)?;
        //Note how we add things to each root/parent of the branch
        if metadata.is_dir() {
            let parent = parent.push(path.clone());
            _walk_dir_manual(parent, fs::read_dir(path)?)?;
        } else {
            parent.push(path);
        }
    }

    Ok(())
}

fn walk_dir_manual(path: &str) -> io::Result<Tree<String>> {
    let mut tree = Tree::new(path.into());

    let root = tree.tree_root_mut();
    //We start the recursive traversing...
    _walk_dir_manual(root, fs::read_dir(path)?)?;
    println!("{}", &tree);
    Ok(tree)
}

fn walk_dir(path: &str) -> io::Result<Tree<String>> {
    let mut tree = Tree::new(path.to_string());
    let mut parent = tree.tree_root_mut().parent;
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|f| !ignore(f.file_name()))
        .filter_map(|e| e.ok())
    {
        //Skip the root, we already have it...
        if entry.depth() == 0 {
            continue;
        }
        let node_id = tree.push_with_level(
            path_to_str(entry.path().to_path_buf()),
            entry.depth(),
            parent,
        );

        if entry.path().is_dir() {
            parent = node_id;
        }
    }
    println!("{}", &tree);
    Ok(tree)
}

fn main() -> io::Result<()> {
    let current = env::current_dir()?;
    let path = current.to_str().unwrap_or_default();
    //Using manual traversing, that build the flat_tree similar
    //to how it feels recursively, adding .childs to .parent
    let tree1 = walk_dir_manual(path)?;

    //Using walkdir crate, that traverse flat and give a .level, so is similar
    //to how it feels to push as vectors
    let tree2 = walk_dir(path)?;

    assert_eq!(tree1.to_string(), tree2.to_string());
    Ok(())
}
