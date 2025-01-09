use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{thread_rng, Rng};
use std::collections::{BTreeMap, HashMap, LinkedList, VecDeque};

fn bench_insertions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insertions");

    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("Vec", size), size, |b, &size| {
            b.iter(|| {
                let mut vec = Vec::with_capacity(size);
                for i in 0..size {
                    vec.push(black_box(i));
                }
            });
        });

        group.bench_with_input(BenchmarkId::new("VecDeque", size), size, |b, &size| {
            b.iter(|| {
                let mut deque = VecDeque::with_capacity(size);
                for i in 0..size {
                    deque.push_back(black_box(i));
                }
            });
        });

        group.bench_with_input(BenchmarkId::new("HashMap", size), size, |b, &size| {
            b.iter(|| {
                let mut map = HashMap::with_capacity(size);
                for i in 0..size {
                    map.insert(black_box(i), black_box(i));
                }
            });
        });

        group.bench_with_input(BenchmarkId::new("BTreeMap", size), size, |b, &size| {
            b.iter(|| {
                let mut btree = BTreeMap::new();
                for i in 0..size {
                    btree.insert(black_box(i), black_box(i));
                }
            });
        });
    }
    group.finish();
}

fn bench_lookups(c: &mut Criterion) {
    let mut group = c.benchmark_group("Lookups");
    let mut rng = thread_rng();

    for size in [100, 1000, 10000].iter() {
        // Prepare data structures
        let vec: Vec<i32> = (0..*size).collect();
        let mut map = HashMap::with_capacity((*size).try_into().unwrap());
        let mut btree = BTreeMap::new();
        for i in 0..*size {
            map.insert(i, i);
            btree.insert(i, i);
        }

        let lookup_keys: Vec<i32> = (0..*size).map(|_| rng.gen_range(0..*size)).collect();

        group.bench_with_input(BenchmarkId::new("Vec", size), size, |b, &_size| {
            b.iter(|| {
                for &key in lookup_keys.iter() {
                    let _ = black_box(vec.binary_search(&key));
                }
            });
        });

        group.bench_with_input(BenchmarkId::new("HashMap", size), size, |b, &_size| {
            b.iter(|| {
                for &key in lookup_keys.iter() {
                    black_box(map.get(&key));
                }
            });
        });

        group.bench_with_input(BenchmarkId::new("BTreeMap", size), size, |b, &_size| {
            b.iter(|| {
                for &key in lookup_keys.iter() {
                    black_box(btree.get(&key));
                }
            });
        });
    }
    group.finish();
}

fn bench_iterations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Iterations");

    for size in [100, 1000, 10000].iter() {
        let vec: Vec<i32> = (0..*size).collect();
        let deque: VecDeque<i32> = (0..*size).collect();
        let list: LinkedList<i32> = (0..*size).collect();

        group.bench_with_input(BenchmarkId::new("Vec", size), size, |b, &_size| {
            b.iter(|| {
                for item in vec.iter() {
                    black_box(item);
                }
            });
        });

        group.bench_with_input(BenchmarkId::new("VecDeque", size), size, |b, &_size| {
            b.iter(|| {
                for item in deque.iter() {
                    black_box(item);
                }
            });
        });

        group.bench_with_input(BenchmarkId::new("LinkedList", size), size, |b, &_size| {
            b.iter(|| {
                for item in list.iter() {
                    black_box(item);
                }
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_insertions, bench_lookups, bench_iterations);
criterion_main!(benches);
