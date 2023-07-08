use criterion::{black_box, criterion_group, criterion_main, Criterion};

use n_times_faster_than_c::*;

const INPUT_SIZE: usize = 1_000_000;

fn benchmark(c: &mut Criterion) {
    let input = gen_random_input(INPUT_SIZE);

    c.bench_function("baseline (unicode)", |b| {
        b.iter(|| baseline(black_box(&input)))
    });
    c.bench_function("baseline", |b| b.iter(|| baseline_bytes(black_box(&input))));
    c.bench_function("idiomatic", |b| {
        b.iter(|| opt1_idiomatic(black_box(&input)))
    });
    c.bench_function("count s", |b| b.iter(|| opt2_count_s(black_box(&input))));
    c.bench_function("simd", |b| b.iter(|| opt3_simd(black_box(&input))));
    c.bench_function("simd unrolled 2x", |b| {
        b.iter(|| opt4_simd_unrolled_2x(black_box(&input)))
    });
    c.bench_function("simd unrolled 4x", |b| {
        b.iter(|| opt5_simd_unrolled_4x(black_box(&input)))
    });
    c.bench_function("simd unrolled 8x", |b| {
        b.iter(|| opt6_simd_unrolled_8x(black_box(&input)))
    });
    c.bench_function("simd unrolled 16x", |b| {
        b.iter(|| opt7_simd_unrolled_16x(black_box(&input)))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
