/// Create a virtual representation of a folder structure
use tree_flat::prelude::*;

fn main() {
    let mut tree = Tree::with_capacity("Users", 6);

    let mut root = tree.root_mut();

    let mut child = root.push("jhon_doe");
    child.push("file1.rs");
    child.push("file2.rs");

    let mut child = root.push("jane_doe");
    child.push("cat.jpg");

    //All  the data is flat!
    assert_eq!(
        tree.as_data(),
        ["Users", "jhon_doe", "file1.rs", "file2.rs", "jane_doe", "cat.jpg",]
    );
    assert_eq!(tree.as_deep(), [0, 1, 2, 2, 1, 2,]);
    assert_eq!(tree.as_parents(), [0, 0, 1, 1, 0, 4,]);

    println!("{}", &tree);

    //Iteration is in pre-order, as was build:
    for path in &tree {
        let level = path.deep();
        let parent = path.parent();
        println!("LEVEL {} / PARENT: {} : {}", level, parent, path);
    }
}
