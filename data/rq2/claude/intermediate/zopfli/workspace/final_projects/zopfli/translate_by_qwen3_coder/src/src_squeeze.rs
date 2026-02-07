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
    unsafe {
        std::ptr::write_bytes(
            (*stats).litlens.as_mut_ptr(),
            0,
            288,
        );
        std::ptr::write_bytes(
            (*stats).dists.as_mut_ptr(),
            0,
            32,
        );
        std::ptr::write_bytes(
            (*stats).ll_symbols.as_mut_ptr(),
            0,
            288,
        );
        std::ptr::write_bytes(
            (*stats).d_symbols.as_mut_ptr(),
            0,
            32,
        );
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
    unsafe {
        (*state).m_w = 1;
        (*state).m_z = 2;
    }
}

fn Ran(state: *mut crate::types::RanState) -> u32 {
    unsafe {
        (*state).m_z = 36969u32.wrapping_mul((*state).m_z & 65535).wrapping_add((*state).m_z >> 16);
        (*state).m_w = 18000u32.wrapping_mul((*state).m_w & 65535).wrapping_add((*state).m_w >> 16);
        ((*state).m_z << 16).wrapping_add((*state).m_w)
    }
}

fn RandomizeFreqs(state: *mut crate::types::RanState, freqs: *mut usize, n: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    while i < n {
        if (crate::src_squeeze::Ran(state) >> 4) % 3 == 0 {
            let idx = (crate::src_squeeze::Ran(state) % (n as u32)) as isize;
            unsafe {
                *freqs.offset(i as isize) = *freqs.offset(idx);
            }
        }
        i += 1;
    }
}

fn RandomizeStatFreqs(state: *mut crate::types::RanState, stats: *mut crate::types::SymbolStats) {
    unsafe {
        RandomizeFreqs(state, (*stats).litlens.as_mut_ptr() as *mut usize, 288);
        RandomizeFreqs(state, (*stats).dists.as_mut_ptr() as *mut usize, 32);
        (*stats).litlens[256] = 1;
    }
}

fn ClearStatFreqs(stats: *mut crate::types::SymbolStats) {
    let mut i: usize;
    i = 0;
    while i < 288 {
        unsafe {
            (*stats).litlens[i] = 0;
        }
        i += 1;
    }
    i = 0;
    while i < 32 {
        unsafe {
            (*stats).dists[i] = 0;
        }
        i += 1;
    }
}

fn GetCostFixed(litlen: u32, dist: u32, unused: *mut std::ffi::c_void) -> f64 {
    let _ = unused;
    if dist == 0 {
        if litlen <= 143 {
            return 8.0;
        } else {
            return 9.0;
        }
    } else {
        // Inline the logic from ZopfliGetDistExtraBits, ZopfliGetLengthExtraBits, ZopfliGetLengthSymbol
        // These are static inline functions from the C headers, not available as extern symbols
        
        // ZopfliGetDistExtraBits: extra bits for distance codes
        let dbits: i32 = if dist < 5 { 0 }
            else if dist < 9 { 1 }
            else if dist < 17 { 2 }
            else if dist < 33 { 3 }
            else if dist < 65 { 4 }
            else if dist < 129 { 5 }
            else if dist < 257 { 6 }
            else if dist < 513 { 7 }
            else if dist < 1025 { 8 }
            else if dist < 2049 { 9 }
            else if dist < 4097 { 10 }
            else if dist < 8193 { 11 }
            else if dist < 16385 { 12 }
            else { 13 };
        
        // ZopfliGetLengthExtraBits: extra bits for length codes
        let lbits: i32 = if litlen < 11 { 0 }
            else if litlen < 19 { 1 }
            else if litlen < 35 { 2 }
            else if litlen < 67 { 3 }
            else if litlen < 131 { 4 }
            else { 5 };
        
        // ZopfliGetLengthSymbol: symbol for length
        let lsym: i32 = if litlen < 11 { (litlen + 254) as i32 }
            else if litlen < 19 { (262 + (litlen - 11) / 2) as i32 }
            else if litlen < 35 { (266 + (litlen - 19) / 4) as i32 }
            else if litlen < 67 { (270 + (litlen - 35) / 8) as i32 }
            else if litlen < 131 { (274 + (litlen - 67) / 16) as i32 }
            else if litlen < 258 { (278 + (litlen - 131) / 32) as i32 }
            else { 285 };
        
        let mut cost: i32 = 0;
        if lsym <= 279 {
            cost += 7;
        } else {
            cost += 8;
        }
        cost += 5;
        return (cost + dbits + lbits) as f64;
    }
}

fn GetCostStat(litlen: u32, dist: u32, context: *mut std::ffi::c_void) -> f64 {
    let stats = context as *mut crate::types::SymbolStats;
    if dist == 0 {
        unsafe { (*stats).ll_symbols[litlen as usize] }
    } else {
        // These are inline/macro functions from zopfli_bin.h, implement locally
        // ZopfliGetLengthSymbol
        let lsym = if litlen <= 10 { litlen + 254 }
            else if litlen <= 18 { ((litlen - 3) / 2) + 261 }
            else if litlen <= 34 { ((litlen - 3) / 4) + 265 }
            else if litlen <= 66 { ((litlen - 3) / 8) + 269 }
            else if litlen <= 130 { ((litlen - 3) / 16) + 273 }
            else if litlen <= 257 { ((litlen - 3) / 32) + 277 }
            else { 285 };
        
        // ZopfliGetLengthExtraBits
        let lbits = if litlen < 3 { 0 }
            else if litlen < 11 { 0 }
            else if litlen < 19 { 1 }
            else if litlen < 35 { 2 }
            else if litlen < 67 { 3 }
            else if litlen < 131 { 4 }
            else { 5 };
        
        // ZopfliGetDistSymbol
        let dsym = if dist <= 4 { dist - 1 }
            else if dist <= 6 { 4 + ((dist - 5) / 1) }
            else if dist <= 8 { 6 + ((dist - 7) / 1) }
            else if dist <= 12 { 8 + ((dist - 9) / 2) }
            else if dist <= 16 { 10 + ((dist - 13) / 2) }
            else if dist <= 24 { 12 + ((dist - 17) / 4) }
            else if dist <= 32 { 14 + ((dist - 25) / 4) }
            else if dist <= 48 { 16 + ((dist - 33) / 8) }
            else if dist <= 64 { 18 + ((dist - 49) / 8) }
            else if dist <= 96 { 20 + ((dist - 65) / 16) }
            else if dist <= 128 { 22 + ((dist - 97) / 16) }
            else if dist <= 192 { 24 + ((dist - 129) / 32) }
            else if dist <= 256 { 26 + ((dist - 193) / 32) }
            else if dist <= 384 { 28 + ((dist - 257) / 64) }
            else if dist <= 512 { 30 + ((dist - 385) / 64) }
            else if dist <= 768 { 32 + ((dist - 513) / 128) }
            else if dist <= 1024 { 34 + ((dist - 769) / 128) }
            else if dist <= 1536 { 36 + ((dist - 1025) / 256) }
            else if dist <= 2048 { 38 + ((dist - 1537) / 256) }
            else if dist <= 3072 { 40 + ((dist - 2049) / 512) }
            else if dist <= 4096 { 42 + ((dist - 3073) / 512) }
            else if dist <= 6144 { 44 + ((dist - 4097) / 1024) }
            else if dist <= 8192 { 46 + ((dist - 6145) / 1024) }
            else if dist <= 12288 { 48 + ((dist - 8193) / 2048) }
            else if dist <= 16384 { 50 + ((dist - 12289) / 2048) }
            else if dist <= 24576 { 52 + ((dist - 16385) / 4096) }
            else { 54 + ((dist - 24577) / 4096) };
        
        // ZopfliGetDistExtraBits
        let dbits = if dist < 5 { 0 }
            else if dist < 9 { 1 }
            else if dist < 17 { 2 }
            else if dist < 33 { 3 }
            else if dist < 65 { 4 }
            else if dist < 129 { 5 }
            else if dist < 257 { 6 }
            else if dist < 513 { 7 }
            else if dist < 1025 { 8 }
            else if dist < 2049 { 9 }
            else if dist < 4097 { 10 }
            else if dist < 8193 { 11 }
            else if dist < 16385 { 12 }
            else { 13 };
        
        unsafe {
            (lbits as f64) + (dbits as f64) + (*stats).ll_symbols[lsym as usize] + (*stats).d_symbols[dsym as usize]
        }
    }
}

fn GetCostModelMinCost(costmodel: *mut crate::types::CostModelFun, costcontext: *mut std::ffi::c_void) -> f64 {
    let mut mincost: f64;
    let mut bestlength: i32 = 0;
    let mut bestdist: i32 = 0;
    let mut i: i32;

    static DSYMBOLS: [i32; 30] = [
        1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513,
        769, 1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577
    ];

    mincost = 1e30;
    i = 3;
    while i < 259 {
        let func = unsafe { *costmodel };
        if let Some(f) = func {
            let c = unsafe { f(i as u32, 1, costcontext) };
            if c < mincost {
                bestlength = i;
                mincost = c;
            }
        }
        i += 1;
    }

    mincost = 1e30;
    i = 0;
    while i < 30 {
        let func = unsafe { *costmodel };
        if let Some(f) = func {
            let c = unsafe { f(3, DSYMBOLS[i as usize] as u32, costcontext) };
            if c < mincost {
                bestdist = DSYMBOLS[i as usize];
                mincost = c;
            }
        }
        i += 1;
    }

    let func = unsafe { *costmodel };
    if let Some(f) = func {
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
    let mut i: usize;
    let mut k: usize;
    let mut kend: usize;
    let mut leng: u16 = 0;
    let mut dist: u16 = 0;
    let mut sublen: [u16; 259] = [0; 259];
    let windowstart = if instart > 32768 { instart - 32768 } else { 0 };
    let mincost = crate::src_squeeze::GetCostModelMinCost(costmodel, costcontext);
    let mincostaddcostj: f64;

    if instart == inend {
        return 0.0;
    }

    crate::src_hash::ZopfliResetHash(32768 as crate::types::size_t, h);
    crate::src_hash::ZopfliWarmupHash(r#in, windowstart as crate::types::size_t, inend as crate::types::size_t, h);
    i = windowstart;
    while i < instart {
        crate::src_hash::ZopfliUpdateHash(r#in, i as crate::types::size_t, inend as crate::types::size_t, h);
        i += 1;
    }

    unsafe {
        for idx in 1..blocksize + 1 {
            *costs.add(idx) = 1e30f32;
        }
        *costs.add(0) = 0.0f32;
        *length_array.add(0) = 0;
    }

    i = instart;
    while i < inend {
        let mut j = i - instart;
        crate::src_hash::ZopfliUpdateHash(r#in, i as crate::types::size_t, inend as crate::types::size_t, h);

        let same_ptr = unsafe {
            crate::compat::c2r_field_ptr_ZopfliHash__same(h as *mut ::core::ffi::c_void)
        };
        let same_arr = same_ptr as *mut u16;
        let same_i = unsafe { *same_arr.add(i & (32768 - 1)) };
        let same_i_minus_258 = unsafe { *same_arr.add((i.wrapping_sub(258)) & (32768 - 1)) };

        if same_i > 258 * 2
            && i > instart + 258 + 1
            && i + 258 * 2 + 1 < inend
            && same_i_minus_258 > 258
        {
            let costmodel_fn = unsafe { *costmodel };
            let symbolcost = if let Some(f) = costmodel_fn {
                unsafe { f(258, 1, costcontext) }
            } else {
                0.0
            };

            for _ in 0..258 {
                unsafe {
                    *costs.add(j + 258) = *costs.add(j) + symbolcost as f32;
                    *length_array.add(j + 258) = 258;
                }
                i += 1;
                j += 1;
                crate::src_hash::ZopfliUpdateHash(r#in, i as crate::types::size_t, inend as crate::types::size_t, h);
            }
        }

        crate::src_lz77::ZopfliFindLongestMatch(
            s,
            h as *const crate::types::ZopfliHash,
            r#in,
            i as crate::types::size_t,
            inend as crate::types::size_t,
            258 as crate::types::size_t,
            sublen.as_mut_ptr(),
            &mut dist,
            &mut leng,
        );

        if i + 1 <= inend {
            let costmodel_fn = unsafe { *costmodel };
            let in_i = unsafe { *r#in.add(i) };
            let newCost = if let Some(f) = costmodel_fn {
                unsafe { f(in_i as u32, 0, costcontext) }
            } else {
                0.0
            } + unsafe { *costs.add(j) } as f64;
            
            if newCost < unsafe { *costs.add(j + 1) } as f64 {
                unsafe {
                    *costs.add(j + 1) = newCost as f32;
                    *length_array.add(j + 1) = 1;
                }
            }
        }

        kend = crate::src_squeeze::zopfli_min(leng as usize, inend - i);
        let mincostaddcostj_local = mincost + unsafe { *costs.add(j) } as f64;
        k = 3;
        while k <= kend {
            if unsafe { *costs.add(j + k) } as f64 <= mincostaddcostj_local {
                k += 1;
                continue;
            }

            let costmodel_fn = unsafe { *costmodel };
            let newCost = if let Some(f) = costmodel_fn {
                unsafe { f(k as u32, sublen[k] as u32, costcontext) }
            } else {
                0.0
            } + unsafe { *costs.add(j) } as f64;

            if newCost < unsafe { *costs.add(j + k) } as f64 {
                unsafe {
                    *costs.add(j + k) = newCost as f32;
                    *length_array.add(j + k) = k as u16;
                }
            }
            k += 1;
        }
        i += 1;
    }

    let result = unsafe { *costs.add(blocksize) } as f64;
    result
}

fn TraceBackwards(size: usize, length_array: *const u16, path: *mut *mut u16, pathsize: *mut usize) {
    let mut index = size;
    if size == 0 {
        return;
    }
    
    unsafe {
        loop {
            // ZOPFLI_APPEND_DATA macro expansion
            if ((*pathsize) & ((*pathsize).wrapping_sub(1))) == 0 {
                if *pathsize == 0 {
                    *path = libc::malloc(std::mem::size_of::<u16>()) as *mut u16;
                } else {
                    *path = libc::realloc(
                        *path as *mut libc::c_void,
                        (*pathsize * 2 * std::mem::size_of::<u16>()) as usize
                    ) as *mut u16;
                }
            }
            *(*path).add(*pathsize) = *length_array.add(index);
            *pathsize += 1;
            
            // Assertions (debug checks, we skip the actual assert_fail calls)
            debug_assert!(*length_array.add(index) as usize <= index);
            debug_assert!(*length_array.add(index) <= 258);
            debug_assert!(*length_array.add(index) != 0);
            
            index -= *length_array.add(index) as usize;
            if index == 0 {
                break;
            }
        }
        
        // Reverse the path array
        let half = *pathsize / 2;
        for i in 0..half {
            let temp = *(*path).add(i);
            *(*path).add(i) = *(*path).add(*pathsize - i - 1);
            *(*path).add(*pathsize - i - 1) = temp;
        }
    }
}

fn FollowPath(s: *mut crate::types::ZopfliBlockState, r#in: *const u8, instart: usize, inend: usize, path: *mut u16, pathsize: usize, store: *mut crate::types::ZopfliLZ77Store, h: *mut crate::types::ZopfliHash) {
    let windowstart: usize = if instart > 32768 { instart - 32768 } else { 0 };
    let mut _total_length_test: usize = 0;

    if instart == inend {
        return;
    }

    crate::src_hash::ZopfliResetHash(32768 as crate::types::size_t, h);
    crate::src_hash::ZopfliWarmupHash(r#in, windowstart as crate::types::size_t, inend as crate::types::size_t, h);
    
    for i in windowstart..instart {
        crate::src_hash::ZopfliUpdateHash(r#in, i as crate::types::size_t, inend as crate::types::size_t, h);
    }

    let mut pos: usize = instart;
    for i in 0..pathsize {
        let mut length: u16 = unsafe { *path.add(i) };
        let mut dummy_length: u16 = 0;
        let mut dist: u16 = 0;

        assert!(pos < inend);

        crate::src_hash::ZopfliUpdateHash(r#in, pos as crate::types::size_t, inend as crate::types::size_t, h);

        if length >= 3 {
            crate::src_lz77::ZopfliFindLongestMatch(
                s,
                h as *const crate::types::ZopfliHash,
                r#in,
                pos as crate::types::size_t,
                inend as crate::types::size_t,
                length as crate::types::size_t,
                std::ptr::null_mut(),
                &mut dist,
                &mut dummy_length,
            );
            assert!(!(dummy_length != length && length > 2 && dummy_length > 2));
            crate::src_lz77::ZopfliVerifyLenDist(r#in, inend as crate::types::size_t, pos as crate::types::size_t, dist, length);
            crate::src_lz77::ZopfliStoreLitLenDist(length, dist, pos as crate::types::size_t, store);
            _total_length_test += length as usize;
        } else {
            length = 1;
            let lit = unsafe { *r#in.add(pos) };
            crate::src_lz77::ZopfliStoreLitLenDist(lit as u16, 0, pos as crate::types::size_t, store);
            _total_length_test += 1;
        }

        assert!(pos + (length as usize) <= inend);
        for j in 1..(length as usize) {
            crate::src_hash::ZopfliUpdateHash(r#in, (pos + j) as crate::types::size_t, inend as crate::types::size_t, h);
        }

        pos += length as usize;
    }
}

fn CalculateStatistics(stats: *mut crate::types::SymbolStats) {
    unsafe {
        crate::src_tree::ZopfliCalculateEntropy(
            (*stats).litlens.as_ptr() as *const crate::types::size_t,
            288 as crate::types::size_t,
            (*stats).ll_symbols.as_mut_ptr(),
        );
        crate::src_tree::ZopfliCalculateEntropy(
            (*stats).dists.as_ptr() as *const crate::types::size_t,
            32 as crate::types::size_t,
            (*stats).d_symbols.as_mut_ptr(),
        );
    }
}

fn GetStatistics(store: *const crate::types::ZopfliLZ77Store, stats: *mut crate::types::SymbolStats) {
    unsafe {
        let size_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(store as *mut ::core::ffi::c_void);
        let store_size = *(size_ptr as *const usize);
        
        let dists_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__dists(store as *mut ::core::ffi::c_void);
        let store_dists = *(dists_ptr as *const *mut u16);
        
        let litlens_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__litlens(store as *mut ::core::ffi::c_void);
        let store_litlens = *(litlens_ptr as *const *mut u16);
        
        let mut i: usize = 0;
        while i < store_size {
            let dist = *store_dists.add(i);
            let litlen = *store_litlens.add(i);
            
            if dist == 0 {
                (*stats).litlens[litlen as usize] += 1;
            } else {
                // ZopfliGetLengthSymbol: length 3-258 maps to symbols 257-285
                let length = litlen as i32;
                let len_sym: i32 = if length < 3 {
                    257
                } else if length < 11 {
                    257 + length - 3
                } else if length < 19 {
                    265 + (length - 11) / 2
                } else if length < 35 {
                    269 + (length - 19) / 4
                } else if length < 67 {
                    273 + (length - 35) / 8
                } else if length < 131 {
                    277 + (length - 67) / 16
                } else if length < 258 {
                    281 + (length - 131) / 32
                } else {
                    285
                };
                (*stats).litlens[len_sym as usize] += 1;
                
                // ZopfliGetDistSymbol: distance 1-32768 maps to symbols 0-29
                let distance = dist as i32;
                let dist_sym: i32 = if distance < 5 {
                    distance - 1
                } else {
                    let mut d = distance - 1;
                    let mut r = 0i32;
                    while d >= 4 {
                        d >>= 1;
                        r += 2;
                    }
                    r + d
                };
                (*stats).dists[dist_sym as usize] += 1;
            }
            i += 1;
        }
        (*stats).litlens[256] = 1;
        
        CalculateStatistics(stats);
    }
}

fn LZ77OptimalRun(s: *mut crate::types::ZopfliBlockState, r#in: *const u8, instart: usize, inend: usize, path: *mut *mut u16, pathsize: *mut usize, length_array: *mut u16, costmodel: Option<crate::types::CostModelFun>, costcontext: *mut std::ffi::c_void, store: *mut crate::types::ZopfliLZ77Store, h: *mut crate::types::ZopfliHash, costs: *mut f32) -> f64 {
    let costmodel_ptr: *mut crate::types::CostModelFun = match costmodel {
        Some(f) => {
            let boxed = Box::new(f);
            Box::into_raw(boxed)
        }
        None => std::ptr::null_mut(),
    };
    
    let cost = crate::src_squeeze::GetBestLengths(
        s, r#in, instart, inend, costmodel_ptr, costcontext, length_array, h, costs
    );
    
    if !costmodel_ptr.is_null() {
        unsafe { let _ = Box::from_raw(costmodel_ptr); }
    }
    
    unsafe {
        libc::free(*path as *mut std::ffi::c_void);
        *path = std::ptr::null_mut();
        *pathsize = 0;
    }
    
    crate::src_squeeze::TraceBackwards(inend - instart, length_array as *const u16, path, pathsize);
    
    unsafe {
        crate::src_squeeze::FollowPath(s, r#in, instart, inend, *path, *pathsize, store, h);
    }
    
    debug_assert!(cost < 1e30f64);
    
    cost
}

pub extern "C" fn ZopfliLZ77Optimal(s: *mut crate::types::ZopfliBlockState, in_: *const ::core::ffi::c_uchar, instart: crate::types::size_t, inend: crate::types::size_t, numiterations: ::core::ffi::c_int, store: *mut crate::types::ZopfliLZ77Store) {
    unsafe {
        let blocksize = inend - instart;
        let length_array = libc::malloc(std::mem::size_of::<u16>() * (blocksize + 1)) as *mut u16;
        let mut path: *mut u16 = std::ptr::null_mut();
        let mut pathsize: usize = 0;
        let mut currentstore: crate::types::ZopfliLZ77Store = std::mem::zeroed();
        let mut hash: crate::types::ZopfliHash = std::mem::zeroed();
        let h: *mut crate::types::ZopfliHash = &mut hash;
        let mut stats: crate::types::SymbolStats = std::mem::zeroed();
        let mut beststats: crate::types::SymbolStats = std::mem::zeroed();
        let mut laststats: crate::types::SymbolStats = std::mem::zeroed();
        let costs = libc::malloc(std::mem::size_of::<f32>() * (blocksize + 1)) as *mut f32;
        let mut cost: f64;
        let mut bestcost: f64 = 1e30;
        let mut lastcost: f64 = 0.0;
        let mut ran_state: crate::types::RanState = std::mem::zeroed();
        let mut lastrandomstep: i32 = -1;

        if costs.is_null() {
            libc::exit(-1);
        }
        if length_array.is_null() {
            libc::exit(-1);
        }

        crate::src_squeeze::InitRanState(&mut ran_state);
        crate::src_squeeze::InitStats(&mut stats);
        crate::src_lz77::ZopfliInitLZ77Store(in_, &mut currentstore);
        crate::src_hash::ZopfliAllocHash(32768, h);

        crate::src_lz77::ZopfliLZ77Greedy(s, in_, instart, inend, &mut currentstore, h);
        crate::src_squeeze::GetStatistics(&currentstore, &mut stats);

        for i in 0..numiterations {
            crate::src_lz77::ZopfliCleanLZ77Store(&mut currentstore);
            crate::src_lz77::ZopfliInitLZ77Store(in_, &mut currentstore);
            let costmodel_fn: crate::types::CostModelFun = std::mem::transmute(crate::src_squeeze::GetCostStat as usize);
            let _ = crate::src_squeeze::LZ77OptimalRun(
                s, in_ as *const u8, instart, inend,
                &mut path, &mut pathsize,
                length_array, Some(costmodel_fn),
                &mut stats as *mut _ as *mut std::ffi::c_void,
                &mut currentstore, h, costs
            );
            
            let currentstore_size = *(crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(&mut currentstore as *mut _ as *mut ::core::ffi::c_void) as *const crate::types::size_t);
            cost = crate::src_deflate::ZopfliCalculateBlockSize(&currentstore, 0, currentstore_size, 2);
            
            let options_ptr = crate::compat::c2r_field_ptr_ZopfliBlockState__options(s as *mut ::core::ffi::c_void) as *const *const crate::types::ZopfliOptions;
            let options = *options_ptr;
            let verbose_more = *(crate::compat::c2r_field_ptr_ZopfliOptions__verbose_more(options as *mut ::core::ffi::c_void) as *const ::core::ffi::c_int);
            let verbose = *(crate::compat::c2r_field_ptr_ZopfliOptions__verbose(options as *mut ::core::ffi::c_void) as *const ::core::ffi::c_int);
            
            if verbose_more != 0 || (verbose != 0 && cost < bestcost) {
                extern "C" {
                    static stderr: *mut libc::FILE;
                }
                libc::fprintf(
                    stderr,
                    b"Iteration %d: %d bit\n\0".as_ptr() as *const i8,
                    i,
                    cost as i32
                );
            }
            
            if cost < bestcost {
                crate::src_lz77::ZopfliCopyLZ77Store(&currentstore, store);
                crate::src_squeeze::CopyStats(&mut stats, &mut beststats);
                bestcost = cost;
            }
            crate::src_squeeze::CopyStats(&mut stats, &mut laststats);
            crate::src_squeeze::ClearStatFreqs(&mut stats);
            crate::src_squeeze::GetStatistics(&currentstore, &mut stats);
            
            if lastrandomstep != -1 {
                crate::src_squeeze::AddWeighedStatFreqs(&stats, 1.0, &laststats, 0.5, &mut stats);
                crate::src_squeeze::CalculateStatistics(&mut stats);
            }
            if i > 5 && cost == lastcost {
                crate::src_squeeze::CopyStats(&mut beststats, &mut stats);
                crate::src_squeeze::RandomizeStatFreqs(&mut ran_state, &mut stats);
                crate::src_squeeze::CalculateStatistics(&mut stats);
                lastrandomstep = i;
            }
            lastcost = cost;
        }

        libc::free(length_array as *mut ::core::ffi::c_void);
        libc::free(path as *mut ::core::ffi::c_void);
        libc::free(costs as *mut ::core::ffi::c_void);
        crate::src_lz77::ZopfliCleanLZ77Store(&mut currentstore);
        crate::src_hash::ZopfliCleanHash(h);
    }
}

pub extern "C" fn ZopfliLZ77OptimalFixed(s: *mut crate::types::ZopfliBlockState, in_: *const ::core::ffi::c_uchar, instart: crate::types::size_t, inend: crate::types::size_t, store: *mut crate::types::ZopfliLZ77Store) {
    unsafe {
        let blocksize = inend - instart;
        let length_array = libc::malloc(std::mem::size_of::<u16>() * (blocksize + 1)) as *mut u16;
        let mut path: *mut u16 = std::ptr::null_mut();
        let mut pathsize: usize = 0;
        let mut hash: crate::types::ZopfliHash = std::mem::zeroed();
        let h: *mut crate::types::ZopfliHash = &mut hash;
        let costs = libc::malloc(std::mem::size_of::<f32>() * (blocksize + 1)) as *mut f32;

        if costs.is_null() {
            libc::exit(-1);
        }
        if length_array.is_null() {
            libc::exit(-1);
        }

        crate::src_hash::ZopfliAllocHash(32768, h);

        let blockstart_ptr = crate::compat::c2r_field_ptr_ZopfliBlockState__blockstart(s as *mut ::core::ffi::c_void);
        *(blockstart_ptr as *mut crate::types::size_t) = instart;
        
        let blockend_ptr = crate::compat::c2r_field_ptr_ZopfliBlockState__blockend(s as *mut ::core::ffi::c_void);
        *(blockend_ptr as *mut crate::types::size_t) = inend;

        let costmodel_fn: crate::types::CostModelFun = std::mem::transmute(GetCostFixed as usize);

        let _cost = crate::src_squeeze::LZ77OptimalRun(
            s,
            in_,
            instart,
            inend,
            &mut path,
            &mut pathsize,
            length_array,
            Some(costmodel_fn),
            std::ptr::null_mut(),
            store,
            h,
            costs,
        );

        libc::free(length_array as *mut ::core::ffi::c_void);
        libc::free(path as *mut ::core::ffi::c_void);
        libc::free(costs as *mut ::core::ffi::c_void);
        crate::src_hash::ZopfliCleanHash(h);
    }
}
