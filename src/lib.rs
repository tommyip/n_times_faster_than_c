use rand::{distributions::Bernoulli, prelude::Distribution, thread_rng};
use std::arch::aarch64;

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
        let mut acc_v = aarch64::vmovq_n_u8(0);
        let sub_v = aarch64::vld1q_dup_u8(&(b's' - 1));
        for block_i in 0..n_blocks {
            let input_v = aarch64::vld1q_u8(input[block_i * N_LANES..].as_ptr());
            let eq_s_v = aarch64::vqsubq_u8(input_v, sub_v);
            acc_v = aarch64::vaddq_u8(acc_v, eq_s_v);
            if block_i % (u8::MAX as usize + 1) == u8::MAX as usize {
                res += aarch64::vaddlvq_u8(acc_v) as i64;
                acc_v = aarch64::vmovq_n_u8(0);
            }
        }
        res += aarch64::vaddlvq_u8(acc_v) as i64;
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
    }

    #[test]
    fn test_correctness_large() {
        let input = gen_random_input(421337);
        let expected = baseline(&input);
        assert_eq!(expected, baseline_bytes(&input));
        assert_eq!(expected, opt1_idiomatic(&input));
        assert_eq!(expected, opt2_count_s(&input));
        assert_eq!(expected, opt3_simd_naive(&input));
    }
}
