// simcrat 测试文件 for zopfli
//
// 目标：做“输出正确性”测试（而不是仅编译检查）。
// 参考输出与 evolc2rust/c2saferrust 一致：
// input="hello zopfli test", format=deflate, numiterations=5 => 固定字节序列。

use crate::*;

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

    // simcrat 版本把 options 编码到 usize 位域里；不同位置的位域在实现中存在不一致，
    // 这里同时设置两处常见的 iterations 位域，以尽量对齐上游语义。
    let mut options = zopfli_init_options();
    options = (options & !(0xFFusize << 8)) | (5usize << 8);
    options = (options & !(0xFFusize << 16)) | (5usize << 16);

    let out = zopfli_compress(&options, 2 /* deflate */, input).expect("zopfli_compress failed");
    assert_eq!(out.len(), 19);

    let hex = to_hex(&out);
    assert_eq!(hex, "cb48cdc9c957a8ca2f48cbc95428492d2e0100");
}

