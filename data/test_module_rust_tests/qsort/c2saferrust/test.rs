// c2saferrust 测试文件 for qsort
// 目标：与 evolc2rust 套件对齐的真实排序正确性测试（仅使用公开的 quickSort）。

use crate::quickSort;

fn is_sorted(xs: &[i32]) -> bool {
    xs.windows(2).all(|w| w[0] <= w[1])
}

#[test]
fn test_small() {
    let mut arr = [3i32, 1, 2];
    unsafe {
        quickSort(arr.as_mut_ptr(), 0, (arr.len() - 1) as i32);
    }
    assert_eq!(arr, [1, 2, 3]);
}

#[test]
fn test_reverse() {
    let mut arr = [5i32, 4, 3, 2, 1];
    unsafe {
        quickSort(arr.as_mut_ptr(), 0, (arr.len() - 1) as i32);
    }
    assert!(is_sorted(&arr));
    assert_eq!(arr, [1, 2, 3, 4, 5]);
}

#[test]
fn test_negatives() {
    let mut arr = [0i32, -1, 5, -3, 2];
    unsafe {
        quickSort(arr.as_mut_ptr(), 0, (arr.len() - 1) as i32);
    }
    assert_eq!(arr, [-3, -1, 0, 2, 5]);
}

#[test]
fn test_duplicates() {
    let mut arr = [7i32, 7, 7, 7];
    unsafe {
        quickSort(arr.as_mut_ptr(), 0, (arr.len() - 1) as i32);
    }
    assert!(is_sorted(&arr));
    assert_eq!(arr, [7, 7, 7, 7]);
}

#[test]
fn test_single_element() {
    let mut arr = [42i32];
    unsafe {
        quickSort(arr.as_mut_ptr(), 0, 0);
    }
    assert_eq!(arr, [42]);
}

#[test]
fn test_already_sorted() {
    let mut arr = [1i32, 2, 3, 4, 5];
    unsafe {
        quickSort(arr.as_mut_ptr(), 0, (arr.len() - 1) as i32);
    }
    assert!(is_sorted(&arr));
}

