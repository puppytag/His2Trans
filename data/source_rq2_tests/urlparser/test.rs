// Rust translation of urlparser test.c
// Copyright (c) 2024
// Translated from C to Rust

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

// url_data_t opaque type
#[repr(C)]
pub struct url_data_t {
    pub whole_url: *mut c_char,
    pub protocol: *mut c_char,
    pub userinfo: *mut c_char,
    pub host: *mut c_char,
    pub port: *mut c_char,
    pub path: *mut c_char,
    pub query: *mut c_char,
    pub fragment: *mut c_char,
    // Additional fields may exist but we don't need them
}

// External declarations for url library functions
extern "C" {
    fn url_parse(url: *const c_char) -> *mut url_data_t;
    fn url_free(data: *mut url_data_t);
    fn url_data_inspect(data: *const url_data_t);
    fn url_is_protocol(proto: *const c_char) -> i32;
    fn url_get_protocol(url: *const c_char) -> *mut c_char;
    fn url_get_userinfo(url: *const c_char) -> *mut c_char;
    fn url_get_hostname(url: *const c_char) -> *mut c_char;
    fn url_get_path(url: *const c_char) -> *mut c_char;
    fn url_get_fragment(url: *const c_char) -> *mut c_char;
    fn url_get_port(url: *const c_char) -> *mut c_char;
    fn url_get_query_value(data: *const url_data_t, key: *const c_char) -> *const c_char;
}

/// Macro to assert string equality and free the returned string
macro_rules! string_assert {
    ($expect:expr, $str_ptr:expr) => {
        unsafe {
            let s = $str_ptr;
            assert!(!s.is_null(), "String pointer is null");
            let actual = CStr::from_ptr(s).to_str().unwrap();
            assert_eq!($expect, actual, "String mismatch: expected '{}', got '{}'", $expect, actual);
            libc::free(s as *mut libc::c_void);
        }
    };
}

fn main() {
    unsafe {
        let gh_url_str = CString::new("git://git@github.com:jwerle/url.h.git").unwrap();
        let url_str = CString::new("http://user:pass@subdomain.host.com:8080/p/%C3%A5/t/h?qu%C3%ABry=strin%C4%9F&foo=bar=yuk&key%23%26%3D=%25&lol#h%C3%a6sh").unwrap();

        let parsed = url_parse(url_str.as_ptr());
        let gh_parsed = url_parse(gh_url_str.as_ptr());

        assert!(!parsed.is_null(), "Failed to parse URL");
        assert!(!gh_parsed.is_null(), "Failed to parse GitHub URL");

        url_data_inspect(parsed);
        url_data_inspect(gh_parsed);

        // Check parsed URL fields
        assert!(!(*parsed).whole_url.is_null());
        assert!(!(*parsed).protocol.is_null());
        assert!(!(*parsed).userinfo.is_null());
        assert!(!(*parsed).host.is_null());
        assert!(!(*parsed).port.is_null());
        assert!(!(*parsed).path.is_null());
        assert!(!(*parsed).query.is_null());
        assert!(!(*parsed).fragment.is_null());

        // Check GitHub parsed URL fields
        assert!(!(*gh_parsed).whole_url.is_null());
        assert!(!(*gh_parsed).protocol.is_null());
        assert!(!(*gh_parsed).userinfo.is_null());
        assert!(!(*gh_parsed).host.is_null());
        assert!(!(*gh_parsed).path.is_null());

        // Test protocol detection
        let http = CString::new("http").unwrap();
        let https = CString::new("https").unwrap();
        let git = CString::new("git").unwrap();
        let ssh = CString::new("ssh").unwrap();
        let sftp = CString::new("sftp").unwrap();
        let ftp = CString::new("ftp").unwrap();
        let javascript = CString::new("javascript").unwrap();

        assert!(url_is_protocol(http.as_ptr()) != 0);
        assert!(url_is_protocol(https.as_ptr()) != 0);
        assert!(url_is_protocol(git.as_ptr()) != 0);
        assert!(url_is_protocol(ssh.as_ptr()) != 0);
        assert!(url_is_protocol(sftp.as_ptr()) != 0);
        assert!(url_is_protocol(ftp.as_ptr()) != 0);
        assert!(url_is_protocol(javascript.as_ptr()) != 0);

        // Test URL component extraction
        string_assert!("http", url_get_protocol(url_str.as_ptr()));
        string_assert!("user:pass", url_get_userinfo(url_str.as_ptr()));
        string_assert!("subdomain.host.com", url_get_hostname(url_str.as_ptr()));
        string_assert!("/p/\u{00c3}\u{00a5}/t/h", url_get_path(url_str.as_ptr()));

        // Test query values
        let query_key1 = CString::new("qu\u{00c3}\u{00ab}ry").unwrap();
        let query_val1 = url_get_query_value(parsed, query_key1.as_ptr());
        assert!(!query_val1.is_null());
        let val1_str = CStr::from_ptr(query_val1).to_str().unwrap();
        assert_eq!(val1_str, "strin\u{00c4}\u{009f}");

        let query_key2 = CString::new("foo").unwrap();
        let query_val2 = url_get_query_value(parsed, query_key2.as_ptr());
        assert!(!query_val2.is_null());
        let val2_str = CStr::from_ptr(query_val2).to_str().unwrap();
        assert_eq!(val2_str, "bar=yuk");

        let query_key3 = CString::new("key#&=").unwrap();
        let query_val3 = url_get_query_value(parsed, query_key3.as_ptr());
        assert!(!query_val3.is_null());
        let val3_str = CStr::from_ptr(query_val3).to_str().unwrap();
        assert_eq!(val3_str, "%");

        let query_key4 = CString::new("lol").unwrap();
        let query_val4 = url_get_query_value(parsed, query_key4.as_ptr());
        assert!(!query_val4.is_null());
        let val4_str = CStr::from_ptr(query_val4).to_str().unwrap();
        assert_eq!(val4_str, "");

        string_assert!("hæsh", url_get_fragment(url_str.as_ptr()));
        string_assert!("8080", url_get_port(url_str.as_ptr()));

        // Test GitHub URL extraction
        string_assert!("git", url_get_protocol(gh_url_str.as_ptr()));
        string_assert!("github.com", url_get_hostname(gh_url_str.as_ptr()));
        string_assert!("git", url_get_userinfo(gh_url_str.as_ptr()));
        string_assert!("jwerle/url.h.git", url_get_path(gh_url_str.as_ptr()));

        url_free(parsed);
        url_free(gh_parsed);

        println!("urlparser tests: ok");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_url() {
        unsafe {
            let url_str = CString::new("http://user:pass@example.com:8080/path?query=value#fragment").unwrap();
            let parsed = url_parse(url_str.as_ptr());
            assert!(!parsed.is_null());
            assert!(!(*parsed).protocol.is_null());
            assert!(!(*parsed).host.is_null());
            url_free(parsed);
        }
    }

    #[test]
    fn test_protocol_detection() {
        unsafe {
            let http = CString::new("http").unwrap();
            let https = CString::new("https").unwrap();
            let git = CString::new("git").unwrap();

            assert!(url_is_protocol(http.as_ptr()) != 0);
            assert!(url_is_protocol(https.as_ptr()) != 0);
            assert!(url_is_protocol(git.as_ptr()) != 0);
        }
    }

    #[test]
    fn test_get_protocol() {
        unsafe {
            let url_str = CString::new("https://example.com").unwrap();
            let proto = url_get_protocol(url_str.as_ptr());
            assert!(!proto.is_null());
            let proto_str = CStr::from_ptr(proto).to_str().unwrap();
            assert_eq!(proto_str, "https");
            libc::free(proto as *mut libc::c_void);
        }
    }

    #[test]
    fn test_get_hostname() {
        unsafe {
            let url_str = CString::new("https://example.com/path").unwrap();
            let host = url_get_hostname(url_str.as_ptr());
            assert!(!host.is_null());
            let host_str = CStr::from_ptr(host).to_str().unwrap();
            assert_eq!(host_str, "example.com");
            libc::free(host as *mut libc::c_void);
        }
    }

    #[test]
    fn test_git_url() {
        unsafe {
            let gh_url_str = CString::new("git://git@github.com:jwerle/url.h.git").unwrap();
            let parsed = url_parse(gh_url_str.as_ptr());
            assert!(!parsed.is_null());
            assert!(!(*parsed).protocol.is_null());
            assert!(!(*parsed).host.is_null());
            url_free(parsed);
        }
    }
}
