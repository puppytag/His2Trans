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
        let bl_count = libc::malloc((std::mem::size_of::<usize>() * (maxbits as usize + 1)) as usize) as *mut usize;
        let next_code = libc::malloc((std::mem::size_of::<usize>() * (maxbits as usize + 1)) as usize) as *mut usize;
        
        let mut i: u32;
        let mut bits: u32;
        let mut code: u32;
        
        i = 0;
        while (i as usize) < n {
            *symbols.offset(i as isize) = 0;
            i += 1;
        }
        
        bits = 0;
        while bits <= maxbits {
            *bl_count.offset(bits as isize) = 0;
            bits += 1;
        }
        
        i = 0;
        while (i as usize) < n {
            let len_i = *lengths.offset(i as isize);
            *bl_count.offset(len_i as isize) += 1;
            i += 1;
        }
        
        code = 0;
        *bl_count.offset(0) = 0;
        bits = 1;
        while bits <= maxbits {
            code = (code + (*bl_count.offset((bits - 1) as isize) as u32)) << 1;
            *next_code.offset(bits as isize) = code as usize;
            bits += 1;
        }
        
        i = 0;
        while (i as usize) < n {
            let len = *lengths.offset(i as isize);
            if len != 0 {
                *symbols.offset(i as isize) = *next_code.offset(len as isize) as u32;
                *next_code.offset(len as isize) += 1;
            }
            i += 1;
        }
        
        libc::free(bl_count as *mut ::core::ffi::c_void);
        libc::free(next_code as *mut ::core::ffi::c_void);
    }
}

pub extern "C" fn ZopfliCalculateEntropy(count: *const crate::types::size_t, n: crate::types::size_t, bitlengths: *mut f64) {
    const K_INV_LOG2: f64 = 1.4426950408889;
    let mut sum: u32 = 0;
    let mut i: usize;
    let log2sum: f64;
    
    i = 0;
    while i < n {
        unsafe {
            sum += *count.add(i) as u32;
        }
        i += 1;
    }
    
    log2sum = if sum == 0 {
        (n as f64).ln()
    } else {
        (sum as f64).ln()
    } * K_INV_LOG2;
    
    i = 0;
    while i < n {
        unsafe {
            if *count.add(i) == 0 {
                *bitlengths.add(i) = log2sum;
            } else {
                *bitlengths.add(i) = log2sum - (*count.add(i) as f64).ln() * K_INV_LOG2;
            }
            
            if *bitlengths.add(i) < 0.0 && *bitlengths.add(i) > -1e-5 {
                *bitlengths.add(i) = 0.0;
            }
            
            debug_assert!(*bitlengths.add(i) >= 0.0);
        }
        i += 1;
    }
}

pub extern "C" fn ZopfliCalculateBitLengths(count: *const crate::types::size_t, n: crate::types::size_t, maxbits: ::core::ffi::c_int, bitlengths: *mut ::core::ffi::c_uint) {
    let error = crate::src_katajainen::ZopfliLengthLimitedCodeLengths(count, n as ::core::ffi::c_int, maxbits, bitlengths);
    let _ = error;
    debug_assert!(error == 0, "!error");
}
