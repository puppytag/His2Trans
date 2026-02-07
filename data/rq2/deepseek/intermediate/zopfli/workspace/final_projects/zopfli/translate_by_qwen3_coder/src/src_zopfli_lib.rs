//! Module: src_zopfli_lib
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

pub extern "C" fn ZopfliCompress(options: *const crate::types::ZopfliOptions, output_type: ZopfliFormat, in_: *const ::core::ffi::c_uchar, insize: crate::types::size_t, out: *mut *mut ::core::ffi::c_uchar, outsize: *mut crate::types::size_t) {
    if output_type == 0 {
        crate::src_gzip_container::ZopfliGzipCompress(options, in_, insize, out, outsize);
    } else if output_type == 1 {
        crate::src_zlib_container::ZopfliZlibCompress(options, in_, insize, out, outsize);
    } else if output_type == 2 {
        let mut bp: ::core::ffi::c_uchar = 0;
        crate::src_deflate::ZopfliDeflate(options, 2, 1, in_, insize, &mut bp, out, outsize);
    } else {
        let _ = 0;
    }
}
