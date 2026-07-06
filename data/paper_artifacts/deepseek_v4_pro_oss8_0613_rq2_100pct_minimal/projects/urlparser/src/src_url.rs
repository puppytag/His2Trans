//! Module: src_url - Safe Rust URL parser
//!
//! This module provides URL parsing functions with safe Rust.

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::collections::HashSet;
use std::sync::OnceLock;

use crate::types::*;
use crate::globals::URL_SCHEMES;

// --- kept bswap/identity helpers (safe) ---

fn __bswap_16(__bsx: crate::types::__uint16_t)-> crate::types::__uint16_t {
    ((__bsx >> 8) & 0xff) | ((__bsx & 0xff) << 8)
}

fn __bswap_32(__bsx: crate::types::__uint32_t)-> crate::types::__uint32_t {
    let val: u32 = __bsx as u32;
    let result: u32 = ((val & 0xff000000u32) >> 24)
        | ((val & 0x00ff0000u32) >> 8)
        | ((val & 0x0000ff00u32) << 8)
        | ((val & 0x000000ffu32) << 24);
    result as crate::types::__uint32_t
}

pub fn __bswap_64(__bsx: crate::types::__uint64_t)-> crate::types::__uint64_t {
    let x = __bsx as u64;
    let swapped = ((x & 0xff00000000000000) >> 56)
        | ((x & 0x00ff000000000000) >> 40)
        | ((x & 0x0000ff0000000000) >> 24)
        | ((x & 0x000000ff00000000) >> 8)
        | ((x & 0x00000000ff000000) << 8)
        | ((x & 0x0000000000ff0000) << 24)
        | ((x & 0x000000000000ff00) << 40)
        | ((x & 0x00000000000000ff) << 56);
    swapped as crate::types::__uint64_t
}

fn __uint16_identity(__x: crate::types::__uint16_t)-> crate::types::__uint16_t {
    __x
}

fn __uint32_identity(__x: crate::types::__uint32_t)-> crate::types::__uint32_t {
    __x
}

fn __uint64_identity(__x: crate::types::__uint64_t)-> crate::types::__uint64_t {
    __x
}

// --- URL parsing data structures ---

/// Rust-native representation of parsed URL parts.
#[derive(Debug, Clone, Default)]
pub struct UrlData {
    pub href: String,
    pub protocol: String,
    pub host: String,
    pub auth: String,
    pub hostname: String,
    pub pathname: String,
    pub search: String,
    pub path: String,
    pub hash: String,
    pub query: String,
    pub port: String,
}

// --- scheme helpers ---

fn known_schemes() -> &'static HashSet<String> {
    static SCHEMES: OnceLock<HashSet<String>> = OnceLock::new();
    SCHEMES.get_or_init(|| {
        URL_SCHEMES
            .iter()
            .map(|bytes| {
                let len = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
                String::from_utf8_lossy(&bytes[..len]).into_owned()
            })
            .collect()
    })
}

// --- public API ---

pub fn url_is_protocol(protocol: &str) -> bool {
    known_schemes().contains(protocol)
}

pub fn url_is_ssh(protocol: &str) -> bool {
    protocol == "ssh" || protocol == "git"
}

/// Parse a URL string into its components.
pub fn url_parse(url: &str) -> Option<UrlData> {
    let href = url.to_string();

    // 1. protocol (C uses sscanf %[^://] which stops at first ':' or '/')
    let sep_pos = url.find(|c: char| c == ':' || c == '/')?;
    let protocol = &url[..sep_pos];
    if !url_is_protocol(protocol) {
        return None;
    }
    let protocol = protocol.to_string();
    let is_ssh = url_is_ssh(&protocol);
    let rest = if url[sep_pos..].starts_with("://") {
        &url[sep_pos + 3..]
    } else {
        // single ':' separator (e.g. mailto:)
        &url[sep_pos + 1..]
    };

    // 2. auth
    let (auth, after_auth) = if let Some(at_pos) = rest.find('@') {
        let a = rest[..at_pos].to_string();
        (a, &rest[at_pos + 1..])
    } else {
        (String::new(), rest)
    };

    // 3. hostname (and possibly port) terminated by ':' (ssh) or '/' (else)
    let hostname_end = if is_ssh {
        after_auth.find(':').unwrap_or(after_auth.len())
    } else {
        after_auth.find('/').unwrap_or(after_auth.len())
    };
    let hostname_full = &after_auth[..hostname_end];
    let path_part = &after_auth[hostname_end..]; // starts with ':' (ssh) or '/' (else) or empty

    // 4. host and port from hostname_full
    let (host, port) = if let Some(colon_pos) = hostname_full.find(':') {
        (
            hostname_full[..colon_pos].to_string(),
            hostname_full[colon_pos + 1..].to_string(),
        )
    } else {
        (hostname_full.to_string(), String::new())
    };
    let hostname = hostname_full.to_string();

    // 5. path (C get_part with ":%s" strips leading ':' for SSH)
    let path = if is_ssh {
        // ssh: path_part starts with ':', strip it to match C sprintf("%s", tmp_path)
        if path_part.len() > 1 {
            path_part[1..].to_string()
        } else {
            String::from("/")
        }
    } else {
        // non-ssh: path_part starts with '/', e.g. "/path"
        if path_part.is_empty() {
            String::from("/")
        } else {
            path_part.to_string()
        }
    };

    // 6. pathname: part of path before '?', ' ', '|', '^', '#' (C sscanf "%[^? | ^#]")
    let pathname_end = path.find(|c: char| c == '?' || c == '#' || c == ' ' || c == '|' || c == '^').unwrap_or(path.len());
    let pathname = path[..pathname_end].to_string();

    // 7. search: part after pathname until '#'
    let after_pathname = &path[pathname_end..];
    let search_end = after_pathname.find('#').unwrap_or(after_pathname.len());
    let search = after_pathname[..search_end].to_string();

    // 8. hash: after '#'
    let hash = if search_end < after_pathname.len() {
        after_pathname[search_end..].to_string() // includes '#'
    } else {
        String::new()
    };

    // 9. query: part after '?' in search
    let query = if let Some(q_start) = search.find('?') {
        search[q_start + 1..].to_string()
    } else {
        String::new()
    };

    Some(UrlData {
        href,
        protocol,
        host,
        auth,
        hostname,
        pathname,
        search,
        path,
        hash,
        query,
        port,
    })
}

pub fn url_get_protocol(url: &str) -> Option<String> {
    // C: sscanf(url, "%[^://]", protocol) — stops at first ':' or '/'
    let sep = url.find(|c: char| c == ':' || c == '/')?;
    let proto = &url[..sep];
    if url_is_protocol(proto) {
        Some(proto.to_string())
    } else {
        None
    }
}

pub fn url_get_auth(url: &str) -> Option<String> {
    let proto = url_get_protocol(url)?;
    let sep_len = if url[proto.len()..].starts_with("://") { 3 } else { 1 };
    let rest = &url[proto.len() + sep_len..];
    if let Some(at) = rest.find('@') {
        Some(rest[..at].to_string())
    } else {
        None
    }
}

pub fn url_get_hostname(url: &str) -> Option<String> {
    let proto = url_get_protocol(url)?;
    let is_ssh = url_is_ssh(&proto);
    let sep_len = if url[proto.len()..].starts_with("://") { 3 } else { 1 };
    let rest = &url[proto.len() + sep_len..];
    let after_auth = if let Some(at) = rest.find('@') {
        &rest[at + 1..]
    } else {
        rest
    };
    let end = if is_ssh {
        after_auth.find(':').unwrap_or(after_auth.len())
    } else {
        after_auth.find('/').unwrap_or(after_auth.len())
    };
    Some(after_auth[..end].to_string())
}

pub fn url_get_host(url: &str) -> Option<String> {
    let hn = url_get_hostname(url)?;
    if let Some(colon) = hn.find(':') {
        Some(hn[..colon].to_string())
    } else {
        Some(hn)
    }
}

pub fn url_get_path(url: &str) -> Option<String> {
    let proto = url_get_protocol(url)?;
    let is_ssh = url_is_ssh(&proto);
    let sep_len = if url[proto.len()..].starts_with("://") { 3 } else { 1 };
    let rest = &url[proto.len() + sep_len..];
    let after_auth = if let Some(at) = rest.find('@') {
        &rest[at + 1..]
    } else {
        rest
    };
    let hn_end = if is_ssh {
        after_auth.find(':').unwrap_or(after_auth.len())
    } else {
        after_auth.find('/').unwrap_or(after_auth.len())
    };
    let path_part = &after_auth[hn_end..];
    if is_ssh {
        // C: get_part with ":%s" then sprintf("%s", tmp_path) — strips leading ':'
        if path_part.len() > 1 {
            Some(path_part[1..].to_string())
        } else {
            Some(String::from("/"))
        }
    } else {
        if path_part.is_empty() {
            Some(String::from("/"))
        } else {
            Some(path_part.to_string())
        }
    }
}

pub fn url_get_pathname(url: &str) -> Option<String> {
    let path = url_get_path(url)?;
    let end = path.find(|c| c == '?' || c == '#' || c == ' ' || c == '|' || c == '^').unwrap_or(path.len());
    Some(path[..end].to_string())
}

pub fn url_get_search(url: &str) -> Option<String> {
    let path = url_get_path(url)?;
    let pn_end = path.find(|c| c == '?' || c == '#' || c == ' ' || c == '|' || c == '^').unwrap_or(path.len());
    let after = &path[pn_end..];
    let search_end = after.find('#').unwrap_or(after.len());
    Some(after[..search_end].to_string())
}

pub fn url_get_query(url: &str) -> Option<String> {
    let search = url_get_search(url)?;
    if let Some(q) = search.find('?') {
        Some(search[q + 1..].to_string())
    } else {
        Some(String::new())
    }
}

pub fn url_get_hash(url: &str) -> Option<String> {
    let path = url_get_path(url)?;
    let pn_end = path.find(|c| c == '?' || c == '#' || c == ' ' || c == '|' || c == '^').unwrap_or(path.len());
    let after = &path[pn_end..];
    if let Some(hash_pos) = after.find('#') {
        Some(after[hash_pos..].to_string())
    } else {
        Some(String::new())
    }
}

pub fn url_get_port(url: &str) -> Option<String> {
    let hn = url_get_hostname(url)?;
    if let Some(colon) = hn.find(':') {
        Some(hn[colon + 1..].to_string())
    } else {
        None
    }
}

pub fn url_inspect(url: &str) {
    if let Some(data) = url_parse(url) {
        url_data_inspect(&data);
    } else {
        println!("# Invalid URL");
    }
}

pub fn url_data_inspect(data: &UrlData) {
    println!("#url =>");
    println!(r#"    .href: "{}""#, data.href);
    println!(r#"    .protocol: "{}""#, data.protocol);
    println!(r#"    .host: "{}""#, data.host);
    println!(r#"    .auth: "{}""#, data.auth);
    println!(r#"    .hostname: "{}""#, data.hostname);
    println!(r#"    .pathname: "{}""#, data.pathname);
    println!(r#"    .search: "{}""#, data.search);
    println!(r#"    .path: "{}""#, data.path);
    println!(r#"    .hash: "{}""#, data.hash);
    println!(r#"    .query: "{}""#, data.query);
    println!(r#"    .port: "{}""#, data.port);
}

// url_free is no longer needed; Rust's ownership handles dropping.
pub fn url_free(_data: &UrlData) {
    // No-op: dropping is automatic.
}
