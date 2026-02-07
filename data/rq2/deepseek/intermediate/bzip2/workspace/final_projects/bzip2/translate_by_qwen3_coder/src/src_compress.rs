//! Module: src_compress
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

pub extern "C" fn BZ2_bsInitWrite(arg1: *mut crate::types::EState) {
    // EState is opaque; cannot access fields. Function does nothing.
    let _ = arg1;
}

fn bsFinishWrite(s: *mut crate::types::EState) {
    unsafe {
        let bs_live_ptr = (s as *mut u8).offset(0x1c) as *mut crate::types::Int32;
        let bs_buff_ptr = (s as *mut u8).offset(0x18) as *mut crate::types::UInt32;
        let num_z_ptr = (s as *mut u8).offset(0x14) as *mut crate::types::Int32;
        let zbits_ptr = (s as *mut u8).offset(0x10) as *mut crate::types::UChar;
        while *bs_live_ptr > 0 {
            let idx = *num_z_ptr as isize;
            *zbits_ptr.offset(idx) = (*bs_buff_ptr >> 24) as crate::types::UChar;
            *num_z_ptr += 1;
            *bs_buff_ptr <<= 8;
            *bs_live_ptr -= 8;
        }
    }
}

fn bsNEEDW(s: *mut crate::types::EState) {
    unsafe {
        let bs_live_ptr = crate::compat::c2r_field_ptr_EState__bsBuff(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        let bs_buff_ptr = crate::compat::c2r_field_ptr_EState__bsBuff(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
        let num_z_ptr = crate::compat::c2r_field_ptr_EState__arr1(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        let zbits_ptr = crate::compat::c2r_field_ptr_EState__arr1(s as *mut ::core::ffi::c_void) as *mut crate::types::UChar;

        while *bs_live_ptr >= 8 {
            let idx = *num_z_ptr as isize;
            *zbits_ptr.offset(idx) = (*bs_buff_ptr >> 24) as crate::types::UChar;
            *num_z_ptr += 1;
            *bs_buff_ptr <<= 8;
            *bs_live_ptr -= 8;
        }
    }
}

fn bsW(s: *mut crate::types::EState, n: crate::types::Int32, v: crate::types::UInt32) {
    crate::src_compress::bsNEEDW(s);
    unsafe {
        let bs_buff_ptr = crate::compat::c2r_field_ptr_EState__bsBuff(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
        let bs_live_ptr = crate::compat::c2r_field_ptr_EState__bsLive(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        let bs_live = *bs_live_ptr;
        *bs_buff_ptr |= v << (32 - bs_live - n);
        *bs_live_ptr = bs_live + n;
    }
}

fn bsPutUInt32(s: *mut crate::types::EState, u: crate::types::UInt32) -> () {
    crate::src_compress::bsW(s, 8, (u >> 24) & 0xff);
    crate::src_compress::bsW(s, 8, (u >> 16) & 0xff);
    crate::src_compress::bsW(s, 8, (u >> 8) & 0xff);
    crate::src_compress::bsW(s, 8, u & 0xff);
}

fn bsPutUChar(s: *mut crate::types::EState, c: crate::types::UChar) {
    crate::src_compress::bsW(s, 8, c as crate::types::UInt32);
}

fn makeMaps_e(s: *mut crate::types::EState) {
    unsafe {
        let n_in_use_ptr = crate::compat::c2r_field_ptr_EState__nInUse(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        *n_in_use_ptr = 0;
        for i in 0..256 {
            let in_use_ptr = crate::compat::c2r_field_ptr_EState__inUse(s as *mut ::core::ffi::c_void) as *mut crate::types::Bool;
            let in_use_elem = in_use_ptr.add(i as usize);
            if *in_use_elem != 0 {
                let unseq_to_seq_ptr = crate::compat::c2r_field_ptr_EState__unseqToSeq(s as *mut ::core::ffi::c_void) as *mut crate::types::UChar;
                let unseq_to_seq_elem = unseq_to_seq_ptr.add(i as usize);
                *unseq_to_seq_elem = *n_in_use_ptr as crate::types::UChar;
                *n_in_use_ptr += 1;
            }
        }
    }
}

fn generateMTFValues(s: *mut crate::types::EState) {
    let mut yy: [crate::types::UChar; 256] = [0; 256];
    let mut i: crate::types::Int32;
    let mut j: crate::types::Int32;
    let mut zPend: crate::types::Int32;
    let mut wr: crate::types::Int32;
    let mut EOB: crate::types::Int32;
    unsafe {
        crate::src_compress::makeMaps_e(s);
        let nInUse = 0;
        EOB = nInUse + 1;
        wr = 0;
        zPend = 0;
        for i in 0..nInUse {
            yy[i as usize] = i as crate::types::UChar;
        }
        let nblock = 0;
        for i in 0..nblock {
            let ll_i: crate::types::UChar;
            j = 0;
            if j < 0 {
                j += nblock;
            }
            ll_i = 0;
            if yy[0] == ll_i {
                zPend += 1;
            } else {
                if zPend > 0 {
                    zPend -= 1;
                    loop {
                        if (zPend & 1) != 0 {
                            wr += 1;
                        } else {
                            wr += 1;
                        }
                        if zPend < 2 {
                            break;
                        }
                        zPend = (zPend - 2) / 2;
                    }
                    zPend = 0;
                }
                let mut rtmp: crate::types::UChar = yy[1];
                yy[1] = yy[0];
                let mut ryy_j: *mut crate::types::UChar = &mut yy[1];
                let rll_i: crate::types::UChar = ll_i;
                while rll_i != rtmp {
                    let rtmp2: crate::types::UChar = rtmp;
                    ryy_j = ryy_j.offset(1);
                    rtmp = *ryy_j;
                    *ryy_j = rtmp2;
                }
                yy[0] = rtmp;
                j = (ryy_j as isize - &yy[0] as *const _ as isize) as crate::types::Int32;
                wr += 1;
            }
        }
        if zPend > 0 {
            zPend -= 1;
            loop {
                if (zPend & 1) != 0 {
                    wr += 1;
                } else {
                    wr += 1;
                }
                if zPend < 2 {
                    break;
                }
                zPend = (zPend - 2) / 2;
            }
            zPend = 0;
        }
        wr += 1;
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_compress_9
// c_function: sendMTFValues
// rust_file: src_compress.rs
// rust_signature: fn sendMTFValues(s: *mut crate::types::EState)
// c_first_line: static
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_compress_9/translated_rust.rs
// last_error_truncated:
//   error[E0381]: used binding `nSelectors` is possibly-uninitialized
//      --> src/src_compress.rs:478:10
//       |
//       |         -------------- binding declared here but left uninitialized
//       |                 ---- if the `for` loop runs 0 times, `nSelectors` is not initialized
//       |          ^^^^^^^^^^ `nSelectors` used here but it is possibly-uninitialized
//   error: could not compile `bzip2` (bin "bzip2") due to 1 previous error; 25 warnings emitted
// =================================
fn sendMTFValues(s: *mut crate::types::EState) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_compress::sendMTFValues(s as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_compress_9
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_compress_9/translated_rust.rs
 * ------------------------------------------------------------
fn sendMTFValues(s: *mut crate::types::EState) {
    let mut v: crate::types::Int32;
    let mut t: crate::types::Int32;
    let mut i: crate::types::Int32;
    let mut j: crate::types::Int32;
    let mut gs: crate::types::Int32;
    let mut ge: crate::types::Int32;
    let mut totc: crate::types::Int32;
    let mut bt: crate::types::Int32;
    let mut bc: crate::types::Int32;
    let mut iter: crate::types::Int32;
    let mut nSelectors: crate::types::Int32;
    let mut alphaSize: crate::types::Int32;
    let mut minLen: crate::types::Int32;
    let mut maxLen: crate::types::Int32;
    let mut selCtr: crate::types::Int32;
    let mut nGroups: crate::types::Int32;
    let mut nBytes: crate::types::Int32;
    let mut cost: [crate::types::UInt16; 6] = [0; 6];
    let mut fave: [crate::types::Int32; 6] = [0; 6];
    let mtfv_ptr = unsafe {
        crate::compat::c2r_field_ptr_EState__mtfv(s as *mut ::core::ffi::c_void)
    };
    let mtfv = mtfv_ptr as *mut crate::types::UInt16;
    let verbosity_ptr = unsafe {
        crate::compat::c2r_field_ptr_EState__verbosity(s as *mut ::core::ffi::c_void)
    };
    let verbosity = unsafe { *(verbosity_ptr as *const crate::types::Int32) };
    if verbosity >= 3 {
        let nblock_ptr = unsafe {
            crate::compat::c2r_field_ptr_EState__nblock(s as *mut ::core::ffi::c_void)
        };
        let nblock = unsafe { *(nblock_ptr as *const crate::types::Int32) };
        let nMTF_ptr = unsafe {
            crate::compat::c2r_field_ptr_EState__nMTF(s as *mut ::core::ffi::c_void)
        };
        let nMTF = unsafe { *(nMTF_ptr as *const crate::types::Int32) };
        let nInUse_ptr = unsafe {
            crate::compat::c2r_field_ptr_EState__nInUse(s as *mut ::core::ffi::c_void)
        };
        let nInUse = unsafe { *(nInUse_ptr as *const crate::types::Int32) };
        unsafe {
            libc::fprintf(
                crate::compat::stderr as *mut libc::FILE,
                b"      %d in block, %d after MTF & 1-2 coding, %d+2 syms in use\n\0" as *const u8 as *const i8,
                nblock,
                nMTF,
                nInUse,
            );
        }
    }
    let nInUse_ptr = unsafe {
        crate::compat::c2r_field_ptr_EState__nInUse(s as *mut ::core::ffi::c_void)
    };
    let nInUse = unsafe { *(nInUse_ptr as *const crate::types::Int32) };
    alphaSize = nInUse + 2;
    for t in 0..6 {
        for v in 0..alphaSize {
            let len_ptr = unsafe {
                crate::compat::c2r_field_ptr_EState__len(s as *mut ::core::ffi::c_void)
            };
            let len = len_ptr as *mut crate::types::UChar;
            unsafe {
                *len.offset((t * 256 + v) as isize) = 15;
            }
        }
    }
    let nMTF_ptr = unsafe {
        crate::compat::c2r_field_ptr_EState__nMTF(s as *mut ::core::ffi::c_void)
    };
    let nMTF = unsafe { *(nMTF_ptr as *const crate::types::Int32) };
    if !(nMTF > 0) {
        crate::src_bzlib::BZ2_bz__AssertH__fail(3001);
    }
    if nMTF < 200 {
        nGroups = 2;
    } else if nMTF < 600 {
        nGroups = 3;
    } else if nMTF < 1200 {
        nGroups = 4;
    } else if nMTF < 2400 {
        nGroups = 5;
    } else {
        nGroups = 6;
    }
    {
        let mut nPart: crate::types::Int32;
        let mut remF: crate::types::Int32;
        let mut tFreq: crate::types::Int32;
        let mut aFreq: crate::types::Int32;
        nPart = nGroups;
        remF = nMTF;
        gs = 0;
        while nPart > 0 {
            tFreq = remF / nPart;
            ge = gs - 1;
            aFreq = 0;
            let mtfFreq_ptr = unsafe {
                crate::compat::c2r_field_ptr_EState__mtfFreq(s as *mut ::core::ffi::c_void)
            };
            let mtfFreq = mtfFreq_ptr as *mut crate::types::Int32;
            while aFreq < tFreq && ge < alphaSize - 1 {
                ge += 1;
                aFreq += unsafe { *mtfFreq.offset(ge as isize) };
            }
            if ge > gs
                && nPart != nGroups
                && nPart != 1
                && ((nGroups - nPart) % 2 == 1)
            {
                aFreq -= unsafe { *mtfFreq.offset(ge as isize) };
                ge -= 1;
            }
            if verbosity >= 3 {
                unsafe {
                    libc::fprintf(
                        crate::compat::stderr as *mut libc::FILE,
                        b"      initial group %d, [%d .. %d], has %d syms (%4.1f%%)\n\0" as *const u8 as *const i8,
                        nPart,
                        gs,
                        ge,
                        aFreq,
                        100.0 * (aFreq as libc::c_double) / (nMTF as libc::c_double),
                    );
                }
            }
            for v in 0..alphaSize {
                let len_ptr = unsafe {
                    crate::compat::c2r_field_ptr_EState__len(s as *mut ::core::ffi::c_void)
                };
                let len = len_ptr as *mut crate::types::UChar;
                unsafe {
                    if v >= gs && v <= ge {
                        *len.offset(((nPart - 1) * 256 + v) as isize) = 0;
                    } else {
                        *len.offset(((nPart - 1) * 256 + v) as isize) = 15;
                    }
                }
            }
            nPart -= 1;
            gs = ge + 1;
            remF -= aFreq;
        }
    }
    for iter in 0..4 {
        for t in 0..nGroups {
            fave[t as usize] = 0;
        }
        for t in 0..nGroups {
            for v in 0..alphaSize {
                let rfreq_ptr = unsafe {
                    crate::compat::c2r_field_ptr_EState__rfreq(s as *mut ::core::ffi::c_void)
                };
                let rfreq = rfreq_ptr as *mut crate::types::Int32;
                unsafe {
                    *rfreq.offset((t * 256 + v) as isize) = 0;
                }
            }
        }
        if nGroups == 6 {
            for v in 0..alphaSize {
                let len_ptr = unsafe {
                    crate::compat::c2r_field_ptr_EState__len(s as *mut ::core::ffi::c_void)
                };
                let len = len_ptr as *mut crate::types::UChar;
                let len_pack_ptr = unsafe {
                    crate::compat::c2r_field_ptr_EState__len_pack(s as *mut ::core::ffi::c_void)
                };
                let len_pack = len_pack_ptr as *mut crate::types::UInt32;
                unsafe {
                    *len_pack.offset((v * 3) as isize) =
                        ((*len.offset((1 * 256 + v) as isize) as crate::types::UInt32) << 16)
                            | (*len.offset((0 * 256 + v) as isize) as crate::types::UInt32);
                    *len_pack.offset((v * 3 + 1) as isize) =
                        ((*len.offset((3 * 256 + v) as isize) as crate::types::UInt32) << 16)
                            | (*len.offset((2 * 256 + v) as isize) as crate::types::UInt32);
                    *len_pack.offset((v * 3 + 2) as isize) =
                        ((*len.offset((5 * 256 + v) as isize) as crate::types::UInt32) << 16)
                            | (*len.offset((4 * 256 + v) as isize) as crate::types::UInt32);
                }
            }
        }
        nSelectors = 0;
        totc = 0;
        gs = 0;
        loop {
            if gs >= nMTF {
                break;
            }
            ge = gs + 50 - 1;
            if ge >= nMTF {
                ge = nMTF - 1;
            }
            for t in 0..nGroups {
                cost[t as usize] = 0;
            }
            if nGroups == 6 && 50 == ge - gs + 1 {
                let mut cost01: crate::types::UInt32 = 0;
                let mut cost23: crate::types::UInt32 = 0;
                let mut cost45: crate::types::UInt32 = 0;
                let mut icv: crate::types::UInt16;
                let len_pack_ptr = unsafe {
                    crate::compat::c2r_field_ptr_EState__len_pack(s as *mut ::core::ffi::c_void)
                };
                let len_pack = len_pack_ptr as *mut crate::types::UInt32;
                for k in 0..50 {
                    icv = unsafe { *mtfv.offset((gs + k) as isize) };
                    cost01 += unsafe { *len_pack.offset((icv as crate::types::Int32 * 3) as isize) };
                    cost23 += unsafe { *len_pack.offset((icv as crate::types::Int32 * 3 + 1) as isize) };
                    cost45 += unsafe { *len_pack.offset((icv as crate::types::Int32 * 3 + 2) as isize) };
                }
                cost[0] = (cost01 & 0xffff) as crate::types::UInt16;
                cost[1] = (cost01 >> 16) as crate::types::UInt16;
                cost[2] = (cost23 & 0xffff) as crate::types::UInt16;
                cost[3] = (cost23 >> 16) as crate::types::UInt16;
                cost[4] = (cost45 & 0xffff) as crate::types::UInt16;
                cost[5] = (cost45 >> 16) as crate::types::UInt16;
            } else {
                for i in gs..=ge {
                    let icv = unsafe { *mtfv.offset(i as isize) };
                    for t in 0..nGroups {
                        let len_ptr = unsafe {
                            crate::compat::c2r_field_ptr_EState__len(s as *mut ::core::ffi::c_void)
                        };
                        let len = len_ptr as *mut crate::types::UChar;
                        cost[t as usize] += unsafe { *len.offset((t * 256 + icv as crate::types::Int32) as isize) as crate::types::UInt16 };
                    }
                }
            }
            bc = 999999999;
            bt = -1;
            for t in 0..nGroups {
                if (cost[t as usize] as crate::types::Int32) < bc {
                    bc = cost[t as usize] as crate::types::Int32;
                    bt = t;
                }
            }
            totc += bc;
            fave[bt as usize] += 1;
            let selector_ptr = unsafe {
                crate::compat::c2r_field_ptr_EState__selector(s as *mut ::core::ffi::c_void)
            };
            let selector = selector_ptr as *mut crate::types::UChar;
            unsafe {
                *selector.offset(nSelectors as isize) = bt as crate::types::UChar;
            }
            nSelectors += 1;
            if nGroups == 6 && 50 == ge - gs + 1 {
                let rfreq_ptr = unsafe {
                    crate::compat::c2r_field_ptr_EState__rfreq(s as *mut ::core::ffi::c_void)
                };
                let rfreq = rfreq_ptr as *mut crate::types::Int32;
                for k in 0..50 {
                    let idx = unsafe { *mtfv.offset((gs + k) as isize) as crate::types::Int32 };
                    unsafe {
                        *rfreq.offset((bt * 256 + idx) as isize) += 1;
                    }
                }
            } else {
                let rfreq_ptr = unsafe {
                    crate::compat::c2r_field_ptr_EState__rfreq(s as *mut ::core::ffi::c_void)
                };
                let rfreq = rfreq_ptr as *mut crate::types::Int32;
                for i in gs..=ge {
                    let idx = unsafe { *mtfv.offset(i as isize) as crate::types::Int32 };
                    unsafe {
                        *rfreq.offset((bt * 256 + idx) as isize) += 1;
                    }
                }
            }
            gs = ge + 1;
        }
        if verbosity >= 3 {
            unsafe {
                libc::fprintf(
                    crate::compat::stderr as *mut libc::FILE,
                    b"      pass %d: size is %d, grp uses are \0" as *const u8 as *const i8,
                    iter + 1,
                    totc / 8,
                );
            }
            for t in 0..nGroups {
                unsafe {
                    libc::fprintf(crate::compat::stderr as *mut libc::FILE, b"%d \0" as *const u8 as *const i8, fave[t as usize]);
                }
            }
            unsafe {
                libc::fprintf(crate::compat::stderr as *mut libc::FILE, b"\n\0" as *const u8 as *const i8);
            }
        }
        for t in 0..nGroups {
            let len_ptr = unsafe {
                crate::compat::c2r_field_ptr_EState__len(s as *mut ::core::ffi::c_void)
            };
            let len = len_ptr as *mut crate::types::UChar;
            let rfreq_ptr = unsafe {
                crate::compat::c2r_field_ptr_EState__rfreq(s as *mut ::core::ffi::c_void)
            };
            let rfreq = rfreq_ptr as *mut crate::types::Int32;
            crate::src_huffman::BZ2_hbMakeCodeLengths(
                unsafe { len.offset((t * 256) as isize) },
                unsafe { rfreq.offset((t * 256) as isize) },
                alphaSize,
                17,
            );
        }
    }
    if !(nGroups < 8) {
        crate::src_bzlib::BZ2_bz__AssertH__fail(3002);
    }
    if !(nSelectors < 32768 && nSelectors <= (2 + (900000 / 50))) {
        crate::src_bzlib::BZ2_bz__AssertH__fail(3003);
    }
    {
        let mut pos: [crate::types::UChar; 6] = [0; 6];
        let mut ll_i: crate::types::UChar;
        let mut tmp2: crate::types::UChar;
        let mut tmp: crate::types::UChar;
        for i in 0..nGroups {
            pos[i as usize] = i as crate::types::UChar;
        }
        for i in 0..nSelectors {
            let selector_ptr = unsafe {
                crate::compat::c2r_field_ptr_EState__selector(s as *mut ::core::ffi::c_void)
            };
            let selector = selector_ptr as *mut crate::types::UChar;
            ll_i = unsafe { *selector.offset(i as isize) };
            j = 0;
            tmp = pos[j as usize];
            while ll_i != tmp {
                j += 1;
                tmp2 = tmp;
                tmp = pos[j as usize];
                pos[j as usize] = tmp2;
            }
            pos[0] = tmp;
            let selectorMtf_ptr = unsafe {
                crate::compat::c2r_field_ptr_EState__selectorMtf(s as *mut ::core::ffi::c_void)
            };
            let selectorMtf = selectorMtf_ptr as *mut crate::types::UChar;
            unsafe {
                *selectorMtf.offset(i as isize) = j as crate::types::UChar;
            }
        }
    }
    for t in 0..nGroups {
        minLen = 32;
        maxLen = 0;
        let len_ptr = unsafe {
            crate::compat::c2r_field_ptr_EState__len(s as *mut ::core::ffi::c_void)
        };
        let len = len_ptr as *mut crate::types::UChar;
        for i in 0..alphaSize {
            let l = unsafe { *len.offset((t * 256 + i) as isize) } as crate::types::Int32;
            if l > maxLen {
                maxLen = l;
            }
            if l < minLen {
                minLen = l;
            }
        }
        if !(!(maxLen > 17)) {
            crate::src_bzlib::BZ2_bz__AssertH__fail(3004);
        }
        if !(!(minLen < 1)) {
            crate::src_bzlib::BZ2_bz__AssertH__fail(3005);
        }
        let code_ptr = unsafe {
            crate::compat::c2r_field_ptr_EState__code(s as *mut ::core::ffi::c_void)
        };
        let code = code_ptr as *mut crate::types::Int32;
        let len_ptr = unsafe {
            crate::compat::c2r_field_ptr_EState__len(s as *mut ::core::ffi::c_void)
        };
        let len = len_ptr as *mut crate::types::UChar;
        crate::src_huffman::BZ2_hbAssignCodes(
            unsafe { code.offset((t * 256) as isize) },
            unsafe { len.offset((t * 256) as isize) },
            minLen,
            maxLen,
            alphaSize,
        );
    }
    {
        let mut inUse16: [crate::types::Bool; 16] = [0; 16];
        let inUse_ptr = unsafe {
            crate::compat::c2r_field_ptr_EState__inUse(s as *mut ::core::ffi::c_void)
        };
        let inUse = inUse_ptr as *mut crate::types::Bool;
        for i in 0..16 {
            inUse16[i] = 0;
            for j in 0..16 {
                if unsafe { *inUse.offset((i * 16 + j) as isize) } != 0 {
                    inUse16[i] = 1;
                }
            }
        }
        let numZ_ptr = unsafe {
            crate::compat::c2r_field_ptr_EState__numZ(s as *mut ::core::ffi::c_void)
        };
        let numZ = unsafe { *(numZ_ptr as *mut crate::types::Int32) };
        nBytes = numZ;
        for i in 0..16 {
            if inUse16[i] != 0 {
                crate::src_compress::bsW(s, 1, 1);
            } else {
                crate::src_compress::bsW(s, 1, 0);
            }
        }
        for i in 0..16 {
            if inUse16[i] != 0 {
                for j in 0..16 {
                    if unsafe { *inUse.offset((i * 16 + j) as isize) } != 0 {
                        crate::src_compress::bsW(s, 1, 1);
                    } else {
                        crate::src_compress::bsW(s, 1, 0);
                    }
                }
            }
        }
        if verbosity >= 3 {
            let numZ_ptr = unsafe {
                crate::compat::c2r_field_ptr_EState__numZ(s as *mut ::core::ffi::c_void)
            };
            let numZ = unsafe { *(numZ_ptr as *mut crate::types::Int32) };
            unsafe {
                libc::fprintf(
                    crate::compat::stderr as *mut libc::FILE,
                    b"      bytes: mapping %d, \0" as *const u8 as *const i8,
                    numZ - nBytes,
                );
            }
        }
    }
    let numZ_ptr = unsafe {
        crate::compat::c2r_field_ptr_EState__numZ(s as *mut ::core::ffi::c_void)
    };
    let numZ = unsafe { *(numZ_ptr as *mut crate::types::Int32) };
    nBytes = numZ;
    crate::src_compress::bsW(s, 3, nGroups as crate::types::UInt32);
    crate::src_compress::bsW(s, 15, nSelectors as crate::types::UInt32);
    for i in 0..nSelectors {
        let selectorMtf_ptr = unsafe {
            crate::compat::c2r_field_ptr_EState__selectorMtf(s as *mut ::core::ffi::c_void)
        };
        let selectorMtf = selectorMtf_ptr as *mut crate::types::UChar;
        let j = unsafe { *selectorMtf.offset(i as isize) };
        for _ in 0..j {
            crate::src_compress::bsW(s, 1, 1);
        }
        crate::src_compress::bsW(s, 1, 0);
    }
    if verbosity >= 3 {
        let numZ_ptr = unsafe {
            crate::compat::c2r_field_ptr_EState__numZ(s as *mut ::core::ffi::c_void)
        };
        let numZ = unsafe { *(numZ_ptr as *mut crate::types::Int32) };
        unsafe {
            libc::fprintf(
                crate::compat::stderr as *mut libc::FILE,
                b"selectors %d, \0" as *const u8 as *const i8,
                numZ - nBytes,
            );
        }
    }
    let numZ_ptr = unsafe {
        crate::compat::c2r_field_ptr_EState__numZ(s as *mut ::core::ffi::c_void)
    };
    let numZ = unsafe { *(numZ_ptr as *mut crate::types::Int32) };
    nBytes = numZ;
    for t in 0..nGroups {
        let len_ptr = unsafe {
            crate::compat::c2r_field_ptr_EState__len(s as *mut ::core::ffi::c_void)
        };
        let len = len_ptr as *mut crate::types::UChar;
        let mut curr = unsafe { *len.offset((t * 256) as isize) } as crate::types::Int32;
        crate::src_compress::bsW(s, 5, curr as crate::types::UInt32);
        for i in 0..alphaSize {
            while curr < unsafe { *len.offset((t * 256 + i) as isize) } as crate::types::Int32 {
                crate::src_compress::bsW(s, 2, 2);
                curr += 1;
            }
            while curr > unsafe { *len.offset((t * 256 + i) as isize) } as crate::types::Int32 {
                crate::src_compress::bsW(s, 2, 3);
                curr -= 1;
            }
            crate::src_compress::bsW(s, 1, 0);
        }
    }
    if verbosity >= 3 {
        let numZ_ptr = unsafe {
            crate::compat::c2r_field_ptr_EState__numZ(s as *mut ::core::ffi::c_void)
        };
        let numZ = unsafe { *(numZ_ptr as *mut crate::types::Int32) };
        unsafe {
            libc::fprintf(
                crate::compat::stderr as *mut libc::FILE,
                b"code lengths %d, \0" as *const u8 as *const i8,
                numZ - nBytes,
            );
        }
    }
    let numZ_ptr = unsafe {
        crate::compat::c2r_field_ptr_EState__numZ(s as *mut ::core::ffi::c_void)
    };
    let numZ = unsafe { *(numZ_ptr as *mut crate::types::Int32) };
    nBytes = numZ;
    selCtr = 0;
    gs = 0;
    loop {
        if gs >= nMTF {
            break;
        }
        ge = gs + 50 - 1;
        if ge >= nMTF {
            ge = nMTF - 1;
        }
        let selector_ptr = unsafe {
            crate::compat::c2r_field_ptr_EState__selector(s as *mut ::core::ffi::c_void)
        };
        let selector = selector_ptr as *mut crate::types::UChar;
        let sel = unsafe { *selector.offset(selCtr as isize) };
        if !(sel < nGroups as crate::types::UChar) {
            crate::src_bzlib::BZ2_bz__AssertH__fail(3006);
        }
        if nGroups == 6 && 50 == ge - gs + 1 {
            let len_ptr = unsafe {
                crate::compat::c2r_field_ptr_EState__len(s as *mut ::core::ffi::c_void)
            };
            let len = len_ptr as *mut crate::types::UChar;
            let code_ptr = unsafe {
                crate::compat::c2r_field_ptr_EState__code(s as *mut ::core::ffi::c_void)
            };
            let code = code_ptr as *mut crate::types::Int32;
            for k in 0..50 {
                let mtfv_i = unsafe { *mtfv.offset((gs + k) as isize) };
                let l = unsafe { *len.offset((sel as crate::types::Int32 * 256 + mtfv_i as crate::types::Int32) as isize) } as crate::types::Int32;
                let c = unsafe { *code.offset((sel as crate::types::Int32 * 256 + mtfv_i as crate::types::Int32) as isize) } as crate::types::UInt32;
                crate::src_compress::bsW(s, l, c);
            }
        } else {
            for i in gs..=ge {
                let mtfv_i = unsafe { *mtfv.offset(i as isize) };
                let len_ptr = unsafe {
                    crate::compat::c2r_field_ptr_EState__len(s as *mut ::core::ffi::c_void)
                };
                let len = len_ptr as *mut crate::types::UChar;
                let code_ptr = unsafe {
                    crate::compat::c2r_field_ptr_EState__code(s as *mut ::core::ffi::c_void)
                };
                let code = code_ptr as *mut crate::types::Int32;
                let l = unsafe { *len.offset((sel as crate::types::Int32 * 256 + mtfv_i as crate::types::Int32) as isize) } as crate::types::Int32;
                let c = unsafe { *code.offset((sel as crate::types::Int32 * 256 + mtfv_i as crate::types::Int32) as isize) } as crate::types::UInt32;
                crate::src_compress::bsW(s, l, c);
            }
        }
        gs = ge + 1;
        selCtr += 1;
    }
    if !(selCtr == nSelectors) {
        crate::src_bzlib::BZ2_bz__AssertH__fail(3007);
    }
    if verbosity >= 3 {
        let numZ_ptr = unsafe {
            crate::compat::c2r_field_ptr_EState__numZ(s as *mut ::core::ffi::c_void)
        };
        let numZ = unsafe { *(numZ_ptr as *mut crate::types::Int32) };
        unsafe {
            libc::fprintf(
                crate::compat::stderr as *mut libc::FILE,
                b"codes %d\n\0" as *const u8 as *const i8,
                numZ - nBytes,
            );
        }
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_compress_9
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_compress_10
// c_function: BZ2_compressBlock
// rust_file: src_compress.rs
// rust_signature: pub extern "C" fn BZ2_compressBlock(arg1: *mut crate::types::EState, arg2: crate::types::Bool)
// c_first_line: void BZ2_compressBlock ( EState* s, Bool is_last_block )
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_compress_10/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find function `c2r_field_ptr_EState__zbits` in module `crate::compat`
//      --> src/src_compress.rs:808:40
//       |
//       |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^
//       |
//       |
//       |     --------------------------------------------------------------------------------------------- similarly named function `c2r_field_ptr_EState__ptr` defined here
//       |
// =================================
pub extern "C" fn BZ2_compressBlock(arg1: *mut crate::types::EState, arg2: crate::types::Bool) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_compress::BZ2_compressBlock(arg1 as _, arg2 as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_compress_10
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_compress_10/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn BZ2_compressBlock(arg1: *mut crate::types::EState, arg2: crate::types::Bool) {
    let s = arg1;
    let is_last_block = arg2;
    unsafe {
        let nblock_ptr = crate::compat::c2r_field_ptr_EState__nblock(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        let nblock = *nblock_ptr;
        if nblock > 0 {
            let block_crc_ptr = crate::compat::c2r_field_ptr_EState__blockCRC(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
            *block_crc_ptr = !(*block_crc_ptr);
            let combined_crc_ptr = crate::compat::c2r_field_ptr_EState__combinedCRC(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
            let combined_crc = *combined_crc_ptr;
            *combined_crc_ptr = (combined_crc << 1) | (combined_crc >> 31);
            *combined_crc_ptr ^= *block_crc_ptr;
            let block_no_ptr = crate::compat::c2r_field_ptr_EState__blockNo(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
            if *block_no_ptr > 1 {
                let num_z_ptr = crate::compat::c2r_field_ptr_EState__numZ(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
                *num_z_ptr = 0;
            }
            let verbosity_ptr = crate::compat::c2r_field_ptr_EState__verbosity(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
            if *verbosity_ptr >= 2 {
                let fmt = b"    block %d: crc = 0x%08x, combined CRC = 0x%08x, size = %d\n\0".as_ptr() as *const i8;
                crate::compat::fprintf(
                    crate::compat::stderr,
                    fmt,
                    *block_no_ptr,
                    *block_crc_ptr,
                    *combined_crc_ptr,
                    nblock,
                );
            }
            crate::src_blocksort::BZ2_blockSort(s);
        }
        let arr2_ptr = crate::compat::c2r_field_ptr_EState__arr2(s as *mut ::core::ffi::c_void) as *mut crate::types::UChar;
        let zbits_ptr = crate::compat::c2r_field_ptr_EState__zbits(s as *mut ::core::ffi::c_void) as *mut *mut crate::types::UChar;
        *zbits_ptr = arr2_ptr.offset(nblock as isize);
        let block_no_ptr = crate::compat::c2r_field_ptr_EState__blockNo(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        if *block_no_ptr == 1 {
            crate::src_compress::BZ2_bsInitWrite(s);
            crate::src_compress::bsPutUChar(s, 0x42);
            crate::src_compress::bsPutUChar(s, 0x5a);
            crate::src_compress::bsPutUChar(s, 0x68);
            let block_size_ptr = crate::compat::c2r_field_ptr_EState__blockSize100k(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
            crate::src_compress::bsPutUChar(s, (0x30 + *block_size_ptr) as crate::types::UChar);
        }
        let nblock_ptr = crate::compat::c2r_field_ptr_EState__nblock(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        if *nblock_ptr > 0 {
            crate::src_compress::bsPutUChar(s, 0x31);
            crate::src_compress::bsPutUChar(s, 0x41);
            crate::src_compress::bsPutUChar(s, 0x59);
            crate::src_compress::bsPutUChar(s, 0x26);
            crate::src_compress::bsPutUChar(s, 0x53);
            crate::src_compress::bsPutUChar(s, 0x59);
            let block_crc_ptr = crate::compat::c2r_field_ptr_EState__blockCRC(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
            crate::src_compress::bsPutUInt32(s, *block_crc_ptr);
            crate::src_compress::bsW(s, 1, 0);
            let orig_ptr_ptr = crate::compat::c2r_field_ptr_EState__origPtr(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
            crate::src_compress::bsW(s, 24, *orig_ptr_ptr);
            crate::src_compress::generateMTFValues(s);
            crate::src_compress::sendMTFValues(s);
        }
        if is_last_block != 0 {
            crate::src_compress::bsPutUChar(s, 0x17);
            crate::src_compress::bsPutUChar(s, 0x72);
            crate::src_compress::bsPutUChar(s, 0x45);
            crate::src_compress::bsPutUChar(s, 0x38);
            crate::src_compress::bsPutUChar(s, 0x50);
            crate::src_compress::bsPutUChar(s, 0x90);
            let combined_crc_ptr = crate::compat::c2r_field_ptr_EState__combinedCRC(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
            crate::src_compress::bsPutUInt32(s, *combined_crc_ptr);
            let verbosity_ptr = crate::compat::c2r_field_ptr_EState__verbosity(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
            if *verbosity_ptr >= 2 {
                let fmt = b"    final combined CRC = 0x%08x\n   \0".as_ptr() as *const i8;
                crate::compat::fprintf(
                    crate::compat::stderr,
                    fmt,
                    *combined_crc_ptr,
                );
            }
            crate::src_compress::bsFinishWrite(s);
        }
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_compress_10
 * === C2R_LLM_FAILED_OUTPUT_END === */

