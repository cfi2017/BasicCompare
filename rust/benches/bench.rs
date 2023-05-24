
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use programrust::{benchmark, benchmark_inner, benchmark_simple, benchmark_simple_inner};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bench_timed 100ops", |b| b.iter(|| benchmark(black_box(100))));
    c.bench_function("bench_inner 100ops", |b| b.iter(|| benchmark_inner(black_box(100))));
    c.bench_function("bench_timed 1000ops", |b| b.iter(|| benchmark(black_box(1000))));
    c.bench_function("bench_inner 1000ops", |b| b.iter(|| benchmark_inner(black_box(1000))));

    c.bench_function("bench_simple_timed 100ops", |b| b.iter(|| benchmark_simple(black_box(100))));
    c.bench_function("bench_simple_inner 100ops", |b| b.iter(|| benchmark_simple_inner(black_box(100))));
    c.bench_function("bench_simple_timed 1000ops", |b| b.iter(|| benchmark_simple(black_box(1000))));
    c.bench_function("bench_simple_inner 1000ops", |b| b.iter(|| benchmark_simple_inner(black_box(1000))));

    let mut g = c.benchmark_group("simple_bench");

    g.bench_function("bench_timed 1000ops", |b| b.iter(|| benchmark(black_box(1000))));
    g.bench_function("bench_inner 1000ops", |b| b.iter(|| benchmark_inner(black_box(1000))));
    g.bench_function("bench_simple_timed 1000ops", |b| b.iter(|| benchmark_simple(black_box(1000))));
    g.bench_function("bench_simple_inner 1000ops", |b| b.iter(|| benchmark_simple_inner(black_box(1000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);