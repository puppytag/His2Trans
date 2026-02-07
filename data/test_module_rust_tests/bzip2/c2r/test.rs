// C2R 框架测试文件 for bzip2
//
// 目标：做“输出正确性”测试（而不是仅编译检查）。
// 最小可验证语义：压缩输出应是合法 bzip2 流（以 "BZh" 开头 + blockSize100k），
// 并且 roundtrip: decompress(compress(x)) == x。

use crate::src_bzlib::{
    BZ2_bzBuffToBuffCompress, BZ2_bzBuffToBuffDecompress, BZ2_bzlibVersion,
};

use std::ffi::CStr;

#[test]
fn test_bzip2_roundtrip_and_header() {
    // Version should be a non-empty C string.
    unsafe {
        let ver = BZ2_bzlibVersion();
        assert!(!ver.is_null());
        let ver = CStr::from_ptr(ver).to_str().unwrap();
        assert!(!ver.is_empty());
    }

    let src = b"hello bzip2 test";
    let src_len: libc::c_uint = src.len() as libc::c_uint;

    // Same heuristic as upstream test.c: src + src/100 + 601.
    let extra: libc::c_uint = 601;
    let comp_cap: libc::c_uint = src_len + (src_len / 100) + extra;
    let mut comp = vec![0u8; comp_cap as usize];
    let mut comp_len: libc::c_uint = comp_cap;

    let rc = unsafe {
        BZ2_bzBuffToBuffCompress(
            comp.as_mut_ptr().cast::<libc::c_char>(),
            &mut comp_len,
            src.as_ptr() as *mut libc::c_char,
            src_len,
            9,  // blockSize100k
            0,  // verbosity
            30, // workFactor
        )
    };
    assert_eq!(rc, 0, "BZ2_bzBuffToBuffCompress should return BZ_OK");
    assert!(comp_len as usize <= comp.len());

    // bzip2 stream header: "BZh" + blockSize100k digit.
    assert!(comp_len >= 4);
    assert_eq!(&comp[..3], b"BZh", "bzip2 header prefix mismatch");
    assert_eq!(comp[3], b'9', "bzip2 block size header mismatch");

    let mut dec = vec![0u8; src.len() + 16];
    let mut dec_len: libc::c_uint = dec.len() as libc::c_uint;
    let rc = unsafe {
        BZ2_bzBuffToBuffDecompress(
            dec.as_mut_ptr().cast::<libc::c_char>(),
            &mut dec_len,
            comp.as_mut_ptr().cast::<libc::c_char>(),
            comp_len,
            0, // small
            0, // verbosity
        )
    };
    assert_eq!(rc, 0, "BZ2_bzBuffToBuffDecompress should return BZ_OK");
    assert_eq!(dec_len as usize, src.len());
    assert_eq!(&dec[..src.len()], src);
}

