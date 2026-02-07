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
    if end - start < 1024 {
        let mut best: f64 = 1e30;
        let mut result: usize = start;
        for i in start..end {
            let v = if let Some(func) = f {
                unsafe { func(i as u64, context) }
            } else {
                1e30
            };
            if v < best {
                best = v;
                result = i;
            }
        }
        unsafe { *smallest = best; }
        return result;
    } else {
        let mut p: [usize; 9] = [0; 9];
        let mut vp: [f64; 9] = [0.0; 9];
        let mut besti: usize;
        let mut best: f64;
        let mut lastbest: f64 = 1e30;
        let mut pos: usize = start;
        let mut start = start;
        let mut end = end;

        loop {
            if end - start <= 9 {
                break;
            }

            for i in 0..9 {
                p[i] = start + (i + 1) * ((end - start) / 10);
                vp[i] = if let Some(func) = f {
                    unsafe { func(p[i] as u64, context) }
                } else {
                    1e30
                };
            }
            besti = 0;
            best = vp[0];
            for i in 1..9 {
                if vp[i] < best {
                    best = vp[i];
                    besti = i;
                }
            }
            if best > lastbest {
                break;
            }

            start = if besti == 0 { start } else { p[besti - 1] };
            end = if besti == 8 { end } else { p[besti + 1] };

            pos = p[besti];
            lastbest = best;
        }
        unsafe { *smallest = lastbest; }
        return pos;
    }
}

fn EstimateCost(lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize) -> f64 {
    crate::src_deflate::ZopfliCalculateBlockSizeAutoType(lz77, lstart, lend)
}

fn SplitCost(i: usize, context: *mut std::ffi::c_void) -> f64 {
    let c = context as *mut crate::types::SplitCostContext;
    unsafe {
        let lz77 = (*c).lz77;
        let start = (*c).start;
        let end = (*c).end;
        crate::src_blocksplitter::EstimateCost(lz77, start, i) + crate::src_blocksplitter::EstimateCost(lz77, i, end)
    }
}

fn AddSorted(value: usize, out: *mut *mut usize, outsize: *mut usize) {
    unsafe {
        // ZOPFLI_APPEND_DATA macro expansion
        if !((*outsize) & ((*outsize).wrapping_sub(1))) != 0 {
            if *outsize == 0 {
                *out = libc::malloc(std::mem::size_of::<usize>()) as *mut usize;
            } else {
                *out = libc::realloc(
                    *out as *mut std::ffi::c_void,
                    (*outsize * 2 * std::mem::size_of::<usize>()) as usize,
                ) as *mut usize;
            }
        }
        *(*out).add(*outsize) = value;
        *outsize += 1;

        // Insertion sort to maintain sorted order
        let mut i: usize = 0;
        while i + 1 < *outsize {
            if *(*out).add(i) > value {
                let mut j: usize = *outsize - 1;
                while j > i {
                    *(*out).add(j) = *(*out).add(j - 1);
                    j -= 1;
                }
                *(*out).add(i) = value;
                break;
            }
            i += 1;
        }
    }
}

fn PrintBlockSplitPoints(lz77: *const crate::types::ZopfliLZ77Store, lz77splitpoints: *const usize, nlz77points: usize) {
    unsafe {
        let mut splitpoints: *mut usize = std::ptr::null_mut();
        let mut npoints: usize = 0;
        let mut i: usize;

        let mut pos: usize = 0;
        if nlz77points > 0 {
            let size_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(lz77 as *mut ::core::ffi::c_void);
            let lz77_size = *(size_ptr as *const usize);
            
            let dists_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__dists(lz77 as *mut ::core::ffi::c_void);
            let dists = *(dists_ptr as *const *mut u16);
            
            let litlens_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__litlens(lz77 as *mut ::core::ffi::c_void);
            let litlens = *(litlens_ptr as *const *mut u16);
            
            i = 0;
            while i < lz77_size {
                let dist_val = *dists.add(i);
                let litlen_val = *litlens.add(i);
                let length: usize = if dist_val == 0 { 1 } else { litlen_val as usize };
                
                if *lz77splitpoints.add(npoints) == i {
                    if (npoints & (npoints.wrapping_sub(1))) == 0 {
                        if npoints == 0 {
                            splitpoints = libc::malloc(std::mem::size_of::<usize>()) as *mut usize;
                        } else {
                            splitpoints = libc::realloc(splitpoints as *mut libc::c_void, npoints * 2 * std::mem::size_of::<usize>()) as *mut usize;
                        }
                    }
                    *splitpoints.add(npoints) = pos;
                    npoints += 1;
                    if npoints == nlz77points {
                        break;
                    }
                }
                pos += length;
                i += 1;
            }
        }
        
        assert!(npoints == nlz77points);

        extern "C" {
            static stderr: *mut libc::FILE;
        }
        
        libc::fprintf(stderr, b"block split points: \0".as_ptr() as *const libc::c_char);
        i = 0;
        while i < npoints {
            libc::fprintf(stderr, b"%d \0".as_ptr() as *const libc::c_char, *splitpoints.add(i) as libc::c_int);
            i += 1;
        }
        libc::fprintf(stderr, b"(hex:\0".as_ptr() as *const libc::c_char);
        i = 0;
        while i < npoints {
            libc::fprintf(stderr, b" %x\0".as_ptr() as *const libc::c_char, *splitpoints.add(i) as libc::c_int);
            i += 1;
        }
        libc::fprintf(stderr, b")\n\0".as_ptr() as *const libc::c_char);

        libc::free(splitpoints as *mut libc::c_void);
    }
}

fn FindLargestSplittableBlock(lz77size: usize, done: *const u8, splitpoints: *const usize, npoints: usize, lstart: *mut usize, lend: *mut usize) -> i32 {
    let mut longest: usize = 0;
    let mut found: i32 = 0;
    let mut i: usize;
    
    i = 0;
    while i <= npoints {
        let start: usize = if i == 0 { 0 } else { unsafe { *splitpoints.add(i - 1) } };
        let end: usize = if i == npoints { lz77size - 1 } else { unsafe { *splitpoints.add(i) } };
        
        let done_at_start = unsafe { *done.add(start) };
        if done_at_start == 0 && end - start > longest {
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

pub extern "C" fn ZopfliBlockSplitLZ77(options: *const crate::types::ZopfliOptions, lz77: *const crate::types::ZopfliLZ77Store, maxblocks: crate::types::size_t, splitpoints: *mut *mut crate::types::size_t, npoints: *mut crate::types::size_t) {
    unsafe {
        let lz77_size_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(lz77 as *mut ::core::ffi::c_void);
        let lz77_size = *(lz77_size_ptr as *const usize);
        
        if lz77_size < 10 {
            return;
        }
        
        let done: *mut u8 = libc::malloc(lz77_size) as *mut u8;
        if done.is_null() {
            libc::exit(-1);
        }
        
        for i in 0..lz77_size {
            *done.add(i) = 0;
        }
        
        let mut lstart: usize = 0;
        let mut lend: usize = lz77_size;
        let mut llpos: usize;
        let mut numblocks: usize = 1;
        
        loop {
            if maxblocks > 0 && numblocks >= maxblocks {
                break;
            }
            
            let mut c = crate::types::SplitCostContext {
                lz77,
                start: lstart,
                end: lend,
                _c2r_private: [],
            };
            
            let mut splitcost: f64 = 0.0;
            let split_cost_fn: crate::types::FindMinimumFun = std::mem::transmute(crate::src_blocksplitter::SplitCost as usize);
            llpos = crate::src_blocksplitter::FindMinimum(
                split_cost_fn,
                &mut c as *mut crate::types::SplitCostContext as *mut ::core::ffi::c_void,
                lstart + 1,
                lend,
                &mut splitcost,
            );
            
            let origcost = crate::src_blocksplitter::EstimateCost(lz77, lstart, lend);
            
            if splitcost > origcost || llpos == lstart + 1 || llpos == lend {
                *done.add(lstart) = 1;
            } else {
                crate::src_blocksplitter::AddSorted(llpos, splitpoints, npoints);
                numblocks += 1;
            }
            
            if crate::src_blocksplitter::FindLargestSplittableBlock(
                lz77_size,
                done as *const u8,
                *splitpoints as *const usize,
                *npoints,
                &mut lstart,
                &mut lend,
            ) == 0 {
                break;
            }
            
            if lend - lstart < 10 {
                break;
            }
        }
        
        let verbose_ptr = crate::compat::c2r_field_ptr_ZopfliOptions__verbose(options as *mut ::core::ffi::c_void);
        let verbose = *(verbose_ptr as *const i32);
        if verbose != 0 {
            crate::src_blocksplitter::PrintBlockSplitPoints(lz77, *splitpoints as *const usize, *npoints);
        }
        
        libc::free(done as *mut ::core::ffi::c_void);
    }
}

pub extern "C" fn ZopfliBlockSplit(options: *const crate::types::ZopfliOptions, in_: *const ::core::ffi::c_uchar, instart: crate::types::size_t, inend: crate::types::size_t, maxblocks: crate::types::size_t, splitpoints: *mut *mut crate::types::size_t, npoints: *mut crate::types::size_t) {
    unsafe {
        let mut pos: crate::types::size_t = 0;
        let mut i: crate::types::size_t;
        let mut s: crate::types::ZopfliBlockState = std::mem::zeroed();
        let mut lz77splitpoints: *mut crate::types::size_t = std::ptr::null_mut();
        let mut nlz77points: crate::types::size_t = 0;
        let mut store: crate::types::ZopfliLZ77Store = std::mem::zeroed();
        let mut hash: crate::types::ZopfliHash = std::mem::zeroed();
        let h: *mut crate::types::ZopfliHash = &mut hash;

        crate::src_lz77::ZopfliInitLZ77Store(in_, &mut store);
        crate::src_lz77::ZopfliInitBlockState(options, instart, inend, 0, &mut s);
        crate::src_hash::ZopfliAllocHash(32768, h);

        *npoints = 0;
        *splitpoints = std::ptr::null_mut();

        crate::src_lz77::ZopfliLZ77Greedy(&mut s, in_, instart, inend, &mut store, h);

        crate::src_blocksplitter::ZopfliBlockSplitLZ77(options, &store, maxblocks, &mut lz77splitpoints, &mut nlz77points);

        pos = instart;
        if nlz77points > 0 {
            let store_ptr = &store as *const crate::types::ZopfliLZ77Store as *const u8;
            let size_offset = std::mem::size_of::<*const u8>() * 4;
            let store_size = *(store_ptr.add(size_offset) as *const crate::types::size_t);
            let litlens_ptr = *(store_ptr.add(std::mem::size_of::<*const u8>()) as *const *mut u16);
            let dists_ptr = *(store_ptr.add(std::mem::size_of::<*const u8>() * 2) as *const *mut u16);
            
            i = 0;
            while i < store_size {
                let dist = *dists_ptr.add(i);
                let litlen = *litlens_ptr.add(i);
                let length: crate::types::size_t = if dist == 0 { 1 } else { litlen as crate::types::size_t };
                
                if *lz77splitpoints.add(*npoints) == i {
                    if (*npoints & ((*npoints).wrapping_sub(1))) == 0 {
                        if *npoints == 0 {
                            *splitpoints = libc::malloc(std::mem::size_of::<crate::types::size_t>()) as *mut crate::types::size_t;
                        } else {
                            *splitpoints = libc::realloc(*splitpoints as *mut libc::c_void, (*npoints * 2 * std::mem::size_of::<crate::types::size_t>())) as *mut crate::types::size_t;
                        }
                    }
                    *(*splitpoints).add(*npoints) = pos;
                    *npoints += 1;
                    if *npoints == nlz77points {
                        break;
                    }
                }
                pos += length;
                i += 1;
            }
        }

        libc::free(lz77splitpoints as *mut libc::c_void);
        crate::src_lz77::ZopfliCleanBlockState(&mut s);
        crate::src_lz77::ZopfliCleanLZ77Store(&mut store);
        crate::src_hash::ZopfliCleanHash(h);
    }
}

pub extern "C" fn ZopfliBlockSplitSimple(in_: *const ::core::ffi::c_uchar, instart: crate::types::size_t, inend: crate::types::size_t, blocksize: crate::types::size_t, splitpoints: *mut *mut crate::types::size_t, npoints: *mut crate::types::size_t) {
    let mut i = instart;
    while i < inend {
        unsafe {
            let n = *npoints;
            if (n & (n.wrapping_sub(1))) == 0 {
                if n == 0 {
                    *splitpoints = libc::malloc(std::mem::size_of::<crate::types::size_t>()) as *mut crate::types::size_t;
                } else {
                    *splitpoints = libc::realloc(
                        *splitpoints as *mut ::core::ffi::c_void,
                        n.wrapping_mul(2).wrapping_mul(std::mem::size_of::<crate::types::size_t>())
                    ) as *mut crate::types::size_t;
                }
            }
            *(*splitpoints).add(n) = i;
            *npoints = n + 1;
        }
        i += blocksize;
    }
    let _ = in_;
}
