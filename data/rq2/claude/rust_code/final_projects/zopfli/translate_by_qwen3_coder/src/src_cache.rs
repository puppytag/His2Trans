//! Module: src_cache
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

pub extern "C" fn ZopfliInitCache(blocksize: crate::types::size_t, lmc: *mut crate::types::ZopfliLongestMatchCache) {
    unsafe {
        let length_ptr = crate::compat::c2r_field_ptr_ZopfliLongestMatchCache__length(lmc as *mut ::core::ffi::c_void);
        let dist_ptr = crate::compat::c2r_field_ptr_ZopfliLongestMatchCache__dist(lmc as *mut ::core::ffi::c_void);
        let sublen_ptr = crate::compat::c2r_field_ptr_ZopfliLongestMatchCache__sublen(lmc as *mut ::core::ffi::c_void);
        
        let blocksize_usize = blocksize as usize;
        
        let length_alloc = libc::malloc(std::mem::size_of::<u16>() * blocksize_usize) as *mut u16;
        *(length_ptr as *mut *mut u16) = length_alloc;
        
        let dist_alloc = libc::malloc(std::mem::size_of::<u16>() * blocksize_usize) as *mut u16;
        *(dist_ptr as *mut *mut u16) = dist_alloc;
        
        let sublen_size = 8usize * 3usize * blocksize_usize;
        let sublen_alloc = libc::malloc(sublen_size) as *mut u8;
        *(sublen_ptr as *mut *mut u8) = sublen_alloc;
        
        if sublen_alloc.is_null() {
            libc::fprintf(
                crate::compat::stderr as *mut libc::FILE,
                b"Error: Out of memory. Tried allocating %lu bytes of memory.\n\0".as_ptr() as *const i8,
                sublen_size as libc::c_ulong,
            );
            libc::exit(1);
        }
        
        let length_arr = *(length_ptr as *mut *mut u16);
        let dist_arr = *(dist_ptr as *mut *mut u16);
        let sublen_arr = *(sublen_ptr as *mut *mut u8);
        
        for i in 0..blocksize_usize {
            *length_arr.add(i) = 1;
        }
        for i in 0..blocksize_usize {
            *dist_arr.add(i) = 0;
        }
        for i in 0..(8usize * blocksize_usize * 3usize) {
            *sublen_arr.add(i) = 0;
        }
    }
}

pub extern "C" fn ZopfliCleanCache(lmc: *mut crate::types::ZopfliLongestMatchCache) {
    unsafe {
        let length_ptr = crate::compat::c2r_field_ptr_ZopfliLongestMatchCache__length(
            lmc as *mut ::core::ffi::c_void
        ) as *mut *mut u16;
        libc::free(*length_ptr as *mut ::core::ffi::c_void);
        
        let dist_ptr = crate::compat::c2r_field_ptr_ZopfliLongestMatchCache__dist(
            lmc as *mut ::core::ffi::c_void
        ) as *mut *mut u16;
        libc::free(*dist_ptr as *mut ::core::ffi::c_void);
        
        let sublen_ptr = crate::compat::c2r_field_ptr_ZopfliLongestMatchCache__sublen(
            lmc as *mut ::core::ffi::c_void
        ) as *mut *mut u8;
        libc::free(*sublen_ptr as *mut ::core::ffi::c_void);
    }
}

pub extern "C" fn ZopfliSublenToCache(sublen: *const ::core::ffi::c_ushort, pos: crate::types::size_t, length: crate::types::size_t, lmc: *mut crate::types::ZopfliLongestMatchCache) {
    let mut j: crate::types::size_t = 0;
    let mut bestlength: ::core::ffi::c_uint = 0;
    
    // Get pointer to lmc->sublen using accessor shim
    let sublen_base = unsafe {
        crate::compat::c2r_field_ptr_ZopfliLongestMatchCache__sublen(lmc as *mut ::core::ffi::c_void)
    };
    
    // cache = &lmc->sublen[8 * pos * 3]
    let cache: *mut ::core::ffi::c_uchar = unsafe {
        (sublen_base as *mut ::core::ffi::c_uchar).add((8 * pos * 3) as usize)
    };
    
    if length < 3 {
        return;
    }
    
    let mut i: crate::types::size_t = 3;
    while i <= length {
        let at_end = i == length;
        let different_next = if !at_end {
            unsafe { *sublen.add(i as usize) != *sublen.add((i + 1) as usize) }
        } else {
            false
        };
        
        if at_end || different_next {
            unsafe {
                *cache.add((j * 3) as usize) = (i - 3) as ::core::ffi::c_uchar;
                let sublen_i = *sublen.add(i as usize);
                *cache.add((j * 3 + 1) as usize) = (sublen_i % 256) as ::core::ffi::c_uchar;
                *cache.add((j * 3 + 2) as usize) = ((sublen_i >> 8) % 256) as ::core::ffi::c_uchar;
            }
            bestlength = i as ::core::ffi::c_uint;
            j += 1;
            if j >= 8 {
                break;
            }
        }
        i += 1;
    }
    
    if j < 8 {
        // assert(bestlength == length)
        if bestlength != length as ::core::ffi::c_uint {
            unsafe {
                libc::abort();
            }
        }
        unsafe {
            *cache.add(((8 - 1) * 3) as usize) = (bestlength - 3) as ::core::ffi::c_uchar;
        }
    } else {
        // assert(bestlength <= length)
        if bestlength > length as ::core::ffi::c_uint {
            unsafe {
                libc::abort();
            }
        }
    }
    
    // assert(bestlength == ZopfliMaxCachedSublen(lmc, pos, length))
    let max_cached = crate::src_cache::ZopfliMaxCachedSublen(lmc as *const crate::types::ZopfliLongestMatchCache, pos, length);
    if bestlength != max_cached {
        unsafe {
            libc::abort();
        }
    }
}

pub extern "C" fn ZopfliCacheToSublen(lmc: *const crate::types::ZopfliLongestMatchCache, pos: crate::types::size_t, length: crate::types::size_t, sublen: *mut ::core::ffi::c_ushort) {
    if length < 3 {
        return;
    }
    
    let maxlength = crate::src_cache::ZopfliMaxCachedSublen(lmc, pos, length);
    let mut prevlength: u32 = 0;
    
    // Get the sublen field pointer using the accessor shim
    let sublen_base_ptr = unsafe {
        crate::compat::c2r_field_ptr_ZopfliLongestMatchCache__sublen(
            lmc as *mut ::core::ffi::c_void
        )
    };
    
    // ZOPFLI_CACHE_LENGTH is 8
    let cache: *mut u8 = unsafe {
        (sublen_base_ptr as *mut u8).add((8 * pos as usize * 3) as usize)
    };
    
    for j in 0..8usize {
        let len_val: u32 = unsafe { *cache.add(j * 3) as u32 + 3 };
        let dist: u16 = unsafe {
            (*cache.add(j * 3 + 1) as u16) + 256 * (*cache.add(j * 3 + 2) as u16)
        };
        
        for i in prevlength..=len_val {
            unsafe {
                *sublen.add(i as usize) = dist;
            }
        }
        
        if len_val == maxlength {
            break;
        }
        prevlength = len_val + 1;
    }
}

pub extern "C" fn ZopfliMaxCachedSublen(lmc: *const crate::types::ZopfliLongestMatchCache, pos: crate::types::size_t, length: crate::types::size_t) -> ::core::ffi::c_uint {
    let _ = length;
    
    unsafe {
        let sublen_ptr = crate::compat::c2r_field_ptr_ZopfliLongestMatchCache__sublen(
            lmc as *mut ::core::ffi::c_void
        ) as *mut u8;
        
        let cache: *mut u8 = sublen_ptr.add(8 * pos * 3);
        
        if *cache.add(1) == 0 && *cache.add(2) == 0 {
            return 0;
        }
        
        (*cache.add((8 - 1) * 3) as ::core::ffi::c_uint) + 3
    }
}
