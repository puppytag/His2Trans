// C2R 框架测试文件 for qsort
// 直接调用翻译后的 Rust 函数
//
// 使用方法：
// 1. 复制此文件到项目的 src/test_c2r.rs
// 2. 在 main.rs 中添加: #[cfg(test)] mod test_c2r;
// 3. 运行 cargo test

use crate::src_qsort::quickSort;

fn is_sorted(arr: &[i32]) -> bool {
    for i in 1..arr.len() {
        if arr[i - 1] > arr[i] {
            return false;
        }
    }
    true
}

#[test]
fn test_small() {
    let mut arr: [i32; 3] = [3, 1, 2];
    unsafe {
        quickSort(arr.as_mut_ptr(), 0, 2);
    }
    assert_eq!(arr[0], 1);
    assert_eq!(arr[1], 2);
    assert_eq!(arr[2], 3);
}

#[test]
fn test_reverse() {
    let mut arr: [i32; 5] = [5, 4, 3, 2, 1];
    unsafe {
        quickSort(arr.as_mut_ptr(), 0, 4);
    }
    assert_eq!(arr[0], 1);
    assert_eq!(arr[1], 2);
    assert_eq!(arr[2], 3);
    assert_eq!(arr[3], 4);
    assert_eq!(arr[4], 5);
}

#[test]
fn test_negatives() {
    let mut arr: [i32; 5] = [0, -1, 5, -3, 2];
    unsafe {
        quickSort(arr.as_mut_ptr(), 0, 4);
    }
    assert_eq!(arr[0], -3);
    assert_eq!(arr[1], -1);
    assert_eq!(arr[2], 0);
    assert_eq!(arr[3], 2);
    assert_eq!(arr[4], 5);
}

#[test]
fn test_duplicates() {
    let mut arr: [i32; 4] = [7, 7, 7, 7];
    unsafe {
        quickSort(arr.as_mut_ptr(), 0, 3);
    }
    assert_eq!(arr[0], 7);
    assert_eq!(arr[1], 7);
    assert_eq!(arr[2], 7);
    assert_eq!(arr[3], 7);
}

#[test]
fn test_single_element() {
    let mut arr: [i32; 1] = [42];
    unsafe {
        quickSort(arr.as_mut_ptr(), 0, 0);
    }
    assert_eq!(arr[0], 42);
}

#[test]
fn test_already_sorted() {
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    unsafe {
        quickSort(arr.as_mut_ptr(), 0, 4);
    }
    assert!(is_sorted(&arr));
}
