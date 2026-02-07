//! Module: src_zlib_container
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

fn adler32(data: *const u8, size: usize) -> u32 {
    const SUMS_OVERFLOW: usize = 5550;
    let mut s1: u32 = 1;
    let mut s2: u32 = 1 >> 16;
    let mut data_ptr = data;
    let mut remaining = size;

    while remaining > 0 {
        let amount = if remaining > SUMS_OVERFLOW { SUMS_OVERFLOW } else { remaining };
        remaining -= amount;
        let mut count = amount;
        while count > 0 {
            unsafe {
                s1 += *data_ptr as u32;
                data_ptr = data_ptr.add(1);
            }
            s2 += s1;
            count -= 1;
        }
        s1 %= 65521;
        s2 %= 65521;
    }

    (s2 << 16) | s1
}

pub extern "C" fn ZopfliZlibCompress(options: *const crate::types::ZopfliOptions, in_: *const ::core::ffi::c_uchar, insize: crate::types::size_t, out: *mut *mut ::core::ffi::c_uchar, outsize: *mut crate::types::size_t) {
    unsafe {
        let mut bitpointer: ::core::ffi::c_uchar = 0;
        let checksum = crate::src_zlib_container::adler32(in_ as *const u8, insize as usize);
        let cmf: u32 = 120;
        let flevel: u32 = 3;
        let fdict: u32 = 0;
        let mut cmfflg: u32 = 256 * cmf + fdict * 32 + flevel * 64;
        let fcheck: u32 = 31 - cmfflg % 31;
        cmfflg += fcheck;

        // ZOPFLI_APPEND_DATA for cmfflg / 256
        {
            if ((*outsize) & ((*outsize).wrapping_sub(1))) == 0 {
                *out = if *outsize == 0 {
                    libc::malloc(std::mem::size_of::<::core::ffi::c_uchar>()) as *mut ::core::ffi::c_uchar
                } else {
                    libc::realloc(*out as *mut libc::c_void, (*outsize * 2) as usize * std::mem::size_of::<::core::ffi::c_uchar>()) as *mut ::core::ffi::c_uchar
                };
            }
            *(*out).add(*outsize as usize) = (cmfflg / 256) as ::core::ffi::c_uchar;
            *outsize += 1;
        }

        // ZOPFLI_APPEND_DATA for cmfflg % 256
        {
            if ((*outsize) & ((*outsize).wrapping_sub(1))) == 0 {
                *out = if *outsize == 0 {
                    libc::malloc(std::mem::size_of::<::core::ffi::c_uchar>()) as *mut ::core::ffi::c_uchar
                } else {
                    libc::realloc(*out as *mut libc::c_void, (*outsize * 2) as usize * std::mem::size_of::<::core::ffi::c_uchar>()) as *mut ::core::ffi::c_uchar
                };
            }
            *(*out).add(*outsize as usize) = (cmfflg % 256) as ::core::ffi::c_uchar;
            *outsize += 1;
        }

        crate::src_deflate::ZopfliDeflate(options, 2, 1, in_, insize, &mut bitpointer, out, outsize);

        // Append checksum bytes (big-endian)
        for shift in [24u32, 16, 8, 0].iter() {
            if ((*outsize) & ((*outsize).wrapping_sub(1))) == 0 {
                *out = if *outsize == 0 {
                    libc::malloc(std::mem::size_of::<::core::ffi::c_uchar>()) as *mut ::core::ffi::c_uchar
                } else {
                    libc::realloc(*out as *mut libc::c_void, (*outsize * 2) as usize * std::mem::size_of::<::core::ffi::c_uchar>()) as *mut ::core::ffi::c_uchar
                };
            }
            *(*out).add(*outsize as usize) = ((checksum >> shift) % 256) as ::core::ffi::c_uchar;
            *outsize += 1;
        }

        let verbose_ptr = crate::compat::c2r_field_ptr_ZopfliOptions__verbose(options as *mut ::core::ffi::c_void);
        let verbose = *(verbose_ptr as *const ::core::ffi::c_int);
        if verbose != 0 {
            let fmt = b"Original Size: %d, Zlib: %d, Compression: %f%% Removed\n\0".as_ptr() as *const ::core::ffi::c_char;
            let compression = 100.0 * (insize as f64 - *outsize as f64) / (insize as f64);
            extern "C" {
                static stderr: *mut libc::FILE;
            }
            libc::fprintf(stderr, fmt, insize as ::core::ffi::c_int, *outsize as ::core::ffi::c_int, compression);
        }
    }
}
