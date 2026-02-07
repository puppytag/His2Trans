//! Module: src_tree
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

pub extern "C" fn ZopfliLengthsToSymbols(lengths: *const ::core::ffi::c_uint, n: crate::types::size_t, maxbits: ::core::ffi::c_uint, symbols: *mut ::core::ffi::c_uint) {
    unsafe {
        let maxbits_usize = maxbits as usize;
        let bl_count = libc::malloc((maxbits_usize + 1) * std::mem::size_of::<usize>()) as *mut usize;
        let next_code = libc::malloc((maxbits_usize + 1) * std::mem::size_of::<usize>()) as *mut usize;
        if bl_count.is_null() || next_code.is_null() {
            libc::free(bl_count as *mut std::ffi::c_void);
            libc::free(next_code as *mut std::ffi::c_void);
            return;
        }
        let n_usize = n as usize;
        for i in 0..n_usize {
            *symbols.add(i) = 0;
        }
        for bits in 0..=maxbits_usize {
            *bl_count.add(bits) = 0;
        }
        for i in 0..n_usize {
            let len = *lengths.add(i);
            assert!(len <= maxbits);
            *bl_count.add(len as usize) += 1;
        }
        let mut code: u32 = 0;
        *bl_count.add(0) = 0;
        for bits in 1..=maxbits_usize {
            code = (code + *bl_count.add(bits - 1) as u32) << 1;
            *next_code.add(bits) = code as usize;
        }
        for i in 0..n_usize {
            let len = *lengths.add(i);
            if len != 0 {
                *symbols.add(i) = *next_code.add(len as usize) as u32;
                *next_code.add(len as usize) += 1;
            }
        }
        libc::free(bl_count as *mut std::ffi::c_void);
        libc::free(next_code as *mut std::ffi::c_void);
    }
}

pub extern "C" fn ZopfliCalculateEntropy(count: *const crate::types::size_t, n: crate::types::size_t, bitlengths: *mut f64) {
    static K_INV_LOG2: f64 = 1.4426950408889;
    let mut sum: u32 = 0;
    let n_usize = n as usize;
    unsafe {
        for i in 0..n_usize {
            sum += *count.add(i) as u32;
        }
        let log2sum = if sum == 0 {
            (n as f64).ln() * K_INV_LOG2
        } else {
            (sum as f64).ln() * K_INV_LOG2
        };
        for i in 0..n_usize {
            let ci = *count.add(i) as u32;
            let mut val = if ci == 0 {
                log2sum
            } else {
                log2sum - (ci as f64).ln() * K_INV_LOG2
            };
            if val < 0.0 && val > -1e-5 {
                val = 0.0;
            }
            if !(val >= 0.0) {
                libc::abort();
            }
            *bitlengths.add(i) = val;
        }
    }
}

pub extern "C" fn ZopfliCalculateBitLengths(count: *const crate::types::size_t, n: crate::types::size_t, maxbits: ::core::ffi::c_int, bitlengths: *mut ::core::ffi::c_uint) {
    let error = crate::src_katajainen::ZopfliLengthLimitedCodeLengths(count, n as ::core::ffi::c_int, maxbits, bitlengths);
    let _ = error;
    if error != 0 {
        unsafe {
            libc::abort();
        }
    }
}
