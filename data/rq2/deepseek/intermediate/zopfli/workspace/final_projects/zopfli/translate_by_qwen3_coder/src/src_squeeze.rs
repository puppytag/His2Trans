//! Module: src_squeeze
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

fn InitStats(stats: *mut crate::types::SymbolStats) {
    if stats.is_null() {
        return;
    }
    unsafe {
        std::ptr::write_bytes((*stats).litlens.as_mut_ptr(), 0, 288);
        std::ptr::write_bytes((*stats).dists.as_mut_ptr(), 0, 32);
        std::ptr::write_bytes((*stats).ll_symbols.as_mut_ptr(), 0, 288);
        std::ptr::write_bytes((*stats).d_symbols.as_mut_ptr(), 0, 32);
    }
}

fn CopyStats(source: *mut crate::types::SymbolStats, dest: *mut crate::types::SymbolStats) {
    unsafe {
        std::ptr::copy_nonoverlapping(
            (*source).litlens.as_ptr(),
            (*dest).litlens.as_mut_ptr(),
            288,
        );
        std::ptr::copy_nonoverlapping(
            (*source).dists.as_ptr(),
            (*dest).dists.as_mut_ptr(),
            32,
        );
        std::ptr::copy_nonoverlapping(
            (*source).ll_symbols.as_ptr(),
            (*dest).ll_symbols.as_mut_ptr(),
            288,
        );
        std::ptr::copy_nonoverlapping(
            (*source).d_symbols.as_ptr(),
            (*dest).d_symbols.as_mut_ptr(),
            32,
        );
    }
}

fn AddWeighedStatFreqs(stats1: *const crate::types::SymbolStats, w1: f64, stats2: *const crate::types::SymbolStats, w2: f64, result: *mut crate::types::SymbolStats) {
    unsafe {
        for i in 0..288 {
            (*result).litlens[i] = ((*stats1).litlens[i] as f64 * w1 + (*stats2).litlens[i] as f64 * w2) as u64;
        }
        for i in 0..32 {
            (*result).dists[i] = ((*stats1).dists[i] as f64 * w1 + (*stats2).dists[i] as f64 * w2) as u64;
        }
        (*result).litlens[256] = 1;
    }
}

fn InitRanState(state: *mut crate::types::RanState) {
    if !state.is_null() {
        unsafe {
            (*state).m_w = 1;
            (*state).m_z = 2;
        }
    }
}

fn Ran(state: *mut crate::types::RanState) -> u32 {
    unsafe {
        let m_z = (*state).m_z;
        let m_w = (*state).m_w;
        let new_z = 36969 * (m_z & 65535) + (m_z >> 16);
        let new_w = 18000 * (m_w & 65535) + (m_w >> 16);
        (*state).m_z = new_z;
        (*state).m_w = new_w;
        (new_z << 16) + new_w
    }
}

fn RandomizeFreqs(state: *mut crate::types::RanState, freqs: *mut usize, n: std::os::raw::c_int) {
    if freqs.is_null() {
        return;
    }
    for i in 0..(n as isize) {
        if (crate::src_squeeze::Ran(state) >> 4) % 3 == 0 {
            let idx = crate::src_squeeze::Ran(state) % (n as u32);
            unsafe {
                *freqs.offset(i) = *freqs.offset(idx as isize);
            }
        }
    }
}

fn RandomizeStatFreqs(state: *mut crate::types::RanState, stats: *mut crate::types::SymbolStats) {
    unsafe {
        crate::src_squeeze::RandomizeFreqs(state, (*stats).litlens.as_mut_ptr() as *mut usize, 288);
        crate::src_squeeze::RandomizeFreqs(state, (*stats).dists.as_mut_ptr() as *mut usize, 32);
        (*stats).litlens[256] = 1;
    }
}

fn ClearStatFreqs(stats: *mut crate::types::SymbolStats) {
    if stats.is_null() {
        return;
    }
    unsafe {
        for i in 0..288 {
            (*stats).litlens[i] = 0;
        }
        for i in 0..32 {
            (*stats).dists[i] = 0;
        }
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_squeeze_9
// c_function: GetCostFixed
// rust_file: src_squeeze.rs
// rust_signature: fn GetCostFixed(litlen: std::os::raw::c_uint, dist: std::os::raw::c_uint, unused: *mut std::ffi::c_void) -> f64
// c_first_line: static double GetCostFixed(unsigned litlen, unsigned dist, void* unused) {
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_squeeze_9/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find function, tuple struct or tuple variant `ZopfliGetDistExtraBits` in module `crate::src_util`
//      --> src/src_squeeze.rs:116:38
//       |
//       |                                      ^^^^^^^^^^^^^^^^^^^^^^ not found in `crate::src_util`
//       |
//   help: consider importing this function
//       |
//       |
// =================================
fn GetCostFixed(litlen: std::os::raw::c_uint, dist: std::os::raw::c_uint, unused: *mut std::ffi::c_void) -> f64 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_squeeze::GetCostFixed(litlen as _, dist as _, unused as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_squeeze_9
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_squeeze_9/translated_rust.rs
 * ------------------------------------------------------------
fn GetCostFixed(litlen: std::os::raw::c_uint, dist: std::os::raw::c_uint, unused: *mut std::ffi::c_void) -> f64 {
    let _ = unused;
    if dist == 0 {
        if litlen <= 143 {
            return 8.0;
        } else {
            return 9.0;
        }
    } else {
        let dbits = crate::src_util::ZopfliGetDistExtraBits(dist as i32) as f64;
        let lbits = crate::src_util::ZopfliGetLengthExtraBits(litlen as i32) as f64;
        let lsym = crate::src_util::ZopfliGetLengthSymbol(litlen as i32);
        let mut cost = 0;
        if lsym <= 279 {
            cost += 7;
        } else {
            cost += 8;
        }
        cost += 5;
        return cost as f64 + dbits + lbits;
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_squeeze_9
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_squeeze_10
// c_function: GetCostStat
// rust_file: src_squeeze.rs
// rust_signature: fn GetCostStat(litlen: u32, dist: u32, context: *mut std::ffi::c_void) -> f64
// c_first_line: static double GetCostStat(unsigned litlen, unsigned dist, void* context) {
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_squeeze_10/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find function, tuple struct or tuple variant `ZopfliGetLengthSymbol` in module `crate::src_util`
//      --> src/src_squeeze.rs:166:37
//       |
//       |                                     ^^^^^^^^^^^^^^^^^^^^^ not found in `crate::src_util`
//       |
//   help: consider importing one of these functions
//       |
//       |
// =================================
fn GetCostStat(litlen: u32, dist: u32, context: *mut std::ffi::c_void) -> f64 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_squeeze::GetCostStat(litlen as _, dist as _, context as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_squeeze_10
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_squeeze_10/translated_rust.rs
 * ------------------------------------------------------------
fn GetCostStat(litlen: u32, dist: u32, context: *mut std::ffi::c_void) -> f64 {
    let stats = context as *mut crate::types::SymbolStats;
    if dist == 0 {
        unsafe { (*stats).ll_symbols[litlen as usize] as f64 }
    } else {
        let lsym = crate::src_util::ZopfliGetLengthSymbol(litlen as i32);
        let lbits = crate::src_util::ZopfliGetLengthExtraBits(litlen as i32) as f64;
        let dsym = crate::src_util::ZopfliGetDistSymbol(dist as i32);
        let dbits = crate::src_util::ZopfliGetDistExtraBits(dist as i32) as f64;
        unsafe {
            lbits + dbits + (*stats).ll_symbols[lsym as usize] as f64 + (*stats).d_symbols[dsym as usize] as f64
        }
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_squeeze_10
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn GetCostModelMinCost(costmodel: *mut crate::types::CostModelFun, costcontext: *mut std::ffi::c_void) -> f64 {
    let mut mincost: f64;
    let mut bestlength: i32 = 0;
    let mut bestdist: i32 = 0;
    let dsymbols: [i32; 30] = [
        1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513,
        769, 1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
    ];
    mincost = 1e30;
    for i in 3..259 {
        let c = if let Some(f) = unsafe { *costmodel } {
            unsafe { f(i as u32, 1, costcontext) }
        } else {
            0.0
        };
        if c < mincost {
            bestlength = i;
            mincost = c;
        }
    }
    mincost = 1e30;
    for i in 0..30 {
        let c = if let Some(f) = unsafe { *costmodel } {
            unsafe { f(3, dsymbols[i] as u32, costcontext) }
        } else {
            0.0
        };
        if c < mincost {
            bestdist = dsymbols[i];
            mincost = c;
        }
    }
    if let Some(f) = unsafe { *costmodel } {
        unsafe { f(bestlength as u32, bestdist as u32, costcontext) }
    } else {
        0.0
    }
}

fn zopfli_min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

fn GetBestLengths(s: *mut crate::types::ZopfliBlockState, r#in: *const u8, instart: usize, inend: usize, costmodel: *mut crate::types::CostModelFun, costcontext: *mut std::ffi::c_void, length_array: *mut u16, h: *mut crate::types::ZopfliHash, costs: *mut f32) -> f64 {
    let blocksize = inend - instart;
    let mut i = 0;
    let mut k: usize;
    let mut kend: usize;
    let mut leng: u16 = 0;
    let mut dist: u16 = 0;
    let mut sublen: [u16; 259] = [0; 259];
    let windowstart = if instart > 32768 { instart - 32768 } else { 0 };
    let mut result: f64;
    let mincost = crate::src_squeeze::GetCostModelMinCost(costmodel, costcontext);
    let mut mincostaddcostj: f64;

    if instart == inend {
        return 0.0;
    }

    crate::src_hash::ZopfliResetHash(32768 as crate::types::size_t, h);
    crate::src_hash::ZopfliWarmupHash(r#in as *const ::core::ffi::c_uchar, windowstart as crate::types::size_t, inend as crate::types::size_t, h);
    i = windowstart;
    while i < instart {
        crate::src_hash::ZopfliUpdateHash(r#in as *const ::core::ffi::c_uchar, i as crate::types::size_t, inend as crate::types::size_t, h);
        i += 1;
    }

    i = 1;
    while i < blocksize + 1 {
        unsafe {
            *costs.add(i) = 1e30;
        }
        i += 1;
    }
    unsafe {
        *costs = 0.0;
        *length_array = 0;
    }

    i = instart;
    while i < inend {
        let j = i - instart;
        crate::src_hash::ZopfliUpdateHash(r#in as *const ::core::ffi::c_uchar, i as crate::types::size_t, inend as crate::types::size_t, h);

        let same_ptr = unsafe {
            crate::compat::c2r_field_ptr_ZopfliHash__same(h as *mut ::core::ffi::c_void)
        };
        let same_val_i = unsafe { *(same_ptr as *const u32).add(i & (32768 - 1)) };
        let same_val_i_minus_258 = unsafe { *(same_ptr as *const u32).add((i - 258) & (32768 - 1)) };
        if same_val_i > 258 * 2
            && i > instart + 258 + 1
            && i + 258 * 2 + 1 < inend
            && same_val_i_minus_258 > 258
        {
            let costmodel_fn = unsafe { core::mem::transmute::<*mut crate::types::CostModelFun, Option<unsafe extern "C" fn(u32, u32, *mut std::ffi::c_void) -> f64>>(costmodel) };
            let symbolcost = if let Some(f) = costmodel_fn {
                unsafe { f(258, 1, costcontext) }
            } else {
                0.0
            };

            k = 0;
            while k < 258 {
                unsafe {
                    *costs.add(j + 258) = *costs.add(j) + symbolcost as f32;
                    *length_array.add(j + 258) = 258;
                }
                i += 1;
                crate::src_hash::ZopfliUpdateHash(r#in as *const ::core::ffi::c_uchar, i as crate::types::size_t, inend as crate::types::size_t, h);
                k += 1;
            }
        }

        crate::src_lz77::ZopfliFindLongestMatch(
            s,
            h as *const crate::types::ZopfliHash,
            r#in as *const ::core::ffi::c_uchar,
            i as crate::types::size_t,
            inend as crate::types::size_t,
            258 as crate::types::size_t,
            sublen.as_mut_ptr() as *mut ::core::ffi::c_ushort,
            &mut dist as *mut u16 as *mut ::core::ffi::c_ushort,
            &mut leng as *mut u16 as *mut ::core::ffi::c_ushort,
        );

        if i + 1 <= inend {
            let costmodel_fn = unsafe { core::mem::transmute::<*mut crate::types::CostModelFun, Option<unsafe extern "C" fn(u32, u32, *mut std::ffi::c_void) -> f64>>(costmodel) };
            let newCost = if let Some(f) = costmodel_fn {
                unsafe { f(r#in.add(i) as *const u8 as u32, 0, costcontext) }
            } else {
                0.0
            } + unsafe { *costs.add(j) as f64 };
            let _ = newCost >= 0.0;
            if newCost < unsafe { *costs.add(j + 1) as f64 } {
                unsafe {
                    *costs.add(j + 1) = newCost as f32;
                    *length_array.add(j + 1) = 1;
                }
            }
        }

        kend = crate::src_squeeze::zopfli_min(leng as usize, inend - i);
        mincostaddcostj = mincost + unsafe { *costs.add(j) as f64 };
        k = 3;
        while k <= kend {
            if unsafe { *costs.add(j + k) as f64 } <= mincostaddcostj {
                k += 1;
                continue;
            }
            let costmodel_fn = unsafe { core::mem::transmute::<*mut crate::types::CostModelFun, Option<unsafe extern "C" fn(u32, u32, *mut std::ffi::c_void) -> f64>>(costmodel) };
            let newCost = if let Some(f) = costmodel_fn {
                unsafe { f(k as u32, sublen[k] as u32, costcontext) }
            } else {
                0.0
            } + unsafe { *costs.add(j) as f64 };
            let _ = newCost >= 0.0;
            if newCost < unsafe { *costs.add(j + k) as f64 } {
                let _ = k <= 258;
                unsafe {
                    *costs.add(j + k) = newCost as f32;
                    *length_array.add(j + k) = k as u16;
                }
            }
            k += 1;
        }
        i += 1;
    }

    let _ = unsafe { *costs.add(blocksize) } >= 0.0;
    result = unsafe { *costs.add(blocksize) as f64 };
    result
}

fn TraceBackwards(size: usize, length_array: *const u16, path: *mut *mut u16, pathsize: *mut usize) {
    if size == 0 {
        return;
    }
    let mut index = size;
    loop {
        unsafe {
            if (*pathsize) & ((*pathsize) - 1) == 0 {
                if *pathsize == 0 {
                    *path = libc::malloc(std::mem::size_of::<u16>()) as *mut u16;
                } else {
                    *path = libc::realloc(*path as *mut libc::c_void, (*pathsize) * 2 * std::mem::size_of::<u16>()) as *mut u16;
                }
            }
            let path_ptr = *path;
            if !path_ptr.is_null() {
                *path_ptr.add(*pathsize) = *length_array.add(index);
            }
            *pathsize += 1;
            let val = *length_array.add(index);
            assert!(val <= index as u16);
            assert!(val <= 258);
            assert!(val != 0);
            index -= val as usize;
        }
        if index == 0 {
            break;
        }
    }
    unsafe {
        let mut i = 0;
        while i < *pathsize / 2 {
            let temp = (*path).add(i).read();
            (*path).add(i).write((*path).add(*pathsize - i - 1).read());
            (*path).add(*pathsize - i - 1).write(temp);
            i += 1;
        }
    }
}

fn FollowPath(s: *mut crate::types::ZopfliBlockState, r#in: *const u8, instart: usize, inend: usize, path: *mut u16, pathsize: usize, store: *mut crate::types::ZopfliLZ77Store, h: *mut crate::types::ZopfliHash) {
    let mut pos: usize = 0;
    let windowstart = if instart > 32768 { instart - 32768 } else { 0 };
    let mut total_length_test: usize = 0;
    if instart == inend {
        return;
    }
    unsafe {
        crate::src_hash::ZopfliResetHash(32768 as crate::types::size_t, h);
        crate::src_hash::ZopfliWarmupHash(r#in, windowstart as crate::types::size_t, inend as crate::types::size_t, h);
        let mut i = windowstart;
        while i < instart {
            crate::src_hash::ZopfliUpdateHash(r#in, i as crate::types::size_t, inend as crate::types::size_t, h);
            i += 1;
        }
    }
    pos = instart;
    let mut i: usize = 0;
    while i < pathsize {
        let mut length: u16 = unsafe { *path.add(i) };
        let mut dummy_length: u16 = 0;
        let mut dist: u16 = 0;
        unsafe {
            assert!(pos < inend);
            crate::src_hash::ZopfliUpdateHash(r#in, pos as crate::types::size_t, inend as crate::types::size_t, h);
        }
        if length >= 3 {
            unsafe {
                crate::src_lz77::ZopfliFindLongestMatch(s, h as *const crate::types::ZopfliHash, r#in as *const ::core::ffi::c_uchar, pos as crate::types::size_t, inend as crate::types::size_t, length as crate::types::size_t, 0 as *mut ::core::ffi::c_ushort, &mut dist as *mut u16 as *mut ::core::ffi::c_ushort, &mut dummy_length as *mut u16 as *mut ::core::ffi::c_ushort);
                assert!(!(dummy_length != length && length > 2 && dummy_length > 2));
                crate::src_lz77::ZopfliVerifyLenDist(r#in as *const ::core::ffi::c_uchar, inend as crate::types::size_t, pos as crate::types::size_t, dist, length);
                crate::src_lz77::ZopfliStoreLitLenDist(length, dist, pos as crate::types::size_t, store);
            }
            total_length_test += length as usize;
        } else {
            length = 1;
            unsafe {
                crate::src_lz77::ZopfliStoreLitLenDist(r#in.add(pos) as u16, 0, pos as crate::types::size_t, store);
            }
            total_length_test += 1;
        }
        unsafe {
            assert!(pos + length as usize <= inend);
            let mut j: usize = 1;
            while j < length as usize {
                crate::src_hash::ZopfliUpdateHash(r#in, (pos + j) as crate::types::size_t, inend as crate::types::size_t, h);
                j += 1;
            }
        }
        pos += length as usize;
        i += 1;
    }
}

fn CalculateStatistics(stats: *mut crate::types::SymbolStats) {
    unsafe {
        crate::src_tree::ZopfliCalculateEntropy(
            ((*stats).litlens).as_ptr() as *const crate::types::size_t,
            288 as crate::types::size_t,
            ((*stats).ll_symbols).as_mut_ptr() as *mut f64,
        );
        crate::src_tree::ZopfliCalculateEntropy(
            ((*stats).dists).as_ptr() as *const crate::types::size_t,
            32 as crate::types::size_t,
            ((*stats).d_symbols).as_mut_ptr() as *mut f64,
        );
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_squeeze_17
// c_function: GetStatistics
// rust_file: src_squeeze.rs
// rust_signature: fn GetStatistics(store: *const crate::types::ZopfliLZ77Store, stats: *mut crate::types::SymbolStats)
// c_first_line: static void GetStatistics(const ZopfliLZ77Store* store, SymbolStats* stats) {
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_squeeze_17/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find function, tuple struct or tuple variant `ZopfliGetLengthSymbol` in module `crate::src_tree`
//      --> src/src_squeeze.rs:340:50
//       |
//       |                                                  ^^^^^^^^^^^^^^^^^^^^^
//       |
//       |
//       | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `ZopfliLengthsToSymbols` defined here
//       |
// =================================
fn GetStatistics(store: *const crate::types::ZopfliLZ77Store, stats: *mut crate::types::SymbolStats) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_squeeze::GetStatistics(store as _, stats as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_squeeze_17
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_squeeze_17/translated_rust.rs
 * ------------------------------------------------------------
fn GetStatistics(store: *const crate::types::ZopfliLZ77Store, stats: *mut crate::types::SymbolStats) {
    unsafe {
        let store_size = (*store).size;
        let store_litlens_ptr = (*store).litlens;
        let store_dists_ptr = (*store).dists;
        let stats_litlens_ptr = (*stats).litlens.as_mut_ptr();
        let stats_dists_ptr = (*stats).dists.as_mut_ptr();
        for i in 0..store_size as usize {
            let litlen = *store_litlens_ptr.add(i);
            let dist = *store_dists_ptr.add(i);
            if dist == 0 {
                *stats_litlens_ptr.add(litlen as usize) += 1;
            } else {
                let ll_symbol = crate::src_tree::ZopfliGetLengthSymbol(litlen as i32) as usize;
                let d_symbol = crate::src_tree::ZopfliGetDistSymbol(dist as i32) as usize;
                *stats_litlens_ptr.add(ll_symbol) += 1;
                *stats_dists_ptr.add(d_symbol) += 1;
            }
        }
        *stats_litlens_ptr.add(256) = 1;
        crate::src_squeeze::CalculateStatistics(stats);
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_squeeze_17
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn LZ77OptimalRun(s: *mut crate::types::ZopfliBlockState, r#in: *const std::ffi::c_uchar, instart: usize, inend: usize, path: *mut *mut std::ffi::c_ushort, pathsize: *mut usize, length_array: *mut std::ffi::c_ushort, costmodel: *mut crate::types::CostModelFun, costcontext: *mut std::ffi::c_void, store: *mut crate::types::ZopfliLZ77Store, h: *mut crate::types::ZopfliHash, costs: *mut std::ffi::c_float) -> f64 {
    let cost = crate::src_squeeze::GetBestLengths(s, r#in, instart, inend, costmodel, costcontext, length_array, h, costs);
    unsafe {
        libc::free((*path) as *mut std::ffi::c_void);
        *path = std::ptr::null_mut();
        *pathsize = 0;
    }
    crate::src_squeeze::TraceBackwards(inend - instart, length_array, path, pathsize);
    unsafe {
        crate::src_squeeze::FollowPath(s, r#in, instart, inend, *path, *pathsize, store, h);
    }
    let _ = cost < 1e30;
    cost
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_squeeze_19
// c_function: ZopfliLZ77Optimal
// rust_file: src_squeeze.rs
// rust_signature: pub extern "C" fn ZopfliLZ77Optimal(s: *mut crate::types::ZopfliBlockState, in_: *const ::core::ffi::c_uchar, instart: crate::types::size_t, inend: crate::types::size_t, numiterations: ::core::ffi::c_int, store: *mut cra...
// c_first_line: void ZopfliLZ77Optimal(ZopfliBlockState *s,
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_squeeze_19/translated_rust.rs
// last_error_truncated:
//   error[E0308]: mismatched types
//      --> src/src_squeeze.rs:615:44
//       |
//       |                              ------------- ^^^^^^^^^^^^^^^^^^^^^ expected `libc::FILE`, found `types::FILE`
//       |                              |
//       |                              arguments to this function are incorrect
//       |
//      --> src/types.rs:69:1
// =================================
pub extern "C" fn ZopfliLZ77Optimal(s: *mut crate::types::ZopfliBlockState, in_: *const ::core::ffi::c_uchar, instart: crate::types::size_t, inend: crate::types::size_t, numiterations: ::core::ffi::c_int, store: *mut crate::types::ZopfliLZ77Store) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_squeeze::ZopfliLZ77Optimal(s as _, in_ as _, instart as _, inend as _, numiterations as _, store as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_squeeze_19
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_squeeze_19/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn ZopfliLZ77Optimal(s: *mut crate::types::ZopfliBlockState, in_: *const ::core::ffi::c_uchar, instart: crate::types::size_t, inend: crate::types::size_t, numiterations: ::core::ffi::c_int, store: *mut crate::types::ZopfliLZ77Store) {
    let blocksize = (inend - instart) as usize;
    let length_array = unsafe { libc::malloc((blocksize + 1) * std::mem::size_of::<u16>()) } as *mut u16;
    let mut path: *mut u16 = std::ptr::null_mut();
    let mut pathsize: usize = 0;
    let mut currentstore: crate::types::ZopfliLZ77Store = unsafe { std::mem::zeroed() };
    let mut hash: crate::types::ZopfliHash = unsafe { std::mem::zeroed() };
    let h = &mut hash as *mut crate::types::ZopfliHash;
    let mut stats: crate::types::SymbolStats = unsafe { std::mem::zeroed() };
    let mut beststats: crate::types::SymbolStats = unsafe { std::mem::zeroed() };
    let mut laststats: crate::types::SymbolStats = unsafe { std::mem::zeroed() };
    let costs = unsafe { libc::malloc((blocksize + 1) * std::mem::size_of::<f32>()) } as *mut f32;
    let mut cost: f64;
    let mut bestcost: f64 = 1e30;
    let mut lastcost: f64 = 0.0;
    let mut ran_state: crate::types::RanState = unsafe { std::mem::zeroed() };
    let mut lastrandomstep: i32 = -1;

    if costs.is_null() || length_array.is_null() {
        unsafe { libc::exit(-1) };
    }

    crate::src_squeeze::InitRanState(&mut ran_state as *mut crate::types::RanState);
    crate::src_squeeze::InitStats(&mut stats as *mut crate::types::SymbolStats);
    crate::src_lz77::ZopfliInitLZ77Store(in_, &mut currentstore as *mut crate::types::ZopfliLZ77Store);
    crate::src_hash::ZopfliAllocHash(32768 as crate::types::size_t, h);

    crate::src_lz77::ZopfliLZ77Greedy(s, in_, instart, inend, &mut currentstore as *mut crate::types::ZopfliLZ77Store, h);
    crate::src_squeeze::GetStatistics(&currentstore as *const crate::types::ZopfliLZ77Store, &mut stats as *mut crate::types::SymbolStats);

    for i in 0..numiterations {
        crate::src_lz77::ZopfliCleanLZ77Store(&mut currentstore as *mut crate::types::ZopfliLZ77Store);
        crate::src_lz77::ZopfliInitLZ77Store(in_, &mut currentstore as *mut crate::types::ZopfliLZ77Store);
        cost = crate::src_squeeze::LZ77OptimalRun(s, in_, instart as usize, inend as usize, &mut path as *mut *mut u16, &mut pathsize as *mut usize, length_array, crate::src_squeeze::GetCostStat as *mut crate::types::CostModelFun, &mut stats as *mut crate::types::SymbolStats as *mut ::core::ffi::c_void, &mut currentstore as *mut crate::types::ZopfliLZ77Store, h, costs);
        cost = crate::src_deflate::ZopfliCalculateBlockSize(&currentstore as *const crate::types::ZopfliLZ77Store, 0 as crate::types::size_t, currentstore.size as crate::types::size_t, 2);
        let options_ptr = unsafe {
            crate::compat::c2r_field_ptr_ZopfliBlockState__options(s as *mut ::core::ffi::c_void)
        };
        let verbose_more = unsafe { *(options_ptr as *const i32).offset(1) };
        let verbose = unsafe { *(options_ptr as *const i32) };
        if verbose_more != 0 || (verbose != 0 && cost < bestcost) {
            let _ = unsafe { libc::fprintf(crate::compat::stderr, b"Iteration %d: %d bit\n\0" as *const u8 as *const i8, i, cost as i32) };
        }
        if cost < bestcost {
            crate::src_lz77::ZopfliCopyLZ77Store(&currentstore as *const crate::types::ZopfliLZ77Store, store);
            crate::src_squeeze::CopyStats(&mut stats as *mut crate::types::SymbolStats, &mut beststats as *mut crate::types::SymbolStats);
            bestcost = cost;
        }
        crate::src_squeeze::CopyStats(&mut stats as *mut crate::types::SymbolStats, &mut laststats as *mut crate::types::SymbolStats);
        crate::src_squeeze::ClearStatFreqs(&mut stats as *mut crate::types::SymbolStats);
        crate::src_squeeze::GetStatistics(&currentstore as *const crate::types::ZopfliLZ77Store, &mut stats as *mut crate::types::SymbolStats);
        if lastrandomstep != -1 {
            crate::src_squeeze::AddWeighedStatFreqs(&stats as *const crate::types::SymbolStats, 1.0, &laststats as *const crate::types::SymbolStats, 0.5, &mut stats as *mut crate::types::SymbolStats);
            crate::src_squeeze::CalculateStatistics(&mut stats as *mut crate::types::SymbolStats);
        }
        if i > 5 && cost == lastcost {
            crate::src_squeeze::CopyStats(&mut beststats as *mut crate::types::SymbolStats, &mut stats as *mut crate::types::SymbolStats);
            crate::src_squeeze::RandomizeStatFreqs(&mut ran_state as *mut crate::types::RanState, &mut stats as *mut crate::types::SymbolStats);
            crate::src_squeeze::CalculateStatistics(&mut stats as *mut crate::types::SymbolStats);
            lastrandomstep = i;
        }
        lastcost = cost;
    }

    unsafe {
        libc::free(length_array as *mut ::core::ffi::c_void);
        libc::free(path as *mut ::core::ffi::c_void);
        libc::free(costs as *mut ::core::ffi::c_void);
    }
    crate::src_lz77::ZopfliCleanLZ77Store(&mut currentstore as *mut crate::types::ZopfliLZ77Store);
    crate::src_hash::ZopfliCleanHash(h);
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_squeeze_19
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn ZopfliLZ77OptimalFixed(s: *mut crate::types::ZopfliBlockState, in_: *const ::core::ffi::c_uchar, instart: crate::types::size_t, inend: crate::types::size_t, store: *mut crate::types::ZopfliLZ77Store) {
    let blocksize = (inend - instart) as usize;
    let length_array = unsafe { libc::malloc((blocksize + 1) * std::mem::size_of::<std::ffi::c_ushort>()) as *mut std::ffi::c_ushort };
    let mut path: *mut std::ffi::c_ushort = std::ptr::null_mut();
    let mut pathsize: usize = 0;
    let mut hash = std::mem::MaybeUninit::<crate::types::ZopfliHash>::uninit();
    let h = unsafe { hash.as_mut_ptr() };
    let costs = unsafe { libc::malloc((blocksize + 1) * std::mem::size_of::<std::ffi::c_float>()) as *mut std::ffi::c_float };

    if costs.is_null() {
        unsafe { libc::exit(-1) };
    }
    if length_array.is_null() {
        unsafe { libc::exit(-1) };
    }

    unsafe {
        crate::src_hash::ZopfliAllocHash(32768 as crate::types::size_t, h);
    }

    let _ = crate::src_squeeze::LZ77OptimalRun(
        s,
        in_,
        instart as usize,
        inend as usize,
        &mut path as *mut *mut std::ffi::c_ushort,
        &mut pathsize as *mut usize,
        length_array,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        store,
        h,
        costs,
    );

    unsafe {
        libc::free(length_array as *mut std::ffi::c_void);
        libc::free(path as *mut std::ffi::c_void);
        libc::free(costs as *mut std::ffi::c_void);
        crate::src_hash::ZopfliCleanHash(h);
    }
}
