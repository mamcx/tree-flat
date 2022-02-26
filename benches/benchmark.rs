use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use ego_tree::Tree as ETree;
use tree_flat::tree::{NodeId, Tree};

#[inline]
fn flat_create(n: u64) -> Tree<u64> {
    let mut tree = Tree::new();

    let root = tree.root_mut(0);

    for i in 1..n {
        tree.add_child(root, i);
    }

    tree
}

#[inline]
fn ego_create(n: u64) -> ETree<u64> {
    let mut tree = ETree::new(0);
    let mut root = tree.root_mut();

    for i in 1..n {
        root.append(i);
    }

    tree
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Trees");
    let id = 1;
    let runs = &1000u64;

    group.bench_with_input(BenchmarkId::new("Ego", id), runs, |b, i| {
        b.iter(|| ego_create(*i))
    });
    group.bench_with_input(BenchmarkId::new("Flat", id), runs, |b, i| {
        b.iter(|| flat_create(*i))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
