# Code for *{n} times faster than C, where n = 128*

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
`opt4_simd`               | 40.748 µs | 22.856 GiB/s | 81.8
`opt5_simd_unrolled_2x`   | 32.277 µs | 28.854 GiB/s | 103.2
`opt5_simd_unrolled_4x`   | 28.265 µs | 32.950 GiB/s | 117.9
`opt5_simd_unrolled_8x`   | 26.323 µs | 35.380 GiB/s | 126.6
`opt5_simd_unrolled_10x`  | 25.896 µs | 35.964 GiB/s | 128.6 🎉
`opt5_simd_unrolled_12x`  | 27.697 µs | 33.626 GiB/s | 120.3
`opt5_simd_unrolled_16x`  | 26.954 µs | 34.553 GiB/s | 123.6
`opt6_chunk_count`[^1]    | 12.517 µs | 74.403 GiB/s | 266.2 🚀

[^1]: Code suggested by Reddit user [u/DavidM603](https://www.reddit.com/r/rust/comments/14yvlc9/comment/jrwkag7).
