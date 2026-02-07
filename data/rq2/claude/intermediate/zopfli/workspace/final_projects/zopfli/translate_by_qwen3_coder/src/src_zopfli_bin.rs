//! Module: src_zopfli_bin
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

fn LoadFile(filename: *const std::ffi::c_char, out: *mut *mut u8, outsize: *mut usize) -> std::ffi::c_int {
    unsafe {
        *out = std::ptr::null_mut();
        *outsize = 0;
        
        let file = libc::fopen(filename, b"rb\0".as_ptr() as *const std::ffi::c_char);
        if file.is_null() {
            return 0;
        }
        
        libc::fseek(file, 0, libc::SEEK_END);
        *outsize = libc::ftell(file) as usize;
        
        if *outsize > 2147483647 {
            let stderr_ptr = libc::fdopen(2, b"w\0".as_ptr() as *const std::ffi::c_char);
            if !stderr_ptr.is_null() {
                libc::fputs(
                    b"Files larger than 2GB are not supported.\n\0".as_ptr() as *const std::ffi::c_char,
                    stderr_ptr,
                );
            }
            libc::exit(1);
        }
        
        libc::rewind(file);
        
        *out = libc::malloc(*outsize) as *mut u8;
        
        if *outsize != 0 && !(*out).is_null() {
            let testsize = libc::fread(*out as *mut std::ffi::c_void, 1, *outsize, file);
            if testsize != *outsize {
                libc::free(*out as *mut std::ffi::c_void);
                *out = std::ptr::null_mut();
                *outsize = 0;
                libc::fclose(file);
                return 0;
            }
        }
        
        libc::fclose(file);
        1
    }
}

fn SaveFile(filename: *const std::ffi::c_char, r#in: *const u8, insize: usize) {
    unsafe {
        let file = libc::fopen(filename, b"wb\0".as_ptr() as *const std::ffi::c_char);
        if file.is_null() {
            let stderr_ptr = libc::fdopen(2, b"w\0".as_ptr() as *const std::ffi::c_char);
            libc::fputs(
                b"Error: Cannot write to output file, terminating.\n\0".as_ptr() as *const std::ffi::c_char,
                stderr_ptr,
            );
            libc::exit(1);
        }
        libc::fwrite(r#in as *const std::ffi::c_void, 1, insize, file);
        libc::fclose(file);
    }
}

fn CompressFile(options: *const crate::types::ZopfliOptions, output_type: crate::types::ZopfliFormat, infilename: *const std::ffi::c_char, outfilename: *const std::ffi::c_char) {
    unsafe {
        let mut in_ptr: *mut u8 = std::ptr::null_mut();
        let mut insize: usize = 0;
        let mut out: *mut u8 = std::ptr::null_mut();
        let mut outsize: usize = 0;

        if LoadFile(infilename, &mut in_ptr, &mut insize) == 0 {
            extern "C" {
                static stderr: *mut libc::FILE;
            }
            libc::fprintf(
                stderr,
                b"Invalid filename: %s\n\0".as_ptr() as *const std::ffi::c_char,
                infilename,
            );
            return;
        }

        crate::src_zopfli_lib::ZopfliCompress(
            options,
            output_type,
            in_ptr,
            insize as crate::types::size_t,
            &mut out,
            &mut outsize as *mut usize as *mut crate::types::size_t,
        );

        if !outfilename.is_null() {
            SaveFile(outfilename, out, outsize);
        } else {
            extern "C" {
                static stdout: *mut libc::FILE;
            }
            libc::fwrite(
                out as *const std::ffi::c_void,
                1,
                outsize,
                stdout,
            );
        }

        libc::free(out as *mut std::ffi::c_void);
        libc::free(in_ptr as *mut std::ffi::c_void);
    }
}

fn AddStrings(str1: *const std::ffi::c_char, str2: *const std::ffi::c_char) -> *mut std::ffi::c_char {
    unsafe {
        let len1 = libc::strlen(str1);
        let len2 = libc::strlen(str2);
        let len = len1 + len2;
        let result = libc::malloc((len + 1) as usize) as *mut std::ffi::c_char;
        if result.is_null() {
            libc::exit(-1);
        }
        libc::strcpy(result, str1);
        libc::strcat(result, str2);
        result
    }
}

fn StringsEqual(str1: *const std::ffi::c_char, str2: *const std::ffi::c_char) -> std::ffi::c_char {
    unsafe {
        if libc::strcmp(str1, str2) == 0 {
            1
        } else {
            0
        }
    }
}

pub extern "C" fn main(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    const ZOPFLI_FORMAT_GZIP: crate::types::ZopfliFormat = 0;
    const ZOPFLI_FORMAT_ZLIB: crate::types::ZopfliFormat = 1;
    const ZOPFLI_FORMAT_DEFLATE: crate::types::ZopfliFormat = 2;
    
    let mut options_storage: [u8; 64] = [0u8; 64];
    let options_ptr = options_storage.as_mut_ptr() as *mut crate::types::ZopfliOptions;
    let mut output_type: crate::types::ZopfliFormat = ZOPFLI_FORMAT_GZIP;
    let mut filename: *const ::core::ffi::c_char = std::ptr::null();
    let mut output_to_stdout: ::core::ffi::c_int = 0;
    
    crate::src_util::ZopfliInitOptions(options_ptr);
    
    let mut i: ::core::ffi::c_int = 1;
    while i < argc {
        let arg: *const ::core::ffi::c_char = unsafe { *argv.offset(i as isize) };
        
        if crate::src_zopfli_bin::StringsEqual(arg, b"-v\0".as_ptr() as *const _) != 0 {
            unsafe { *options_storage.as_mut_ptr().offset(0) = 1; }
        } else if crate::src_zopfli_bin::StringsEqual(arg, b"-c\0".as_ptr() as *const _) != 0 {
            output_to_stdout = 1;
        } else if crate::src_zopfli_bin::StringsEqual(arg, b"--deflate\0".as_ptr() as *const _) != 0 {
            output_type = ZOPFLI_FORMAT_DEFLATE;
        } else if crate::src_zopfli_bin::StringsEqual(arg, b"--zlib\0".as_ptr() as *const _) != 0 {
            output_type = ZOPFLI_FORMAT_ZLIB;
        } else if crate::src_zopfli_bin::StringsEqual(arg, b"--gzip\0".as_ptr() as *const _) != 0 {
            output_type = ZOPFLI_FORMAT_GZIP;
        } else if crate::src_zopfli_bin::StringsEqual(arg, b"--splitlast\0".as_ptr() as *const _) != 0 {
            // ignored
        } else if unsafe { *arg.offset(0) } == b'-' as i8 && unsafe { *arg.offset(1) } == b'-' as i8 
            && unsafe { *arg.offset(2) } == b'i' as i8
            && unsafe { *arg.offset(3) } >= b'0' as i8 && unsafe { *arg.offset(3) } <= b'9' as i8 {
            let num = unsafe { libc::atoi(arg.offset(3)) };
            unsafe { *(options_storage.as_mut_ptr().offset(4) as *mut i32) = num; }
        } else if crate::src_zopfli_bin::StringsEqual(arg, b"-h\0".as_ptr() as *const _) != 0 {
            unsafe {
                let stderr_ptr = crate::compat::stderr;
                libc::fprintf(stderr_ptr as *mut libc::FILE,
                    b"Usage: zopfli [OPTION]... FILE...\n  -h    gives this help\n  -c    write the result on standard output, instead of disk filename + '.gz'\n  -v    verbose mode\n  --i#  perform # iterations (default 15). More gives more compression but is slower. Examples: --i10, --i50, --i1000\n\0".as_ptr() as *const _);
                libc::fprintf(stderr_ptr as *mut libc::FILE,
                    b"  --gzip        output to gzip format (default)\n  --zlib        output to zlib format instead of gzip\n  --deflate     output to deflate format instead of gzip\n  --splitlast   ignored, left for backwards compatibility\n\0".as_ptr() as *const _);
            }
            return 0;
        }
        i += 1;
    }
    
    let numiterations = unsafe { *(options_storage.as_ptr().offset(4) as *const i32) };
    if numiterations < 1 {
        unsafe {
            let stderr_ptr = crate::compat::stderr;
            libc::fprintf(stderr_ptr as *mut libc::FILE, b"Error: must have 1 or more iterations\n\0".as_ptr() as *const _);
        }
        return 0;
    }
    
    i = 1;
    while i < argc {
        let arg_i = unsafe { *argv.offset(i as isize) };
        if unsafe { *arg_i } != b'-' as i8 {
            let mut outfilename: *mut ::core::ffi::c_char;
            filename = arg_i;
            if output_to_stdout != 0 {
                outfilename = std::ptr::null_mut();
            } else if output_type == ZOPFLI_FORMAT_GZIP {
                outfilename = crate::src_zopfli_bin::AddStrings(filename, b".gz\0".as_ptr() as *const _);
            } else if output_type == ZOPFLI_FORMAT_ZLIB {
                outfilename = crate::src_zopfli_bin::AddStrings(filename, b".zlib\0".as_ptr() as *const _);
            } else {
                outfilename = crate::src_zopfli_bin::AddStrings(filename, b".deflate\0".as_ptr() as *const _);
            }
            let verbose = unsafe { *options_storage.as_ptr().offset(0) };
            if verbose != 0 && !outfilename.is_null() {
                unsafe {
                    let stderr_ptr = crate::compat::stderr;
                    libc::fprintf(stderr_ptr as *mut libc::FILE, b"Saving to: %s\n\0".as_ptr() as *const _, outfilename);
                }
            }
            crate::src_zopfli_bin::CompressFile(options_ptr as *const _, output_type, filename, outfilename as *const _);
            unsafe { libc::free(outfilename as *mut _); }
        }
        i += 1;
    }
    
    if filename.is_null() {
        unsafe {
            let stderr_ptr = crate::compat::stderr;
            libc::fprintf(stderr_ptr as *mut libc::FILE,
                b"Please provide filename\nFor help, type: %s -h\n\0".as_ptr() as *const _,
                *argv.offset(0));
        }
    }
    
    0
}
