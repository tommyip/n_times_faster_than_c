use rand::{distributions::Bernoulli, prelude::Distribution, thread_rng};
use std::arch::aarch64::{vaddlvq_u8, vaddq_u8, vld1q_dup_u8, vld1q_u8, vmovq_n_u8, vqsubq_u8};

pub fn baseline(input: &str) -> i64 {
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

pub fn baseline_bytes(input: &str) -> i64 {
    let bytes = input.as_bytes();
    let mut res = 0;
    for b in bytes {
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
        .as_bytes()
        .into_iter()
        .map(|&b| match b {
            b's' => 1,
            b'p' => -1,
            _ => 0,
        })
        .sum()
}

pub fn opt2_count_s(input: &str) -> i64 {
    let n_s = input.as_bytes().into_iter().filter(|&&b| b == b's').count();
    return (2 * n_s) as i64 - input.len() as i64;
}

pub fn opt3_simd_naive(input: &str) -> i64 {
    let n = input.len();
    const N_LANES: usize = 16;
    let n_blocks = n / N_LANES;
    let n_fast = n_blocks * N_LANES;
    let mut res = 0;
    unsafe {
        // saturating subtract ('s' - 1) from s/p
        // so we get 1 for s, 0 for p
        let mut acc_v = vmovq_n_u8(0);
        let sub_v = vld1q_dup_u8(&(b's' - 1));
        for block_i in 0..n_blocks {
            let input_v = vld1q_u8(input[block_i * N_LANES..].as_ptr());
            let eq_s_v = vqsubq_u8(input_v, sub_v);
            acc_v = vaddq_u8(acc_v, eq_s_v);
            if block_i % (u8::MAX as usize + 1) == u8::MAX as usize {
                res += vaddlvq_u8(acc_v) as i64;
                acc_v = vmovq_n_u8(0);
            }
        }
        res += vaddlvq_u8(acc_v) as i64;
    }
    res = (2 * res) - n_fast as i64;
    res + baseline_bytes(&input[n_fast..])
}

pub fn opt4_simd_unrolled(input: &str) -> i64 {
    let n = input.len();
    const N_LANES: usize = 16;
    const UNROLL_FACTOR: usize = 2;
    const N_ELEM_PER_ITER: usize = N_LANES * UNROLL_FACTOR;
    let n_blocks = n / N_ELEM_PER_ITER;
    let n_fast = n_blocks * N_ELEM_PER_ITER;
    let mut res = 0;
    unsafe {
        let mut acc1_v = vmovq_n_u8(0);
        let mut acc2_v = vmovq_n_u8(0);
        let sub_v = vld1q_dup_u8(&(b's' - 1));
        for block_i in 0..n_blocks {
            let offset = block_i * N_LANES * UNROLL_FACTOR;

            let input1_v = vld1q_u8(input[offset..].as_ptr());
            let eq_s1_v = vqsubq_u8(input1_v, sub_v);
            acc1_v = vaddq_u8(acc1_v, eq_s1_v);

            let input2_v = vld1q_u8(input[offset + N_LANES..].as_ptr());
            let eq_s2_v = vqsubq_u8(input2_v, sub_v);
            acc2_v = vaddq_u8(acc2_v, eq_s2_v);

            if block_i % (u8::MAX as usize + 1) == u8::MAX as usize {
                res += vaddlvq_u8(acc1_v) as i64;
                res += vaddlvq_u8(acc2_v) as i64;
                acc1_v = vmovq_n_u8(0);
                acc2_v = vmovq_n_u8(0);
            }
        }
        res += vaddlvq_u8(acc1_v) as i64;
        res += vaddlvq_u8(acc2_v) as i64;
    }
    res = (2 * res) - n_fast as i64;
    res + baseline_bytes(&input[n_fast..])
}

pub fn opt5_simd_unrolled_4x(input: &str) -> i64 {
    let n = input.len();
    const N_LANES: usize = 16;
    const UNROLL_FACTOR: usize = 4;
    const N_ELEM_PER_ITER: usize = N_LANES * UNROLL_FACTOR;
    let n_blocks = n / N_ELEM_PER_ITER;
    let n_fast = n_blocks * N_ELEM_PER_ITER;
    let mut res = 0;
    unsafe {
        let mut acc1_v = vmovq_n_u8(0);
        let mut acc2_v = vmovq_n_u8(0);
        let mut acc3_v = vmovq_n_u8(0);
        let mut acc4_v = vmovq_n_u8(0);
        let sub_v = vld1q_dup_u8(&(b's' - 1));
        for block_i in 0..n_blocks {
            let offset = block_i * N_LANES * UNROLL_FACTOR;

            let input1_v = vld1q_u8(input[offset..].as_ptr());
            let eq_s1_v = vqsubq_u8(input1_v, sub_v);
            acc1_v = vaddq_u8(acc1_v, eq_s1_v);

            let input2_v = vld1q_u8(input[offset + N_LANES..].as_ptr());
            let eq_s2_v = vqsubq_u8(input2_v, sub_v);
            acc2_v = vaddq_u8(acc2_v, eq_s2_v);

            let input3_v = vld1q_u8(input[offset + 2 * N_LANES..].as_ptr());
            let eq_s3_v = vqsubq_u8(input3_v, sub_v);
            acc3_v = vaddq_u8(acc3_v, eq_s3_v);

            let input4_v = vld1q_u8(input[offset + 3 * N_LANES..].as_ptr());
            let eq_s4_v = vqsubq_u8(input4_v, sub_v);
            acc4_v = vaddq_u8(acc4_v, eq_s4_v);

            if block_i % (u8::MAX as usize + 1) == u8::MAX as usize {
                res += vaddlvq_u8(acc1_v) as i64;
                res += vaddlvq_u8(acc2_v) as i64;
                res += vaddlvq_u8(acc3_v) as i64;
                res += vaddlvq_u8(acc4_v) as i64;
                acc1_v = vmovq_n_u8(0);
                acc2_v = vmovq_n_u8(0);
                acc3_v = vmovq_n_u8(0);
                acc4_v = vmovq_n_u8(0);
            }
        }
        res += vaddlvq_u8(acc1_v) as i64;
        res += vaddlvq_u8(acc2_v) as i64;
        res += vaddlvq_u8(acc3_v) as i64;
        res += vaddlvq_u8(acc4_v) as i64;
    }
    res = (2 * res) - n_fast as i64;
    res + baseline_bytes(&input[n_fast..])
}

pub fn opt6_simd_unrolled_8x(input: &str) -> i64 {
    let n = input.len();
    const N_LANES: usize = 16;
    const UNROLL_FACTOR: usize = 8;
    const N_ELEM_PER_ITER: usize = N_LANES * UNROLL_FACTOR;
    let n_blocks = n / N_ELEM_PER_ITER;
    let n_fast = n_blocks * N_ELEM_PER_ITER;
    let mut res = 0;
    unsafe {
        let mut acc1_v = vmovq_n_u8(0);
        let mut acc2_v = vmovq_n_u8(0);
        let mut acc3_v = vmovq_n_u8(0);
        let mut acc4_v = vmovq_n_u8(0);
        let mut acc5_v = vmovq_n_u8(0);
        let mut acc6_v = vmovq_n_u8(0);
        let mut acc7_v = vmovq_n_u8(0);
        let mut acc8_v = vmovq_n_u8(0);
        let sub_v = vld1q_dup_u8(&(b's' - 1));
        for block_i in 0..n_blocks {
            let offset = block_i * N_LANES * UNROLL_FACTOR;

            let input1_v = vld1q_u8(input[offset..].as_ptr());
            let eq_s1_v = vqsubq_u8(input1_v, sub_v);
            acc1_v = vaddq_u8(acc1_v, eq_s1_v);

            let input2_v = vld1q_u8(input[offset + N_LANES..].as_ptr());
            let eq_s2_v = vqsubq_u8(input2_v, sub_v);
            acc2_v = vaddq_u8(acc2_v, eq_s2_v);

            let input3_v = vld1q_u8(input[offset + 2 * N_LANES..].as_ptr());
            let eq_s3_v = vqsubq_u8(input3_v, sub_v);
            acc3_v = vaddq_u8(acc3_v, eq_s3_v);

            let input4_v = vld1q_u8(input[offset + 3 * N_LANES..].as_ptr());
            let eq_s4_v = vqsubq_u8(input4_v, sub_v);
            acc4_v = vaddq_u8(acc4_v, eq_s4_v);

            let input5_v = vld1q_u8(input[offset + 4 * N_LANES..].as_ptr());
            let eq_s5_v = vqsubq_u8(input5_v, sub_v);
            acc5_v = vaddq_u8(acc5_v, eq_s5_v);

            let input6_v = vld1q_u8(input[offset + 5 * N_LANES..].as_ptr());
            let eq_s6_v = vqsubq_u8(input6_v, sub_v);
            acc6_v = vaddq_u8(acc6_v, eq_s6_v);

            let input7_v = vld1q_u8(input[offset + 6 * N_LANES..].as_ptr());
            let eq_s7_v = vqsubq_u8(input7_v, sub_v);
            acc7_v = vaddq_u8(acc7_v, eq_s7_v);

            let input8_v = vld1q_u8(input[offset + 7 * N_LANES..].as_ptr());
            let eq_s8_v = vqsubq_u8(input8_v, sub_v);
            acc8_v = vaddq_u8(acc8_v, eq_s8_v);

            if block_i % (u8::MAX as usize + 1) == u8::MAX as usize {
                res += vaddlvq_u8(acc1_v) as i64;
                res += vaddlvq_u8(acc2_v) as i64;
                res += vaddlvq_u8(acc3_v) as i64;
                res += vaddlvq_u8(acc4_v) as i64;
                res += vaddlvq_u8(acc5_v) as i64;
                res += vaddlvq_u8(acc6_v) as i64;
                res += vaddlvq_u8(acc7_v) as i64;
                res += vaddlvq_u8(acc8_v) as i64;
                acc1_v = vmovq_n_u8(0);
                acc2_v = vmovq_n_u8(0);
                acc3_v = vmovq_n_u8(0);
                acc4_v = vmovq_n_u8(0);
                acc5_v = vmovq_n_u8(0);
                acc6_v = vmovq_n_u8(0);
                acc7_v = vmovq_n_u8(0);
                acc8_v = vmovq_n_u8(0);
            }
        }
        res += vaddlvq_u8(acc1_v) as i64;
        res += vaddlvq_u8(acc2_v) as i64;
        res += vaddlvq_u8(acc3_v) as i64;
        res += vaddlvq_u8(acc4_v) as i64;
        res += vaddlvq_u8(acc5_v) as i64;
        res += vaddlvq_u8(acc6_v) as i64;
        res += vaddlvq_u8(acc7_v) as i64;
        res += vaddlvq_u8(acc8_v) as i64;
    }
    res = (2 * res) - n_fast as i64;
    res + baseline_bytes(&input[n_fast..])
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

    #[test]
    fn test_correctness() {
        let input = "spssssp";
        let expected = 3;
        assert_eq!(expected, baseline(input));
        assert_eq!(expected, baseline_bytes(input));
        assert_eq!(expected, opt1_idiomatic(input));
        assert_eq!(expected, opt2_count_s(input));
        assert_eq!(expected, opt3_simd_naive(input));
        assert_eq!(expected, opt4_simd_unrolled(input));
        assert_eq!(expected, opt5_simd_unrolled_4x(input));
        assert_eq!(expected, opt6_simd_unrolled_8x(input));
    }

    #[test]
    fn test_correctness_large() {
        let input = gen_random_input(421337);
        let expected = baseline(&input);
        assert_eq!(expected, baseline_bytes(&input));
        assert_eq!(expected, opt1_idiomatic(&input));
        assert_eq!(expected, opt2_count_s(&input));
        assert_eq!(expected, opt3_simd_naive(&input));
        assert_eq!(expected, opt4_simd_unrolled(&input));
        assert_eq!(expected, opt5_simd_unrolled_4x(&input));
        assert_eq!(expected, opt6_simd_unrolled_8x(&input));
    }
}
