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
    unsafe {
        let newchain: *mut crate::types::Node;
        let oldchain: *mut crate::types::Node;
        
        let lists_index = lists.offset(index as isize);
        let lastcount = (*(*lists_index)[1]).count;
        
        if index == 0 && lastcount >= numsymbols {
            return;
        }
        
        newchain = (*pool).next;
        (*pool).next = (*pool).next.offset(1);
        oldchain = (*lists_index)[1];
        
        (*lists_index)[0] = oldchain;
        (*lists_index)[1] = newchain;
        
        if index == 0 {
            let leaf = leaves.offset(lastcount as isize);
            crate::src_katajainen::InitNode((*leaf).weight as usize, lastcount + 1, std::ptr::null_mut(), newchain);
        } else {
            let lists_prev = lists.offset((index - 1) as isize);
            let sum = (*(*lists_prev)[0]).weight + (*(*lists_prev)[1]).weight;
            let leaf = leaves.offset(lastcount as isize);
            
            if lastcount < numsymbols && sum > (*leaf).weight {
                crate::src_katajainen::InitNode((*leaf).weight as usize, lastcount + 1, (*oldchain).tail, newchain);
            } else {
                crate::src_katajainen::InitNode(sum as usize, lastcount, (*lists_prev)[1], newchain);
                crate::src_katajainen::BoundaryPM(lists, leaves, numsymbols, pool, index - 1);
                crate::src_katajainen::BoundaryPM(lists, leaves, numsymbols, pool, index - 1);
            }
        }
    }
}

fn BoundaryPMFinal(lists: *mut [*mut crate::types::Node; 2], leaves: *mut crate::types::Node, numsymbols: i32, pool: *mut crate::types::NodePool, index: i32) {
    unsafe {
        let lastcount = (*(*lists.offset(index as isize))[1]).count;
        
        let sum = (*(*lists.offset((index - 1) as isize))[0]).weight + (*(*lists.offset((index - 1) as isize))[1]).weight;
        
        if lastcount < numsymbols && sum > (*leaves.offset(lastcount as isize)).weight {
            let newchain = (*pool).next;
            let oldchain = (*(*lists.offset(index as isize))[1]).tail;
            
            (*lists.offset(index as isize))[1] = newchain;
            (*newchain).count = lastcount + 1;
            (*newchain).tail = oldchain;
        } else {
            (*(*lists.offset(index as isize))[1]).tail = (*lists.offset((index - 1) as isize))[1];
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

fn ExtractBitLengths(chain: *mut crate::types::Node, leaves: *mut crate::types::Node, bitlengths: *mut std::os::raw::c_uint) {
    let mut counts: [i32; 16] = [0; 16];
    let mut end: u32 = 16;
    let mut ptr: u32 = 15;
    let mut value: u32 = 1;
    let mut node: *mut crate::types::Node;
    let mut val: i32;

    node = chain;
    while !node.is_null() {
        end -= 1;
        unsafe {
            counts[end as usize] = (*node).count;
            node = (*node).tail;
        }
    }

    val = counts[15];
    while ptr >= end {
        unsafe {
            while val > counts[(ptr - 1) as usize] {
                let leaf_idx = (val - 1) as isize;
                let leaf_count = (*leaves.offset(leaf_idx)).count;
                *bitlengths.offset(leaf_count as isize) = value;
                val -= 1;
            }
        }
        ptr -= 1;
        value += 1;
    }
}

fn LeafComparator(a: *const std::ffi::c_void, b: *const std::ffi::c_void) -> i32 {
    unsafe {
        let node_a = a as *const crate::types::Node;
        let node_b = b as *const crate::types::Node;
        let weight_a = (*node_a).weight as i32;
        let weight_b = (*node_b).weight as i32;
        weight_a - weight_b
    }
}

pub extern "C" fn ZopfliLengthLimitedCodeLengths(frequencies: *const crate::types::size_t, n: ::core::ffi::c_int, maxbits: ::core::ffi::c_int, bitlengths: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int {
    unsafe {
        let mut pool: crate::types::NodePool = std::mem::zeroed();
        let mut i: ::core::ffi::c_int;
        let mut numsymbols: ::core::ffi::c_int = 0;
        let mut numBoundaryPMRuns: ::core::ffi::c_int;
        let nodes: *mut crate::types::Node;
        let mut maxbits = maxbits;

        let lists: *mut [*mut crate::types::Node; 2];

        let leaves: *mut crate::types::Node = libc::malloc((n as usize) * std::mem::size_of::<crate::types::Node>()) as *mut crate::types::Node;

        i = 0;
        while i < n {
            *bitlengths.offset(i as isize) = 0;
            i += 1;
        }

        i = 0;
        while i < n {
            if *frequencies.offset(i as isize) != 0 {
                (*leaves.offset(numsymbols as isize)).weight = *frequencies.offset(i as isize) as u64;
                (*leaves.offset(numsymbols as isize)).count = i;
                numsymbols += 1;
            }
            i += 1;
        }

        if (1 << maxbits) < numsymbols {
            libc::free(leaves as *mut ::core::ffi::c_void);
            return 1;
        }
        if numsymbols == 0 {
            libc::free(leaves as *mut ::core::ffi::c_void);
            return 0;
        }
        if numsymbols == 1 {
            *bitlengths.offset((*leaves.offset(0)).count as isize) = 1;
            libc::free(leaves as *mut ::core::ffi::c_void);
            return 0;
        }
        if numsymbols == 2 {
            *bitlengths.offset((*leaves.offset(0)).count as isize) += 1;
            *bitlengths.offset((*leaves.offset(1)).count as isize) += 1;
            libc::free(leaves as *mut ::core::ffi::c_void);
            return 0;
        }

        i = 0;
        while i < numsymbols {
            let weight_bits = std::mem::size_of::<u64>() * 8 - 9;
            let threshold: u64 = 1u64 << weight_bits;
            if (*leaves.offset(i as isize)).weight >= threshold {
                libc::free(leaves as *mut ::core::ffi::c_void);
                return 1;
            }
            let w: u64 = (*leaves.offset(i as isize)).weight;
            let c: u64 = (*leaves.offset(i as isize)).count as u64;
            (*leaves.offset(i as isize)).weight = (w << 9) | c;
            i += 1;
        }
        
        extern "C" fn leaf_comparator_wrapper(a: *const ::core::ffi::c_void, b: *const ::core::ffi::c_void) -> ::core::ffi::c_int {
            crate::src_katajainen::LeafComparator(a, b)
        }
        
        libc::qsort(
            leaves as *mut ::core::ffi::c_void,
            numsymbols as usize,
            std::mem::size_of::<crate::types::Node>(),
            Some(leaf_comparator_wrapper),
        );
        
        i = 0;
        while i < numsymbols {
            (*leaves.offset(i as isize)).weight >>= 9;
            i += 1;
        }

        if numsymbols - 1 < maxbits {
            maxbits = numsymbols - 1;
        }

        nodes = libc::malloc((maxbits as usize) * 2 * (numsymbols as usize) * std::mem::size_of::<crate::types::Node>()) as *mut crate::types::Node;
        pool.next = nodes;

        lists = libc::malloc((maxbits as usize) * std::mem::size_of::<[*mut crate::types::Node; 2]>()) as *mut [*mut crate::types::Node; 2];
        crate::src_katajainen::InitLists(&mut pool, leaves as *const crate::types::Node, maxbits, lists);

        numBoundaryPMRuns = 2 * numsymbols - 4;
        i = 0;
        while i < numBoundaryPMRuns - 1 {
            crate::src_katajainen::BoundaryPM(lists, leaves, numsymbols, &mut pool, maxbits - 1);
            i += 1;
        }
        crate::src_katajainen::BoundaryPMFinal(lists, leaves, numsymbols, &mut pool, maxbits - 1);

        crate::src_katajainen::ExtractBitLengths((*lists.offset((maxbits - 1) as isize))[1], leaves, bitlengths);

        libc::free(lists as *mut ::core::ffi::c_void);
        libc::free(leaves as *mut ::core::ffi::c_void);
        libc::free(nodes as *mut ::core::ffi::c_void);
        0
    }
}
