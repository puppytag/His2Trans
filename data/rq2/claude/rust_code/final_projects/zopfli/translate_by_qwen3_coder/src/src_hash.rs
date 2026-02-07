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
        // h->head = (int*)malloc(sizeof(*h->head) * 65536);
        let head_ptr = crate::compat::c2r_field_ptr_ZopfliHash__head(h as *mut ::core::ffi::c_void) as *mut *mut i32;
        *head_ptr = libc::malloc(std::mem::size_of::<i32>() * 65536) as *mut i32;

        // h->prev = (unsigned short*)malloc(sizeof(*h->prev) * window_size);
        let prev_ptr = crate::compat::c2r_field_ptr_ZopfliHash__prev(h as *mut ::core::ffi::c_void) as *mut *mut u16;
        *prev_ptr = libc::malloc(std::mem::size_of::<u16>() * window_size as usize) as *mut u16;

        // h->hashval = (int*)malloc(sizeof(*h->hashval) * window_size);
        let hashval_ptr = crate::compat::c2r_field_ptr_ZopfliHash__hashval(h as *mut ::core::ffi::c_void) as *mut *mut i32;
        *hashval_ptr = libc::malloc(std::mem::size_of::<i32>() * window_size as usize) as *mut i32;

        // h->same = (unsigned short*)malloc(sizeof(*h->same) * window_size);
        let same_ptr = crate::compat::c2r_field_ptr_ZopfliHash__same(h as *mut ::core::ffi::c_void) as *mut *mut u16;
        *same_ptr = libc::malloc(std::mem::size_of::<u16>() * window_size as usize) as *mut u16;

        // h->head2 = (int*)malloc(sizeof(*h->head2) * 65536);
        let head2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__head2(h as *mut ::core::ffi::c_void) as *mut *mut i32;
        *head2_ptr = libc::malloc(std::mem::size_of::<i32>() * 65536) as *mut i32;

        // h->prev2 = (unsigned short*)malloc(sizeof(*h->prev2) * window_size);
        let prev2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__prev2(h as *mut ::core::ffi::c_void) as *mut *mut u16;
        *prev2_ptr = libc::malloc(std::mem::size_of::<u16>() * window_size as usize) as *mut u16;

        // h->hashval2 = (int*)malloc(sizeof(*h->hashval2) * window_size);
        let hashval2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__hashval2(h as *mut ::core::ffi::c_void) as *mut *mut i32;
        *hashval2_ptr = libc::malloc(std::mem::size_of::<i32>() * window_size as usize) as *mut i32;
    }
}

pub extern "C" fn ZopfliResetHash(window_size: crate::types::size_t, h: *mut crate::types::ZopfliHash) {
    unsafe {
        let val_ptr = crate::compat::c2r_field_ptr_ZopfliHash__val(h as *mut ::core::ffi::c_void) as *mut i32;
        *val_ptr = 0;

        let head_ptr = crate::compat::c2r_field_ptr_ZopfliHash__head(h as *mut ::core::ffi::c_void) as *mut *mut i32;
        let head = *head_ptr;
        for i in 0..65536usize {
            *head.add(i) = -1;
        }

        let prev_ptr = crate::compat::c2r_field_ptr_ZopfliHash__prev(h as *mut ::core::ffi::c_void) as *mut *mut u16;
        let prev = *prev_ptr;
        let hashval_ptr = crate::compat::c2r_field_ptr_ZopfliHash__hashval(h as *mut ::core::ffi::c_void) as *mut *mut i32;
        let hashval = *hashval_ptr;
        for i in 0..window_size as usize {
            *prev.add(i) = i as u16;
            *hashval.add(i) = -1;
        }

        let same_ptr = crate::compat::c2r_field_ptr_ZopfliHash__same(h as *mut ::core::ffi::c_void) as *mut *mut u16;
        let same = *same_ptr;
        for i in 0..window_size as usize {
            *same.add(i) = 0;
        }

        let val2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__val2(h as *mut ::core::ffi::c_void) as *mut i32;
        *val2_ptr = 0;

        let head2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__head2(h as *mut ::core::ffi::c_void) as *mut *mut i32;
        let head2 = *head2_ptr;
        for i in 0..65536usize {
            *head2.add(i) = -1;
        }

        let prev2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__prev2(h as *mut ::core::ffi::c_void) as *mut *mut u16;
        let prev2 = *prev2_ptr;
        let hashval2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__hashval2(h as *mut ::core::ffi::c_void) as *mut *mut i32;
        let hashval2 = *hashval2_ptr;
        for i in 0..window_size as usize {
            *prev2.add(i) = i as u16;
            *hashval2.add(i) = -1;
        }
    }
}

pub extern "C" fn ZopfliCleanHash(h: *mut crate::types::ZopfliHash) {
    unsafe {
        let head_ptr = crate::compat::c2r_field_ptr_ZopfliHash__head(h as *mut ::core::ffi::c_void);
        libc::free(*(head_ptr as *mut *mut ::core::ffi::c_void));
        
        let prev_ptr = crate::compat::c2r_field_ptr_ZopfliHash__prev(h as *mut ::core::ffi::c_void);
        libc::free(*(prev_ptr as *mut *mut ::core::ffi::c_void));
        
        let hashval_ptr = crate::compat::c2r_field_ptr_ZopfliHash__hashval(h as *mut ::core::ffi::c_void);
        libc::free(*(hashval_ptr as *mut *mut ::core::ffi::c_void));
        
        let head2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__head2(h as *mut ::core::ffi::c_void);
        libc::free(*(head2_ptr as *mut *mut ::core::ffi::c_void));
        
        let prev2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__prev2(h as *mut ::core::ffi::c_void);
        libc::free(*(prev2_ptr as *mut *mut ::core::ffi::c_void));
        
        let hashval2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__hashval2(h as *mut ::core::ffi::c_void);
        libc::free(*(hashval2_ptr as *mut *mut ::core::ffi::c_void));
        
        let same_ptr = crate::compat::c2r_field_ptr_ZopfliHash__same(h as *mut ::core::ffi::c_void);
        libc::free(*(same_ptr as *mut *mut ::core::ffi::c_void));
    }
}

fn UpdateHashValue(h: *mut crate::types::ZopfliHash, c: u8) {
    unsafe {
        let val_ptr = crate::compat::c2r_field_ptr_ZopfliHash__val(h as *mut ::core::ffi::c_void) as *mut i32;
        let old_val = *val_ptr;
        *val_ptr = (((old_val) << 5) ^ (c as i32)) & 32767;
    }
}

pub extern "C" fn ZopfliUpdateHash(array: *const ::core::ffi::c_uchar, pos: crate::types::size_t, end: crate::types::size_t, h: *mut crate::types::ZopfliHash) {
    unsafe {
        let hpos: u16 = (pos & (32768 - 1)) as u16;
        let mut amount: usize = 0;

        let c = if pos + 3 <= end {
            *array.add((pos + 3 - 1) as usize)
        } else {
            0
        };
        crate::src_hash::UpdateHashValue(h, c);

        let val_ptr = crate::compat::c2r_field_ptr_ZopfliHash__val(h as *mut ::core::ffi::c_void) as *mut i32;
        let val = *val_ptr;

        let hashval_ptr = crate::compat::c2r_field_ptr_ZopfliHash__hashval(h as *mut ::core::ffi::c_void) as *mut *mut i32;
        let hashval = *hashval_ptr;
        *hashval.add(hpos as usize) = val;

        let head_ptr = crate::compat::c2r_field_ptr_ZopfliHash__head(h as *mut ::core::ffi::c_void) as *mut *mut i32;
        let head = *head_ptr;

        let prev_ptr = crate::compat::c2r_field_ptr_ZopfliHash__prev(h as *mut ::core::ffi::c_void) as *mut *mut u16;
        let prev = *prev_ptr;

        if *head.add(val as usize) != -1 && *hashval.add(*head.add(val as usize) as usize) == val {
            *prev.add(hpos as usize) = *head.add(val as usize) as u16;
        } else {
            *prev.add(hpos as usize) = hpos;
        }
        *head.add(val as usize) = hpos as i32;

        let same_ptr = crate::compat::c2r_field_ptr_ZopfliHash__same(h as *mut ::core::ffi::c_void) as *mut *mut u16;
        let same = *same_ptr;

        let prev_pos = ((pos as isize - 1) as usize) & (32768 - 1);
        if *same.add(prev_pos) > 1 {
            amount = (*same.add(prev_pos) - 1) as usize;
        }

        while pos + amount + 1 < end
            && *array.add(pos as usize) == *array.add((pos + amount + 1) as usize)
            && amount < (u16::MAX as usize)
        {
            amount += 1;
        }
        *same.add(hpos as usize) = amount as u16;

        let val2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__val2(h as *mut ::core::ffi::c_void) as *mut i32;
        let same_val = *same.add(hpos as usize) as i32;
        *val2_ptr = ((same_val - 3) & 255) ^ val;
        let val2 = *val2_ptr;

        let hashval2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__hashval2(h as *mut ::core::ffi::c_void) as *mut *mut i32;
        let hashval2 = *hashval2_ptr;
        *hashval2.add(hpos as usize) = val2;

        let head2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__head2(h as *mut ::core::ffi::c_void) as *mut *mut i32;
        let head2 = *head2_ptr;

        let prev2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__prev2(h as *mut ::core::ffi::c_void) as *mut *mut u16;
        let prev2 = *prev2_ptr;

        if *head2.add(val2 as usize) != -1 && *hashval2.add(*head2.add(val2 as usize) as usize) == val2 {
            *prev2.add(hpos as usize) = *head2.add(val2 as usize) as u16;
        } else {
            *prev2.add(hpos as usize) = hpos;
        }
        *head2.add(val2 as usize) = hpos as i32;
    }
}

pub extern "C" fn ZopfliWarmupHash(array: *const ::core::ffi::c_uchar, pos: crate::types::size_t, end: crate::types::size_t, h: *mut crate::types::ZopfliHash) {
    unsafe {
        crate::src_hash::UpdateHashValue(h, *array.add(pos + 0));
        if pos + 1 < end {
            crate::src_hash::UpdateHashValue(h, *array.add(pos + 1));
        }
    }
}
