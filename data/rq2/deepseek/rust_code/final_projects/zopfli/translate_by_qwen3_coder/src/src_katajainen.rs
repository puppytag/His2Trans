//! Module: src_katajainen
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

fn InitNode(weight: usize, count: i32, tail: *mut crate::types::Node, node: *mut crate::types::Node) {
    unsafe {
        (*node).weight = weight as u64;
        (*node).count = count;
        (*node).tail = tail;
    }
}

fn BoundaryPM(lists: *mut [*mut crate::types::Node; 2], leaves: *mut crate::types::Node, numsymbols: i32, pool: *mut crate::types::NodePool, index: i32) {
    if lists.is_null() || leaves.is_null() || pool.is_null() {
        return;
    }
    let lastcount = unsafe { (*(*lists.offset(index as isize))[1]).count };
    if index == 0 && lastcount >= numsymbols {
        return;
    }
    let newchain = unsafe { (*pool).next };
    let oldchain = unsafe { (*lists.offset(index as isize))[1] };
    unsafe {
        (*pool).next = (*pool).next.offset(1);
    }
    unsafe {
        (*lists.offset(index as isize))[0] = oldchain;
        (*lists.offset(index as isize))[1] = newchain;
    }
    if index == 0 {
        crate::src_katajainen::InitNode(
            unsafe { (*leaves.offset(lastcount as isize)).weight } as usize,
            lastcount + 1,
            std::ptr::null_mut(),
            newchain,
        );
    } else {
        let sum = unsafe {
            (*(*lists.offset((index - 1) as isize))[0]).weight as usize
                + (*(*lists.offset((index - 1) as isize))[1]).weight as usize
        };
        if lastcount < numsymbols
            && sum > unsafe { (*leaves.offset(lastcount as isize)).weight as usize }
        {
            crate::src_katajainen::InitNode(
                unsafe { (*leaves.offset(lastcount as isize)).weight } as usize,
                lastcount + 1,
                unsafe { (*oldchain).tail },
                newchain,
            );
        } else {
            crate::src_katajainen::InitNode(
                sum,
                lastcount,
                unsafe { (*lists.offset((index - 1) as isize))[1] },
                newchain,
            );
            crate::src_katajainen::BoundaryPM(lists, leaves, numsymbols, pool, index - 1);
            crate::src_katajainen::BoundaryPM(lists, leaves, numsymbols, pool, index - 1);
        }
    }
}

fn BoundaryPMFinal(lists: *mut [*mut crate::types::Node; 2], leaves: *mut crate::types::Node, numsymbols: i32, pool: *mut crate::types::NodePool, index: i32) {
    unsafe {
        let lastcount = (*lists.offset(index as isize))[1].as_ref().unwrap().count;
        let sum = (*lists.offset((index - 1) as isize))[0].as_ref().unwrap().weight as usize + (*lists.offset((index - 1) as isize))[1].as_ref().unwrap().weight as usize;
        if lastcount < numsymbols && sum > (*leaves.offset(lastcount as isize)).weight as usize {
            let newchain = (*pool).next;
            let oldchain = (*lists.offset(index as isize))[1].as_ref().unwrap().tail;
            (*lists.offset(index as isize))[1] = newchain;
            (*newchain).count = lastcount + 1;
            (*newchain).tail = oldchain;
        } else {
            (*lists.offset(index as isize))[1].as_mut().unwrap().tail = (*lists.offset((index - 1) as isize))[1];
        }
    }
}

fn InitLists(pool: *mut crate::types::NodePool, leaves: *const crate::types::Node, maxbits: i32, lists: *mut [*mut crate::types::Node; 2]) {
    unsafe {
        let node0 = (*pool).next;
        (*pool).next = (*pool).next.offset(1);
        let node1 = (*pool).next;
        (*pool).next = (*pool).next.offset(1);
        crate::src_katajainen::InitNode((*leaves.offset(0)).weight as usize, 1, std::ptr::null_mut(), node0);
        crate::src_katajainen::InitNode((*leaves.offset(1)).weight as usize, 2, std::ptr::null_mut(), node1);
        for i in 0..maxbits {
            (*lists.offset(i as isize))[0] = node0;
            (*lists.offset(i as isize))[1] = node1;
        }
    }
}

fn ExtractBitLengths(chain: *mut crate::types::Node, leaves: *mut crate::types::Node, bitlengths: *mut u32) {
    let mut counts: [i32; 16] = [0; 16];
    let mut end: u32 = 16;
    let mut ptr: u32 = 15;
    let mut value: u32 = 1;
    let mut node = chain;
    let mut val: i32;

    while !node.is_null() {
        unsafe {
            end -= 1;
            counts[end as usize] = (*node).count;
            node = (*node).tail;
        }
    }

    val = counts[15];
    while ptr >= end {
        while val > counts[(ptr - 1) as usize] {
            unsafe {
                bitlengths.offset((*leaves.offset((val - 1) as isize)).count as isize).write(value);
            }
            val -= 1;
        }
        ptr -= 1;
        value += 1;
    }
}

fn LeafComparator(a: *const std::ffi::c_void, b: *const std::ffi::c_void) -> i32 {
    unsafe {
        let a_node = a as *const crate::types::Node;
        let b_node = b as *const crate::types::Node;
        ((*a_node).weight as i64 - (*b_node).weight as i64) as i32
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_katajainen_7
// c_function: ZopfliLengthLimitedCodeLengths
// rust_file: src_katajainen.rs
// rust_signature: pub extern "C" fn ZopfliLengthLimitedCodeLengths(frequencies: *const crate::types::size_t, n: ::core::ffi::c_int, maxbits: ::core::ffi::c_int, bitlengths: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int
// c_first_line: int ZopfliLengthLimitedCodeLengths(
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_katajainen_7/translated_rust.rs
// last_error_truncated:
//   error[E0308]: mismatched types
//      --> src/src_katajainen.rs:155:64
//       |
//       |                 --------------------------------------------   ^^^^^^^^^^^^^^^^^^^^^^ expected `u64`, found `usize`
//       |                 |
//       |                 expected due to the type of this binding
//   error[E0308]: mismatched types
//      --> src/src_katajainen.rs:180:55
// =================================
pub extern "C" fn ZopfliLengthLimitedCodeLengths(frequencies: *const crate::types::size_t, n: ::core::ffi::c_int, maxbits: ::core::ffi::c_int, bitlengths: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_katajainen::ZopfliLengthLimitedCodeLengths(frequencies as _, n as _, maxbits as _, bitlengths as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_katajainen_7
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_katajainen_7/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn ZopfliLengthLimitedCodeLengths(frequencies: *const crate::types::size_t, n: ::core::ffi::c_int, maxbits: ::core::ffi::c_int, bitlengths: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int {
    unsafe {
        let mut pool = crate::types::NodePool { next: std::ptr::null_mut() };
        let mut numsymbols = 0;
        let leaves = libc::malloc((n as usize) * std::mem::size_of::<crate::types::Node>()) as *mut crate::types::Node;
        if leaves.is_null() {
            return 1;
        }
        for i in 0..(n as isize) {
            *bitlengths.offset(i) = 0;
        }
        for i in 0..(n as isize) {
            if *frequencies.offset(i) != 0 {
                (*leaves.offset(numsymbols as isize)).weight = *frequencies.offset(i);
                (*leaves.offset(numsymbols as isize)).count = i as i32;
                numsymbols += 1;
            }
        }
        if (1 << maxbits) < numsymbols {
            libc::free(leaves as *mut std::ffi::c_void);
            return 1;
        }
        if numsymbols == 0 {
            libc::free(leaves as *mut std::ffi::c_void);
            return 0;
        }
        if numsymbols == 1 {
            *bitlengths.offset((*leaves.offset(0)).count as isize) = 1;
            libc::free(leaves as *mut std::ffi::c_void);
            return 0;
        }
        if numsymbols == 2 {
            *bitlengths.offset((*leaves.offset(0)).count as isize) += 1;
            *bitlengths.offset((*leaves.offset(1)).count as isize) += 1;
            libc::free(leaves as *mut std::ffi::c_void);
            return 0;
        }
        for i in 0..numsymbols {
            if (*leaves.offset(i as isize)).weight >= ((1 as crate::types::size_t) << (std::mem::size_of::<crate::types::size_t>() * 8 - 9)) {
                libc::free(leaves as *mut std::ffi::c_void);
                return 1;
            }
            (*leaves.offset(i as isize)).weight = ((*leaves.offset(i as isize)).weight << 9) | ((*leaves.offset(i as isize)).count as u64);
        }
        libc::qsort(leaves as *mut std::ffi::c_void, numsymbols as usize, std::mem::size_of::<crate::types::Node>() as usize, Some(std::mem::transmute::<_, extern "C" fn(*const std::ffi::c_void, *const std::ffi::c_void) -> i32>(crate::src_katajainen::LeafComparator)));
        for i in 0..numsymbols {
            (*leaves.offset(i as isize)).weight >>= 9;
        }
        let mut maxbits = maxbits;
        if numsymbols - 1 < maxbits {
            maxbits = numsymbols - 1;
        }
        let nodes = libc::malloc((maxbits as usize) * 2 * (numsymbols as usize) * std::mem::size_of::<crate::types::Node>()) as *mut crate::types::Node;
        if nodes.is_null() {
            libc::free(leaves as *mut std::ffi::c_void);
            return 1;
        }
        pool.next = nodes;
        let lists = libc::malloc((maxbits as usize) * std::mem::size_of::<[*mut crate::types::Node; 2]>()) as *mut [*mut crate::types::Node; 2];
        if lists.is_null() {
            libc::free(nodes as *mut std::ffi::c_void);
            libc::free(leaves as *mut std::ffi::c_void);
            return 1;
        }
        crate::src_katajainen::InitLists(&mut pool as *mut crate::types::NodePool, leaves as *const crate::types::Node, maxbits, lists);
        let numBoundaryPMRuns = 2 * numsymbols - 4;
        for i in 0..(numBoundaryPMRuns - 1) {
            crate::src_katajainen::BoundaryPM(lists, leaves, numsymbols as i32, &mut pool as *mut crate::types::NodePool, maxbits - 1);
        }
        crate::src_katajainen::BoundaryPMFinal(lists, leaves, numsymbols as i32, &mut pool as *mut crate::types::NodePool, maxbits - 1);
        crate::src_katajainen::ExtractBitLengths((*lists.offset((maxbits - 1) as isize))[1], leaves, bitlengths);
        libc::free(lists as *mut std::ffi::c_void);
        libc::free(leaves as *mut std::ffi::c_void);
        libc::free(nodes as *mut std::ffi::c_void);
        0
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_katajainen_7
 * === C2R_LLM_FAILED_OUTPUT_END === */

