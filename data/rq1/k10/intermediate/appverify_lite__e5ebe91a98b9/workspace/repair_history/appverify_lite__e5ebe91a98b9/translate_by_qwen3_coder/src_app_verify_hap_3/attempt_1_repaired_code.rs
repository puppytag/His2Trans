fn GetChunkSumCount(fileSize: i32, coreDirectorySize: i32, eocdSize: i32, rootHashLen: i32) -> i32 {
    let chunkSize: i32 = 1024 * 1024;
    let maxSize: i32 = 0x7fffffff - chunkSize;
    if fileSize > maxSize || coreDirectorySize > maxSize || eocdSize > maxSize {
        return 0;
    }
    let count: i32 = ((fileSize - 1 + chunkSize) / chunkSize) + ((coreDirectorySize - 1 + chunkSize) / chunkSize) +
        ((eocdSize - 1 + chunkSize) / chunkSize);
    if rootHashLen < 0 || (((0x7fffffff - 5) / count) < rootHashLen) {
        unsafe {
            let _ = crate::compat::HiLogPrint(3, 6, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: overflow count: %d, chunkDigestLen: %d\0".as_ptr() as *const i8, b"GetChunkSumCount\0".as_ptr() as *const i8, 103, count, rootHashLen);
        }
        return 0;
    }
    unsafe {
        let _ = crate::compat::HiLogPrint(3, 4, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: get sum count %d\0".as_ptr() as *const i8, b"GetChunkSumCount\0".as_ptr() as *const i8, 106, count);
    }
    return count;
}