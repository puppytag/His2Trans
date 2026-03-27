// simcrat 测试文件 for urlparser
//
// 目标：与上游 `test_module/urlparser/tests/test.c` 的断言保持一致。

use crate::*;

#[test]
fn test_url_is_protocol() {
    assert!(url_is_protocol("http"));
    assert!(url_is_protocol("https"));
    assert!(!url_is_protocol("file"));
}

#[test]
fn test_url_getters() {
    let url = "http://user:pass@subdomain.host.com:8080/p/a/t/h?query=string#hash";
    let gh_url = "git://git@github.com:jwerle/url.h.git";

    assert_eq!(url_get_protocol(url).expect("url_get_protocol failed"), "http");
    assert_eq!(url_get_auth(url).expect("url_get_auth failed"), "user:pass");
    assert_eq!(
        url_get_hostname(url).expect("url_get_hostname failed"),
        "subdomain.host.com:8080"
    );
    assert_eq!(url_get_host(url).expect("url_get_host missing"), "subdomain.host.com");
    assert_eq!(url_get_pathname(url).expect("url_get_pathname failed"), "/p/a/t/h");
    assert_eq!(
        url_get_path(url).expect("url_get_path failed"),
        "/p/a/t/h?query=string#hash"
    );
    assert_eq!(url_get_search(url).expect("url_get_search missing"), "?query=string");
    assert_eq!(url_get_query(url).expect("url_get_query missing"), "query=string");
    assert_eq!(url_get_hash(url).expect("url_get_hash failed"), "#hash");
    assert_eq!(url_get_port(url).expect("url_get_port missing"), "8080");

    assert_eq!(url_get_protocol(gh_url).expect("url_get_protocol(gh) failed"), "git");
    assert_eq!(url_get_host(gh_url).expect("url_get_host(gh) missing"), "github.com");
    assert_eq!(url_get_hostname(gh_url).expect("url_get_hostname(gh) failed"), "github.com");
    assert_eq!(url_get_auth(gh_url).expect("url_get_auth(gh) failed"), "git");
    assert_eq!(url_get_pathname(gh_url).expect("url_get_pathname(gh) failed"), "jwerle/url.h.git");
    assert_eq!(url_get_path(gh_url).expect("url_get_path(gh) failed"), "jwerle/url.h.git");
}

#[test]
fn test_url_parse_nonzero() {
    let url = "http://user:pass@subdomain.host.com:8080/p/a/t/h?query=string#hash";
    let gh_url = "git://git@github.com:jwerle/url.h.git";

    let parsed = url_parse(url).expect("url_parse failed");
    let gh_parsed = url_parse(gh_url).expect("url_parse(gh) failed");

    assert!(parsed != 0);
    assert!(gh_parsed != 0);
}
