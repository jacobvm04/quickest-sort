#![feature(test)]

extern crate test;

/// Quick-sort in place
///
/// Example
/// ```
/// let mut array = vec![3, 1, 2];
/// quickest_sort::sort(&mut array);
///
/// assert_eq!(array, vec![1, 2, 3]);
/// ```
pub fn sort<T>(array: &mut [T])
where
    T: Ord,
{
    // base case
    if array.len() <= 1 {
        return;
    }

    // we need to use split_at_mut because we need to obtain a immutable reference to the pivot element
    // while taking mutable references to other elements in the array
    let (pivot, sub_array) = array.split_at_mut(1);
    let pivot = &pivot[0];

    let mut left_count = 0;

    for i in 0..sub_array.len() {
        if &sub_array[i] < pivot {
            sub_array.swap(left_count, i);

            left_count += 1;
        }
    }

    array.swap(0, left_count); // move the pivot to be between the left and right arrays

    // recursive calls
    sort(&mut array[..left_count]); // sort left sub-array
    sort(&mut array[(left_count + 1)..]); // sort right sub-array
}

/// baseline slow sorting algorithm to compare to
#[cfg(test)]
fn selection_sort(array: &mut [i32]) {
    if array.len() < 1 {
        return;
    }

    for i in 0..array.len() {
        let min_index = array[i..]
            .iter()
            .enumerate()
            .min_by(|num1, num2| num1.1.cmp(num2.1))
            .map(|(i, _val)| i)
            .unwrap_or_else(|| unreachable!());

        array.swap(i, i + min_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use test::Bencher;

    fn check_sorted<T>(array: &Vec<T>)
    where
        T: Ord,
    {
        for j in 1..array.len() {
            assert!(&array[j] >= &array[j - 1]);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig { cases: 1000, max_shrink_iters: 10_000, ..ProptestConfig::default() })]
        #[test]
        fn test_selection_sort_i32(mut array in prop::collection::vec(prop::num::i32::ANY, 0..=1_000)) {
            selection_sort(&mut array);
            check_sorted(&array);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig { cases: 1000, max_shrink_iters: 10_000, ..ProptestConfig::default() })]
        #[test]
        fn test_sort_i32(mut array in prop::collection::vec(prop::num::i32::ANY, 0..=1_000)) {
            sort(&mut array);
            check_sorted(&array);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig { cases: 1000, max_shrink_iters: 10_000, ..ProptestConfig::default() })]
        #[test]
        fn test_sort_char(mut array in prop::collection::vec(prop::char::any(), 0..=100)) {
            sort(&mut array);
            check_sorted(&array);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig { cases: 10, ..ProptestConfig::default() })]
        #[test]
        fn test_sort_string(mut array in prop::collection::vec(prop::arbitrary::any::<String>(), 0..=10_000)) {
            sort(&mut array);
            check_sorted(&array);
        }
    }

    #[bench]
    fn bench_std_sort_i32(b: &mut Bencher) {
        let _ = b.iter(|| {
            let mut array: Vec<i32> = test::black_box(
                (0..10_000)
                    .map(|_| rand::thread_rng().gen_range(-1000..=1000))
                    .collect(),
            );

            test::black_box(array.sort());
            test::black_box(check_sorted(&array));
        });
    }

    #[bench]
    fn bench_std_sort_unstable_i32(b: &mut Bencher) {
        let _ = b.iter(|| {
            let mut array: Vec<i32> = test::black_box(
                (0..10_000)
                    .map(|_| rand::thread_rng().gen_range(-1000..=1000))
                    .collect(),
            );

            test::black_box(array.sort_unstable());
            test::black_box(check_sorted(&array));
        });
    }

    #[bench]
    fn bench_selection_sort_i32(b: &mut Bencher) {
        let _ = b.iter(|| {
            let mut array: Vec<i32> = test::black_box(
                (0..10_000)
                    .map(|_| rand::thread_rng().gen_range(-1000..=1000))
                    .collect(),
            );

            test::black_box(selection_sort(&mut array));
            test::black_box(check_sorted(&array));
        });
    }

    #[bench]
    fn bench_quickest_sort_i32(b: &mut Bencher) {
        let _ = b.iter(|| {
            let mut array: Vec<i32> = test::black_box(
                (0..10_000)
                    .map(|_| rand::thread_rng().gen_range(-1000..=1000))
                    .collect(),
            );

            test::black_box(sort(&mut array));
            test::black_box(check_sorted(&array));
        });
    }
}
