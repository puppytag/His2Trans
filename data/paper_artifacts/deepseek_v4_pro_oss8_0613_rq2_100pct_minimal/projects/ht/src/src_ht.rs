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
use core::ffi::CStr;

fn hash_key(key: &core::ffi::CStr) -> u64 {
    let mut hash: u64 = 14695981039346656037u64;
    for &b in key.to_bytes() {
        hash ^= b as u64;
        hash = hash.wrapping_mul(1099511628211u64);
    }
    hash
}

fn main() {}
