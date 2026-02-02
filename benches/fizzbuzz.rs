use fzbz::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn naive_bench() -> Vec<Answer> {
    fizzbuzz_all(naive, divan::black_box(2_000_000))
}

#[divan::bench]
fn mod_then_match_bench() -> Vec<Answer> {
    fizzbuzz_all(mod_then_match, divan::black_box(2_000_000))
}

#[divan::bench]
fn early_return_before_mod_bench() -> Vec<Answer> {
    fizzbuzz_all(early_return_before_mod, divan::black_box(2_000_000))
}

#[divan::bench]
fn single_string_scan_bench() -> Vec<Answer> {
    fizzbuzz_all(single_string_scan, divan::black_box(2_000_000))
}

#[divan::bench]
fn single_string_scan_early_fizzbuzz_bench() -> Vec<Answer> {
    fizzbuzz_all(
        single_string_scan_early_fizzbuzz,
        divan::black_box(2_000_000),
    )
}

#[divan::bench]
fn only_using_mod_bench() -> Vec<Answer> {
    fizzbuzz_all(only_using_mod, divan::black_box(2_000_000))
}

#[divan::bench]
fn only_using_mod_with_early_return_bench() -> Vec<Answer> {
    fizzbuzz_all(
        only_using_mod_with_early_return,
        divan::black_box(2_000_000),
    )
}
