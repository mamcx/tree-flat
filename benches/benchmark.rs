use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use ego_tree::NodeMut as ENodeMut;
use ego_tree::Tree as ETree;
use std::iter::StepBy;
use std::ops::{Range, RangeInclusive};
use tree_flat::prelude::*;

mod ego {
    use super::*;

    fn sub_level(mut parent: ENodeMut<usize>, num: &mut usize, deep: &mut usize, count: u64) {
        if *deep > 10 {
            return;
        }
        *num += 1;
        let mut l = parent.append(*num);
        for _x in 0..=count {
            *num += 1;
            l.append(*num);
        }
        *deep += 1;
        sub_level(l, num, deep, count);
        *num += 1;
    }

    pub(crate) fn create_hierarchy(n: u64) {
        let mut tree = ETree::new(0);
        let mut root = tree.root_mut();
        let mut num = 1;
        let mut deep = 1;
        for i in 0..=n {
            let l1 = root.append(num);

            sub_level(l1, &mut num, &mut deep, i);
        }
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

    fn sub_level(mut parent: NodeMut<usize>, num: &mut usize, count: u64) {
        if parent.deep() > 10 {
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

    pub(crate) fn create_hierarchy(n: u64) {
        let mut tree = Tree::new(0);
        let mut root = tree.root_mut();
        let mut num = 1;
        for i in 0..=n {
            let l1 = root.push(num);

            sub_level(l1, &mut num, i);
        }
    }

    pub(crate) fn create(n: u64) {
        let mut tree = Tree::with_capacity(0, n as usize);

        let mut root = tree.root_mut();

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

//Check creating tree with only 1 level deep
pub fn create(c: &mut Criterion) {
    let range = (0..=10_000).step_by(2_500);
    make_benchmark(c, "Create Tree Simple", 1, range, ego::create, flat::create)
}

//Check creating tree with at most 10 levels deep
pub fn hierarchy(c: &mut Criterion) {
    let range = (0..=1_000).step_by(250);
    make_benchmark(
        c,
        "Create Tree Hierarchy",
        2,
        range,
        ego::create_hierarchy,
        flat::create_hierarchy,
    )
}

criterion_group!(benches, create, hierarchy);
criterion_main!(benches);
