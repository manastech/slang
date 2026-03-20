use criterion::{black_box, criterion_group, criterion_main, Criterion};
use slang_solidity_v2_semantic::ir::Interner;

fn generate_unique_strings(n: usize) -> Vec<String> {
    (0..n).map(|i| format!("identifier_{i}")).collect()
}

fn generate_zipf_indices(n: usize, vocab: usize) -> Vec<usize> {
    // Simple Zipf-like distribution: index i has weight 1/(i+1)
    // Use deterministic sequence for reproducibility
    let mut indices = Vec::with_capacity(n);
    let mut acc = 0u64;
    for i in 0..n {
        // Simple hash-like deterministic mapping biased toward lower indices
        let raw = (i as u64).wrapping_mul(2654435761) % (vocab as u64 * 4);
        let idx = (raw as f64).sqrt() as usize % vocab;
        indices.push(idx);
        acc = acc.wrapping_add(raw); // prevent dead-code elimination
    }
    let _ = acc;
    indices
}

fn bench_all_unique(c: &mut Criterion) {
    let mut group = c.benchmark_group("all_unique");
    let strings = generate_unique_strings(10_000);

    group.bench_function("intern", |b| {
        b.iter(|| {
            let mut interner = Interner::new();
            for s in &strings {
                black_box(interner.intern(black_box(s)));
            }
        });
    });

    group.bench_function("intern_check_first", |b| {
        b.iter(|| {
            let mut interner = Interner::new();
            for s in &strings {
                black_box(interner.intern_check_first(black_box(s)));
            }
        });
    });

    group.finish();
}

fn bench_all_duplicates(c: &mut Criterion) {
    let mut group = c.benchmark_group("all_duplicates");
    let strings = generate_unique_strings(1_000);

    group.bench_function("intern", |b| {
        b.iter_batched(
            || {
                let mut interner = Interner::new();
                for s in &strings {
                    interner.intern(s);
                }
                interner
            },
            |mut interner| {
                for s in &strings {
                    black_box(interner.intern(black_box(s)));
                }
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.bench_function("intern_check_first", |b| {
        b.iter_batched(
            || {
                let mut interner = Interner::new();
                for s in &strings {
                    interner.intern(s);
                }
                interner
            },
            |mut interner| {
                for s in &strings {
                    black_box(interner.intern_check_first(black_box(s)));
                }
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.finish();
}

fn bench_mixed(c: &mut Criterion) {
    let mut group = c.benchmark_group("mixed");
    let vocab_size = 500;
    let num_ops = 10_000;
    let vocab = generate_unique_strings(vocab_size);
    let indices = generate_zipf_indices(num_ops, vocab_size);
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

criterion_group!(benches, bench_all_unique, bench_all_duplicates, bench_mixed);
criterion_main!(benches);
