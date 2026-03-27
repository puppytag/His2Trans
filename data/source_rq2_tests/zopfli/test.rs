// Rust translation of zopfli test.c
// Copyright (c) 2024
// Translated from C to Rust

use std::os::raw::{c_int, c_uchar};
use std::ptr;

// ZopfliOptions structure
#[repr(C)]
pub struct ZopfliOptions {
    pub verbose: c_int,
    pub verbose_more: c_int,
    pub numiterations: c_int,
    pub blocksplitting: c_int,
    pub blocksplittinglast: c_int,
    pub blocksplittingmax: c_int,
}

// ZopfliFormat enum
#[repr(C)]
pub enum ZopfliFormat {
    ZOPFLI_FORMAT_GZIP = 0,
    ZOPFLI_FORMAT_ZLIB = 1,
    ZOPFLI_FORMAT_DEFLATE = 2,
}

// External declarations for zopfli library functions
extern "C" {
    fn ZopfliInitOptions(options: *mut ZopfliOptions);
    fn ZopfliCompress(
        options: *const ZopfliOptions,
        output_type: ZopfliFormat,
        input: *const c_uchar,
        insize: usize,
        out: *mut *mut c_uchar,
        outsize: *mut usize,
    );
}

fn to_hex(data: &[u8]) -> String {
    data.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}

fn main() {
    let input = b"hello zopfli test";
    let insize = input.len();

    let mut opts = ZopfliOptions {
        verbose: 0,
        verbose_more: 0,
        numiterations: 0,
        blocksplitting: 0,
        blocksplittinglast: 0,
        blocksplittingmax: 0,
    };

    unsafe {
        ZopfliInitOptions(&mut opts);
    }
    opts.numiterations = 5;

    let mut out: *mut c_uchar = ptr::null_mut();
    let mut outsize: usize = 0;

    unsafe {
        ZopfliCompress(
            &opts,
            ZopfliFormat::ZOPFLI_FORMAT_DEFLATE,
            input.as_ptr(),
            insize,
            &mut out,
            &mut outsize,
        );

        assert!(!out.is_null(), "Output is null");
        assert!(outsize > 0, "Output size should be > 0");

        let out_slice = std::slice::from_raw_parts(out, outsize);
        let hex = to_hex(out_slice);

        // Stable reference for this repo snapshot + options + input
        let expected = "cb48cdc9c957a8ca2f48cbc95428492d2e0100";
        assert_eq!(outsize, 19, "Expected output size 19, got {}", outsize);
        assert_eq!(hex, expected, "Hex mismatch: {} vs {}", hex, expected);

        println!("{}:{}", outsize, hex);

        // Free the allocated memory
        libc::free(out as *mut libc::c_void);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zopfli_compress() {
        let input = b"hello zopfli test";
        let insize = input.len();

        let mut opts = ZopfliOptions {
            verbose: 0,
            verbose_more: 0,
            numiterations: 0,
            blocksplitting: 0,
            blocksplittinglast: 0,
            blocksplittingmax: 0,
        };

        unsafe {
            ZopfliInitOptions(&mut opts);
        }
        opts.numiterations = 5;

        let mut out: *mut c_uchar = ptr::null_mut();
        let mut outsize: usize = 0;

        unsafe {
            ZopfliCompress(
                &opts,
                ZopfliFormat::ZOPFLI_FORMAT_DEFLATE,
                input.as_ptr(),
                insize,
                &mut out,
                &mut outsize,
            );

            assert!(!out.is_null());
            assert!(outsize > 0);

            let out_slice = std::slice::from_raw_parts(out, outsize);
            let hex = to_hex(out_slice);

            let expected = "cb48cdc9c957a8ca2f48cbc95428492d2e0100";
            assert_eq!(outsize, 19);
            assert_eq!(hex, expected);

            libc::free(out as *mut libc::c_void);
        }
    }
}
