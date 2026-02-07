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
            rootHashLen,
        );
        if sumCount == 0 {
            return false;
        }
        
        let sumOfChunksLen = 5 + sumCount * rootHashLen;
        let mut chunkDigest = crate::types::HapBuf {
            buffer: std::ptr::null_mut(),
            len: 0,
        };
        
        if !crate::src_app_centraldirectory::CreateHapBuffer(&mut chunkDigest, sumOfChunksLen) {
            return false;
        }
        
        crate::src_app_centraldirectory::HapPutByte(&chunkDigest as *const crate::types::HapBuf, 0, 0x5a as ::core::ffi::c_char);
        crate::src_app_centraldirectory::HapSetInt32(&chunkDigest as *const crate::types::HapBuf, 1, sumCount);
        
        let mut offset: i32 = 5;
        
        let mut ret = crate::src_app_verify_hap::ComputerFileHash(signInfo, digestAlgorithm, fp, &chunkDigest as *const crate::types::HapBuf, &mut offset);
        if ret != crate::types::V_OK as i32 {
            crate::src_app_centraldirectory::ClearHapBuffer(&mut chunkDigest);
            return false;
        }
        
        ret = crate::src_app_verify_hap::ComputerCoreDirHash(signInfo, digestAlgorithm, fp, &chunkDigest as *const crate::types::HapBuf, &mut offset);
        if ret != crate::types::V_OK as i32 {
            crate::src_app_centraldirectory::ClearHapBuffer(&mut chunkDigest);
            return false;
        }
        
        ret = crate::src_app_verify_hap::ComputerEocdHash(signInfo, digestAlgorithm, fp, &chunkDigest as *const crate::types::HapBuf, &mut offset);
        if ret != crate::types::V_OK as i32 {
            crate::src_app_centraldirectory::ClearHapBuffer(&mut chunkDigest);
            return false;
        }
        
        ret = crate::src_app_verify_hap::ComputeDigestsWithOptionalBlock(digestAlgorithm, fp, signInfo, &chunkDigest as *const crate::types::HapBuf, actualDigest);
        if ret != crate::types::V_OK as i32 {
            crate::src_app_centraldirectory::ClearHapBuffer(&mut chunkDigest);
            return false;
        }
        
        crate::src_app_centraldirectory::ClearHapBuffer(&mut chunkDigest);
        true
    }
}