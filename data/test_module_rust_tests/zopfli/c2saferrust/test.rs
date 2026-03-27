// c2saferrust 测试文件 for zopfli
//
// 目标：做“输出正确性”测试（而不是仅编译检查）。
// 使用上游用例（test_module/zopfli/tests/test.c）的稳定参考输出：
// ZopfliCompress(deflate, input="hello zopfli test", numiterations=5) => 固定字节序列。

use crate::zopfli_lib::{ZopfliCompress, ZopfliOptions, ZOPFLI_FORMAT_DEFLATE};
use std::ptr;

extern "C" {
    fn free(ptr: *mut core::ffi::c_void);
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
        numiterations: 5,
        blocksplitting: 1,
        blocksplittinglast: 0,
        blocksplittingmax: 15,
    };

    let mut out: *mut u8 = ptr::null_mut();
    let mut outsize: usize = 0;

    unsafe {
        ZopfliCompress(
            &opts as *const _,
            ZOPFLI_FORMAT_DEFLATE,
            input.as_ptr(),
            input.len(),
            &mut out,
            &mut outsize,
        );

        assert!(!out.is_null());
        assert_eq!(outsize, 19);

        let out_slice = std::slice::from_raw_parts(out, outsize);
        let hex = to_hex(out_slice);
        assert_eq!(hex, "cb48cdc9c957a8ca2f48cbc95428492d2e0100");

        free(out as *mut core::ffi::c_void);
    }
}

