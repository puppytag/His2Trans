//! Module: src_blocksplitter
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

fn FindMinimum(f: crate::types::FindMinimumFun, context: *mut std::ffi::c_void, start: usize, end: usize, smallest: *mut f64) -> usize {
    if end.wrapping_sub(start) < 1024 {
        let mut best = 1e30;
        let mut result = start;
        let mut i = start;
        while i < end {
            let v = if let Some(func) = f { unsafe { func(i as u64, context) } } else { 0.0 };
            if v < best {
                best = v;
                result = i;
            }
            i = i.wrapping_add(1);
        }
        unsafe { *smallest = best };
        return result;
    } else {
        let mut p: [usize; 9] = [0; 9];
        let mut vp: [f64; 9] = [0.0; 9];
        let mut lastbest = 1e30;
        let mut pos = start;
        let mut start = start;
        let mut end = end;
        loop {
            if end.wrapping_sub(start) <= 9 {
                break;
            }
            let mut i = 0;
            while i < 9 {
                p[i] = start.wrapping_add((i.wrapping_add(1)).wrapping_mul((end.wrapping_sub(start)) / (9 + 1)));
                vp[i] = if let Some(func) = f { unsafe { func(p[i] as u64, context) } } else { 0.0 };
                i = i.wrapping_add(1);
            }
            let mut besti = 0;
            let mut best = vp[0];
            let mut i = 1;
            while i < 9 {
                if vp[i] < best {
                    best = vp[i];
                    besti = i;
                }
                i = i.wrapping_add(1);
            }
            if best > lastbest {
                break;
            }
            start = if besti == 0 { start } else { p[besti.wrapping_sub(1)] };
            end = if besti == 9 - 1 { end } else { p[besti.wrapping_add(1)] };
            pos = p[besti];
            lastbest = best;
        }
        unsafe { *smallest = lastbest };
        return pos;
    }
}

fn EstimateCost(lz77: *const crate::types::ZopfliLZ77Store, lstart: crate::types::size_t, lend: crate::types::size_t) -> f64 {
    crate::src_deflate::ZopfliCalculateBlockSizeAutoType(lz77, lstart, lend)
}

fn SplitCost(i: usize, context: *mut std::ffi::c_void) -> f64 {
    unsafe {
        let c = context as *mut crate::types::SplitCostContext;
        let start = (*c).start as crate::types::size_t;
        let end = (*c).end as crate::types::size_t;
        let lz77 = (*c).lz77;
        crate::src_blocksplitter::EstimateCost(lz77, start, i as crate::types::size_t) +
        crate::src_blocksplitter::EstimateCost(lz77, i as crate::types::size_t, end)
    }
}

fn AddSorted(value: usize, out: *mut *mut usize, outsize: *mut usize) {
    unsafe {
        let outsize_val = *outsize;
        if outsize_val & (outsize_val.wrapping_sub(1)) == 0 {
            let new_size = if outsize_val == 0 { 1 } else { outsize_val * 2 };
            let new_ptr = libc::realloc((*out) as *mut libc::c_void, (new_size * std::mem::size_of::<usize>()) as libc::size_t) as *mut usize;
            if !new_ptr.is_null() {
                *out = new_ptr;
            }
        }
        let out_ptr = *out;
        if !out_ptr.is_null() {
            *out_ptr.add(outsize_val) = value;
            *outsize = outsize_val + 1;
        }
        let mut i = 0;
        while i + 1 < *outsize {
            if (*out).add(i).read() > value {
                let mut j = *outsize - 1;
                while j > i {
                    (*out).add(j).write((*out).add(j - 1).read());
                    j -= 1;
                }
                (*out).add(i).write(value);
                break;
            }
            i += 1;
        }
    }
}

fn PrintBlockSplitPoints(lz77: *const crate::types::ZopfliLZ77Store, lz77splitpoints: *const crate::types::size_t, nlz77points: crate::types::size_t) {
    let mut splitpoints: *mut crate::types::size_t = std::ptr::null_mut();
    let mut npoints: crate::types::size_t = 0;
    let mut pos: crate::types::size_t = 0;
    if nlz77points > 0 {
        let size_ptr = unsafe {
            crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(lz77 as *mut ::core::ffi::c_void)
        };
        let size = unsafe { *(size_ptr as *const crate::types::size_t) };
        let dists_ptr = unsafe {
            crate::compat::c2r_field_ptr_ZopfliLZ77Store__dists(lz77 as *mut ::core::ffi::c_void)
        };
        let litlens_ptr = unsafe {
            crate::compat::c2r_field_ptr_ZopfliLZ77Store__litlens(lz77 as *mut ::core::ffi::c_void)
        };
        let dists = dists_ptr as *const crate::types::size_t;
        let litlens = litlens_ptr as *const crate::types::size_t;
        for i in 0..size {
            let length = if unsafe { *dists.offset(i as isize) } == 0 {
                1
            } else {
                unsafe { *litlens.offset(i as isize) }
            };
            if unsafe { *lz77splitpoints.offset(npoints as isize) } == i {
                if npoints & (npoints - 1) == 0 {
                    let new_size = if npoints == 0 {
                        1
                    } else {
                        npoints * 2
                    };
                    splitpoints = unsafe {
                        libc::realloc(
                            splitpoints as *mut libc::c_void,
                            (new_size * std::mem::size_of::<crate::types::size_t>()) as usize,
                        ) as *mut crate::types::size_t
                    };
                }
                unsafe {
                    *splitpoints.offset(npoints as isize) = pos;
                }
                npoints += 1;
                if npoints == nlz77points {
                    break;
                }
            }
            pos += length;
        }
    }
    assert!(npoints == nlz77points);
    unsafe {
        libc::fprintf(
            crate::compat::stderr as *mut libc::FILE,
            b"block split points: \0" as *const u8 as *const libc::c_char,
        );
    }
    for i in 0..npoints {
        unsafe {
            libc::fprintf(
                crate::compat::stderr as *mut libc::FILE,
                b"%d \0" as *const u8 as *const libc::c_char,
                *splitpoints.offset(i as isize) as libc::c_int,
            );
        }
    }
    unsafe {
        libc::fprintf(
            crate::compat::stderr as *mut libc::FILE,
            b"(hex:\0" as *const u8 as *const libc::c_char,
        );
    }
    for i in 0..npoints {
        unsafe {
            libc::fprintf(
                crate::compat::stderr as *mut libc::FILE,
                b" %x\0" as *const u8 as *const libc::c_char,
                *splitpoints.offset(i as isize) as libc::c_int,
            );
        }
    }
    unsafe {
        libc::fprintf(
            crate::compat::stderr as *mut libc::FILE,
            b")\n\0" as *const u8 as *const libc::c_char,
        );
    }
    unsafe {
        libc::free(splitpoints as *mut libc::c_void);
    }
}

fn FindLargestSplittableBlock(lz77size: usize, done: *const u8, splitpoints: *const usize, npoints: usize, lstart: *mut usize, lend: *mut usize) -> i32 {
    let mut longest = 0usize;
    let mut found = 0i32;
    let mut i = 0usize;
    while i <= npoints {
        let start = if i == 0 { 0 } else { unsafe { *splitpoints.add(i - 1) } };
        let end = if i == npoints { lz77size - 1 } else { unsafe { *splitpoints.add(i) } };
        if unsafe { *done.add(start) } == 0 && end - start > longest {
            unsafe {
                *lstart = start;
                *lend = end;
            }
            found = 1;
            longest = end - start;
        }
        i += 1;
    }
    found
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_blocksplitter_7
// c_function: ZopfliBlockSplitLZ77
// rust_file: src_blocksplitter.rs
// rust_signature: pub extern "C" fn ZopfliBlockSplitLZ77(options: *const crate::types::ZopfliOptions, lz77: *const crate::types::ZopfliLZ77Store, maxblocks: crate::types::size_t, splitpoints: *mut *mut crate::types::size_t, npoints: *mut ...
// c_first_line: void ZopfliBlockSplitLZ77(const ZopfliOptions* options,
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_blocksplitter_7/translated_rust.rs
// last_error_truncated:
//   error[E0609]: no field `size` on type `types::ZopfliLZ77Store`
//      --> src/src_blocksplitter.rs:228:33
//       |
//       |                                 ^^^^ unknown field
//       |
//   error[E0308]: mismatched types
//      --> src/src_blocksplitter.rs:255:22
//       |
// =================================
pub extern "C" fn ZopfliBlockSplitLZ77(options: *const crate::types::ZopfliOptions, lz77: *const crate::types::ZopfliLZ77Store, maxblocks: crate::types::size_t, splitpoints: *mut *mut crate::types::size_t, npoints: *mut crate::types::size_t) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_blocksplitter::ZopfliBlockSplitLZ77(options as _, lz77 as _, maxblocks as _, splitpoints as _, npoints as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_blocksplitter_7
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_blocksplitter_7/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn ZopfliBlockSplitLZ77(options: *const crate::types::ZopfliOptions, lz77: *const crate::types::ZopfliLZ77Store, maxblocks: crate::types::size_t, splitpoints: *mut *mut crate::types::size_t, npoints: *mut crate::types::size_t) {
    unsafe {
        let lz77_size = (*lz77).size as usize;
        if lz77_size < 10 {
            return;
        }
        let done = libc::malloc(lz77_size) as *mut u8;
        if done.is_null() {
            libc::exit(-1);
        }
        std::ptr::write_bytes(done, 0, lz77_size);
        let mut lstart: usize = 0;
        let mut lend: usize = lz77_size;
        let mut numblocks: usize = 1;
        loop {
            if maxblocks > 0 && numblocks >= maxblocks as usize {
                break;
            }
            let mut c = crate::types::SplitCostContext {
                lz77,
                start: lstart,
                end: lend,
                _c2r_private: [0; 0],
            };
            if !(lstart < lend) {
                panic!("assertion failed: lstart < lend");
            }
            let mut splitcost: f64 = 0.0;
            let llpos = crate::src_blocksplitter::FindMinimum(
                Some(crate::src_blocksplitter::SplitCost),
                &mut c as *mut crate::types::SplitCostContext as *mut ::core::ffi::c_void,
                lstart + 1,
                lend,
                &mut splitcost as *mut f64,
            );
            if !(llpos > lstart) {
                panic!("assertion failed: llpos > lstart");
            }
            if !(llpos < lend) {
                panic!("assertion failed: llpos < lend");
            }
            let origcost = crate::src_blocksplitter::EstimateCost(lz77, lstart as crate::types::size_t, lend as crate::types::size_t);
            if splitcost > origcost || llpos == lstart + 1 || llpos == lend {
                *done.add(lstart) = 1;
            } else {
                crate::src_blocksplitter::AddSorted(llpos, splitpoints as *mut *mut usize, npoints as *mut usize);
                numblocks += 1;
            }
            let splitpoints_val = *splitpoints;
            let npoints_val = *npoints;
            let found = crate::src_blocksplitter::FindLargestSplittableBlock(
                lz77_size,
                done as *const u8,
                splitpoints_val as *const usize,
                npoints_val as usize,
                &mut lstart as *mut usize,
                &mut lend as *mut usize,
            );
            if found == 0 {
                break;
            }
            if lend - lstart < 10 {
                break;
            }
        }
        let verbose = (*options).verbose;
        if verbose != 0 {
            let splitpoints_val = *splitpoints;
            let npoints_val = *npoints;
            crate::src_blocksplitter::PrintBlockSplitPoints(lz77, splitpoints_val, npoints_val);
        }
        libc::free(done as *mut libc::c_void);
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_blocksplitter_7
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn ZopfliBlockSplit(options: *const crate::types::ZopfliOptions, in_: *const ::core::ffi::c_uchar, instart: crate::types::size_t, inend: crate::types::size_t, maxblocks: crate::types::size_t, splitpoints: *mut *mut crate::types::size_t, npoints: *mut crate::types::size_t) {
    let mut s = std::mem::MaybeUninit::<crate::types::ZopfliBlockState>::uninit();
    let mut store = std::mem::MaybeUninit::<crate::types::ZopfliLZ77Store>::uninit();
    let mut hash = std::mem::MaybeUninit::<crate::types::ZopfliHash>::uninit();
    let mut lz77splitpoints: *mut crate::types::size_t = std::ptr::null_mut();
    let mut nlz77points: crate::types::size_t = 0;
    let mut pos: crate::types::size_t = 0;
    let mut i: crate::types::size_t = 0;

    unsafe {
        crate::src_lz77::ZopfliInitLZ77Store(in_, store.as_mut_ptr());
        crate::src_lz77::ZopfliInitBlockState(options, instart, inend, 0, s.as_mut_ptr());
        crate::src_hash::ZopfliAllocHash(32768, hash.as_mut_ptr());

        *npoints = 0;
        *splitpoints = std::ptr::null_mut();

        crate::src_lz77::ZopfliLZ77Greedy(s.as_mut_ptr(), in_, instart, inend, store.as_mut_ptr(), hash.as_mut_ptr());

        crate::src_blocksplitter::ZopfliBlockSplitLZ77(options, store.as_ptr(), maxblocks, &mut lz77splitpoints, &mut nlz77points);

        pos = instart;
        if nlz77points > 0 {
            let store_ref = store.assume_init_ref();
            let store_size = if !store_ref.litlens.is_null() {
                let _ = store_ref.litlens;
                0
            } else {
                0
            };
            for i in 0..store_size {
                let length = if !store_ref.dists.is_null() && unsafe { *store_ref.dists.add(i) } == 0 {
                    1
                } else if !store_ref.litlens.is_null() {
                    unsafe { *store_ref.litlens.add(i) as crate::types::size_t }
                } else {
                    1
                };
                if !lz77splitpoints.is_null() && unsafe { *lz77splitpoints.add(*npoints) } == i {
                    if (*npoints & (*npoints).wrapping_sub(1)) == 0 {
                        let new_size = if *npoints == 0 { 1 } else { *npoints * 2 };
                        let new_ptr = libc::realloc(*splitpoints as *mut libc::c_void, (new_size * std::mem::size_of::<crate::types::size_t>()) as libc::size_t);
                        *splitpoints = new_ptr as *mut crate::types::size_t;
                    }
                    if !(*splitpoints).is_null() {
                        unsafe { *(*splitpoints).add(*npoints) = pos };
                    }
                    *npoints += 1;
                    if *npoints == nlz77points {
                        break;
                    }
                }
                pos = pos.wrapping_add(length);
            }
        }
        assert!(*npoints == nlz77points);

        if !lz77splitpoints.is_null() {
            libc::free(lz77splitpoints as *mut libc::c_void);
        }
        crate::src_lz77::ZopfliCleanBlockState(s.as_mut_ptr());
        crate::src_lz77::ZopfliCleanLZ77Store(store.as_mut_ptr());
        crate::src_hash::ZopfliCleanHash(hash.as_mut_ptr());
    }
}

pub extern "C" fn ZopfliBlockSplitSimple(in_: *const ::core::ffi::c_uchar, instart: crate::types::size_t, inend: crate::types::size_t, blocksize: crate::types::size_t, splitpoints: *mut *mut crate::types::size_t, npoints: *mut crate::types::size_t) {
    let mut i = instart;
    while i < inend {
        unsafe {
            let n = *npoints;
            if n & (n.wrapping_sub(1)) == 0 {
                let new_size = if n == 0 { 1 } else { n * 2 };
                let new_ptr = libc::realloc(
                    (*splitpoints) as *mut libc::c_void,
                    (new_size as usize).wrapping_mul(std::mem::size_of::<crate::types::size_t>())
                ) as *mut crate::types::size_t;
                if new_ptr.is_null() {
                    return;
                }
                *splitpoints = new_ptr;
            }
            let arr = *splitpoints;
            *arr.add(*npoints as usize) = i;
            *npoints += 1;
        }
        i += blocksize;
    }
    let _ = in_;
}
