use crate::prelude::*;

// This is the tree used for the tests:
// . 0
// ├── 1
// ├   ├── 2
// ├── 3
// ├   ├── 4
// ├   ├   ├── 5
// ├   ├── 6
// ├── 7
// ├   ├── 8
// ├   ├   ├── 9
// ├   ├   ├── 10
// ├   ├── 11
// ├   ├   ├── 12
// ├   ├   ├── 13
// └────── 14
fn build() -> Tree<i32> {
    let mut tree = Tree::with_capacity(0, 15);

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

fn sub_level(mut parent: NodeMut<usize>, num: &mut usize, count: usize) {
    if parent.get_parent_level() > 10 {
        return;
    }
    *num += 1;
    let mut l = parent.push(*num);
    for _x in 0..=count {
        *num += 1;
        l.push(*num);
    }
    sub_level(l, num, count);
    *num += 1;
}

#[test]
fn create_hierarchy() {
    let n = 100;
    let mut tree = Tree::new(0);
    let mut root = tree.root_mut();
    let mut num = 1;
    for i in 0..=n {
        let l1 = root.push(num);

        sub_level(l1, &mut num, i);
    }

    dbg!(tree.len());
    println!("{tree}");
}

#[test]
fn create() {
    let tree = build();
    assert_eq!(tree.len(), 15);
    assert_eq!(tree.data.len(), 15);
    assert_eq!(tree.level.len(), 15);
    assert_eq!(tree.parent.len(), 15);
    assert_eq!(tree.node(14.into()).unwrap().data, &14);
    assert!(tree.node(15.into()).is_none());
    println!("{tree}");
}

#[test]
fn create_push_direct() {
    let mut tree1 = Tree::with_capacity(0, 3);
    let mut tree2 = tree1.clone();

    tree1.root_mut().push(1).push(3);
    tree1.root_mut().push(2);
    println!("{tree1}");
    let parent = tree2.push_with_level(1, 1, 0.into());
    tree2.push_with_level(3, 2, parent);
    tree2.push_with_level(2, 1, 0.into());
    println!("{tree2}");

    assert_eq!(tree1, tree2);
}

#[test]
fn create_manual() {
    let mut tree = Tree::new(0);

    let mut root = tree.root_mut();
    root.push(1);

    println!("{tree}");

    let mut tree = Tree::new(0);
    let parent = tree.root_mut().parent;
    tree.push_with_level(1, 1, parent);
    println!("{tree}");
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

fn make_childs(tree: &Tree<i32>, of_parent: usize) -> Vec<i32> {
    let parent = of_parent.into();

    let node = tree.node(parent).unwrap();

    node.children().map(|x| *x.data).collect()
}

#[test]
fn childs() {
    let tree = build();
    //println!("{tree}");
    let childs = make_childs(&tree, 0);
    assert_eq!(&tree.data[1..], childs.as_slice(), "of root");

    let childs = make_childs(&tree, 1);
    assert_eq!(&[2], childs.as_slice());

    let childs = make_childs(&tree, 3);
    assert_eq!(&[4, 5, 6], childs.as_slice());

    let childs = make_childs(&tree, 4);
    assert_eq!(&[5], childs.as_slice());

    let childs = make_childs(&tree, 7);
    assert_eq!(&[8, 9, 10, 11, 12, 13, 14], childs.as_slice());

    let childs = make_childs(&tree, 14);
    assert!(childs.is_empty());
}

fn make_parents(tree: &Tree<i32>, of_child: usize) -> Vec<i32> {
    let child = of_child.into();

    let node = tree.node(child).unwrap();

    node.parents().map(|x| *x.data).collect()
}

#[test]
fn parents() {
    let tree = build();
    //println!("{tree}");
    let parents = make_parents(&tree, 0);
    assert_eq!(&tree.data[1..1], parents.as_slice());

    let parents = make_parents(&tree, 1);
    assert_eq!(&[0], parents.as_slice());

    let parents = make_parents(&tree, 4);
    assert_eq!(&[3, 0], parents.as_slice());

    let parents = make_parents(&tree, 10);
    assert_eq!(&[8, 7, 0], parents.as_slice());

    let parents = make_parents(&tree, 14);
    assert_eq!(&[7, 0], parents.as_slice());
}

fn make_siblings(tree: &Tree<i32>, sibling_of: usize) -> Vec<i32> {
    let sibling = sibling_of.into();

    let node = tree.node(sibling).unwrap();

    node.siblings().map(|x| *x.data).collect()
}

#[test]
fn siblings() {
    let tree = build();
    //println!("{tree}");
    let siblings = make_siblings(&tree, 0);
    assert_eq!(&tree.data[1..1], siblings.as_slice());

    let siblings = make_siblings(&tree, 1);
    assert_eq!(&[3, 7], siblings.as_slice());

    let siblings = make_siblings(&tree, 2);
    assert_eq!(&[4, 6, 8, 11, 14], siblings.as_slice());

    let siblings = make_siblings(&tree, 10);
    assert_eq!(&[5, 9, 12, 13], siblings.as_slice());
}
