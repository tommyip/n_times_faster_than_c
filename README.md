# Code for *{n} times faster than C, where n = 128*

*Actually, n = 290 🤯*

## Benchmark Setup

Rust version: `rustc 1.70.0 (90c541806 2023-05-31)`  
Run test: `cargo test`  
Run benchmark: `cargo bench`

Machine: Apple MacBook Pro 14-inch (2021)  
Processor: Apple M1 Pro  
Memory: 16 GB

Input size: 1,000,000 characters  
Input generation: `s` and `p` are chosen with randomly 50% probability.

## Benchmark Result

Function                  | Time      | Throughput   | Relative speed
------------------------- | --------- | ------------ | --------------
`baseline_unicode`        | 3.7511 ms | 254.24 MiB/s | 0.88
`baseline`                | 3.3316 ms | 286.25 MiB/s | 1
`opt1_idiomatic`          | 227.33 µs | 4.0968 GiB/s | 14.7
`opt2_count_s`            | 152.44 µs | 6.1096 GiB/s | 21.9
`opt3_count_s_branchless` | 72.902 µs | 12.775 GiB/s | 45.7
`opt4_simd`               | 43.131 µs | 21.593 GiB/s | 77.2
`opt5_simd_unrolled_2x`   | 32.810 µs | 28.385 GiB/s | 101.5
`opt5_simd_unrolled_4x`   | 28.524 µs | 32.650 GiB/s | 116.8
`opt5_simd_unrolled_8x`   | 26.518 µs | 35.120 GiB/s | 125.64
`opt5_simd_unrolled_10x`  | 26.070 µs | 35.724 GiB/s | 127.8 🎉
`opt5_simd_unrolled_12x`  | 27.833 µs | 33.461 GiB/s | 119.7
`opt5_simd_unrolled_16x`  | 27.157 µs | 34.293 GiB/s | 122.7
`opt6_chunk_count`[^1]    | 14.597 µs | 63.802 GiB/s | 228.2
`opt6_chunk_exact_count` [^2] | 11.489 µs | 81.060 GiB/s | 290.0 🚀

[^1]: Credit to Reddit user [u/DavidM603](https://www.reddit.com/r/rust/comments/14yvlc9/comment/jrwkag7).
[^2]: Credit to Reddit user [u/Sharlinator](https://www.reddit.com/r/rust/comments/14yvlc9/comment/jrwt29t).

## Credit

Thanks [u/DavidM603](https://www.reddit.com/user/DavidM603/) and [u/Sharlinator](https://www.reddit.com/user/Sharlinator/) for contributing even faster and cleaner solutions.

Thanks [@PeterFaiman](https://github.com/PeterFaiman) for catching and fixing an overflow problem in multiple optimizations ([#1](https://github.com/tommyip/n_times_faster_than_c/pull/1)).
