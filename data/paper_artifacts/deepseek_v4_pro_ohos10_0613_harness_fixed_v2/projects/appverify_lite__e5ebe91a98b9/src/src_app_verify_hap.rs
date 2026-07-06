//! Module: src_app_verify_hap
//!
//! Auto-generated skeleton - function bodies are unimplemented.

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::types::*;
use crate::globals::*;
use crate::compat::*;

pub extern "C" fn GetDigestAlgorithmId(signAlgorithm: u32) -> i32 {
    match signAlgorithm & crate::types::ALGORITHM_MASK {
        0x01 | 0x04 => crate::types::MBEDTLS_MD_SHA256 as i32,
        0x02 | 0x05 => crate::types::MBEDTLS_MD_SHA384 as i32,
        0x03 | 0x06 => crate::types::MBEDTLS_MD_SHA512 as i32,
        _ => crate::types::V_ERR as i32,
    }
}

fn ComputeBlockHash(block: *const std::ffi::c_char, blockLen: i32, alg: i32, result: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    if block.is_null() || result.is_null() || offset.is_null() {
        return V_ERR as i32;
    }
    let md_info = unsafe { mbedtls_md_info_from_type(alg as crate::types::mbedtls_md_type_t) };
    if md_info.is_null() {
        return V_ERR as i32;
    }
    let mut pos: i32 = 0;
    let mut raw_buf_len = blockLen;
    let md_ctx_size = std::mem::size_of::<crate::types::mbedtls_md_context_t>();
    let md_ctx = unsafe { libc::malloc(md_ctx_size as usize) as *mut crate::types::mbedtls_md_context_t };
    if md_ctx.is_null() {
        return V_ERR as i32;
    }
    let mut err = false;
    while raw_buf_len > 0 && !err {
        unsafe { mbedtls_md_init(md_ctx); }
        let read_len = std::cmp::min(raw_buf_len, HASH_BLOB_LEN as i32);
        let setup_ret = unsafe { mbedtls_md_setup(md_ctx, md_info, 0) };
        if setup_ret != V_OK as i32 {
            err = true;
            break;
        }
        let hlen = unsafe { mbedtls_md_get_size(md_info) } as crate::types::size_t;
        if hlen == 0 || hlen > MAX_HASH_SIZE as crate::types::size_t {
            err = true;
            break;
        }
        let starts_ret = unsafe { mbedtls_md_starts(md_ctx) };
        if starts_ret != V_OK as i32 {
            err = true;
            break;
        }
        let mut chunk_content_prefix: [u8; 5] = [0xa5, 0, 0, 0, 0];
        chunk_content_prefix[1..5].copy_from_slice(&read_len.to_be_bytes());
        let update1_ret = unsafe { mbedtls_md_update(md_ctx, chunk_content_prefix.as_ptr(), 5 as crate::types::size_t) };
        if update1_ret != V_OK as i32 {
            err = true;
            break;
        }
        let data = unsafe { (block as *const u8).add(pos as usize) };
        let update2_ret = unsafe { mbedtls_md_update(md_ctx, data, read_len as crate::types::size_t) };
        if update2_ret != V_OK as i32 {
            err = true;
            break;
        }
        raw_buf_len -= read_len;
        pos += read_len;
        let outbuf = unsafe { libc::malloc(hlen as usize) as *mut u8 };
        if outbuf.is_null() {
            err = true;
            break;
        }
        let finish_ret = unsafe { mbedtls_md_finish(md_ctx, outbuf) };
        if finish_ret != V_OK as i32 {
            unsafe { std::ptr::write_bytes(outbuf, 0, hlen as usize); }
            unsafe { libc::free(outbuf as *mut ::core::ffi::c_void); }
            err = true;
            break;
        }
        let current_offset = unsafe { *offset };
        crate::src_app_centraldirectory::HapPutData(result, current_offset, outbuf as *const u8, hlen as i32);
        unsafe { *offset += hlen as i32; }
        unsafe { std::ptr::write_bytes(outbuf, 0, hlen as usize); }
        unsafe { libc::free(outbuf as *mut ::core::ffi::c_void); }
        unsafe { mbedtls_md_free(md_ctx); }
    }
    if err {
        unsafe { mbedtls_md_free(md_ctx); }
        unsafe { libc::free(md_ctx as *mut ::core::ffi::c_void); }
        return V_ERR as i32;
    } else {
        unsafe { libc::free(md_ctx as *mut ::core::ffi::c_void); }
        return V_OK as i32;
    }
}

fn GetChunkSumCount(fileSize: i32, coreDirectorySize: i32, eocdSize: i32, rootHashLen: i32)-> i32 {
    let chunk_size: i32 = 1024 * 1024;
    let max_size: i32 = 0x7fffffff - chunk_size;
    if fileSize > max_size || coreDirectorySize > max_size || eocdSize > max_size {
        return 0;
    }
    let count = (fileSize - 1 + chunk_size) / chunk_size
        + (coreDirectorySize - 1 + chunk_size) / chunk_size
        + (eocdSize - 1 + chunk_size) / chunk_size;
    if count == 0 || rootHashLen < 0 || (0x7fffffff - 5) / count < rootHashLen {
        // HiLogPrint not available in this context; side-effect (logging) omitted
        return 0;
    }
    // HiLogPrint(LOG_CORE, LOG_INFO, ...) omitted
    count
}

fn ComputeDigestsWithOptionalBlock(digestAlgorithm: i32, fp: i32, signInfo: *const crate::types::SignatureInfo, chunkDigest: *const crate::types::HapBuf, fianlDigest: *const crate::types::HapBuf) -> i32 {
    let mut rst = crate::types::V_ERR as i32;
    let mut rawBuf: *mut ::core::ffi::c_char = std::ptr::null_mut();
    let mut outbuf: *mut u8 = std::ptr::null_mut();
    let rootHashLen = crate::src_app_verify::GetHashUnitLen(digestAlgorithm);
    if rootHashLen <= 0 || rootHashLen > crate::types::MAX_HASH_SIZE as i32 {
        return rst;
    }
    let mdInfo = unsafe { mbedtls_md_info_from_type(digestAlgorithm as crate::types::mbedtls_md_type_t) };
    if mdInfo.is_null() {
        return crate::types::V_ERR as i32;
    }
    let mut mdCtx = unsafe {
        libc::malloc(std::mem::size_of::<crate::types::mbedtls_md_context_t>()) as *mut crate::types::mbedtls_md_context_t
    };
    if mdCtx.is_null() {
        return crate::types::V_ERR as i32;
    }
    unsafe { mbedtls_md_init(mdCtx); }
    let mut ret = unsafe { mbedtls_md_setup(mdCtx, mdInfo, 0) };
    let mut rawLen: i32 = 0;
    let mut blockHead = crate::types::BlockHead { type_: 0, length: 0, offset: 0 };
    let mut readLen: i32 = 0;

    'cleanup: loop {
        if ret != crate::types::V_OK as i32 {
            break 'cleanup;
        }
        ret = unsafe { mbedtls_md_starts(mdCtx) };
        if ret != crate::types::V_OK as i32 {
            break 'cleanup;
        }
        readLen = unsafe { (*chunkDigest).len };
        let chunk_buf = unsafe { (*chunkDigest).buffer };
        ret = unsafe { mbedtls_md_update(mdCtx, chunk_buf as *const u8, (readLen as usize).try_into().unwrap()) };
        if ret != crate::types::V_OK as i32 {
            break 'cleanup;
        }
        rawBuf = crate::src_app_verify::GetSignBlockByType(signInfo, fp, crate::types::PROFILE_BLOCK_WITHSIGN_TYPE as i32, &mut rawLen, &mut blockHead);
        if rawBuf.is_null() {
            break 'cleanup;
        }
        readLen = rawLen;
        ret = unsafe { mbedtls_md_update(mdCtx, rawBuf as *const u8, (readLen as usize).try_into().unwrap()) };
        if ret != crate::types::V_OK as i32 {
            break 'cleanup;
        }
        outbuf = unsafe { libc::malloc(rootHashLen as usize) as *mut u8 };
        if outbuf.is_null() {
            break 'cleanup;
        }
        ret = unsafe { mbedtls_md_finish(mdCtx, outbuf) };
        if ret != crate::types::V_OK as i32 {
            break 'cleanup;
        }
        crate::src_app_centraldirectory::HapPutData(fianlDigest, 0, outbuf as *const u8, rootHashLen);
        unsafe { std::ptr::write_bytes(outbuf, 0, rootHashLen as usize); }
        rst = crate::types::V_OK as i32;
        break;
    }

    unsafe { mbedtls_md_free(mdCtx); }
    if !mdCtx.is_null() {
        unsafe { libc::free(mdCtx as *mut libc::c_void); }
    }
    if !rawBuf.is_null() {
        unsafe { libc::free(rawBuf as *mut libc::c_void); }
    }
    if !outbuf.is_null() {
        unsafe { libc::free(outbuf as *mut libc::c_void); }
    }
    rst
}

fn HapUpdateDigistHead(_digestAlgorithm: i32, mdCtx: *mut crate::types::mbedtls_md_context_t, mdInfo: *const crate::types::mbedtls_md_info_t, readLen: i32, hlen: *mut usize)-> i32 {
    unsafe { mbedtls_md_init(mdCtx); }
    let ret_setup = unsafe { mbedtls_md_setup(mdCtx, mdInfo, 0) };
    if ret_setup != 0 { return crate::types::V_ERR as i32; }
    let hlen_local = unsafe { mbedtls_md_get_size(mdInfo) as usize };
    unsafe { *hlen = hlen_local; }
    if hlen_local == 0 || hlen_local > 64 { return crate::types::V_ERR as i32; }
    let ret_starts = unsafe { mbedtls_md_starts(mdCtx) };
    if ret_starts != 0 { return crate::types::V_ERR as i32; }
    let mut chunkContentPrefix: [u8; 5] = [0xa5, 0, 0, 0, 0];
    let rl_bytes = readLen.to_ne_bytes();
    (&mut chunkContentPrefix[1..]).copy_from_slice(&rl_bytes);
    let ret_update = unsafe { mbedtls_md_update(mdCtx, chunkContentPrefix.as_ptr(), 5 as crate::types::size_t) };
    if ret_update != 0 { return crate::types::V_ERR as i32; }
    crate::types::V_OK as i32
}

fn UpdateSmallBlock(readLen: i32, fp: i32, mdCtx: *mut crate::types::mbedtls_md_context_t)-> i32 {
    extern "C" {
        fn mbedtls_md_update(
            ctx: *mut crate::types::mbedtls_md_context_t,
            input: *const u8,
            ilen: usize,
        ) -> i32;
    }
    let mut read_len_left = readLen;
    while read_len_left > 0 {
        let once_read = if read_len_left > 65536 { 65536 } else { read_len_left };
        let once_buf = unsafe { libc::malloc(once_read as usize) as *mut u8 };
        if once_buf.is_null() {
            return crate::types::V_ERR as i32;
        }
        let len = unsafe { libc::read(fp as i32, once_buf as *mut libc::c_void, once_read as usize) as i32 };
        if len != once_read {
            unsafe {
                libc::free(once_buf as *mut libc::c_void);
            }
            return crate::types::V_ERR as i32;
        }
        let ret = unsafe { mbedtls_md_update(mdCtx, once_buf, once_read as usize) };
        unsafe {
            libc::free(once_buf as *mut libc::c_void);
        }
        if ret != crate::types::V_OK as i32 {
            return ret;
        }
        read_len_left -= once_read;
    }
    crate::types::V_OK as i32
}

fn ComputerFileHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32)-> i32 {
    let mdCtx = unsafe {
        libc::malloc(core::mem::size_of::<crate::types::mbedtls_md_context_t>())
            as *mut crate::types::mbedtls_md_context_t
    };
    if mdCtx.is_null() {
        println!("[ComputerFileHash:{}] mdCtx is null", line!());
        return crate::types::V_ERR as i32;
    }
    unsafe { libc::lseek(fp, 0, libc::SEEK_SET); }
    let mut pos: i32 = 0;
    let full_sign_offset = unsafe { (*signInfo).fullSignBlockOffset };
    let mut rawBufLen = full_sign_offset;
    while rawBufLen > 0 {
        let mut hlen: usize = 0;
        let readLen = if rawBufLen > 1024 * 1024 { 1024 * 1024 } else { rawBufLen };
        let mdInfo = unsafe { mbedtls_md_info_from_type(digestAlgorithm as crate::types::mbedtls_md_type_t) };
        if mdInfo.is_null() {
            println!("[ComputerFileHash:{}] mdInfo is null", line!());
            unsafe { libc::free(mdCtx as *mut ::core::ffi::c_void); }
            return crate::types::V_ERR as i32;
        }
        let mut ret = crate::src_app_verify_hap::HapUpdateDigistHead(
            digestAlgorithm,
            mdCtx,
            mdInfo,
            readLen,
            &mut hlen,
        );
        if ret != crate::types::V_OK as i32 {
            println!("[ComputerFileHash:{}] HapUpdateDigistHead failed", line!());
            unsafe { mbedtls_md_free(mdCtx); }
            unsafe { libc::free(mdCtx as *mut ::core::ffi::c_void); }
            return crate::types::V_ERR as i32;
        }
        println!(
            "[ComputerFileHash:{}] content: {}, {}",
            line!(),
            rawBufLen,
            pos
        );
        ret = crate::src_app_verify_hap::UpdateSmallBlock(readLen, fp, mdCtx);
        if ret != crate::types::V_OK as i32 {
            println!("[ComputerFileHash:{}] UpdateSmallBlock failed", line!());
            unsafe { mbedtls_md_free(mdCtx); }
            unsafe { libc::free(mdCtx as *mut ::core::ffi::c_void); }
            return crate::types::V_ERR as i32;
        }
        rawBufLen -= readLen;
        pos += readLen;
        let outbuf = unsafe { libc::malloc(hlen) as *mut u8 };
        if outbuf.is_null() {
            println!("[ComputerFileHash:{}] outbuf is null", line!());
            unsafe { mbedtls_md_free(mdCtx); }
            unsafe { libc::free(mdCtx as *mut ::core::ffi::c_void); }
            return crate::types::V_ERR as i32;
        }
        ret = unsafe { mbedtls_md_finish(mdCtx, outbuf) };
        crate::src_app_centraldirectory::HapPutData(
            chunkDigest,
            unsafe { *offset },
            outbuf as *const u8,
            hlen as i32,
        );
        unsafe { core::ptr::write_bytes(outbuf, 0u8, hlen); }
        unsafe { *offset += hlen as i32; }
        unsafe { libc::free(outbuf as *mut ::core::ffi::c_void); }
        if ret != crate::types::V_OK as i32 {
            println!("[ComputerFileHash:{}] mbedtls_md_finish failed", line!());
            unsafe { mbedtls_md_free(mdCtx); }
            unsafe { libc::free(mdCtx as *mut ::core::ffi::c_void); }
            return crate::types::V_ERR as i32;
        }
        unsafe { mbedtls_md_free(mdCtx); }
    }
    unsafe { mbedtls_md_free(mdCtx); }
    unsafe { libc::free(mdCtx as *mut ::core::ffi::c_void); }
    crate::types::V_OK as i32
}

fn ComputerCoreDirHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32)-> i32 {
    let centralDirSize = unsafe { (*signInfo).hapEocdOffset - (*signInfo).hapCoreDirOffset };
    if centralDirSize <= 0 {
        return crate::types::V_ERR as i32;
    }
    let dirBuf: *mut std::ffi::c_char = unsafe { libc::malloc(centralDirSize as usize) as *mut std::ffi::c_char };
    if dirBuf.is_null() {
        return crate::types::V_ERR as i32;
    }
    unsafe { libc::lseek(fp as libc::c_int, (*signInfo).hapCoreDirOffset as libc::off_t, libc::SEEK_SET); }
    let len = unsafe { libc::read(fp as libc::c_int, dirBuf as *mut libc::c_void, centralDirSize as usize) as i32 };
    if len != centralDirSize {
        unsafe { libc::free(dirBuf as *mut libc::c_void); }
        return crate::types::V_ERR as i32;
    }
    let ret = crate::src_app_verify_hap::ComputeBlockHash(
        dirBuf as *const std::ffi::c_char,
        centralDirSize,
        digestAlgorithm,
        chunkDigest,
        offset,
    );
    unsafe { std::ptr::write_bytes(dirBuf, 0u8, centralDirSize as usize); }
    unsafe { libc::free(dirBuf as *mut libc::c_void); }
    if ret != crate::types::V_OK as i32 {
        return ret;
    }
    crate::types::V_OK as i32
}

fn ComputerEocdHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32)-> i32 {
    if signInfo.is_null() {
        return crate::types::V_ERR as i32;
    }
    let eocd_size = unsafe { (*signInfo).hapEocdSize };
    if eocd_size <= 0 {
        return crate::types::V_ERR as i32;
    }
    let eocd_size_usize = eocd_size as usize;
    let eocdBuf_raw = unsafe { libc::malloc(eocd_size_usize) };
    if eocdBuf_raw.is_null() {
        return crate::types::V_ERR as i32;
    }
    let eocdBuf = eocdBuf_raw as *mut crate::types::HapEocd;

    unsafe { libc::lseek(fp, (*signInfo).hapEocdOffset as i64, 0); }
    let len = unsafe { libc::read(fp, eocdBuf as *mut ::core::ffi::c_void, eocd_size_usize) as i32 };
    if len != eocd_size {
        unsafe { libc::free(eocdBuf_raw); }
        return crate::types::V_ERR as i32;
    }

    let buf_ptr = unsafe { std::ptr::addr_of_mut!((*eocdBuf).eocdHead.coreDirOffset) as *mut u8 };
    crate::src_app_common::HapPutInt32(
        buf_ptr,
        std::mem::size_of::<i32>() as i32,
        unsafe { (*signInfo).fullSignBlockOffset },
    );

    let ret = crate::src_app_verify_hap::ComputeBlockHash(
        eocdBuf as *const ::core::ffi::c_char,
        len,
        digestAlgorithm,
        chunkDigest,
        offset,
    );

    unsafe { std::ptr::write_bytes(eocdBuf_raw as *mut u8, 0u8, eocd_size_usize); }
    unsafe { libc::free(eocdBuf_raw); }

    if ret != crate::types::V_OK as i32 {
        return ret;
    }

    crate::types::V_OK as i32
}

pub extern "C" fn VerifyIntegrityChunk(digestAlgorithm: i32, fp: i32, signInfo: *const crate::types::SignatureInfo, actualDigest: *const crate::types::HapBuf) -> bool {
    if signInfo.is_null() || actualDigest.is_null() || unsafe { (*actualDigest).buffer.is_null() } {
        return false;
    }

    let central_dir_size = unsafe { (*signInfo).hapEocdOffset - (*signInfo).hapCoreDirOffset };

    let root_hash_len = crate::src_app_verify::GetHashUnitLen(digestAlgorithm);
    if root_hash_len < 0 {
        return false;
    }

    let full_sign_offset = unsafe { (*signInfo).fullSignBlockOffset };
    let hap_eocd_size = unsafe { (*signInfo).hapEocdSize };
    let sum_count = crate::src_app_verify_hap::GetChunkSumCount(
        full_sign_offset,
        central_dir_size,
        hap_eocd_size,
        root_hash_len,
    );
    if sum_count == 0 {
        return false;
    }

    let sum_of_chunks_len = 5 + sum_count * root_hash_len;

    let mut chunk_digest = crate::types::HapBuf {
        buffer: core::ptr::null_mut(),
        len: 0,
    };

    if !crate::src_app_centraldirectory::CreateHapBuffer(&mut chunk_digest as *mut crate::types::HapBuf, sum_of_chunks_len) {
        return false;
    }

    crate::src_app_centraldirectory::HapPutByte(&chunk_digest as *const crate::types::HapBuf, 0, 0x5a as ::core::ffi::c_char);
    crate::src_app_centraldirectory::HapSetInt32(&chunk_digest as *const crate::types::HapBuf, 1, sum_count);

    let mut offset: i32 = 5;

    let ret = crate::src_app_verify_hap::ComputerFileHash(
        signInfo,
        digestAlgorithm,
        fp,
        &chunk_digest as *const crate::types::HapBuf,
        &mut offset as *mut i32,
    );
    if ret != 0 {
        crate::src_app_centraldirectory::ClearHapBuffer(&mut chunk_digest as *mut crate::types::HapBuf);
        return false;
    }

    let ret = crate::src_app_verify_hap::ComputerCoreDirHash(
        signInfo,
        digestAlgorithm,
        fp,
        &chunk_digest as *const crate::types::HapBuf,
        &mut offset as *mut i32,
    );
    if ret != 0 {
        crate::src_app_centraldirectory::ClearHapBuffer(&mut chunk_digest as *mut crate::types::HapBuf);
        return false;
    }

    let ret = crate::src_app_verify_hap::ComputerEocdHash(
        signInfo,
        digestAlgorithm,
        fp,
        &chunk_digest as *const crate::types::HapBuf,
        &mut offset as *mut i32,
    );
    if ret != 0 {
        crate::src_app_centraldirectory::ClearHapBuffer(&mut chunk_digest as *mut crate::types::HapBuf);
        return false;
    }

    let ret = crate::src_app_verify_hap::ComputeDigestsWithOptionalBlock(
        digestAlgorithm,
        fp,
        signInfo,
        &chunk_digest as *const crate::types::HapBuf,
        actualDigest,
    );
    if ret != 0 {
        crate::src_app_centraldirectory::ClearHapBuffer(&mut chunk_digest as *mut crate::types::HapBuf);
        return false;
    }

    crate::src_app_centraldirectory::ClearHapBuffer(&mut chunk_digest as *mut crate::types::HapBuf);
    true
}
