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

fn AddBit(bit: i32, bp: *mut u8, out: *mut *mut u8, outsize: *mut usize) {
    unsafe {
        if *bp == 0 {
            let size = *outsize;
            if (size & (size.wrapping_sub(1))) == 0 {
                if size == 0 {
                    *out = libc::malloc(std::mem::size_of::<u8>()) as *mut u8;
                } else {
                    *out = libc::realloc(*out as *mut std::ffi::c_void, size * 2 * std::mem::size_of::<u8>()) as *mut u8;
                }
            }
            *(*out).add(size) = 0;
            *outsize = size + 1;
        }
        let idx = *outsize - 1;
        *(*out).add(idx) |= (bit << *bp) as u8;
        *bp = (*bp + 1) & 7;
    }
}

fn AddBits(symbol: u32, length: u32, bp: *mut u8, out: *mut *mut u8, outsize: *mut usize) {
    unsafe {
        for i in 0..length {
            let bit = (symbol >> i) & 1;
            if *bp == 0 {
                let current_size = *outsize;
                if (current_size & (current_size.wrapping_sub(1))) == 0 {
                    if current_size == 0 {
                        *out = libc::malloc(std::mem::size_of::<u8>()) as *mut u8;
                    } else {
                        *out = libc::realloc(*out as *mut libc::c_void, current_size * 2 * std::mem::size_of::<u8>()) as *mut u8;
                    }
                }
                *(*out).add(current_size) = 0;
                *outsize = current_size + 1;
            }
            *(*out).add(*outsize - 1) |= (bit as u8) << *bp;
            *bp = (*bp + 1) & 7;
        }
    }
}

fn AddHuffmanBits(symbol: u32, length: u32, bp: *mut u8, out: *mut *mut u8, outsize: *mut usize) {
    unsafe {
        for i in 0..length {
            let bit = (symbol >> (length - i - 1)) & 1;
            if *bp == 0 {
                let current_size = *outsize;
                if (current_size & (current_size.wrapping_sub(1))) == 0 {
                    if current_size == 0 {
                        *out = libc::malloc(std::mem::size_of::<u8>()) as *mut u8;
                    } else {
                        *out = libc::realloc(*out as *mut libc::c_void, current_size * 2 * std::mem::size_of::<u8>()) as *mut u8;
                    }
                }
                *(*out).add(current_size) = 0;
                *outsize = current_size + 1;
            }
            *(*out).add(*outsize - 1) |= (bit as u8) << *bp;
            *bp = (*bp + 1) & 7;
        }
    }
}

fn PatchDistanceCodesForBuggyDecoders(d_lengths: *mut c_uint) {
    let mut num_dist_codes: i32 = 0;
    let mut i: i32;
    
    i = 0;
    while i < 30 {
        unsafe {
            if *d_lengths.offset(i as isize) != 0 {
                num_dist_codes += 1;
            }
        }
        if num_dist_codes >= 2 {
            return;
        }
        i += 1;
    }
    
    if num_dist_codes == 0 {
        unsafe {
            *d_lengths.offset(0) = 1;
            *d_lengths.offset(1) = 1;
        }
    } else if num_dist_codes == 1 {
        unsafe {
            let idx = if *d_lengths.offset(0) != 0 { 1 } else { 0 };
            *d_lengths.offset(idx) = 1;
        }
    }
}

fn EncodeTree(ll_lengths: *const u32, d_lengths: *const u32, use_16: i32, use_17: i32, use_18: i32, bp: *mut u8, out: *mut *mut u8, outsize: *mut usize) -> usize {
    let mut rle: *mut u32 = std::ptr::null_mut();
    let mut rle_bits: *mut u32 = std::ptr::null_mut();
    let mut rle_size: usize = 0;
    let mut rle_bits_size: usize = 0;
    let mut hlit: u32 = 29;
    let mut hdist: u32 = 29;
    let mut hclen: u32;
    let hlit2: u32;
    let mut i: usize;
    let mut j: usize;
    let mut clcounts: [usize; 19] = [0; 19];
    let mut clcl: [u32; 19] = [0; 19];
    let mut clsymbols: [u32; 19] = [0; 19];

    const ORDER: [u32; 19] = [
        16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15
    ];
    let size_only = out.is_null();
    let mut result_size: usize = 0;

    unsafe {
        while hlit > 0 && *ll_lengths.add(257 + hlit as usize - 1) == 0 {
            hlit -= 1;
        }
        while hdist > 0 && *d_lengths.add(1 + hdist as usize - 1) == 0 {
            hdist -= 1;
        }
        hlit2 = hlit + 257;

        let lld_total = hlit2 + hdist + 1;

        i = 0;
        while i < lld_total as usize {
            let symbol: u8 = if i < hlit2 as usize {
                *ll_lengths.add(i) as u8
            } else {
                *d_lengths.add(i - hlit2 as usize) as u8
            };
            let mut count: u32 = 1;

            if use_16 != 0 || (symbol == 0 && (use_17 != 0 || use_18 != 0)) {
                j = i + 1;
                while j < lld_total as usize {
                    let next_sym = if j < hlit2 as usize {
                        *ll_lengths.add(j) as u8
                    } else {
                        *d_lengths.add(j - hlit2 as usize) as u8
                    };
                    if symbol != next_sym {
                        break;
                    }
                    count += 1;
                    j += 1;
                }
            }
            i += count as usize - 1;

            if symbol == 0 && count >= 3 {
                if use_18 != 0 {
                    while count >= 11 {
                        let count2 = if count > 138 { 138 } else { count };
                        if !size_only {
                            if rle_size & (rle_size.wrapping_sub(1)) == 0 {
                                rle = if rle_size == 0 {
                                    libc::malloc(std::mem::size_of::<u32>()) as *mut u32
                                } else {
                                    libc::realloc(rle as *mut _, rle_size * 2 * std::mem::size_of::<u32>()) as *mut u32
                                };
                            }
                            *rle.add(rle_size) = 18;
                            rle_size += 1;

                            if rle_bits_size & (rle_bits_size.wrapping_sub(1)) == 0 {
                                rle_bits = if rle_bits_size == 0 {
                                    libc::malloc(std::mem::size_of::<u32>()) as *mut u32
                                } else {
                                    libc::realloc(rle_bits as *mut _, rle_bits_size * 2 * std::mem::size_of::<u32>()) as *mut u32
                                };
                            }
                            *rle_bits.add(rle_bits_size) = count2 - 11;
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
                            if rle_size & (rle_size.wrapping_sub(1)) == 0 {
                                rle = if rle_size == 0 {
                                    libc::malloc(std::mem::size_of::<u32>()) as *mut u32
                                } else {
                                    libc::realloc(rle as *mut _, rle_size * 2 * std::mem::size_of::<u32>()) as *mut u32
                                };
                            }
                            *rle.add(rle_size) = 17;
                            rle_size += 1;

                            if rle_bits_size & (rle_bits_size.wrapping_sub(1)) == 0 {
                                rle_bits = if rle_bits_size == 0 {
                                    libc::malloc(std::mem::size_of::<u32>()) as *mut u32
                                } else {
                                    libc::realloc(rle_bits as *mut _, rle_bits_size * 2 * std::mem::size_of::<u32>()) as *mut u32
                                };
                            }
                            *rle_bits.add(rle_bits_size) = count2 - 3;
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
                    if rle_size & (rle_size.wrapping_sub(1)) == 0 {
                        rle = if rle_size == 0 {
                            libc::malloc(std::mem::size_of::<u32>()) as *mut u32
                        } else {
                            libc::realloc(rle as *mut _, rle_size * 2 * std::mem::size_of::<u32>()) as *mut u32
                        };
                    }
                    *rle.add(rle_size) = symbol as u32;
                    rle_size += 1;

                    if rle_bits_size & (rle_bits_size.wrapping_sub(1)) == 0 {
                        rle_bits = if rle_bits_size == 0 {
                            libc::malloc(std::mem::size_of::<u32>()) as *mut u32
                        } else {
                            libc::realloc(rle_bits as *mut _, rle_bits_size * 2 * std::mem::size_of::<u32>()) as *mut u32
                        };
                    }
                    *rle_bits.add(rle_bits_size) = 0;
                    rle_bits_size += 1;
                }
                while count >= 3 {
                    let count2 = if count > 6 { 6 } else { count };
                    if !size_only {
                        if rle_size & (rle_size.wrapping_sub(1)) == 0 {
                            rle = if rle_size == 0 {
                                libc::malloc(std::mem::size_of::<u32>()) as *mut u32
                            } else {
                                libc::realloc(rle as *mut _, rle_size * 2 * std::mem::size_of::<u32>()) as *mut u32
                            };
                        }
                        *rle.add(rle_size) = 16;
                        rle_size += 1;

                        if rle_bits_size & (rle_bits_size.wrapping_sub(1)) == 0 {
                            rle_bits = if rle_bits_size == 0 {
                                libc::malloc(std::mem::size_of::<u32>()) as *mut u32
                            } else {
                                libc::realloc(rle_bits as *mut _, rle_bits_size * 2 * std::mem::size_of::<u32>()) as *mut u32
                            };
                        }
                        *rle_bits.add(rle_bits_size) = count2 - 3;
                        rle_bits_size += 1;
                    }
                    clcounts[16] += 1;
                    count -= count2;
                }
            }

            clcounts[symbol as usize] += count as usize;
            while count > 0 {
                if !size_only {
                    if rle_size & (rle_size.wrapping_sub(1)) == 0 {
                        rle = if rle_size == 0 {
                            libc::malloc(std::mem::size_of::<u32>()) as *mut u32
                        } else {
                            libc::realloc(rle as *mut _, rle_size * 2 * std::mem::size_of::<u32>()) as *mut u32
                        };
                    }
                    *rle.add(rle_size) = symbol as u32;
                    rle_size += 1;

                    if rle_bits_size & (rle_bits_size.wrapping_sub(1)) == 0 {
                        rle_bits = if rle_bits_size == 0 {
                            libc::malloc(std::mem::size_of::<u32>()) as *mut u32
                        } else {
                            libc::realloc(rle_bits as *mut _, rle_bits_size * 2 * std::mem::size_of::<u32>()) as *mut u32
                        };
                    }
                    *rle_bits.add(rle_bits_size) = 0;
                    rle_bits_size += 1;
                }
                count -= 1;
            }
            i += 1;
        }

        crate::src_tree::ZopfliCalculateBitLengths(clcounts.as_ptr() as *const usize, 19, 7, clcl.as_mut_ptr());
        if !size_only {
            crate::src_tree::ZopfliLengthsToSymbols(clcl.as_ptr(), 19, 7, clsymbols.as_mut_ptr());
        }

        hclen = 15;
        while hclen > 0 && clcounts[ORDER[(hclen + 4 - 1) as usize] as usize] == 0 {
            hclen -= 1;
        }

        if !size_only {
            crate::src_deflate::AddBits(hlit, 5, bp, out, outsize);
            crate::src_deflate::AddBits(hdist, 5, bp, out, outsize);
            crate::src_deflate::AddBits(hclen, 4, bp, out, outsize);

            for idx in 0..(hclen + 4) as usize {
                crate::src_deflate::AddBits(clcl[ORDER[idx] as usize], 3, bp, out, outsize);
            }

            for idx in 0..rle_size {
                let sym_idx = *rle.add(idx) as usize;
                let symbol = clsymbols[sym_idx];
                crate::src_deflate::AddHuffmanBits(symbol, clcl[sym_idx], bp, out, outsize);

                if *rle.add(idx) == 16 {
                    crate::src_deflate::AddBits(*rle_bits.add(idx), 2, bp, out, outsize);
                } else if *rle.add(idx) == 17 {
                    crate::src_deflate::AddBits(*rle_bits.add(idx), 3, bp, out, outsize);
                } else if *rle.add(idx) == 18 {
                    crate::src_deflate::AddBits(*rle_bits.add(idx), 7, bp, out, outsize);
                }
            }
        }

        result_size += 14;
        result_size += (hclen + 4) as usize * 3;
        for idx in 0..19 {
            result_size += clcl[idx] as usize * clcounts[idx];
        }

        result_size += clcounts[16] * 2;
        result_size += clcounts[17] * 3;
        result_size += clcounts[18] * 7;

        libc::free(rle as *mut _);
        libc::free(rle_bits as *mut _);
    }

    result_size
}

fn AddDynamicTree(ll_lengths: *const c_uint, d_lengths: *const c_uint, bp: *mut u8, out: *mut *mut u8, outsize: *mut usize) {
    let mut best: i32 = 0;
    let mut bestsize: usize = 0;

    for i in 0..8 {
        let size = EncodeTree(
            ll_lengths as *const u32,
            d_lengths as *const u32,
            i & 1,
            i & 2,
            i & 4,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        if bestsize == 0 || size < bestsize {
            bestsize = size;
            best = i;
        }
    }

    EncodeTree(
        ll_lengths as *const u32,
        d_lengths as *const u32,
        best & 1,
        best & 2,
        best & 4,
        bp,
        out,
        outsize,
    );
}

fn CalculateTreeSize(ll_lengths: *const u32, d_lengths: *const u32) -> usize {
    let mut result: usize = 0;
    let mut i: i32;

    i = 0;
    while i < 8 {
        let size = EncodeTree(
            ll_lengths,
            d_lengths,
            i & 1,
            i & 2,
            i & 4,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        if result == 0 || size < result {
            result = size;
        }
        i += 1;
    }

    result
}

fn AddLZ77Data(lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize, expected_data_size: usize, ll_symbols: *const u32, ll_lengths: *const u32, d_symbols: *const u32, d_lengths: *const u32, bp: *mut u8, out: *mut *mut u8, outsize: *mut usize) {
    let zopfli_get_length_symbol = |l: u32| -> u32 {
        static TABLE: [u32; 259] = [
            0, 0, 0, 257, 258, 259, 260, 261, 262, 263, 264, 265, 265, 266, 266, 267, 267, 268, 268,
            269, 269, 269, 269, 270, 270, 270, 270, 271, 271, 271, 271, 272, 272, 272, 272,
            273, 273, 273, 273, 273, 273, 273, 273, 274, 274, 274, 274, 274, 274, 274, 274,
            275, 275, 275, 275, 275, 275, 275, 275, 276, 276, 276, 276, 276, 276, 276, 276,
            277, 277, 277, 277, 277, 277, 277, 277, 277, 277, 277, 277, 277, 277, 277, 277,
            278, 278, 278, 278, 278, 278, 278, 278, 278, 278, 278, 278, 278, 278, 278, 278,
            279, 279, 279, 279, 279, 279, 279, 279, 279, 279, 279, 279, 279, 279, 279, 279,
            280, 280, 280, 280, 280, 280, 280, 280, 280, 280, 280, 280, 280, 280, 280, 280,
            281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281,
            281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281,
            282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282,
            282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282,
            283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283,
            283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283,
            284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284,
            284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 285,
        ];
        TABLE[l as usize]
    };
    let zopfli_get_length_extra_bits = |l: u32| -> u32 {
        static TABLE: [u32; 259] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
            4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
            5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
            5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
            5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
            5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0,
        ];
        TABLE[l as usize]
    };
    let zopfli_get_length_extra_bits_value = |l: u32| -> u32 {
        static TABLE: [u32; 259] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3,
            0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7,
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 0,
        ];
        TABLE[l as usize]
    };
    let zopfli_get_dist_symbol = |d: u32| -> u32 {
        if d < 5 { return if d < 3 { d.saturating_sub(1) } else { d - 1 }; }
        if d < 193 { let l = (31 ^ (d - 1).leading_zeros()) as u32; return (l * 2) + ((d - 1) >> (l - 1)) as u32 - 2; }
        if d < 32769 { let l = (31 ^ (d - 1).leading_zeros()) as u32; return (l * 2) + ((d - 1) >> (l - 1)) as u32 - 2; }
        if d < 32769 { 28 } else { 29 }
    };
    let zopfli_get_dist_extra_bits = |d: u32| -> u32 { if d < 5 { 0 } else { (31 ^ (d - 1).leading_zeros()) as u32 - 1 } };
    let zopfli_get_dist_extra_bits_value = |d: u32| -> u32 { if d < 5 { 0 } else { let l = (31 ^ (d - 1).leading_zeros()) as u32; (d - 1) & ((1 << (l - 1)) - 1) } };
    let mut testlength: usize = 0;
    for i in lstart..lend {
        let dists_ptr = unsafe { crate::compat::c2r_field_ptr_ZopfliLZ77Store__dists(lz77 as *mut ::core::ffi::c_void) };
        let litlens_ptr = unsafe { crate::compat::c2r_field_ptr_ZopfliLZ77Store__litlens(lz77 as *mut ::core::ffi::c_void) };
        let dist = unsafe { *((*(dists_ptr as *const *mut u16)).add(i)) } as u32;
        let litlen = unsafe { *((*(litlens_ptr as *const *mut u16)).add(i)) } as u32;
        if dist == 0 {
            assert!(litlen < 256);
            assert!(unsafe { *ll_lengths.add(litlen as usize) } > 0);
            unsafe { AddHuffmanBits(*ll_symbols.add(litlen as usize), *ll_lengths.add(litlen as usize), bp, out, outsize); }
            testlength += 1;
        } else {
            let lls = zopfli_get_length_symbol(litlen);
            let ds = zopfli_get_dist_symbol(dist);
            assert!(litlen >= 3 && litlen <= 288);
            assert!(unsafe { *ll_lengths.add(lls as usize) } > 0);
            assert!(unsafe { *d_lengths.add(ds as usize) } > 0);
            unsafe {
                AddHuffmanBits(*ll_symbols.add(lls as usize), *ll_lengths.add(lls as usize), bp, out, outsize);
                AddBits(zopfli_get_length_extra_bits_value(litlen), zopfli_get_length_extra_bits(litlen), bp, out, outsize);
                AddHuffmanBits(*d_symbols.add(ds as usize), *d_lengths.add(ds as usize), bp, out, outsize);
                AddBits(zopfli_get_dist_extra_bits_value(dist), zopfli_get_dist_extra_bits(dist), bp, out, outsize);
            }
            testlength += litlen as usize;
        }
    }
    assert!(expected_data_size == 0 || testlength == expected_data_size);
}

fn GetFixedTree(ll_lengths: *mut u32, d_lengths: *mut u32) -> () {
    let mut i: usize;
    i = 0;
    while i < 144 {
        unsafe { *ll_lengths.add(i) = 8; }
        i += 1;
    }
    i = 144;
    while i < 256 {
        unsafe { *ll_lengths.add(i) = 9; }
        i += 1;
    }
    i = 256;
    while i < 280 {
        unsafe { *ll_lengths.add(i) = 7; }
        i += 1;
    }
    i = 280;
    while i < 288 {
        unsafe { *ll_lengths.add(i) = 8; }
        i += 1;
    }
    i = 0;
    while i < 32 {
        unsafe { *d_lengths.add(i) = 5; }
        i += 1;
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_deflate_10
// c_function: CalculateBlockSymbolSizeSmall
// rust_file: src_deflate.rs
// rust_signature: fn CalculateBlockSymbolSizeSmall(ll_lengths: *const u32, d_lengths: *const u32, lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize) -> usize
// c_first_line: static size_t CalculateBlockSymbolSizeSmall(const unsigned* ll_lengths,
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/claude-test0125/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_deflate_10/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find function, tuple struct or tuple variant `ZopfliGetLengthSymbol` in this scope
//      --> src/src_deflate.rs:188:29
//       |
//       |                             ^^^^^^^^^^^^^^^^^^^^^ not found in this scope
//   error[E0425]: cannot find function, tuple struct or tuple variant `ZopfliGetDistSymbol` in this scope
//      --> src/src_deflate.rs:189:28
//       |
//       |                            ^^^^^^^^^^^^^^^^^^^ not found in this scope
// =================================
fn CalculateBlockSymbolSizeSmall(ll_lengths: *const u32, d_lengths: *const u32, lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize) -> usize {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_deflate::CalculateBlockSymbolSizeSmall(ll_lengths as _, d_lengths as _, lz77 as _, lstart as _, lend as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_deflate_10
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/claude-test0125/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_deflate_10/translated_rust.rs
 * ------------------------------------------------------------
fn CalculateBlockSymbolSizeSmall(ll_lengths: *const u32, d_lengths: *const u32, lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize) -> usize {
    let mut result: usize = 0;
    
    for i in lstart..lend {
        // Get lz77->size via accessor shim
        let size_ptr = unsafe {
            crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(lz77 as *mut ::core::ffi::c_void)
        };
        let lz77_size = unsafe { *(size_ptr as *const usize) };
        
        // Assert i < lz77->size
        debug_assert!(i < lz77_size);
        
        // Get lz77->litlens via accessor shim
        let litlens_ptr = unsafe {
            crate::compat::c2r_field_ptr_ZopfliLZ77Store__litlens(lz77 as *mut ::core::ffi::c_void)
        };
        let litlens = unsafe { *(litlens_ptr as *const *mut u16) };
        
        // Get lz77->dists via accessor shim
        let dists_ptr = unsafe {
            crate::compat::c2r_field_ptr_ZopfliLZ77Store__dists(lz77 as *mut ::core::ffi::c_void)
        };
        let dists = unsafe { *(dists_ptr as *const *mut u16) };
        
        let litlen = unsafe { *litlens.add(i) } as u32;
        let dist = unsafe { *dists.add(i) } as u32;
        
        // Assert lz77->litlens[i] < 259
        debug_assert!(litlen < 259);
        
        if dist == 0 {
            result += unsafe { *ll_lengths.add(litlen as usize) } as usize;
        } else {
            let ll_symbol = ZopfliGetLengthSymbol(litlen as i32);
            let d_symbol = ZopfliGetDistSymbol(dist as i32);
            result += unsafe { *ll_lengths.add(ll_symbol as usize) } as usize;
            result += unsafe { *d_lengths.add(d_symbol as usize) } as usize;
            result += ZopfliGetLengthSymbolExtraBits(ll_symbol) as usize;
            result += ZopfliGetDistSymbolExtraBits(d_symbol) as usize;
        }
    }
    
    result += unsafe { *ll_lengths.add(256) } as usize;
    result
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_deflate_10
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn CalculateBlockSymbolSizeGivenCounts(ll_counts: *const usize, d_counts: *const usize, ll_lengths: *const u32, d_lengths: *const u32, lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize) -> usize {
    const ZOPFLI_NUM_LL: usize = 288;
    const ZOPFLI_NUM_D: usize = 32;
    
    static LENGTH_EXTRA_BITS: [i32; 29] = [
        0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2,
        3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0
    ];
    
    static DIST_EXTRA_BITS: [i32; 30] = [
        0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6,
        7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13
    ];
    
    let mut result: usize = 0;
    
    if lstart + ZOPFLI_NUM_LL * 3 > lend {
        return CalculateBlockSymbolSizeSmall(ll_lengths, d_lengths, lz77, lstart, lend);
    } else {
        unsafe {
            for i in 0..256usize {
                result += (*ll_lengths.add(i) as usize) * (*ll_counts.add(i));
            }
            for i in 257..286usize {
                result += (*ll_lengths.add(i) as usize) * (*ll_counts.add(i));
                let extra_bits = LENGTH_EXTRA_BITS[i - 257] as usize;
                result += extra_bits * (*ll_counts.add(i));
            }
            for i in 0..30usize {
                result += (*d_lengths.add(i) as usize) * (*d_counts.add(i));
                let extra_bits = DIST_EXTRA_BITS[i] as usize;
                result += extra_bits * (*d_counts.add(i));
            }
            result += *ll_lengths.add(256) as usize;
        }
        return result;
    }
}

fn CalculateBlockSymbolSize(ll_lengths: *const u32, d_lengths: *const u32, lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize) -> usize {
    if lstart + 288 * 3 > lend {
        return CalculateBlockSymbolSizeSmall(ll_lengths, d_lengths, lz77, lstart, lend);
    } else {
        let mut ll_counts: [usize; 288] = [0; 288];
        let mut d_counts: [usize; 32] = [0; 32];
        crate::src_lz77::ZopfliLZ77GetHistogram(
            lz77,
            lstart as crate::types::size_t,
            lend as crate::types::size_t,
            ll_counts.as_mut_ptr(),
            d_counts.as_mut_ptr(),
        );
        return CalculateBlockSymbolSizeGivenCounts(
            ll_counts.as_ptr(),
            d_counts.as_ptr(),
            ll_lengths,
            d_lengths,
            lz77,
            lstart,
            lend,
        );
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
    let mut length = length;
    let mut i: i32;
    let mut k: i32;
    let mut stride: i32;
    let mut symbol: usize;
    let mut sum: usize;
    let mut limit: usize;
    let mut good_for_rle: *mut i32;

    loop {
        if length < 0 {
            break;
        }
        if length == 0 {
            return;
        }
        if unsafe { *counts.offset((length - 1) as isize) } != 0 {
            break;
        }
        length -= 1;
    }

    good_for_rle = unsafe { libc::malloc((length as usize) * std::mem::size_of::<i32>()) as *mut i32 };
    i = 0;
    while i < length {
        unsafe { *good_for_rle.offset(i as isize) = 0 };
        i += 1;
    }

    symbol = unsafe { *counts.offset(0) };
    stride = 0;
    i = 0;
    while i < length + 1 {
        if i == length || unsafe { *counts.offset(i as isize) } != symbol {
            if (symbol == 0 && stride >= 5) || (symbol != 0 && stride >= 7) {
                k = 0;
                while k < stride {
                    unsafe { *good_for_rle.offset((i - k - 1) as isize) = 1 };
                    k += 1;
                }
            }
            stride = 1;
            if i != length {
                symbol = unsafe { *counts.offset(i as isize) };
            }
        } else {
            stride += 1;
        }
        i += 1;
    }

    stride = 0;
    limit = unsafe { *counts.offset(0) };
    sum = 0;
    i = 0;
    while i < length + 1 {
        if i == length || unsafe { *good_for_rle.offset(i as isize) } != 0
            || crate::src_deflate::AbsDiff(unsafe { *counts.offset(i as isize) }, limit) >= 4
        {
            if stride >= 4 || (stride >= 3 && sum == 0) {
                let mut count = ((sum + (stride as usize) / 2) / (stride as usize)) as i32;
                if count < 1 {
                    count = 1;
                }
                if sum == 0 {
                    count = 0;
                }
                k = 0;
                while k < stride {
                    unsafe { *counts.offset((i - k - 1) as isize) = count as usize };
                    k += 1;
                }
            }
            stride = 0;
            sum = 0;
            if i < length - 3 {
                limit = (unsafe {
                    *counts.offset(i as isize)
                        + *counts.offset((i + 1) as isize)
                        + *counts.offset((i + 2) as isize)
                        + *counts.offset((i + 3) as isize)
                        + 2
                }) / 4;
            } else if i < length {
                limit = unsafe { *counts.offset(i as isize) };
            } else {
                limit = 0;
            }
        }
        stride += 1;
        if i != length {
            sum += unsafe { *counts.offset(i as isize) };
        }
        i += 1;
    }

    unsafe { libc::free(good_for_rle as *mut ::core::ffi::c_void) };
}

fn TryOptimizeHuffmanForRle(lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize, ll_counts: *const usize, d_counts: *const usize, ll_lengths: *mut u32, d_lengths: *mut u32) -> f64 {
    let mut ll_counts2: [usize; 288] = [0; 288];
    let mut d_counts2: [usize; 32] = [0; 32];
    let mut ll_lengths2: [u32; 288] = [0; 288];
    let mut d_lengths2: [u32; 32] = [0; 32];

    unsafe {
        let treesize = crate::src_deflate::CalculateTreeSize(ll_lengths, d_lengths);
        let datasize = crate::src_deflate::CalculateBlockSymbolSizeGivenCounts(
            ll_counts, d_counts, ll_lengths, d_lengths, lz77, lstart, lend
        );

        std::ptr::copy_nonoverlapping(ll_counts, ll_counts2.as_mut_ptr(), 288);
        std::ptr::copy_nonoverlapping(d_counts, d_counts2.as_mut_ptr(), 32);

        crate::src_deflate::OptimizeHuffmanForRle(288, ll_counts2.as_mut_ptr());
        crate::src_deflate::OptimizeHuffmanForRle(32, d_counts2.as_mut_ptr());

        crate::src_tree::ZopfliCalculateBitLengths(
            ll_counts2.as_ptr(), 288, 15, ll_lengths2.as_mut_ptr()
        );
        crate::src_tree::ZopfliCalculateBitLengths(
            d_counts2.as_ptr(), 32, 15, d_lengths2.as_mut_ptr()
        );

        crate::src_deflate::PatchDistanceCodesForBuggyDecoders(d_lengths2.as_mut_ptr());

        let treesize2 = crate::src_deflate::CalculateTreeSize(
            ll_lengths2.as_ptr(), d_lengths2.as_ptr()
        );
        let datasize2 = crate::src_deflate::CalculateBlockSymbolSizeGivenCounts(
            ll_counts, d_counts, ll_lengths2.as_ptr(), d_lengths2.as_ptr(), lz77, lstart, lend
        );

        if (treesize2 + datasize2) < (treesize + datasize) {
            std::ptr::copy_nonoverlapping(ll_lengths2.as_ptr(), ll_lengths, 288);
            std::ptr::copy_nonoverlapping(d_lengths2.as_ptr(), d_lengths, 32);
            return (treesize2 + datasize2) as f64;
        }
        (treesize + datasize) as f64
    }
}

fn GetDynamicLengths(lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize, ll_lengths: *mut u32, d_lengths: *mut u32) -> f64 {
    let mut ll_counts: [usize; 288] = [0; 288];
    let mut d_counts: [usize; 32] = [0; 32];

    unsafe {
        crate::src_lz77::ZopfliLZ77GetHistogram(
            lz77,
            lstart as crate::types::size_t,
            lend as crate::types::size_t,
            ll_counts.as_mut_ptr(),
            d_counts.as_mut_ptr(),
        );
    }
    
    ll_counts[256] = 1;
    
    unsafe {
        crate::src_tree::ZopfliCalculateBitLengths(
            ll_counts.as_ptr() as *const crate::types::size_t,
            288 as crate::types::size_t,
            15,
            ll_lengths,
        );
        crate::src_tree::ZopfliCalculateBitLengths(
            d_counts.as_ptr() as *const crate::types::size_t,
            32 as crate::types::size_t,
            15,
            d_lengths,
        );
    }
    
    crate::src_deflate::PatchDistanceCodesForBuggyDecoders(d_lengths);
    
    crate::src_deflate::TryOptimizeHuffmanForRle(
        lz77,
        lstart,
        lend,
        ll_counts.as_ptr(),
        d_counts.as_ptr(),
        ll_lengths,
        d_lengths,
    )
}

pub extern "C" fn ZopfliCalculateBlockSize(lz77: *const crate::types::ZopfliLZ77Store, lstart: crate::types::size_t, lend: crate::types::size_t, btype: ::core::ffi::c_int) -> f64 {
    let mut ll_lengths: [u32; 288] = [0; 288];
    let mut d_lengths: [u32; 32] = [0; 32];

    let mut result: f64 = 3.0;

    if btype == 0 {
        let length = crate::src_lz77::ZopfliLZ77GetByteRange(lz77, lstart, lend);
        let rem = length % 65535;
        let blocks = length / 65535 + if rem != 0 { 1 } else { 0 };

        return (blocks * 5 * 8 + length * 8) as f64;
    }
    if btype == 1 {
        GetFixedTree(ll_lengths.as_mut_ptr(), d_lengths.as_mut_ptr());
        result += CalculateBlockSymbolSize(
            ll_lengths.as_ptr(),
            d_lengths.as_ptr(),
            lz77,
            lstart as usize,
            lend as usize,
        ) as f64;
    } else {
        result += GetDynamicLengths(
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
    
    // Access lz77->size via accessor shim
    let size_ptr = unsafe {
        crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(lz77 as *mut ::core::ffi::c_void)
    };
    let size_val = unsafe { *(size_ptr as *const crate::types::size_t) };
    
    let fixedcost = if size_val > 1000 {
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

fn AddNonCompressedBlock(options: *const crate::types::ZopfliOptions, r#final: i32, r#in: *const u8, instart: usize, inend: usize, bp: *mut u8, out: *mut *mut u8, outsize: *mut usize) {
    let _ = options;
    let mut pos = instart;
    
    loop {
        let mut blocksize: u16 = 65535;
        
        if pos + (blocksize as usize) > inend {
            blocksize = (inend - pos) as u16;
        }
        let currentfinal = if pos + (blocksize as usize) >= inend { 1 } else { 0 };
        
        let nlen: u16 = !blocksize;
        
        AddBit(if r#final != 0 && currentfinal != 0 { 1 } else { 0 }, bp, out, outsize);
        AddBit(0, bp, out, outsize);
        AddBit(0, bp, out, outsize);
        
        unsafe { *bp = 0; }
        
        // ZOPFLI_APPEND_DATA macro expansion for blocksize % 256
        unsafe {
            let sz = *outsize;
            if (sz & (sz.wrapping_sub(1))) == 0 {
                if sz == 0 {
                    *out = libc::malloc(std::mem::size_of::<u8>()) as *mut u8;
                } else {
                    *out = libc::realloc(*out as *mut std::ffi::c_void, sz * 2 * std::mem::size_of::<u8>()) as *mut u8;
                }
            }
            *(*out).add(sz) = (blocksize % 256) as u8;
            *outsize += 1;
        }
        
        // blocksize / 256 % 256
        unsafe {
            let sz = *outsize;
            if (sz & (sz.wrapping_sub(1))) == 0 {
                if sz == 0 {
                    *out = libc::malloc(std::mem::size_of::<u8>()) as *mut u8;
                } else {
                    *out = libc::realloc(*out as *mut std::ffi::c_void, sz * 2 * std::mem::size_of::<u8>()) as *mut u8;
                }
            }
            *(*out).add(sz) = ((blocksize / 256) % 256) as u8;
            *outsize += 1;
        }
        
        // nlen % 256
        unsafe {
            let sz = *outsize;
            if (sz & (sz.wrapping_sub(1))) == 0 {
                if sz == 0 {
                    *out = libc::malloc(std::mem::size_of::<u8>()) as *mut u8;
                } else {
                    *out = libc::realloc(*out as *mut std::ffi::c_void, sz * 2 * std::mem::size_of::<u8>()) as *mut u8;
                }
            }
            *(*out).add(sz) = (nlen % 256) as u8;
            *outsize += 1;
        }
        
        // nlen / 256 % 256
        unsafe {
            let sz = *outsize;
            if (sz & (sz.wrapping_sub(1))) == 0 {
                if sz == 0 {
                    *out = libc::malloc(std::mem::size_of::<u8>()) as *mut u8;
                } else {
                    *out = libc::realloc(*out as *mut std::ffi::c_void, sz * 2 * std::mem::size_of::<u8>()) as *mut u8;
                }
            }
            *(*out).add(sz) = ((nlen / 256) % 256) as u8;
            *outsize += 1;
        }
        
        // Copy block data
        for i in 0..(blocksize as usize) {
            unsafe {
                let sz = *outsize;
                if (sz & (sz.wrapping_sub(1))) == 0 {
                    if sz == 0 {
                        *out = libc::malloc(std::mem::size_of::<u8>()) as *mut u8;
                    } else {
                        *out = libc::realloc(*out as *mut std::ffi::c_void, sz * 2 * std::mem::size_of::<u8>()) as *mut u8;
                    }
                }
                *(*out).add(sz) = *r#in.add(pos + i);
                *outsize += 1;
            }
        }
        
        if currentfinal != 0 {
            break;
        }
        pos += blocksize as usize;
    }
}

fn AddLZ77Block(options: *const crate::types::ZopfliOptions, btype: i32, r#final: i32, lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize, expected_data_size: usize, bp: *mut u8, out: *mut *mut u8, outsize: *mut usize) {
    let mut ll_lengths: [u32; 288] = [0; 288];
    let mut d_lengths: [u32; 32] = [0; 32];
    let mut ll_symbols: [u32; 288] = [0; 288];
    let mut d_symbols: [u32; 32] = [0; 32];
    let mut detect_block_size: usize;
    let compressed_size: usize;
    let mut uncompressed_size: usize = 0;
    
    unsafe {
        detect_block_size = *outsize;
        
        if btype == 0 {
            let length = crate::src_lz77::ZopfliLZ77GetByteRange(lz77, lstart as crate::types::size_t, lend as crate::types::size_t) as usize;
            let pos_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__pos(lz77 as *mut ::core::ffi::c_void) as *mut *mut usize;
            let pos = if lstart == lend { 0 } else { *(*pos_ptr).add(lstart) };
            let end = pos + length;
            let data_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__data(lz77 as *mut ::core::ffi::c_void) as *mut *const u8;
            crate::src_deflate::AddNonCompressedBlock(options, r#final, *data_ptr, pos, end, bp, out, outsize);
            return;
        }
        
        crate::src_deflate::AddBit(r#final, bp, out, outsize);
        crate::src_deflate::AddBit(btype & 1, bp, out, outsize);
        crate::src_deflate::AddBit((btype & 2) >> 1, bp, out, outsize);
        
        if btype == 1 {
            crate::src_deflate::GetFixedTree(ll_lengths.as_mut_ptr(), d_lengths.as_mut_ptr());
        } else {
            let detect_tree_size: usize;
            assert!(btype == 2);
            
            let _ = crate::src_deflate::GetDynamicLengths(lz77, lstart, lend, ll_lengths.as_mut_ptr(), d_lengths.as_mut_ptr());
            
            detect_tree_size = *outsize;
            crate::src_deflate::AddDynamicTree(ll_lengths.as_ptr(), d_lengths.as_ptr(), bp, out, outsize);
            
            let verbose_ptr = crate::compat::c2r_field_ptr_ZopfliOptions__verbose(options as *mut ::core::ffi::c_void) as *mut i32;
            if *verbose_ptr != 0 {
                extern "C" {
                    static stderr: *mut libc::FILE;
                }
                libc::fprintf(
                    stderr,
                    b"treesize: %d\n\0".as_ptr() as *const i8,
                    (*outsize - detect_tree_size) as i32
                );
            }
        }
        
        crate::src_tree::ZopfliLengthsToSymbols(ll_lengths.as_ptr(), 288, 15, ll_symbols.as_mut_ptr());
        crate::src_tree::ZopfliLengthsToSymbols(d_lengths.as_ptr(), 32, 15, d_symbols.as_mut_ptr());
        
        detect_block_size = *outsize;
        crate::src_deflate::AddLZ77Data(lz77, lstart, lend, expected_data_size,
                                        ll_symbols.as_ptr(), ll_lengths.as_ptr(),
                                        d_symbols.as_ptr(), d_lengths.as_ptr(),
                                        bp, out, outsize);
        
        crate::src_deflate::AddHuffmanBits(ll_symbols[256], ll_lengths[256], bp, out, outsize);
        
        let dists_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__dists(lz77 as *mut ::core::ffi::c_void) as *mut *mut u16;
        let litlens_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__litlens(lz77 as *mut ::core::ffi::c_void) as *mut *mut u16;
        
        for i in lstart..lend {
            if *(*dists_ptr).add(i) == 0 {
                uncompressed_size += 1;
            } else {
                uncompressed_size += *(*litlens_ptr).add(i) as usize;
            }
        }
        
        compressed_size = *outsize - detect_block_size;
        
        let verbose_ptr = crate::compat::c2r_field_ptr_ZopfliOptions__verbose(options as *mut ::core::ffi::c_void) as *mut i32;
        if *verbose_ptr != 0 {
            extern "C" {
                static stderr: *mut libc::FILE;
            }
            libc::fprintf(
                stderr,
                b"compressed block size: %d (%dk) (unc: %d)\n\0".as_ptr() as *const i8,
                compressed_size as i32,
                (compressed_size / 1024) as i32,
                uncompressed_size as i32
            );
        }
    }
}

fn AddLZ77BlockAutoType(options: *const crate::types::ZopfliOptions, r#final: i32, lz77: *const crate::types::ZopfliLZ77Store, lstart: usize, lend: usize, expected_data_size: usize, bp: *mut u8, out: *mut *mut u8, outsize: *mut usize) {
    unsafe {
        let uncompressedcost = crate::src_deflate::ZopfliCalculateBlockSize(lz77, lstart as crate::types::size_t, lend as crate::types::size_t, 0);
        let mut fixedcost = crate::src_deflate::ZopfliCalculateBlockSize(lz77, lstart as crate::types::size_t, lend as crate::types::size_t, 1);
        let dyncost = crate::src_deflate::ZopfliCalculateBlockSize(lz77, lstart as crate::types::size_t, lend as crate::types::size_t, 2);

        let lz77_size = *(crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(lz77 as *mut ::core::ffi::c_void) as *const crate::types::size_t);
        let expensivefixed: i32 = if (lz77_size < 1000) || (fixedcost <= dyncost * 1.1) { 1 } else { 0 };

        let mut fixedstore: crate::types::ZopfliLZ77Store = std::mem::zeroed();

        if lstart == lend {
            crate::src_deflate::AddBits(r#final as u32, 1, bp, out, outsize);
            crate::src_deflate::AddBits(1, 2, bp, out, outsize);
            crate::src_deflate::AddBits(0, 7, bp, out, outsize);
            return;
        }

        let lz77_data = *(crate::compat::c2r_field_ptr_ZopfliLZ77Store__data(lz77 as *mut ::core::ffi::c_void) as *const *const u8);
        crate::src_lz77::ZopfliInitLZ77Store(lz77_data, &mut fixedstore);

        if expensivefixed != 0 {
            let lz77_pos = *(crate::compat::c2r_field_ptr_ZopfliLZ77Store__pos(lz77 as *mut ::core::ffi::c_void) as *const *mut crate::types::size_t);
            let instart = *lz77_pos.add(lstart);
            let inend = instart + crate::src_lz77::ZopfliLZ77GetByteRange(lz77, lstart as crate::types::size_t, lend as crate::types::size_t);

            let mut s: crate::types::ZopfliBlockState = std::mem::zeroed();
            crate::src_lz77::ZopfliInitBlockState(options, instart as crate::types::size_t, inend as crate::types::size_t, 1, &mut s);
            crate::src_squeeze::ZopfliLZ77OptimalFixed(&mut s, lz77_data, instart as crate::types::size_t, inend as crate::types::size_t, &mut fixedstore);
            
            let fixedstore_size = *(crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(&mut fixedstore as *mut _ as *mut ::core::ffi::c_void) as *const crate::types::size_t);
            fixedcost = crate::src_deflate::ZopfliCalculateBlockSize(&fixedstore, 0, fixedstore_size, 1);
            crate::src_lz77::ZopfliCleanBlockState(&mut s);
        }

        if uncompressedcost < fixedcost && uncompressedcost < dyncost {
            crate::src_deflate::AddLZ77Block(options, 0, r#final, lz77, lstart, lend, expected_data_size, bp, out, outsize);
        } else if fixedcost < dyncost {
            if expensivefixed != 0 {
                let fixedstore_size = *(crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(&mut fixedstore as *mut _ as *mut ::core::ffi::c_void) as *const crate::types::size_t);
                crate::src_deflate::AddLZ77Block(options, 1, r#final, &fixedstore, 0, fixedstore_size as usize, expected_data_size, bp, out, outsize);
            } else {
                crate::src_deflate::AddLZ77Block(options, 1, r#final, lz77, lstart, lend, expected_data_size, bp, out, outsize);
            }
        } else {
            crate::src_deflate::AddLZ77Block(options, 2, r#final, lz77, lstart, lend, expected_data_size, bp, out, outsize);
        }

        crate::src_lz77::ZopfliCleanLZ77Store(&mut fixedstore);
    }
}

pub extern "C" fn ZopfliDeflatePart(options: *const crate::types::ZopfliOptions, btype: ::core::ffi::c_int, final_: ::core::ffi::c_int, in_: *const ::core::ffi::c_uchar, instart: crate::types::size_t, inend: crate::types::size_t, bp: *mut ::core::ffi::c_uchar, out: *mut *mut ::core::ffi::c_uchar, outsize: *mut crate::types::size_t) {
    let mut splitpoints_uncompressed: *mut crate::types::size_t = std::ptr::null_mut();
    let mut npoints: crate::types::size_t = 0;
    let mut splitpoints: *mut crate::types::size_t = std::ptr::null_mut();
    let mut totalcost: f64 = 0.0;
    let mut lz77: crate::types::ZopfliLZ77Store = unsafe { std::mem::zeroed() };

    if btype == 0 {
        crate::src_deflate::AddNonCompressedBlock(options, final_ as i32, in_, instart, inend, bp, out, outsize as *mut usize);
        return;
    } else if btype == 1 {
        let mut store: crate::types::ZopfliLZ77Store = unsafe { std::mem::zeroed() };
        let mut s: crate::types::ZopfliBlockState = unsafe { std::mem::zeroed() };
        crate::src_lz77::ZopfliInitLZ77Store(in_, &mut store);
        crate::src_lz77::ZopfliInitBlockState(options, instart, inend, 1, &mut s);
        crate::src_squeeze::ZopfliLZ77OptimalFixed(&mut s, in_, instart, inend, &mut store);
        let store_size = unsafe { *(crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(&store as *const _ as *mut ::core::ffi::c_void) as *const crate::types::size_t) };
        crate::src_deflate::AddLZ77Block(options, btype as i32, final_ as i32, &store, 0, store_size, 0, bp, out, outsize as *mut usize);
        crate::src_lz77::ZopfliCleanBlockState(&mut s);
        crate::src_lz77::ZopfliCleanLZ77Store(&mut store);
        return;
    }

    let blocksplitting = unsafe { *(crate::compat::c2r_field_ptr_ZopfliOptions__blocksplitting(options as *mut ::core::ffi::c_void) as *const ::core::ffi::c_int) };
    if blocksplitting != 0 {
        let blocksplittingmax = unsafe { *(crate::compat::c2r_field_ptr_ZopfliOptions__blocksplittingmax(options as *mut ::core::ffi::c_void) as *const ::core::ffi::c_int) };
        crate::src_blocksplitter::ZopfliBlockSplit(options, in_, instart, inend, blocksplittingmax as crate::types::size_t, &mut splitpoints_uncompressed, &mut npoints);
        splitpoints = unsafe { libc::malloc(std::mem::size_of::<crate::types::size_t>() * npoints) } as *mut crate::types::size_t;
    }

    crate::src_lz77::ZopfliInitLZ77Store(in_, &mut lz77);

    let mut i: crate::types::size_t = 0;
    while i <= npoints {
        let start = if i == 0 { instart } else { unsafe { *splitpoints_uncompressed.add(i - 1) } };
        let end = if i == npoints { inend } else { unsafe { *splitpoints_uncompressed.add(i) } };
        let mut s: crate::types::ZopfliBlockState = unsafe { std::mem::zeroed() };
        let mut store: crate::types::ZopfliLZ77Store = unsafe { std::mem::zeroed() };
        crate::src_lz77::ZopfliInitLZ77Store(in_, &mut store);
        crate::src_lz77::ZopfliInitBlockState(options, start, end, 1, &mut s);
        let numiterations = unsafe { *(crate::compat::c2r_field_ptr_ZopfliOptions__numiterations(options as *mut ::core::ffi::c_void) as *const ::core::ffi::c_int) };
        crate::src_squeeze::ZopfliLZ77Optimal(&mut s, in_, start, end, numiterations, &mut store);
        let store_size = unsafe { *(crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(&store as *const _ as *mut ::core::ffi::c_void) as *const crate::types::size_t) };
        totalcost += crate::src_deflate::ZopfliCalculateBlockSizeAutoType(&store, 0, store_size);
        crate::src_lz77::ZopfliAppendLZ77Store(&store, &mut lz77);
        if i < npoints {
            let lz77_size = unsafe { *(crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(&lz77 as *const _ as *mut ::core::ffi::c_void) as *const crate::types::size_t) };
            unsafe { *splitpoints.add(i) = lz77_size };
        }
        crate::src_lz77::ZopfliCleanBlockState(&mut s);
        crate::src_lz77::ZopfliCleanLZ77Store(&mut store);
        i += 1;
    }

    if blocksplitting != 0 && npoints > 1 {
        let mut splitpoints2: *mut crate::types::size_t = std::ptr::null_mut();
        let mut npoints2: crate::types::size_t = 0;
        let mut totalcost2: f64 = 0.0;
        let blocksplittingmax = unsafe { *(crate::compat::c2r_field_ptr_ZopfliOptions__blocksplittingmax(options as *mut ::core::ffi::c_void) as *const ::core::ffi::c_int) };
        crate::src_blocksplitter::ZopfliBlockSplitLZ77(options, &lz77, blocksplittingmax as crate::types::size_t, &mut splitpoints2, &mut npoints2);
        let lz77_size = unsafe { *(crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(&lz77 as *const _ as *mut ::core::ffi::c_void) as *const crate::types::size_t) };
        i = 0;
        while i <= npoints2 {
            let start = if i == 0 { 0 } else { unsafe { *splitpoints2.add(i - 1) } };
            let end = if i == npoints2 { lz77_size } else { unsafe { *splitpoints2.add(i) } };
            totalcost2 += crate::src_deflate::ZopfliCalculateBlockSizeAutoType(&lz77, start, end);
            i += 1;
        }
        if totalcost2 < totalcost {
            unsafe { libc::free(splitpoints as *mut ::core::ffi::c_void) };
            splitpoints = splitpoints2;
            npoints = npoints2;
        } else {
            unsafe { libc::free(splitpoints2 as *mut ::core::ffi::c_void) };
        }
    }

    let lz77_size = unsafe { *(crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(&lz77 as *const _ as *mut ::core::ffi::c_void) as *const crate::types::size_t) };
    i = 0;
    while i <= npoints {
        let start = if i == 0 { 0 } else { unsafe { *splitpoints.add(i - 1) } };
        let end = if i == npoints { lz77_size } else { unsafe { *splitpoints.add(i) } };
        let is_final = if i == npoints && final_ != 0 { 1 } else { 0 };
        crate::src_deflate::AddLZ77BlockAutoType(options, is_final, &lz77, start, end, 0, bp, out, outsize as *mut usize);
        i += 1;
    }

    crate::src_lz77::ZopfliCleanLZ77Store(&mut lz77);
    unsafe { libc::free(splitpoints as *mut ::core::ffi::c_void) };
    unsafe { libc::free(splitpoints_uncompressed as *mut ::core::ffi::c_void) };
}

pub extern "C" fn ZopfliDeflate(options: *const crate::types::ZopfliOptions, btype: ::core::ffi::c_int, final_: ::core::ffi::c_int, in_: *const ::core::ffi::c_uchar, insize: crate::types::size_t, bp: *mut ::core::ffi::c_uchar, out: *mut *mut ::core::ffi::c_uchar, outsize: *mut crate::types::size_t) {
    let offset: crate::types::size_t = unsafe { *outsize };
    
    let mut i: crate::types::size_t = 0;
    loop {
        let masterfinal: ::core::ffi::c_int = if i + 1000000 >= insize { 1 } else { 0 };
        let final2: ::core::ffi::c_int = if final_ != 0 && masterfinal != 0 { 1 } else { 0 };
        let size: crate::types::size_t = if masterfinal != 0 { insize - i } else { 1000000 };
        
        crate::src_deflate::ZopfliDeflatePart(
            options,
            btype,
            final2,
            in_,
            i,
            i + size,
            bp,
            out,
            outsize
        );
        
        i += size;
        
        if i >= insize {
            break;
        }
    }
    
    let verbose_ptr = unsafe {
        crate::compat::c2r_field_ptr_ZopfliOptions__verbose(options as *mut ::core::ffi::c_void)
    };
    let verbose = unsafe { *(verbose_ptr as *const ::core::ffi::c_int) };
    
    if verbose != 0 {
        unsafe {
            extern "C" {
                static stderr: *mut libc::FILE;
            }
            libc::fprintf(
                stderr,
                b"Original Size: %lu, Deflate: %lu, Compression: %f%% Removed\n\0".as_ptr() as *const ::core::ffi::c_char,
                insize as ::core::ffi::c_ulong,
                (*outsize - offset) as ::core::ffi::c_ulong,
                100.0 * (insize as f64 - (*outsize - offset) as f64) / (insize as f64)
            );
        }
    }
}
