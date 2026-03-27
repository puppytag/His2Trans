// Rust translation of qsort test.c
// Copyright (c) 2024
// Translated from C to Rust

use std::os::raw::c_int;

// External declaration for quickSort function
extern "C" {
    fn quickSort(arr: *mut c_int, low: c_int, high: c_int);
}

fn is_sorted(arr: &[i32]) -> bool {
    for i in 1..arr.len() {
        if arr[i - 1] > arr[i] {
            return false;
        }
    }
    true
}

fn assert_eq_arr(got: &[i32], want: &[i32]) {
    assert_eq!(got.len(), want.len(), "Array length mismatch");
    for i in 0..got.len() {
        assert_eq!(got[i], want[i], "Element mismatch at index {}", i);
    }
}

fn run_case(name: &str, arr: &mut [i32]) {
    if !arr.is_empty() {
        unsafe {
            quickSort(arr.as_mut_ptr(), 0, (arr.len() - 1) as c_int);
        }
    }
    if !is_sorted(arr) {
        eprintln!("[fail] {}: array not sorted", name);
        std::process::abort();
    }
}

fn main() {
    // Test case: small
    {
        let mut arr = [3, 1, 2];
        let want = [1, 2, 3];
        run_case("small", &mut arr);
        assert_eq_arr(&arr, &want);
    }

    // Test case: reverse
    {
        let mut arr = [5, 4, 3, 2, 1];
        let want = [1, 2, 3, 4, 5];
        run_case("reverse", &mut arr);
        assert_eq_arr(&arr, &want);
    }

    // Test case: negatives
    {
        let mut arr = [0, -1, 5, -3, 2];
        let want = [-3, -1, 0, 2, 5];
        run_case("negatives", &mut arr);
        assert_eq_arr(&arr, &want);
    }

    // Test case: duplicates
    {
        let mut arr = [7, 7, 7, 7];
        let want = [7, 7, 7, 7];
        run_case("duplicates", &mut arr);
        assert_eq_arr(&arr, &want);
    }

    // Test case: random_1000
    {
        const N: usize = 1000;
        let mut arr: Vec<i32> = Vec::with_capacity(N);

        // Simple LCG random number generator (seed = 1)
        let mut seed: u32 = 1;
        for _ in 0..N {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let val = ((seed >> 16) % 2000) as i32 - 1000;
            arr.push(val);
        }

        run_case("random_1000", &mut arr);
    }

    println!("qsort tests: ok");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small() {
        let mut arr = [3, 1, 2];
        let want = [1, 2, 3];
        run_case("small", &mut arr);
        assert_eq_arr(&arr, &want);
    }

    #[test]
    fn test_reverse() {
        let mut arr = [5, 4, 3, 2, 1];
        let want = [1, 2, 3, 4, 5];
        run_case("reverse", &mut arr);
        assert_eq_arr(&arr, &want);
    }

    #[test]
    fn test_negatives() {
        let mut arr = [0, -1, 5, -3, 2];
        let want = [-3, -1, 0, 2, 5];
        run_case("negatives", &mut arr);
        assert_eq_arr(&arr, &want);
    }

    #[test]
    fn test_duplicates() {
        let mut arr = [7, 7, 7, 7];
        let want = [7, 7, 7, 7];
        run_case("duplicates", &mut arr);
        assert_eq_arr(&arr, &want);
    }

    #[test]
    fn test_random_1000() {
        const N: usize = 1000;
        let mut arr: Vec<i32> = Vec::with_capacity(N);

        let mut seed: u32 = 1;
        for _ in 0..N {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let val = ((seed >> 16) % 2000) as i32 - 1000;
            arr.push(val);
        }

        run_case("random_1000", &mut arr);
    }
}
