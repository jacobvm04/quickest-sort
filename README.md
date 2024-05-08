# Quickest Sort

Blazingly fast quick-sort implementation validated with `proptest`.

## Benchmarks

Run benchmarks with `cargo bench` (requires rust nightly).

Benchmark results on an M2 Max Macbook:

```sh
test tests::bench_quickest_sort_i32     ... bench:     412,916 ns/iter (+/- 36,892)
test tests::bench_selection_sort_i32    ... bench:  20,364,375 ns/iter (+/- 1,237,768)
test tests::bench_std_sort_i32          ... bench:     391,834 ns/iter (+/- 24,933)
test tests::bench_std_sort_unstable_i32 ... bench:     238,943 ns/iter (+/- 10,257)
```
