use rand::{distributions::Bernoulli, prelude::Distribution, thread_rng};
use seq_macro::seq;
use std::arch::aarch64::{vaddlvq_u8, vaddq_u8, vandq_u8, vld1q_u8, vmovq_n_u8};

pub fn baseline_unicode(input: &str) -> i64 {
    let mut res = 0;
    for c in input.chars() {
        match c {
            's' => res += 1,
            'p' => res -= 1,
            _ => continue,
        }
    }
    res
}

pub fn baseline(input: &str) -> i64 {
    let mut res = 0;
    for b in input.bytes() {
        match b {
            b's' => res += 1,
            b'p' => res -= 1,
            _ => continue,
        }
    }
    res
}

pub fn opt1_idiomatic(input: &str) -> i64 {
    input
        .bytes()
        .map(|b| match b {
            b's' => 1,
            b'p' => -1,
            _ => 0,
        })
        .sum()
}

pub fn opt2_count_s(input: &str) -> i64 {
    let n_s = input.bytes().filter(|&b| b == b's').count();
    (2 * n_s) as i64 - input.len() as i64
}

pub fn opt3_count_s_branchless(input: &str) -> i64 {
    let n_s = input.bytes().map(|b| (b & 1) as i64).sum::<i64>();
    (2 * n_s) - input.len() as i64
}

pub fn opt4_simd(input: &str) -> i64 {
    let n = input.len();
    const N_LANES: usize = 16;
    let n_iters = n / N_LANES;
    let n_simd_elems = n_iters * N_LANES;
    let mut res = 0;
    unsafe {
        let mut acc_v = vmovq_n_u8(0);
        let one_v = vmovq_n_u8(1);
        for block_i in 0..n_iters {
            let input_v = vld1q_u8(input[block_i * N_LANES..].as_ptr());
            let eq_s_v = vandq_u8(input_v, one_v);
            acc_v = vaddq_u8(acc_v, eq_s_v);
            if block_i % u8::MAX as usize == (u8::MAX - 1) as usize {
                res += vaddlvq_u8(acc_v) as i64;
                acc_v = vmovq_n_u8(0);
            }
        }
        res += vaddlvq_u8(acc_v) as i64;
    }
    res = (2 * res) - n_simd_elems as i64;
    res + baseline(&input[n_simd_elems..])
}

macro_rules! simd_unrolled {
    ($func_name:ident, $unroll_factor:literal) => {
         pub fn $func_name(input: &str) -> i64 {
            let n = input.len();
            const N_LANES: usize = 16;
            const N_ELEM_PER_ITER: usize = N_LANES * $unroll_factor;
            let n_iters = n / N_ELEM_PER_ITER;
            let n_simd_elems = n_iters * N_ELEM_PER_ITER;
            let mut res = 0;
            unsafe {
                seq!(I in 0..$unroll_factor {
                    let mut v_acc~I = vmovq_n_u8(0);
                });
                let one_v = vmovq_n_u8(1);
                for block_i in 0..n_iters {
                    let offset = block_i * N_LANES * $unroll_factor;
                    seq!(I in 0..$unroll_factor {
                        let v_input~I = vld1q_u8(input[offset + I * N_LANES..].as_ptr());
                        let v_eq_s~I= vandq_u8(v_input~I, one_v);
                        v_acc~I = vaddq_u8(v_acc~I, v_eq_s~I);
                    });
                    if block_i % u8::MAX as usize == (u8::MAX - 1) as usize {
                        seq!(I in 0..$unroll_factor {
                            res += vaddlvq_u8(v_acc~I) as i64;
                            v_acc~I = vmovq_n_u8(0);
                        });
                    }
                }
                seq!(I in 0..$unroll_factor {
                    res += vaddlvq_u8(v_acc~I) as i64;
                });
            }
            res = (2 * res) - n_simd_elems as i64;
            res + baseline(&input[n_simd_elems..])
        }
    }
}

simd_unrolled!(opt5_simd_unrolled_2x, 2);
simd_unrolled!(opt5_simd_unrolled_4x, 4);
simd_unrolled!(opt5_simd_unrolled_8x, 8);
simd_unrolled!(opt5_simd_unrolled_10x, 10);
simd_unrolled!(opt5_simd_unrolled_12x, 12);
simd_unrolled!(opt5_simd_unrolled_16x, 16);

/// Credit to u/DavidM603
/// https://www.reddit.com/r/rust/comments/14yvlc9/comment/jrwkag7
pub fn opt6_chunk_count(input: &str) -> i64 {
    let n_s = input
        .as_bytes()
        .chunks(192)
        .map(|chunk| chunk.iter().map(|&b| b & 1).sum::<u8>())
        .map(|chunk_total| chunk_total as i64)
        .sum::<i64>();
    (2 * n_s) - input.len() as i64
}

/// Credit to u/Sharlinator
/// https://www.reddit.com/r/rust/comments/14yvlc9/comment/jrwt29t
pub fn opt6_chunk_exact_count(input: &str) -> i64 {
    let iter = input.as_bytes().chunks_exact(192);
    let rest = iter.remainder();
    let mut n_s = iter
        .map(|chunk| chunk.iter().map(|&b| b & 1).sum::<u8>())
        .map(|chunk_total| chunk_total as i64)
        .sum::<i64>();
    n_s += rest.iter().map(|&b| b & 1).sum::<u8>() as i64;
    (2 * n_s) - input.len() as i64
}

pub fn gen_random_input(size: usize) -> String {
    let mut input = String::with_capacity(size);
    let dist = Bernoulli::new(0.5).unwrap();
    dist.sample_iter(&mut thread_rng())
        .take(size)
        .map(|b| if b { 's' } else { 'p' })
        .for_each(|c| input.push(c));
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_eq_all {
        ($expected:expr, $input:expr) => {
            assert_eq!($expected, baseline_unicode($input));
            assert_eq!($expected, baseline($input));
            assert_eq!($expected, opt1_idiomatic($input));
            assert_eq!($expected, opt2_count_s($input));
            assert_eq!($expected, opt3_count_s_branchless($input));
            assert_eq!($expected, opt4_simd($input));
            assert_eq!($expected, opt5_simd_unrolled_2x($input));
            assert_eq!($expected, opt5_simd_unrolled_4x($input));
            assert_eq!($expected, opt5_simd_unrolled_8x($input));
            assert_eq!($expected, opt5_simd_unrolled_10x($input));
            assert_eq!($expected, opt5_simd_unrolled_12x($input));
            assert_eq!($expected, opt5_simd_unrolled_16x($input));
            assert_eq!($expected, opt6_chunk_count($input));
            assert_eq!($expected, opt6_chunk_exact_count($input));
        };
    }

    #[test]
    fn test_simple() {
        assert_eq_all!(3, "spssssp");
        assert_eq_all!(100, &(0..100).into_iter().map(|_| 's').collect::<String>());
        assert_eq_all!(-100, &(0..100).into_iter().map(|_| 'p').collect::<String>());
    }

    #[test]
    fn test_large() {
        let input = gen_random_input(421337);
        let expected = baseline_unicode(&input);
        assert_eq_all!(expected, &input);
    }

    #[test]
    fn test_all_s() {
        let expected = 1024 * 1024;
        let input = "s".repeat(expected);
        assert_eq_all!(expected as i64, &input);
    }
}
