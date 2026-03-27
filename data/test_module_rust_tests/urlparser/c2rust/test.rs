// Unified RQ2 test suite for urlparser in C2Rust mode.

use crate::src_test::*;
use libc::{c_char, c_void};
use std::ffi::{CStr, CString};

unsafe fn cstr_to_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    CStr::from_ptr(ptr).to_string_lossy().to_string()
}

unsafe fn free_c(ptr: *mut c_char) {
    if !ptr.is_null() {
        libc::free(ptr.cast::<c_void>());
    }
}

#[test]
fn test_url_is_protocol() {
    unsafe {
        let http = CString::new("http").unwrap();
        let https = CString::new("https").unwrap();
        let git = CString::new("git").unwrap();
        let file = CString::new("file").unwrap();
        assert!(url_is_protocol(http.as_ptr() as *mut _));
        assert!(url_is_protocol(https.as_ptr() as *mut _));
        assert!(url_is_protocol(git.as_ptr() as *mut _));
        assert!(!url_is_protocol(file.as_ptr() as *mut _));
    }
}

#[test]
fn test_url_getters_match_reference() {
    unsafe {
        let url = CString::new("http://user:pass@subdomain.host.com:8080/p/a/t/h?query=string#hash").unwrap();
        let proto = url_get_protocol(url.as_ptr() as *mut _);
        let auth = url_get_auth(url.as_ptr() as *mut _);
        let hostname = url_get_hostname(url.as_ptr() as *mut _);
        let host = url_get_host(url.as_ptr() as *mut _);
        let pathname = url_get_pathname(url.as_ptr() as *mut _);
        let path = url_get_path(url.as_ptr() as *mut _);
        let search = url_get_search(url.as_ptr() as *mut _);
        let query = url_get_query(url.as_ptr() as *mut _);
        let hash = url_get_hash(url.as_ptr() as *mut _);
        let port = url_get_port(url.as_ptr() as *mut _);

        assert_eq!(cstr_to_string(proto), "http");
        assert_eq!(cstr_to_string(auth), "user:pass");
        assert_eq!(cstr_to_string(hostname), "subdomain.host.com:8080");
        assert_eq!(cstr_to_string(host), "subdomain.host.com");
        assert_eq!(cstr_to_string(pathname), "/p/a/t/h");
        assert_eq!(cstr_to_string(path), "/p/a/t/h?query=string#hash");
        assert_eq!(cstr_to_string(search), "?query=string");
        assert_eq!(cstr_to_string(query), "query=string");
        assert_eq!(cstr_to_string(hash), "#hash");
        assert_eq!(cstr_to_string(port), "8080");

        free_c(proto);
        free_c(auth);
        free_c(hostname);
        free_c(host);
        free_c(pathname);
        free_c(path);
        free_c(search);
        free_c(query);
        free_c(hash);
        free_c(port);
    }
}

#[test]
fn test_url_parse_non_null() {
    unsafe {
        let url = CString::new("http://user:pass@subdomain.host.com:8080/p/a/t/h?query=string#hash").unwrap();
        let gh_url = CString::new("git://git@github.com:jwerle/url.h.git").unwrap();
        let parsed = url_parse(url.as_ptr() as *mut _);
        let gh_parsed = url_parse(gh_url.as_ptr() as *mut _);
        assert!(!parsed.is_null());
        assert!(!gh_parsed.is_null());
        url_free(parsed);
        url_free(gh_parsed);
    }
}
