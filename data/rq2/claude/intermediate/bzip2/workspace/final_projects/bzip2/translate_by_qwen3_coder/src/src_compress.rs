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
    unsafe {
        // Use accessor shims to access opaque struct fields
        let bsLive_ptr = crate::compat::c2r_field_ptr_EState__bsLive(
            arg1 as *mut ::core::ffi::c_void
        );
        *(bsLive_ptr as *mut crate::types::Int32) = 0;
        
        let bsBuff_ptr = crate::compat::c2r_field_ptr_EState__bsBuff(
            arg1 as *mut ::core::ffi::c_void
        );
        *(bsBuff_ptr as *mut crate::types::UInt32) = 0;
    }
}

fn bsFinishWrite(s: *mut crate::types::EState) {
    unsafe {
        let bsLive_ptr = crate::compat::c2r_field_ptr_EState__bsLive(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        let bsBuff_ptr = crate::compat::c2r_field_ptr_EState__bsBuff(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
        let numZ_ptr = crate::compat::c2r_field_ptr_EState__numZ(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        let zbits_ptr = crate::compat::c2r_field_ptr_EState__zbits(s as *mut ::core::ffi::c_void) as *mut *mut crate::types::UChar;
        
        while *bsLive_ptr > 0 {
            let zbits = *zbits_ptr;
            let numZ = *numZ_ptr;
            let bsBuff = *bsBuff_ptr;
            
            *zbits.offset(numZ as isize) = (bsBuff >> 24) as crate::types::UChar;
            *numZ_ptr += 1;
            *bsBuff_ptr <<= 8;
            *bsLive_ptr -= 8;
        }
    }
}

fn bsNEEDW(s: *mut crate::types::EState) {
    unsafe {
        let bsLive_ptr = crate::compat::c2r_field_ptr_EState__bsLive(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        let bsBuff_ptr = crate::compat::c2r_field_ptr_EState__bsBuff(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
        let numZ_ptr = crate::compat::c2r_field_ptr_EState__numZ(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        let zbits_ptr = crate::compat::c2r_field_ptr_EState__zbits(s as *mut ::core::ffi::c_void) as *mut *mut crate::types::UChar;
        
        while *bsLive_ptr >= 8 {
            let zbits = *zbits_ptr;
            let numZ = *numZ_ptr;
            *zbits.offset(numZ as isize) = ((*bsBuff_ptr) >> 24) as crate::types::UChar;
            *numZ_ptr += 1;
            *bsBuff_ptr <<= 8;
            *bsLive_ptr -= 8;
        }
    }
}

fn bsW(s: *mut crate::types::EState, n: crate::types::Int32, v: crate::types::UInt32) {
    crate::src_compress::bsNEEDW(s);
    
    unsafe {
        // Get bsLive field
        let bsLive_ptr = crate::compat::c2r_field_ptr_EState__bsLive(s as *mut ::core::ffi::c_void);
        let bsLive = *(bsLive_ptr as *const crate::types::Int32);
        
        // Get bsBuff field
        let bsBuff_ptr = crate::compat::c2r_field_ptr_EState__bsBuff(s as *mut ::core::ffi::c_void);
        let bsBuff = *(bsBuff_ptr as *const crate::types::UInt32);
        
        // s->bsBuff |= (v << (32 - s->bsLive - n));
        let shift_amount = 32 - bsLive - n;
        let new_bsBuff = bsBuff | (v << shift_amount);
        *(bsBuff_ptr as *mut crate::types::UInt32) = new_bsBuff;
        
        // s->bsLive += n;
        let new_bsLive = bsLive + n;
        *(bsLive_ptr as *mut crate::types::Int32) = new_bsLive;
    }
}

fn bsPutUInt32(s: *mut crate::types::EState, u: crate::types::UInt32) {
    bsW(s, 8, (u >> 24) & 0xff);
    bsW(s, 8, (u >> 16) & 0xff);
    bsW(s, 8, (u >> 8) & 0xff);
    bsW(s, 8, u & 0xff);
}

fn bsPutUChar(s: *mut crate::types::EState, c: crate::types::UChar) {
    crate::src_compress::bsW(s, 8, c as crate::types::UInt32);
}

fn makeMaps_e(s: *mut crate::types::EState) {
    let mut i: crate::types::Int32;
    
    unsafe {
        // Set nInUse to 0
        let nInUse_ptr = crate::compat::c2r_field_ptr_EState__nInUse(s as *mut ::core::ffi::c_void);
        *(nInUse_ptr as *mut crate::types::Int32) = 0;
        
        let inUse_ptr = crate::compat::c2r_field_ptr_EState__inUse(s as *mut ::core::ffi::c_void) as *mut crate::types::Bool;
        let unseqToSeq_ptr = crate::compat::c2r_field_ptr_EState__unseqToSeq(s as *mut ::core::ffi::c_void) as *mut crate::types::UChar;
        
        i = 0;
        while i < 256 {
            if *inUse_ptr.offset(i as isize) != 0 {
                let nInUse_val = *(nInUse_ptr as *mut crate::types::Int32);
                *unseqToSeq_ptr.offset(i as isize) = nInUse_val as crate::types::UChar;
                *(nInUse_ptr as *mut crate::types::Int32) = nInUse_val + 1;
            }
            i += 1;
        }
    }
}

fn generateMTFValues(s: *mut crate::types::EState) {
    let mut yy: [crate::types::UChar; 256] = [0; 256];
    let mut i: crate::types::Int32;
    let mut j: crate::types::Int32;
    let mut zPend: crate::types::Int32;
    let mut wr: crate::types::Int32;
    let EOB: crate::types::Int32;

    unsafe {
        let ptr: *mut crate::types::UInt32 = *(crate::compat::c2r_field_ptr_EState__ptr(s as *mut ::core::ffi::c_void) as *mut *mut crate::types::UInt32);
        let block: *mut crate::types::UChar = *(crate::compat::c2r_field_ptr_EState__block(s as *mut ::core::ffi::c_void) as *mut *mut crate::types::UChar);
        let mtfv: *mut crate::types::UInt16 = *(crate::compat::c2r_field_ptr_EState__mtfv(s as *mut ::core::ffi::c_void) as *mut *mut crate::types::UInt16);
        let mtfFreq: *mut crate::types::Int32 = crate::compat::c2r_field_ptr_EState__mtfFreq(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        let unseqToSeq: *mut crate::types::UChar = crate::compat::c2r_field_ptr_EState__unseqToSeq(s as *mut ::core::ffi::c_void) as *mut crate::types::UChar;
        let nInUse_ptr = crate::compat::c2r_field_ptr_EState__nInUse(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        let nblock_ptr = crate::compat::c2r_field_ptr_EState__nblock(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        let nMTF_ptr = crate::compat::c2r_field_ptr_EState__nMTF(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;

        crate::src_compress::makeMaps_e(s);
        EOB = *nInUse_ptr + 1;

        i = 0;
        while i <= EOB {
            *mtfFreq.offset(i as isize) = 0;
            i += 1;
        }

        wr = 0;
        zPend = 0;
        i = 0;
        while i < *nInUse_ptr {
            yy[i as usize] = i as crate::types::UChar;
            i += 1;
        }

        i = 0;
        while i < *nblock_ptr {
            let ll_i: crate::types::UChar;
            j = *ptr.offset(i as isize) as crate::types::Int32 - 1;
            if j < 0 {
                j += *nblock_ptr;
            }
            ll_i = *unseqToSeq.offset(*block.offset(j as isize) as isize);

            if yy[0] == ll_i {
                zPend += 1;
            } else {
                if zPend > 0 {
                    zPend -= 1;
                    loop {
                        if (zPend & 1) != 0 {
                            *mtfv.offset(wr as isize) = 1;
                            wr += 1;
                            *mtfFreq.offset(1) += 1;
                        } else {
                            *mtfv.offset(wr as isize) = 0;
                            wr += 1;
                            *mtfFreq.offset(0) += 1;
                        }
                        if zPend < 2 {
                            break;
                        }
                        zPend = (zPend - 2) / 2;
                    }
                    zPend = 0;
                }
                {
                    let mut rtmp: crate::types::UChar;
                    let mut ryy_j: *mut crate::types::UChar;
                    let rll_i: crate::types::UChar;
                    rtmp = yy[1];
                    yy[1] = yy[0];
                    ryy_j = &mut yy[1] as *mut crate::types::UChar;
                    rll_i = ll_i;
                    while rll_i != rtmp {
                        let rtmp2: crate::types::UChar;
                        ryy_j = ryy_j.offset(1);
                        rtmp2 = rtmp;
                        rtmp = *ryy_j;
                        *ryy_j = rtmp2;
                    }
                    yy[0] = rtmp;
                    j = ryy_j.offset_from(&mut yy[0] as *mut crate::types::UChar) as crate::types::Int32;
                    *mtfv.offset(wr as isize) = (j + 1) as crate::types::UInt16;
                    wr += 1;
                    *mtfFreq.offset((j + 1) as isize) += 1;
                }
            }
            i += 1;
        }

        if zPend > 0 {
            zPend -= 1;
            loop {
                if (zPend & 1) != 0 {
                    *mtfv.offset(wr as isize) = 1;
                    wr += 1;
                    *mtfFreq.offset(1) += 1;
                } else {
                    *mtfv.offset(wr as isize) = 0;
                    wr += 1;
                    *mtfFreq.offset(0) += 1;
                }
                if zPend < 2 {
                    break;
                }
                zPend = (zPend - 2) / 2;
            }
            let _ = zPend;
        }

        *mtfv.offset(wr as isize) = EOB as crate::types::UInt16;
        wr += 1;
        *mtfFreq.offset(EOB as isize) += 1;

        *nMTF_ptr = wr;
    }
}

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
    let mut nSelectors: crate::types::Int32 = 0;
    let mut alphaSize: crate::types::Int32;
    let mut minLen: crate::types::Int32;
    let mut maxLen: crate::types::Int32;
    let mut selCtr: crate::types::Int32;
    let mut nGroups: crate::types::Int32;
    let mut nBytes: crate::types::Int32;
    let mut cost: [crate::types::UInt16; 6] = [0; 6];
    let mut fave: [crate::types::Int32; 6] = [0; 6];

    unsafe {
        let mtfv_ptr = crate::compat::c2r_field_ptr_EState__mtfv(s as *mut ::core::ffi::c_void);
        let mtfv: *mut crate::types::UInt16 = *(mtfv_ptr as *mut *mut crate::types::UInt16);
        let verbosity_ptr = crate::compat::c2r_field_ptr_EState__verbosity(s as *mut ::core::ffi::c_void);
        let verbosity = *(verbosity_ptr as *mut crate::types::Int32);
        let nblock_ptr = crate::compat::c2r_field_ptr_EState__nblock(s as *mut ::core::ffi::c_void);
        let nblock = *(nblock_ptr as *mut crate::types::Int32);
        let nMTF_ptr = crate::compat::c2r_field_ptr_EState__nMTF(s as *mut ::core::ffi::c_void);
        let nMTF = *(nMTF_ptr as *mut crate::types::Int32);
        let nInUse_ptr = crate::compat::c2r_field_ptr_EState__nInUse(s as *mut ::core::ffi::c_void);
        let nInUse = *(nInUse_ptr as *mut crate::types::Int32);
        let len_ptr = crate::compat::c2r_field_ptr_EState__len(s as *mut ::core::ffi::c_void);
        let len = len_ptr as *mut [[crate::types::UChar; 258]; 6];
        let len_pack_ptr = crate::compat::c2r_field_ptr_EState__len_pack(s as *mut ::core::ffi::c_void);
        let len_pack = len_pack_ptr as *mut [[crate::types::UInt32; 3]; 258];
        let mtfFreq_ptr = crate::compat::c2r_field_ptr_EState__mtfFreq(s as *mut ::core::ffi::c_void);
        let mtfFreq = mtfFreq_ptr as *mut [crate::types::Int32; 258];
        let rfreq_ptr = crate::compat::c2r_field_ptr_EState__rfreq(s as *mut ::core::ffi::c_void);
        let rfreq = rfreq_ptr as *mut [[crate::types::Int32; 258]; 6];
        let selector_ptr = crate::compat::c2r_field_ptr_EState__selector(s as *mut ::core::ffi::c_void);
        let selector = selector_ptr as *mut [crate::types::UChar; 18002];
        let selectorMtf_ptr = crate::compat::c2r_field_ptr_EState__selectorMtf(s as *mut ::core::ffi::c_void);
        let selectorMtf = selectorMtf_ptr as *mut [crate::types::UChar; 18002];
        let inUse_ptr = crate::compat::c2r_field_ptr_EState__inUse(s as *mut ::core::ffi::c_void);
        let inUse = inUse_ptr as *mut [crate::types::Bool; 256];
        let code_ptr = crate::compat::c2r_field_ptr_EState__code(s as *mut ::core::ffi::c_void);
        let code = code_ptr as *mut [[crate::types::Int32; 258]; 6];
        let numZ_ptr = crate::compat::c2r_field_ptr_EState__numZ(s as *mut ::core::ffi::c_void);

        let stderr_file = crate::compat::stderr as *mut libc::FILE;

        if verbosity >= 3 {
            libc::fprintf(stderr_file, b"      %d in block, %d after MTF & 1-2 coding, %d+2 syms in use\n\0".as_ptr() as *const libc::c_char, nblock, nMTF, nInUse);
        }

        alphaSize = nInUse + 2;
        t = 0;
        while t < 6 { v = 0; while v < alphaSize { (*len)[t as usize][v as usize] = 15; v += 1; } t += 1; }

        if !(nMTF > 0) { crate::src_bzlib::BZ2_bz__AssertH__fail(3001); }
        if nMTF < 200 { nGroups = 2; }
        else if nMTF < 600 { nGroups = 3; }
        else if nMTF < 1200 { nGroups = 4; }
        else if nMTF < 2400 { nGroups = 5; }
        else { nGroups = 6; }

        {
            let mut nPart = nGroups; let mut remF = nMTF; gs = 0;
            while nPart > 0 {
                let tFreq = remF / nPart; ge = gs - 1; let mut aFreq = 0i32;
                while aFreq < tFreq && ge < alphaSize - 1 { ge += 1; aFreq += (*mtfFreq)[ge as usize]; }
                if ge > gs && nPart != nGroups && nPart != 1 && ((nGroups - nPart) % 2 == 1) { aFreq -= (*mtfFreq)[ge as usize]; ge -= 1; }
                if verbosity >= 3 { libc::fprintf(stderr_file, b"      initial group %d, [%d .. %d], has %d syms (%4.1f%%)\n\0".as_ptr() as *const libc::c_char, nPart, gs, ge, aFreq, (100.0f64 * aFreq as f64) / nMTF as f64); }
                v = 0; while v < alphaSize { if v >= gs && v <= ge { (*len)[(nPart - 1) as usize][v as usize] = 0; } else { (*len)[(nPart - 1) as usize][v as usize] = 15; } v += 1; }
                nPart -= 1; gs = ge + 1; remF -= aFreq;
            }
        }

        iter = 0;
        while iter < 4 {
            t = 0; while t < nGroups { fave[t as usize] = 0; t += 1; }
            t = 0; while t < nGroups { v = 0; while v < alphaSize { (*rfreq)[t as usize][v as usize] = 0; v += 1; } t += 1; }
            if nGroups == 6 { v = 0; while v < alphaSize { (*len_pack)[v as usize][0] = (((*len)[1][v as usize] as u32) << 16) | ((*len)[0][v as usize] as u32); (*len_pack)[v as usize][1] = (((*len)[3][v as usize] as u32) << 16) | ((*len)[2][v as usize] as u32); (*len_pack)[v as usize][2] = (((*len)[5][v as usize] as u32) << 16) | ((*len)[4][v as usize] as u32); v += 1; } }
            nSelectors = 0; totc = 0; gs = 0;
            loop {
                if gs >= nMTF { break; } ge = gs + 50 - 1; if ge >= nMTF { ge = nMTF - 1; }
                t = 0; while t < nGroups { cost[t as usize] = 0; t += 1; }
                if nGroups == 6 && 50 == ge - gs + 1 {
                    let mut cost01: u32 = 0; let mut cost23: u32 = 0; let mut cost45: u32 = 0;
                    for nn in 0..50 { let icv = *mtfv.offset((gs + nn) as isize) as usize; cost01 += (*len_pack)[icv][0]; cost23 += (*len_pack)[icv][1]; cost45 += (*len_pack)[icv][2]; }
                    cost[0] = (cost01 & 0xffff) as u16; cost[1] = (cost01 >> 16) as u16; cost[2] = (cost23 & 0xffff) as u16; cost[3] = (cost23 >> 16) as u16; cost[4] = (cost45 & 0xffff) as u16; cost[5] = (cost45 >> 16) as u16;
                } else { i = gs; while i <= ge { let icv = *mtfv.offset(i as isize) as usize; t = 0; while t < nGroups { cost[t as usize] += (*len)[t as usize][icv] as u16; t += 1; } i += 1; } }
                bc = 999999999; bt = -1; t = 0; while t < nGroups { if (cost[t as usize] as i32) < bc { bc = cost[t as usize] as i32; bt = t; } t += 1; }
                totc += bc; fave[bt as usize] += 1; (*selector)[nSelectors as usize] = bt as u8; nSelectors += 1;
                if nGroups == 6 && 50 == ge - gs + 1 { for nn in 0..50 { let idx = *mtfv.offset((gs + nn) as isize) as usize; (*rfreq)[bt as usize][idx] += 1; } }
                else { i = gs; while i <= ge { let idx = *mtfv.offset(i as isize) as usize; (*rfreq)[bt as usize][idx] += 1; i += 1; } }
                gs = ge + 1;
            }
            if verbosity >= 3 { libc::fprintf(stderr_file, b"      pass %d: size is %d, grp uses are \0".as_ptr() as *const libc::c_char, iter + 1, totc / 8); t = 0; while t < nGroups { libc::fprintf(stderr_file, b"%d \0".as_ptr() as *const libc::c_char, fave[t as usize]); t += 1; } libc::fprintf(stderr_file, b"\n\0".as_ptr() as *const libc::c_char); }
            t = 0; while t < nGroups { crate::src_huffman::BZ2_hbMakeCodeLengths((*len)[t as usize].as_mut_ptr(), (*rfreq)[t as usize].as_mut_ptr(), alphaSize, 17); t += 1; }
            iter += 1;
        }
        if !(nGroups < 8) { crate::src_bzlib::BZ2_bz__AssertH__fail(3002); }
        if !(nSelectors < 32768 && nSelectors <= (2 + (900000 / 50))) { crate::src_bzlib::BZ2_bz__AssertH__fail(3003); }
        { let mut pos: [u8; 6] = [0; 6]; i = 0; while i < nGroups { pos[i as usize] = i as u8; i += 1; } i = 0; while i < nSelectors { let ll_i = (*selector)[i as usize]; j = 0; let mut tmp = pos[j as usize]; while ll_i != tmp { j += 1; let tmp2 = tmp; tmp = pos[j as usize]; pos[j as usize] = tmp2; } pos[0] = tmp; (*selectorMtf)[i as usize] = j as u8; i += 1; } }
        t = 0; while t < nGroups { minLen = 32; maxLen = 0; i = 0; while i < alphaSize { if (*len)[t as usize][i as usize] as i32 > maxLen { maxLen = (*len)[t as usize][i as usize] as i32; } if ((*len)[t as usize][i as usize] as i32) < minLen { minLen = (*len)[t as usize][i as usize] as i32; } i += 1; } if !(!(maxLen > 17)) { crate::src_bzlib::BZ2_bz__AssertH__fail(3004); } if !(!(minLen < 1)) { crate::src_bzlib::BZ2_bz__AssertH__fail(3005); } crate::src_huffman::BZ2_hbAssignCodes((*code)[t as usize].as_mut_ptr(), (*len)[t as usize].as_mut_ptr(), minLen, maxLen, alphaSize); t += 1; }
        { let mut inUse16: [crate::types::Bool; 16] = [0; 16]; i = 0; while i < 16 { inUse16[i as usize] = 0; j = 0; while j < 16 { if (*inUse)[(i * 16 + j) as usize] != 0 { inUse16[i as usize] = 1; } j += 1; } i += 1; } nBytes = *(numZ_ptr as *mut crate::types::Int32); i = 0; while i < 16 { if inUse16[i as usize] != 0 { crate::src_compress::bsW(s, 1, 1); } else { crate::src_compress::bsW(s, 1, 0); } i += 1; } i = 0; while i < 16 { if inUse16[i as usize] != 0 { j = 0; while j < 16 { if (*inUse)[(i * 16 + j) as usize] != 0 { crate::src_compress::bsW(s, 1, 1); } else { crate::src_compress::bsW(s, 1, 0); } j += 1; } } i += 1; } if verbosity >= 3 { libc::fprintf(stderr_file, b"      bytes: mapping %d, \0".as_ptr() as *const libc::c_char, *(numZ_ptr as *mut crate::types::Int32) - nBytes); } }
        nBytes = *(numZ_ptr as *mut crate::types::Int32); crate::src_compress::bsW(s, 3, nGroups as u32); crate::src_compress::bsW(s, 15, nSelectors as u32); i = 0; while i < nSelectors { j = 0; while j < (*selectorMtf)[i as usize] as i32 { crate::src_compress::bsW(s, 1, 1); j += 1; } crate::src_compress::bsW(s, 1, 0); i += 1; }
        if verbosity >= 3 { libc::fprintf(stderr_file, b"selectors %d, \0".as_ptr() as *const libc::c_char, *(numZ_ptr as *mut crate::types::Int32) - nBytes); }
        nBytes = *(numZ_ptr as *mut crate::types::Int32); t = 0; while t < nGroups { let mut curr = (*len)[t as usize][0] as i32; crate::src_compress::bsW(s, 5, curr as u32); i = 0; while i < alphaSize { while curr < (*len)[t as usize][i as usize] as i32 { crate::src_compress::bsW(s, 2, 2); curr += 1; } while curr > (*len)[t as usize][i as usize] as i32 { crate::src_compress::bsW(s, 2, 3); curr -= 1; } crate::src_compress::bsW(s, 1, 0); i += 1; } t += 1; }
        if verbosity >= 3 { libc::fprintf(stderr_file, b"code lengths %d, \0".as_ptr() as *const libc::c_char, *(numZ_ptr as *mut crate::types::Int32) - nBytes); }
        nBytes = *(numZ_ptr as *mut crate::types::Int32); selCtr = 0; gs = 0;
        loop { if gs >= nMTF { break; } ge = gs + 50 - 1; if ge >= nMTF { ge = nMTF - 1; } if !((*selector)[selCtr as usize] < nGroups as u8) { crate::src_bzlib::BZ2_bz__AssertH__fail(3006); } if nGroups == 6 && 50 == ge - gs + 1 { let sel = (*selector)[selCtr as usize] as usize; let s_len_sel_selCtr = (*len)[sel].as_ptr(); let s_code_sel_selCtr = (*code)[sel].as_ptr(); for nn in 0..50 { let mtfv_i = *mtfv.offset((gs + nn) as isize) as usize; crate::src_compress::bsW(s, *s_len_sel_selCtr.add(mtfv_i) as i32, *s_code_sel_selCtr.add(mtfv_i) as u32); } } else { i = gs; while i <= ge { let sel = (*selector)[selCtr as usize] as usize; let idx = *mtfv.offset(i as isize) as usize; crate::src_compress::bsW(s, (*len)[sel][idx] as i32, (*code)[sel][idx] as u32); i += 1; } } gs = ge + 1; selCtr += 1; }
        if !(selCtr == nSelectors) { crate::src_bzlib::BZ2_bz__AssertH__fail(3007); }
        if verbosity >= 3 { libc::fprintf(stderr_file, b"codes %d\n\0".as_ptr() as *const libc::c_char, *(numZ_ptr as *mut crate::types::Int32) - nBytes); }
    }
}

pub extern "C" fn BZ2_compressBlock(arg1: *mut crate::types::EState, arg2: crate::types::Bool) {
    let s = arg1;
    let is_last_block = arg2;
    
    unsafe {
        let nblock_ptr = crate::compat::c2r_field_ptr_EState__nblock(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        let nblock = *nblock_ptr;
        
        if nblock > 0 {
            let blockCRC_ptr = crate::compat::c2r_field_ptr_EState__blockCRC(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
            *blockCRC_ptr = !(*blockCRC_ptr);
            
            let combinedCRC_ptr = crate::compat::c2r_field_ptr_EState__combinedCRC(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
            *combinedCRC_ptr = (*combinedCRC_ptr << 1) | (*combinedCRC_ptr >> 31);
            *combinedCRC_ptr ^= *blockCRC_ptr;
            
            let blockNo_ptr = crate::compat::c2r_field_ptr_EState__blockNo(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
            if *blockNo_ptr > 1 {
                let numZ_ptr = crate::compat::c2r_field_ptr_EState__numZ(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
                *numZ_ptr = 0;
            }
            
            let verbosity_ptr = crate::compat::c2r_field_ptr_EState__verbosity(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
            if *verbosity_ptr >= 2 {
                libc::fprintf(
                    crate::compat::stderr as *mut libc::FILE,
                    b"    block %d: crc = 0x%08x, combined CRC = 0x%08x, size = %d\n\0".as_ptr() as *const i8,
                    *blockNo_ptr,
                    *blockCRC_ptr,
                    *combinedCRC_ptr,
                    nblock
                );
            }
            
            crate::src_blocksort::BZ2_blockSort(s);
        }
        
        let arr2_ptr = crate::compat::c2r_field_ptr_EState__arr2(s as *mut ::core::ffi::c_void) as *mut *mut crate::types::UInt32;
        let zbits_ptr = crate::compat::c2r_field_ptr_EState__zbits(s as *mut ::core::ffi::c_void) as *mut *mut crate::types::UChar;
        let nblock_ptr = crate::compat::c2r_field_ptr_EState__nblock(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        *zbits_ptr = ((*arr2_ptr) as *mut crate::types::UChar).offset(*nblock_ptr as isize);
        
        let blockNo_ptr = crate::compat::c2r_field_ptr_EState__blockNo(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        if *blockNo_ptr == 1 {
            crate::src_compress::BZ2_bsInitWrite(s);
            crate::src_compress::bsPutUChar(s, 0x42);
            crate::src_compress::bsPutUChar(s, 0x5a);
            crate::src_compress::bsPutUChar(s, 0x68);
            let blockSize100k_ptr = crate::compat::c2r_field_ptr_EState__blockSize100k(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
            crate::src_compress::bsPutUChar(s, (0x30 + *blockSize100k_ptr) as crate::types::UChar);
        }
        
        let nblock_ptr = crate::compat::c2r_field_ptr_EState__nblock(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        if *nblock_ptr > 0 {
            crate::src_compress::bsPutUChar(s, 0x31);
            crate::src_compress::bsPutUChar(s, 0x41);
            crate::src_compress::bsPutUChar(s, 0x59);
            crate::src_compress::bsPutUChar(s, 0x26);
            crate::src_compress::bsPutUChar(s, 0x53);
            crate::src_compress::bsPutUChar(s, 0x59);
            
            let blockCRC_ptr = crate::compat::c2r_field_ptr_EState__blockCRC(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
            crate::src_compress::bsPutUInt32(s, *blockCRC_ptr);
            
            crate::src_compress::bsW(s, 1, 0);
            
            let origPtr_ptr = crate::compat::c2r_field_ptr_EState__origPtr(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
            crate::src_compress::bsW(s, 24, *origPtr_ptr as crate::types::UInt32);
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
            
            let combinedCRC_ptr = crate::compat::c2r_field_ptr_EState__combinedCRC(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
            crate::src_compress::bsPutUInt32(s, *combinedCRC_ptr);
            
            let verbosity_ptr = crate::compat::c2r_field_ptr_EState__verbosity(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
            if *verbosity_ptr >= 2 {
                libc::fprintf(
                    crate::compat::stderr as *mut libc::FILE,
                    b"    final combined CRC = 0x%08x\n   \0".as_ptr() as *const i8,
                    *combinedCRC_ptr
                );
            }
            crate::src_compress::bsFinishWrite(s);
        }
    }
}
