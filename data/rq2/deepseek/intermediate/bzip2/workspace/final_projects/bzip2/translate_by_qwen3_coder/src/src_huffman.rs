//! Module: src_huffman
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

fn WEIGHTOF(zz0: crate::types::Int32) -> crate::types::Int32 {
    zz0 & 0xffffff00u32 as i32
}

fn DEPTHOF(zz1: crate::types::Int32) -> crate::types::Int32 {
    zz1 & 0x000000ff
}

fn MYMAX(zz2: crate::types::Int32, zz3: crate::types::Int32) -> crate::types::Int32 {
    if zz2 > zz3 { zz2 } else { zz3 }
}

fn ADDWEIGHTS(zw1: crate::types::Int32, zw2: crate::types::Int32) -> crate::types::Int32 {
    (crate::src_huffman::WEIGHTOF(zw1) + crate::src_huffman::WEIGHTOF(zw2)) | (1 + crate::src_huffman::MYMAX(crate::src_huffman::DEPTHOF(zw1), crate::src_huffman::DEPTHOF(zw2)))
}

fn UPHEAP(z: crate::types::Int32, heap: *mut crate::types::Int32, weight: *mut crate::types::Int32) {
    let mut zz = z;
    let tmp = unsafe { *heap.add(zz as usize) };
    while unsafe { *weight.add(tmp as usize) } < unsafe { *weight.add((*heap.add((zz >> 1) as usize)) as usize) } {
        unsafe { *heap.add(zz as usize) = *heap.add((zz >> 1) as usize); }
        zz >>= 1;
    }
    unsafe { *heap.add(zz as usize) = tmp; }
}

fn DOWNHEAP(z: crate::types::Int32, heap: *mut crate::types::Int32, weight: *mut crate::types::Int32, nHeap: crate::types::Int32) {
    let mut zz = z;
    let tmp = unsafe { *heap.offset(zz as isize) };
    loop {
        let mut yy = zz << 1;
        if yy > nHeap {
            break;
        }
        if yy < nHeap && unsafe { *weight.offset(*heap.offset((yy + 1) as isize) as isize) } < unsafe { *weight.offset(*heap.offset(yy as isize) as isize) } {
            yy += 1;
        }
        if unsafe { *weight.offset(tmp as isize) } < unsafe { *weight.offset(*heap.offset(yy as isize) as isize) } {
            break;
        }
        unsafe { *heap.offset(zz as isize) = *heap.offset(yy as isize); }
        zz = yy;
    }
    unsafe { *heap.offset(zz as isize) = tmp; }
}

pub extern "C" fn BZ2_hbMakeCodeLengths(arg1: *mut crate::types::UChar, arg2: *mut crate::types::Int32, arg3: crate::types::Int32, arg4: crate::types::Int32) {
    let len = arg1;
    let freq = arg2;
    let alphaSize = arg3;
    let maxLen = arg4;
    let mut nNodes: crate::types::Int32;
    let mut nHeap: crate::types::Int32;
    let mut n1: crate::types::Int32;
    let mut n2: crate::types::Int32;
    let mut i: crate::types::Int32;
    let mut j: crate::types::Int32;
    let mut k: crate::types::Int32;
    let mut tooLong: crate::types::Bool;
    let mut heap: [crate::types::Int32; 258 + 2] = [0; 258 + 2];
    let mut weight: [crate::types::Int32; 258 * 2] = [0; 258 * 2];
    let mut parent: [crate::types::Int32; 258 * 2] = [0; 258 * 2];

    for i in 0..alphaSize {
        unsafe {
            let f = *freq.offset(i as isize);
            weight[(i + 1) as usize] = (if f == 0 { 1 } else { f }) << 8;
        }
    }

    loop {
        nNodes = alphaSize;
        nHeap = 0;

        heap[0] = 0;
        weight[0] = 0;
        parent[0] = -2;

        for i in 1..=alphaSize {
            parent[i as usize] = -1;
            nHeap += 1;
            heap[nHeap as usize] = i;
            crate::src_huffman::UPHEAP(nHeap, heap.as_mut_ptr(), weight.as_mut_ptr());
        }

        if !(nHeap < (258 + 2)) {
            crate::src_bzlib::BZ2_bz__AssertH__fail(2001);
        }

        while nHeap > 1 {
            n1 = heap[1];
            heap[1] = heap[nHeap as usize];
            nHeap -= 1;
            crate::src_huffman::DOWNHEAP(1, heap.as_mut_ptr(), weight.as_mut_ptr(), nHeap);
            n2 = heap[1];
            heap[1] = heap[nHeap as usize];
            nHeap -= 1;
            crate::src_huffman::DOWNHEAP(1, heap.as_mut_ptr(), weight.as_mut_ptr(), nHeap);
            nNodes += 1;
            parent[n1 as usize] = nNodes;
            parent[n2 as usize] = nNodes;
            weight[nNodes as usize] = crate::src_huffman::ADDWEIGHTS(weight[n1 as usize], weight[n2 as usize]);
            parent[nNodes as usize] = -1;
            nHeap += 1;
            heap[nHeap as usize] = nNodes;
            crate::src_huffman::UPHEAP(nHeap, heap.as_mut_ptr(), weight.as_mut_ptr());
        }

        if !(nNodes < (258 * 2)) {
            crate::src_bzlib::BZ2_bz__AssertH__fail(2002);
        }

        tooLong = 0;
        for i in 1..=alphaSize {
            j = 0;
            k = i;
            while parent[k as usize] >= 0 {
                k = parent[k as usize];
                j += 1;
            }
            unsafe {
                *len.offset((i - 1) as isize) = j as crate::types::UChar;
            }
            if j > maxLen {
                tooLong = 1;
            }
        }

        if tooLong == 0 {
            break;
        }

        for i in 1..=alphaSize {
            j = weight[i as usize] >> 8;
            j = 1 + (j / 2);
            weight[i as usize] = j << 8;
        }
    }
}

pub extern "C" fn BZ2_hbAssignCodes(arg1: *mut crate::types::Int32, arg2: *mut crate::types::UChar, arg3: crate::types::Int32, arg4: crate::types::Int32, arg5: crate::types::Int32) {
    let code = arg1;
    let length = arg2;
    let min_len = arg3;
    let max_len = arg4;
    let alpha_size = arg5;
    let mut vec: crate::types::Int32 = 0;
    let mut n = min_len;
    while n <= max_len {
        let mut i: crate::types::Int32 = 0;
        while i < alpha_size {
            unsafe {
                if (*length.offset(i as isize)) as crate::types::Int32 == n {
                    *code.offset(i as isize) = vec;
                    vec += 1;
                }
            }
            i += 1;
        }
        vec <<= 1;
        n += 1;
    }
}

pub extern "C" fn BZ2_hbCreateDecodeTables(arg1: *mut crate::types::Int32, arg2: *mut crate::types::Int32, arg3: *mut crate::types::Int32, arg4: *mut crate::types::UChar, arg5: crate::types::Int32, arg6: crate::types::Int32, arg7: crate::types::Int32) {
    let limit = arg1;
    let base = arg2;
    let perm = arg3;
    let length = arg4;
    let min_len = arg5;
    let max_len = arg6;
    let alpha_size = arg7;
    let mut pp = 0;
    let mut i = min_len;
    while i <= max_len {
        let mut j = 0;
        while j < alpha_size {
            unsafe {
                if *length.offset(j as isize) == i as u8 {
                    *perm.offset(pp as isize) = j;
                    pp += 1;
                }
            }
            j += 1;
        }
        i += 1;
    }
    let mut i = 0;
    while i < 23 {
        unsafe {
            *base.offset(i as isize) = 0;
        }
        i += 1;
    }
    let mut i = 0;
    while i < alpha_size {
        unsafe {
            let idx = *length.offset(i as isize) as i32 + 1;
            *base.offset(idx as isize) += 1;
        }
        i += 1;
    }
    let mut i = 1;
    while i < 23 {
        unsafe {
            *base.offset(i as isize) += *base.offset((i - 1) as isize);
        }
        i += 1;
    }
    let mut i = 0;
    while i < 23 {
        unsafe {
            *limit.offset(i as isize) = 0;
        }
        i += 1;
    }
    let mut vec = 0;
    let mut i = min_len;
    while i <= max_len {
        unsafe {
            vec += *base.offset((i + 1) as isize) - *base.offset(i as isize);
            *limit.offset(i as isize) = vec - 1;
            vec <<= 1;
        }
        i += 1;
    }
    let mut i = min_len + 1;
    while i <= max_len {
        unsafe {
            *base.offset(i as isize) = ((*limit.offset((i - 1) as isize) + 1) << 1) - *base.offset(i as isize);
        }
        i += 1;
    }
}
