use std::hint::black_box;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};

use sums::{
    BLOCK, chunked_sum, expanded_fold_sum, fold_sum, for_sum, iter_sum, wide_sum_fold0,
    wide_sum_fold1, wide_sum_fold2,
};

const CASES: &[(usize, &str)] = &[
    (BLOCK * BLOCK / 2 - 1, "N = BLOCK²/2 - 1"),
    (BLOCK * BLOCK / 2, "N = BLOCK²/2"),
    (BLOCK * BLOCK / 2 + 1, "N = BLOCK²/2 + 1"),
    (200_000_000, "N = 200M"),
];

const FUNCS: &[(&str, fn(&[f64]) -> f64)] = &[
    ("for_sum", for_sum),
    ("iter_sum", iter_sum),
    ("fold_sum", fold_sum),
    ("chunked_sum", chunked_sum),
    ("wide_sum_fold0", wide_sum_fold0),
    ("wide_sum_fold1", wide_sum_fold1),
    ("wide_sum_fold2", wide_sum_fold2),
    ("expanded_fold_sum", expanded_fold_sum),
];

fn bench_sums(c: &mut Criterion) {
    for &(n, label) in CASES {
        let mut group = c.benchmark_group(label);
        group.throughput(Throughput::Elements(n as u64));

        let data: Vec<f64> = (1..=n).rev().map(|x| x as f64).collect();

        for &(name, func) in FUNCS {
            group.bench_function(name, |b| b.iter(|| func(black_box(&data))));
        }

        group.finish();
    }
}

criterion_group!(benches, bench_sums);
criterion_main!(benches);
