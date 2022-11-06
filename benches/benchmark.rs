use std::iter::StepBy;
use std::ops::RangeInclusive;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use ego_tree::NodeMut as ENodeMut;
use ego_tree::Tree as ETree;

use tree_flat::prelude::*;

const RUNS_SIMPLE: u64 = 100;
const RUNS_HIERARCHY: u64 = 100;

// Pick one of the child nodes at level 1 (first the 1st, then some around the middle)
// based on RUNS_HIERARCHY / 4 values...
fn _get_child_node(run: u64) -> u64 {
    assert!(RUNS_HIERARCHY >= 100);
    match run {
        0 => 1,
        25 => 3_628,
        50 => 5_644,
        75 => 8_101,
        100 => 22_978,
        _ => unreachable!("{}", run),
    }
}

// Pick of the highest node at the end
// based on RUNS_HIERARCHY / 4 values...
fn _get_parent_node(run: u64) -> u64 {
    assert!(RUNS_HIERARCHY >= 100);
    match run {
        0 => 18,
        25 => 3_618,
        50 => 12_843,
        75 => 27_693,
        100 => 48_168,
        _ => unreachable!("{}", run),
    }
}

// Generate a tree with 10 levels (this is with RUN=0)
// . 0
// ├── 1
// ├   ├── 2
// ├   ├   ├── 3
// ├   ├   ├── 4
// ├   ├   ├   ├── 5
// ├   ├   ├   ├── 6
// ├   ├   ├   ├   ├── 7
// ├   ├   ├   ├   ├── 8
// ├   ├   ├   ├   ├   ├── 9
// ├   ├   ├   ├   ├   ├── 10
// ├   ├   ├   ├   ├   ├   ├── 11
// ├   ├   ├   ├   ├   ├   ├── 12
// ├   ├   ├   ├   ├   ├   ├   ├── 13
// ├   ├   ├   ├   ├   ├   ├   ├── 14
// ├   ├   ├   ├   ├   ├   ├   ├   ├── 15
// ├   ├   ├   ├   ├   ├   ├   ├   ├── 16
// ├   ├   ├   ├   ├   ├   ├   ├   ├   ├── 17
// ├   ├   ├   ├   ├   ├   ├   ├   ├   ├── 18
// ├   ├   ├   ├   ├   ├   ├   ├   ├   ├   ├── 19
#[macro_export]
macro_rules! hierarchy {
    ($tree:ident, $root_mut:ident, $node_mut:ident, $push:ident) => {
        fn sub_level(mut parent: $node_mut<u64>, num: &mut u64, level: u64, count: u64) {
            if level > 10 {
                return;
            }
            *num += 1;
            let mut l = parent.$push(*num);
            for _x in 0..=count {
                *num += 1;
                l.$push(*num);
            }
            sub_level(l, num, level + 1, count);
            *num += 1;
        }

        pub(crate) fn _create_hierarchy(n: u64) -> $tree<u64> {
            let mut tree = $tree::new(0);
            let mut root = tree.$root_mut();
            let mut num = 1;
            for i in 0..=n {
                let l1 = root.$push(num);

                sub_level(l1, &mut num, 1, i);
            }

            tree
        }
    };
}

mod ego {
    use super::*;

    hierarchy!(ETree, root_mut, ENodeMut, append);

    pub(crate) fn create_hierarchy(n: u64) {
        _create_hierarchy(n);
    }

    pub(crate) fn iter_hierarchy(_run: u64, t: ETree<u64>) {
        for _x in t {}
    }

    pub(crate) fn iter_children(run: u64, t: ETree<u64>) {
        let idx = _get_child_node(run);
        let node = t.nodes().into_iter().find(|x| *x.value() == idx).unwrap();

        for _x in node.children() {}
    }

    pub(crate) fn iter_parent(run: u64, t: ETree<u64>) {
        let idx = _get_parent_node(run);
        let node = t.nodes().into_iter().find(|x| *x.value() == idx).unwrap();

        for _x in node.parent() {}
    }

    pub(crate) fn create(n: u64) {
        let mut tree = ETree::with_capacity(0, n as usize);
        let mut root = tree.root_mut();

        for i in 1..n {
            root.append(i);
        }
    }
}

mod flat {
    use super::*;

    hierarchy!(Tree, tree_root_mut, TreeMut, push);

    pub(crate) fn create_hierarchy(n: u64) {
        _create_hierarchy(n);
    }

    pub(crate) fn iter_hierarchy(_run: u64, t: Tree<u64>) {
        for _x in &t {}
    }

    pub(crate) fn iter_children(run: u64, t: Tree<u64>) {
        let idx = _get_child_node(run);
        let node = t.node((idx as usize).into()).unwrap();

        for _x in node.children() {}
    }

    pub(crate) fn iter_parent(run: u64, t: Tree<u64>) {
        let idx = _get_parent_node(run);
        let node = t.node((idx as usize).into()).unwrap();

        for _x in node.children() {}
    }

    pub(crate) fn create(n: u64) {
        let mut tree = Tree::with_capacity(0, n as usize);

        let mut root = tree.tree_root_mut();

        for i in 1..n {
            root.push(i);
        }
    }
}

pub fn make_benchmark<E, F>(
    c: &mut Criterion,
    name: &str,
    id: i32,
    range: StepBy<RangeInclusive<u64>>,
    f_ego: F,
    f_flat: E,
) where
    F: Fn(u64) -> (),
    E: Fn(u64) -> (),
{
    let mut group = c.benchmark_group(name);

    for runs in range {
        group.throughput(Throughput::Elements(runs as u64));

        group.bench_with_input(BenchmarkId::new("Ego", id), &runs, |b, i| {
            b.iter(|| f_ego(*i))
        });
        group.bench_with_input(BenchmarkId::new("Flat", id), &runs, |b, i| {
            b.iter(|| f_flat(*i))
        });
    }

    group.finish();
}

pub fn make_benchmark_prep<E, F, TE, TF>(
    c: &mut Criterion,
    name: &str,
    id: i32,
    range: StepBy<RangeInclusive<u64>>,
    t_ego: TE,
    f_ego: E,
    t_flat: TF,
    f_flat: F,
) where
    TE: Fn(u64) -> ETree<u64>,
    E: Fn(u64, ETree<u64>) -> (),
    TF: Fn(u64) -> Tree<u64>,
    F: Fn(u64, Tree<u64>) -> (),
{
    let mut group = c.benchmark_group(name);

    for runs in range {
        group.throughput(Throughput::Elements(runs as u64));

        let et = t_ego(runs);
        let ft = t_flat(runs);

        assert_eq!(
            ft.as_data(),
            et.values().copied().collect::<Vec<_>>().as_slice()
        );

        group.bench_with_input(BenchmarkId::new("Ego", id), &et, |b, i| {
            b.iter(|| f_ego(runs, i.clone()))
        });
        group.bench_with_input(BenchmarkId::new("Flat", id), &ft, |b, i| {
            b.iter(|| f_flat(runs, i.clone()))
        });
    }

    group.finish();
}

//Check creating tree with only 1 level level
pub fn create(c: &mut Criterion) {
    let range = (0..=RUNS_SIMPLE).step_by((RUNS_SIMPLE / 4) as usize);
    make_benchmark(c, "Create Tree Simple", 1, range, ego::create, flat::create)
}

//Check creating tree with at most 10 levels level
pub fn hierarchy(c: &mut Criterion) {
    let range = (0..=RUNS_HIERARCHY).step_by((RUNS_HIERARCHY / 4) as usize);
    make_benchmark(
        c,
        "Create Tree Hierarchy",
        2,
        range,
        ego::create_hierarchy,
        flat::create_hierarchy,
    )
}

// Check traversing the tree
pub fn hierarchy_iter(c: &mut Criterion) {
    let range = (0..=RUNS_HIERARCHY).step_by((RUNS_HIERARCHY / 4) as usize);
    make_benchmark_prep(
        c,
        "Iter Tree Hierarchy",
        3,
        range,
        ego::_create_hierarchy,
        ego::iter_hierarchy,
        flat::_create_hierarchy,
        flat::iter_hierarchy,
    )
}

// Check traversing the tree by children, selecting a root around the middle
pub fn iter_children(c: &mut Criterion) {
    let range = (0..=RUNS_HIERARCHY).step_by((RUNS_HIERARCHY / 4) as usize);
    make_benchmark_prep(
        c,
        "Iter Tree children",
        4,
        range,
        ego::_create_hierarchy,
        ego::iter_children,
        flat::_create_hierarchy,
        flat::iter_children,
    )
}

// Check traversing the tree by parent, selecting the tallest node around the end
pub fn iter_parents(c: &mut Criterion) {
    let range = (0..=RUNS_HIERARCHY).step_by((RUNS_HIERARCHY / 4) as usize);
    make_benchmark_prep(
        c,
        "Iter Tree parents",
        5,
        range,
        ego::_create_hierarchy,
        ego::iter_parent,
        flat::_create_hierarchy,
        flat::iter_parent,
    )
}

criterion_group!(
    benches,
    create,
    hierarchy,
    hierarchy_iter,
    iter_children,
    iter_parents
);
criterion_main!(benches);
