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
    match signAlgorithm & 0x000f {
        0x01 | 0x04 => crate::types::MBEDTLS_MD_SHA256 as i32,
        0x02 | 0x05 => crate::types::MBEDTLS_MD_SHA384 as i32,
        0x03 | 0x06 => crate::types::MBEDTLS_MD_SHA512 as i32,
        _ => {
            unsafe {
                crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: signAlgorithm: %u error\0".as_ptr() as *const ::core::ffi::c_char,
                    b"GetDigestAlgorithmId\0".as_ptr() as *const ::core::ffi::c_char,
                    38i32,
                    signAlgorithm,
                );
            }
            crate::types::V_ERR as i32
        }
    }
}

fn ComputeBlockHash(block: *const std::ffi::c_char, blockLen: i32, alg: i32, result: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    unsafe {
        let mdInfo = crate::compat::mbedtls_md_info_from_type(alg as crate::types::mbedtls_md_type_t);
        if mdInfo.is_null() {
            return crate::types::V_ERR as i32;
        }
        
        let mut pos: i32 = 0;
        let mut rawBufLen: i32 = blockLen;
        
        let mdCtx = libc::malloc(core::mem::size_of::<crate::types::mbedtls_md_context_t>()) as *mut crate::types::mbedtls_md_context_t;
        if mdCtx.is_null() {
            return crate::types::V_ERR as i32;
        }
        
        while rawBufLen > 0 {
            crate::compat::mbedtls_md_init(mdCtx);
            let readLen: i32 = if rawBufLen > (1024 * 1024) { 1024 * 1024 } else { rawBufLen };
            
            let mut ret = crate::compat::mbedtls_md_setup(mdCtx, mdInfo, 0);
            if ret != crate::types::V_OK as i32 {
                crate::compat::mbedtls_md_free(mdCtx);
                libc::free(mdCtx as *mut core::ffi::c_void);
                return crate::types::V_ERR as i32;
            }
            
            let hlen: crate::types::size_t = crate::compat::mbedtls_md_get_size(mdInfo) as crate::types::size_t;
            if hlen == 0 || hlen > 64 {
                crate::compat::mbedtls_md_free(mdCtx);
                libc::free(mdCtx as *mut core::ffi::c_void);
                return crate::types::V_ERR as i32;
            }
            
            ret = crate::compat::mbedtls_md_starts(mdCtx);
            if ret != crate::types::V_OK as i32 {
                crate::compat::mbedtls_md_free(mdCtx);
                libc::free(mdCtx as *mut core::ffi::c_void);
                return crate::types::V_ERR as i32;
            }
            
            let mut chunkContentPrefix: [u8; 5] = [0xa5, 0, 0, 0, 0];
            if crate::compat::memcpy_s(
                chunkContentPrefix.as_mut_ptr().add(1) as *mut core::ffi::c_void,
                4,
                &readLen as *const i32 as *const core::ffi::c_void,
                core::mem::size_of::<i32>() as crate::types::size_t
            ) != 0 {
                crate::compat::mbedtls_md_free(mdCtx);
                libc::free(mdCtx as *mut core::ffi::c_void);
                return crate::types::V_ERR as i32;
            }
            
            ret = crate::compat::mbedtls_md_update(mdCtx, chunkContentPrefix.as_ptr(), 5);
            if ret != crate::types::V_OK as i32 {
                crate::compat::mbedtls_md_free(mdCtx);
                libc::free(mdCtx as *mut core::ffi::c_void);
                return crate::types::V_ERR as i32;
            }
            
            ret = crate::compat::mbedtls_md_update(mdCtx, (block as *const u8).offset(pos as isize), readLen as crate::types::size_t);
            if ret != crate::types::V_OK as i32 {
                crate::compat::mbedtls_md_free(mdCtx);
                libc::free(mdCtx as *mut core::ffi::c_void);
                return crate::types::V_ERR as i32;
            }
            
            rawBufLen -= readLen;
            pos += readLen;
            
            let outbuf = libc::malloc(hlen as usize) as *mut u8;
            if outbuf.is_null() {
                crate::compat::mbedtls_md_free(mdCtx);
                libc::free(mdCtx as *mut core::ffi::c_void);
                return crate::types::V_ERR as i32;
            }
            
            ret = crate::compat::mbedtls_md_finish(mdCtx, outbuf);
            crate::src_app_centraldirectory::HapPutData(result, *offset, outbuf, hlen as i32);
            *offset += hlen as i32;
            let _ = crate::compat::memset_s(outbuf as *mut core::ffi::c_void, hlen, 0, hlen);
            libc::free(outbuf as *mut core::ffi::c_void);
            
            if ret != crate::types::V_OK as i32 {
                crate::compat::mbedtls_md_free(mdCtx);
                libc::free(mdCtx as *mut core::ffi::c_void);
                return crate::types::V_ERR as i32;
            }
            crate::compat::mbedtls_md_free(mdCtx);
        }
        
        libc::free(mdCtx as *mut core::ffi::c_void);
        crate::types::V_OK as i32
    }
}

fn GetChunkSumCount(fileSize: i32, coreDirectorySize: i32, eocdSize: i32, rootHashLen: i32) -> i32 {
    let chunkSize: i32 = 1024 * 1024;
    let maxSize: i32 = 0x7fffffff - chunkSize;
    
    if fileSize > maxSize || coreDirectorySize > maxSize || eocdSize > maxSize {
        return 0;
    }
    
    let count: i32 = ((fileSize - 1 + chunkSize) / chunkSize) 
        + ((coreDirectorySize - 1 + chunkSize) / chunkSize)
        + ((eocdSize - 1 + chunkSize) / chunkSize);
    
    if rootHashLen < 0 || ((0x7fffffff - 5) / count) < rootHashLen {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: overflow count: %d, chunkDigestLen: %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"GetChunkSumCount\0".as_ptr() as *const ::core::ffi::c_char,
                103i32,
                count,
                rootHashLen,
            );
        }
        return 0;
    }
    
    unsafe {
        let _ = HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: get sum count %d\0".as_ptr() as *const ::core::ffi::c_char,
            b"GetChunkSumCount\0".as_ptr() as *const ::core::ffi::c_char,
            106i32,
            count,
        );
    }
    
    count
}

fn ComputeDigestsWithOptionalBlock(digestAlgorithm: i32, fp: i32, signInfo: *const crate::types::SignatureInfo, chunkDigest: *const crate::types::HapBuf, fianlDigest: *const crate::types::HapBuf) -> i32 {
    let mut rst: i32 = crate::types::V_ERR as i32;
    let mut rawBuf: *mut ::core::ffi::c_char = std::ptr::null_mut();
    let mut outbuf: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let rootHashLen = crate::src_app_verify::GetHashUnitLen(digestAlgorithm);
    
    if rootHashLen <= 0 || rootHashLen > crate::types::MAX_HASH_SIZE as i32 {
        return rst;
    }
    
    let mdInfo = unsafe { mbedtls_md_info_from_type(digestAlgorithm as crate::types::mbedtls_md_type_t) };
    if mdInfo.is_null() {
        return crate::types::V_ERR as i32;
    }
    
    let mdCtx = unsafe { libc::malloc(std::mem::size_of::<crate::types::mbedtls_md_context_t>()) } as *mut crate::types::mbedtls_md_context_t;
    if mdCtx.is_null() {
        return crate::types::V_ERR as i32;
    }
    
    unsafe { mbedtls_md_init(mdCtx) };
    let mut ret = unsafe { mbedtls_md_setup(mdCtx, mdInfo, 0) };
    let mut rawLen: i32 = 0;
    let mut blockHead: crate::types::BlockHead = unsafe { std::mem::zeroed() };
    
    'exit: loop {
        if ret != crate::types::V_OK as i32 {
            break 'exit;
        }
        
        ret = unsafe { mbedtls_md_starts(mdCtx) };
        if ret != crate::types::V_OK as i32 {
            break 'exit;
        }
        
        let readLen = unsafe { (*chunkDigest).len };
        ret = unsafe { mbedtls_md_update(mdCtx, (*chunkDigest).buffer as *const ::core::ffi::c_uchar, readLen as u32) };
        if ret != crate::types::V_OK as i32 {
            break 'exit;
        }
        
        rawBuf = crate::src_app_verify::GetSignBlockByType(signInfo, fp, crate::types::PROFILE_BLOCK_WITHSIGN_TYPE as i32, &mut rawLen, &mut blockHead);
        if rawBuf.is_null() {
            break 'exit;
        }
        
        let readLen2 = rawLen;
        ret = unsafe { mbedtls_md_update(mdCtx, rawBuf as *const ::core::ffi::c_uchar, readLen2 as u32) };
        if ret != crate::types::V_OK as i32 {
            break 'exit;
        }
        
        outbuf = unsafe { libc::malloc(rootHashLen as usize) } as *mut ::core::ffi::c_uchar;
        if outbuf.is_null() {
            break 'exit;
        }
        
        ret = unsafe { mbedtls_md_finish(mdCtx, outbuf) };
        if ret != crate::types::V_OK as i32 {
            break 'exit;
        }
        
        crate::src_app_centraldirectory::HapPutData(fianlDigest, 0, outbuf, rootHashLen);
        let _ = unsafe { memset_s(outbuf as *mut ::core::ffi::c_void, rootHashLen as u32, 0, rootHashLen as u32) };
        rst = crate::types::V_OK as i32;
        break 'exit;
    }
    
    unsafe { mbedtls_md_free(mdCtx) };
    if !mdCtx.is_null() {
        unsafe { libc::free(mdCtx as *mut ::core::ffi::c_void) };
    }
    if !rawBuf.is_null() {
        unsafe { libc::free(rawBuf as *mut ::core::ffi::c_void) };
    }
    if !outbuf.is_null() {
        unsafe { libc::free(outbuf as *mut ::core::ffi::c_void) };
    }
    
    rst
}

fn HapUpdateDigistHead(digestAlgorithm: i32, mdCtx: *mut crate::types::mbedtls_md_context_t, mdInfo: *const crate::types::mbedtls_md_info_t, readLen: i32, hlen: *mut crate::types::size_t) -> i32 {
    use crate::compat::*;
    use crate::types::*;
    
    unsafe {
        mbedtls_md_init(mdCtx);
        let ret = mbedtls_md_setup(mdCtx, mdInfo, 0);
        if ret != 0 {
            return V_ERR as i32;
        }
        
        *hlen = mbedtls_md_get_size(mdInfo) as size_t;
        if *hlen == 0 || *hlen > MAX_HASH_SIZE as size_t {
            return V_ERR as i32;
        }
        
        let ret = mbedtls_md_starts(mdCtx);
        if ret != 0 {
            return V_ERR as i32;
        }
        
        let mut chunkContentPrefix: [u8; 5] = [HAP_SECOND_LEVEL_CHUNK_PREFIX as u8, 0, 0, 0, 0];
        
        let result = memcpy_s(
            chunkContentPrefix.as_mut_ptr().add(1) as *mut ::core::ffi::c_void,
            (HAP_DIGEST_PRIFIX_LEN - 1) as u32,
            &readLen as *const i32 as *const ::core::ffi::c_void,
            std::mem::size_of::<i32>() as u32
        );
        if result != EOK as i32 {
            return V_ERR as i32;
        }
        
        let ret = mbedtls_md_update(mdCtx, chunkContentPrefix.as_ptr(), HAP_DIGEST_PRIFIX_LEN as u32);
        if ret != 0 {
            return V_ERR as i32;
        }
        
        V_OK as i32
    }
}

fn UpdateSmallBlock(readLen: i32, fp: i32, mdCtx: *mut crate::types::mbedtls_md_context_t) -> i32 {
    let mut readLenLeft: i32 = readLen;
    while readLenLeft > 0 {
        let onceRead: i32 = if readLenLeft > (1024 * 64) { 1024 * 64 } else { readLenLeft };
        let onceBuf: *mut u8 = unsafe { libc::malloc(onceRead as usize) as *mut u8 };
        if onceBuf.is_null() {
            unsafe {
                crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: \"onceBuf\" is null\0".as_ptr() as *const i8,
                    b"UpdateSmallBlock\0".as_ptr() as *const i8,
                    193i32,
                );
            }
            return crate::types::V_ERR as i32;
        }
        let len: isize = unsafe { libc::read(fp, onceBuf as *mut core::ffi::c_void, (core::mem::size_of::<u8>() * onceRead as usize) as usize) };
        if len as i32 != onceRead {
            unsafe {
                crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: \"fread err: %d, %d\0".as_ptr() as *const i8,
                    b"UpdateSmallBlock\0".as_ptr() as *const i8,
                    196i32,
                    len as i32,
                    onceRead,
                );
                libc::free(onceBuf as *mut core::ffi::c_void);
            }
            return crate::types::V_ERR as i32;
        }
        let ret: i32 = unsafe { crate::compat::mbedtls_md_update(mdCtx, onceBuf, onceRead as u32) };
        unsafe {
            libc::free(onceBuf as *mut core::ffi::c_void);
        }
        if ret != crate::types::V_OK as i32 {
            unsafe {
                crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const i8,
                    b"UpdateSmallBlock\0".as_ptr() as *const i8,
                    202i32,
                );
            }
            return ret;
        }
        readLenLeft -= onceRead;
    }
    crate::types::V_OK as i32
}

fn ComputerFileHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    unsafe {
        let mdCtx = libc::malloc(std::mem::size_of::<crate::types::mbedtls_md_context_t>()) as *mut crate::types::mbedtls_md_context_t;
        if mdCtx.is_null() {
            HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: mdCtx is null\0".as_ptr() as *const i8, b"ComputerFileHash\0".as_ptr() as *const i8, 212i32);
            return crate::types::V_ERR as i32;
        }
        
        libc::lseek(fp, 0, crate::types::SEEK_SET as i32);
        let mut pos: i32 = 0;
        let mut rawBufLen: i32 = (*signInfo).fullSignBlockOffset;
        
        while rawBufLen > 0 {
            let mut hlen: crate::types::size_t = 0;
            let readLen: i32 = if rawBufLen > (1024 * 1024) { 1024 * 1024 } else { rawBufLen };
            let mdInfo = mbedtls_md_info_from_type(digestAlgorithm as crate::types::mbedtls_md_type_t);
            if mdInfo.is_null() {
                if !mdCtx.is_null() {
                    libc::free(mdCtx as *mut ::core::ffi::c_void);
                }
                return crate::types::V_ERR as i32;
            }
            
            let mut ret = crate::src_app_verify_hap::HapUpdateDigistHead(digestAlgorithm, mdCtx, mdInfo, readLen, &mut hlen);
            if ret != crate::types::V_OK as i32 {
                HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: ret not ok\0".as_ptr() as *const i8, b"ComputerFileHash\0".as_ptr() as *const i8, 225i32);
                mbedtls_md_free(mdCtx);
                if !mdCtx.is_null() {
                    libc::free(mdCtx as *mut ::core::ffi::c_void);
                }
                return crate::types::V_ERR as i32;
            }
            
            HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: content: %d, %d\0".as_ptr() as *const i8, b"ComputerFileHash\0".as_ptr() as *const i8, 226i32, rawBufLen, pos);
            
            ret = crate::src_app_verify_hap::UpdateSmallBlock(readLen, fp, mdCtx);
            if ret != crate::types::V_OK as i32 {
                HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: ret not ok\0".as_ptr() as *const i8, b"ComputerFileHash\0".as_ptr() as *const i8, 228i32);
                mbedtls_md_free(mdCtx);
                if !mdCtx.is_null() {
                    libc::free(mdCtx as *mut ::core::ffi::c_void);
                }
                return crate::types::V_ERR as i32;
            }
            
            rawBufLen -= readLen;
            pos += readLen;
            
            let outbuf = libc::malloc(hlen as usize) as *mut u8;
            if outbuf.is_null() {
                HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: outbuf is null\0".as_ptr() as *const i8, b"ComputerFileHash\0".as_ptr() as *const i8, 232i32);
                mbedtls_md_free(mdCtx);
                if !mdCtx.is_null() {
                    libc::free(mdCtx as *mut ::core::ffi::c_void);
                }
                return crate::types::V_ERR as i32;
            }
            
            ret = mbedtls_md_finish(mdCtx, outbuf);
            crate::src_app_centraldirectory::HapPutData(chunkDigest, *offset, outbuf, hlen as i32);
            let _ = memset_s(outbuf as *mut ::core::ffi::c_void, hlen as crate::types::size_t, 0, hlen as crate::types::size_t);
            *offset += hlen as i32;
            
            if !outbuf.is_null() {
                libc::free(outbuf as *mut ::core::ffi::c_void);
            }
            
            if ret != crate::types::V_OK as i32 {
                HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: ret not ok\0".as_ptr() as *const i8, b"ComputerFileHash\0".as_ptr() as *const i8, 238i32);
                mbedtls_md_free(mdCtx);
                if !mdCtx.is_null() {
                    libc::free(mdCtx as *mut ::core::ffi::c_void);
                }
                return crate::types::V_ERR as i32;
            }
            mbedtls_md_free(mdCtx);
        }
        
        if !mdCtx.is_null() {
            libc::free(mdCtx as *mut ::core::ffi::c_void);
        }
        crate::types::V_OK as i32
    }
}

fn ComputerCoreDirHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    unsafe {
        let centralDirSize = (*signInfo).hapEocdOffset - (*signInfo).hapCoreDirOffset;
        if centralDirSize <= 0 {
            return crate::types::V_ERR as i32;
        }
        let dirBuf = libc::malloc(centralDirSize as usize) as *mut std::ffi::c_char;
        if dirBuf.is_null() {
            return crate::types::V_ERR as i32;
        }
        libc::lseek(fp, (*signInfo).hapCoreDirOffset as libc::off_t, crate::types::SEEK_SET as i32);
        let len = libc::read(fp, dirBuf as *mut std::ffi::c_void, (std::mem::size_of::<std::ffi::c_char>() * centralDirSize as usize) as usize) as i32;
        if len != centralDirSize {
            if !dirBuf.is_null() {
                libc::free(dirBuf as *mut std::ffi::c_void);
            }
            return crate::types::V_ERR as i32;
        }
        let ret = crate::src_app_verify_hap::ComputeBlockHash(dirBuf as *const std::ffi::c_char, centralDirSize, digestAlgorithm, chunkDigest, offset);
        let _ = crate::compat::memset_s(dirBuf as *mut std::ffi::c_void, centralDirSize as u32, 0, centralDirSize as u32);
        if !dirBuf.is_null() {
            libc::free(dirBuf as *mut std::ffi::c_void);
        }
        if ret != crate::types::V_OK as i32 {
            return ret;
        }
        crate::types::V_OK as i32
    }
}

fn ComputerEocdHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    unsafe {
        if (*signInfo).hapEocdSize <= 0 {
            return crate::types::V_ERR as i32;
        }
        
        let eocdBuf = libc::malloc((*signInfo).hapEocdSize as usize) as *mut crate::types::HapEocd;
        if eocdBuf.is_null() {
            return crate::types::V_ERR as i32;
        }
        
        libc::lseek(fp, (*signInfo).hapEocdOffset as libc::off_t, crate::types::SEEK_SET as i32);
        let len = libc::read(fp, eocdBuf as *mut libc::c_void, (*signInfo).hapEocdSize as usize) as i32;
        
        if len != (*signInfo).hapEocdSize {
            libc::free(eocdBuf as *mut libc::c_void);
            return crate::types::V_ERR as i32;
        }
        
        // Calculate offset to coreDirOffset field manually to avoid packed struct reference issues
        let eocd_base = eocdBuf as *mut u8;
        let core_dir_offset_ptr = eocd_base.add(std::mem::offset_of!(crate::types::HapEocd, eocdHead) + std::mem::offset_of!(crate::types::MinEocd, coreDirOffset));
        crate::src_app_common::HapPutInt32(core_dir_offset_ptr, std::mem::size_of::<i32>() as i32, (*signInfo).fullSignBlockOffset);
        
        let ret = crate::src_app_verify_hap::ComputeBlockHash(
            eocdBuf as *const std::ffi::c_char,
            len,
            digestAlgorithm,
            chunkDigest,
            offset
        );
        
        let _ = memset_s(
            eocdBuf as *mut core::ffi::c_void,
            (*signInfo).hapEocdSize as u32,
            0,
            (*signInfo).hapEocdSize as u32
        );
        
        libc::free(eocdBuf as *mut libc::c_void);
        
        if ret != crate::types::V_OK as i32 {
            return ret;
        }
        
        crate::types::V_OK as i32
    }
}

pub extern "C" fn VerifyIntegrityChunk(digestAlgorithm: i32, fp: i32, signInfo: *const crate::types::SignatureInfo, actualDigest: *const crate::types::HapBuf) -> bool {
    unsafe {
        if signInfo.is_null() || actualDigest.is_null() || (*actualDigest).buffer.is_null() {
            return false;
        }
        
        let centralDirSize = (*signInfo).hapEocdOffset - (*signInfo).hapCoreDirOffset;
        let rootHashLen = crate::src_app_verify::GetHashUnitLen(digestAlgorithm);
        if rootHashLen < 0 {
            return false;
        }
        
        let sumCount = crate::src_app_verify_hap::GetChunkSumCount(
            (*signInfo).fullSignBlockOffset,
            centralDirSize,
            (*signInfo).hapEocdSize,
            rootHashLen
        );
        if sumCount == 0 {
            return false;
        }
        
        let sumOfChunksLen = 5 + sumCount * rootHashLen;
        let mut chunkDigest: crate::types::HapBuf = core::mem::zeroed();
        if !crate::src_app_centraldirectory::CreateHapBuffer(&mut chunkDigest, sumOfChunksLen) {
            return false;
        }
        
        crate::src_app_centraldirectory::HapPutByte(&chunkDigest, 0, 0x5a as ::core::ffi::c_char);
        crate::src_app_centraldirectory::HapSetInt32(&chunkDigest, 1, sumCount);
        
        let mut offset: i32 = 5;
        let mut ret: i32;
        
        ret = crate::src_app_verify_hap::ComputerFileHash(signInfo, digestAlgorithm, fp, &chunkDigest, &mut offset);
        if ret != crate::types::V_OK as i32 {
            crate::src_app_centraldirectory::ClearHapBuffer(&mut chunkDigest);
            return false;
        }
        
        ret = crate::src_app_verify_hap::ComputerCoreDirHash(signInfo, digestAlgorithm, fp, &chunkDigest, &mut offset);
        if ret != crate::types::V_OK as i32 {
            crate::src_app_centraldirectory::ClearHapBuffer(&mut chunkDigest);
            return false;
        }
        
        ret = crate::src_app_verify_hap::ComputerEocdHash(signInfo, digestAlgorithm, fp, &chunkDigest, &mut offset);
        if ret != crate::types::V_OK as i32 {
            crate::src_app_centraldirectory::ClearHapBuffer(&mut chunkDigest);
            return false;
        }
        
        ret = crate::src_app_verify_hap::ComputeDigestsWithOptionalBlock(digestAlgorithm, fp, signInfo, &chunkDigest, actualDigest);
        if ret != crate::types::V_OK as i32 {
            crate::src_app_centraldirectory::ClearHapBuffer(&mut chunkDigest);
            return false;
        }
        
        crate::src_app_centraldirectory::ClearHapBuffer(&mut chunkDigest);
        true
    }
}
