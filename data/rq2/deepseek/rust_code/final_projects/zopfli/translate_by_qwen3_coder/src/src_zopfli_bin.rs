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

fn LoadFile(filename: *const std::ffi::c_char, out: *mut *mut std::os::raw::c_uchar, outsize: *mut usize) -> std::os::raw::c_int {
    unsafe {
        *out = std::ptr::null_mut();
        *outsize = 0;
        let file = crate::compat::fopen(filename, b"rb\0".as_ptr() as *const _);
        if file.is_null() {
            return 0;
        }
        if crate::compat::fseek(file, 0, 2) != 0 {
            crate::compat::fclose(file);
            return 0;
        }
        let size = crate::compat::ftell(file) as usize;
        if size > 2147483647 {
            crate::compat::fprintf(crate::compat::stderr, b"Files larger than 2GB are not supported.\n\0".as_ptr() as *const _);
            crate::compat::exit(1);
        }
        crate::compat::rewind(file);
        *outsize = size;
        *out = libc::malloc(size) as *mut std::os::raw::c_uchar;
        if size != 0 && !(*out).is_null() {
            let testsize = crate::compat::fread(*out as *mut libc::c_void, 1, size as u64, file);
            if testsize != size as u64 {
                libc::free(*out as *mut libc::c_void);
                *out = std::ptr::null_mut();
                *outsize = 0;
                crate::compat::fclose(file);
                return 0;
            }
        }
        let _ = (*outsize == 0 || !(*out).is_null());
        crate::compat::fclose(file);
        1
    }
}

fn SaveFile(filename: *const std::ffi::c_char, r#in: *const u8, insize: usize) -> () {
    use crate::compat::*;
    use crate::globals::*;
    unsafe {
        let file = fopen(filename, b"wb\0".as_ptr() as *const _);
        if file.is_null() {
            fprintf(stderr, b"Error: Cannot write to output file, terminating.\n\0".as_ptr() as *const _);
            exit(1);
        }
        let _ = file;
        fwrite(r#in as *const _, 1, insize as u64, file);
        fclose(file);
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_zopfli_bin_3
// c_function: CompressFile
// rust_file: src_zopfli_bin.rs
// rust_signature: fn CompressFile(options: *const crate::types::ZopfliOptions, output_type: crate::types::ZopfliFormat, infilename: *const std::ffi::c_char, outfilename: *const std::ffi::c_char)
// c_first_line: static void CompressFile(const ZopfliOptions* options,
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_zopfli_bin_3/translated_rust.rs
// last_error_truncated:
//   error[E0308]: mismatched types
//      --> src/src_zopfli_bin.rs:75:27
//       |
//       |             ------------- ^^^^^^ expected `libc::FILE`, found `types::FILE`
//       |             |
//       |             arguments to this function are incorrect
//       |
//      --> src/types.rs:69:1
// =================================
fn CompressFile(options: *const crate::types::ZopfliOptions, output_type: crate::types::ZopfliFormat, infilename: *const std::ffi::c_char, outfilename: *const std::ffi::c_char) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_zopfli_bin::CompressFile(options as _, output_type as _, infilename as _, outfilename as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_zopfli_bin_3
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_zopfli_bin_3/translated_rust.rs
 * ------------------------------------------------------------
fn CompressFile(options: *const crate::types::ZopfliOptions, output_type: crate::types::ZopfliFormat, infilename: *const std::ffi::c_char, outfilename: *const std::ffi::c_char) {
    let mut in_ptr: *mut std::os::raw::c_uchar = std::ptr::null_mut();
    let mut insize: usize = 0;
    let mut out_ptr: *mut std::os::raw::c_uchar = std::ptr::null_mut();
    let mut outsize: crate::types::size_t = 0;

    if crate::src_zopfli_bin::LoadFile(infilename, &mut in_ptr, &mut insize) == 0 {
        unsafe {
            let stderr_var = libc::fdopen(libc::STDERR_FILENO, b"w\0" as *const u8 as *const i8);
            libc::fprintf(stderr, b"Invalid filename: %s\n\0" as *const u8 as *const i8, infilename);
        }
        return;
    }

    crate::src_zopfli_lib::ZopfliCompress(options, output_type, in_ptr as *const _, insize as crate::types::size_t, &mut out_ptr, &mut outsize);

    if !outfilename.is_null() {
        crate::src_zopfli_bin::SaveFile(outfilename, out_ptr as *const u8, outsize as usize);
    } else {
        unsafe {
            let stdout_var = libc::fdopen(libc::STDOUT_FILENO, b"w\0" as *const u8 as *const i8);
            libc::fwrite(out_ptr as *const std::ffi::c_void, 1, outsize as usize, stdout);
        }
    }

    unsafe {
        libc::free(out_ptr as *mut std::ffi::c_void);
        libc::free(in_ptr as *mut std::ffi::c_void);
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_zopfli_bin_3
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn AddStrings(str1: *const std::ffi::c_char, str2: *const std::ffi::c_char) -> *mut std::ffi::c_char {
    unsafe {
        let len = libc::strlen(str1) + libc::strlen(str2);
        let result = libc::malloc(len + 1) as *mut std::ffi::c_char;
        if result.is_null() {
            libc::exit(-1);
        }
        libc::strcpy(result, str1);
        libc::strcat(result, str2);
        result
    }
}

fn StringsEqual(str1: *const std::ffi::c_char, str2: *const std::ffi::c_char) -> std::ffi::c_char {
    unsafe { (libc::strcmp(str1, str2) == 0) as std::ffi::c_char }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_zopfli_bin_6
// c_function: main
// rust_file: src_zopfli_bin.rs
// rust_signature: pub extern "C" fn main(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int
// c_first_line: int main(int argc, char* argv[]) {
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_zopfli_bin_6/translated_rust.rs
// last_error_truncated:
//   error[E0530]: let bindings cannot shadow statics
//      --> src/src_zopfli_bin.rs:173:21
//       |
//       |     ---------------- the static `stderr` is imported here
//       |                     ^^^^^^ cannot be named the same as a static
//   error[E0530]: let bindings cannot shadow statics
//      --> src/src_zopfli_bin.rs:217:17
//       |
// =================================
pub extern "C" fn main(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_zopfli_bin_6
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_zopfli_bin_6/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn main(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut options = crate::types::ZopfliOptions { _opaque: [0u8; 0] };
    let mut output_type = 0;
    let mut filename: *const std::ffi::c_char = std::ptr::null();
    let mut output_to_stdout: std::os::raw::c_int = 0;

    unsafe {
        crate::src_util::ZopfliInitOptions(&mut options);
    }

    let mut i: std::os::raw::c_int = 1;
    while i < argc {
        let arg = unsafe { *argv.offset(i as isize) };
        if unsafe { crate::src_zopfli_bin::StringsEqual(arg, b"-v\0".as_ptr() as *const _) } != 0 {
        } else if unsafe { crate::src_zopfli_bin::StringsEqual(arg, b"-c\0".as_ptr() as *const _) } != 0 {
            output_to_stdout = 1;
        } else if unsafe { crate::src_zopfli_bin::StringsEqual(arg, b"--deflate\0".as_ptr() as *const _) } != 0 {
            output_type = 1;
        } else if unsafe { crate::src_zopfli_bin::StringsEqual(arg, b"--zlib\0".as_ptr() as *const _) } != 0 {
            output_type = 2;
        } else if unsafe { crate::src_zopfli_bin::StringsEqual(arg, b"--gzip\0".as_ptr() as *const _) } != 0 {
            output_type = 0;
        } else if unsafe { crate::src_zopfli_bin::StringsEqual(arg, b"--splitlast\0".as_ptr() as *const _) } != 0 {
        } else if unsafe { (*arg) == b'-' as i8 && *arg.offset(1) == b'-' as i8 && *arg.offset(2) == b'i' as i8
            && *arg.offset(3) >= b'0' as i8 && *arg.offset(3) <= b'9' as i8 } {
            let _ = unsafe { libc::atoi(arg.offset(3)) };
        } else if unsafe { crate::src_zopfli_bin::StringsEqual(arg, b"-h\0".as_ptr() as *const _) } != 0 {
            unsafe {
                let stderr = libc::fdopen(2, b"w\0".as_ptr() as *const _);
                libc::fprintf(stderr, b"Usage: zopfli [OPTION]... FILE...\n\0".as_ptr() as *const _);
                libc::fprintf(stderr, b"  -h    gives this help\n\0".as_ptr() as *const _);
                libc::fprintf(stderr, b"  -c    write the result on standard output, instead of disk filename + '.gz'\n\0".as_ptr() as *const _);
                libc::fprintf(stderr, b"  -v    verbose mode\n\0".as_ptr() as *const _);
                libc::fprintf(stderr, b"  --i#  perform # iterations (default 15). More gives more compression but is slower. Examples: --i10, --i50, --i1000\n\0".as_ptr() as *const _);
                libc::fprintf(stderr, b"  --gzip        output to gzip format (default)\n\0".as_ptr() as *const _);
                libc::fprintf(stderr, b"  --zlib        output to zlib format instead of gzip\n\0".as_ptr() as *const _);
                libc::fprintf(stderr, b"  --deflate     output to deflate format instead of gzip\n\0".as_ptr() as *const _);
                libc::fprintf(stderr, b"  --splitlast   ignored, left for backwards compatibility\n\0".as_ptr() as *const _);
                libc::fclose(stderr);
            }
            return 0;
        }
        i += 1;
    }

    i = 1;
    while i < argc {
        let arg_i = unsafe { *argv.offset(i as isize) };
        if unsafe { *arg_i } != b'-' as i8 {
            let mut outfilename: *mut std::ffi::c_char = std::ptr::null_mut();
            filename = arg_i;
            if output_to_stdout != 0 {
                outfilename = std::ptr::null_mut();
            } else if output_type == 0 {
                outfilename = crate::src_zopfli_bin::AddStrings(filename, b".gz\0".as_ptr() as *const _);
            } else if output_type == 2 {
                outfilename = crate::src_zopfli_bin::AddStrings(filename, b".zlib\0".as_ptr() as *const _);
            } else if output_type == 1 {
                outfilename = crate::src_zopfli_bin::AddStrings(filename, b".deflate\0".as_ptr() as *const _);
            } else {
                outfilename = std::ptr::null_mut();
            }
            crate::src_zopfli_bin::CompressFile(&options, output_type, filename, outfilename);
            if !outfilename.is_null() {
                unsafe { libc::free(outfilename as *mut std::ffi::c_void); }
            }
        }
        i += 1;
    }

    if filename.is_null() {
        unsafe {
            let stderr = libc::fdopen(2, b"w\0".as_ptr() as *const _);
            libc::fprintf(stderr, b"Please provide filename\nFor help, type: %s -h\n\0".as_ptr() as *const _, *argv);
            libc::fclose(stderr);
        }
    }

    0
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_zopfli_bin_6
 * === C2R_LLM_FAILED_OUTPUT_END === */

