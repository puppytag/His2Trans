//! Module: src_ht
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

fn hash_key(key: *const std::ffi::c_char) -> u64 {
    const FNV_OFFSET: u64 = 14695981039346656037u64;
    const FNV_PRIME: u64 = 1099511628211u64;
    
    let mut hash: u64 = FNV_OFFSET;
    let mut p = key;
    
    unsafe {
        while *p != 0 {
            hash ^= (*p as u8) as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
            p = p.add(1);
        }
    }
    
    hash
}

pub extern "C" fn main() {}
