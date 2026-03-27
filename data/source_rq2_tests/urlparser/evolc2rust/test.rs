// evo-c2rust-v2 测试文件 for urlparser (url.h)
//
// 目标：与上游 `test_module/urlparser/tests/test.c` 的断言保持一致（3 个 #[test]）。

use crate::src::url_h::*;

use libc::{c_char, c_void};
use std::ffi::{CStr, CString};

unsafe fn cstr_to_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    CStr::from_ptr(ptr).to_string_lossy().to_string()
}

unsafe fn free_c_malloc(ptr: *mut c_char) {
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

        assert_eq!(url_is_protocol(http.as_ptr()), 1);
        assert_eq!(url_is_protocol(https.as_ptr()), 1);
        assert_eq!(url_is_protocol(git.as_ptr()), 1);
        assert_eq!(url_is_protocol(file.as_ptr()), 0);
    }
}

#[test]
fn test_url_getters_match_reference() {
    unsafe {
        let url = CString::new("http://user:pass@subdomain.host.com:8080/p/a/t/h?query=string#hash").unwrap();
        let gh_url = CString::new("git://git@github.com:jwerle/url.h.git").unwrap();

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

        free_c_malloc(proto);
        free_c_malloc(auth);
        free_c_malloc(hostname);
        free_c_malloc(host);
        free_c_malloc(pathname);
        free_c_malloc(path);
        free_c_malloc(search);
        free_c_malloc(query);
        free_c_malloc(hash);
        free_c_malloc(port);

        let gh_proto = url_get_protocol(gh_url.as_ptr() as *mut _);
        let gh_host = url_get_host(gh_url.as_ptr() as *mut _);
        let gh_hostname = url_get_hostname(gh_url.as_ptr() as *mut _);
        let gh_auth = url_get_auth(gh_url.as_ptr() as *mut _);
        let gh_pathname = url_get_pathname(gh_url.as_ptr() as *mut _);
        let gh_path = url_get_path(gh_url.as_ptr() as *mut _);

        assert_eq!(cstr_to_string(gh_proto), "git");
        assert_eq!(cstr_to_string(gh_host), "github.com");
        assert_eq!(cstr_to_string(gh_hostname), "github.com");
        assert_eq!(cstr_to_string(gh_auth), "git");
        assert_eq!(cstr_to_string(gh_pathname), "jwerle/url.h.git");
        assert_eq!(cstr_to_string(gh_path), "jwerle/url.h.git");

        free_c_malloc(gh_proto);
        free_c_malloc(gh_host);
        free_c_malloc(gh_hostname);
        free_c_malloc(gh_auth);
        free_c_malloc(gh_pathname);
        free_c_malloc(gh_path);
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

