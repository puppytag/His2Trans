pub extern "C" fn VerifyIntegrityChunk(digestAlgorithm: i32, fp: i32, signInfo: *const crate::types::SignatureInfo, actualDigest: *const crate::types::HapBuf) -> bool {
    if signInfo.is_null() || actualDigest.is_null() {
        return false;
    }
    let actual_digest = unsafe { &*actualDigest };
    if actual_digest.buffer.is_null() {
        return false;
    }
    let sign_info = unsafe { &*signInfo };
    let central_dir_size = sign_info.hapEocdOffset - sign_info.hapCoreDirOffset;
    let root_hash_len = crate::src_app_verify::GetHashUnitLen(digestAlgorithm);
    if root_hash_len < 0 {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: alg error\0".as_ptr() as *const i8, "VerifyIntegrityChunk\0".as_ptr() as *const i8, 304);
        return false;
    }
    let sum_count = crate::src_app_verify_hap::GetChunkSumCount(sign_info.fullSignBlockOffset, central_dir_size, sign_info.hapEocdSize, root_hash_len);
    if sum_count == 0 {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: sum count error\0".as_ptr() as *const i8, "VerifyIntegrityChunk\0".as_ptr() as *const i8, 310);
        return false;
    }
    let sum_of_chunks_len = 5 + sum_count * root_hash_len;
    let mut chunk_digest = crate::types::HapBuf {
        buffer: std::ptr::null_mut(),
        len: 0,
    };
    if !crate::src_app_centraldirectory::CreateHapBuffer(&mut chunk_digest as *mut crate::types::HapBuf, sum_of_chunks_len) {
        return false;
    }
    let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_INFO as u32, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: alg: %d\0".as_ptr() as *const i8, "VerifyIntegrityChunk\0".as_ptr() as *const i8, 318, digestAlgorithm);
    crate::src_app_centraldirectory::HapPutByte(&chunk_digest as *const crate::types::HapBuf, 0, 0x5a as i8);
    crate::src_app_centraldirectory::HapSetInt32(&chunk_digest as *const crate::types::HapBuf, 1, sum_count);
    let mut offset = 5;
    let mut ret;
    ret = crate::src_app_verify_hap::ComputerFileHash(signInfo, digestAlgorithm, fp, &chunk_digest as *const crate::types::HapBuf, &mut offset as *mut i32);
    if ret != crate::types::V_OK as i32 {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, "VerifyIntegrityChunk\0".as_ptr() as *const i8, 324);
        crate::src_app_centraldirectory::ClearHapBuffer(&mut chunk_digest as *mut crate::types::HapBuf);
        return false;
    }
    ret = crate::src_app_verify_hap::ComputerCoreDirHash(signInfo, digestAlgorithm, fp, &chunk_digest as *const crate::types::HapBuf, &mut offset as *mut i32);
    if ret != crate::types::V_OK as i32 {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, "VerifyIntegrityChunk\0".as_ptr() as *const i8, 326);
        crate::src_app_centraldirectory::ClearHapBuffer(&mut chunk_digest as *mut crate::types::HapBuf);
        return false;
    }
    ret = crate::src_app_verify_hap::ComputerEocdHash(signInfo, digestAlgorithm, fp, &chunk_digest as *const crate::types::HapBuf, &mut offset as *mut i32);
    if ret != crate::types::V_OK as i32 {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, "VerifyIntegrityChunk\0".as_ptr() as *const i8, 328);
        crate::src_app_centraldirectory::ClearHapBuffer(&mut chunk_digest as *mut crate::types::HapBuf);
        return false;
    }
    ret = crate::src_app_verify_hap::ComputeDigestsWithOptionalBlock(digestAlgorithm, fp, signInfo, &chunk_digest as *const crate::types::HapBuf, actualDigest);
    if ret != crate::types::V_OK as i32 {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, "VerifyIntegrityChunk\0".as_ptr() as *const i8, 330);
        crate::src_app_centraldirectory::ClearHapBuffer(&mut chunk_digest as *mut crate::types::HapBuf);
        return false;
    }
    crate::src_app_centraldirectory::ClearHapBuffer(&mut chunk_digest as *mut crate::types::HapBuf);
    let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_INFO as u32, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: finish\0".as_ptr() as *const i8, "VerifyIntegrityChunk\0".as_ptr() as *const i8, 332);
    true
}