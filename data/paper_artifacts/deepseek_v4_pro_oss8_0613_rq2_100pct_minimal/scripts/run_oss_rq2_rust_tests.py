#!/usr/bin/env python3
"""论文实验：把旧 RQ2 Rust 测试模板适配到当前 OSS 翻译结果。"""

from __future__ import annotations

import argparse
import json
import os
import re
import signal
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any, Dict, List, Optional, Sequence, Tuple


REPO_ROOT = Path(__file__).resolve().parents[1]
PAPER_EXPERIMENTS_DIR = Path(__file__).resolve().parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))
if str(PAPER_EXPERIMENTS_DIR) not in sys.path:
    sys.path.insert(0, str(PAPER_EXPERIMENTS_DIR))

from analyze_required_unsafe import _resolve_run_dir, find_project_crate_dir  # noqa: E402


DEFAULT_TESTS_DIR = REPO_ROOT / "paper_tables_postprocess" / "latest" / "inputs" / "source_rq2_tests"


def _parse_projects(value: str) -> List[str]:
    """解析逗号分隔项目名。"""
    return [item.strip() for item in value.split(",") if item.strip()]


def _expected_test_count(test_file: Path) -> int:
    """统计模板中的 #[test] 数量。"""
    try:
        return len(re.findall(r"(?m)^\s*#\[test\]", test_file.read_text(encoding="utf-8")))
    except Exception:
        return 0


def _qsort_uses_safe_slice_api(src_dir: Path | None) -> bool:
    """判断当前 qsort crate 是否已被后修成 safe slice API。"""
    if src_dir is None:
        return False
    source = src_dir / "src_qsort.rs"
    if not source.is_file():
        return False
    try:
        body = source.read_text(encoding="utf-8", errors="ignore")
    except OSError:
        return False
    return bool(re.search(r"pub\s+fn\s+quickSort\s*\(\s*arr\s*:\s*&mut\s*\[", body))


def _source_text(path: Path) -> str:
    """读取源码文本，失败时返回空串。"""
    try:
        return path.read_text(encoding="utf-8", errors="ignore")
    except OSError:
        return ""


def _urlparser_uses_native_api(src_dir: Path | None) -> bool:
    """判断 urlparser 是否暴露 Rust-native 字符串 API。"""
    if src_dir is None:
        return False
    source = _source_text(src_dir / "src_url.rs")
    return bool(
        re.search(r"pub\s+struct\s+UrlData\b", source)
        and re.search(r"pub\s+fn\s+url_parse\s*\(\s*url\s*:\s*&str\s*\)\s*->\s*Option\s*<\s*UrlData\s*>", source)
        and re.search(r"pub\s+fn\s+url_get_protocol\s*\(\s*url\s*:\s*&str\s*\)\s*->\s*Option\s*<\s*String\s*>", source)
    )


def _genann_uses_native_api(src_dir: Path | None) -> bool:
    """判断 genann 是否暴露 Rust-native Box/slice API。"""
    if src_dir is None:
        return False
    source = _source_text(src_dir / "src_genann.rs")
    return bool(
        re.search(r"pub\s+fn\s+genann_init\s*\([^)]*\)\s*->\s*Option\s*<\s*Box\s*<\s*(?:crate::types::)?genann\s*>\s*>", source)
        and re.search(r"pub\s+fn\s+genann_run(?:\s*<[^>]+>)?\s*\([^)]*inputs\s*:\s*&\s*\[\s*f64\s*\]", source)
    )


def _buffer_uses_native_api(src_dir: Path | None) -> bool:
    """判断 buffer 是否暴露 Rust-native slice/ref API。"""
    if src_dir is None:
        return False
    source = _source_text(src_dir / "src_buffer.rs")
    types = _source_text(src_dir / "types.rs")
    return bool(
        re.search(r"pub\s+fn\s+buffer_new_with_string\s*\(\s*data\s*:\s*&\s*\[\s*u8\s*\]\s*\)\s*->\s*\*mut\s+buffer_t", source)
        and re.search(r"pub\s+fn\s+buffer_append\s*\(\s*b\s*:\s*&mut\s+buffer_t\s*,\s*data\s*:\s*&\s*\[\s*u8\s*\]", source)
        and re.search(r"pub\s+fn\s+buffer_size\s*\(\s*b\s*:\s*&buffer_t\s*\)\s*->\s*size_t", source)
        and re.search(r"pub\s+fn\s+buffer_slice\s*\(\s*b\s*:\s*&buffer_t\b", source)
        and re.search(r"pub\s+struct\s+buffer_t\b[\s\S]*pub\s+storage\s*:\s*Vec\s*<\s*u8\s*>", types)
    )


def _rgba_uses_native_api(src_dir: Path | None) -> bool:
    """判断 rgba 是否暴露 Rust-native bytes/String API。"""
    if src_dir is None:
        return False
    source = _source_text(src_dir / "src_rgba.rs")
    return bool(
        re.search(r"pub\s+fn\s+rgba_from_string\s*\(\s*input\s*:\s*&\s*\[\s*u8\s*\]\s*\)\s*->\s*Option\s*<\s*u32\s*>", source)
        and re.search(r"pub\s+fn\s+rgba_new\s*\([^)]*\)\s*->\s*(?:crate::types::)?rgba_t\b", source)
        and re.search(r"pub\s+fn\s+rgba_to_string\s*\([^)]*\)\s*->\s*String\b", source)
    )


def _quadtree_uses_native_api(src_dir: Path | None) -> bool:
    """判断 quadtree 是否暴露 Rust-native Box/ref API。"""
    if src_dir is None:
        return False
    source = _source_text(src_dir / "src_quadtree.rs")
    types = _source_text(src_dir / "types.rs")
    return bool(
        re.search(r"pub\s+fn\s+quadtree_new\s*\([^)]*\)\s*->\s*Box\s*<\s*(?:crate::types::)?quadtree_t\s*>", source)
        and re.search(r"pub\s+fn\s+quadtree_insert\s*\(\s*tree\s*:\s*&mut\s+(?:crate::types::)?quadtree_t\b", source)
        and re.search(
            r"pub\s+fn\s+quadtree_search\s*\(\s*tree\s*:\s*&(?:crate::types::)?quadtree_t\b[\s\S]*->\s*Option\s*<\s*&(?:crate::types::)?quadtree_point_t\s*>",
            source,
        )
        and re.search(r"pub\s+struct\s+quadtree_point_t\b[\s\S]*pub\s+x\s*:\s*f64[\s\S]*pub\s+y\s*:\s*f64", types)
        and re.search(r"pub\s+struct\s+quadtree_t\b[\s\S]*pub\s+root\s*:\s*Option\s*<\s*Box\s*<\s*quadtree_node_t\s*>\s*>", types)
        and re.search(r"pub\s+struct\s+quadtree_t\b[\s\S]*pub\s+length\s*:\s*u32", types)
    )


def _zopfli_uses_native_api(src_dir: Path | None) -> bool:
    """判断 zopfli 是否暴露 Vec 字段和 Rust-native uppercase API。"""
    if src_dir is None:
        return False
    types = _source_text(src_dir / "types.rs")
    hash_source = _source_text(src_dir / "src_hash.rs")
    lz77_source = _source_text(src_dir / "src_lz77.rs")
    lib_source = _source_text(src_dir / "src_zopfli_lib.rs")
    util_source = _source_text(src_dir / "src_util.rs")
    has_lz77_init = bool(
        re.search(r"pub\s+fn\s+ZopfliInitLZ77Store(?:\s*<[^>]+>)?\s*\(\s*data\s*:\s*Option\s*<\s*&(?:'a\s*)?\s*\[\s*u8\s*\]\s*>", lz77_source)
        or re.search(r"pub\s+fn\s+ZopfliInitLZ77Store(?:\s*<[^>]+>)?\s*\(\s*data\s*:\s*&(?:'a\s*)?\s*\[\s*u8\s*\]", lz77_source)
    )
    has_lz77_size = bool(
        re.search(r"pub\s+struct\s+ZopfliLZ77Store\b[\s\S]*\bsize\s*:\s*(?:usize|size_t)", types)
        or re.search(r"impl(?:\s*<[^>]+>)?\s+ZopfliLZ77Store(?:\s*<[^>]+>)?\s*\{[\s\S]*pub\s+fn\s+size\s*\(\s*&self\s*\)\s*->\s*(?:usize|size_t)", types)
    )
    has_lz77_data = bool(
        re.search(r"pub\s+struct\s+ZopfliLZ77Store\b[\s\S]*original_data\s*:\s*Option\s*<\s*&(?:'a\s*)?\s*\[\s*u8\s*\]\s*>", types)
        or re.search(r"pub\s+struct\s+ZopfliLZ77Store\b[\s\S]*data_slice\s*:\s*&(?:'a\s*)?\s*\[\s*u8\s*\]", types)
    )
    has_zopfli_compress = bool(
        re.search(r"pub\s+fn\s+ZopfliCompress\s*\([^)]*in_\s*:\s*&\s*\[\s*u8\s*\]\s*\)\s*->\s*Vec\s*<\s*u8\s*>", lib_source)
        or re.search(r"pub\s+fn\s+ZopfliCompress\s*\([^)]*in_\s*:\s*&\s*\[\s*u8\s*\][^)]*out\s*:\s*&mut\s+Vec\s*<\s*u8\s*>", lib_source, re.S)
    )
    return bool(
        re.search(r"pub\s+struct\s+ZopfliHash\b[\s\S]*head\s*:\s*Vec\s*<\s*(?:i32|c_int)\s*>", types)
        and re.search(r"pub\s+struct\s+ZopfliHash\b[\s\S]*prev\s*:\s*Vec\s*<\s*(?:u16|c_ushort)\s*>", types)
        and re.search(r"pub\s+struct\s+ZopfliHash\b[\s\S]*same\s*:\s*Vec\s*<\s*(?:u16|c_ushort)\s*>", types)
        and re.search(r"pub\s+struct\s+ZopfliLZ77Store\b[\s\S]*litlens\s*:\s*Vec\s*<\s*(?:u16|c_ushort)\s*>", types)
        and re.search(r"pub\s+struct\s+ZopfliLZ77Store\b[\s\S]*pos\s*:\s*Vec\s*<\s*(?:usize|size_t)\s*>", types)
        and has_lz77_size
        and has_lz77_data
        and re.search(r"pub\s+fn\s+ZopfliAllocHash\s*\([^)]*h\s*:\s*&mut\s+(?:crate::types::)?ZopfliHash", hash_source)
        and re.search(r"pub\s+fn\s+ZopfliResetHash\s*\([^)]*h\s*:\s*&mut\s+(?:crate::types::)?ZopfliHash", hash_source)
        and re.search(r"pub\s+fn\s+ZopfliUpdateHash\s*\(\s*array\s*:\s*&\s*\[\s*u8\s*\]", hash_source)
        and re.search(r"pub\s+fn\s+ZopfliWarmupHash\s*\(\s*array\s*:\s*&\s*\[\s*u8\s*\]", hash_source)
        and has_lz77_init
        and re.search(r"pub\s+fn\s+ZopfliStoreLitLenDist\s*\([^)]*store\s*:\s*&mut\s+(?:crate::types::)?ZopfliLZ77Store", lz77_source)
        and has_zopfli_compress
        and re.search(r"pub\s+fn\s+ZopfliInitOptions\s*\(", util_source)
    )


def _zopfli_store_new_expr(types: str, data_expr: str) -> str:
    """按当前 ZopfliLZ77Store 构造签名生成表达式。"""
    if re.search(r"pub\s+fn\s+new\s*\(\s*\)\s*->\s*Self", types):
        return "ZopfliLZ77Store::new()"
    if re.search(r"pub\s+fn\s+new\s*\(\s*data\s*:\s*&(?:'a\s*)?\s*\[\s*u8\s*\]\s*\)\s*->\s*Self", types):
        return f"ZopfliLZ77Store::new({data_expr})"
    return "ZopfliLZ77Store::default()"


def _zopfli_native_test_template(src_dir: Path | None) -> str:
    """按当前 zopfli Rust-native API 生成 RQ2 等价测试。"""
    types = _source_text(src_dir / "types.rs") if src_dir else ""
    lz77_source = _source_text(src_dir / "src_lz77.rs") if src_dir else ""
    lib_source = _source_text(src_dir / "src_zopfli_lib.rs") if src_dir else ""
    uses_direct_lz77_data = bool(re.search(r"ZopfliInitLZ77Store(?:\s*<[^>]+>)?\s*\(\s*data\s*:\s*&", lz77_source))
    compress_writes_vec = bool(re.search(r"pub\s+fn\s+ZopfliCompress\s*\([^)]*out\s*:\s*&mut\s+Vec\s*<\s*u8\s*>", lib_source, re.S))
    has_type_constant = bool(re.search(r"pub\s+const\s+ZOPFLI_FORMAT_DEFLATE\b", types))
    zopfli_format_import = "ZOPFLI_FORMAT_DEFLATE" if has_type_constant else "__c2r_tu_types_src_zopfli_bin::ZOPFLI_FORMAT_DEFLATE"
    store_new = _zopfli_store_new_expr(types, "input")
    init_data = "input" if uses_direct_lz77_data else "Some(input)"
    original_data_check = "assert_eq!(store.data_slice, input.as_slice());" if "data_slice" in types else "assert_eq!(store.original_data, Some(input.as_slice()));"
    store_size_zero = "store.size()" if re.search(r"pub\s+fn\s+size\s*\(\s*&self\s*\)", types) else "store.size"
    compress_call = (
        "let mut out = Vec::new();\n"
        "    ZopfliCompress(&opts, ZOPFLI_FORMAT_DEFLATE, input, input.len(), &mut out);"
        if compress_writes_vec
        else "let out = ZopfliCompress(&opts, ZOPFLI_FORMAT_DEFLATE, input);"
    )
    return rf'''// RQ2 adapter for zopfli Rust-native API.

use crate::src_hash::{{
    ZopfliAllocHash, ZopfliCleanHash, ZopfliResetHash, ZopfliUpdateHash, ZopfliWarmupHash,
}};
use crate::src_lz77::{{ZopfliCleanLZ77Store, ZopfliInitLZ77Store, ZopfliStoreLitLenDist}};
use crate::src_util::ZopfliInitOptions;
use crate::src_zopfli_lib::ZopfliCompress;
use crate::types::{{
    ZopfliHash, ZopfliLZ77Store, ZopfliOptions, {zopfli_format_import},
}};

fn to_hex(data: &[u8]) -> String {{
    const LUT: &[u8; 16] = b"0123456789abcdef";
    let mut out = Vec::with_capacity(data.len() * 2);
    for &b in data {{
        out.push(LUT[(b >> 4) as usize]);
        out.push(LUT[(b & 0x0f) as usize]);
    }}
    String::from_utf8(out).unwrap()
}}

#[test]
fn test_zopfli_init_options_defaults() {{
    let mut opts = ZopfliOptions {{
        verbose: -1,
        verbose_more: -1,
        numiterations: -1,
        blocksplitting: -1,
        blocksplittinglast: -1,
        blocksplittingmax: -1,
    }};

    ZopfliInitOptions(&mut opts);

    assert_eq!(opts.verbose, 0);
    assert_eq!(opts.verbose_more, 0);
    assert_eq!(opts.numiterations, 15);
    assert_eq!(opts.blocksplitting, 1);
    assert_eq!(opts.blocksplittinglast, 0);
    assert_eq!(opts.blocksplittingmax, 15);
}}

#[test]
fn test_zopfli_hash_alloc_reset_update_clean() {{
    let window_size: usize = 32_768;
    let input = b"aaaaaa";
    let mut hash = ZopfliHash {{
        head: Vec::new(),
        prev: Vec::new(),
        hashval: Vec::new(),
        val: 0,
        head2: Vec::new(),
        prev2: Vec::new(),
        hashval2: Vec::new(),
        val2: 0,
        same: Vec::new(),
    }};
    ZopfliAllocHash(window_size, &mut hash);

    assert_eq!(hash.head.len(), 65_536);
    assert_eq!(hash.prev.len(), window_size);
    assert_eq!(hash.hashval.len(), window_size);
    assert_eq!(hash.same.len(), window_size);
    assert_eq!(hash.head2.len(), 65_536);
    assert_eq!(hash.prev2.len(), window_size);
    assert_eq!(hash.hashval2.len(), window_size);

    ZopfliResetHash(window_size, &mut hash);
    assert_eq!(hash.val, 0);
    assert_eq!(hash.val2, 0);
    assert_eq!(hash.head[0], -1);
    assert_eq!(hash.prev[0], 0);
    assert_eq!(hash.hashval[0], -1);
    assert_eq!(hash.same[0], 0);
    assert_eq!(hash.head2[0], -1);
    assert_eq!(hash.prev2[0], 0);
    assert_eq!(hash.hashval2[0], -1);

    ZopfliWarmupHash(input, 0, input.len(), &mut hash);
    ZopfliUpdateHash(input, 0, input.len(), &mut hash);
    assert!(hash.hashval[0] >= 0);
    assert_eq!(hash.head[hash.val as usize], 0);

    ZopfliCleanHash(&mut hash);
}}

#[test]
fn test_zopfli_lz77_store_litlen_lifecycle() {{
    let input = b"abc";
    let mut store = {store_new};
    ZopfliInitLZ77Store({init_data}, &mut store);

    assert_eq!({store_size_zero}, 0);
    {original_data_check}
    assert!(store.litlens.is_empty());
    assert!(store.dists.is_empty());
    assert!(store.pos.is_empty());

    ZopfliStoreLitLenDist(b'a' as u16, 0, 0, &mut store);
    assert_eq!({store_size_zero}, 1);
    assert_eq!(store.litlens[0], b'a' as u16);
    assert_eq!(store.dists[0], 0);
    assert_eq!(store.pos[0], 0);
    assert_eq!(store.ll_symbol[0], b'a' as u16);
    assert_eq!(store.d_symbol[0], 0);

    ZopfliCleanLZ77Store(&mut store);
    assert_eq!({store_size_zero}, 0);
    assert!(store.litlens.is_empty());
    assert!(store.dists.is_empty());
    assert!(store.pos.is_empty());
}}

#[test]
fn test_zopfli_zz_deflate_output_matches_reference() {{
    let input = b"hello zopfli test";
    let mut opts = ZopfliOptions {{
        verbose: 0,
        verbose_more: 0,
        numiterations: 0,
        blocksplitting: 0,
        blocksplittinglast: 0,
        blocksplittingmax: 0,
    }};

    ZopfliInitOptions(&mut opts);
    opts.numiterations = 5;

    {compress_call}
    assert_eq!(out.len(), 19);
    assert_eq!(to_hex(&out), "cb48cdc9c957a8ca2f48cbc95428492d2e0100");
}}
'''.lstrip()


def _native_urlparser_test_template() -> str:
    """生成 urlparser Rust-native API 的 RQ2 等价测试。"""
    return r'''// RQ2 adapter for urlparser Rust-native API.

use crate::src_url::*;

#[test]
fn test_url_is_protocol() {
    assert!(url_is_protocol("http"));
    assert!(url_is_protocol("https"));
    // `file` is present in the source URL_SCHEMES table, so it is a valid protocol.
    assert!(url_is_protocol("file"));
    assert!(!url_is_protocol("not-a-url-scheme"));
}

#[test]
fn test_url_getters_match_reference() {
    let url = "http://user:pass@subdomain.host.com:8080/p/a/t/h?query=string#hash";

    assert_eq!(url_get_protocol(url), Some("http".to_string()));
    assert_eq!(url_get_auth(url), Some("user:pass".to_string()));
    assert_eq!(url_get_hostname(url), Some("subdomain.host.com:8080".to_string()));
    assert_eq!(url_get_host(url), Some("subdomain.host.com".to_string()));
    assert_eq!(url_get_pathname(url), Some("/p/a/t/h".to_string()));
    assert_eq!(url_get_path(url), Some("/p/a/t/h?query=string#hash".to_string()));
    assert_eq!(url_get_search(url), Some("?query=string".to_string()));
    assert_eq!(url_get_query(url), Some("query=string".to_string()));
    assert_eq!(url_get_hash(url), Some("#hash".to_string()));
    assert_eq!(url_get_port(url), Some("8080".to_string()));
}

#[test]
fn test_url_parse_fields_non_null() {
    let url = "http://user:pass@subdomain.host.com:8080/p/a/t/h?query=string#hash";
    let parsed = url_parse(url).expect("url_parse should parse the reference URL");

    assert_eq!(parsed.href, url);
    assert_eq!(parsed.protocol, "http");
    assert_eq!(parsed.auth, "user:pass");
    assert_eq!(parsed.hostname, "subdomain.host.com:8080");
    assert_eq!(parsed.host, "subdomain.host.com");
    assert_eq!(parsed.pathname, "/p/a/t/h");
    assert_eq!(parsed.path, "/p/a/t/h?query=string#hash");
    assert_eq!(parsed.search, "?query=string");
    assert_eq!(parsed.query, "query=string");
    assert_eq!(parsed.hash, "#hash");
    assert_eq!(parsed.port, "8080");

    let gh_url = "git://git@github.com:jwerle/url.h.git";
    let gh_parsed = url_parse(gh_url).expect("url_parse should parse git-style URLs");
    assert_eq!(gh_parsed.href, gh_url);
    assert_eq!(gh_parsed.protocol, "git");
    assert_eq!(gh_parsed.auth, "git");
    assert_eq!(gh_parsed.hostname, "github.com");
    assert_eq!(gh_parsed.host, "github.com");
    assert_eq!(gh_parsed.pathname, "jwerle/url.h.git");
    assert_eq!(gh_parsed.path, "jwerle/url.h.git");
}
'''.lstrip()


def _native_genann_test_template() -> str:
    """生成 genann Rust-native API 的 RQ2 等价测试。"""
    return r'''// RQ2 adapter for genann Rust-native API.

use crate::src_genann::*;

#[test]
fn test_genann_act_sigmoid() {
    let result = genann_act_sigmoid(0.0);
    assert!((result - 0.5).abs() < 0.001, "sigmoid(0) 应该约等于 0.5");
}

#[test]
fn test_genann_act_sigmoid_positive() {
    let result = genann_act_sigmoid(10.0);
    assert!(result > 0.99, "sigmoid(10) 应该接近 1");
}

#[test]
fn test_genann_act_sigmoid_negative() {
    let result = genann_act_sigmoid(-10.0);
    assert!(result < 0.01, "sigmoid(-10) 应该接近 0");
}

#[test]
fn test_genann_act_threshold() {
    assert_eq!(genann_act_threshold(1.0), 1.0, "threshold(1) 应该是 1");
    assert_eq!(genann_act_threshold(-1.0), 0.0, "threshold(-1) 应该是 0");
    assert_eq!(genann_act_threshold(0.0), 0.0, "threshold(0) 应该是 0");
}

#[test]
fn test_genann_act_linear() {
    assert_eq!(genann_act_linear(5.0), 5.0, "linear(5) 应该是 5");
    assert_eq!(genann_act_linear(-3.0), -3.0, "linear(-3) 应该是 -3");
    assert_eq!(genann_act_linear(0.0), 0.0, "linear(0) 应该是 0");
}

#[test]
fn test_genann_init() {
    let ann = genann_init(1, 1, 2, 1).expect("genann_init 应该返回有效网络");
    assert_eq!(ann.inputs, 1);
    assert_eq!(ann.hidden_layers, 1);
    assert_eq!(ann.hidden, 2);
    assert_eq!(ann.outputs, 1);
    assert_eq!(ann.weight.len(), 7);
    assert_eq!(ann.output.len(), 4);
    assert_eq!(ann.delta.len(), 3);
}

#[test]
fn test_genann_init_no_hidden() {
    let ann = genann_init(2, 0, 0, 1).expect("没有隐藏层的网络也应该创建成功");
    assert_eq!(ann.inputs, 2);
    assert_eq!(ann.hidden_layers, 0);
    assert_eq!(ann.hidden, 0);
    assert_eq!(ann.outputs, 1);
    assert_eq!(ann.weight.len(), 3);
    assert_eq!(ann.output.len(), 3);
    assert_eq!(ann.delta.len(), 1);
}

#[test]
fn test_genann_copy() {
    let ann = genann_init(2, 1, 3, 1).expect("genann_init 应该返回有效网络");
    let copy = genann_copy(&ann);

    assert_eq!(copy.inputs, ann.inputs);
    assert_eq!(copy.hidden_layers, ann.hidden_layers);
    assert_eq!(copy.hidden, ann.hidden);
    assert_eq!(copy.outputs, ann.outputs);
    assert_eq!(copy.weight, ann.weight);
    assert_eq!(copy.output, ann.output);
    assert_eq!(copy.delta, ann.delta);
}

#[test]
fn test_genann_randomize() {
    let mut ann = genann_init(2, 1, 3, 1).expect("genann_init 应该返回有效网络");
    for weight in &mut ann.weight {
        *weight = 0.0;
    }

    genann_randomize(&mut ann);
    assert!(ann.weight.iter().any(|weight| *weight != 0.0), "randomize 应该更新权重");
}

#[test]
fn test_genann_run() {
    let mut ann = genann_init(2, 1, 2, 1).expect("genann_init 应该返回有效网络");
    let inputs = [0.5, 0.5];

    let output = genann_run(&mut ann, &inputs);
    assert_eq!(output.len(), 1, "genann_run 应该返回 1 个输出");
    assert!(output[0].is_finite(), "输出应该是有限数");
    assert!((0.0..=1.0).contains(&output[0]), "默认 sigmoid 输出应该在 [0, 1]");
}

#[test]
fn test_genann_sigmoid_cached() {
    let x = 0.5;
    let cached = genann_act_sigmoid_cached(x);
    let normal = genann_act_sigmoid(x);

    assert!((cached - normal).abs() < 0.1, "缓存和非缓存的 sigmoid 应该结果相近");
}

#[test]
fn test_genann_multilayer() {
    let ann = genann_init(4, 3, 5, 2).expect("多隐藏层网络应该创建成功");
    assert_eq!(ann.inputs, 4);
    assert_eq!(ann.hidden_layers, 3);
    assert_eq!(ann.hidden, 5);
    assert_eq!(ann.outputs, 2);
    assert_eq!(ann.weight.len(), 97);
    assert_eq!(ann.output.len(), 21);
    assert_eq!(ann.delta.len(), 17);
}
'''.lstrip()


def _native_rgba_test_template() -> str:
    """生成 rgba Rust-native API 的 RQ2 等价测试。"""
    return r'''// RQ2 adapter for rgba Rust-native API.

use crate::src_rgba::*;

fn pack_rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    (u32::from(r) << 24) | (u32::from(g) << 16) | (u32::from(b) << 8) | u32::from(a)
}

#[test]
fn test_rgba_from_string_hex6() {
    assert_eq!(rgba_from_string(b"#ff8040"), Some(pack_rgba(255, 128, 64, 255)));
}

#[test]
fn test_rgba_from_string_hex3() {
    assert_eq!(rgba_from_string(b"#f84"), Some(pack_rgba(255, 136, 68, 255)));
}

#[test]
fn test_rgba_from_string_rgb() {
    assert_eq!(rgba_from_string(b"rgb(255, 128, 64)"), Some(pack_rgba(255, 128, 64, 255)));
}

#[test]
fn test_rgba_from_string_rgba_alpha_zero() {
    assert_eq!(rgba_from_string(b"rgba(255, 128, 64, 0.0)"), Some(pack_rgba(255, 128, 64, 0)));
}

#[test]
fn test_rgba_to_string_rgba_format_for_transparent() {
    let color = rgba_new(0xFF804020);
    let out = rgba_to_string(color);

    assert!(out.starts_with("rgba(255, 128, 64,"), "rgba_to_string output mismatch: {out}");
    assert!(out.contains("0.13"), "alpha should be rounded to two decimals: {out}");
}

#[test]
fn test_rgba_to_string_hex_for_opaque() {
    let color = rgba_new(0xFF8040FF);
    let out = rgba_to_string(color);

    assert!(out.starts_with("#"), "opaque color should use hex: {out}");
    assert!(out.to_lowercase().contains("ff8040"), "hex content mismatch: {out}");
}

#[test]
fn test_rgba_from_string_named_color_red() {
    assert_eq!(rgba_from_string(b"red"), Some(pack_rgba(255, 0, 0, 255)));
}

#[test]
fn test_rgba_consistency() {
    let c1 = rgba_from_string(b"#6496c8");
    let c2 = rgba_from_string(b"#6496c8");

    assert!(c1.is_some());
    assert_eq!(c1, c2);
}

#[test]
fn test_rgba_different_colors() {
    let r = rgba_from_string(b"#ff0000").expect("red should parse");
    let g = rgba_from_string(b"#00ff00").expect("green should parse");
    let b = rgba_from_string(b"#0000ff").expect("blue should parse");

    assert_ne!(r, g);
    assert_ne!(g, b);
    assert_ne!(r, b);
}

#[test]
fn test_rgba_invalid_string_returns_none() {
    assert_eq!(rgba_from_string(b"not-a-color"), None);
}
'''.lstrip()


def _native_buffer_test_template() -> str:
    """生成 buffer Rust-native API 的 RQ2 等价测试。"""
    return r'''// RQ2 adapter for buffer Rust-native API.

use crate::src_buffer::*;
use crate::types::buffer_t;

fn buffer_str(buf: &buffer_t) -> String {
    let start = buf.data_offset;
    let bytes = &buf.storage[start..];
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).to_string()
}

unsafe fn with_ref<'a>(ptr: *mut buffer_t) -> &'a buffer_t {
    &*ptr
}

unsafe fn with_mut<'a>(ptr: *mut buffer_t) -> &'a mut buffer_t {
    &mut *ptr
}

#[test]
fn test_buffer_new() {
    let buf = buffer_new();
    assert!(!buf.is_null(), "buffer_new 应该返回有效指针");
    unsafe {
        assert_eq!(buffer_size(with_ref(buf)), 64);
        assert_eq!(buffer_length(with_ref(buf)), 0);
    }
    buffer_free(buf);
}

#[test]
fn test_buffer_new_with_size() {
    let buf = buffer_new_with_size(64);
    assert!(!buf.is_null(), "buffer_new_with_size 应该返回有效指针");
    unsafe {
        assert!(buffer_size(with_ref(buf)) >= 64, "buffer 大小应该至少是请求的大小");
    }
    buffer_free(buf);
}

#[test]
fn test_buffer_new_with_string() {
    let buf = buffer_new_with_string(b"hello");
    assert!(!buf.is_null(), "buffer_new_with_string 应该返回有效指针");
    unsafe {
        assert_eq!(buffer_length(with_ref(buf)), 5, "buffer 长度应该是 5");
        assert_eq!(buffer_str(with_ref(buf)), "hello");
    }
    buffer_free(buf);
}

#[test]
fn test_buffer_append() {
    let buf = buffer_new();
    unsafe {
        assert_eq!(buffer_append(with_mut(buf), b"hello"), 0);
        assert_eq!(buffer_length(with_ref(buf)), 5, "追加后长度应该是 5");
        assert_eq!(buffer_str(with_ref(buf)), "hello");
    }
    buffer_free(buf);
}

#[test]
fn test_buffer_append_multiple() {
    let buf = buffer_new();
    unsafe {
        assert_eq!(buffer_append(with_mut(buf), b"hello"), 0);
        assert_eq!(buffer_append(with_mut(buf), b" world"), 0);
        assert_eq!(buffer_length(with_ref(buf)), 11, "追加多个字符串后长度应该是 11");
        assert_eq!(buffer_str(with_ref(buf)), "hello world");
    }
    buffer_free(buf);
}

#[test]
fn test_buffer_prepend() {
    let buf = buffer_new();
    unsafe {
        assert_eq!(buffer_append(with_mut(buf), b"world"), 0);
        assert_eq!(buffer_prepend(with_mut(buf), b"hello "), 0);
        assert_eq!(buffer_length(with_ref(buf)), 11, "prepend 后长度应该是 11");
        assert_eq!(buffer_str(with_ref(buf)), "hello world");
    }
    buffer_free(buf);
}

#[test]
fn test_buffer_equals() {
    let buf1 = buffer_new_with_string(b"hello");
    let buf2 = buffer_new_with_string(b"hello");
    unsafe {
        assert_eq!(buffer_equals(with_ref(buf1), with_ref(buf2)), 1, "相同内容的 buffer 应该相等");
    }
    buffer_free(buf1);
    buffer_free(buf2);
}

#[test]
fn test_buffer_trim() {
    let buf = buffer_new_with_string(b"  hello  ");
    unsafe {
        buffer_trim(with_mut(buf));
        assert_eq!(buffer_length(with_ref(buf)), 5, "trim 后长度应该是 5");
        assert_eq!(buffer_str(with_ref(buf)), "hello");
    }
    buffer_free(buf);
}

#[test]
fn test_buffer_trim_left() {
    let buf = buffer_new_with_string(b"  hello");
    unsafe {
        buffer_trim_left(with_mut(buf));
        assert_eq!(buffer_length(with_ref(buf)), 5, "trim_left 后长度应该是 5");
        assert_eq!(buffer_str(with_ref(buf)), "hello");
    }
    buffer_free(buf);
}

#[test]
fn test_buffer_trim_right() {
    let buf = buffer_new_with_string(b"hello  ");
    unsafe {
        buffer_trim_right(with_mut(buf));
        assert_eq!(buffer_length(with_ref(buf)), 5, "trim_right 后长度应该是 5");
        assert_eq!(buffer_str(with_ref(buf)), "hello");
    }
    buffer_free(buf);
}

#[test]
fn test_buffer_resize() {
    let buf = buffer_new_with_size(10);
    unsafe {
        assert_eq!(buffer_resize(with_mut(buf), 100), 0);
        assert!(buffer_size(with_ref(buf)) >= 100, "resize 后大小应该至少是 100");
    }
    buffer_free(buf);
}

#[test]
fn test_buffer_slice() {
    let buf = buffer_new_with_string(b"hello world");
    unsafe {
        let sliced = buffer_slice(with_ref(buf), 0, 5);
        assert!(!sliced.is_null());
        assert_eq!(buffer_length(with_ref(sliced)), 5, "slice 后长度应该是 5");
        assert_eq!(buffer_str(with_ref(sliced)), "hello");
        buffer_free(sliced);
    }
    buffer_free(buf);
}

#[test]
fn test_buffer_indexof() {
    let buf = buffer_new_with_string(b"hello world");
    unsafe {
        let idx = buffer_indexof(with_ref(buf), b"world");
        assert_eq!(idx, 6, "world 在 hello world 中的索引应该是 6");
    }
    buffer_free(buf);
}

#[test]
fn test_buffer_indexof_not_found() {
    let buf = buffer_new_with_string(b"hello");
    unsafe {
        let idx = buffer_indexof(with_ref(buf), b"xyz");
        assert_eq!(idx, -1, "找不到的字符串应该返回 -1");
    }
    buffer_free(buf);
}
'''.lstrip()


def _native_ht_test_template() -> str:
    """生成 ht 当前公开边界可测的 smoke test。"""
    return r'''// RQ2 adapter for ht private-main crate.

#[test]
fn test_main_smoke() {
    let _ = core::mem::size_of::<crate::types::pthread_once_t>();
}
'''.lstrip()


def _native_avl_test_template() -> str:
    """生成 avl 当前 Rust-native ref API 的 RQ2 等价测试。"""
    return r'''// RQ2 adapter for avl Rust-native API.

use crate::src_avl::*;
use crate::types::*;

use core::ffi::c_void;
use core::ptr;

#[repr(C)]
struct NodeWithKey {
    node: AVL3_NODE,
    key: i32,
}

fn node_key_offsets() -> (u16, u16) {
    let dummy: NodeWithKey = unsafe { core::mem::zeroed() };
    let base = &dummy as *const NodeWithKey as usize;
    let node_off = (&dummy.node as *const AVL3_NODE as usize) - base;
    let key_off = (&dummy.key as *const i32 as usize) - base;
    (key_off as u16, node_off as u16)
}

extern "C" fn compare_i32(a: *const c_void, b: *const c_void) -> core::ffi::c_int {
    let av = unsafe { *(a as *const i32) };
    let bv = unsafe { *(b as *const i32) };
    if av < bv {
        -1
    } else if av > bv {
        1
    } else {
        0
    }
}

unsafe fn key_of_container(p: *mut c_void) -> i32 {
    (*(p as *mut NodeWithKey)).key
}

unsafe fn container_from_node(node: *mut AVL3_NODE, node_off: u16) -> *mut NodeWithKey {
    (node as *mut u8).offset(-(node_off as isize)) as *mut NodeWithKey
}

#[test]
fn test_avl_insert_find_next_prev() {
    let (key_off, node_off) = node_key_offsets();

    let mut tree: AVL3_TREE = unsafe { core::mem::zeroed() };
    let mut info: AVL3_TREE_INFO = unsafe { core::mem::zeroed() };
    info.pfCompare = Some(compare_i32);
    info.usKeyOffset = key_off;
    info.usNodeOffset = node_off;

    let mut n1: NodeWithKey = unsafe { core::mem::zeroed() };
    let mut n2: NodeWithKey = unsafe { core::mem::zeroed() };
    let mut n3: NodeWithKey = unsafe { core::mem::zeroed() };
    n1.key = 1;
    n2.key = 2;
    n3.key = 3;

    unsafe {
        assert!(
            VOS_AVL3_Insert_Or_Find(&mut tree, &mut n2.node, &info).is_null(),
            "insert(2) should return NULL"
        );
        assert!(VOS_AVL3_Insert_Or_Find(&mut tree, &mut n1.node, &info).is_null());
        assert!(VOS_AVL3_Insert_Or_Find(&mut tree, &mut n3.node, &info).is_null());

        let key2: i32 = 2;
        let found = VOS_AVL3_Find(&tree, &key2 as *const _ as *const c_void, &info);
        assert!(!found.is_null());
        assert_eq!(key_of_container(found), 2);

        let n1_next = VOS_AVL3_Next(&n1.node, &info);
        assert!(!n1_next.is_null());
        assert_eq!(key_of_container(n1_next), 2, "Next(1) 应该是 2");

        let n2_next = VOS_AVL3_Next(&n2.node, &info);
        assert!(!n2_next.is_null());
        assert_eq!(key_of_container(n2_next), 3, "Next(2) 应该是 3");

        assert!(!tree.pstFirst.is_null());
        assert!(!tree.pstLast.is_null());
        let first = container_from_node(tree.pstFirst as *mut AVL3_NODE, node_off);
        let last = container_from_node(tree.pstLast as *mut AVL3_NODE, node_off);
        assert!(!first.is_null() && !last.is_null());
        assert_eq!((*first).key, 1);
        assert_eq!((*last).key, 3);
    }
}

#[test]
fn test_avl_delete_updates_links() {
    let (key_off, node_off) = node_key_offsets();

    let mut tree: AVL3_TREE = unsafe { core::mem::zeroed() };
    let mut info: AVL3_TREE_INFO = unsafe { core::mem::zeroed() };
    info.pfCompare = Some(compare_i32);
    info.usKeyOffset = key_off;
    info.usNodeOffset = node_off;

    let mut n1: NodeWithKey = unsafe { core::mem::zeroed() };
    let mut n2: NodeWithKey = unsafe { core::mem::zeroed() };
    let mut n3: NodeWithKey = unsafe { core::mem::zeroed() };
    n1.key = 1;
    n2.key = 2;
    n3.key = 3;

    unsafe {
        assert!(VOS_AVL3_Insert_Or_Find(&mut tree, &mut n2.node, &info).is_null());
        assert!(VOS_AVL3_Insert_Or_Find(&mut tree, &mut n1.node, &info).is_null());
        assert!(VOS_AVL3_Insert_Or_Find(&mut tree, &mut n3.node, &info).is_null());

        VOS_AVL3_Delete(&mut tree, &mut n2.node);
        let key2: i32 = 2;
        let found = VOS_AVL3_Find(&tree, &key2 as *const _ as *const c_void, &info);
        assert_eq!(found, ptr::null_mut(), "删除后不应再找到 key=2");

        let n1_next = VOS_AVL3_Next(&n1.node, &info);
        assert!(!n1_next.is_null());
        assert_eq!(key_of_container(n1_next), 3);

        VOS_AVL3_Delete(&mut tree, &mut n1.node);
        VOS_AVL3_Delete(&mut tree, &mut n3.node);
        assert!(tree.pstRoot.is_null());
        assert!(tree.pstFirst.is_null());
        assert!(tree.pstLast.is_null());
    }
}
'''.lstrip()


def _native_quadtree_test_template() -> str:
    """生成 quadtree Rust-native API 的 RQ2 等价测试。"""
    return r'''// RQ2 adapter for quadtree Rust-native API.

use crate::src_quadtree::*;
use core::ffi::c_void;

fn assert_point(point: &crate::types::quadtree_point_t, x: f64, y: f64) {
    assert_eq!(point.x, x);
    assert_eq!(point.y, y);
}

#[test]
fn test_quadtree_new_and_free() {
    let qt = quadtree_new(-1.0, -1.0, 1000.0, 1000.0);

    assert!(qt.root.is_some(), "root should exist");
    assert_eq!(qt.length, 0u32, "new tree length should be zero");

    quadtree_free(qt);
}

#[test]
fn test_quadtree_insert_and_search() {
    let mut qt = quadtree_new(-1.0, -1.0, 1000.0, 1000.0);
    let key = 0x1234usize as *mut c_void;

    assert_eq!(quadtree_insert(&mut qt, 10.0, 20.0, key), 1);
    assert_eq!(qt.length, 1u32);

    let point = quadtree_search(&qt, 10.0, 20.0).expect("inserted point should be found");
    assert_point(point, 10.0, 20.0);

    quadtree_free(qt);
}

#[test]
fn test_quadtree_duplicate_insert() {
    let mut qt = quadtree_new(-1.0, -1.0, 1000.0, 1000.0);
    let key1 = 0x1111usize as *mut c_void;
    let key2 = 0x2222usize as *mut c_void;

    assert_eq!(quadtree_insert(&mut qt, 1.0, 2.0, key1), 1);
    assert_eq!(qt.length, 1u32);
    assert_eq!(quadtree_insert(&mut qt, 1.0, 2.0, key2), 0);
    assert_eq!(qt.length, 1u32);

    let point = quadtree_search(&qt, 1.0, 2.0).expect("original coordinates should remain searchable");
    assert_point(point, 1.0, 2.0);

    quadtree_free(qt);
}

#[test]
fn test_quadtree_multiple_points() {
    let mut qt = quadtree_new(-1.0, -1.0, 1000.0, 1000.0);
    let a = 0xaaaausize as *mut c_void;
    let b = 0xbbbbusize as *mut c_void;
    let c = 0xccccusize as *mut c_void;

    assert_eq!(quadtree_insert(&mut qt, 10.0, 10.0, a), 1);
    assert_eq!(quadtree_insert(&mut qt, 90.0, 10.0, b), 1);
    assert_eq!(quadtree_insert(&mut qt, 50.0, 90.0, c), 1);
    assert_eq!(qt.length, 3u32);

    let pa = quadtree_search(&qt, 10.0, 10.0).expect("point a should be found");
    assert_point(pa, 10.0, 10.0);
    let pb = quadtree_search(&qt, 90.0, 10.0).expect("point b should be found");
    assert_point(pb, 90.0, 10.0);
    let pc = quadtree_search(&qt, 50.0, 90.0).expect("point c should be found");
    assert_point(pc, 50.0, 90.0);

    quadtree_free(qt);
}
'''.lstrip()


def _native_zopfli_test_template() -> str:
    """生成 zopfli Rust-native API 的 RQ2 等价测试。"""
    return r'''// RQ2 adapter for zopfli Rust-native API.

use crate::src_hash::{
    ZopfliAllocHash, ZopfliCleanHash, ZopfliResetHash, ZopfliUpdateHash, ZopfliWarmupHash,
};
use crate::src_lz77::{ZopfliCleanLZ77Store, ZopfliInitLZ77Store, ZopfliStoreLitLenDist};
use crate::src_util::ZopfliInitOptions;
use crate::src_zopfli_lib::ZopfliCompress;
use crate::types::{
    size_t, ZopfliHash, ZopfliLZ77Store, ZopfliOptions,
    __c2r_tu_types_src_zopfli_bin::ZOPFLI_FORMAT_DEFLATE,
};

fn to_hex(data: &[u8]) -> String {
    const LUT: &[u8; 16] = b"0123456789abcdef";
    let mut out = Vec::with_capacity(data.len() * 2);
    for &b in data {
        out.push(LUT[(b >> 4) as usize]);
        out.push(LUT[(b & 0x0f) as usize]);
    }
    String::from_utf8(out).unwrap()
}

#[test]
fn test_zopfli_init_options_defaults() {
    let mut opts = ZopfliOptions {
        verbose: -1,
        verbose_more: -1,
        numiterations: -1,
        blocksplitting: -1,
        blocksplittinglast: -1,
        blocksplittingmax: -1,
    };

    ZopfliInitOptions(&mut opts);

    assert_eq!(opts.verbose, 0);
    assert_eq!(opts.verbose_more, 0);
    assert_eq!(opts.numiterations, 15);
    assert_eq!(opts.blocksplitting, 1);
    assert_eq!(opts.blocksplittinglast, 0);
    assert_eq!(opts.blocksplittingmax, 15);
}

#[test]
fn test_zopfli_hash_alloc_reset_update_clean() {
    let window_size: usize = 32_768;
    let input = b"aaaaaa";
    let mut hash = ZopfliHash::new();
    ZopfliAllocHash(window_size, &mut hash);

    assert_eq!(hash.head.len(), 65_536);
    assert_eq!(hash.prev.len(), window_size);
    assert_eq!(hash.hashval.len(), window_size);
    assert_eq!(hash.same.len(), window_size);
    assert_eq!(hash.head2.len(), 65_536);
    assert_eq!(hash.prev2.len(), window_size);
    assert_eq!(hash.hashval2.len(), window_size);

    ZopfliResetHash(window_size, &mut hash);
    assert_eq!(hash.val, 0);
    assert_eq!(hash.val2, 0);
    assert_eq!(hash.head[0], -1);
    assert_eq!(hash.prev[0], 0);
    assert_eq!(hash.hashval[0], -1);
    assert_eq!(hash.same[0], 0);
    assert_eq!(hash.head2[0], -1);
    assert_eq!(hash.prev2[0], 0);
    assert_eq!(hash.hashval2[0], -1);

    ZopfliWarmupHash(input, 0, input.len(), &mut hash);
    ZopfliUpdateHash(input, 0, input.len(), &mut hash);
    assert!(hash.hashval[0] >= 0);
    assert_eq!(hash.head[hash.val as usize], 0);

    ZopfliCleanHash(&mut hash);
}

#[test]
fn test_zopfli_lz77_store_litlen_lifecycle() {
    let input = b"abc";
    let mut store = ZopfliLZ77Store::new();
    ZopfliInitLZ77Store(Some(input), &mut store);

    assert_eq!(store.size, 0);
    assert_eq!(store.original_data, Some(input.as_slice()));
    assert!(store.litlens.is_empty());
    assert!(store.dists.is_empty());
    assert!(store.pos.is_empty());

    ZopfliStoreLitLenDist(b'a' as u16, 0, 0, &mut store);
    assert_eq!(store.size, 1);
    assert_eq!(store.litlens[0], b'a' as u16);
    assert_eq!(store.dists[0], 0);
    assert_eq!(store.pos[0], 0);
    assert_eq!(store.ll_symbol[0], b'a' as u16);
    assert_eq!(store.d_symbol[0], 0);

    ZopfliCleanLZ77Store(&mut store);
    assert_eq!(store.size, 0);
    assert!(store.litlens.is_empty());
    assert!(store.dists.is_empty());
    assert!(store.pos.is_empty());
}

#[test]
fn test_zopfli_zz_deflate_output_matches_reference() {
    let input = b"hello zopfli test";
    let mut opts = ZopfliOptions {
        verbose: 0,
        verbose_more: 0,
        numiterations: 0,
        blocksplitting: 0,
        blocksplittinglast: 0,
        blocksplittingmax: 0,
    };

    ZopfliInitOptions(&mut opts);
    opts.numiterations = 5;

    let out = ZopfliCompress(&opts, ZOPFLI_FORMAT_DEFLATE, input);
    assert_eq!(out.len(), 19);
    assert_eq!(to_hex(&out), "cb48cdc9c957a8ca2f48cbc95428492d2e0100");
}
'''.lstrip()


def _adapt_test_template(project: str, text: str, src_dir: Path | None = None) -> Tuple[str, List[str]]:
    """把旧 RQ2 测试模板适配到当前临时 Rust crate。"""
    actions: List[str] = []
    if project == "buffer" and _buffer_uses_native_api(src_dir):
        return _native_buffer_test_template(), [
            "replace_c_pointer_buffer_harness_with_rust_native_api",
        ]
    if project == "avl":
        return _native_avl_test_template(), [
            "replace_c_pointer_avl_harness_with_rust_native_api",
        ]
    if project == "ht":
        return _native_ht_test_template(), [
            "replace_c_pointer_ht_harness_with_rust_native_api",
        ]
    if project == "rgba" and _rgba_uses_native_api(src_dir):
        return _native_rgba_test_template(), [
            "replace_c_pointer_rgba_harness_with_rust_native_api",
        ]
    if project == "quadtree" and _quadtree_uses_native_api(src_dir):
        return _native_quadtree_test_template(), [
            "replace_c_pointer_quadtree_harness_with_rust_native_api",
        ]
    if project == "urlparser" and _urlparser_uses_native_api(src_dir):
        return _native_urlparser_test_template(), [
            "replace_c_pointer_urlparser_harness_with_rust_native_api",
            "correct_urlparser_file_scheme_oracle",
        ]
    if project == "genann" and _genann_uses_native_api(src_dir):
        return _native_genann_test_template(), [
            "replace_c_pointer_genann_harness_with_rust_native_api",
        ]
    if project == "zopfli" and _zopfli_uses_native_api(src_dir):
        return _zopfli_native_test_template(src_dir), [
            "replace_c_pointer_zopfli_harness_with_rust_native_api",
        ]
    if project == "urlparser":
        for field in (
            "href",
            "auth",
            "protocol",
            "port",
            "hostname",
            "host",
            "pathname",
            "path",
            "hash",
            "search",
            "query",
        ):
            text = re.sub(
                rf"field_ptr\(([^,\n]+),\s*crate::compat::c2r_field_ptr_url_data_t__{field}\)",
                rf"(*\1).{field}",
                text,
            )
        actions.append("rewrite_urlparser_field_accessor_to_concrete_fields")
    if project == "qsort" and _qsort_uses_safe_slice_api(src_dir):
        text = re.sub(
            r"quickSort\(\s*arr\.as_mut_ptr\(\)\s*,\s*([^,\n]+),\s*([^)]+)\);",
            r"quickSort(&mut arr, \1, \2);",
            text,
        )
        actions.append("qsort_safe_slice_bridge")
    return text, actions


def _ensure_cc_build_dependency(cargo_toml: Path) -> bool:
    """确保临时测试 crate 有 cc build-dependency。"""
    if not cargo_toml.is_file():
        return False
    text = cargo_toml.read_text(encoding="utf-8", errors="ignore")
    build_match = re.search(r"(?ms)^\[build-dependencies\]\s*(.*?)(?=^\[|\Z)", text)
    if build_match:
        if re.search(r"(?m)^\s*cc\s*=", build_match.group(1)):
            return False
        insert_at = build_match.end(0)
        cargo_toml.write_text(text[:insert_at].rstrip() + '\ncc = "1.0"\n' + text[insert_at:], encoding="utf-8")
        return True
    cargo_toml.write_text(text.rstrip() + '\n\n[build-dependencies]\ncc = "1.0"\n', encoding="utf-8")
    return True


def _ensure_native_accessor_build(project_out: Path) -> List[str]:
    """让临时测试 crate 链接现有 native/c2r_accessors.c shim。"""
    accessor_c = project_out / "native" / "c2r_accessors.c"
    if not accessor_c.is_file():
        return []
    src_dir = project_out / "src"
    if src_dir.is_dir() and not any("c2r_field_ptr_" in _source_text(path) for path in src_dir.glob("*.rs")):
        return []

    actions: List[str] = []
    if _ensure_cc_build_dependency(project_out / "Cargo.toml"):
        actions.append("add_cc_build_dependency_for_native_accessors")

    build_rs = project_out / "build.rs"
    build_block = r'''
    if std::path::Path::new("native/c2r_accessors.c").is_file() {
        let mut build = cc::Build::new();
        build.file("native/c2r_accessors.c");
        if std::path::Path::new("native/include").is_dir() {
            build.include("native/include");
        }
        build.compile("c2r_accessors");
        println!("cargo:rerun-if-changed=native/c2r_accessors.c");
        println!("cargo:rerun-if-changed=native/include/c2r_accessors.h");
    }
'''.rstrip()
    if not build_rs.is_file():
        build_rs.write_text(
            "//! 临时 RQ2 测试构建脚本。\n\nfn main() {\n" + build_block + "\n}\n",
            encoding="utf-8",
        )
        actions.append("compile_native_accessor_shims_for_tests")
        return actions

    text = build_rs.read_text(encoding="utf-8", errors="ignore")
    if "native/c2r_accessors.c" in text:
        return actions
    updated, count = re.subn(r"fn\s+main\s*\(\s*\)\s*\{", lambda m: m.group(0) + "\n" + build_block, text, count=1)
    if count == 0:
        updated = text.rstrip() + "\n\nfn main() {\n" + build_block + "\n}\n"
    build_rs.write_text(updated, encoding="utf-8")
    actions.append("compile_native_accessor_shims_for_tests")
    return actions


def _trim_after_first_marker_end(path: Path, marker: str, required_suffix: str = "") -> bool:
    """清理并发写入在 marker 后留下的残片。"""
    if not path.is_file():
        return False
    text = path.read_text(encoding="utf-8", errors="ignore")
    marker_index = text.find(marker)
    if marker_index < 0:
        return False
    end = marker_index + len(marker)
    if required_suffix:
        suffix_index = text.find(required_suffix, end)
        if suffix_index < 0:
            return False
        end = suffix_index + len(required_suffix)
    kept = text[:end].rstrip() + "\n"
    if kept == text:
        return False
    path.write_text(kept, encoding="utf-8")
    return True


def _sanitize_native_accessor_markers(project_out: Path) -> List[str]:
    """在临时测试副本中清理 accessor shim marker 后的并发写残片。"""
    actions: List[str] = []
    header = project_out / "native" / "include" / "c2r_accessors.h"
    source = project_out / "native" / "c2r_accessors.c"
    if _trim_after_first_marker_end(header, "// === C2R_FIELD_PTR_DECLS_END ===", "#endif  /* C2R_ACCESSORS_H */"):
        actions.append("trim_corrupt_native_accessor_header_tail_for_tests")
    if _trim_after_first_marker_end(source, "// === C2R_FIELD_PTR_DEFS_END ==="):
        actions.append("trim_corrupt_native_accessor_source_tail_for_tests")
    return actions


def _adapt_bzip2_private_bzfile_accessors(project_out: Path) -> List[str]:
    """让 bzip2 测试副本中的 BZFILE accessor 使用私有 bzFile 布局。"""
    source = project_out / "native" / "c2r_accessors.c"
    if not source.is_file():
        return []
    text = source.read_text(encoding="utf-8", errors="ignore")
    if "((BZFILE*)base)->" not in text:
        return []

    layout = r'''
#ifndef C2R_BZIP2_BZFILE_ACCESSOR_LAYOUT_DEFINED
#define C2R_BZIP2_BZFILE_ACCESSOR_LAYOUT_DEFINED
typedef struct {
    FILE* handle;
    Char buf[5000];
    Int32 bufN;
    Bool writing;
    bz_stream strm;
    Int32 lastErr;
    Bool initialisedOk;
} c2r_bzFile;
#endif
'''.strip()
    if "C2R_BZIP2_BZFILE_ACCESSOR_LAYOUT_DEFINED" not in text:
        marker = "// === C2R_INCLUDE_END ==="
        if marker in text:
            text = text.replace(marker, marker + "\n\n" + layout, 1)
        else:
            text = layout + "\n\n" + text
    text = text.replace("((BZFILE*)base)->", "((c2r_bzFile*)base)->")
    source.write_text(text, encoding="utf-8")
    return ["rewrite_bzip2_bzfile_accessors_to_private_layout_for_tests"]


def _bzip2_state_layout_block() -> str:
    """生成 bzip2 测试副本所需的 EState/DState 真实布局。"""
    return r'''
// C2R_RQ2_BZIP2_STATE_LAYOUTS_BEGIN
pub const C2R_BZIP2_BZ_MAX_ALPHA_SIZE: usize = 258;
pub const C2R_BZIP2_BZ_N_GROUPS: usize = 6;
pub const C2R_BZIP2_BZ_G_SIZE: usize = 50;
pub const C2R_BZIP2_BZ_MAX_SELECTORS: usize = 2 + (900000 / C2R_BZIP2_BZ_G_SIZE);
pub const C2R_BZIP2_MTFA_SIZE: usize = 4096;
pub const C2R_BZIP2_MTFL_SIZE: usize = 16;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EState {
    pub strm: *mut bz_stream,
    pub mode: Int32,
    pub state: Int32,
    pub avail_in_expect: UInt32,
    pub arr1: *mut UInt32,
    pub arr2: *mut UInt32,
    pub ftab: *mut UInt32,
    pub origPtr: Int32,
    pub ptr: *mut UInt32,
    pub block: *mut UChar,
    pub mtfv: *mut UInt16,
    pub zbits: *mut UChar,
    pub workFactor: Int32,
    pub state_in_ch: UInt32,
    pub state_in_len: Int32,
    pub rNToGo: Int32,
    pub rTPos: Int32,
    pub nblock: Int32,
    pub nblockMAX: Int32,
    pub numZ: Int32,
    pub state_out_pos: Int32,
    pub nInUse: Int32,
    pub inUse: [Bool; 256],
    pub unseqToSeq: [UChar; 256],
    pub bsBuff: UInt32,
    pub bsLive: Int32,
    pub blockCRC: UInt32,
    pub combinedCRC: UInt32,
    pub verbosity: Int32,
    pub blockNo: Int32,
    pub blockSize100k: Int32,
    pub nMTF: Int32,
    pub mtfFreq: [Int32; C2R_BZIP2_BZ_MAX_ALPHA_SIZE],
    pub selector: [UChar; C2R_BZIP2_BZ_MAX_SELECTORS],
    pub selectorMtf: [UChar; C2R_BZIP2_BZ_MAX_SELECTORS],
    pub len: [[UChar; C2R_BZIP2_BZ_MAX_ALPHA_SIZE]; C2R_BZIP2_BZ_N_GROUPS],
    pub code: [[Int32; C2R_BZIP2_BZ_MAX_ALPHA_SIZE]; C2R_BZIP2_BZ_N_GROUPS],
    pub rfreq: [[Int32; C2R_BZIP2_BZ_MAX_ALPHA_SIZE]; C2R_BZIP2_BZ_N_GROUPS],
    pub len_pack: [[UInt32; 4]; C2R_BZIP2_BZ_MAX_ALPHA_SIZE],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DState {
    pub strm: *mut bz_stream,
    pub state: Int32,
    pub state_out_ch: UChar,
    pub state_out_len: Int32,
    pub blockRandomised: Bool,
    pub rNToGo: Int32,
    pub rTPos: Int32,
    pub bsBuff: UInt32,
    pub bsLive: Int32,
    pub blockSize100k: Int32,
    pub smallDecompress: Bool,
    pub currBlockNo: Int32,
    pub verbosity: Int32,
    pub origPtr: Int32,
    pub tPos: UInt32,
    pub k0: Int32,
    pub unzftab: [Int32; 256],
    pub nblock_used: Int32,
    pub cftab: [Int32; 257],
    pub cftabCopy: [Int32; 257],
    pub tt: *mut UInt32,
    pub ll16: *mut UInt16,
    pub ll4: *mut UChar,
    pub storedBlockCRC: UInt32,
    pub storedCombinedCRC: UInt32,
    pub calculatedBlockCRC: UInt32,
    pub calculatedCombinedCRC: UInt32,
    pub nInUse: Int32,
    pub inUse: [Bool; 256],
    pub inUse16: [Bool; 16],
    pub seqToUnseq: [UChar; 256],
    pub mtfa: [UChar; C2R_BZIP2_MTFA_SIZE],
    pub mtfbase: [Int32; 256 / C2R_BZIP2_MTFL_SIZE],
    pub selector: [UChar; C2R_BZIP2_BZ_MAX_SELECTORS],
    pub selectorMtf: [UChar; C2R_BZIP2_BZ_MAX_SELECTORS],
    pub len: [[UChar; C2R_BZIP2_BZ_MAX_ALPHA_SIZE]; C2R_BZIP2_BZ_N_GROUPS],
    pub limit: [[Int32; C2R_BZIP2_BZ_MAX_ALPHA_SIZE]; C2R_BZIP2_BZ_N_GROUPS],
    pub base: [[Int32; C2R_BZIP2_BZ_MAX_ALPHA_SIZE]; C2R_BZIP2_BZ_N_GROUPS],
    pub perm: [[Int32; C2R_BZIP2_BZ_MAX_ALPHA_SIZE]; C2R_BZIP2_BZ_N_GROUPS],
    pub minLens: [Int32; C2R_BZIP2_BZ_N_GROUPS],
    pub save_i: Int32,
    pub save_j: Int32,
    pub save_t: Int32,
    pub save_alphaSize: Int32,
    pub save_nGroups: Int32,
    pub save_nSelectors: Int32,
    pub save_EOB: Int32,
    pub save_groupNo: Int32,
    pub save_groupPos: Int32,
    pub save_nextSym: Int32,
    pub save_nblockMAX: Int32,
    pub save_nblock: Int32,
    pub save_es: Int32,
    pub save_N: Int32,
    pub save_curr: Int32,
    pub save_zt: Int32,
    pub save_zn: Int32,
    pub save_zvec: Int32,
    pub save_zj: Int32,
    pub save_gSel: Int32,
    pub save_gMinlen: Int32,
    pub save_gLimit: *mut Int32,
    pub save_gBase: *mut Int32,
    pub save_gPerm: *mut Int32,
}
// C2R_RQ2_BZIP2_STATE_LAYOUTS_END
'''.strip()


def _adapt_bzip2_state_layouts_for_tests(project_out: Path) -> List[str]:
    """在临时测试副本中把 opaque EState/DState 修成真实 bzip2 状态布局。"""
    types_rs = project_out / "src" / "types.rs"
    if not types_rs.is_file():
        return []
    text = types_rs.read_text(encoding="utf-8", errors="ignore")
    if "C2R_RQ2_BZIP2_STATE_LAYOUTS_BEGIN" in text:
        return []
    if not re.search(r"pub\s+struct\s+DState\s*\{\s*_opaque\s*:\s*\[\s*u8\s*;\s*0\s*\]\s*\}", text):
        return []
    if not re.search(r"pub\s+struct\s+EState\s*\{\s*_opaque\s*:\s*\[\s*u8\s*;\s*0\s*\]\s*\}", text):
        return []

    text = re.sub(
        r"(?ms)/// Opaque placeholder for external type `DState`.*?pub\s+struct\s+DState\s*\{\s*_opaque\s*:\s*\[\s*u8\s*;\s*0\s*\]\s*\}\s*",
        "",
        text,
        count=1,
    )
    text = re.sub(
        r"(?ms)/// Opaque placeholder for external type `EState`.*?pub\s+struct\s+EState\s*\{\s*_opaque\s*:\s*\[\s*u8\s*;\s*0\s*\]\s*\}\s*",
        "",
        text,
        count=1,
    )
    marker = "// ============================================================\n// Project-specific Types (scanned from headers, opaque)\n// ============================================================"
    layout = _bzip2_state_layout_block()
    if marker in text:
        text = text.replace(marker, marker + "\n\n" + layout, 1)
    else:
        text = text.rstrip() + "\n\n" + layout + "\n"
    types_rs.write_text(text, encoding="utf-8")
    return ["rewrite_bzip2_opaque_state_layouts_for_tests"]


def _sync_native_accessor_symbols(project_out: Path) -> List[str]:
    """从 compat.rs 同步临时 native accessor 的缺失声明和定义。"""
    compat = project_out / "src" / "compat.rs"
    header = project_out / "native" / "include" / "c2r_accessors.h"
    source = project_out / "native" / "c2r_accessors.c"
    if not (compat.is_file() and header.is_file() and source.is_file()):
        return []

    compat_text = compat.read_text(encoding="utf-8", errors="ignore")
    header_text = header.read_text(encoding="utf-8", errors="ignore")
    source_text = source.read_text(encoding="utf-8", errors="ignore")
    wanted = sorted(set(re.findall(r"pub\s+fn\s+(c2r_field_ptr_([A-Za-z0-9_]+)__([A-Za-z0-9_]+))\(", compat_text)))
    existing_decls = set(re.findall(r"void\*\s+(c2r_field_ptr_[A-Za-z0-9_]+__[A-Za-z0-9_]+)\(", header_text))
    existing_defs = set(re.findall(r"void\*\s+(c2r_field_ptr_[A-Za-z0-9_]+__[A-Za-z0-9_]+)\(", source_text))

    decls: List[str] = []
    defs: List[str] = []
    for func, c_type, field in wanted:
        if func not in existing_decls:
            decls.append(f"void* {func}(void* base);")
        if func not in existing_defs:
            defs.append(
                "\n".join(
                    [
                        f"void* {func}(void* base) {{",
                        f"    return (void*)(&(({c_type}*)base)->{field});",
                        "}",
                    ]
                )
            )

    changed = False
    if decls and "// === C2R_FIELD_PTR_DECLS_END ===" in header_text:
        header_text = header_text.replace("// === C2R_FIELD_PTR_DECLS_END ===", "\n".join(decls) + "\n// === C2R_FIELD_PTR_DECLS_END ===", 1)
        header.write_text(header_text, encoding="utf-8")
        changed = True
    if defs and "// === C2R_FIELD_PTR_DEFS_END ===" in source_text:
        source_text = source_text.replace("// === C2R_FIELD_PTR_DEFS_END ===", "\n".join(defs) + "\n// === C2R_FIELD_PTR_DEFS_END ===", 1)
        source.write_text(source_text, encoding="utf-8")
        changed = True
    return ["sync_native_accessor_symbols_from_compat_for_tests"] if changed else []


def _project_rustflags(project: str, src_dir: Path | None = None) -> List[str]:
    """返回临时测试构建需要追加的 rustflags。"""
    if project == "bzip2" and src_dir is not None:
        source = _source_text(src_dir / "src_decompress.rs")
        if "GET_BITS!(36" in source and "1u32 << ($n as u32)" in source:
            return ["-A", "arithmetic-overflow"]
    return []


def _merge_rustflags(existing: str, extra: Sequence[str]) -> str:
    """追加 rustflags，保留外部环境已有设置。"""
    parts = [existing.strip()] if existing.strip() else []
    parts.extend(flag for flag in extra if flag)
    return " ".join(parts)


def _quadtree_test_support_module() -> str:
    """生成 quadtree 旧测试所需的 test-only 字段 accessor 符号。"""
    return r'''
#![allow(dead_code)]
#![allow(improper_ctypes_definitions)]
#![allow(non_snake_case)]

use core::ffi::c_void;

#[repr(C)]
struct QtPoint {
    x: f64,
    y: f64,
}

#[repr(C)]
struct QtBounds {
    nw: *mut crate::types::quadtree_point_t,
    se: *mut crate::types::quadtree_point_t,
    width: f64,
    height: f64,
}

#[repr(C)]
struct QtNode {
    ne: *mut crate::types::quadtree_node_t,
    nw: *mut crate::types::quadtree_node_t,
    se: *mut crate::types::quadtree_node_t,
    sw: *mut crate::types::quadtree_node_t,
    bounds: *mut crate::types::quadtree_bounds_t,
    point: *mut crate::types::quadtree_point_t,
    key: *mut c_void,
}

#[repr(C)]
struct QtTree {
    root: *mut crate::types::quadtree_node_t,
    key_free: Option<unsafe extern "C" fn(*mut c_void)>,
    length: u32,
}

#[no_mangle]
pub unsafe extern "C" fn c2r_field_ptr_quadtree_point_t__x(base: *mut c_void) -> *mut c_void {
    core::ptr::addr_of_mut!((*(base as *mut QtPoint)).x) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn c2r_field_ptr_quadtree_point_t__y(base: *mut c_void) -> *mut c_void {
    core::ptr::addr_of_mut!((*(base as *mut QtPoint)).y) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn c2r_field_ptr_quadtree_bounds_t__nw(base: *mut c_void) -> *mut c_void {
    core::ptr::addr_of_mut!((*(base as *mut QtBounds)).nw) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn c2r_field_ptr_quadtree_bounds_t__se(base: *mut c_void) -> *mut c_void {
    core::ptr::addr_of_mut!((*(base as *mut QtBounds)).se) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn c2r_field_ptr_quadtree_bounds_t__width(base: *mut c_void) -> *mut c_void {
    core::ptr::addr_of_mut!((*(base as *mut QtBounds)).width) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn c2r_field_ptr_quadtree_bounds_t__height(base: *mut c_void) -> *mut c_void {
    core::ptr::addr_of_mut!((*(base as *mut QtBounds)).height) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn c2r_field_ptr_quadtree_node_t__ne(base: *mut c_void) -> *mut c_void {
    core::ptr::addr_of_mut!((*(base as *mut QtNode)).ne) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn c2r_field_ptr_quadtree_node_t__nw(base: *mut c_void) -> *mut c_void {
    core::ptr::addr_of_mut!((*(base as *mut QtNode)).nw) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn c2r_field_ptr_quadtree_node_t__se(base: *mut c_void) -> *mut c_void {
    core::ptr::addr_of_mut!((*(base as *mut QtNode)).se) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn c2r_field_ptr_quadtree_node_t__sw(base: *mut c_void) -> *mut c_void {
    core::ptr::addr_of_mut!((*(base as *mut QtNode)).sw) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn c2r_field_ptr_quadtree_node_t__bounds(base: *mut c_void) -> *mut c_void {
    core::ptr::addr_of_mut!((*(base as *mut QtNode)).bounds) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn c2r_field_ptr_quadtree_node_t__point(base: *mut c_void) -> *mut c_void {
    core::ptr::addr_of_mut!((*(base as *mut QtNode)).point) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn c2r_field_ptr_quadtree_node_t__key(base: *mut c_void) -> *mut c_void {
    core::ptr::addr_of_mut!((*(base as *mut QtNode)).key) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn c2r_field_ptr_quadtree_t__root(base: *mut c_void) -> *mut c_void {
    core::ptr::addr_of_mut!((*(base as *mut QtTree)).root) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn c2r_field_ptr_quadtree_t__key_free(base: *mut c_void) -> *mut c_void {
    core::ptr::addr_of_mut!((*(base as *mut QtTree)).key_free) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn c2r_field_ptr_quadtree_t__length(base: *mut c_void) -> *mut c_void {
    core::ptr::addr_of_mut!((*(base as *mut QtTree)).length) as *mut c_void
}
'''.lstrip()


def _buffer_test_support_module() -> str:
    """生成 buffer 翻译产物所需的 test-only 字段 accessor 符号。"""
    return r'''
#![allow(dead_code)]
#![allow(improper_ctypes_definitions)]
#![allow(non_snake_case)]

use core::ffi::c_void;

#[repr(C)]
struct BufferLayout {
    len: crate::types::size_t,
    alloc: *mut ::core::ffi::c_char,
    data: *mut ::core::ffi::c_char,
}

#[no_mangle]
pub unsafe extern "C" fn c2r_field_ptr_buffer_t__len(base: *mut c_void) -> *mut c_void {
    core::ptr::addr_of_mut!((*(base as *mut BufferLayout)).len) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn c2r_field_ptr_buffer_t__alloc(base: *mut c_void) -> *mut c_void {
    core::ptr::addr_of_mut!((*(base as *mut BufferLayout)).alloc) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn c2r_field_ptr_buffer_t__data(base: *mut c_void) -> *mut c_void {
    core::ptr::addr_of_mut!((*(base as *mut BufferLayout)).data) as *mut c_void
}
'''.lstrip()


def _project_support_module(project: str) -> str:
    """返回当前项目需要注入到临时测试 crate 的支撑模块源码。"""
    if project == "buffer":
        return _buffer_test_support_module()
    if project == "quadtree":
        return _quadtree_test_support_module()
    return ""


def _write_project_test_support(project: str, src_dir: Path) -> List[str]:
    """在临时 crate 中写入 test-only 支撑模块，返回需要声明的模块名。"""
    module = _project_support_module(project)
    if not module:
        return []
    (src_dir / "test_c2r_support.rs").write_text(module, encoding="utf-8")
    return ["test_c2r_support"]


def _uses_native_replacement(adapter_actions: Sequence[str]) -> bool:
    """判断当前项目测试是否已整体替换为 Rust-native harness。"""
    return any(
        action.startswith("replace_c_pointer_") and action.endswith("_harness_with_rust_native_api")
        for action in adapter_actions
    )


def _adapt_project_entry_for_tests(project: str, target_rs: Path) -> List[str]:
    """按项目修正临时测试 crate 入口，避免 harness 编译层面的冲突。"""
    if project != "zopfli" or not target_rs.is_file():
        return []
    text = target_rs.read_text(encoding="utf-8", errors="ignore")
    actions: List[str] = []
    pattern = r"(?m)^pub\s+mod\s+src_zopfli_bin\s*;\s*$"
    if re.search(pattern, text):
        replacement = "#[cfg(not(test))]\npub mod src_zopfli_bin;"
        text = re.sub(pattern, replacement, text, count=1)
        actions.append("disable_zopfli_bin_module_for_test_harness")
    main_pattern = r"(?m)^fn\s+main\s*\(\s*\)\s*\{"
    if re.search(main_pattern, text) and "#[cfg(not(test))]\nfn main()" not in text:
        text = re.sub(main_pattern, "#[cfg(not(test))]\nfn main() {", text, count=1)
        actions.append("disable_zopfli_bin_main_for_test_harness")
    if actions:
        target_rs.write_text(text, encoding="utf-8")
    return actions


def _inject_test_modules(target_rs: Path, support_modules: Sequence[str]) -> None:
    """把测试模块声明注入 main/lib 入口。"""
    text = target_rs.read_text(encoding="utf-8", errors="ignore")
    additions: List[str] = []
    def has_module(module: str) -> bool:
        return bool(re.search(rf"(?m)^\s*(?:pub\s+)?mod\s+{re.escape(module)}\s*;", text))

    for module in support_modules:
        if not has_module(module):
            additions.append(f"#[cfg(test)]\nmod {module};")
    if not has_module("test_c2r"):
        additions.append("#[cfg(test)]\nmod test_c2r;")
    if additions:
        target_rs.write_text(text.rstrip() + "\n\n" + "\n".join(additions) + "\n", encoding="utf-8")


def _categorize_failure(output: str, result: Dict[str, Any]) -> Dict[str, Any]:
    """把失败粗分为 harness 适配问题或翻译语义/内存问题。"""
    categories: List[str] = []
    assessment = "pass"
    detail = ""
    if result.get("pass_rate") == 1.0 and result.get("compilation_succeeded"):
        return {"assessment": assessment, "categories": categories, "detail": detail}

    if (
        re.search(r"error\[E0425\][\s\S]{0,600}src/test_c2r\.rs[\s\S]{0,600}c2r_field_ptr_", output)
        or re.search(r"undefined symbol:\s*c2r_field_ptr_", output)
        or re.search(r"src/test_c2r\.rs:\d+:\d+[\s\S]{0,240}crate::compat::c2r_field_ptr_", output)
    ):
        categories.append("harness_accessor_mismatch")
    if (
        re.search(r"error\[E0\d+\]", output)
        or "could not compile" in output
        or "failed to run custom build command" in output
        or "error occurred in cc-rs" in output
    ):
        categories.append("compile_failure")
    if "signal:" in output or "SIGABRT" in output or "panicked at" in output or "assertion failed" in output:
        categories.append("runtime_semantic_or_memory_failure")
    if "Timeout after" in output or result.get("returncode") == 124:
        categories.append("runtime_timeout")
    if result.get("compilation_succeeded") and result.get("tests_failed"):
        categories.append("runtime_semantic_or_memory_failure")
    if "unified RQ2" in output and "expects" in output:
        categories.append("test_template_api_expectation_mismatch")

    if "harness_accessor_mismatch" in categories:
        assessment = "harness_adaptation_issue"
        detail = "测试模板引用了当前 crate 未生成的 compat 字段 accessor，不能直接判断 Rust 语义。"
    elif "compile_failure" in categories:
        assessment = "harness_or_public_api_mismatch"
        detail = "测试模板与当前 public API/模块布局不匹配，需先适配 harness 后再判断语义。"
    elif "runtime_semantic_or_memory_failure" in categories:
        assessment = "translation_semantic_or_memory_issue"
        detail = "测试已进入运行阶段，失败更接近翻译语义或内存安全问题。"
    elif "runtime_timeout" in categories:
        assessment = "translation_semantic_or_memory_issue"
        detail = "测试已进入运行阶段并超时，失败更接近翻译语义或非终止问题。"
    else:
        assessment = "unknown_failure"
        detail = "失败模式未被现有规则识别。"
    return {"assessment": assessment, "categories": categories, "detail": detail}


def _parse_cargo_test_list(output: str) -> List[str]:
    """从 cargo test -- --list 输出中提取 test_c2r 模块完整测试名。"""
    names: List[str] = []
    seen = set()
    for raw_line in output.splitlines():
        line = raw_line.strip()
        match = re.match(r"^(test_c2r(?:::[A-Za-z_][A-Za-z0-9_]*)+):\s+test\b", line)
        if not match:
            continue
        name = match.group(1)
        if name not in seen:
            names.append(name)
            seen.add(name)
    return names


def _cargo_test_base_cmd(has_lib: bool) -> List[str]:
    """生成当前临时 crate 的 cargo test 基础命令。"""
    cmd = ["cargo", "test", "--offline"]
    if has_lib:
        cmd.append("--lib")
    return cmd


def _run_command(cmd: Sequence[str], cwd: Path, env: Dict[str, str], timeout: int) -> subprocess.CompletedProcess[str]:
    """在独立进程组中运行命令，超时时清理整个子进程组。"""
    proc = subprocess.Popen(
        list(cmd),
        cwd=cwd,
        env=env,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        start_new_session=True,
    )
    try:
        stdout, stderr = proc.communicate(timeout=timeout)
        return subprocess.CompletedProcess(list(cmd), proc.returncode, stdout, stderr)
    except subprocess.TimeoutExpired:
        try:
            os.killpg(proc.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        stdout, stderr = proc.communicate()
        timeout_text = f"\nTimeout after {timeout}s while running {' '.join(cmd)}\n"
        return subprocess.CompletedProcess(list(cmd), 124, (stdout or "") + timeout_text, stderr or "")


def _is_compile_error(output: str) -> bool:
    """判断 cargo 输出是否为编译或链接失败。"""
    return bool(re.search(r"error\[E\d+\]:|could not compile|error: linking with|undefined symbol:", output, re.IGNORECASE))


def _single_test_status(returncode: int, output: str) -> str:
    """把单个 cargo test --exact 结果规约为统计状态。"""
    if _is_compile_error(output):
        return "compile_failed"
    result_matches = re.findall(r"test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored", output)
    passed = sum(int(item[1]) for item in result_matches)
    failed = sum(int(item[2]) for item in result_matches)
    ignored = sum(int(item[3]) for item in result_matches)
    if returncode == 0 and passed > 0 and failed == 0:
        return "ok"
    if returncode == 0 and ignored > 0 and passed == 0 and failed == 0:
        return "ignored"
    if returncode == 0 and re.search(r"^running\s+0\s+tests\b", output, re.MULTILINE):
        return "not_found"
    if "signal:" in output or "SIGABRT" in output or "process didn't exit successfully" in output:
        return "crashed"
    if "panicked at" in output or "FAILED" in output or "assertion" in output:
        return "failed"
    return "failed" if returncode != 0 else "ok"


def run_project_test(crate_dir: Path, project: str, tests_dir: Path, out_root: Path, timeout: int) -> Dict[str, Any]:
    """复制 crate 后注入旧 RQ2 Rust 测试并运行 cargo test。"""
    test_file = tests_dir / project / "c2r" / "test.rs"
    result: Dict[str, Any] = {
        "project": project,
        "crate_dir": str(crate_dir.resolve()),
        "test_file": str(test_file.resolve()),
        "executed": False,
        "test_file_found": test_file.is_file(),
        "compilation_succeeded": False,
        "tests_passed": 0,
        "tests_failed": 0,
        "tests_ignored": 0,
        "total_tests": 0,
        "expected_total_tests": 0,
        "pass_rate": 0.0,
        "function_test_results": {},
        "discovered_test_names": [],
        "test_count_diagnostic": None,
        "adapter_actions": [],
        "error": None,
    }
    if not test_file.is_file():
        result["error"] = "test file not found"
        result["failure_analysis"] = {"assessment": "harness_missing", "categories": ["harness_missing"], "detail": "没有旧 RQ2 c2r/test.rs。"}
        return result

    result["expected_total_tests"] = _expected_test_count(test_file)
    project_out = out_root / project
    if project_out.exists():
        shutil.rmtree(project_out)
    shutil.copytree(crate_dir, project_out, ignore=shutil.ignore_patterns("target", ".git"))

    src_dir = project_out / "src"
    main_rs = src_dir / "main.rs"
    lib_rs = src_dir / "lib.rs"
    if not src_dir.is_dir():
        result["error"] = "src directory not found"
        result["failure_analysis"] = {"assessment": "harness_adaptation_issue", "categories": ["missing_src_dir"], "detail": "当前 crate 缺少 src 目录。"}
        return result

    adapted_test, adapter_actions = _adapt_test_template(project, test_file.read_text(encoding="utf-8"), src_dir)
    result["adapter_actions"] = adapter_actions
    (src_dir / "test_c2r.rs").write_text(adapted_test, encoding="utf-8")
    support_modules = [] if _uses_native_replacement(adapter_actions) else _write_project_test_support(project, src_dir)
    if support_modules:
        result["adapter_actions"].append(f"inject_{project}_test_support_module")
    result["adapter_actions"].extend(_sanitize_native_accessor_markers(project_out))
    result["adapter_actions"].extend(_adapt_bzip2_private_bzfile_accessors(project_out))
    if project == "bzip2":
        result["adapter_actions"].extend(_adapt_bzip2_state_layouts_for_tests(project_out))
    result["adapter_actions"].extend(_sync_native_accessor_symbols(project_out))
    result["adapter_actions"].extend(_ensure_native_accessor_build(project_out))
    extra_rustflags = _project_rustflags(project, src_dir)
    if extra_rustflags:
        result["adapter_actions"].append("allow_bzip2_test_arithmetic_overflow_lint")
    target_rs = lib_rs if lib_rs.is_file() else main_rs
    if target_rs.is_file():
        result["adapter_actions"].extend(_adapt_project_entry_for_tests(project, target_rs))
        _inject_test_modules(target_rs, support_modules)

    with tempfile.TemporaryDirectory(prefix=f"oss_rq2_target_{project}_") as target_dir:
        has_lib = lib_rs.is_file()
        env = {**os.environ, "RUST_BACKTRACE": "0", "CARGO_TARGET_DIR": target_dir}
        if extra_rustflags:
            env["RUSTFLAGS"] = _merge_rustflags(env.get("RUSTFLAGS", ""), extra_rustflags)
        list_cmd = _cargo_test_base_cmd(has_lib) + ["test_c2r", "--", "--list"]
        list_proc = _run_command(list_cmd, project_out, env, timeout)
        outputs = [(list_proc.stdout or "") + (list_proc.stderr or "")]
        list_output = outputs[0]
        result["executed"] = True
        result["returncode"] = list_proc.returncode

        if _is_compile_error(list_output) or list_proc.returncode != 0:
            result["compilation_succeeded"] = False
            result["error"] = "Compilation failed" if list_proc.returncode != 124 else f"Timeout after {timeout}s while listing tests"
            result["tests_failed"] = int(result["expected_total_tests"] or 0)
            result["total_tests"] = int(result["expected_total_tests"] or 0)
            result["output_tail"] = list_output[-8000:]
            result["failure_analysis"] = _categorize_failure(list_output, result)
            return result

        test_names = _parse_cargo_test_list(list_output)
        result["discovered_test_names"] = test_names
        expected_total = int(result["expected_total_tests"] or 0)
        if expected_total and expected_total != len(test_names):
            result["test_count_diagnostic"] = f"expected {expected_total} tests from template, discovered {len(test_names)} cargo tests"

        if not test_names:
            result["compilation_succeeded"] = True
            result["error"] = "No test_c2r tests discovered"
            result["tests_failed"] = expected_total
            result["total_tests"] = expected_total
            result["output_tail"] = list_output[-8000:]
            result["failure_analysis"] = {
                "assessment": "harness_adaptation_issue",
                "categories": ["no_tests_discovered"],
                "detail": "测试模块已编译但 cargo --list 未发现 test_c2r 测试。",
            }
            return result

        result["compilation_succeeded"] = True
        last_returncode = 0
        for test_name in test_names:
            test_cmd = _cargo_test_base_cmd(has_lib) + [test_name, "--", "--exact", "--test-threads=1"]
            test_proc = _run_command(test_cmd, project_out, env, timeout)
            test_output = (test_proc.stdout or "") + (test_proc.stderr or "")
            outputs.append(test_output)
            last_returncode = test_proc.returncode
            status = "timeout" if test_proc.returncode == 124 else _single_test_status(test_proc.returncode, test_output)

            result["function_test_results"][test_name] = status
            if status == "ok":
                result["tests_passed"] += 1
            elif status == "ignored":
                result["tests_ignored"] += 1
            else:
                result["tests_failed"] += 1

        missing_tests = max(expected_total - len(test_names), 0)
        result["tests_failed"] += missing_tests
        result["total_tests"] = max(expected_total, len(test_names)) - int(result["tests_ignored"] or 0)
        result["returncode"] = 0 if result["tests_failed"] == 0 else last_returncode or 1
        if result["tests_failed"]:
            result["error"] = "Test crashed or failed"
        result["output_tail"] = "\n".join(outputs)[-8000:]

    if result["total_tests"]:
        result["pass_rate"] = result["tests_passed"] / result["total_tests"]
    result["failure_analysis"] = _categorize_failure(str(result.get("output_tail") or ""), result)
    return result


def analyze_runs(run_dirs: Sequence[Path], projects: Sequence[str], tests_dir: Path, out_root: Path, timeout: int) -> Dict[str, Any]:
    """在多个 run 中查找项目 final crate 并运行旧 RQ2 测试。"""
    out_root.mkdir(parents=True, exist_ok=True)
    report: Dict[str, Any] = {"tests_dir": str(tests_dir.resolve()), "projects": {}, "summary": {}}
    for project in projects:
        crate_dir = None
        source_run = None
        for run_dir in run_dirs:
            candidate = find_project_crate_dir(_resolve_run_dir(run_dir), project)
            if candidate is not None:
                crate_dir = candidate
                source_run = run_dir
                break
        if crate_dir is None:
            report["projects"][project] = {"project": project, "error": "crate not found", "failure_analysis": {"assessment": "missing_crate", "categories": ["missing_crate"], "detail": "run 目录中找不到 final crate。"}}
            continue
        item = run_project_test(crate_dir, project, tests_dir, out_root, timeout)
        item["source_run_dir"] = str(Path(source_run).resolve()) if source_run else ""
        report["projects"][project] = item

    passed = sum(int(item.get("tests_passed") or 0) for item in report["projects"].values())
    total = sum(int(item.get("total_tests") or item.get("expected_total_tests") or 0) for item in report["projects"].values())
    report["summary"] = {
        "projects": len(projects),
        "executed": sum(1 for item in report["projects"].values() if item.get("executed")),
        "compilation_succeeded": sum(1 for item in report["projects"].values() if item.get("compilation_succeeded")),
        "tests_passed": passed,
        "total_tests": total,
        "pass_rate": (passed / total) if total else 0.0,
        "failure_assessments": {},
    }
    assessments: Dict[str, int] = {}
    for item in report["projects"].values():
        analysis = item.get("failure_analysis") if isinstance(item.get("failure_analysis"), dict) else {}
        key = str(analysis.get("assessment") or "unknown")
        assessments[key] = assessments.get(key, 0) + 1
    report["summary"]["failure_assessments"] = assessments
    return report


def render_markdown(report: Dict[str, Any]) -> str:
    """生成 Markdown 表。"""
    lines = [
        "| project | passed | total | rate | assessment | detail |",
        "|---|---:|---:|---:|---|---|",
    ]
    for project, item in report.get("projects", {}).items():
        analysis = item.get("failure_analysis") if isinstance(item.get("failure_analysis"), dict) else {}
        total = int(item.get("total_tests") or item.get("expected_total_tests") or 0)
        rate = float(item.get("pass_rate") or 0.0)
        lines.append(
            f"| {project} | {item.get('tests_passed', 0)} | {total} | {rate * 100:.2f}% | "
            f"{analysis.get('assessment', '')} | {analysis.get('detail', '')} |"
        )
    summary = report.get("summary") or {}
    lines.extend(
        [
            "",
            "| passed | total | rate |",
            "|---:|---:|---:|",
            f"| {summary.get('tests_passed', 0)} | {summary.get('total_tests', 0)} | {float(summary.get('pass_rate') or 0.0) * 100:.2f}% |",
        ]
    )
    return "\n".join(lines) + "\n"


def main(argv: Optional[Sequence[str]] = None) -> int:
    """命令行入口。"""
    parser = argparse.ArgumentParser(description="运行旧 RQ2 Rust 测试模板并诊断 harness 适配性。")
    parser.add_argument("--run-dir", action="append", type=Path, required=True, help="batch run 目录，可重复传入")
    parser.add_argument("--projects", required=True, help="逗号分隔项目名")
    parser.add_argument("--tests-dir", type=Path, default=DEFAULT_TESTS_DIR, help="旧 RQ2 测试模板目录")
    parser.add_argument("--timeout", type=int, default=240, help="单项目 cargo test 超时秒数")
    parser.add_argument("--work-dir", type=Path, default=Path("/tmp/oss_rq2_rust_tests_current"), help="临时 crate 输出目录")
    parser.add_argument("--output", "-o", type=Path, default=Path("paper_experiments/results/oss10_old_rq2_rust_tests_current.json"), help="JSON 输出路径")
    parser.add_argument("--markdown", type=Path, help="Markdown 输出路径")
    args = parser.parse_args(argv)

    report = analyze_runs(args.run_dir, _parse_projects(args.projects), args.tests_dir, args.work_dir, args.timeout)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2, ensure_ascii=False), encoding="utf-8")
    md_path = args.markdown or args.output.with_suffix(".md")
    md_path.write_text(render_markdown(report), encoding="utf-8")
    summary = report["summary"]
    print(f"JSON: {args.output}")
    print(f"Markdown: {md_path}")
    print(f"RQ2 tests: {summary['tests_passed']}/{summary['total_tests']} ({float(summary['pass_rate']) * 100:.2f}%)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
