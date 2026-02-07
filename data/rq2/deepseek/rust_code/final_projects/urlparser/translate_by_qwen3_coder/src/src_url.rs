//! Module: src_url
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

fn __bswap_16(__bsx: crate::types::__uint16_t) -> crate::types::__uint16_t {
    ((__bsx >> 8) & 0xff) | ((__bsx & 0xff) << 8)
}

fn __bswap_32(__bsx: crate::types::__uint32_t) -> crate::types::__uint32_t {
    (((__bsx & 0xff000000u32) >> 24) | ((__bsx & 0x00ff0000u32) >> 8) | ((__bsx & 0x0000ff00u32) << 8) | ((__bsx & 0x000000ffu32) << 24))
}

pub extern "C" fn __bswap_64(__bsx: crate::types::__uint64_t) -> crate::types::__uint64_t {
    (((__bsx & 0xff00000000000000u64) >> 56) |
     ((__bsx & 0x00ff000000000000u64) >> 40) |
     ((__bsx & 0x0000ff0000000000u64) >> 24) |
     ((__bsx & 0x000000ff00000000u64) >> 8) |
     ((__bsx & 0x00000000ff000000u64) << 8) |
     ((__bsx & 0x0000000000ff0000u64) << 24) |
     ((__bsx & 0x000000000000ff00u64) << 40) |
     ((__bsx & 0x00000000000000ffu64) << 56))
}

fn __uint16_identity(__x: crate::types::__uint16_t) -> crate::types::__uint16_t {
    __x
}

fn __uint32_identity(__x: crate::types::__uint32_t) -> crate::types::__uint32_t {
    __x
}

fn __uint64_identity(__x: crate::types::__uint64_t) -> crate::types::__uint64_t {
    __x
}

pub extern "C" fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    unimplemented!()
}

fn strff(ptr: *mut std::ffi::c_char, n: i32) -> *mut std::ffi::c_char {
    unimplemented!()
}

fn strrwd(ptr: *mut std::ffi::c_char, n: std::os::raw::c_int) -> *mut std::ffi::c_char {
    unimplemented!()
}

fn get_part(url: *mut std::ffi::c_char, format: *const std::ffi::c_char, l: std::os::raw::c_int) -> *mut std::ffi::c_char {
    unimplemented!()
}

pub extern "C" fn url_parse(url: *mut ::core::ffi::c_char) -> *mut crate::types::url_data_t {
    unimplemented!()
}

pub extern "C" fn url_is_protocol(str_: *mut ::core::ffi::c_char) -> bool {
    unsafe {
        let count = crate::globals::URL_SCHEMES.len();
        for i in 0..count {
            if libc::strcmp(crate::globals::URL_SCHEMES[i], str_) == 0 {
                return true;
            }
        }
        false
    }
}

pub extern "C" fn url_is_ssh(str_: *mut ::core::ffi::c_char) -> bool {
    let dup = unsafe { strdup(str_) };
    if dup.is_null() {
        return false;
    }
    let result = unsafe {
        libc::strcmp(dup, b"ssh\0".as_ptr() as *const _) == 0 ||
        libc::strcmp(dup, b"git\0".as_ptr() as *const _) == 0
    };
    unsafe { libc::free(dup as *mut _) };
    result
}

pub extern "C" fn url_get_protocol(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    unsafe {
        let protocol = libc::malloc(16) as *mut ::core::ffi::c_char;
        if protocol.is_null() {
            return std::ptr::null_mut();
        }
        if libc::sscanf(url, b"%[^://]\0".as_ptr() as *const _, protocol) == 1 {
            if url_is_protocol(protocol) {
                return protocol;
            }
        }
        libc::free(protocol as *mut libc::c_void);
        std::ptr::null_mut()
    }
}

pub extern "C" fn url_get_auth(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let protocol = unsafe { url_get_protocol(url) };
    if protocol.is_null() {
        return std::ptr::null_mut();
    }
    let l = unsafe { libc::strlen(protocol) as i32 } + 3;
    unsafe { get_part(url, b"%[^@]\0".as_ptr() as *const std::ffi::c_char, l) }
}

pub extern "C" fn url_get_hostname(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut l: std::os::raw::c_int = 3;
    let protocol = unsafe { url_get_protocol(url) };
    if protocol.is_null() {
        return std::ptr::null_mut();
    }
    let tmp_protocol = unsafe { strdup(protocol) };
    let auth = unsafe { url_get_auth(url) };
    if !auth.is_null() {
        l += unsafe { libc::strlen(auth) } as std::os::raw::c_int + 1;
        unsafe { libc::free(auth as *mut libc::c_void) };
    }
    l += unsafe { libc::strlen(protocol) } as std::os::raw::c_int;
    unsafe { libc::free(protocol as *mut libc::c_void) };
    let hostname = if unsafe { url_is_ssh(tmp_protocol) } {
        get_part(url, b"%[^:]\0" as *const u8 as *const std::ffi::c_char, l)
    } else {
        get_part(url, b"%[^/]\0" as *const u8 as *const std::ffi::c_char, l)
    };
    unsafe { libc::free(tmp_protocol as *mut libc::c_void) };
    hostname
}

pub extern "C" fn url_get_host(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    unsafe {
        let host = libc::malloc(1) as *mut ::core::ffi::c_char;
        let hostname = url_get_hostname(url);
        if host.is_null() || hostname.is_null() {
            if !host.is_null() {
                libc::free(host as *mut ::core::ffi::c_void);
            }
            if !hostname.is_null() {
                libc::free(hostname as *mut ::core::ffi::c_void);
            }
            return std::ptr::null_mut();
        }
        let _ = libc::sscanf(hostname, b"%[^:]\0".as_ptr() as *const ::core::ffi::c_char, host);
        libc::free(hostname as *mut ::core::ffi::c_void);
        host
    }
}

pub extern "C" fn url_get_pathname(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let path = url_get_path(url);
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let path_len = unsafe { libc::strlen(path) };
    let pathname = unsafe { libc::malloc(path_len + 1) as *mut ::core::ffi::c_char };
    if pathname.is_null() {
        unsafe { libc::free(path as *mut libc::c_void) };
        return std::ptr::null_mut();
    }
    unsafe {
        std::ptr::write_bytes(pathname, 0, (path_len + 1) as usize);
        let matched = libc::sscanf(path, b"%[^?]\0".as_ptr() as *const ::core::ffi::c_char, pathname);
        libc::free(path as *mut libc::c_void);
        if matched != 1 {
            libc::free(pathname as *mut libc::c_void);
            return std::ptr::null_mut();
        }
    }
    pathname
}

pub extern "C" fn url_get_path(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut l = 3;
    let protocol = unsafe { url_get_protocol(url) };
    let auth = unsafe { url_get_auth(url) };
    let hostname = unsafe { url_get_hostname(url) };
    if protocol.is_null() || hostname.is_null() {
        return std::ptr::null_mut();
    }
    let is_ssh = unsafe { url_is_ssh(protocol) };
    l += unsafe { libc::strlen(protocol) as i32 } + unsafe { libc::strlen(hostname) as i32 };
    if !auth.is_null() {
        l += unsafe { libc::strlen(auth) as i32 } + 1;
    }
    let format = if is_ssh {
        b":%s\0".as_ptr() as *const std::ffi::c_char
    } else {
        b"/%s\0".as_ptr() as *const std::ffi::c_char
    };
    let tmp_path = unsafe { get_part(url, format, l) };
    let fmt = if is_ssh {
        b"%s\0".as_ptr() as *const std::ffi::c_char
    } else {
        b"/%s\0".as_ptr() as *const std::ffi::c_char
    };
    let tmp_path_len = unsafe { libc::strlen(tmp_path) };
    let path = unsafe { libc::malloc(tmp_path_len + 1) as *mut std::ffi::c_char };
    if !path.is_null() {
        unsafe { libc::sprintf(path, fmt, tmp_path); }
    }
    if !auth.is_null() {
        unsafe { libc::free(auth as *mut std::ffi::c_void); }
    }
    unsafe {
        libc::free(protocol as *mut std::ffi::c_void);
        libc::free(hostname as *mut std::ffi::c_void);
        libc::free(tmp_path as *mut std::ffi::c_void);
    }
    path
}

pub extern "C" fn url_get_search(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let path = url_get_path(url);
    let pathname = url_get_pathname(url);
    let search = unsafe { libc::malloc(1) as *mut ::core::ffi::c_char };
    if path.is_null() || search.is_null() {
        return std::ptr::null_mut();
    }
    let tmp_path = strff(path, unsafe { libc::strlen(path) as i32 });
    unsafe {
        *search = 0;
        libc::sscanf(tmp_path, b"%[^#]\0".as_ptr() as *const _, search);
    }
    let _ = strrwd(tmp_path, unsafe { libc::strlen(pathname) as std::os::raw::c_int });
    unsafe {
        libc::free(path as *mut _);
        libc::free(pathname as *mut _);
    }
    search
}

pub extern "C" fn url_get_query(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let search = unsafe { crate::compat::url_get_search(url) };
    if search.is_null() {
        return std::ptr::null_mut();
    }
    let query = unsafe { libc::malloc(1) as *mut ::core::ffi::c_char };
    if query.is_null() {
        unsafe { libc::free(search as *mut ::core::ffi::c_void) };
        return std::ptr::null_mut();
    }
    let _ = unsafe { libc::sscanf(search, b"?%s\0".as_ptr() as *const ::core::ffi::c_char, query) };
    unsafe { libc::free(search as *mut ::core::ffi::c_void) };
    query
}

pub extern "C" fn url_get_hash(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    unsafe {
        let hash = libc::malloc(1) as *mut ::core::ffi::c_char;
        if hash.is_null() {
            return std::ptr::null_mut();
        }
        let path = url_get_path(url);
        if path.is_null() {
            libc::free(hash as *mut ::core::ffi::c_void);
            return std::ptr::null_mut();
        }
        let pathname = url_get_pathname(url);
        if pathname.is_null() {
            libc::free(hash as *mut ::core::ffi::c_void);
            libc::free(path as *mut ::core::ffi::c_void);
            return std::ptr::null_mut();
        }
        let search = url_get_search(url);
        let pathname_len = libc::strlen(pathname) as i32;
        let search_len = libc::strlen(search) as i32;
        let tmp_path = strff(path, pathname_len + search_len);
        *hash = 0;
        libc::sscanf(tmp_path, b"%s\0".as_ptr() as *const ::core::ffi::c_char, hash);
        let _ = strrwd(tmp_path, pathname_len + search_len);
        libc::free(tmp_path as *mut ::core::ffi::c_void);
        libc::free(pathname as *mut ::core::ffi::c_void);
        libc::free(path as *mut ::core::ffi::c_void);
        if !search.is_null() {
            libc::free(search as *mut ::core::ffi::c_void);
        }
        hash
    }
}

pub extern "C" fn url_get_port(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    unsafe {
        let port = libc::malloc(1) as *mut ::core::ffi::c_char;
        let hostname = url_get_hostname(url);
        let host = url_get_host(url);
        if port.is_null() || hostname.is_null() {
            return std::ptr::null_mut();
        }
        let host_len = libc::strlen(host) as i32 + 1;
        let tmp_hostname = strff(hostname, host_len);
        libc::sscanf(tmp_hostname, b"%s\0".as_ptr() as *const _, port);
        libc::free(hostname as *mut _);
        libc::free(tmp_hostname as *mut _);
        port
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_url_18
// c_function: url_inspect
// rust_file: src_url.rs
// rust_signature: pub extern "C" fn url_inspect(url: *mut ::core::ffi::c_char)
// c_first_line: void
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/urlparser/workspace/repair_history/urlparser/translate_by_qwen3_coder/_manual_fix/src_url_18/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find function `url_data_inspect` in module `crate::compat`
//      --> src/src_url.rs:302:20
//       |
//       |                    ^^^^^^^^^^^^^^^^ not found in `crate::compat`
//       |
//   help: consider importing this function
//       |
//       |
// =================================
pub extern "C" fn url_inspect(url: *mut ::core::ffi::c_char) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_url::url_inspect(url as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_url_18
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/urlparser/workspace/repair_history/urlparser/translate_by_qwen3_coder/_manual_fix/src_url_18/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn url_inspect(url: *mut ::core::ffi::c_char) {
    let data = crate::compat::url_parse(url);
    crate::compat::url_data_inspect(data);
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_url_18
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn url_data_inspect(data: *mut crate::types::url_data_t) {
    if data.is_null() {
        return;
    }
    unsafe {
        crate::compat::printf(b"#url =>\n\0" as *const u8 as *const ::core::ffi::c_char);
        crate::compat::printf(b"    .href: \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char, std::ptr::null::<::core::ffi::c_char>());
        crate::compat::printf(b"    .protocol: \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char, std::ptr::null::<::core::ffi::c_char>());
        crate::compat::printf(b"    .host: \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char, std::ptr::null::<::core::ffi::c_char>());
        crate::compat::printf(b"    .auth: \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char, std::ptr::null::<::core::ffi::c_char>());
        crate::compat::printf(b"    .hostname: \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char, std::ptr::null::<::core::ffi::c_char>());
        crate::compat::printf(b"    .pathname: \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char, std::ptr::null::<::core::ffi::c_char>());
        crate::compat::printf(b"    .search: \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char, std::ptr::null::<::core::ffi::c_char>());
        crate::compat::printf(b"    .path: \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char, std::ptr::null::<::core::ffi::c_char>());
        crate::compat::printf(b"    .hash: \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char, std::ptr::null::<::core::ffi::c_char>());
        crate::compat::printf(b"    .query: \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char, std::ptr::null::<::core::ffi::c_char>());
        crate::compat::printf(b"    .port: \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char, std::ptr::null::<::core::ffi::c_char>());
    }
}

pub extern "C" fn url_free(data: *mut crate::types::url_data_t) {
    if data.is_null() {
        return;
    }
    unsafe {
        libc::free(data as *mut ::core::ffi::c_void);
    }
}
