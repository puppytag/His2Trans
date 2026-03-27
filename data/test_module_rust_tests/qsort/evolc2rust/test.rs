// evo-c2rust-v2 测试文件 for qsort
// 目标：真实排序正确性测试（仅使用公开的 quickSort）。

use crate::src::qsort_c::quickSort;

fn is_sorted(xs: &[i32]) -> bool {
    xs.windows(2).all(|w| w[0] <= w[1])
}

#[test]
fn test_small() {
    unsafe {
        let mut arr = [3i32, 1, 2];
        quickSort(arr.as_mut_ptr(), 0, (arr.len() - 1) as i32);
        assert_eq!(arr, [1, 2, 3]);
    }
}

#[test]
fn test_reverse() {
    unsafe {
        let mut arr = [5i32, 4, 3, 2, 1];
        quickSort(arr.as_mut_ptr(), 0, (arr.len() - 1) as i32);
        assert!(is_sorted(&arr));
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}

#[test]
fn test_negatives() {
    unsafe {
        let mut arr = [0i32, -1, 5, -3, 2];
        quickSort(arr.as_mut_ptr(), 0, (arr.len() - 1) as i32);
        assert_eq!(arr, [-3, -1, 0, 2, 5]);
    }
}

#[test]
fn test_duplicates() {
    unsafe {
        let mut arr = [7i32, 7, 7, 7];
        quickSort(arr.as_mut_ptr(), 0, (arr.len() - 1) as i32);
        assert!(is_sorted(&arr));
        assert_eq!(arr, [7, 7, 7, 7]);
    }
}

#[test]
fn test_single_element() {
    unsafe {
        let mut arr = [42i32];
        quickSort(arr.as_mut_ptr(), 0, 0);
        assert_eq!(arr, [42]);
    }
}

#[test]
fn test_already_sorted() {
    unsafe {
        let mut arr = [1i32, 2, 3, 4, 5];
        quickSort(arr.as_mut_ptr(), 0, (arr.len() - 1) as i32);
        assert!(is_sorted(&arr));
    }
}

