// c2rust 测试文件 for qsort
// 适配 c2rust 原始指针版本的函数签名
//
// c2rust 函数签名:
// - swap(a: *mut c_int, b: *mut c_int)
// - partition(arr: *mut c_int, low: c_int, high: c_int) -> c_int
// - quickSort(arr: *mut c_int, low: c_int, high: c_int)

use std::os::raw::c_int;
use crate::src_qsort::{swap, partition, quickSort};

fn is_sorted(arr: &[c_int]) -> bool {
    for i in 1..arr.len() {
        if arr[i - 1] > arr[i] {
            return false;
        }
    }
    true
}

#[test]
fn test_small() {
    let mut arr: [c_int; 3] = [3, 1, 2];
    unsafe {
        quickSort(arr.as_mut_ptr(), 0, 2);
    }
    assert_eq!(arr, [1, 2, 3]);
}

#[test]
fn test_reverse() {
    let mut arr: [c_int; 5] = [5, 4, 3, 2, 1];
    unsafe {
        quickSort(arr.as_mut_ptr(), 0, 4);
    }
    assert_eq!(arr, [1, 2, 3, 4, 5]);
}

#[test]
fn test_negatives() {
    let mut arr: [c_int; 5] = [0, -1, 5, -3, 2];
    unsafe {
        quickSort(arr.as_mut_ptr(), 0, 4);
    }
    assert_eq!(arr, [-3, -1, 0, 2, 5]);
}

#[test]
fn test_duplicates() {
    let mut arr: [c_int; 4] = [7, 7, 7, 7];
    unsafe {
        quickSort(arr.as_mut_ptr(), 0, 3);
    }
    assert_eq!(arr, [7, 7, 7, 7]);
}

#[test]
fn test_swap() {
    let mut a: c_int = 1;
    let mut b: c_int = 2;
    unsafe {
        swap(&mut a, &mut b);
    }
    assert_eq!(a, 2);
    assert_eq!(b, 1);
}

#[test]
fn test_partition() {
    let mut arr: [c_int; 8] = [3, 1, 4, 1, 5, 9, 2, 6];
    let pivot_idx = unsafe { partition(arr.as_mut_ptr(), 0, 7) };
    // pivot 后所有左边元素应该 <= pivot，右边 >= pivot
    let pivot = arr[pivot_idx as usize];
    for i in 0..pivot_idx as usize {
        assert!(arr[i] <= pivot, "左边元素应该 <= pivot");
    }
    for i in (pivot_idx as usize + 1)..8 {
        assert!(arr[i] >= pivot, "右边元素应该 >= pivot");
    }
}

#[test]
fn test_single_element() {
    let mut arr: [c_int; 1] = [42];
    unsafe {
        quickSort(arr.as_mut_ptr(), 0, 0);
    }
    assert_eq!(arr[0], 42);
}

#[test]
fn test_two_elements() {
    let mut arr: [c_int; 2] = [2, 1];
    unsafe {
        quickSort(arr.as_mut_ptr(), 0, 1);
    }
    assert_eq!(arr, [1, 2]);
}

#[test]
fn test_already_sorted() {
    let mut arr: [c_int; 5] = [1, 2, 3, 4, 5];
    unsafe {
        quickSort(arr.as_mut_ptr(), 0, 4);
    }
    assert!(is_sorted(&arr));
}
