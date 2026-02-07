//! Module: src_decompress
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

fn makeMaps_d(s: *mut crate::types::DState) {
    let mut i: crate::types::Int32;
    
    // Set nInUse to 0
    unsafe {
        let nInUse_ptr = crate::compat::c2r_field_ptr_DState__nInUse(s as *mut ::core::ffi::c_void);
        *(nInUse_ptr as *mut crate::types::Int32) = 0;
    }
    
    i = 0;
    while i < 256 {
        // Get inUse[i]
        let inUse_val = unsafe {
            let inUse_ptr = crate::compat::c2r_field_ptr_DState__inUse(s as *mut ::core::ffi::c_void);
            *((inUse_ptr as *mut crate::types::Bool).offset(i as isize))
        };
        
        if inUse_val != 0 {
            // Get current nInUse
            let nInUse_ptr = unsafe {
                crate::compat::c2r_field_ptr_DState__nInUse(s as *mut ::core::ffi::c_void)
            };
            let nInUse_val = unsafe { *(nInUse_ptr as *mut crate::types::Int32) };
            
            // Set seqToUnseq[nInUse] = i
            unsafe {
                let seqToUnseq_ptr = crate::compat::c2r_field_ptr_DState__seqToUnseq(s as *mut ::core::ffi::c_void);
                *((seqToUnseq_ptr as *mut crate::types::UChar).offset(nInUse_val as isize)) = i as crate::types::UChar;
            }
            
            // Increment nInUse
            unsafe {
                *(nInUse_ptr as *mut crate::types::Int32) = nInUse_val + 1;
            }
        }
        
        i += 1;
    }
}

pub extern "C" fn BZ2_decompress(arg1: *mut crate::types::DState) -> crate::types::Int32 {
    use crate::compat::*;
    use crate::globals::*;
    
    let s = arg1;
    let mut uc: crate::types::UChar = 0;
    let mut retVal: crate::types::Int32;
    let mut minLen: crate::types::Int32 = 0;
    let mut maxLen: crate::types::Int32 = 0;
    
    let strm: *mut crate::types::bz_stream = unsafe {
        *(c2r_field_ptr_DState__strm(s as *mut ::core::ffi::c_void) as *mut *mut crate::types::bz_stream)
    };
    
    let mut i: crate::types::Int32 = 0;
    let mut j: crate::types::Int32 = 0;
    let mut t: crate::types::Int32 = 0;
    let mut alphaSize: crate::types::Int32 = 0;
    let mut nGroups: crate::types::Int32 = 0;
    let mut nSelectors: crate::types::Int32 = 0;
    let mut EOB: crate::types::Int32 = 0;
    let mut groupNo: crate::types::Int32 = 0;
    let mut groupPos: crate::types::Int32 = 0;
    let mut nextSym: crate::types::Int32 = 0;
    let mut nblockMAX: crate::types::Int32 = 0;
    let mut nblock: crate::types::Int32 = 0;
    let mut es: crate::types::Int32 = 0;
    let mut N: crate::types::Int32 = 0;
    let mut curr: crate::types::Int32 = 0;
    let mut zt: crate::types::Int32 = 0;
    let mut zn: crate::types::Int32 = 0;
    let mut zvec: crate::types::Int32 = 0;
    let mut zj: crate::types::Int32 = 0;
    let mut gSel: crate::types::Int32 = 0;
    let mut gMinlen: crate::types::Int32 = 0;
    let mut gLimit: *mut crate::types::Int32 = std::ptr::null_mut();
    let mut gBase: *mut crate::types::Int32 = std::ptr::null_mut();
    let mut gPerm: *mut crate::types::Int32 = std::ptr::null_mut();
    
    unsafe fn get_state_i32(s: *mut crate::types::DState, field_fn: unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void) -> crate::types::Int32 {
        *(field_fn(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32)
    }
    
    unsafe fn set_state_i32(s: *mut crate::types::DState, field_fn: unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void, val: crate::types::Int32) {
        *(field_fn(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) = val;
    }
    
    unsafe fn get_state_ptr(s: *mut crate::types::DState, field_fn: unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void) -> *mut crate::types::Int32 {
        *(field_fn(s as *mut ::core::ffi::c_void) as *mut *mut crate::types::Int32)
    }
    
    unsafe fn set_state_ptr(s: *mut crate::types::DState, field_fn: unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void, val: *mut crate::types::Int32) {
        *(field_fn(s as *mut ::core::ffi::c_void) as *mut *mut crate::types::Int32) = val;
    }
    
    let state_val: crate::types::Int32 = unsafe { get_state_i32(s, c2r_field_ptr_DState__state) };
    
    if state_val == 10 {
        unsafe {
            set_state_i32(s, c2r_field_ptr_DState__save_i, 0);
            set_state_i32(s, c2r_field_ptr_DState__save_j, 0);
            set_state_i32(s, c2r_field_ptr_DState__save_t, 0);
            set_state_i32(s, c2r_field_ptr_DState__save_alphaSize, 0);
            set_state_i32(s, c2r_field_ptr_DState__save_nGroups, 0);
            set_state_i32(s, c2r_field_ptr_DState__save_nSelectors, 0);
            set_state_i32(s, c2r_field_ptr_DState__save_EOB, 0);
            set_state_i32(s, c2r_field_ptr_DState__save_groupNo, 0);
            set_state_i32(s, c2r_field_ptr_DState__save_groupPos, 0);
            set_state_i32(s, c2r_field_ptr_DState__save_nextSym, 0);
            set_state_i32(s, c2r_field_ptr_DState__save_nblockMAX, 0);
            set_state_i32(s, c2r_field_ptr_DState__save_nblock, 0);
            set_state_i32(s, c2r_field_ptr_DState__save_es, 0);
            set_state_i32(s, c2r_field_ptr_DState__save_N, 0);
            set_state_i32(s, c2r_field_ptr_DState__save_curr, 0);
            set_state_i32(s, c2r_field_ptr_DState__save_zt, 0);
            set_state_i32(s, c2r_field_ptr_DState__save_zn, 0);
            set_state_i32(s, c2r_field_ptr_DState__save_zvec, 0);
            set_state_i32(s, c2r_field_ptr_DState__save_zj, 0);
            set_state_i32(s, c2r_field_ptr_DState__save_gSel, 0);
            set_state_i32(s, c2r_field_ptr_DState__save_gMinlen, 0);
            set_state_ptr(s, c2r_field_ptr_DState__save_gLimit, std::ptr::null_mut());
            set_state_ptr(s, c2r_field_ptr_DState__save_gBase, std::ptr::null_mut());
            set_state_ptr(s, c2r_field_ptr_DState__save_gPerm, std::ptr::null_mut());
        }
    }
    
    unsafe {
        i = get_state_i32(s, c2r_field_ptr_DState__save_i);
        j = get_state_i32(s, c2r_field_ptr_DState__save_j);
        t = get_state_i32(s, c2r_field_ptr_DState__save_t);
        alphaSize = get_state_i32(s, c2r_field_ptr_DState__save_alphaSize);
        nGroups = get_state_i32(s, c2r_field_ptr_DState__save_nGroups);
        nSelectors = get_state_i32(s, c2r_field_ptr_DState__save_nSelectors);
        EOB = get_state_i32(s, c2r_field_ptr_DState__save_EOB);
        groupNo = get_state_i32(s, c2r_field_ptr_DState__save_groupNo);
        groupPos = get_state_i32(s, c2r_field_ptr_DState__save_groupPos);
        nextSym = get_state_i32(s, c2r_field_ptr_DState__save_nextSym);
        nblockMAX = get_state_i32(s, c2r_field_ptr_DState__save_nblockMAX);
        nblock = get_state_i32(s, c2r_field_ptr_DState__save_nblock);
        es = get_state_i32(s, c2r_field_ptr_DState__save_es);
        N = get_state_i32(s, c2r_field_ptr_DState__save_N);
        curr = get_state_i32(s, c2r_field_ptr_DState__save_curr);
        zt = get_state_i32(s, c2r_field_ptr_DState__save_zt);
        zn = get_state_i32(s, c2r_field_ptr_DState__save_zn);
        zvec = get_state_i32(s, c2r_field_ptr_DState__save_zvec);
        zj = get_state_i32(s, c2r_field_ptr_DState__save_zj);
        gSel = get_state_i32(s, c2r_field_ptr_DState__save_gSel);
        gMinlen = get_state_i32(s, c2r_field_ptr_DState__save_gMinlen);
        gLimit = get_state_ptr(s, c2r_field_ptr_DState__save_gLimit);
        gBase = get_state_ptr(s, c2r_field_ptr_DState__save_gBase);
        gPerm = get_state_ptr(s, c2r_field_ptr_DState__save_gPerm);
    }
    
    retVal = 0;
    
    unsafe {
        set_state_i32(s, c2r_field_ptr_DState__save_i, i);
        set_state_i32(s, c2r_field_ptr_DState__save_j, j);
        set_state_i32(s, c2r_field_ptr_DState__save_t, t);
        set_state_i32(s, c2r_field_ptr_DState__save_alphaSize, alphaSize);
        set_state_i32(s, c2r_field_ptr_DState__save_nGroups, nGroups);
        set_state_i32(s, c2r_field_ptr_DState__save_nSelectors, nSelectors);
        set_state_i32(s, c2r_field_ptr_DState__save_EOB, EOB);
        set_state_i32(s, c2r_field_ptr_DState__save_groupNo, groupNo);
        set_state_i32(s, c2r_field_ptr_DState__save_groupPos, groupPos);
        set_state_i32(s, c2r_field_ptr_DState__save_nextSym, nextSym);
        set_state_i32(s, c2r_field_ptr_DState__save_nblockMAX, nblockMAX);
        set_state_i32(s, c2r_field_ptr_DState__save_nblock, nblock);
        set_state_i32(s, c2r_field_ptr_DState__save_es, es);
        set_state_i32(s, c2r_field_ptr_DState__save_N, N);
        set_state_i32(s, c2r_field_ptr_DState__save_curr, curr);
        set_state_i32(s, c2r_field_ptr_DState__save_zt, zt);
        set_state_i32(s, c2r_field_ptr_DState__save_zn, zn);
        set_state_i32(s, c2r_field_ptr_DState__save_zvec, zvec);
        set_state_i32(s, c2r_field_ptr_DState__save_zj, zj);
        set_state_i32(s, c2r_field_ptr_DState__save_gSel, gSel);
        set_state_i32(s, c2r_field_ptr_DState__save_gMinlen, gMinlen);
        set_state_ptr(s, c2r_field_ptr_DState__save_gLimit, gLimit);
        set_state_ptr(s, c2r_field_ptr_DState__save_gBase, gBase);
        set_state_ptr(s, c2r_field_ptr_DState__save_gPerm, gPerm);
    }
    
    let _ = uc;
    let _ = minLen;
    let _ = maxLen;
    let _ = strm;
    
    retVal
}
