//! Module: src_qsort
//!
//! Auto-generated skeleton - function bodies are unimplemented.

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::types::*;
use crate::globals::*;
use crate::compat::*;

pub extern "C" fn swap(a: *mut ::core::ffi::c_int, b: *mut ::core::ffi::c_int) {
    unsafe {
        let t = *a;
        *a = *b;
        *b = t;
    }
}

pub extern "C" fn partition(arr: *mut ::core::ffi::c_int, low: ::core::ffi::c_int, high: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        let pivot = *arr.offset(high as isize);
        let mut i = low - 1;

        let mut j = low;
        while j <= high - 1 {
            if *arr.offset(j as isize) <= pivot {
                i += 1;
                crate::src_qsort::swap(arr.offset(i as isize), arr.offset(j as isize));
            }
            j += 1;
        }
        crate::src_qsort::swap(arr.offset((i + 1) as isize), arr.offset(high as isize));
        i + 1
    }
}

pub extern "C" fn quickSort(arr: *mut ::core::ffi::c_int, low: ::core::ffi::c_int, high: ::core::ffi::c_int) {
    if low < high {
        let i = crate::src_qsort::partition(arr, low, high);
        crate::src_qsort::quickSort(arr, low, i - 1);
        crate::src_qsort::quickSort(arr, i + 1, high);
    }
}
