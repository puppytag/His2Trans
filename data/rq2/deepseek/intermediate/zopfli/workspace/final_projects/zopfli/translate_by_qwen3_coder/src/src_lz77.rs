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

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_lz77_1
// c_function: ZopfliInitLZ77Store
// rust_file: src_lz77.rs
// rust_signature: pub extern "C" fn ZopfliInitLZ77Store(data: *const ::core::ffi::c_uchar, store: *mut crate::types::ZopfliLZ77Store)
// c_first_line: void ZopfliInitLZ77Store(const unsigned char* data, ZopfliLZ77Store* store) {
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_lz77_1/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find function `c2r_field_ptr_ZopfliLZ77Store__ll_counts` in module `crate::compat`
//      --> src/src_lz77.rs:28:26
//       |
//       |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//       |
//       |
//       |     ----------------------------------------------------------------------------------------------------------- similarly named function `c2r_field_ptr_ZopfliLZ77Store__d_counts` defined here
//       |
// =================================
pub extern "C" fn ZopfliInitLZ77Store(data: *const ::core::ffi::c_uchar, store: *mut crate::types::ZopfliLZ77Store) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_lz77::ZopfliInitLZ77Store(data as _, store as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_lz77_1
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_lz77_1/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn ZopfliInitLZ77Store(data: *const ::core::ffi::c_uchar, store: *mut crate::types::ZopfliLZ77Store) {
    if store.is_null() {
        return;
    }
    unsafe {
        let store_void = store as *mut ::core::ffi::c_void;
        *(crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(store_void) as *mut crate::types::size_t) = 0;
        *(crate::compat::c2r_field_ptr_ZopfliLZ77Store__litlens(store_void) as *mut *mut ::core::ffi::c_ushort) = std::ptr::null_mut();
        *(crate::compat::c2r_field_ptr_ZopfliLZ77Store__dists(store_void) as *mut *mut ::core::ffi::c_ushort) = std::ptr::null_mut();
        *(crate::compat::c2r_field_ptr_ZopfliLZ77Store__pos(store_void) as *mut *mut crate::types::size_t) = std::ptr::null_mut();
        *(crate::compat::c2r_field_ptr_ZopfliLZ77Store__ll_symbol(store_void) as *mut *mut ::core::ffi::c_ushort) = std::ptr::null_mut();
        *(crate::compat::c2r_field_ptr_ZopfliLZ77Store__d_symbol(store_void) as *mut *mut ::core::ffi::c_ushort) = std::ptr::null_mut();
        *(crate::compat::c2r_field_ptr_ZopfliLZ77Store__d_counts(store_void) as *mut *mut crate::types::size_t) = std::ptr::null_mut();
        *(crate::compat::c2r_field_ptr_ZopfliLZ77Store__ll_counts(store_void) as *mut *mut crate::types::size_t) = std::ptr::null_mut();
        *(crate::compat::c2r_field_ptr_ZopfliLZ77Store__data(store_void) as *mut *const ::core::ffi::c_uchar) = data;
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_lz77_1
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn ZopfliCleanLZ77Store(store: *mut crate::types::ZopfliLZ77Store) {
    if store.is_null() {
        return;
    }
    unsafe {
        let s = store as *mut crate::types::ZopfliLZ77Store;
        libc::free((*s).litlens as *mut std::ffi::c_void);
        libc::free((*s).dists as *mut std::ffi::c_void);
        libc::free((*s).pos as *mut std::ffi::c_void);
        libc::free((*s).ll_symbol as *mut std::ffi::c_void);
        libc::free((*s).d_symbol as *mut std::ffi::c_void);
        libc::free((*s).ll_counts as *mut std::ffi::c_void);
        libc::free((*s).d_counts as *mut std::ffi::c_void);
    }
}

fn CeilDiv(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}

pub extern "C" fn ZopfliCopyLZ77Store(source: *const crate::types::ZopfliLZ77Store, dest: *mut crate::types::ZopfliLZ77Store) {
    unsafe {
        let source_size_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(source as *mut ::core::ffi::c_void) as *const crate::types::size_t;
        let source_size = *source_size_ptr as usize;
        let llsize = 288 * crate::src_lz77::CeilDiv(source_size, 288);
        let dsize = 32 * crate::src_lz77::CeilDiv(source_size, 32);
        crate::src_lz77::ZopfliCleanLZ77Store(dest);
        let source_data_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__data(source as *mut ::core::ffi::c_void) as *const ::core::ffi::c_uchar;
        crate::src_lz77::ZopfliInitLZ77Store(source_data_ptr, dest);
        let dest_litlens_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__litlens(dest as *mut ::core::ffi::c_void) as *mut *mut u16;
        *dest_litlens_ptr = libc::malloc((std::mem::size_of::<u16>() * source_size) as usize) as *mut u16;
        let dest_dists_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__dists(dest as *mut ::core::ffi::c_void) as *mut *mut u16;
        *dest_dists_ptr = libc::malloc((std::mem::size_of::<u16>() * source_size) as usize) as *mut u16;
        let dest_pos_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__pos(dest as *mut ::core::ffi::c_void) as *mut *mut usize;
        *dest_pos_ptr = libc::malloc((std::mem::size_of::<usize>() * source_size) as usize) as *mut usize;
        let dest_ll_symbol_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__ll_symbol(dest as *mut ::core::ffi::c_void) as *mut *mut u16;
        *dest_ll_symbol_ptr = libc::malloc((std::mem::size_of::<u16>() * source_size) as usize) as *mut u16;
        let dest_d_symbol_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__d_symbol(dest as *mut ::core::ffi::c_void) as *mut *mut u16;
        *dest_d_symbol_ptr = libc::malloc((std::mem::size_of::<u16>() * source_size) as usize) as *mut u16;
        let dest_ll_counts_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__ll_counts(dest as *mut ::core::ffi::c_void) as *mut *mut usize;
        *dest_ll_counts_ptr = libc::malloc((std::mem::size_of::<usize>() * llsize) as usize) as *mut usize;
        let dest_d_counts_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__d_counts(dest as *mut ::core::ffi::c_void) as *mut *mut usize;
        *dest_d_counts_ptr = libc::malloc((std::mem::size_of::<usize>() * dsize) as usize) as *mut usize;
        if (*dest_litlens_ptr).is_null() || (*dest_dists_ptr).is_null() {
            libc::exit(-1);
        }
        if (*dest_pos_ptr).is_null() {
            libc::exit(-1);
        }
        if (*dest_ll_symbol_ptr).is_null() || (*dest_d_symbol_ptr).is_null() {
            libc::exit(-1);
        }
        if (*dest_ll_counts_ptr).is_null() || (*dest_d_counts_ptr).is_null() {
            libc::exit(-1);
        }
        let dest_size_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(dest as *mut ::core::ffi::c_void) as *mut crate::types::size_t;
        *dest_size_ptr = source_size as crate::types::size_t;
        let source_litlens_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__litlens(source as *mut ::core::ffi::c_void) as *const *const u16;
        let source_dists_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__dists(source as *mut ::core::ffi::c_void) as *const *const u16;
        let source_pos_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__pos(source as *mut ::core::ffi::c_void) as *const *const usize;
        let source_ll_symbol_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__ll_symbol(source as *mut ::core::ffi::c_void) as *const *const u16;
        let source_d_symbol_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__d_symbol(source as *mut ::core::ffi::c_void) as *const *const u16;
        let source_ll_counts_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__ll_counts(source as *mut ::core::ffi::c_void) as *const *const usize;
        let source_d_counts_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__d_counts(source as *mut ::core::ffi::c_void) as *const *const usize;
        for i in 0..source_size {
            *(*dest_litlens_ptr).add(i) = *(*source_litlens_ptr).add(i);
            *(*dest_dists_ptr).add(i) = *(*source_dists_ptr).add(i);
            *(*dest_pos_ptr).add(i) = *(*source_pos_ptr).add(i);
            *(*dest_ll_symbol_ptr).add(i) = *(*source_ll_symbol_ptr).add(i);
            *(*dest_d_symbol_ptr).add(i) = *(*source_d_symbol_ptr).add(i);
        }
        for i in 0..llsize {
            *(*dest_ll_counts_ptr).add(i) = *(*source_ll_counts_ptr).add(i);
        }
        for i in 0..dsize {
            *(*dest_d_counts_ptr).add(i) = *(*source_d_counts_ptr).add(i);
        }
    }
}

pub extern "C" fn ZopfliStoreLitLenDist(length: ::core::ffi::c_ushort, dist: ::core::ffi::c_ushort, pos: crate::types::size_t, store: *mut crate::types::ZopfliLZ77Store) {
    unsafe {
        let store = &mut *store;
        let origsize = 0;
        let llstart = 288 * (origsize / 288);
        let dstart = 32 * (origsize / 32);
        let mut i: crate::types::size_t = 0;

        if origsize % 288 == 0 {
            let mut llsize = origsize;
            for i in 0..288 {
                if llsize & (llsize - 1) == 0 {
                    let new_size = if llsize == 0 { 1 } else { llsize * 2 };
                    let new_ptr = if llsize == 0 {
                        libc::malloc((std::mem::size_of::<crate::types::size_t>() * new_size) as usize) as *mut crate::types::size_t
                    } else {
                        libc::realloc(store.ll_counts as *mut libc::c_void, (std::mem::size_of::<crate::types::size_t>() * new_size) as usize) as *mut crate::types::size_t
                    };
                    store.ll_counts = new_ptr;
                }
                let val = if origsize == 0 { 0 } else { *store.ll_counts.offset((origsize - 288 + i) as isize) };
                *store.ll_counts.offset(llsize as isize) = val;
                llsize += 1;
            }
        }
        if origsize % 32 == 0 {
            let mut dsize = origsize;
            for i in 0..32 {
                if dsize & (dsize - 1) == 0 {
                    let new_size = if dsize == 0 { 1 } else { dsize * 2 };
                    let new_ptr = if dsize == 0 {
                        libc::malloc((std::mem::size_of::<crate::types::size_t>() * new_size) as usize) as *mut crate::types::size_t
                    } else {
                        libc::realloc(store.d_counts as *mut libc::c_void, (std::mem::size_of::<crate::types::size_t>() * new_size) as usize) as *mut crate::types::size_t
                    };
                    store.d_counts = new_ptr;
                }
                let val = if origsize == 0 { 0 } else { *store.d_counts.offset((origsize - 32 + i) as isize) };
                *store.d_counts.offset(dsize as isize) = val;
                dsize += 1;
            }
        }

        let mut temp_size = origsize;
        if temp_size & (temp_size - 1) == 0 {
            let new_size = if temp_size == 0 { 1 } else { temp_size * 2 };
            let new_ptr = if temp_size == 0 {
                libc::malloc((std::mem::size_of::<::core::ffi::c_ushort>() * new_size) as usize) as *mut ::core::ffi::c_ushort
            } else {
                libc::realloc(store.litlens as *mut libc::c_void, (std::mem::size_of::<::core::ffi::c_ushort>() * new_size) as usize) as *mut ::core::ffi::c_ushort
            };
            store.litlens = new_ptr;
        }
        *store.litlens.offset(temp_size as isize) = length;
        temp_size += 1;

        temp_size = origsize;
        if temp_size & (temp_size - 1) == 0 {
            let new_size = if temp_size == 0 { 1 } else { temp_size * 2 };
            let new_ptr = if temp_size == 0 {
                libc::malloc((std::mem::size_of::<::core::ffi::c_ushort>() * new_size) as usize) as *mut ::core::ffi::c_ushort
            } else {
                libc::realloc(store.dists as *mut libc::c_void, (std::mem::size_of::<::core::ffi::c_ushort>() * new_size) as usize) as *mut ::core::ffi::c_ushort
            };
            store.dists = new_ptr;
        }
        *store.dists.offset(temp_size as isize) = dist;
        temp_size += 1;

        temp_size = origsize;
        if temp_size & (temp_size - 1) == 0 {
            let new_size = if temp_size == 0 { 1 } else { temp_size * 2 };
            let new_ptr = if temp_size == 0 {
                libc::malloc((std::mem::size_of::<crate::types::size_t>() * new_size) as usize) as *mut crate::types::size_t
            } else {
                libc::realloc(store.pos as *mut libc::c_void, (std::mem::size_of::<crate::types::size_t>() * new_size) as usize) as *mut crate::types::size_t
            };
            store.pos = new_ptr;
        }
        *store.pos.offset(temp_size as isize) = pos;
        temp_size += 1;

        let _ = length < 259;

        if dist == 0 {
            temp_size = origsize;
            if temp_size & (temp_size - 1) == 0 {
                let new_size = if temp_size == 0 { 1 } else { temp_size * 2 };
                let new_ptr = if temp_size == 0 {
                    libc::malloc((std::mem::size_of::<::core::ffi::c_ushort>() * new_size) as usize) as *mut ::core::ffi::c_ushort
                } else {
                    libc::realloc(store.ll_symbol as *mut libc::c_void, (std::mem::size_of::<::core::ffi::c_ushort>() * new_size) as usize) as *mut ::core::ffi::c_ushort
                };
                store.ll_symbol = new_ptr;
            }
            *store.ll_symbol.offset(temp_size as isize) = length;
            temp_size += 1;

            temp_size = origsize;
            if temp_size & (temp_size - 1) == 0 {
                let new_size = if temp_size == 0 { 1 } else { temp_size * 2 };
                let new_ptr = if temp_size == 0 {
                    libc::malloc((std::mem::size_of::<::core::ffi::c_ushort>() * new_size) as usize) as *mut ::core::ffi::c_ushort
                } else {
                    libc::realloc(store.d_symbol as *mut libc::c_void, (std::mem::size_of::<::core::ffi::c_ushort>() * new_size) as usize) as *mut ::core::ffi::c_ushort
                };
                store.d_symbol = new_ptr;
            }
            *store.d_symbol.offset(temp_size as isize) = 0;
            temp_size += 1;
            *store.ll_counts.offset((llstart + length as crate::types::size_t) as isize) += 1;
        } else {
            temp_size = origsize;
            if temp_size & (temp_size - 1) == 0 {
                let new_size = if temp_size == 0 { 1 } else { temp_size * 2 };
                let new_ptr = if temp_size == 0 {
                    libc::malloc((std::mem::size_of::<::core::ffi::c_ushort>() * new_size) as usize) as *mut ::core::ffi::c_ushort
                } else {
                    libc::realloc(store.ll_symbol as *mut libc::c_void, (std::mem::size_of::<::core::ffi::c_ushort>() * new_size) as usize) as *mut ::core::ffi::c_ushort
                };
                store.ll_symbol = new_ptr;
            }
            let lsym = length;
            *store.ll_symbol.offset(temp_size as isize) = lsym;
            temp_size += 1;

            temp_size = origsize;
            if temp_size & (temp_size - 1) == 0 {
                let new_size = if temp_size == 0 { 1 } else { temp_size * 2 };
                let new_ptr = if temp_size == 0 {
                    libc::malloc((std::mem::size_of::<::core::ffi::c_ushort>() * new_size) as usize) as *mut ::core::ffi::c_ushort
                } else {
                    libc::realloc(store.d_symbol as *mut libc::c_void, (std::mem::size_of::<::core::ffi::c_ushort>() * new_size) as usize) as *mut ::core::ffi::c_ushort
                };
                store.d_symbol = new_ptr;
            }
            let dsym = dist;
            *store.d_symbol.offset(temp_size as isize) = dsym;
            temp_size += 1;

            *store.ll_counts.offset((llstart + lsym as crate::types::size_t) as isize) += 1;
            *store.d_counts.offset((dstart + dsym as crate::types::size_t) as isize) += 1;
        }
    }
}

pub extern "C" fn ZopfliAppendLZ77Store(store: *const crate::types::ZopfliLZ77Store, target: *mut crate::types::ZopfliLZ77Store) {
    unsafe {
        let store_size_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__size(store as *mut ::core::ffi::c_void) as *const crate::types::size_t;
        let store_size = *store_size_ptr;
        let mut i: crate::types::size_t = 0;
        while i < store_size {
            let litlens_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__litlens(store as *mut ::core::ffi::c_void) as *const ::core::ffi::c_ushort;
            let dists_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__dists(store as *mut ::core::ffi::c_void) as *const ::core::ffi::c_ushort;
            let pos_ptr = crate::compat::c2r_field_ptr_ZopfliLZ77Store__pos(store as *mut ::core::ffi::c_void) as *const crate::types::size_t;
            let length = *litlens_ptr.offset(i as isize);
            let dist = *dists_ptr.offset(i as isize);
            let pos = *pos_ptr.offset(i as isize);
            crate::src_lz77::ZopfliStoreLitLenDist(length, dist, pos, target);
            i += 1;
        }
    }
}

pub extern "C" fn ZopfliLZ77GetByteRange(lz77: *const crate::types::ZopfliLZ77Store, lstart: crate::types::size_t, lend: crate::types::size_t) -> crate::types::size_t {
    if lstart == lend {
        return 0;
    }
    let l = lend - 1;
    let lz77_ptr = lz77 as *mut ::core::ffi::c_void;
    let pos_ptr = unsafe {
        crate::compat::c2r_field_ptr_ZopfliLZ77Store__pos(lz77_ptr) as *const crate::types::size_t
    };
    let litlens_ptr = unsafe {
        crate::compat::c2r_field_ptr_ZopfliLZ77Store__litlens(lz77_ptr) as *const crate::types::size_t
    };
    let dists_ptr = unsafe {
        crate::compat::c2r_field_ptr_ZopfliLZ77Store__dists(lz77_ptr) as *const crate::types::size_t
    };
    let pos_l = unsafe { *pos_ptr.add(l) };
    let pos_lstart = unsafe { *pos_ptr.add(lstart) };
    let dist_l = unsafe { *dists_ptr.add(l) };
    let litlen_l = unsafe { *litlens_ptr.add(l) };
    if dist_l == 0 {
        (pos_l + 1).wrapping_sub(pos_lstart)
    } else {
        (pos_l + litlen_l).wrapping_sub(pos_lstart)
    }
}

fn ZopfliLZ77GetHistogramAt(lz77: *const crate::types::ZopfliLZ77Store, lpos: usize, ll_counts: *mut usize, d_counts: *mut usize) {
    let llpos = 288 * (lpos / 288);
    let dpos = 32 * (lpos / 32);
    unsafe {
        let ll_counts_base = (*lz77).ll_counts;
        let ll_symbol_base = (*lz77).ll_symbol;
        let d_counts_base = (*lz77).d_counts;
        let d_symbol_base = (*lz77).d_symbol;
        let dists_base = (*lz77).dists;

        for i in 0..288 {
            *ll_counts.add(i) = *ll_counts_base.add(llpos + i);
        }
        for i in 0..32 {
            *d_counts.add(i) = *d_counts_base.add(dpos + i);
        }
    }
}

pub extern "C" fn ZopfliLZ77GetHistogram(lz77: *const crate::types::ZopfliLZ77Store, lstart: crate::types::size_t, lend: crate::types::size_t, ll_counts: *mut crate::types::size_t, d_counts: *mut crate::types::size_t) {
    let lstart = lstart as usize;
    let lend = lend as usize;
    if lstart + 288 * 3 > lend {
        unsafe {
            std::ptr::write_bytes(ll_counts, 0, 288);
            std::ptr::write_bytes(d_counts, 0, 32);
        }
        let mut i = lstart;
        while i < lend {
            unsafe {
                let ll_symbol_ptr = (*lz77).ll_symbol;
                let idx = *ll_symbol_ptr.add(i) as usize;
                *ll_counts.add(idx) += 1;
                let dists_ptr = (*lz77).dists;
                if *dists_ptr.add(i) != 0 {
                    let d_symbol_ptr = (*lz77).d_symbol;
                    let didx = *d_symbol_ptr.add(i) as usize;
                    *d_counts.add(didx) += 1;
                }
            }
            i += 1;
        }
    } else {
        unsafe {
            crate::src_lz77::ZopfliLZ77GetHistogramAt(lz77, lend - 1, ll_counts, d_counts);
        }
        if lstart > 0 {
            let mut ll_counts2: [usize; 288] = [0; 288];
            let mut d_counts2: [usize; 32] = [0; 32];
            unsafe {
                crate::src_lz77::ZopfliLZ77GetHistogramAt(lz77, lstart - 1, ll_counts2.as_mut_ptr(), d_counts2.as_mut_ptr());
            }
            for i in 0..288 {
                unsafe {
                    *ll_counts.add(i) -= ll_counts2[i];
                }
            }
            for i in 0..32 {
                unsafe {
                    *d_counts.add(i) -= d_counts2[i];
                }
            }
        }
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_lz77_10
// c_function: ZopfliInitBlockState
// rust_file: src_lz77.rs
// rust_signature: pub extern "C" fn ZopfliInitBlockState(options: *const crate::types::ZopfliOptions, blockstart: crate::types::size_t, blockend: crate::types::size_t, add_lmc: ::core::ffi::c_int, s: *mut crate::types::ZopfliBlockState)
// c_first_line: void ZopfliInitBlockState(const ZopfliOptions* options,
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_lz77_10/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find function `c2r_field_ptr_ZopfliBlockState__options` in module `crate::compat`
//      --> src/src_lz77.rs:403:42
//       |
//       |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//       |
//       |
//       |     ------------------------------------------------------------------------------------------------------------ similarly named function `c2r_field_ptr_ZopfliBlockState__blockend` defined here
//       |
// =================================
pub extern "C" fn ZopfliInitBlockState(options: *const crate::types::ZopfliOptions, blockstart: crate::types::size_t, blockend: crate::types::size_t, add_lmc: ::core::ffi::c_int, s: *mut crate::types::ZopfliBlockState) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_lz77::ZopfliInitBlockState(options as _, blockstart as _, blockend as _, add_lmc as _, s as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_lz77_10
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_lz77_10/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn ZopfliInitBlockState(options: *const crate::types::ZopfliOptions, blockstart: crate::types::size_t, blockend: crate::types::size_t, add_lmc: ::core::ffi::c_int, s: *mut crate::types::ZopfliBlockState) {
    unsafe {
        let s_ptr = s as *mut ::core::ffi::c_void;
        let options_ptr = crate::compat::c2r_field_ptr_ZopfliBlockState__options(s_ptr) as *mut *const crate::types::ZopfliOptions;
        *options_ptr = options;
        let blockstart_ptr = crate::compat::c2r_field_ptr_ZopfliBlockState__blockstart(s_ptr) as *mut crate::types::size_t;
        *blockstart_ptr = blockstart;
        let blockend_ptr = crate::compat::c2r_field_ptr_ZopfliBlockState__blockend(s_ptr) as *mut crate::types::size_t;
        *blockend_ptr = blockend;
        if add_lmc != 0 {
            let lmc_ptr = crate::compat::c2r_field_ptr_ZopfliBlockState__lmc(s_ptr) as *mut *mut crate::types::ZopfliLongestMatchCache;
            *lmc_ptr = libc::malloc(std::mem::size_of::<crate::types::ZopfliLongestMatchCache>() as usize) as *mut crate::types::ZopfliLongestMatchCache;
            crate::src_cache::ZopfliInitCache(blockend - blockstart, *lmc_ptr);
        } else {
            let lmc_ptr = crate::compat::c2r_field_ptr_ZopfliBlockState__lmc(s_ptr) as *mut *mut crate::types::ZopfliLongestMatchCache;
            *lmc_ptr = std::ptr::null_mut();
        }
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_lz77_10
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn ZopfliCleanBlockState(s: *mut crate::types::ZopfliBlockState) {
    if s.is_null() {
        return;
    }
    let lmc_ptr = unsafe {
        crate::compat::c2r_field_ptr_ZopfliBlockState__lmc(s as *mut ::core::ffi::c_void)
    };
    if !lmc_ptr.is_null() {
        let lmc = lmc_ptr as *mut crate::types::ZopfliLongestMatchCache;
        unsafe {
            crate::src_cache::ZopfliCleanCache(lmc);
            libc::free(lmc as *mut ::core::ffi::c_void);
        }
        unsafe {
            *(crate::compat::c2r_field_ptr_ZopfliBlockState__lmc(s as *mut ::core::ffi::c_void) as *mut *mut crate::types::ZopfliLongestMatchCache) = std::ptr::null_mut();
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
    let _ = data;
    let _ = datasize;
    let _ = pos;
    let _ = dist;
    let _ = length;
}

fn GetMatch(scan: *const libc::c_uchar, r#match: *const libc::c_uchar, end: *const libc::c_uchar, safe_end: *const libc::c_uchar) -> *const libc::c_uchar {
    let mut scan = scan;
    let mut r#match = r#match;
    if std::mem::size_of::<usize>() == 8 {
        while (scan as usize) < (safe_end as usize) && unsafe { *(scan as *const usize) == *(r#match as *const usize) } {
            scan = unsafe { scan.add(8) };
            r#match = unsafe { r#match.add(8) };
        }
    } else if std::mem::size_of::<u32>() == 4 {
        while (scan as usize) < (safe_end as usize) && unsafe { *(scan as *const u32) == *(r#match as *const u32) } {
            scan = unsafe { scan.add(4) };
            r#match = unsafe { r#match.add(4) };
        }
    } else {
        while (scan as usize) < (safe_end as usize) && unsafe { *scan == *r#match } {
            scan = unsafe { scan.add(1) };
            r#match = unsafe { r#match.add(1) };
            if (scan as usize) >= (safe_end as usize) || unsafe { *scan != *r#match } { break; }
            scan = unsafe { scan.add(1) };
            r#match = unsafe { r#match.add(1) };
            if (scan as usize) >= (safe_end as usize) || unsafe { *scan != *r#match } { break; }
            scan = unsafe { scan.add(1) };
            r#match = unsafe { r#match.add(1) };
            if (scan as usize) >= (safe_end as usize) || unsafe { *scan != *r#match } { break; }
            scan = unsafe { scan.add(1) };
            r#match = unsafe { r#match.add(1) };
            if (scan as usize) >= (safe_end as usize) || unsafe { *scan != *r#match } { break; }
            scan = unsafe { scan.add(1) };
            r#match = unsafe { r#match.add(1) };
            if (scan as usize) >= (safe_end as usize) || unsafe { *scan != *r#match } { break; }
            scan = unsafe { scan.add(1) };
            r#match = unsafe { r#match.add(1) };
            if (scan as usize) >= (safe_end as usize) || unsafe { *scan != *r#match } { break; }
            scan = unsafe { scan.add(1) };
            r#match = unsafe { r#match.add(1) };
            if (scan as usize) >= (safe_end as usize) || unsafe { *scan != *r#match } { break; }
            scan = unsafe { scan.add(1) };
            r#match = unsafe { r#match.add(1) };
        }
    }
    while (scan as usize) != (end as usize) && unsafe { *scan == *r#match } {
        scan = unsafe { scan.add(1) };
        r#match = unsafe { r#match.add(1) };
    }
    scan
}

fn TryGetFromLongestMatchCache(s: *mut crate::types::ZopfliBlockState, pos: usize, limit: *mut usize, sublen: *mut u16, distance: *mut u16, length: *mut u16) -> i32 {
    let s_void = s as *mut ::core::ffi::c_void;
    let blockstart_ptr = unsafe { crate::compat::c2r_field_ptr_ZopfliBlockState__blockstart(s_void) as *const usize };
    let blockstart = unsafe { *blockstart_ptr };
    let lmcpos = pos.wrapping_sub(blockstart);
    let lmc_ptr = unsafe { crate::compat::c2r_field_ptr_ZopfliBlockState__lmc(s_void) as *mut crate::types::ZopfliLongestMatchCache };
    let lmc = if lmc_ptr.is_null() { std::ptr::null_mut() } else { lmc_ptr };
    let cache_available = if !lmc.is_null() {
        let length_ptr = unsafe { (*lmc).length } as *mut crate::types::size_t;
        let dist_ptr = unsafe { (*lmc).dist } as *mut u16;
        let length_val = unsafe { *length_ptr.add(lmcpos) };
        let dist_val = unsafe { *dist_ptr.add(lmcpos) };
        (length_val == 0 || dist_val != 0) as u8
    } else {
        0u8
    };
    let limit_ok_for_cache = if cache_available != 0 && !lmc.is_null() {
        let current_limit = unsafe { *limit };
        let length_ptr = unsafe { (*lmc).length } as *mut crate::types::size_t;
        let cached_length = unsafe { *length_ptr.add(lmcpos) };
        (current_limit == 258 || cached_length <= current_limit || (!sublen.is_null() && {
            let max_cached = crate::src_cache::ZopfliMaxCachedSublen(lmc as *const _, lmcpos as crate::types::size_t, cached_length);
            max_cached as usize >= current_limit
        })) as u8
    } else {
        0u8
    };
    if !lmc.is_null() && limit_ok_for_cache != 0 && cache_available != 0 {
        let length_ptr = unsafe { (*lmc).length } as *mut crate::types::size_t;
        let cached_length = unsafe { *length_ptr.add(lmcpos) };
        if sublen.is_null() || cached_length <= crate::src_cache::ZopfliMaxCachedSublen(lmc as *const _, lmcpos as crate::types::size_t, cached_length) as crate::types::size_t {
            let mut len_val = cached_length as u16;
            let current_limit = unsafe { *limit } as u16;
            if len_val > current_limit {
                len_val = current_limit;
            }
            unsafe { *length = len_val; }
            if !sublen.is_null() {
                crate::src_cache::ZopfliCacheToSublen(lmc as *const _, lmcpos as crate::types::size_t, len_val as crate::types::size_t, sublen);
                let dist_val = unsafe { *sublen.add(len_val as usize) };
                unsafe { *distance = dist_val; }
                if unsafe { *limit } == 258 && len_val >= 3 {
                    let dist_ptr = unsafe { (*lmc).dist } as *mut u16;
                    let cached_dist = unsafe { *dist_ptr.add(lmcpos) };
                    let _ = dist_val == cached_dist;
                }
            } else {
                let dist_ptr = unsafe { (*lmc).dist } as *mut u16;
                let cached_dist = unsafe { *dist_ptr.add(lmcpos) };
                unsafe { *distance = cached_dist; }
            }
            return 1;
        }
        unsafe { *limit = cached_length; }
    }
    0
}

fn StoreInLongestMatchCache(s: *mut crate::types::ZopfliBlockState, pos: usize, limit: usize, sublen: *const u16, distance: u16, length: u16) {
    unsafe {
        let blockstart_ptr = crate::compat::c2r_field_ptr_ZopfliBlockState__blockstart(s as *mut ::core::ffi::c_void);
        let blockstart = *(blockstart_ptr as *const crate::types::size_t);
        let lmcpos = pos - blockstart as usize;

        let lmc_ptr = crate::compat::c2r_field_ptr_ZopfliBlockState__lmc(s as *mut ::core::ffi::c_void);
        let lmc = *(lmc_ptr as *mut *mut crate::types::ZopfliLongestMatchCache);
        let mut cache_available = 0u8;
        if !lmc.is_null() {
            let length_ptr = (*lmc).length;
            let dist_ptr = (*lmc).dist;
            if !length_ptr.is_null() && !dist_ptr.is_null() {
                let len_val = *length_ptr.add(lmcpos);
                let dist_val = *dist_ptr.add(lmcpos);
                cache_available = if len_val == 0 || dist_val != 0 { 1 } else { 0 };
            }
        }

        if !lmc.is_null() && limit == 258 && !sublen.is_null() && cache_available == 0 {
            let length_ptr = (*lmc).length;
            let dist_ptr = (*lmc).dist;
            if !length_ptr.is_null() && !dist_ptr.is_null() {
                let len_val = *length_ptr.add(lmcpos);
                let dist_val = *dist_ptr.add(lmcpos);
                if !(len_val == 1 && dist_val == 0) {
                    panic!("assertion failed: s->lmc->length[lmcpos] == 1 && s->lmc->dist[lmcpos] == 0");
                }
                let new_dist = if length < 3 { 0 } else { distance };
                let new_len = if length < 3 { 0 } else { length };
                *dist_ptr.add(lmcpos) = new_dist;
                *length_ptr.add(lmcpos) = new_len;
                let len_val2 = *length_ptr.add(lmcpos);
                let dist_val2 = *dist_ptr.add(lmcpos);
                if len_val2 == 1 && dist_val2 == 0 {
                    panic!("assertion failed: !(s->lmc->length[lmcpos] == 1 && s->lmc->dist[lmcpos] == 0)");
                }
                crate::src_cache::ZopfliSublenToCache(sublen as *const ::core::ffi::c_ushort, lmcpos as crate::types::size_t, length as crate::types::size_t, lmc);
            }
        }
    }
}

pub extern "C" fn ZopfliFindLongestMatch(s: *mut crate::types::ZopfliBlockState, h: *const crate::types::ZopfliHash, array: *const ::core::ffi::c_uchar, pos: crate::types::size_t, size: crate::types::size_t, limit: crate::types::size_t, sublen: *mut ::core::ffi::c_ushort, distance: *mut ::core::ffi::c_ushort, length: *mut ::core::ffi::c_ushort) {
    let pos_usize = pos as usize;
    let size_usize = size as usize;
    let limit_usize = limit as usize;
    let hpos = (pos_usize & (32768 - 1)) as u16;
    let mut bestdist: u16 = 0;
    let mut bestlength: u16 = 1;
    let mut chain_counter: i32 = 8192;
    let mut dist: u32 = 0;
    let h_void = h as *mut ::core::ffi::c_void;
    let mut hhead_ptr = unsafe { crate::compat::c2r_field_ptr_ZopfliHash__head(h_void) as *mut i32 };
    let mut hprev_ptr = unsafe { crate::compat::c2r_field_ptr_ZopfliHash__prev(h_void) as *mut u16 };
    let mut hhashval_ptr = unsafe { crate::compat::c2r_field_ptr_ZopfliHash__hashval(h_void) as *mut i32 };
    let mut hval = unsafe { *(crate::compat::c2r_field_ptr_ZopfliHash__val(h_void) as *mut i32) };
    let mut limit_local = limit_usize;
    if unsafe { crate::src_lz77::TryGetFromLongestMatchCache(s, pos_usize, &mut limit_local, sublen, distance, length) } != 0 {
        let _ = pos_usize + unsafe { *length as usize } <= size_usize;
        return;
    }
    let _ = limit_usize <= 258;
    let _ = limit_usize >= 3;
    let _ = pos_usize < size_usize;
    if size_usize - pos_usize < 3 {
        unsafe {
            *length = 0;
            *distance = 0;
        }
        return;
    }
    let mut limit_local_usize = limit_usize;
    if pos_usize + limit_usize > size_usize {
        limit_local_usize = size_usize - pos_usize;
    }
    let arrayend = unsafe { array.add(pos_usize).add(limit_local_usize) };
    let arrayend_safe = unsafe { arrayend.sub(8) };
    let _ = hval < 65536;
    let mut pp = unsafe { *hhead_ptr.add(hval as usize) as u16 };
    let mut p = unsafe { *hprev_ptr.add(pp as usize) };
    let _ = pp == hpos;
    dist = if p < pp { (pp - p) as u32 } else { ((32768 - p) + pp) as u32 };
    while dist < 32768 {
        let mut currentlength: u16 = 0;
        let _ = p < 32768;
        let _ = p == unsafe { *hprev_ptr.add(pp as usize) };
        let _ = unsafe { *hhashval_ptr.add(p as usize) } == hval;
        if dist > 0 {
            let _ = pos_usize < size_usize;
            let _ = dist as usize <= pos_usize;
            let scan = unsafe { array.add(pos_usize) };
            let match_ptr = unsafe { array.add(pos_usize - dist as usize) };
            if pos_usize + bestlength as usize >= size_usize || unsafe { *scan.add(bestlength as usize) } == unsafe { *match_ptr.add(bestlength as usize) } {
                let same0_ptr = unsafe { crate::compat::c2r_field_ptr_ZopfliHash__same(h_void) as *mut u16 };
                let same0 = unsafe { *same0_ptr.add(hpos as usize) };
                if same0 > 2 && unsafe { *scan } == unsafe { *match_ptr } {
                    let same1 = unsafe { *same0_ptr.add(((pos_usize - dist as usize) & (32768 - 1)) as usize) };
                    let mut same = if same0 < same1 { same0 } else { same1 };
                    if same > limit_local_usize as u16 { same = limit_local_usize as u16; }
                    let scan2 = unsafe { scan.add(same as usize) };
                    let match2 = unsafe { match_ptr.add(same as usize) };
                    let scan3 = crate::src_lz77::GetMatch(scan2, match2, arrayend, arrayend_safe);
                    currentlength = (scan3 as usize - scan as usize) as u16;
                } else {
                    let scan3 = crate::src_lz77::GetMatch(scan, match_ptr, arrayend, arrayend_safe);
                    currentlength = (scan3 as usize - scan as usize) as u16;
                }
            }
            if currentlength > bestlength {
                if !sublen.is_null() {
                    let mut j = bestlength + 1;
                    while j <= currentlength {
                        unsafe { *sublen.add(j as usize) = dist as u16; }
                        j += 1;
                    }
                }
                bestdist = dist as u16;
                bestlength = currentlength;
                if currentlength >= limit_local_usize as u16 { break; }
            }
        }
        let hhead2_ptr = unsafe { crate::compat::c2r_field_ptr_ZopfliHash__head2(h_void) as *mut i32 };
        if hhead_ptr != hhead2_ptr && bestlength >= unsafe { *(crate::compat::c2r_field_ptr_ZopfliHash__same(h_void) as *mut u16).add(hpos as usize) } && unsafe { *(crate::compat::c2r_field_ptr_ZopfliHash__val2(h_void) as *mut i32) } == unsafe { *(crate::compat::c2r_field_ptr_ZopfliHash__hashval2(h_void) as *mut i32).add(p as usize) } {
            hhead_ptr = hhead2_ptr;
            hprev_ptr = unsafe { crate::compat::c2r_field_ptr_ZopfliHash__prev2(h_void) as *mut u16 };
            hhashval_ptr = unsafe { crate::compat::c2r_field_ptr_ZopfliHash__hashval2(h_void) as *mut i32 };
            hval = unsafe { *(crate::compat::c2r_field_ptr_ZopfliHash__val2(h_void) as *mut i32) };
        }
        pp = p;
        p = unsafe { *hprev_ptr.add(p as usize) };
        if p == pp { break; }
        dist += if p < pp { (pp - p) as u32 } else { ((32768 - p) + pp) as u32 };
        chain_counter -= 1;
        if chain_counter <= 0 { break; }
    }
    crate::src_lz77::StoreInLongestMatchCache(s, pos_usize, limit_local_usize, sublen as *const u16, bestdist, bestlength);
    let _ = bestlength <= limit_local_usize as u16;
    unsafe {
        *distance = bestdist;
        *length = bestlength;
    }
    let _ = pos_usize + unsafe { *length as usize } <= size_usize;
}

pub extern "C" fn ZopfliLZ77Greedy(s: *mut crate::types::ZopfliBlockState, in_: *const ::core::ffi::c_uchar, instart: crate::types::size_t, inend: crate::types::size_t, store: *mut crate::types::ZopfliLZ77Store, h: *mut crate::types::ZopfliHash) {
    if instart == inend {
        return;
    }
    let windowstart = if instart > 32768 { instart - 32768 } else { 0 };
    let mut dummysublen: [::core::ffi::c_ushort; 259] = [0; 259];
    let mut i = 0;
    let mut j;
    let mut leng: ::core::ffi::c_ushort = 0;
    let mut dist: ::core::ffi::c_ushort = 0;
    let mut lengthscore: i32;
    let mut prev_length: u32 = 0;
    let mut prev_match: u32 = 0;
    let mut prevlengthscore: i32;
    let mut match_available: i32 = 0;

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
        crate::src_lz77::ZopfliFindLongestMatch(s, h, in_, i, inend, 258, dummysublen.as_mut_ptr(), &mut dist, &mut leng);
        lengthscore = crate::src_lz77::GetLengthScore(leng as i32, dist as i32);

        prevlengthscore = crate::src_lz77::GetLengthScore(prev_length as i32, prev_match as i32);
        if match_available != 0 {
            match_available = 0;
            if lengthscore > prevlengthscore + 1 {
                unsafe {
                    crate::src_lz77::ZopfliStoreLitLenDist(*in_.offset((i - 1) as isize) as ::core::ffi::c_ushort, 0, i - 1, store);
                }
                if lengthscore >= 3 && leng < 258 {
                    match_available = 1;
                    prev_length = leng as u32;
                    prev_match = dist as u32;
                    continue;
                }
            } else {
                leng = prev_length as ::core::ffi::c_ushort;
                dist = prev_match as ::core::ffi::c_ushort;
                lengthscore = prevlengthscore;
                crate::src_lz77::ZopfliVerifyLenDist(in_, inend, i - 1, dist, leng);
                crate::src_lz77::ZopfliStoreLitLenDist(leng, dist, i - 1, store);
                j = 2;
                while j < leng as usize {
                    let _ = i < inend;
                    i += 1;
                    crate::src_hash::ZopfliUpdateHash(in_, i, inend, h);
                    j += 1;
                }
                continue;
            }
        } else if lengthscore >= 3 && leng < 258 {
            match_available = 1;
            prev_length = leng as u32;
            prev_match = dist as u32;
            continue;
        }

        if lengthscore >= 3 {
            crate::src_lz77::ZopfliVerifyLenDist(in_, inend, i, dist, leng);
            crate::src_lz77::ZopfliStoreLitLenDist(leng, dist, i, store);
        } else {
            leng = 1;
            unsafe {
                crate::src_lz77::ZopfliStoreLitLenDist(*in_.offset(i as isize) as ::core::ffi::c_ushort, 0, i, store);
            }
        }
        j = 1;
        while j < leng as usize {
            let _ = i < inend;
            i += 1;
            crate::src_hash::ZopfliUpdateHash(in_, i, inend, h);
            j += 1;
        }
    }
}
