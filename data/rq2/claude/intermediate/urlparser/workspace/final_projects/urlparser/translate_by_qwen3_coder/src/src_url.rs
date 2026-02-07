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
    (((__bsx >> 8) & 0xff) | ((__bsx & 0xff) << 8)) as crate::types::__uint16_t
}

fn __bswap_32(__bsx: crate::types::__uint32_t) -> crate::types::__uint32_t {
    (((__bsx) & 0xff000000u32) >> 24)
        | (((__bsx) & 0x00ff0000u32) >> 8)
        | (((__bsx) & 0x0000ff00u32) << 8)
        | (((__bsx) & 0x000000ffu32) << 24)
}

pub extern "C" fn __bswap_64(__bsx: crate::types::__uint64_t) -> crate::types::__uint64_t {
    (((__bsx) & 0xff00000000000000u64) >> 56)
        | (((__bsx) & 0x00ff000000000000u64) >> 40)
        | (((__bsx) & 0x0000ff0000000000u64) >> 24)
        | (((__bsx) & 0x000000ff00000000u64) >> 8)
        | (((__bsx) & 0x00000000ff000000u64) << 8)
        | (((__bsx) & 0x0000000000ff0000u64) << 24)
        | (((__bsx) & 0x000000000000ff00u64) << 40)
        | (((__bsx) & 0x00000000000000ffu64) << 56)
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
    unsafe {
        let n: usize = libc::strlen(__s) as usize + 1;
        let dup: *mut ::core::ffi::c_char = libc::malloc(n) as *mut ::core::ffi::c_char;
        if !dup.is_null() {
            libc::strcpy(dup, __s);
        }
        dup
    }
}

fn strff(ptr: *mut std::ffi::c_char, n: std::ffi::c_int) -> *mut std::ffi::c_char {
    let mut ptr = ptr;
    let mut _y: std::ffi::c_int = 0;
    for _i in 0..n {
        unsafe {
            _y = *ptr as std::ffi::c_int;
            ptr = ptr.offset(1);
        }
    }
    unsafe { strdup(ptr) }
}

fn strrwd(ptr: *mut std::ffi::c_char, n: i32) -> *mut std::ffi::c_char {
    let mut current_ptr = ptr;
    let mut _y: i32 = 0;
    
    for _i in 0..n {
        unsafe {
            _y = *current_ptr as i32;
            current_ptr = current_ptr.offset(-1);
        }
    }
    
    unsafe { strdup(current_ptr) }
}

fn get_part(url: *mut std::ffi::c_char, format: *const std::ffi::c_char, l: std::ffi::c_int) -> *mut std::ffi::c_char {
    let mut has: bool = false;
    let tmp: *mut std::ffi::c_char = unsafe { libc::malloc(std::mem::size_of::<std::ffi::c_char>()) as *mut std::ffi::c_char };
    let tmp_url: *mut std::ffi::c_char = unsafe { strdup(url) };
    let mut fmt_url: *mut std::ffi::c_char = unsafe { strdup(url) };
    let mut ret: *mut std::ffi::c_char = unsafe { libc::malloc(std::mem::size_of::<std::ffi::c_char>()) as *mut std::ffi::c_char };

    if tmp.is_null() || tmp_url.is_null() || fmt_url.is_null() || ret.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        libc::strcpy(tmp, b"\0".as_ptr() as *const std::ffi::c_char);
        libc::strcpy(fmt_url, b"\0".as_ptr() as *const std::ffi::c_char);
    }

    fmt_url = strff(fmt_url, l);

    unsafe {
        libc::sscanf(fmt_url, format, tmp);
    }

    if unsafe { libc::strcmp(tmp, tmp_url) } != 0 {
        has = true;
        ret = unsafe { strdup(tmp) };
    }

    fmt_url = strrwd(fmt_url, l);

    unsafe {
        libc::free(tmp as *mut libc::c_void);
        libc::free(tmp_url as *mut libc::c_void);
        libc::free(fmt_url as *mut libc::c_void);
    }

    if has { ret } else { std::ptr::null_mut() }
}

pub extern "C" fn url_parse(url: *mut ::core::ffi::c_char) -> *mut crate::types::url_data_t {
    unsafe {
        let data = libc::malloc(std::mem::size_of::<crate::types::url_data_t>()) as *mut crate::types::url_data_t;
        if data.is_null() { return std::ptr::null_mut(); }
        let tmp_url = strdup(url);
        let protocol = url_get_protocol(tmp_url);
        if protocol.is_null() { return std::ptr::null_mut(); }
        let protocol_len = libc::strlen(protocol) as i32 + 3;
        let is_ssh = url_is_ssh(protocol);
        let mut auth = libc::malloc(1) as *mut i8;
        let mut auth_len: i32 = 0;
        if !libc::strstr(tmp_url, b"@\0".as_ptr() as *const i8).is_null() {
            auth = get_part(tmp_url, b"%[^@]\0".as_ptr() as *const i8, protocol_len);
            auth_len = libc::strlen(auth) as i32;
            if !auth.is_null() { auth_len += 1; }
        }
        let hostname = if is_ssh { get_part(tmp_url, b"%[^:]\0".as_ptr() as *const i8, protocol_len + auth_len) }
                       else { get_part(tmp_url, b"%[^/]\0".as_ptr() as *const i8, protocol_len + auth_len) };
        if hostname.is_null() { return std::ptr::null_mut(); }
        let hostname_len = libc::strlen(hostname) as i32;
        let mut tmp_hostname = strdup(hostname);
        let host = libc::malloc(libc::strlen(tmp_hostname)) as *mut i8;
        libc::sscanf(tmp_hostname, b"%[^:]\0".as_ptr() as *const i8, host);
        if host.is_null() { return std::ptr::null_mut(); }
        let host_len = libc::strlen(host) as i32;
        let mut tmp_path = if is_ssh { get_part(tmp_url, b":%s\0".as_ptr() as *const i8, protocol_len + auth_len + hostname_len) }
                           else { get_part(tmp_url, b"/%s\0".as_ptr() as *const i8, protocol_len + auth_len + hostname_len) };
        let path = libc::malloc(libc::strlen(tmp_path)) as *mut i8;
        if path.is_null() { return std::ptr::null_mut(); }
        let fmt = if is_ssh { b"%s\0".as_ptr() } else { b"/%s\0".as_ptr() } as *const i8;
        libc::sprintf(path, fmt, tmp_path);
        libc::free(tmp_path as *mut _);
        let pathname = libc::malloc(1) as *mut i8;
        if pathname.is_null() { return std::ptr::null_mut(); }
        *pathname = 0;
        tmp_path = strdup(path);
        libc::sscanf(tmp_path, b"%[^? | ^#]\0".as_ptr() as *const i8, pathname);
        let pathname_len = libc::strlen(pathname) as i32;
        let search = libc::malloc(std::mem::size_of::<*mut i8>()) as *mut i8;
        if search.is_null() { return std::ptr::null_mut(); }
        tmp_path = strff(tmp_path, pathname_len);
        *search = 0;
        libc::sscanf(tmp_path, b"%[^#]\0".as_ptr() as *const i8, search);
        let search_len = libc::strlen(search) as i32;
        libc::free(tmp_path as *mut _);
        let query = libc::malloc(1) as *mut i8;
        if query.is_null() { return std::ptr::null_mut(); }
        *query = 0;
        libc::sscanf(search, b"?%s\0".as_ptr() as *const i8, query);
        let hash = libc::malloc(1) as *mut i8;
        if hash.is_null() { return std::ptr::null_mut(); }
        tmp_path = strff(path, pathname_len + search_len);
        *hash = 0;
        libc::sscanf(tmp_path, b"%s\0".as_ptr() as *const i8, hash);
        libc::free(tmp_path as *mut _);
        let port = libc::malloc(1) as *mut i8;
        if port.is_null() { return std::ptr::null_mut(); }
        *port = 0;
        tmp_hostname = strff(hostname, host_len + 1);
        libc::sscanf(tmp_hostname, b"%s\0".as_ptr() as *const i8, port);
        libc::free(tmp_hostname as *mut _);
        data
    }
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
    unsafe {
        let str_copy = strdup(str_ as *const ::core::ffi::c_char);
        if str_copy.is_null() {
            return false;
        }
        
        let ssh_str = b"ssh\0".as_ptr() as *const ::core::ffi::c_char;
        let git_str = b"git\0".as_ptr() as *const ::core::ffi::c_char;
        
        if libc::strcmp(str_copy, ssh_str) == 0 || libc::strcmp(str_copy, git_str) == 0 {
            libc::free(str_copy as *mut ::core::ffi::c_void);
            return true;
        }
        
        libc::free(str_copy as *mut ::core::ffi::c_void);
        false
    }
}

pub extern "C" fn url_get_protocol(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    unsafe {
        let protocol = libc::malloc(16 * std::mem::size_of::<::core::ffi::c_char>()) as *mut ::core::ffi::c_char;
        if protocol.is_null() {
            return std::ptr::null_mut();
        }
        libc::sscanf(
            url as *const ::core::ffi::c_char,
            b"%[^://]\0".as_ptr() as *const ::core::ffi::c_char,
            protocol,
        );
        if url_is_protocol(protocol) {
            return protocol;
        }
        libc::free(protocol as *mut ::core::ffi::c_void);
        std::ptr::null_mut()
    }
}

pub extern "C" fn url_get_auth(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let protocol = url_get_protocol(url);
    if protocol.is_null() {
        return std::ptr::null_mut();
    }
    let l: std::ffi::c_int = unsafe { libc::strlen(protocol) as std::ffi::c_int } + 3;
    get_part(url, b"%[^@]\0".as_ptr() as *const std::ffi::c_char, l)
}

pub extern "C" fn url_get_hostname(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut l: std::ffi::c_int = 3;
    let protocol = url_get_protocol(url);
    let tmp_protocol = unsafe { strdup(protocol) };
    let auth = url_get_auth(url);

    if protocol.is_null() {
        return std::ptr::null_mut();
    }

    if !auth.is_null() {
        l += unsafe { libc::strlen(auth) as std::ffi::c_int } + 1;
    }
    if !auth.is_null() {
        unsafe { libc::free(auth as *mut libc::c_void) };
    }

    l += unsafe { libc::strlen(protocol) as std::ffi::c_int };

    unsafe { libc::free(protocol as *mut libc::c_void) };

    let hostname = if url_is_ssh(tmp_protocol) {
        get_part(url, b"%[^:]\0".as_ptr() as *const std::ffi::c_char, l)
    } else {
        get_part(url, b"%[^/]\0".as_ptr() as *const std::ffi::c_char, l)
    };

    unsafe { libc::free(tmp_protocol as *mut libc::c_void) };

    hostname
}

pub extern "C" fn url_get_host(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    unsafe {
        let host = libc::malloc(std::mem::size_of::<::core::ffi::c_char>()) as *mut ::core::ffi::c_char;
        let hostname = url_get_hostname(url);

        if host.is_null() || hostname.is_null() {
            return std::ptr::null_mut();
        }

        let format = b"%[^:]\0".as_ptr() as *const ::core::ffi::c_char;
        libc::sscanf(hostname as *const ::core::ffi::c_char, format, host);

        libc::free(hostname as *mut ::core::ffi::c_void);

        host
    }
}

pub extern "C" fn url_get_pathname(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    unsafe {
        let path = url_get_path(url);
        let pathname = libc::malloc(std::mem::size_of::<::core::ffi::c_char>()) as *mut ::core::ffi::c_char;

        if path.is_null() || pathname.is_null() {
            return std::ptr::null_mut();
        }

        *pathname = 0;
        libc::sscanf(
            path as *const ::core::ffi::c_char,
            b"%[^?]\0".as_ptr() as *const ::core::ffi::c_char,
            pathname,
        );

        libc::free(path as *mut ::core::ffi::c_void);

        pathname
    }
}

pub extern "C" fn url_get_path(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut l: std::ffi::c_int = 3;
    let protocol = url_get_protocol(url);
    let auth = url_get_auth(url);
    let hostname = url_get_hostname(url);

    if protocol.is_null() || hostname.is_null() {
        return std::ptr::null_mut();
    }

    let is_ssh = url_is_ssh(protocol);

    l += unsafe { libc::strlen(protocol) as std::ffi::c_int + libc::strlen(hostname) as std::ffi::c_int };

    if !auth.is_null() {
        l += unsafe { libc::strlen(auth) as std::ffi::c_int } + 1;
    }

    let tmp_path = if is_ssh {
        get_part(url, b":%s\0".as_ptr() as *const std::ffi::c_char, l)
    } else {
        get_part(url, b"/%s\0".as_ptr() as *const std::ffi::c_char, l)
    };

    let fmt = if is_ssh {
        b"%s\0".as_ptr() as *const std::ffi::c_char
    } else {
        b"/%s\0".as_ptr() as *const std::ffi::c_char
    };

    let tmp_path_len = unsafe { libc::strlen(tmp_path) };
    let path = unsafe { libc::malloc((tmp_path_len + 2) * std::mem::size_of::<std::ffi::c_char>()) as *mut std::ffi::c_char };
    unsafe { libc::sprintf(path, fmt, tmp_path) };

    if !auth.is_null() {
        unsafe { libc::free(auth as *mut libc::c_void) };
    }
    unsafe { libc::free(protocol as *mut libc::c_void) };
    unsafe { libc::free(hostname as *mut libc::c_void) };
    unsafe { libc::free(tmp_path as *mut libc::c_void) };

    path
}

pub extern "C" fn url_get_search(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let path = url_get_path(url);
    let pathname = url_get_pathname(url);
    let search = unsafe { libc::malloc(core::mem::size_of::<::core::ffi::c_char>()) as *mut ::core::ffi::c_char };

    if path.is_null() || search.is_null() {
        return core::ptr::null_mut();
    }

    let pathname_len = unsafe { libc::strlen(pathname) as std::ffi::c_int };
    let tmp_path = strff(path, pathname_len);
    
    unsafe {
        *search = 0;
        libc::sscanf(tmp_path, b"%[^#]\0".as_ptr() as *const ::core::ffi::c_char, search);
    }

    let _ = strrwd(tmp_path, pathname_len as i32);

    unsafe {
        libc::free(path as *mut ::core::ffi::c_void);
        libc::free(pathname as *mut ::core::ffi::c_void);
    }

    search
}

pub extern "C" fn url_get_query(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let search = url_get_search(url);
    let query = unsafe { libc::malloc(std::mem::size_of::<::core::ffi::c_char>()) as *mut ::core::ffi::c_char };
    if search.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        libc::sscanf(search, b"?%s\0".as_ptr() as *const ::core::ffi::c_char, query);
        libc::free(search as *mut ::core::ffi::c_void);
    }
    query
}

pub extern "C" fn url_get_hash(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    unsafe {
        let hash: *mut ::core::ffi::c_char = libc::malloc(core::mem::size_of::<::core::ffi::c_char>()) as *mut ::core::ffi::c_char;
        if hash.is_null() {
            return core::ptr::null_mut();
        }

        let path = url_get_path(url);
        if path.is_null() {
            return core::ptr::null_mut();
        }

        let pathname = url_get_pathname(url);
        if pathname.is_null() {
            return core::ptr::null_mut();
        }

        let search = url_get_search(url);

        let pathname_len: i32 = libc::strlen(pathname) as i32;
        let search_len: i32 = libc::strlen(search) as i32;
        let mut tmp_path = strff(path, pathname_len + search_len);

        *hash = 0;
        libc::sscanf(tmp_path, b"%s\0".as_ptr() as *const ::core::ffi::c_char, hash);
        tmp_path = strrwd(tmp_path, pathname_len + search_len);
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
        let port = libc::malloc(std::mem::size_of::<::core::ffi::c_char>()) as *mut ::core::ffi::c_char;
        let hostname = url_get_hostname(url);
        let host = url_get_host(url);
        
        if port.is_null() || hostname.is_null() {
            return std::ptr::null_mut();
        }
        
        let host_len = libc::strlen(host);
        let tmp_hostname = strff(hostname, (host_len + 1) as std::ffi::c_int);
        
        libc::sscanf(tmp_hostname, b"%s\0".as_ptr() as *const ::core::ffi::c_char, port);
        
        libc::free(hostname as *mut ::core::ffi::c_void);
        libc::free(tmp_hostname as *mut ::core::ffi::c_void);
        
        port
    }
}

pub extern "C" fn url_inspect(url: *mut ::core::ffi::c_char) {
    let data = url_parse(url);
    url_data_inspect(data);
}

pub extern "C" fn url_data_inspect(data: *mut crate::types::url_data_t) {
    unsafe {
        libc::printf(b"#url =>\n\0".as_ptr() as *const i8);
        
        let href_ptr = crate::compat::c2r_field_ptr_url_data_t__href(data as *mut ::core::ffi::c_void);
        libc::printf(b"    .href: \"%s\"\n\0".as_ptr() as *const i8, *(href_ptr as *const *mut i8));
        
        let protocol_ptr = crate::compat::c2r_field_ptr_url_data_t__protocol(data as *mut ::core::ffi::c_void);
        libc::printf(b"    .protocol: \"%s\"\n\0".as_ptr() as *const i8, *(protocol_ptr as *const *mut i8));
        
        let host_ptr = crate::compat::c2r_field_ptr_url_data_t__host(data as *mut ::core::ffi::c_void);
        libc::printf(b"    .host: \"%s\"\n\0".as_ptr() as *const i8, *(host_ptr as *const *mut i8));
        
        let auth_ptr = crate::compat::c2r_field_ptr_url_data_t__auth(data as *mut ::core::ffi::c_void);
        libc::printf(b"    .auth: \"%s\"\n\0".as_ptr() as *const i8, *(auth_ptr as *const *mut i8));
        
        let hostname_ptr = crate::compat::c2r_field_ptr_url_data_t__hostname(data as *mut ::core::ffi::c_void);
        libc::printf(b"    .hostname: \"%s\"\n\0".as_ptr() as *const i8, *(hostname_ptr as *const *mut i8));
        
        let pathname_ptr = crate::compat::c2r_field_ptr_url_data_t__pathname(data as *mut ::core::ffi::c_void);
        libc::printf(b"    .pathname: \"%s\"\n\0".as_ptr() as *const i8, *(pathname_ptr as *const *mut i8));
        
        let search_ptr = crate::compat::c2r_field_ptr_url_data_t__search(data as *mut ::core::ffi::c_void);
        libc::printf(b"    .search: \"%s\"\n\0".as_ptr() as *const i8, *(search_ptr as *const *mut i8));
        
        let path_ptr = crate::compat::c2r_field_ptr_url_data_t__path(data as *mut ::core::ffi::c_void);
        libc::printf(b"    .path: \"%s\"\n\0".as_ptr() as *const i8, *(path_ptr as *const *mut i8));
        
        let hash_ptr = crate::compat::c2r_field_ptr_url_data_t__hash(data as *mut ::core::ffi::c_void);
        libc::printf(b"    .hash: \"%s\"\n\0".as_ptr() as *const i8, *(hash_ptr as *const *mut i8));
        
        let query_ptr = crate::compat::c2r_field_ptr_url_data_t__query(data as *mut ::core::ffi::c_void);
        libc::printf(b"    .query: \"%s\"\n\0".as_ptr() as *const i8, *(query_ptr as *const *mut i8));
        
        let port_ptr = crate::compat::c2r_field_ptr_url_data_t__port(data as *mut ::core::ffi::c_void);
        libc::printf(b"    .port: \"%s\"\n\0".as_ptr() as *const i8, *(port_ptr as *const *mut i8));
    }
}

pub extern "C" fn url_free(data: *mut crate::types::url_data_t) {
    if data.is_null() {
        return;
    }
    
    unsafe {
        let auth_ptr = crate::compat::c2r_field_ptr_url_data_t__auth(data as *mut ::core::ffi::c_void);
        let auth = *(auth_ptr as *const *mut ::core::ffi::c_char);
        if !auth.is_null() {
            libc::free(auth as *mut ::core::ffi::c_void);
        }
        
        let protocol_ptr = crate::compat::c2r_field_ptr_url_data_t__protocol(data as *mut ::core::ffi::c_void);
        let protocol = *(protocol_ptr as *const *mut ::core::ffi::c_char);
        if !protocol.is_null() {
            libc::free(protocol as *mut ::core::ffi::c_void);
        }
        
        let hostname_ptr = crate::compat::c2r_field_ptr_url_data_t__hostname(data as *mut ::core::ffi::c_void);
        let hostname = *(hostname_ptr as *const *mut ::core::ffi::c_char);
        if !hostname.is_null() {
            libc::free(hostname as *mut ::core::ffi::c_void);
        }
        
        let host_ptr = crate::compat::c2r_field_ptr_url_data_t__host(data as *mut ::core::ffi::c_void);
        let host = *(host_ptr as *const *mut ::core::ffi::c_char);
        if !host.is_null() {
            libc::free(host as *mut ::core::ffi::c_void);
        }
        
        let pathname_ptr = crate::compat::c2r_field_ptr_url_data_t__pathname(data as *mut ::core::ffi::c_void);
        let pathname = *(pathname_ptr as *const *mut ::core::ffi::c_char);
        if !pathname.is_null() {
            libc::free(pathname as *mut ::core::ffi::c_void);
        }
        
        let path_ptr = crate::compat::c2r_field_ptr_url_data_t__path(data as *mut ::core::ffi::c_void);
        let path = *(path_ptr as *const *mut ::core::ffi::c_char);
        if !path.is_null() {
            libc::free(path as *mut ::core::ffi::c_void);
        }
        
        let hash_ptr = crate::compat::c2r_field_ptr_url_data_t__hash(data as *mut ::core::ffi::c_void);
        let hash = *(hash_ptr as *const *mut ::core::ffi::c_char);
        if !hash.is_null() {
            libc::free(hash as *mut ::core::ffi::c_void);
        }
        
        let search_ptr = crate::compat::c2r_field_ptr_url_data_t__search(data as *mut ::core::ffi::c_void);
        let search = *(search_ptr as *const *mut ::core::ffi::c_char);
        if !search.is_null() {
            libc::free(search as *mut ::core::ffi::c_void);
        }
        
        let query_ptr = crate::compat::c2r_field_ptr_url_data_t__query(data as *mut ::core::ffi::c_void);
        let query = *(query_ptr as *const *mut ::core::ffi::c_char);
        if !query.is_null() {
            libc::free(query as *mut ::core::ffi::c_void);
        }
    }
}
