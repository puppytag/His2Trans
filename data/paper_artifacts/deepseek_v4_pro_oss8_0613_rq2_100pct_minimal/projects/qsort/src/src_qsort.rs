//! Module: src_qsort
//!
//! Safe Rust implementation of quicksort (Lomuto partition).

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::types::*;
use crate::globals::*;
use crate::compat::*;

pub fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i: isize = low as isize - 1;
    for j in low..high {
        if arr[j] <= pivot {
            i += 1;
            arr.swap(i as usize, j);
        }
    }
    let i1 = i + 1;
    arr.swap(i1 as usize, high);
    i1 as usize
}

pub fn quickSort(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let i = partition(arr, low, high);
        if i > low {
            quickSort(arr, low, i - 1);
        }
        if i + 1 <= high {
            quickSort(arr, i + 1, high);
        }
    }
}
