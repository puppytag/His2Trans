// simcrat 测试文件 for qsort
//
// 目标：真实排序正确性测试（与 evolc2rust/c2saferrust 语义一致，6 个 #[test]）。

use crate::*;

fn is_sorted(xs: &[i32]) -> bool {
    xs.windows(2).all(|w| w[0] <= w[1])
}

#[test]
fn test_small() {
    let mut arr = [3i32, 1, 2];
    let len = arr.len();
    quick_sort(&mut arr, 0..len);
    assert_eq!(arr, [1, 2, 3]);
}

#[test]
fn test_reverse() {
    let mut arr = [5i32, 4, 3, 2, 1];
    let len = arr.len();
    quick_sort(&mut arr, 0..len);
    assert!(is_sorted(&arr));
    assert_eq!(arr, [1, 2, 3, 4, 5]);
}

#[test]
fn test_negatives() {
    let mut arr = [0i32, -1, 5, -3, 2];
    let len = arr.len();
    quick_sort(&mut arr, 0..len);
    assert_eq!(arr, [-3, -1, 0, 2, 5]);
}

#[test]
fn test_duplicates() {
    let mut arr = [7i32, 7, 7, 7];
    let len = arr.len();
    quick_sort(&mut arr, 0..len);
    assert!(is_sorted(&arr));
    assert_eq!(arr, [7, 7, 7, 7]);
}

#[test]
fn test_single_element() {
    let mut arr = [42i32];
    let len = arr.len();
    quick_sort(&mut arr, 0..len);
    assert_eq!(arr, [42]);
}

#[test]
fn test_already_sorted() {
    let mut arr = [1i32, 2, 3, 4, 5];
    let len = arr.len();
    quick_sort(&mut arr, 0..len);
    assert!(is_sorted(&arr));
}
