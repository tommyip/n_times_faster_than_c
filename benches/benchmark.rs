use criterion::{criterion_group, criterion_main, Criterion};

use n_times_faster_than_c::*;

const INPUT_SIZE: usize = 1_000_000;

fn benchmark(c: &mut Criterion) {
    let input = gen_random_input(INPUT_SIZE);

    let mut group = c.benchmark_group("run_switches");
    group.throughput(criterion::Throughput::Bytes(INPUT_SIZE as u64));

    macro_rules! bench {
        ($fn_name:ident) => {
            group.bench_with_input(stringify!($fn_name), &input, |b, input| {
                b.iter(|| $fn_name(input));
            });
        };
    }

    bench!(baseline_unicode);
    bench!(baseline);
    bench!(opt1_idiomatic);
    bench!(opt2_count_s);
    bench!(opt3_count_s_branchless);
    bench!(opt4_simd);
    bench!(opt5_simd_unrolled_2x);
    bench!(opt5_simd_unrolled_4x);
    bench!(opt5_simd_unrolled_8x);
    bench!(opt5_simd_unrolled_10x);
    bench!(opt5_simd_unrolled_12x);
    bench!(opt5_simd_unrolled_16x);
    bench!(opt6_chunk_count);
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
