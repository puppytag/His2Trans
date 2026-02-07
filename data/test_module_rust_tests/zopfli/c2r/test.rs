// C2R 框架测试文件 for zopfli
//
// 目标：做“输出正确性”测试（而不是仅编译检查）。
// 参考输出与上游测试一致：
// input="hello zopfli test", format=deflate, numiterations=5 => 固定字节序列。

use crate::src_util::ZopfliInitOptions;
use crate::src_zopfli_lib::ZopfliCompress;

use std::ptr;

#[repr(C)]
struct ZopfliOptions {
    verbose: i32,
    verbose_more: i32,
    numiterations: i32,
    blocksplitting: i32,
    blocksplittinglast: i32,
    blocksplittingmax: i32,
}

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
fn test_zopfli_deflate_output_matches_reference() {
    let input = b"hello zopfli test";

    let mut opts = ZopfliOptions {
        verbose: 0,
        verbose_more: 0,
        numiterations: 0,
        blocksplitting: 0,
        blocksplittinglast: 0,
        blocksplittingmax: 0,
    };

    unsafe {
        // Initialize defaults using the translated API (uses field accessor shims).
        ZopfliInitOptions(&mut opts as *mut _ as *mut crate::types::ZopfliOptions);
        opts.numiterations = 5;

        let mut out: *mut u8 = ptr::null_mut();
        let mut outsize: crate::types::size_t = 0;

        ZopfliCompress(
            &opts as *const _ as *const crate::types::ZopfliOptions,
            crate::types::__c2r_tu_types_src_zopfli_bin::ZOPFLI_FORMAT_DEFLATE,
            input.as_ptr(),
            input.len() as crate::types::size_t,
            &mut out,
            &mut outsize,
        );

        assert!(!out.is_null());
        assert_eq!(outsize as usize, 19);

        let out_slice = std::slice::from_raw_parts(out, outsize as usize);
        let hex = to_hex(out_slice);
        assert_eq!(hex, "cb48cdc9c957a8ca2f48cbc95428492d2e0100");

        libc::free(out as *mut _);
    }
}

