//! Module: src_deflate
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

fn AddBit(bit: c_int, bp: *mut u8, out: *mut *mut u8, outsize: *mut usize) {
    unsafe {
        if *bp == 0 {
            let outsize_val = *outsize;
            if outsize_val & (outsize_val - 1) == 0 {
                let new_size = if outsize_val == 0 {
                    1
                } else {
                    outsize_val * 2
                };
                let new_ptr = if outsize_val == 0 {
                    libc::malloc(new_size) as *mut u8
                } else {
                    libc::realloc(*out as *mut libc::c_void, new_size) as *mut u8
                };
                *out = new_ptr;
            }
            let out_ptr = *out;
            let outsize_ptr = &mut *outsize;
            *out_ptr.add(*outsize_ptr) = 0;
            *outsize_ptr += 1;
        }
        let outsize_val = *outsize;
        let bp_val = *bp;
        let out_ptr = *out;
        *out_ptr.add(outsize_val - 1) |= (bit as u8) << bp_val;
        *bp = (bp_val + 1) & 7;
    }
}

fn AddBits(symbol: u32, length: u32, bp: *const u8, out: *mut *mut u8, outsize: *mut usize) {
    let mut i: u32 = 0;
    while i < length {
        let bit = (symbol >> i) & 1;
        unsafe {
            if *bp == 0 {
                if (*outsize) & ((*outsize).wrapping_sub(1)) == 0 {
                    let new_size = if *outsize == 0 { 1 } else { (*outsize) * 2 };
                    let new_ptr = libc::realloc((*out) as *mut libc::c_void, new_size) as *mut u8;
                    if new_ptr.is_null() {
                        return;
                    }
                    *out = new_ptr;
                }
                (*out).offset(*outsize as isize).write(0);
                *outsize += 1;
            }
            let idx = *outsize - 1;
            let current = *(*out).offset(idx as isize);
            let val = current | ((bit as u8) << *bp);
            *(*out).offset(idx as isize) = val;
            *(bp as *mut u8) = (*bp + 1) & 7;
        }
        i += 1;
    }
}

fn AddHuffmanBits(symbol: u32, length: u32, bp: *mut u8, out: *mut *mut u8, outsize: *mut usize) {
    for i in 0..length {
        let bit = (symbol >> (length - i - 1)) & 1;
        unsafe {
            if *bp == 0 {
                let outsize_val = *outsize;
                if outsize_val & (outsize_val.wrapping_sub(1)) == 0 {
                    let new_size = if outsize_val == 0 { 1 } else { outsize_val * 2 };
                    let new_out = libc::realloc(*out as *mut libc::c_void, new_size) as *mut u8;
                    if !new_out.is_null() {
                        *out = new_out;
                    }
                }
                let outsize_val = *outsize;
                let out_ptr = *out;
                if !out_ptr.is_null() && outsize_val < libc::size_t::MAX {
                    *out_ptr.add(outsize_val) = 0;
                }
                *outsize = outsize_val.wrapping_add(1);
            }
            let outsize_val = *outsize;
            let out_ptr = *out;
            if !out_ptr.is_null() && outsize_val > 0 {
                let last_idx = outsize_val - 1;
                *out_ptr.add(last_idx) |= (bit as u8) << *bp;
            }
            *bp = (*bp + 1) & 7;
        }
    }
}

fn PatchDistanceCodesForBuggyDecoders(d_lengths: *mut u32) {
    let mut num_dist_codes = 0;
    for i in 0..30 {
        unsafe {
            if *d_lengths.add(i) != 0 {
                num_dist_codes += 1;
            }
        }
        if num_dist_codes >= 2 {
            return;
        }
    }
    if num_dist_codes == 0 {
        unsafe {
            *d_lengths = 1;
            *d_lengths.add(1) = 1;
        }
    } else if num_dist_codes == 1 {
        unsafe {
            let idx = if *d_lengths != 0 { 1 } else { 0 };
            *d_lengths.add(idx) = 1;
        }
    }
}

fn EncodeTree(ll_lengths: *const u32, d_lengths: *const u32, use_16: i32, use_17: i32, use_18: i32, bp: *mut u8, out: *mut *mut u8, outsize: *mut usize) -> usize {
    let mut lld_total: u32;
    let mut rle: *mut u32 = std::ptr::null_mut();
    let mut rle_bits: *mut u32 = std::ptr::null_mut();
    let mut rle_size: usize = 0;
    let mut rle_bits_size: usize = 0;
    let mut hlit: u32 = 29;
    let mut hdist: u32 = 29;
    let mut hclen: u32;
    let mut hlit2: u32;
    let mut i: usize;
    let mut j: usize;
    let mut clcounts: [usize; 19] = [0; 19];
    let mut clcl: [u32; 19] = [0; 19];
    let mut clsymbols: [u32; 19] = [0; 19];
    let order: [usize; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];
    let size_only = out.is_null();
    let mut result_size: usize = 0;

    while hlit > 0 && unsafe { *ll_lengths.offset((257 + hlit - 1) as isize) } == 0 {
        hlit -= 1;
    }
    while hdist > 0 && unsafe { *d_lengths.offset((1 + hdist - 1) as isize) } == 0 {
        hdist -= 1;
    }
    hlit2 = hlit + 257;
    lld_total = hlit2 + hdist + 1;

    i = 0;
    while i < lld_total as usize {
        let symbol: u32 = if i < hlit2 as usize {
            unsafe { *ll_lengths.offset(i as isize) }
        } else {
            unsafe { *d_lengths.offset((i - hlit2 as usize) as isize) }
        };
        let mut count: u32 = 1;
        if use_16 != 0 || (symbol == 0 && (use_17 != 0 || use_18 != 0)) {
            j = i + 1;
            while j < lld_total as usize && symbol == (if j < hlit2 as usize {
                unsafe { *ll_lengths.offset(j as isize) }
            } else {
                unsafe { *d_lengths.offset((j - hlit2 as usize) as isize) }
            }) {
                count += 1;
                j += 1;
            }
        }
        i += (count - 1) as usize;

        if symbol == 0 && count >= 3 {
            if use_18 != 0 {
                while count >= 11 {
                    let count2 = if count > 138 { 138 } else { count };
                    if !size_only {
                        if (rle_size & (rle_size - 1)) == 0 {
                            rle = unsafe {
                                if rle_size == 0 {
                                    libc::malloc(std::mem::size_of::<u32>() as usize) as *mut u32
                                } else {
                                    libc::realloc(rle as *mut libc::c_void, rle_size * 2 * std::mem::size_of::<u32>()) as *mut u32
                                }
                            };
                        }
                        unsafe { *rle.offset(rle_size as isize) = 18 };
                        rle_size += 1;
                        if (rle_bits_size & (rle_bits_size - 1)) == 0 {
                            rle_bits = unsafe {
                                if rle_bits_size == 0 {
                                    libc::malloc(std::mem::size_of::<u32>() as usize) as *mut u32
                                } else {
                                    libc::realloc(rle_bits as *mut libc::c_void, rle_bits_size * 2 * std::mem::size_of::<u32>()) as *mut u32
                                }
                            };
                        }
                        unsafe { *rle_bits.offset(rle_bits_size as isize) = count2 - 11 };
                        rle_bits_size += 1;
                    }
                    clcounts[18] += 1;
                    count -= count2;
                }
            }
            if use_17 != 0 {
                while count >= 3 {
                    let count2 = if count > 10 { 10 } else { count };
                    if !size_only {
                        if (rle_size & (rle_size - 1)) == 0 {
                            rle = unsafe {
                                if rle_size == 0 {
                                    libc::malloc(std::mem::size_of::<u32>() as usize) as *mut u32
                                } else {
                                    libc::realloc(rle as *mut libc::c_void, rle_size * 2 * std::mem::size_of::<u32>()) as *mut u32
                                }
                            };
                        }
                        unsafe { *rle.offset(rle_size as isize) = 17 };
                        rle_size += 1;
                        if (rle_bits_size & (rle_bits_size - 1)) == 0 {
                            rle_bits = unsafe {
                                if rle_bits_size == 0 {
                                    libc::malloc(std::mem::size_of::<u32>() as usize) as *mut u32
                                } else {
                                    libc::realloc(rle_bits as *mut libc::c_void, rle_bits_size * 2 * std::mem::size_of::<u32>()) as *mut u32
                                }
                            };
                        }
                        unsafe { *rle_bits.offset(rle_bits_size as isize) = count2 - 3 };
                        rle_bits_size += 1;
                    }
                    clcounts[17] += 1;
                    count -= count2;
                }
            }
        }

        if use_16 != 0 && count >= 4 {
            count -= 1;
            clcounts[symbol as usize] += 1;
            if !size_only {
                if (rle_size & (rle_size - 1)) == 0 {
                    rle = unsafe {
                        if rle_size == 0 {
                            libc::malloc(std::mem::size_of::<u32>() as usize) as *mut u32
                        } else {
                            libc::realloc(rle as *mut libc::c_void, rle_size * 2 * std::mem::size_of::<u32>()) as *mut u32
                        }
                    };
                }
                unsafe { *rle.offset(rle_size as isize) = symbol };
                rle_size += 1;
                if (rle_bits_size & (rle_bits_size - 1)) == 0 {
                    rle_bits = unsafe {
                        if rle_bits_size == 0 {
                            libc::malloc(std::mem::size_of::<u32>() as usize) as *mut u32
                        } else {
                            libc::realloc(rle_bits as *mut libc::c_void, rle_bits_size * 2 * std::mem::size_of::<u32>()) as *mut u32
                        }
                    };
                }
                unsafe { *rle_bits.offset(rle_bits_size as isize) = 0 };
                rle_bits_size += 1;
            }
            while count >= 3 {
                let count2 = if count > 6 { 6 } else { count };
                if !size_only {
                    if (rle_size & (rle_size - 1)) == 0 {
                        rle = unsafe {
                            if rle_size == 0 {
                                libc::malloc(std::mem::size_of::<u32>() as usize) as *mut u32
                            } else {
                                libc::realloc(rle as *mut libc::c_void, rle_size * 2 * std::mem::size_of::<u32>()) as *mut u32
                            }
                        };
                    }
                    unsafe { *rle.offset(rle_size as isize) = 16 };
                    rle_size += 1;
                    if (rle_bits_size & (rle_bits_size - 1)) == 0 {
                        rle_bits = unsafe {
                            if rle_bits_size == 0 {
                                libc::malloc(std::mem::size_of::<u32>() as usize) as *mut u32
                            } else {
                                libc::realloc(rle_bits as *mut libc::c_void, rle_bits_size * 2 * std::mem::size_of::<u32>()) as *mut u32
                            }
                        };
                    }
                    unsafe { *rle_bits.offset(rle_bits_size as isize) = count2 - 3 };
                    rle_bits_size += 1;
                }
                clcounts[16] += 1;
                count -= count2;
            }
        }

        clcounts[symbol as usize] += count as usize;
        while count > 0 {
            if !size_only {
                if (rle_size & (rle_size - 1)) == 0 {
                    rle = unsafe {
                        if rle_size == 0 {
                            libc::malloc(std::mem::size_of::<u32>() as usize) as *mut u32
                        } else {
                            libc::realloc(rle as *mut libc::c_void, rle_size * 2 * std::mem::size_of::<u32>()) as *mut u32
                        }
                    };
                }
                unsafe { *rle.offset(rle_size as isize) = symbol };
                rle_size += 1;
                if (rle_bits_size & (rle_bits_size - 1)) == 0 {
                    rle_bits = unsafe {
                        if rle_bits_size == 0 {
                            libc::malloc(std::mem::size_of::<u32>() as usize) as *mut u32
                        } else {
                            libc::realloc(rle_bits as *mut libc::c_void, rle_bits_size * 2 * std::mem::size_of::<u32>()) as *mut u32
                        }
                    };
                }
                unsafe { *rle_bits.offset(rle_bits_size as isize) = 0 };
                rle_bits_size += 1;
            }
            count -= 1;
        }
        i += 1;
    }

    crate::src_tree::ZopfliCalculateBitLengths(clcounts.as_ptr() as *const crate::types::size_t, 19, 7, clcl.as_mut_ptr());
    if !size_only {
        crate::src_tree::ZopfliLengthsToSymbols(clcl.as_ptr(), 19, 7, clsymbols.as_mut_ptr());
    }

    hclen = 15;
    while hclen > 0 && clcounts[order[(hclen + 4 - 1) as usize]] == 0 {
        hclen -= 1;
    }

    if !size_only {
        crate::src_deflate::AddBits(hlit, 5, bp, out, outsize);
        crate::src_deflate::AddBits(hdist, 5, bp, out, outsize);
        crate::src_deflate::AddBits(hclen, 4, bp, out, outsize);
        for i in 0..(hclen + 4) as usize {
            crate::src_deflate::AddBits(clcl[order[i]], 3, bp, out, outsize);
        }
        for i in 0..rle_size {
            let symbol = clsymbols[unsafe { *rle.offset(i as isize) } as usize];
            crate::src_deflate::AddHuffmanBits(symbol, clcl[unsafe { *rle.offset(i as isize) } as usize], bp, out, outsize);
            let rle_val = unsafe { *rle.offset(i as isize) };
            if rle_val == 16 {
                crate::src_deflate::AddBits(unsafe { *rle_bits.offset(i as isize) }, 2, bp, out, outsize);
            } else if rle_val == 17 {
                crate::src_deflate::AddBits(unsafe { *rle_bits.offset(i as isize) }, 3, bp, out, outsize);
            } else if rle_val == 18 {
                crate::src_deflate::AddBits(unsafe { *rle_bits.offset(i as isize) }, 7, bp, out, outsize);
            }
        }
    }

    result_size += 14;
    result_size += (hclen + 4) as usize * 3;
    for i in 0..19 {
        result_size += clcl[i] as usize * clcounts[i];
    }
    result_size += clcounts[16] * 2;
    result_size += clcounts[17] * 3;
    result_size += clcounts[18] * 7;

    if !rle.is_null() {
        unsafe { libc::free(rle as *mut libc::c_void) };
    }
    if !rle_bits.is_null() {
        unsafe { libc::free(rle_bits as *mut libc::c_void) };
    }

    result_size
}

fn AddDynamicTree(ll_lengths: *const u32, d_lengths: *const u32, bp: *mut u8, out: *mut *mut u8, outsize: *mut usize) {
    let mut best = 0;
    let mut bestsize = 0usize;
    for i in 0..8 {
        let size = crate::src_deflate::EncodeTree(ll_lengths, d_lengths, (i & 1) as i32, (i & 2) as i32, (i & 4) as i32, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        if bestsize == 0 || size < bestsize {
            bestsize = size;
            best = i;
        }
    }
    let _ = crate::src_deflate::EncodeTree(ll_lengths, d_lengths, (best & 1) as i32, (best & 2) as i32, (best & 4) as i32, bp, out, outsize);
}

fn CalculateTreeSize(ll_lengths: *const u32, d_lengths: *const u32) -> usize {
    let mut result: usize = 0;
    for i in 0..8 {
        let size = crate::src_deflate::EncodeTree(
            ll_lengths,
            d_lengths,
            (i & 1) as i32,
            (i & 2) as i32,
            (i & 4) as i32,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        if result == 0 || size < result {
            result = size;
        }
    }
    result
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_deflate_8
// c_function: AddLZ77Data
// rust_file: src_deflate.rs
// rust_signature: fn AddLZ77Data(lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize, expected_data_size: usize, ll_symbols: *const u32, ll_lengths: *const u32, d_symbols: *const u32, d_lengths: *const u32, bp: *mut u8,...
// c_first_line: static void AddLZ77Data(const ZopfliLZ77Store* lz77,
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_deflate_8/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find function, tuple struct or tuple variant `ZopfliGetLengthSymbol` in module `crate::compat`
//      --> src/src_deflate.rs:158:38
//       |
//       |                                      ^^^^^^^^^^^^^^^^^^^^^ not found in `crate::compat`
//       |
//   help: consider importing one of these functions
//       |
//       |
// =================================
fn AddLZ77Data(lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize, expected_data_size: usize, ll_symbols: *const u32, ll_lengths: *const u32, d_symbols: *const u32, d_lengths: *const u32, bp: *mut u8, out: *mut *mut u8, outsize: *mut usize) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_deflate::AddLZ77Data(lz77 as _, lstart as _, lend as _, expected_data_size as _, ll_symbols as _, ll_lengths as _, d_symbols as _, d_lengths as _, bp as _, out as _, outsize as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_deflate_8
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_deflate_8/translated_rust.rs
 * ------------------------------------------------------------
fn AddLZ77Data(lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize, expected_data_size: usize, ll_symbols: *const u32, ll_lengths: *const u32, d_symbols: *const u32, d_lengths: *const u32, bp: *mut u8, out: *mut *mut u8, outsize: *mut usize) {
    let mut testlength: usize = 0;
    let mut i = lstart;
    while i < lend {
        let dist = unsafe { *(*lz77).dists.add(i) } as u32;
        let litlen = unsafe { *(*lz77).litlens.add(i) } as u32;
        if dist == 0 {
            let _ = litlen < 256;
            let _ = unsafe { *ll_lengths.add(litlen as usize) } > 0;
            crate::src_deflate::AddHuffmanBits(
                unsafe { *ll_symbols.add(litlen as usize) },
                unsafe { *ll_lengths.add(litlen as usize) },
                bp,
                out,
                outsize,
            );
            testlength += 1;
        } else {
            let lls = crate::compat::ZopfliGetLengthSymbol(litlen as i32) as u32;
            let ds = crate::compat::ZopfliGetDistSymbol(dist as i32) as u32;
            let _ = litlen >= 3 && litlen <= 288;
            let _ = unsafe { *ll_lengths.add(lls as usize) } > 0;
            let _ = unsafe { *d_lengths.add(ds as usize) } > 0;
            crate::src_deflate::AddHuffmanBits(
                unsafe { *ll_symbols.add(lls as usize) },
                unsafe { *ll_lengths.add(lls as usize) },
                bp,
                out,
                outsize,
            );
            crate::src_deflate::AddBits(
                crate::compat::ZopfliGetLengthExtraBitsValue(litlen as i32) as u32,
                crate::compat::ZopfliGetLengthExtraBits(litlen as i32) as u32,
                bp as *const u8,
                out,
                outsize,
            );
            crate::src_deflate::AddHuffmanBits(
                unsafe { *d_symbols.add(ds as usize) },
                unsafe { *d_lengths.add(ds as usize) },
                bp,
                out,
                outsize,
            );
            crate::src_deflate::AddBits(
                crate::compat::ZopfliGetDistExtraBitsValue(dist as i32) as u32,
                crate::compat::ZopfliGetDistExtraBits(dist as i32) as u32,
                bp as *const u8,
                out,
                outsize,
            );
            testlength += litlen as usize;
        }
        i += 1;
    }
    let _ = expected_data_size == 0 || testlength == expected_data_size;
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_deflate_8
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn GetFixedTree(ll_lengths: *mut u32, d_lengths: *mut u32) {
    unsafe {
        let mut i: usize = 0;
        while i < 144 {
            *ll_lengths.add(i) = 8;
            i += 1;
        }
        while i < 256 {
            *ll_lengths.add(i) = 9;
            i += 1;
        }
        while i < 280 {
            *ll_lengths.add(i) = 7;
            i += 1;
        }
        while i < 288 {
            *ll_lengths.add(i) = 8;
            i += 1;
        }
        i = 0;
        while i < 32 {
            *d_lengths.add(i) = 5;
            i += 1;
        }
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_deflate_10
// c_function: CalculateBlockSymbolSizeSmall
// rust_file: src_deflate.rs
// rust_signature: fn CalculateBlockSymbolSizeSmall(ll_lengths: *const u32, d_lengths: *const u32, lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize) -> usize
// c_first_line: static size_t CalculateBlockSymbolSizeSmall(const unsigned* ll_lengths,
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_deflate_10/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find function, tuple struct or tuple variant `ZopfliGetLengthSymbol` in module `crate::compat`
//      --> src/src_deflate.rs:197:53
//       |
//       |                                                     ^^^^^^^^^^^^^^^^^^^^^ not found in `crate::compat`
//   error[E0425]: cannot find function, tuple struct or tuple variant `ZopfliGetDistSymbol` in module `crate::compat`
//      --> src/src_deflate.rs:198:52
//       |
//       |                                                    ^^^^^^^^^^^^^^^^^^^ not found in `crate::compat`
// =================================
fn CalculateBlockSymbolSizeSmall(ll_lengths: *const u32, d_lengths: *const u32, lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize) -> usize {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_deflate::CalculateBlockSymbolSizeSmall(ll_lengths as _, d_lengths as _, lz77 as _, lstart as _, lend as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_deflate_10
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_deflate_10/translated_rust.rs
 * ------------------------------------------------------------
fn CalculateBlockSymbolSizeSmall(ll_lengths: *const u32, d_lengths: *const u32, lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize) -> usize {
    let mut result: usize = 0;
    let mut i = lstart;
    while i < lend {
        let lz77_ptr = lz77 as *mut ::core::ffi::c_void;
        let size_ptr = unsafe {
            crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(lz77_ptr)
        };
        let size = unsafe { *(size_ptr as *const usize) };
        if !(i < size) {
            unsafe { libc::abort(); }
        }
        let litlens_ptr = unsafe {
            crate::compat::c2r_field_ptr_ZopfliLZ77Store__litlens(lz77_ptr)
        };
        let litlen = unsafe { *((litlens_ptr as *const u16).add(i)) };
        if !(litlen < 259) {
            unsafe { libc::abort(); }
        }
        let dists_ptr = unsafe {
            crate::compat::c2r_field_ptr_ZopfliLZ77Store__dists(lz77_ptr)
        };
        let dist = unsafe { *((dists_ptr as *const u16).add(i)) };
        if dist == 0 {
            result += unsafe { *ll_lengths.add(litlen as usize) } as usize;
        } else {
            let ll_symbol = unsafe { crate::compat::ZopfliGetLengthSymbol(litlen as i32) };
            let d_symbol = unsafe { crate::compat::ZopfliGetDistSymbol(dist as i32) };
            result += unsafe { *ll_lengths.add(ll_symbol as usize) } as usize;
            result += unsafe { *d_lengths.add(d_symbol as usize) } as usize;
            result += unsafe { crate::compat::ZopfliGetLengthSymbolExtraBits(ll_symbol) } as usize;
            result += unsafe { crate::compat::ZopfliGetDistSymbolExtraBits(d_symbol) } as usize;
        }
        i += 1;
    }
    result += unsafe { *ll_lengths.add(256) } as usize;
    result
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_deflate_10
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_deflate_11
// c_function: CalculateBlockSymbolSizeGivenCounts
// rust_file: src_deflate.rs
// rust_signature: fn CalculateBlockSymbolSizeGivenCounts(ll_counts: *const usize, d_counts: *const usize, ll_lengths: *const u32, d_lengths: *const u32, lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize) -> usize
// c_first_line: static size_t CalculateBlockSymbolSizeGivenCounts(const size_t* ll_counts,
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_deflate_11/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find function, tuple struct or tuple variant `ZopfliGetLengthSymbolExtraBits` in this scope
//      --> src/src_deflate.rs:342:28
//       |
//       |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
//       |
//   help: consider importing this function
//       |
//       |
// =================================
fn CalculateBlockSymbolSizeGivenCounts(ll_counts: *const usize, d_counts: *const usize, ll_lengths: *const u32, d_lengths: *const u32, lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize) -> usize {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_deflate::CalculateBlockSymbolSizeGivenCounts(ll_counts as _, d_counts as _, ll_lengths as _, d_lengths as _, lz77 as _, lstart as _, lend as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_deflate_11
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_deflate_11/translated_rust.rs
 * ------------------------------------------------------------
fn CalculateBlockSymbolSizeGivenCounts(ll_counts: *const usize, d_counts: *const usize, ll_lengths: *const u32, d_lengths: *const u32, lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize) -> usize {
    let mut result: usize = 0;
    if lstart + 288 * 3 > lend {
        return crate::src_deflate::CalculateBlockSymbolSizeSmall(ll_lengths, d_lengths, lz77, lstart, lend);
    } else {
        let mut i: usize = 0;
        while i < 256 {
            unsafe {
                result += (*ll_lengths.add(i) as usize) * (*ll_counts.add(i));
            }
            i += 1;
        }
        i = 257;
        while i < 286 {
            unsafe {
                result += (*ll_lengths.add(i) as usize) * (*ll_counts.add(i));
                result += (ZopfliGetLengthSymbolExtraBits(i as i32) as usize) * (*ll_counts.add(i));
            }
            i += 1;
        }
        i = 0;
        while i < 30 {
            unsafe {
                result += (*d_lengths.add(i) as usize) * (*d_counts.add(i));
                result += (ZopfliGetDistSymbolExtraBits(i as i32) as usize) * (*d_counts.add(i));
            }
            i += 1;
        }
        unsafe {
            result += *ll_lengths.add(256) as usize;
        }
        return result;
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_deflate_11
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn CalculateBlockSymbolSize(ll_lengths: *const u32, d_lengths: *const u32, lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize) -> usize {
    if lstart + 288 * 3 > lend {
        crate::src_deflate::CalculateBlockSymbolSizeSmall(ll_lengths, d_lengths, lz77, lstart, lend)
    } else {
        let mut ll_counts: [usize; 288] = [0; 288];
        let mut d_counts: [usize; 32] = [0; 32];
        crate::src_lz77::ZopfliLZ77GetHistogram(lz77, lstart as crate::types::size_t, lend as crate::types::size_t, ll_counts.as_mut_ptr() as *mut crate::types::size_t, d_counts.as_mut_ptr() as *mut crate::types::size_t);
        crate::src_deflate::CalculateBlockSymbolSizeGivenCounts(ll_counts.as_ptr(), d_counts.as_ptr(), ll_lengths, d_lengths, lz77, lstart, lend)
    }
}

fn AbsDiff(x: usize, y: usize) -> usize {
    if x > y {
        x - y
    } else {
        y - x
    }
}

pub extern "C" fn OptimizeHuffmanForRle(length: ::core::ffi::c_int, counts: *mut crate::types::size_t) {
    if length <= 0 || counts.is_null() {
        return;
    }
    let mut length = length as usize;
    unsafe {
        while length > 0 {
            if *counts.offset(length as isize - 1) != 0 {
                break;
            }
            length -= 1;
        }
    }
    if length == 0 {
        return;
    }
    let good_for_rle = unsafe { libc::malloc((length as libc::size_t).wrapping_mul(std::mem::size_of::<libc::c_int>())) } as *mut libc::c_int;
    if good_for_rle.is_null() {
        return;
    }
    unsafe {
        std::ptr::write_bytes(good_for_rle, 0, length);
    }
    unsafe {
        let mut symbol = *counts;
        let mut stride = 0;
        for i in 0..=length {
            if i == length || *counts.offset(i as isize) != symbol {
                if (symbol == 0 && stride >= 5) || (symbol != 0 && stride >= 7) {
                    for k in 0..stride {
                        *good_for_rle.offset((i - k - 1) as isize) = 1;
                    }
                }
                stride = 1;
                if i != length {
                    symbol = *counts.offset(i as isize);
                }
            } else {
                stride += 1;
            }
        }
        let mut stride = 0;
        let mut limit = *counts;
        let mut sum = 0;
        for i in 0..=length {
            if i == length || *good_for_rle.offset(i as isize) != 0 || crate::src_deflate::AbsDiff(*counts.offset(i as isize), limit) >= 4 {
                if stride >= 4 || (stride >= 3 && sum == 0) {
                    let mut count = (sum + stride / 2) / stride;
                    if count < 1 {
                        count = 1;
                    }
                    if sum == 0 {
                        count = 0;
                    }
                    for k in 0..stride {
                        *counts.offset((i - k - 1) as isize) = count as crate::types::size_t;
                    }
                }
                stride = 0;
                sum = 0;
                if i < length.wrapping_sub(3) {
                    limit = (*counts.offset(i as isize) + *counts.offset((i + 1) as isize) + *counts.offset((i + 2) as isize) + *counts.offset((i + 3) as isize) + 2) / 4;
                } else if i < length {
                    limit = *counts.offset(i as isize);
                } else {
                    limit = 0;
                }
            }
            stride += 1;
            if i != length {
                sum += *counts.offset(i as isize);
            }
        }
        libc::free(good_for_rle as *mut libc::c_void);
    }
}

fn TryOptimizeHuffmanForRle(lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize, ll_counts: *const usize, d_counts: *const usize, ll_lengths: *mut u32, d_lengths: *mut u32) -> f64 {
    let mut ll_counts2: [usize; 288] = [0; 288];
    let mut d_counts2: [usize; 32] = [0; 32];
    let mut ll_lengths2: [u32; 288] = [0; 288];
    let mut d_lengths2: [u32; 32] = [0; 32];
    let treesize;
    let datasize;
    let treesize2;
    let datasize2;

    treesize = crate::src_deflate::CalculateTreeSize(ll_lengths, d_lengths) as f64;
    datasize = crate::src_deflate::CalculateBlockSymbolSizeGivenCounts(ll_counts, d_counts, ll_lengths, d_lengths, lz77, lstart, lend) as f64;

    unsafe {
        std::ptr::copy_nonoverlapping(ll_counts, ll_counts2.as_mut_ptr(), 288);
        std::ptr::copy_nonoverlapping(d_counts, d_counts2.as_mut_ptr(), 32);
    }
    crate::src_deflate::OptimizeHuffmanForRle(288, ll_counts2.as_mut_ptr() as *mut crate::types::size_t);
    crate::src_deflate::OptimizeHuffmanForRle(32, d_counts2.as_mut_ptr() as *mut crate::types::size_t);
    crate::src_tree::ZopfliCalculateBitLengths(ll_counts2.as_ptr() as *const crate::types::size_t, 288, 15, ll_lengths2.as_mut_ptr() as *mut ::core::ffi::c_uint);
    crate::src_tree::ZopfliCalculateBitLengths(d_counts2.as_ptr() as *const crate::types::size_t, 32, 15, d_lengths2.as_mut_ptr() as *mut ::core::ffi::c_uint);
    crate::src_deflate::PatchDistanceCodesForBuggyDecoders(d_lengths2.as_mut_ptr());

    treesize2 = crate::src_deflate::CalculateTreeSize(ll_lengths2.as_ptr(), d_lengths2.as_ptr()) as f64;
    datasize2 = crate::src_deflate::CalculateBlockSymbolSizeGivenCounts(ll_counts, d_counts, ll_lengths2.as_ptr(), d_lengths2.as_ptr(), lz77, lstart, lend) as f64;

    if treesize2 + datasize2 < treesize + datasize {
        unsafe {
            std::ptr::copy_nonoverlapping(ll_lengths2.as_ptr(), ll_lengths, 288);
            std::ptr::copy_nonoverlapping(d_lengths2.as_ptr(), d_lengths, 32);
        }
        return treesize2 + datasize2;
    }
    treesize + datasize
}

fn GetDynamicLengths(lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize, ll_lengths: *mut u32, d_lengths: *mut u32) -> f64 {
    let mut ll_counts: [usize; 288] = [0; 288];
    let mut d_counts: [usize; 32] = [0; 32];
    crate::src_lz77::ZopfliLZ77GetHistogram(lz77, lstart as crate::types::size_t, lend as crate::types::size_t, ll_counts.as_mut_ptr() as *mut crate::types::size_t, d_counts.as_mut_ptr() as *mut crate::types::size_t);
    ll_counts[256] = 1;
    crate::src_tree::ZopfliCalculateBitLengths(ll_counts.as_ptr() as *const crate::types::size_t, 288 as crate::types::size_t, 15, ll_lengths);
    crate::src_tree::ZopfliCalculateBitLengths(d_counts.as_ptr() as *const crate::types::size_t, 32 as crate::types::size_t, 15, d_lengths);
    crate::src_deflate::PatchDistanceCodesForBuggyDecoders(d_lengths);
    crate::src_deflate::TryOptimizeHuffmanForRle(lz77, lstart, lend, ll_counts.as_ptr(), d_counts.as_ptr(), ll_lengths, d_lengths)
}

pub extern "C" fn ZopfliCalculateBlockSize(lz77: *const crate::types::ZopfliLZ77Store, lstart: crate::types::size_t, lend: crate::types::size_t, btype: ::core::ffi::c_int) -> f64 {
    let mut ll_lengths: [u32; 288] = [0; 288];
    let mut d_lengths: [u32; 32] = [0; 32];
    let mut result: f64 = 3.0;

    if btype == 0 {
        let length = crate::src_lz77::ZopfliLZ77GetByteRange(lz77, lstart, lend);
        let rem = length % 65535;
        let blocks = length / 65535 + if rem != 0 { 1 } else { 0 };
        return (blocks * 5 * 8) as f64 + (length * 8) as f64;
    } else if btype == 1 {
        crate::src_deflate::GetFixedTree(ll_lengths.as_mut_ptr(), d_lengths.as_mut_ptr());
        result += crate::src_deflate::CalculateBlockSymbolSize(
            ll_lengths.as_ptr(),
            d_lengths.as_ptr(),
            lz77,
            lstart as usize,
            lend as usize,
        ) as f64;
    } else {
        result += crate::src_deflate::GetDynamicLengths(
            lz77,
            lstart as usize,
            lend as usize,
            ll_lengths.as_mut_ptr(),
            d_lengths.as_mut_ptr(),
        );
    }

    result
}

pub extern "C" fn ZopfliCalculateBlockSizeAutoType(lz77: *const crate::types::ZopfliLZ77Store, lstart: crate::types::size_t, lend: crate::types::size_t) -> f64 {
    let uncompressedcost = crate::src_deflate::ZopfliCalculateBlockSize(lz77, lstart, lend, 0);
    let size_ptr = unsafe {
        crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(lz77 as *mut ::core::ffi::c_void)
    };
    let size = unsafe { *(size_ptr as *const crate::types::size_t) };
    let fixedcost = if size > 1000 {
        uncompressedcost
    } else {
        crate::src_deflate::ZopfliCalculateBlockSize(lz77, lstart, lend, 1)
    };
    let dyncost = crate::src_deflate::ZopfliCalculateBlockSize(lz77, lstart, lend, 2);
    if uncompressedcost < fixedcost && uncompressedcost < dyncost {
        uncompressedcost
    } else if fixedcost < dyncost {
        fixedcost
    } else {
        dyncost
    }
}

fn AddNonCompressedBlock(options: *const crate::types::ZopfliOptions, r#final: std::os::raw::c_int, r#in: *const std::os::raw::c_uchar, instart: usize, inend: usize, bp: *mut std::os::raw::c_uchar, out: *mut *mut std::os::raw::c_uchar, outsize: *mut usize) {
    let mut pos = instart;
    let _ = options;
    loop {
        let mut blocksize: u16 = 65535;
        if pos + blocksize as usize > inend {
            blocksize = (inend - pos) as u16;
        }
        let currentfinal = (pos + blocksize as usize) >= inend;
        let nlen = !blocksize;

        crate::src_deflate::AddBit((r#final != 0 && currentfinal) as std::os::raw::c_int, bp, out, outsize);
        crate::src_deflate::AddBit(0, bp, out, outsize);
        crate::src_deflate::AddBit(0, bp, out, outsize);

        unsafe {
            *bp = 0;
        }

        unsafe {
            if (*outsize) & ((*outsize).wrapping_sub(1)) == 0 {
                let new_size = if *outsize == 0 { 1 } else { *outsize * 2 };
                let new_ptr = libc::realloc((*out) as *mut libc::c_void, new_size) as *mut std::os::raw::c_uchar;
                if new_ptr.is_null() {
                    return;
                }
                *out = new_ptr;
            }
            *(*out).add(*outsize) = (blocksize % 256) as std::os::raw::c_uchar;
            *outsize += 1;
        }
        unsafe {
            if (*outsize) & ((*outsize).wrapping_sub(1)) == 0 {
                let new_size = if *outsize == 0 { 1 } else { *outsize * 2 };
                let new_ptr = libc::realloc((*out) as *mut libc::c_void, new_size) as *mut std::os::raw::c_uchar;
                if new_ptr.is_null() {
                    return;
                }
                *out = new_ptr;
            }
            *(*out).add(*outsize) = ((blocksize / 256) % 256) as std::os::raw::c_uchar;
            *outsize += 1;
        }
        unsafe {
            if (*outsize) & ((*outsize).wrapping_sub(1)) == 0 {
                let new_size = if *outsize == 0 { 1 } else { *outsize * 2 };
                let new_ptr = libc::realloc((*out) as *mut libc::c_void, new_size) as *mut std::os::raw::c_uchar;
                if new_ptr.is_null() {
                    return;
                }
                *out = new_ptr;
            }
            *(*out).add(*outsize) = (nlen % 256) as std::os::raw::c_uchar;
            *outsize += 1;
        }
        unsafe {
            if (*outsize) & ((*outsize).wrapping_sub(1)) == 0 {
                let new_size = if *outsize == 0 { 1 } else { *outsize * 2 };
                let new_ptr = libc::realloc((*out) as *mut libc::c_void, new_size) as *mut std::os::raw::c_uchar;
                if new_ptr.is_null() {
                    return;
                }
                *out = new_ptr;
            }
            *(*out).add(*outsize) = ((nlen / 256) % 256) as std::os::raw::c_uchar;
            *outsize += 1;
        }

        for i in 0..blocksize as usize {
            unsafe {
                if (*outsize) & ((*outsize).wrapping_sub(1)) == 0 {
                    let new_size = if *outsize == 0 { 1 } else { *outsize * 2 };
                    let new_ptr = libc::realloc((*out) as *mut libc::c_void, new_size) as *mut std::os::raw::c_uchar;
                    if new_ptr.is_null() {
                        return;
                    }
                    *out = new_ptr;
                }
                *(*out).add(*outsize) = *r#in.add(pos + i);
                *outsize += 1;
            }
        }

        if currentfinal {
            break;
        }
        pos += blocksize as usize;
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_deflate_20
// c_function: AddLZ77Block
// rust_file: src_deflate.rs
// rust_signature: fn AddLZ77Block(options: *const crate::types::ZopfliOptions, btype: i32, r#final: i32, lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize, expected_data_size: usize, bp: *mut u8, out: *mut *mut u8, ou...
// c_first_line: static void AddLZ77Block(const ZopfliOptions* options, int btype, int final,
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_deflate_20/translated_rust.rs
// last_error_truncated:
//   error: expected expression, found `as`
//      --> src/src_deflate.rs:739:111
//       |
//       |                                                                                                               ^^ expected expression
//       |
//   help: parentheses are required to parse this as an expression
//       |
//       |                                                                                +                              +
// =================================
fn AddLZ77Block(options: *const crate::types::ZopfliOptions, btype: i32, r#final: i32, lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize, expected_data_size: usize, bp: *mut u8, out: *mut *mut u8, outsize: *mut usize) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_deflate::AddLZ77Block(options as _, btype as _, r#final as _, lz77 as _, lstart as _, lend as _, expected_data_size as _, bp as _, out as _, outsize as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_deflate_20
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_deflate_20/translated_rust.rs
 * ------------------------------------------------------------
fn AddLZ77Block(options: *const crate::types::ZopfliOptions, btype: i32, r#final: i32, lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize, expected_data_size: usize, bp: *mut u8, out: *mut *mut u8, outsize: *mut usize) {
    let mut ll_lengths: [u32; 288] = [0; 288];
    let mut d_lengths: [u32; 32] = [0; 32];
    let mut ll_symbols: [u32; 288] = [0; 288];
    let mut d_symbols: [u32; 32] = [0; 32];
    let mut detect_block_size: usize;
    let mut compressed_size: usize;
    let mut uncompressed_size: usize = 0;
    let mut i: usize;
    if btype == 0 {
        let length = crate::src_lz77::ZopfliLZ77GetByteRange(lz77, lstart as crate::types::size_t, lend as crate::types::size_t) as usize;
        let pos = if lstart == lend { 0 } else {
            unsafe {
                let pos_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__pos(lz77 as *mut ::core::ffi::c_void) as *mut usize;
                *pos_ptr.add(lstart)
            }
        };
        let end = pos + length;
        let data_ptr = unsafe {
            crate::compat::c2r_field_ptr_ZopfliLZ77Store__data(lz77 as *mut ::core::ffi::c_void) as *const u8
        };
        crate::src_deflate::AddNonCompressedBlock(options, r#final as std::os::raw::c_int, data_ptr, pos, end, bp as *mut std::os::raw::c_uchar, out as *mut *mut std::os::raw::c_uchar, outsize);
        return;
    }
    crate::src_deflate::AddBit(r#final as std::os::raw::c_int, bp, out, outsize);
    crate::src_deflate::AddBit(btype & 1, bp, out, outsize);
    crate::src_deflate::AddBit((btype & 2) >> 1, bp, out, outsize);
    if btype == 1 {
        crate::src_deflate::GetFixedTree(ll_lengths.as_mut_ptr(), d_lengths.as_mut_ptr());
    } else {
        let detect_tree_size: usize;
        if !(btype == 2) {
            unsafe { libc::abort(); }
        }
        let _ = crate::src_deflate::GetDynamicLengths(lz77, lstart, lend, ll_lengths.as_mut_ptr(), d_lengths.as_mut_ptr());
        detect_tree_size = unsafe { *outsize };
        crate::src_deflate::AddDynamicTree(ll_lengths.as_ptr(), d_lengths.as_ptr(), bp, out, outsize);
        let verbose_ptr = unsafe {
            crate::compat::c2r_field_ptr_ZopfliOptions__verbose(options as *mut ::core::ffi::c_void) as *mut i32
        };
        if unsafe { *verbose_ptr != 0 } {
            let treesize = unsafe { *outsize - detect_tree_size };
            let stderr_var = unsafe { libc::fdopen(2, "w\0".as_ptr() as *const i8) };
            if !stderr.is_null() {
                unsafe { libc::fprintf(stderr, "treesize: %d\n\0".as_ptr() as *const i8, treesize as i32); }
            }
        }
    }
    crate::src_tree::ZopfliLengthsToSymbols(ll_lengths.as_ptr(), 288 as crate::types::size_t, 15, ll_symbols.as_mut_ptr());
    crate::src_tree::ZopfliLengthsToSymbols(d_lengths.as_ptr(), 32 as crate::types::size_t, 15, d_symbols.as_mut_ptr());
    detect_block_size = unsafe { *outsize };
    crate::src_deflate::AddLZ77Data(lz77, lstart, lend, expected_data_size, ll_symbols.as_ptr(), ll_lengths.as_ptr(), d_symbols.as_ptr(), d_lengths.as_ptr(), bp, out, outsize);
    crate::src_deflate::AddHuffmanBits(ll_symbols[256], ll_lengths[256], bp, out, outsize);
    i = lstart;
    while i < lend {
        let dists_ptr = unsafe {
            crate::compat::c2r_field_ptr_ZopfliLZ77Store__dists(lz77 as *mut ::core::ffi::c_void) as *mut u16
        };
        let litlens_ptr = unsafe {
            crate::compat::c2r_field_ptr_ZopfliLZ77Store__litlens(lz77 as *mut ::core::ffi::c_void) as *mut u16
        };
        uncompressed_size += if unsafe { *dists_ptr.add(i) } == 0 { 1 } else { unsafe { *litlens_ptr.add(i) } as usize };
        i += 1;
    }
    compressed_size = unsafe { *outsize - detect_block_size };
    let verbose_ptr = unsafe {
        crate::compat::c2r_field_ptr_ZopfliOptions__verbose(options as *mut ::core::ffi::c_void) as *mut i32
    };
    if unsafe { *verbose_ptr != 0 } {
        let stderr_var = unsafe { libc::fdopen(2, "w\0".as_ptr() as *const i8) };
        if !stderr.is_null() {
            unsafe { libc::fprintf(stderr, "compressed block size: %d (%dk) (unc: %d)\n\0".as_ptr() as *const i8, compressed_size as i32, (compressed_size / 1024) as i32, uncompressed_size as i32); }
        }
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_deflate_20
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn AddLZ77BlockAutoType(options: *const crate::types::ZopfliOptions, r#final: i32, lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize, expected_data_size: usize, bp: *mut u8, out: *mut *mut u8, outsize: *mut usize) {
    let uncompressedcost = crate::src_deflate::ZopfliCalculateBlockSize(lz77, lstart as crate::types::size_t, lend as crate::types::size_t, 0);
    let mut fixedcost = crate::src_deflate::ZopfliCalculateBlockSize(lz77, lstart as crate::types::size_t, lend as crate::types::size_t, 1);
    let dyncost = crate::src_deflate::ZopfliCalculateBlockSize(lz77, lstart as crate::types::size_t, lend as crate::types::size_t, 2);
    let expensivefixed = fixedcost <= dyncost * 1.1;
    let mut fixedstore = std::mem::MaybeUninit::<crate::types::ZopfliLZ77Store>::uninit();
    if lstart == lend {
        crate::src_deflate::AddBits(r#final as u32, 1, bp, out, outsize);
        crate::src_deflate::AddBits(1, 2, bp, out, outsize);
        crate::src_deflate::AddBits(0, 7, bp, out, outsize);
        return;
    }
    unsafe {
        crate::src_lz77::ZopfliInitLZ77Store(std::ptr::null(), fixedstore.as_mut_ptr());
    }
    let fixedstore_ptr = unsafe { fixedstore.assume_init_mut() };
    if expensivefixed {
        let instart = 0;
        let inend = crate::src_lz77::ZopfliLZ77GetByteRange(lz77, lstart as crate::types::size_t, lend as crate::types::size_t);
        let mut s = std::mem::MaybeUninit::<crate::types::ZopfliBlockState>::uninit();
        unsafe {
            crate::src_lz77::ZopfliInitBlockState(options, instart, inend, 1, s.as_mut_ptr());
            crate::src_squeeze::ZopfliLZ77OptimalFixed(s.as_mut_ptr(), std::ptr::null(), instart, inend, fixedstore_ptr);
        }
        fixedcost = crate::src_deflate::ZopfliCalculateBlockSize(fixedstore_ptr, 0, 0, 1);
        unsafe {
            crate::src_lz77::ZopfliCleanBlockState(s.as_mut_ptr());
        }
    }
    if uncompressedcost < fixedcost && uncompressedcost < dyncost {
        crate::src_deflate::AddLZ77Block(options, 0, r#final, lz77, lstart, lend, expected_data_size, bp, out, outsize);
    } else if fixedcost < dyncost {
        if expensivefixed {
            crate::src_deflate::AddLZ77Block(options, 1, r#final, fixedstore_ptr, 0, 0, expected_data_size, bp, out, outsize);
        } else {
            crate::src_deflate::AddLZ77Block(options, 1, r#final, lz77, lstart, lend, expected_data_size, bp, out, outsize);
        }
    } else {
        crate::src_deflate::AddLZ77Block(options, 2, r#final, lz77, lstart, lend, expected_data_size, bp, out, outsize);
    }
    unsafe {
        crate::src_lz77::ZopfliCleanLZ77Store(fixedstore_ptr);
    }
}

pub extern "C" fn ZopfliDeflatePart(options: *const crate::types::ZopfliOptions, btype: ::core::ffi::c_int, final_: ::core::ffi::c_int, in_: *const ::core::ffi::c_uchar, instart: crate::types::size_t, inend: crate::types::size_t, bp: *mut ::core::ffi::c_uchar, out: *mut *mut ::core::ffi::c_uchar, outsize: *mut crate::types::size_t) {
    let mut splitpoints_uncompressed: *mut crate::types::size_t = std::ptr::null_mut();
    let mut npoints: crate::types::size_t = 0;
    let mut splitpoints: *mut crate::types::size_t = std::ptr::null_mut();
    let mut totalcost: f64 = 0.0;
    let mut lz77: crate::types::ZopfliLZ77Store = unsafe { std::mem::zeroed() };

    if btype == 0 {
        crate::src_deflate::AddNonCompressedBlock(options, final_, in_, instart as usize, inend as usize, bp, out, outsize);
        return;
    } else if btype == 1 {
        let mut store: crate::types::ZopfliLZ77Store = unsafe { std::mem::zeroed() };
        let mut s: crate::types::ZopfliBlockState = unsafe { std::mem::zeroed() };
        crate::src_lz77::ZopfliInitLZ77Store(in_, &mut store);
        crate::src_lz77::ZopfliInitBlockState(options, instart, inend, 1, &mut s);
        crate::src_squeeze::ZopfliLZ77OptimalFixed(&mut s, in_, instart, inend, &mut store);
        crate::src_deflate::AddLZ77Block(options, btype, final_, &store, 0, store.size as usize, 0, bp, out, outsize);
        crate::src_lz77::ZopfliCleanBlockState(&mut s);
        crate::src_lz77::ZopfliCleanLZ77Store(&mut store);
        return;
    }

    let blocksplitting_ptr = unsafe {
        crate::compat::c2r_field_ptr_ZopfliOptions__blocksplitting(options as *mut ::core::ffi::c_void)
    };
    let blocksplitting = if blocksplitting_ptr.is_null() {
        0
    } else {
        unsafe { *(blocksplitting_ptr as *const i32) }
    };
    if blocksplitting != 0 {
        let blocksplittingmax_ptr = unsafe {
            crate::compat::c2r_field_ptr_ZopfliOptions__blocksplittingmax(options as *mut ::core::ffi::c_void)
        };
        let blocksplittingmax = if blocksplittingmax_ptr.is_null() {
            0
        } else {
            unsafe { *(blocksplittingmax_ptr as *const crate::types::size_t) }
        };
        crate::src_blocksplitter::ZopfliBlockSplit(options, in_, instart, inend, blocksplittingmax, &mut splitpoints_uncompressed, &mut npoints);
        if npoints > 0 {
            splitpoints = unsafe { libc::malloc((npoints as usize) * std::mem::size_of::<crate::types::size_t>()) as *mut crate::types::size_t };
        }
    }

    crate::src_lz77::ZopfliInitLZ77Store(in_, &mut lz77);

    for i in 0..=npoints {
        let start = if i == 0 { instart } else { unsafe { *splitpoints_uncompressed.offset((i - 1) as isize) } };
        let end = if i == npoints { inend } else { unsafe { *splitpoints_uncompressed.offset(i as isize) } };
        let mut s: crate::types::ZopfliBlockState = unsafe { std::mem::zeroed() };
        let mut store: crate::types::ZopfliLZ77Store = unsafe { std::mem::zeroed() };
        crate::src_lz77::ZopfliInitLZ77Store(in_, &mut store);
        crate::src_lz77::ZopfliInitBlockState(options, start, end, 1, &mut s);
        let numiterations_ptr = unsafe {
            crate::compat::c2r_field_ptr_ZopfliOptions__numiterations(options as *mut ::core::ffi::c_void)
        };
        let numiterations = if numiterations_ptr.is_null() {
            0
        } else {
            unsafe { *(numiterations_ptr as *const i32) }
        };
        crate::src_squeeze::ZopfliLZ77Optimal(&mut s, in_, start, end, numiterations, &mut store);
        totalcost += crate::src_deflate::ZopfliCalculateBlockSizeAutoType(&store, 0, store.size);
        crate::src_lz77::ZopfliAppendLZ77Store(&store, &mut lz77);
        if i < npoints {
            unsafe { *splitpoints.offset(i as isize) = lz77.size; }
        }
        crate::src_lz77::ZopfliCleanBlockState(&mut s);
        crate::src_lz77::ZopfliCleanLZ77Store(&mut store);
    }

    if blocksplitting != 0 && npoints > 1 {
        let mut splitpoints2: *mut crate::types::size_t = std::ptr::null_mut();
        let mut npoints2: crate::types::size_t = 0;
        let mut totalcost2: f64 = 0.0;
        let blocksplittingmax_ptr = unsafe {
            crate::compat::c2r_field_ptr_ZopfliOptions__blocksplittingmax(options as *mut ::core::ffi::c_void)
        };
        let blocksplittingmax = if blocksplittingmax_ptr.is_null() {
            0
        } else {
            unsafe { *(blocksplittingmax_ptr as *const crate::types::size_t) }
        };
        crate::src_blocksplitter::ZopfliBlockSplitLZ77(options, &lz77, blocksplittingmax, &mut splitpoints2, &mut npoints2);
        for i in 0..=npoints2 {
            let start = if i == 0 { 0 } else { unsafe { *splitpoints2.offset((i - 1) as isize) } };
            let end = if i == npoints2 { lz77.size } else { unsafe { *splitpoints2.offset(i as isize) } };
            totalcost2 += crate::src_deflate::ZopfliCalculateBlockSizeAutoType(&lz77, start, end);
        }
        if totalcost2 < totalcost {
            unsafe { libc::free(splitpoints as *mut libc::c_void); }
            splitpoints = splitpoints2;
            npoints = npoints2;
        } else {
            unsafe { libc::free(splitpoints2 as *mut libc::c_void); }
        }
    }

    for i in 0..=npoints {
        let start = if i == 0 { 0 } else { unsafe { *splitpoints.offset((i - 1) as isize) } };
        let end = if i == npoints { lz77.size } else { unsafe { *splitpoints.offset(i as isize) } };
        let final_block = if i == npoints { final_ } else { 0 };
        crate::src_deflate::AddLZ77BlockAutoType(options, final_block, &lz77, start as usize, end as usize, 0, bp, out, outsize);
    }

    crate::src_lz77::ZopfliCleanLZ77Store(&mut lz77);
    if !splitpoints.is_null() {
        unsafe { libc::free(splitpoints as *mut libc::c_void); }
    }
    if !splitpoints_uncompressed.is_null() {
        unsafe { libc::free(splitpoints_uncompressed as *mut libc::c_void); }
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_deflate_23
// c_function: ZopfliDeflate
// rust_file: src_deflate.rs
// rust_signature: pub extern "C" fn ZopfliDeflate(options: *const crate::types::ZopfliOptions, btype: ::core::ffi::c_int, final_: ::core::ffi::c_int, in_: *const ::core::ffi::c_uchar, insize: crate::types::size_t, bp: *mut ::core::ffi::c_...
// c_first_line: void ZopfliDeflate(const ZopfliOptions* options, int btype, int final,
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_deflate_23/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find value `stderr` in crate `libc`
//      --> src/src_deflate.rs:964:27
//       |
//       |                           ^^^^^^ not found in `libc`
//       |
//      --> src/__c2r_generated/c2rust_fallback/src_blocksplitter.rs:64:5
//       |
//       |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `crate::src_deflate::__c2rust_fallback::src_blocksplitter::stderr`: not accessible
// =================================
pub extern "C" fn ZopfliDeflate(options: *const crate::types::ZopfliOptions, btype: ::core::ffi::c_int, final_: ::core::ffi::c_int, in_: *const ::core::ffi::c_uchar, insize: crate::types::size_t, bp: *mut ::core::ffi::c_uchar, out: *mut *mut ::core::ffi::c_uchar, outsize: *mut crate::types::size_t) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_deflate::ZopfliDeflate(options as _, btype as _, final_ as _, in_ as _, insize as _, bp as _, out as _, outsize as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_deflate_23
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_deflate_23/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn ZopfliDeflate(options: *const crate::types::ZopfliOptions, btype: ::core::ffi::c_int, final_: ::core::ffi::c_int, in_: *const ::core::ffi::c_uchar, insize: crate::types::size_t, bp: *mut ::core::ffi::c_uchar, out: *mut *mut ::core::ffi::c_uchar, outsize: *mut crate::types::size_t) {
    let offset = unsafe { *outsize };
    let mut i: crate::types::size_t = 0;
    while i < insize {
        let masterfinal = if i + 1000000 >= insize { 1 } else { 0 };
        let final2 = final_ & masterfinal;
        let size = if masterfinal != 0 { insize - i } else { 1000000 };
        crate::src_deflate::ZopfliDeflatePart(options, btype, final2, in_, i, i + size, bp, out, outsize);
        i += size;
    }
    unsafe {
        if !options.is_null() {
            let verbose_ptr = crate::compat::c2r_field_ptr_ZopfliOptions__verbose(options as *mut ::core::ffi::c_void);
            let verbose = *(verbose_ptr as *const ::core::ffi::c_int);
            if verbose != 0 {
                let outsize_val = *outsize;
                let deflate_size = outsize_val - offset;
                let removed = insize - deflate_size;
                libc::fprintf(
                    libc::stderr,
                    b"Original Size: %lu, Deflate: %lu, Compression: %f%% Removed\n\0" as *const u8 as *const libc::c_char,
                    insize as libc::c_ulong,
                    deflate_size as libc::c_ulong,
                    100.0 * (removed as libc::c_double) / (insize as libc::c_double)
                );
            }
        }
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_deflate_23
 * === C2R_LLM_FAILED_OUTPUT_END === */

