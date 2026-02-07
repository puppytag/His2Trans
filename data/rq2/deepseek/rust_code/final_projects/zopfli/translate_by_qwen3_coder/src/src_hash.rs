//! Module: src_hash
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

pub extern "C" fn ZopfliAllocHash(window_size: crate::types::size_t, h: *mut crate::types::ZopfliHash) {
    unsafe {
        let head_size = 65536 * std::mem::size_of::<i32>();
        let prev_size = (window_size as usize) * std::mem::size_of::<u16>();
        let hashval_size = (window_size as usize) * std::mem::size_of::<i32>();
        let same_size = (window_size as usize) * std::mem::size_of::<u16>();
        let head2_size = 65536 * std::mem::size_of::<i32>();
        let prev2_size = (window_size as usize) * std::mem::size_of::<u16>();
        let hashval2_size = (window_size as usize) * std::mem::size_of::<i32>();

        let head_ptr = libc::malloc(head_size) as *mut i32;
        let prev_ptr = libc::malloc(prev_size) as *mut u16;
        let hashval_ptr = libc::malloc(hashval_size) as *mut i32;
        let same_ptr = libc::malloc(same_size) as *mut u16;
        let head2_ptr = libc::malloc(head2_size) as *mut i32;
        let prev2_ptr = libc::malloc(prev2_size) as *mut u16;
        let hashval2_ptr = libc::malloc(hashval2_size) as *mut i32;

        let h_ptr = h as *mut u8;
        std::ptr::copy_nonoverlapping(&head_ptr as *const *mut i32 as *const u8, h_ptr, std::mem::size_of::<*mut i32>());
        std::ptr::copy_nonoverlapping(&prev_ptr as *const *mut u16 as *const u8, h_ptr.add(std::mem::size_of::<*mut i32>()), std::mem::size_of::<*mut u16>());
        std::ptr::copy_nonoverlapping(&hashval_ptr as *const *mut i32 as *const u8, h_ptr.add(std::mem::size_of::<*mut i32>() + std::mem::size_of::<*mut u16>()), std::mem::size_of::<*mut i32>());
        std::ptr::copy_nonoverlapping(&same_ptr as *const *mut u16 as *const u8, h_ptr.add(std::mem::size_of::<*mut i32>() + std::mem::size_of::<*mut u16>() + std::mem::size_of::<*mut i32>()), std::mem::size_of::<*mut u16>());
        std::ptr::copy_nonoverlapping(&head2_ptr as *const *mut i32 as *const u8, h_ptr.add(std::mem::size_of::<*mut i32>() + std::mem::size_of::<*mut u16>() + std::mem::size_of::<*mut i32>() + std::mem::size_of::<*mut u16>()), std::mem::size_of::<*mut i32>());
        std::ptr::copy_nonoverlapping(&prev2_ptr as *const *mut u16 as *const u8, h_ptr.add(std::mem::size_of::<*mut i32>() + std::mem::size_of::<*mut u16>() + std::mem::size_of::<*mut i32>() + std::mem::size_of::<*mut u16>() + std::mem::size_of::<*mut i32>()), std::mem::size_of::<*mut u16>());
        std::ptr::copy_nonoverlapping(&hashval2_ptr as *const *mut i32 as *const u8, h_ptr.add(std::mem::size_of::<*mut i32>() + std::mem::size_of::<*mut u16>() + std::mem::size_of::<*mut i32>() + std::mem::size_of::<*mut u16>() + std::mem::size_of::<*mut i32>() + std::mem::size_of::<*mut u16>()), std::mem::size_of::<*mut i32>());
    }
}

pub extern "C" fn ZopfliResetHash(window_size: crate::types::size_t, h: *mut crate::types::ZopfliHash) {
    if h.is_null() {
        return;
    }
    unsafe {
        let h_ptr = h as *mut u8;
        let window_size_usize = window_size as usize;
        let head_offset = 0usize;
        let head2_offset = 65536 * 2;
        let prev_offset = head2_offset + 65536 * 2;
        let prev2_offset = prev_offset + window_size_usize * 2;
        let hashval_offset = prev2_offset + window_size_usize * 2;
        let hashval2_offset = hashval_offset + window_size_usize * 2;
        let same_offset = hashval2_offset + window_size_usize * 2;
        let val_offset = same_offset + window_size_usize * 2;
        let val2_offset = val_offset + 4;

        *(h_ptr.add(val_offset) as *mut i32) = 0;
        for i in 0..65536 {
            *(h_ptr.add(head_offset + i * 2) as *mut i16) = -1;
        }
        for i in 0..window_size_usize {
            *(h_ptr.add(prev_offset + i * 2) as *mut i16) = i as i16;
            *(h_ptr.add(hashval_offset + i * 2) as *mut i16) = -1;
        }
        for i in 0..window_size_usize {
            *(h_ptr.add(same_offset + i * 2) as *mut i16) = 0;
        }
        *(h_ptr.add(val2_offset) as *mut i32) = 0;
        for i in 0..65536 {
            *(h_ptr.add(head2_offset + i * 2) as *mut i16) = -1;
        }
        for i in 0..window_size_usize {
            *(h_ptr.add(prev2_offset + i * 2) as *mut i16) = i as i16;
            *(h_ptr.add(hashval2_offset + i * 2) as *mut i16) = -1;
        }
    }
}

pub extern "C" fn ZopfliCleanHash(h: *mut crate::types::ZopfliHash) {
    if h.is_null() {
        return;
    }
    unsafe {
        let h_void = h as *mut ::core::ffi::c_void;
        let head_ptr = crate::compat::c2r_field_ptr_ZopfliHash__head2(h_void) as *mut *mut ::core::ffi::c_void;
        if !head_ptr.is_null() {
            libc::free(*head_ptr as *mut ::core::ffi::c_void);
        }
        let prev_ptr = crate::compat::c2r_field_ptr_ZopfliHash__prev2(h_void) as *mut *mut ::core::ffi::c_void;
        if !prev_ptr.is_null() {
            libc::free(*prev_ptr as *mut ::core::ffi::c_void);
        }
        let hashval_ptr = crate::compat::c2r_field_ptr_ZopfliHash__hashval2(h_void) as *mut *mut ::core::ffi::c_void;
        if !hashval_ptr.is_null() {
            libc::free(*hashval_ptr as *mut ::core::ffi::c_void);
        }
        let head2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__head2(h_void) as *mut *mut ::core::ffi::c_void;
        if !head2_ptr.is_null() {
            libc::free(*head2_ptr as *mut ::core::ffi::c_void);
        }
        let prev2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__prev2(h_void) as *mut *mut ::core::ffi::c_void;
        if !prev2_ptr.is_null() {
            libc::free(*prev2_ptr as *mut ::core::ffi::c_void);
        }
        let hashval2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__hashval2(h_void) as *mut *mut ::core::ffi::c_void;
        if !hashval2_ptr.is_null() {
            libc::free(*hashval2_ptr as *mut ::core::ffi::c_void);
        }
        let same_ptr = crate::compat::c2r_field_ptr_ZopfliHash__head2(h_void) as *mut *mut ::core::ffi::c_void;
        if !same_ptr.is_null() {
            libc::free(*same_ptr as *mut ::core::ffi::c_void);
        }
    }
}

fn UpdateHashValue(h: *mut crate::types::ZopfliHash, c: u8) {
    unsafe {
        let val_ptr = (h as *mut u8).offset(0) as *mut u16;
        *val_ptr = ((*val_ptr << 5) ^ (c as u16)) & 32767;
    }
}

pub extern "C" fn ZopfliUpdateHash(array: *const ::core::ffi::c_uchar, pos: crate::types::size_t, end: crate::types::size_t, h: *mut crate::types::ZopfliHash) {
    let hpos = (pos & (32768 - 1)) as u16;
    let mut amount: crate::types::size_t = 0;

    let c = if pos + 3 <= end {
        unsafe { *array.add((pos + 3 - 1) as usize) }
    } else {
        0
    };
    crate::src_hash::UpdateHashValue(h, c);

    unsafe {
        let val_ptr = crate::compat::c2r_field_ptr_ZopfliHash__val(h as *mut ::core::ffi::c_void) as *mut u16;
        let val = *val_ptr;
        let hashval_ptr = crate::compat::c2r_field_ptr_ZopfliHash__hashval(h as *mut ::core::ffi::c_void) as *mut u16;
        *hashval_ptr.add(hpos as usize) = val;
        let head_ptr = crate::compat::c2r_field_ptr_ZopfliHash__head(h as *mut ::core::ffi::c_void) as *mut i32;
        let prev_ptr = crate::compat::c2r_field_ptr_ZopfliHash__prev(h as *mut ::core::ffi::c_void) as *mut u16;
        if *head_ptr.add(val as usize) != -1 && *hashval_ptr.add(*head_ptr.add(val as usize) as usize) == val {
            *prev_ptr.add(hpos as usize) = *head_ptr.add(val as usize) as u16;
        } else {
            *prev_ptr.add(hpos as usize) = hpos;
        }
        *head_ptr.add(val as usize) = hpos as i32;

        let same_ptr = crate::compat::c2r_field_ptr_ZopfliHash__same(h as *mut ::core::ffi::c_void) as *mut u16;
        let prev_hpos = ((pos as isize - 1) & (32768 - 1)) as u16;
        if *same_ptr.add(prev_hpos as usize) > 1 {
            amount = (*same_ptr.add(prev_hpos as usize) - 1) as crate::types::size_t;
        }
        let mut pos_usize = pos as usize;
        let end_usize = end as usize;
        while pos_usize + amount + 1 < end_usize &&
              *array.add(pos_usize) == *array.add(pos_usize + amount as usize + 1) &&
              amount < u16::MAX as crate::types::size_t {
            amount += 1;
        }
        *same_ptr.add(hpos as usize) = amount as u16;

        let val2 = ((*same_ptr.add(hpos as usize) as i32 - 3) & 255) as u16 ^ val;
        let val2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__val2(h as *mut ::core::ffi::c_void) as *mut u16;
        *val2_ptr = val2;
        let hashval2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__hashval2(h as *mut ::core::ffi::c_void) as *mut u16;
        *hashval2_ptr.add(hpos as usize) = val2;
        let head2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__head2(h as *mut ::core::ffi::c_void) as *mut i32;
        let prev2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__prev2(h as *mut ::core::ffi::c_void) as *mut u16;
        if *head2_ptr.add(val2 as usize) != -1 && *hashval2_ptr.add(*head2_ptr.add(val2 as usize) as usize) == val2 {
            *prev2_ptr.add(hpos as usize) = *head2_ptr.add(val2 as usize) as u16;
        } else {
            *prev2_ptr.add(hpos as usize) = hpos;
        }
        *head2_ptr.add(val2 as usize) = hpos as i32;
    }
}

pub extern "C" fn ZopfliWarmupHash(array: *const ::core::ffi::c_uchar, pos: crate::types::size_t, end: crate::types::size_t, h: *mut crate::types::ZopfliHash) {
    unsafe {
        crate::src_hash::UpdateHashValue(h, *array.offset(pos as isize));
        if pos + 1 < end {
            crate::src_hash::UpdateHashValue(h, *array.offset((pos + 1) as isize));
        }
    }
}
