use criterion::{black_box, criterion_group, criterion_main, Criterion};
use slang_solidity_v2_semantic::ir::Interner;
use rand::seq::SliceRandom;

fn generate_unique_strings(n: usize) -> Vec<String> {
    (0..n).map(|i| format!("identifier_{i}")).collect()
}

fn generate_shuffled_indices(n: usize, vocab: usize) -> Vec<usize> {
    let mut indices = Vec::with_capacity(n);
    for i in 0..n {
        indices.push(i % vocab);
    }
    indices.shuffle(&mut rand::thread_rng());
    indices
}

fn bench_all_twice(c: &mut Criterion) {
    let mut group = c.benchmark_group("twice");
    let strings = generate_unique_strings(1000);

    group.bench_function("intern", |b| {
        b.iter(
            || {
                let mut interner = Interner::new();
                for s in &strings {
                    black_box(interner.intern(black_box(s)));
                }
                for s in &strings {
                    black_box(interner.intern(black_box(s)));
                }
            }
        );
    });

    group.bench_function("intern_check_first", |b| {
        b.iter(
            || {
                let mut interner = Interner::new();
                for s in &strings {
                    black_box(interner.intern_check_first(black_box(s)));
                }
                for s in &strings {
                    black_box(interner.intern_check_first(black_box(s)));
                }
            }
        );
    });

    group.finish();
}

fn bench_shuffled(c: &mut Criterion) {
    let mut group = c.benchmark_group("3.5 times shuffled");
    let vocab_size = 1000;
    let num_ops = 3500;
    let vocab = generate_unique_strings(vocab_size);
    let indices = generate_shuffled_indices(num_ops, vocab_size);
    let lookup: Vec<&str> = indices.iter().map(|&i| vocab[i].as_str()).collect();

    group.bench_function("intern", |b| {
        b.iter(|| {
            let mut interner = Interner::new();
            for &s in &lookup {
                black_box(interner.intern(black_box(s)));
            }
        });
    });

    group.bench_function("intern_check_first", |b| {
        b.iter(|| {
            let mut interner = Interner::new();
            for &s in &lookup {
                black_box(interner.intern_check_first(black_box(s)));
            }
        });
    });

    group.finish();
}

criterion_group!(benches, bench_all_twice, bench_shuffled);
criterion_main!(benches);
