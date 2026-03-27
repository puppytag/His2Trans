// c2saferrust 测试文件 for urlparser
// 目标：与 evolc2rust 套件对齐的真实解析正确性测试（protocol/host/port/path/query/hash）。

use crate::*;
use std::ffi::{CStr, CString};
use std::ptr::NonNull;

unsafe fn ptr_to_string(ptr: *mut std::os::raw::c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    CStr::from_ptr(ptr).to_string_lossy().into_owned()
}

unsafe fn free_c(ptr: *mut std::os::raw::c_char) {
    if !ptr.is_null() {
        libc::free(ptr as *mut _);
    }
}

#[test]
fn test_url_is_protocol() {
    let http = CString::new("http").unwrap();
    let https = CString::new("https").unwrap();
    let file = CString::new("file").unwrap();
    assert!(url_is_protocol(http.as_ptr() as *mut _));
    assert!(url_is_protocol(https.as_ptr() as *mut _));
    assert!(!url_is_protocol(file.as_ptr() as *mut _));
}

#[test]
fn test_url_getters() {
    unsafe {
        let url = CString::new("http://user:pass@subdomain.host.com:8080/p/a/t/h?query=string#hash").unwrap();

        let proto = url_get_protocol(url.as_ptr() as *mut _);
        let auth = url_get_auth(url.as_ptr() as *mut _);
        let hostname = url_get_hostname(url.as_ptr() as *mut _);
        let host = url_get_host(url.as_ptr() as *mut _);
        let port = url_get_port(url.as_ptr() as *mut _);
        let pathname = url_get_pathname(url.as_ptr() as *mut _);
        let path = url_get_path(url.as_ptr() as *mut _);
        let search = url_get_search(url.as_ptr() as *mut _);
        let query = url_get_query(url.as_ptr() as *mut _);
        let hash = url_get_hash(url.as_ptr() as *mut _);

        assert_eq!(ptr_to_string(proto), "http");
        assert_eq!(ptr_to_string(auth), "user:pass");

        assert_eq!(ptr_to_string(hostname), "subdomain.host.com:8080");
        assert_eq!(ptr_to_string(host), "subdomain.host.com");
        assert_eq!(ptr_to_string(port), "8080");
        assert_eq!(ptr_to_string(pathname), "/p/a/t/h");
        assert_eq!(ptr_to_string(path), "/p/a/t/h?query=string#hash");
        assert_eq!(ptr_to_string(search), "?query=string");
        assert_eq!(ptr_to_string(query), "query=string");
        assert_eq!(ptr_to_string(hash), "#hash");

        free_c(proto);
        free_c(auth);
        free_c(hostname);
        free_c(host);
        free_c(port);
        free_c(pathname);
        free_c(path);
        free_c(search);
        free_c(query);
        free_c(hash);
    }
}

#[test]
fn test_url_parse_basic() {
    unsafe {
        let url = CString::new("http://user:pass@subdomain.host.com:8080/p/a/t/h?query=string#hash").unwrap();
        let gh_url = CString::new("git://git@github.com:jwerle/url.h.git").unwrap();
        let data = url_parse(url.as_ptr() as *mut _);
        let gh_data = url_parse(gh_url.as_ptr() as *mut _);
        assert!(!data.is_null(), "url_parse 应返回有效指针");
        assert!(!gh_data.is_null(), "url_parse(gh_url) 应返回有效指针");

        // Match upstream test.c: ensure key fields are present.
        assert!(!(*data).protocol.is_null());
        assert!(!(*data).auth.is_null());
        assert!(!(*data).hostname.is_null());
        assert!(!(*data).port.is_null());
        assert!(!(*data).path.is_null());
        assert!(!(*data).pathname.is_null());
        assert!(!(*data).search.is_null());
        assert!(!(*data).query.is_null());
        assert!(!(*data).hash.is_null());
        assert!(!(*data).host.is_null());
        assert!(!(*data).href.is_null());

        assert_eq!(CStr::from_ptr((*data).protocol).to_string_lossy(), "http");
        assert_eq!(CStr::from_ptr((*data).auth).to_string_lossy(), "user:pass");
        assert_eq!(
            CStr::from_ptr((*data).hostname).to_string_lossy(),
            "subdomain.host.com:8080"
        );
        assert_eq!(CStr::from_ptr((*data).host).to_string_lossy(), "subdomain.host.com");
        assert_eq!(CStr::from_ptr((*data).port).to_string_lossy(), "8080");
        assert_eq!(CStr::from_ptr((*data).pathname).to_string_lossy(), "/p/a/t/h");
        assert_eq!(
            CStr::from_ptr((*data).path).to_string_lossy(),
            "/p/a/t/h?query=string#hash"
        );
        assert_eq!(CStr::from_ptr((*data).search).to_string_lossy(), "?query=string");
        assert_eq!(CStr::from_ptr((*data).query).to_string_lossy(), "query=string");
        assert_eq!(CStr::from_ptr((*data).hash).to_string_lossy(), "#hash");

        url_free(NonNull::new(data));
        url_free(NonNull::new(gh_data));
    }
}
