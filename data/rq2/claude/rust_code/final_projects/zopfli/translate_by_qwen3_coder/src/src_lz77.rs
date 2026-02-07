//! Module: src_lz77
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

pub extern "C" fn ZopfliInitLZ77Store(data: *const ::core::ffi::c_uchar, store: *mut crate::types::ZopfliLZ77Store) {
    unsafe {
        // size = 0
        let size_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(store as *mut ::core::ffi::c_void);
        *(size_ptr as *mut crate::types::size_t) = 0;
        
        // litlens = 0
        let litlens_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__litlens(store as *mut ::core::ffi::c_void);
        *(litlens_ptr as *mut *mut ::core::ffi::c_ushort) = std::ptr::null_mut();
        
        // dists = 0
        let dists_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__dists(store as *mut ::core::ffi::c_void);
        *(dists_ptr as *mut *mut ::core::ffi::c_ushort) = std::ptr::null_mut();
        
        // pos = 0
        let pos_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__pos(store as *mut ::core::ffi::c_void);
        *(pos_ptr as *mut *mut crate::types::size_t) = std::ptr::null_mut();
        
        // data = data
        let data_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__data(store as *mut ::core::ffi::c_void);
        *(data_ptr as *mut *const ::core::ffi::c_uchar) = data;
        
        // ll_symbol = 0
        let ll_symbol_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__ll_symbol(store as *mut ::core::ffi::c_void);
        *(ll_symbol_ptr as *mut *mut ::core::ffi::c_ushort) = std::ptr::null_mut();
        
        // d_symbol = 0
        let d_symbol_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__d_symbol(store as *mut ::core::ffi::c_void);
        *(d_symbol_ptr as *mut *mut ::core::ffi::c_ushort) = std::ptr::null_mut();
        
        // ll_counts = 0
        let ll_counts_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__ll_counts(store as *mut ::core::ffi::c_void);
        *(ll_counts_ptr as *mut *mut crate::types::size_t) = std::ptr::null_mut();
        
        // d_counts = 0
        let d_counts_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__d_counts(store as *mut ::core::ffi::c_void);
        *(d_counts_ptr as *mut *mut crate::types::size_t) = std::ptr::null_mut();
    }
}

pub extern "C" fn ZopfliCleanLZ77Store(store: *mut crate::types::ZopfliLZ77Store) {
    unsafe {
        let litlens_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__litlens(
            store as *mut ::core::ffi::c_void
        );
        let litlens = *(litlens_ptr as *const *mut ::core::ffi::c_void);
        libc::free(litlens);

        let dists_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__dists(
            store as *mut ::core::ffi::c_void
        );
        let dists = *(dists_ptr as *const *mut ::core::ffi::c_void);
        libc::free(dists);

        let pos_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__pos(
            store as *mut ::core::ffi::c_void
        );
        let pos = *(pos_ptr as *const *mut ::core::ffi::c_void);
        libc::free(pos);

        let ll_symbol_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__ll_symbol(
            store as *mut ::core::ffi::c_void
        );
        let ll_symbol = *(ll_symbol_ptr as *const *mut ::core::ffi::c_void);
        libc::free(ll_symbol);

        let d_symbol_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__d_symbol(
            store as *mut ::core::ffi::c_void
        );
        let d_symbol = *(d_symbol_ptr as *const *mut ::core::ffi::c_void);
        libc::free(d_symbol);

        let ll_counts_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__ll_counts(
            store as *mut ::core::ffi::c_void
        );
        let ll_counts = *(ll_counts_ptr as *const *mut ::core::ffi::c_void);
        libc::free(ll_counts);

        let d_counts_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__d_counts(
            store as *mut ::core::ffi::c_void
        );
        let d_counts = *(d_counts_ptr as *const *mut ::core::ffi::c_void);
        libc::free(d_counts);
    }
}

fn CeilDiv(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}

pub extern "C" fn ZopfliCopyLZ77Store(source: *const crate::types::ZopfliLZ77Store, dest: *mut crate::types::ZopfliLZ77Store) {
    unsafe {
        let source_size_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(source as *mut ::core::ffi::c_void);
        let source_size = *(source_size_ptr as *const usize);
        
        let llsize: usize = 288 * crate::src_lz77::CeilDiv(source_size, 288);
        let dsize: usize = 32 * crate::src_lz77::CeilDiv(source_size, 32);
        
        crate::src_lz77::ZopfliCleanLZ77Store(dest);
        
        let source_data_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__data(source as *mut ::core::ffi::c_void);
        let source_data = *(source_data_ptr as *const *const u8);
        crate::src_lz77::ZopfliInitLZ77Store(source_data, dest);
        
        let dest_litlens_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__litlens(dest as *mut ::core::ffi::c_void);
        *(dest_litlens_ptr as *mut *mut u16) = libc::malloc(std::mem::size_of::<u16>() * source_size) as *mut u16;
        
        let dest_dists_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__dists(dest as *mut ::core::ffi::c_void);
        *(dest_dists_ptr as *mut *mut u16) = libc::malloc(std::mem::size_of::<u16>() * source_size) as *mut u16;
        
        let dest_pos_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__pos(dest as *mut ::core::ffi::c_void);
        *(dest_pos_ptr as *mut *mut usize) = libc::malloc(std::mem::size_of::<usize>() * source_size) as *mut usize;
        
        let dest_ll_symbol_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__ll_symbol(dest as *mut ::core::ffi::c_void);
        *(dest_ll_symbol_ptr as *mut *mut u16) = libc::malloc(std::mem::size_of::<u16>() * source_size) as *mut u16;
        
        let dest_d_symbol_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__d_symbol(dest as *mut ::core::ffi::c_void);
        *(dest_d_symbol_ptr as *mut *mut u16) = libc::malloc(std::mem::size_of::<u16>() * source_size) as *mut u16;
        
        let dest_ll_counts_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__ll_counts(dest as *mut ::core::ffi::c_void);
        *(dest_ll_counts_ptr as *mut *mut usize) = libc::malloc(std::mem::size_of::<usize>() * llsize) as *mut usize;
        
        let dest_d_counts_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__d_counts(dest as *mut ::core::ffi::c_void);
        *(dest_d_counts_ptr as *mut *mut usize) = libc::malloc(std::mem::size_of::<usize>() * dsize) as *mut usize;
        
        let dest_litlens = *(dest_litlens_ptr as *const *mut u16);
        let dest_dists = *(dest_dists_ptr as *const *mut u16);
        let dest_pos = *(dest_pos_ptr as *const *mut usize);
        let dest_ll_symbol = *(dest_ll_symbol_ptr as *const *mut u16);
        let dest_d_symbol = *(dest_d_symbol_ptr as *const *mut u16);
        let dest_ll_counts = *(dest_ll_counts_ptr as *const *mut usize);
        let dest_d_counts = *(dest_d_counts_ptr as *const *mut usize);
        
        if dest_litlens.is_null() || dest_dists.is_null() { libc::exit(-1); }
        if dest_pos.is_null() { libc::exit(-1); }
        if dest_ll_symbol.is_null() || dest_d_symbol.is_null() { libc::exit(-1); }
        if dest_ll_counts.is_null() || dest_d_counts.is_null() { libc::exit(-1); }
        
        let dest_size_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(dest as *mut ::core::ffi::c_void);
        *(dest_size_ptr as *mut usize) = source_size;
        
        let source_litlens_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__litlens(source as *mut ::core::ffi::c_void);
        let source_litlens = *(source_litlens_ptr as *const *const u16);
        let source_dists_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__dists(source as *mut ::core::ffi::c_void);
        let source_dists = *(source_dists_ptr as *const *const u16);
        let source_pos_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__pos(source as *mut ::core::ffi::c_void);
        let source_pos = *(source_pos_ptr as *const *const usize);
        let source_ll_symbol_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__ll_symbol(source as *mut ::core::ffi::c_void);
        let source_ll_symbol = *(source_ll_symbol_ptr as *const *const u16);
        let source_d_symbol_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__d_symbol(source as *mut ::core::ffi::c_void);
        let source_d_symbol = *(source_d_symbol_ptr as *const *const u16);
        let source_ll_counts_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__ll_counts(source as *mut ::core::ffi::c_void);
        let source_ll_counts = *(source_ll_counts_ptr as *const *const usize);
        let source_d_counts_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__d_counts(source as *mut ::core::ffi::c_void);
        let source_d_counts = *(source_d_counts_ptr as *const *const usize);
        
        for i in 0..source_size {
            *dest_litlens.add(i) = *source_litlens.add(i);
            *dest_dists.add(i) = *source_dists.add(i);
            *dest_pos.add(i) = *source_pos.add(i);
            *dest_ll_symbol.add(i) = *source_ll_symbol.add(i);
            *dest_d_symbol.add(i) = *source_d_symbol.add(i);
        }
        for i in 0..llsize {
            *dest_ll_counts.add(i) = *source_ll_counts.add(i);
        }
        for i in 0..dsize {
            *dest_d_counts.add(i) = *source_d_counts.add(i);
        }
    }
}

pub extern "C" fn ZopfliStoreLitLenDist(length: ::core::ffi::c_ushort, dist: ::core::ffi::c_ushort, pos: crate::types::size_t, store: *mut crate::types::ZopfliLZ77Store) {
    unsafe {
        let size_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(store as *mut ::core::ffi::c_void) as *mut crate::types::size_t;
        let origsize = *size_ptr;
        let llstart = 288 * (origsize / 288);
        let dstart = 32 * (origsize / 32);

        let ll_counts_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__ll_counts(store as *mut ::core::ffi::c_void) as *mut *mut crate::types::size_t;
        let d_counts_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__d_counts(store as *mut ::core::ffi::c_void) as *mut *mut crate::types::size_t;
        let litlens_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__litlens(store as *mut ::core::ffi::c_void) as *mut *mut ::core::ffi::c_ushort;
        let dists_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__dists(store as *mut ::core::ffi::c_void) as *mut *mut ::core::ffi::c_ushort;
        let pos_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__pos(store as *mut ::core::ffi::c_void) as *mut *mut crate::types::size_t;
        let ll_symbol_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__ll_symbol(store as *mut ::core::ffi::c_void) as *mut *mut ::core::ffi::c_ushort;
        let d_symbol_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__d_symbol(store as *mut ::core::ffi::c_void) as *mut *mut ::core::ffi::c_ushort;

        if origsize % 288 == 0 {
            let mut llsize = origsize;
            for i in 0..288usize {
                if (llsize & (llsize.wrapping_sub(1))) == 0 {
                    *ll_counts_ptr = if llsize == 0 {
                        libc::malloc(std::mem::size_of::<crate::types::size_t>()) as *mut crate::types::size_t
                    } else {
                        libc::realloc(*ll_counts_ptr as *mut ::core::ffi::c_void, llsize * 2 * std::mem::size_of::<crate::types::size_t>()) as *mut crate::types::size_t
                    };
                }
                *(*ll_counts_ptr).add(llsize) = if origsize == 0 { 0 } else { *(*ll_counts_ptr).add(origsize - 288 + i) };
                llsize += 1;
            }
        }

        if origsize % 32 == 0 {
            let mut dsize = origsize;
            for i in 0..32usize {
                if (dsize & (dsize.wrapping_sub(1))) == 0 {
                    *d_counts_ptr = if dsize == 0 {
                        libc::malloc(std::mem::size_of::<crate::types::size_t>()) as *mut crate::types::size_t
                    } else {
                        libc::realloc(*d_counts_ptr as *mut ::core::ffi::c_void, dsize * 2 * std::mem::size_of::<crate::types::size_t>()) as *mut crate::types::size_t
                    };
                }
                *(*d_counts_ptr).add(dsize) = if origsize == 0 { 0 } else { *(*d_counts_ptr).add(origsize - 32 + i) };
                dsize += 1;
            }
        }

        if (*size_ptr & ((*size_ptr).wrapping_sub(1))) == 0 {
            *litlens_ptr = if *size_ptr == 0 {
                libc::malloc(std::mem::size_of::<::core::ffi::c_ushort>()) as *mut ::core::ffi::c_ushort
            } else {
                libc::realloc(*litlens_ptr as *mut ::core::ffi::c_void, *size_ptr * 2 * std::mem::size_of::<::core::ffi::c_ushort>()) as *mut ::core::ffi::c_ushort
            };
        }
        *(*litlens_ptr).add(*size_ptr) = length;
        *size_ptr += 1;
        *size_ptr = origsize;

        if (*size_ptr & ((*size_ptr).wrapping_sub(1))) == 0 {
            *dists_ptr = if *size_ptr == 0 {
                libc::malloc(std::mem::size_of::<::core::ffi::c_ushort>()) as *mut ::core::ffi::c_ushort
            } else {
                libc::realloc(*dists_ptr as *mut ::core::ffi::c_void, *size_ptr * 2 * std::mem::size_of::<::core::ffi::c_ushort>()) as *mut ::core::ffi::c_ushort
            };
        }
        *(*dists_ptr).add(*size_ptr) = dist;
        *size_ptr += 1;
        *size_ptr = origsize;

        if (*size_ptr & ((*size_ptr).wrapping_sub(1))) == 0 {
            *pos_ptr = if *size_ptr == 0 {
                libc::malloc(std::mem::size_of::<crate::types::size_t>()) as *mut crate::types::size_t
            } else {
                libc::realloc(*pos_ptr as *mut ::core::ffi::c_void, *size_ptr * 2 * std::mem::size_of::<crate::types::size_t>()) as *mut crate::types::size_t
            };
        }
        *(*pos_ptr).add(*size_ptr) = pos;
        *size_ptr += 1;

        assert!(length < 259);

        fn get_length_symbol(l: u16) -> i32 {
            if l <= 10 { return 257 + l as i32 - 3; }
            if l <= 18 { return 265 + (l as i32 - 11) / 2; }
            if l <= 34 { return 269 + (l as i32 - 19) / 4; }
            if l <= 66 { return 273 + (l as i32 - 35) / 8; }
            if l <= 130 { return 277 + (l as i32 - 67) / 16; }
            if l <= 257 { return 281 + (l as i32 - 131) / 32; }
            285
        }
        fn get_dist_symbol(d: u16) -> i32 {
            if d <= 4 { return d as i32 - 1; }
            let mut r = 0i32; let mut v = d as i32;
            while v > 4 { v >>= 1; r += 2; }
            r + (d as i32 - 1) / (1 << (r / 2)) - 1
        }

        if dist == 0 {
            *size_ptr = origsize;
            if (*size_ptr & ((*size_ptr).wrapping_sub(1))) == 0 {
                *ll_symbol_ptr = if *size_ptr == 0 {
                    libc::malloc(std::mem::size_of::<::core::ffi::c_ushort>()) as *mut ::core::ffi::c_ushort
                } else {
                    libc::realloc(*ll_symbol_ptr as *mut ::core::ffi::c_void, *size_ptr * 2 * std::mem::size_of::<::core::ffi::c_ushort>()) as *mut ::core::ffi::c_ushort
                };
            }
            *(*ll_symbol_ptr).add(*size_ptr) = length;
            *size_ptr += 1;

            *size_ptr = origsize;
            if (*size_ptr & ((*size_ptr).wrapping_sub(1))) == 0 {
                *d_symbol_ptr = if *size_ptr == 0 {
                    libc::malloc(std::mem::size_of::<::core::ffi::c_ushort>()) as *mut ::core::ffi::c_ushort
                } else {
                    libc::realloc(*d_symbol_ptr as *mut ::core::ffi::c_void, *size_ptr * 2 * std::mem::size_of::<::core::ffi::c_ushort>()) as *mut ::core::ffi::c_ushort
                };
            }
            *(*d_symbol_ptr).add(*size_ptr) = 0;
            *size_ptr += 1;

            *(*ll_counts_ptr).add(llstart + length as usize) += 1;
        } else {
            let len_sym = get_length_symbol(length);
            let dist_sym = get_dist_symbol(dist);

            *size_ptr = origsize;
            if (*size_ptr & ((*size_ptr).wrapping_sub(1))) == 0 {
                *ll_symbol_ptr = if *size_ptr == 0 {
                    libc::malloc(std::mem::size_of::<::core::ffi::c_ushort>()) as *mut ::core::ffi::c_ushort
                } else {
                    libc::realloc(*ll_symbol_ptr as *mut ::core::ffi::c_void, *size_ptr * 2 * std::mem::size_of::<::core::ffi::c_ushort>()) as *mut ::core::ffi::c_ushort
                };
            }
            *(*ll_symbol_ptr).add(*size_ptr) = len_sym as ::core::ffi::c_ushort;
            *size_ptr += 1;

            *size_ptr = origsize;
            if (*size_ptr & ((*size_ptr).wrapping_sub(1))) == 0 {
                *d_symbol_ptr = if *size_ptr == 0 {
                    libc::malloc(std::mem::size_of::<::core::ffi::c_ushort>()) as *mut ::core::ffi::c_ushort
                } else {
                    libc::realloc(*d_symbol_ptr as *mut ::core::ffi::c_void, *size_ptr * 2 * std::mem::size_of::<::core::ffi::c_ushort>()) as *mut ::core::ffi::c_ushort
                };
            }
            *(*d_symbol_ptr).add(*size_ptr) = dist_sym as ::core::ffi::c_ushort;
            *size_ptr += 1;

            *(*ll_counts_ptr).add(llstart + len_sym as usize) += 1;
            *(*d_counts_ptr).add(dstart + dist_sym as usize) += 1;
        }
    }
}

pub extern "C" fn ZopfliAppendLZ77Store(store: *const crate::types::ZopfliLZ77Store, target: *mut crate::types::ZopfliLZ77Store) {
    unsafe {
        let size_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(store as *mut ::core::ffi::c_void);
        let size = *(size_ptr as *const crate::types::size_t);
        
        let litlens_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__litlens(store as *mut ::core::ffi::c_void);
        let litlens = *(litlens_ptr as *const *mut ::core::ffi::c_ushort);
        
        let dists_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__dists(store as *mut ::core::ffi::c_void);
        let dists = *(dists_ptr as *const *mut ::core::ffi::c_ushort);
        
        let pos_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__pos(store as *mut ::core::ffi::c_void);
        let pos = *(pos_ptr as *const *mut crate::types::size_t);
        
        let mut i: crate::types::size_t = 0;
        while i < size {
            crate::src_lz77::ZopfliStoreLitLenDist(
                *litlens.add(i as usize),
                *dists.add(i as usize),
                *pos.add(i as usize),
                target
            );
            i += 1;
        }
    }
}

pub extern "C" fn ZopfliLZ77GetByteRange(lz77: *const crate::types::ZopfliLZ77Store, lstart: crate::types::size_t, lend: crate::types::size_t) -> crate::types::size_t {
    if lstart == lend {
        return 0;
    }
    
    let l = lend - 1;
    
    unsafe {
        let pos_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__pos(lz77 as *mut ::core::ffi::c_void) as *mut *mut crate::types::size_t;
        let dists_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__dists(lz77 as *mut ::core::ffi::c_void) as *mut *mut u16;
        let litlens_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__litlens(lz77 as *mut ::core::ffi::c_void) as *mut *mut u16;
        
        let pos_array = *pos_ptr;
        let dists_array = *dists_ptr;
        let litlens_array = *litlens_ptr;
        
        let pos_l = *pos_array.add(l);
        let pos_lstart = *pos_array.add(lstart);
        let dist_l = *dists_array.add(l);
        let litlen_l = *litlens_array.add(l);
        
        let add_val: crate::types::size_t = if dist_l == 0 {
            1
        } else {
            litlen_l as crate::types::size_t
        };
        
        pos_l + add_val - pos_lstart
    }
}

fn ZopfliLZ77GetHistogramAt(lz77: *const crate::types::ZopfliLZ77Store, lpos: usize, ll_counts: *mut usize, d_counts: *mut usize) {
    let llpos = 288 * (lpos / 288);
    let dpos = 32 * (lpos / 32);
    
    unsafe {
        // Get field pointers via accessor shims
        let ll_counts_field_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__ll_counts(
            lz77 as *mut ::core::ffi::c_void
        ) as *mut *mut usize;
        let ll_counts_arr = *ll_counts_field_ptr;
        
        let d_counts_field_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__d_counts(
            lz77 as *mut ::core::ffi::c_void
        ) as *mut *mut usize;
        let d_counts_arr = *d_counts_field_ptr;
        
        let ll_symbol_field_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__ll_symbol(
            lz77 as *mut ::core::ffi::c_void
        ) as *mut *mut u16;
        let ll_symbol_arr = *ll_symbol_field_ptr;
        
        let d_symbol_field_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__d_symbol(
            lz77 as *mut ::core::ffi::c_void
        ) as *mut *mut u16;
        let d_symbol_arr = *d_symbol_field_ptr;
        
        let dists_field_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__dists(
            lz77 as *mut ::core::ffi::c_void
        ) as *mut *mut u16;
        let dists_arr = *dists_field_ptr;
        
        let size_field_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(
            lz77 as *mut ::core::ffi::c_void
        ) as *mut usize;
        let size = *size_field_ptr;
        
        // Copy ll_counts
        for i in 0..288 {
            *ll_counts.add(i) = *ll_counts_arr.add(llpos + i);
        }
        
        // Adjust ll_counts
        let mut i = lpos + 1;
        while i < llpos + 288 && i < size {
            let sym = *ll_symbol_arr.add(i) as usize;
            *ll_counts.add(sym) = (*ll_counts.add(sym)).wrapping_sub(1);
            i += 1;
        }
        
        // Copy d_counts
        for i in 0..32 {
            *d_counts.add(i) = *d_counts_arr.add(dpos + i);
        }
        
        // Adjust d_counts
        let mut i = lpos + 1;
        while i < dpos + 32 && i < size {
            if *dists_arr.add(i) != 0 {
                let sym = *d_symbol_arr.add(i) as usize;
                *d_counts.add(sym) = (*d_counts.add(sym)).wrapping_sub(1);
            }
            i += 1;
        }
    }
}

pub extern "C" fn ZopfliLZ77GetHistogram(lz77: *const crate::types::ZopfliLZ77Store, lstart: crate::types::size_t, lend: crate::types::size_t, ll_counts: *mut crate::types::size_t, d_counts: *mut crate::types::size_t) {
    unsafe {
        if lstart + 288 * 3 > lend {
            std::ptr::write_bytes(ll_counts, 0, 288);
            std::ptr::write_bytes(d_counts, 0, 32);
            
            let ll_symbol_ptr = *(crate::compat::c2r_field_ptr_ZopfliLZ77Store__ll_symbol(lz77 as *mut ::core::ffi::c_void) as *const *mut u16);
            let dists_ptr = *(crate::compat::c2r_field_ptr_ZopfliLZ77Store__dists(lz77 as *mut ::core::ffi::c_void) as *const *mut u16);
            let d_symbol_ptr = *(crate::compat::c2r_field_ptr_ZopfliLZ77Store__d_symbol(lz77 as *mut ::core::ffi::c_void) as *const *mut u16);
            
            let mut i = lstart;
            while i < lend {
                let ll_sym = *ll_symbol_ptr.add(i) as usize;
                *ll_counts.add(ll_sym) += 1;
                if *dists_ptr.add(i) != 0 {
                    let d_sym = *d_symbol_ptr.add(i) as usize;
                    *d_counts.add(d_sym) += 1;
                }
                i += 1;
            }
        } else {
            crate::src_lz77::ZopfliLZ77GetHistogramAt(lz77, lend - 1, ll_counts, d_counts);
            if lstart > 0 {
                let mut ll_counts2: [crate::types::size_t; 288] = [0; 288];
                let mut d_counts2: [crate::types::size_t; 32] = [0; 32];
                crate::src_lz77::ZopfliLZ77GetHistogramAt(lz77, lstart - 1, ll_counts2.as_mut_ptr(), d_counts2.as_mut_ptr());
                
                for i in 0..288 {
                    *ll_counts.add(i) -= ll_counts2[i];
                }
                for i in 0..32 {
                    *d_counts.add(i) -= d_counts2[i];
                }
            }
        }
    }
}

pub extern "C" fn ZopfliInitBlockState(options: *const crate::types::ZopfliOptions, blockstart: crate::types::size_t, blockend: crate::types::size_t, add_lmc: ::core::ffi::c_int, s: *mut crate::types::ZopfliBlockState) {
    unsafe {
        // s->options = options;
        let options_ptr = crate::compat::c2r_field_ptr_ZopfliBlockState__options(s as *mut ::core::ffi::c_void);
        *(options_ptr as *mut *const crate::types::ZopfliOptions) = options;
        
        // s->blockstart = blockstart;
        let blockstart_ptr = crate::compat::c2r_field_ptr_ZopfliBlockState__blockstart(s as *mut ::core::ffi::c_void);
        *(blockstart_ptr as *mut crate::types::size_t) = blockstart;
        
        // s->blockend = blockend;
        let blockend_ptr = crate::compat::c2r_field_ptr_ZopfliBlockState__blockend(s as *mut ::core::ffi::c_void);
        *(blockend_ptr as *mut crate::types::size_t) = blockend;
        
        // s->lmc handling
        let lmc_ptr = crate::compat::c2r_field_ptr_ZopfliBlockState__lmc(s as *mut ::core::ffi::c_void);
        
        if add_lmc != 0 {
            let lmc = libc::malloc(std::mem::size_of::<crate::types::ZopfliLongestMatchCache>()) as *mut crate::types::ZopfliLongestMatchCache;
            *(lmc_ptr as *mut *mut crate::types::ZopfliLongestMatchCache) = lmc;
            crate::src_cache::ZopfliInitCache(blockend - blockstart, lmc);
        } else {
            *(lmc_ptr as *mut *mut crate::types::ZopfliLongestMatchCache) = std::ptr::null_mut();
        }
    }
}

pub extern "C" fn ZopfliCleanBlockState(s: *mut crate::types::ZopfliBlockState) {
    unsafe {
        let lmc_ptr = crate::compat::c2r_field_ptr_ZopfliBlockState__lmc(s as *mut ::core::ffi::c_void);
        let lmc = *(lmc_ptr as *mut *mut crate::types::ZopfliLongestMatchCache);
        
        if !lmc.is_null() {
            crate::src_cache::ZopfliCleanCache(lmc);
            libc::free(lmc as *mut ::core::ffi::c_void);
        }
    }
}

fn GetLengthScore(length: i32, distance: i32) -> i32 {
    if distance > 1024 {
        length - 1
    } else {
        length
    }
}

pub extern "C" fn ZopfliVerifyLenDist(data: *const ::core::ffi::c_uchar, datasize: crate::types::size_t, pos: crate::types::size_t, dist: ::core::ffi::c_ushort, length: ::core::ffi::c_ushort) {
    assert!(pos + (length as usize) <= datasize);
    
    for i in 0..(length as usize) {
        unsafe {
            let idx1 = pos - (dist as usize) + i;
            let idx2 = pos + i;
            if *data.add(idx1) != *data.add(idx2) {
                assert_eq!(*data.add(idx1), *data.add(idx2));
                break;
            }
        }
    }
}

fn GetMatch(scan: *const u8, r#match: *const u8, end: *const u8, safe_end: *const u8) -> *const u8 {
    let mut scan = scan;
    let mut r#match = r#match;
    
    if std::mem::size_of::<usize>() == 8 {
        unsafe {
            while scan < safe_end && *(scan as *const u64) == *(r#match as *const u64) {
                scan = scan.add(8);
                r#match = r#match.add(8);
            }
        }
    } else if std::mem::size_of::<u32>() == 4 {
        unsafe {
            while scan < safe_end && *(scan as *const u32) == *(r#match as *const u32) {
                scan = scan.add(4);
                r#match = r#match.add(4);
            }
        }
    } else {
        unsafe {
            while scan < safe_end && *scan == *r#match {
                scan = scan.add(1);
                r#match = r#match.add(1);
                if scan >= safe_end || *scan != *r#match { break; }
                scan = scan.add(1);
                r#match = r#match.add(1);
                if scan >= safe_end || *scan != *r#match { break; }
                scan = scan.add(1);
                r#match = r#match.add(1);
                if scan >= safe_end || *scan != *r#match { break; }
                scan = scan.add(1);
                r#match = r#match.add(1);
                if scan >= safe_end || *scan != *r#match { break; }
                scan = scan.add(1);
                r#match = r#match.add(1);
                if scan >= safe_end || *scan != *r#match { break; }
                scan = scan.add(1);
                r#match = r#match.add(1);
                if scan >= safe_end || *scan != *r#match { break; }
                scan = scan.add(1);
                r#match = r#match.add(1);
                if scan >= safe_end || *scan != *r#match { break; }
                scan = scan.add(1);
                r#match = r#match.add(1);
            }
        }
    }
    
    unsafe {
        while scan != end && *scan == *r#match {
            scan = scan.add(1);
            r#match = r#match.add(1);
        }
    }
    
    scan
}

fn TryGetFromLongestMatchCache(s: *mut crate::types::ZopfliBlockState, pos: usize, limit: *mut usize, sublen: *mut u16, distance: *mut u16, length: *mut u16) -> i32 {
    unsafe {
        // Get blockstart via accessor shim
        let blockstart_ptr = crate::compat::c2r_field_ptr_ZopfliBlockState__blockstart(s as *mut ::core::ffi::c_void);
        let blockstart = *(blockstart_ptr as *const usize);
        
        let lmcpos: usize = pos - blockstart;
        
        // Get lmc via accessor shim
        let lmc_ptr = crate::compat::c2r_field_ptr_ZopfliBlockState__lmc(s as *mut ::core::ffi::c_void);
        let lmc = *(lmc_ptr as *const *mut crate::types::ZopfliLongestMatchCache);
        
        if lmc.is_null() {
            return 0;
        }
        
        // Get length and dist arrays from lmc - since ZopfliLongestMatchCache is opaque,
        // we need to use the cache functions instead of direct field access
        // We'll use ZopfliMaxCachedSublen to check cache availability
        
        // For cache_available check, we need length[lmcpos] and dist[lmcpos]
        // Since the type is opaque, we use the API functions
        let max_cached = crate::src_cache::ZopfliMaxCachedSublen(lmc as *const _, lmcpos as crate::types::size_t, 258 as crate::types::size_t);
        
        // Simplified logic: if max_cached > 0, cache is available
        let cache_available: u8 = if max_cached > 0 { 1 } else { 0 };
        
        let limit_val = *limit;
        let limit_ok_for_cache: u8 = if cache_available != 0 && 
            (limit_val == 258 || (max_cached as usize) >= limit_val || 
             (!sublen.is_null() && (max_cached as usize) >= limit_val)) { 1 } else { 0 };
        
        if limit_ok_for_cache != 0 && cache_available != 0 {
            let max_sublen = crate::src_cache::ZopfliMaxCachedSublen(lmc as *const _, lmcpos as crate::types::size_t, 258 as crate::types::size_t);
            
            if sublen.is_null() || (max_sublen as usize) >= limit_val {
                // Use max_cached as length approximation
                let mut len_val = max_cached as u16;
                if (len_val as usize) > limit_val {
                    len_val = limit_val as u16;
                }
                *length = len_val;
                
                if !sublen.is_null() {
                    crate::src_cache::ZopfliCacheToSublen(lmc as *const _, lmcpos as crate::types::size_t, len_val as crate::types::size_t, sublen);
                    *distance = *sublen.offset(len_val as isize);
                } else {
                    // Without direct field access, use sublen array approach
                    let mut temp_sublen: [u16; 259] = [0; 259];
                    crate::src_cache::ZopfliCacheToSublen(lmc as *const _, lmcpos as crate::types::size_t, len_val as crate::types::size_t, temp_sublen.as_mut_ptr());
                    *distance = temp_sublen[len_val as usize];
                }
                return 1;
            }
            
            // Update limit
            *limit = max_cached as usize;
        }
        
        0
    }
}

fn StoreInLongestMatchCache(s: *mut crate::types::ZopfliBlockState, pos: usize, limit: usize, sublen: *const u16, distance: u16, length: u16) {
    unsafe {
        let blockstart_ptr = crate::compat::c2r_field_ptr_ZopfliBlockState__blockstart(s as *mut ::core::ffi::c_void);
        let blockstart = *(blockstart_ptr as *const usize);
        let lmcpos = pos - blockstart;
        
        let lmc_ptr = crate::compat::c2r_field_ptr_ZopfliBlockState__lmc(s as *mut ::core::ffi::c_void);
        let lmc = *(lmc_ptr as *const *mut crate::types::ZopfliLongestMatchCache);
        
        if lmc.is_null() {
            return;
        }
        
        let lmc_length_ptr = crate::compat::c2r_field_ptr_ZopfliLongestMatchCache__length(lmc as *mut ::core::ffi::c_void);
        let lmc_length = lmc_length_ptr as *mut u16;
        let lmc_dist_ptr = crate::compat::c2r_field_ptr_ZopfliLongestMatchCache__dist(lmc as *mut ::core::ffi::c_void);
        let lmc_dist = lmc_dist_ptr as *mut u16;
        
        let cache_available: u8 = if *lmc_length.add(lmcpos) == 0 || *lmc_dist.add(lmcpos) != 0 {
            1
        } else {
            0
        };
        
        if limit == 258 && !sublen.is_null() && cache_available == 0 {
            assert!(*lmc_length.add(lmcpos) == 1 && *lmc_dist.add(lmcpos) == 0);
            
            *lmc_dist.add(lmcpos) = if length < 3 { 0 } else { distance };
            *lmc_length.add(lmcpos) = if length < 3 { 0 } else { length };
            
            assert!(!(*lmc_length.add(lmcpos) == 1 && *lmc_dist.add(lmcpos) == 0));
            
            crate::src_cache::ZopfliSublenToCache(sublen, lmcpos as crate::types::size_t, length as crate::types::size_t, lmc);
        }
    }
}

pub extern "C" fn ZopfliFindLongestMatch(s: *mut crate::types::ZopfliBlockState, h: *const crate::types::ZopfliHash, array: *const ::core::ffi::c_uchar, pos: crate::types::size_t, size: crate::types::size_t, limit: crate::types::size_t, sublen: *mut ::core::ffi::c_ushort, distance: *mut ::core::ffi::c_ushort, length: *mut ::core::ffi::c_ushort) {
    let hpos: u16 = (pos & (32768 - 1)) as u16;
    let mut bestdist: u16 = 0;
    let mut bestlength: u16 = 1;
    let mut limit = limit;
    
    let mut chain_counter: i32 = 8192;
    let mut dist: u32;
    
    unsafe {
        let hhead_ptr = crate::compat::c2r_field_ptr_ZopfliHash__head(h as *mut ::core::ffi::c_void);
        let mut hhead: *mut i32 = *(hhead_ptr as *const *mut i32);
        let hprev_ptr = crate::compat::c2r_field_ptr_ZopfliHash__prev(h as *mut ::core::ffi::c_void);
        let mut hprev: *mut u16 = *(hprev_ptr as *const *mut u16);
        let hhashval_ptr = crate::compat::c2r_field_ptr_ZopfliHash__hashval(h as *mut ::core::ffi::c_void);
        let mut hhashval: *mut i32 = *(hhashval_ptr as *const *mut i32);
        let hval_ptr = crate::compat::c2r_field_ptr_ZopfliHash__val(h as *mut ::core::ffi::c_void);
        let mut hval: i32 = *(hval_ptr as *const i32);
        
        if crate::src_lz77::TryGetFromLongestMatchCache(s, pos, &mut limit as *mut usize, sublen, distance, length) != 0 {
            return;
        }
        
        if size - pos < 3 {
            *length = 0;
            *distance = 0;
            return;
        }
        
        if pos + limit > size {
            limit = size - pos;
        }
        let arrayend: *const u8 = array.add(pos + limit);
        let arrayend_safe: *const u8 = arrayend.sub(8);
        
        let mut pp: u16 = *hhead.offset(hval as isize) as u16;
        let mut p: u16 = *hprev.offset(pp as isize);
        
        dist = if p < pp { (pp - p) as u32 } else { ((32768 - p as u32) + pp as u32) };
        
        while dist < 32768 {
            let mut currentlength: u16 = 0;
            
            if dist > 0 {
                let scan_start: *const u8 = array.add(pos);
                let match_start: *const u8 = array.add(pos - dist as usize);
                
                if pos + bestlength as usize >= size || *scan_start.add(bestlength as usize) == *match_start.add(bestlength as usize) {
                    let mut scan: *const u8 = scan_start;
                    let mut match_ptr: *const u8 = match_start;
                    
                    let same_ptr = crate::compat::c2r_field_ptr_ZopfliHash__same(h as *mut ::core::ffi::c_void);
                    let same_arr: *mut u16 = *(same_ptr as *const *mut u16);
                    let same0: u16 = *same_arr.offset((pos & (32768 - 1)) as isize);
                    
                    if same0 > 2 && *scan == *match_ptr {
                        let same1: u16 = *same_arr.offset(((pos - dist as usize) & (32768 - 1)) as isize);
                        let mut same: u16 = if same0 < same1 { same0 } else { same1 };
                        if same as usize > limit { same = limit as u16; }
                        scan = scan.add(same as usize);
                        match_ptr = match_ptr.add(same as usize);
                    }
                    
                    scan = crate::src_lz77::GetMatch(scan, match_ptr, arrayend, arrayend_safe);
                    currentlength = (scan as usize - scan_start as usize) as u16;
                }
                
                if currentlength > bestlength {
                    if !sublen.is_null() {
                        for j in (bestlength + 1)..=currentlength {
                            *sublen.offset(j as isize) = dist as u16;
                        }
                    }
                    bestdist = dist as u16;
                    bestlength = currentlength;
                    if currentlength as usize >= limit { break; }
                }
            }
            
            let head2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__head2(h as *mut ::core::ffi::c_void);
            let head2: *mut i32 = *(head2_ptr as *const *mut i32);
            let same_ptr = crate::compat::c2r_field_ptr_ZopfliHash__same(h as *mut ::core::ffi::c_void);
            let same_arr: *mut u16 = *(same_ptr as *const *mut u16);
            let val2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__val2(h as *mut ::core::ffi::c_void);
            let val2: i32 = *(val2_ptr as *const i32);
            let hashval2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__hashval2(h as *mut ::core::ffi::c_void);
            let hashval2: *mut i32 = *(hashval2_ptr as *const *mut i32);
            
            if hhead != head2 && bestlength >= *same_arr.offset(hpos as isize) && val2 == *hashval2.offset(p as isize) {
                hhead = head2;
                let prev2_ptr = crate::compat::c2r_field_ptr_ZopfliHash__prev2(h as *mut ::core::ffi::c_void);
                hprev = *(prev2_ptr as *const *mut u16);
                hhashval = hashval2;
                hval = val2;
            }
            
            pp = p;
            p = *hprev.offset(p as isize);
            if p == pp { break; }
            
            dist += if p < pp { (pp - p) as u32 } else { ((32768 - p as u32) + pp as u32) };
            
            chain_counter -= 1;
            if chain_counter <= 0 { break; }
        }
        
        crate::src_lz77::StoreInLongestMatchCache(s, pos, limit, sublen as *const u16, bestdist, bestlength);
        
        *distance = bestdist;
        *length = bestlength;
    }
}

pub extern "C" fn ZopfliLZ77Greedy(s: *mut crate::types::ZopfliBlockState, in_: *const ::core::ffi::c_uchar, instart: crate::types::size_t, inend: crate::types::size_t, store: *mut crate::types::ZopfliLZ77Store, h: *mut crate::types::ZopfliHash) {
    let mut i: crate::types::size_t;
    let mut j: crate::types::size_t;
    let mut leng: ::core::ffi::c_ushort = 0;
    let mut dist: ::core::ffi::c_ushort = 0;
    let mut lengthscore: i32;
    let windowstart: crate::types::size_t = if instart > 32768 { instart - 32768 } else { 0 };
    let mut dummysublen: [::core::ffi::c_ushort; 259] = [0; 259];
    
    let mut prev_length: u32 = 0;
    let mut prev_match: u32 = 0;
    let mut prevlengthscore: i32;
    let mut match_available: i32 = 0;
    
    if instart == inend {
        return;
    }
    
    crate::src_hash::ZopfliResetHash(32768, h);
    crate::src_hash::ZopfliWarmupHash(in_, windowstart, inend, h);
    i = windowstart;
    while i < instart {
        crate::src_hash::ZopfliUpdateHash(in_, i, inend, h);
        i += 1;
    }
    
    i = instart;
    while i < inend {
        crate::src_hash::ZopfliUpdateHash(in_, i, inend, h);
        
        crate::src_lz77::ZopfliFindLongestMatch(
            s, h as *const _, in_, i, inend, 258,
            dummysublen.as_mut_ptr(), &mut dist, &mut leng
        );
        lengthscore = crate::src_lz77::GetLengthScore(leng as i32, dist as i32);
        
        prevlengthscore = crate::src_lz77::GetLengthScore(prev_length as i32, prev_match as i32);
        if match_available != 0 {
            match_available = 0;
            if lengthscore > prevlengthscore + 1 {
                let lit = unsafe { *in_.add(i - 1) };
                crate::src_lz77::ZopfliStoreLitLenDist(lit as ::core::ffi::c_ushort, 0, i - 1, store);
                if lengthscore >= 3 && leng < 258 {
                    match_available = 1;
                    prev_length = leng as u32;
                    prev_match = dist as u32;
                    i += 1;
                    continue;
                }
            } else {
                leng = prev_length as ::core::ffi::c_ushort;
                dist = prev_match as ::core::ffi::c_ushort;
                lengthscore = prevlengthscore;
                
                crate::src_lz77::ZopfliVerifyLenDist(in_, inend, i - 1, dist, leng);
                crate::src_lz77::ZopfliStoreLitLenDist(leng, dist, i - 1, store);
                j = 2;
                while j < leng as crate::types::size_t {
                    assert!(i < inend);
                    i += 1;
                    crate::src_hash::ZopfliUpdateHash(in_, i, inend, h);
                    j += 1;
                }
                i += 1;
                continue;
            }
        } else if lengthscore >= 3 && leng < 258 {
            match_available = 1;
            prev_length = leng as u32;
            prev_match = dist as u32;
            i += 1;
            continue;
        }
        
        if lengthscore >= 3 {
            crate::src_lz77::ZopfliVerifyLenDist(in_, inend, i, dist, leng);
            crate::src_lz77::ZopfliStoreLitLenDist(leng, dist, i, store);
        } else {
            leng = 1;
            let lit = unsafe { *in_.add(i) };
            crate::src_lz77::ZopfliStoreLitLenDist(lit as ::core::ffi::c_ushort, 0, i, store);
        }
        j = 1;
        while j < leng as crate::types::size_t {
            assert!(i < inend);
            i += 1;
            crate::src_hash::ZopfliUpdateHash(in_, i, inend, h);
            j += 1;
        }
        i += 1;
    }
}
