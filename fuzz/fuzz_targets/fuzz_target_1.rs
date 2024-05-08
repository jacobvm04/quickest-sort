#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

fn check_sorted(array: &Vec<i32>) {
    for j in 1..array.len() {
        assert!(&array[j] >= &array[j - 1]);
    }
}

#[derive(Arbitrary, PartialEq, Debug, Eq)]
struct SortInput(Vec<i32>);

fuzz_target!(|input: SortInput| {
    let mut array = input.0;

    quickest_sort::sort(&mut array);
    check_sorted(&mut array);
});
