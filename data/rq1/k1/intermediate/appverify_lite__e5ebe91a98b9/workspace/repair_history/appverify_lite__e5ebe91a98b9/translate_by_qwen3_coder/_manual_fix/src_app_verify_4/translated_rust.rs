fn GetSignHead(file: *const crate::types::FileRead, signInfo: *mut crate::types::SignatureInfo) -> i32 {
    use crate::src_app_centraldirectory::FindSignature;
    use crate::src_app_verify::SignHeadN2H;
    let mut fileSt: crate::compat::stat = unsafe { std::mem::zeroed() };
    let ret = unsafe { crate::compat::fstat((*file).fp, &mut fileSt) };
    let sign_head_size = std::mem::size_of::<crate::types::HwSignHead>() as i64;
    if ret != 0 || fileSt.st_size < sign_head_size {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: fstat error, %d, filelen: %d\0".as_ptr() as *const _,
                b"GetSignHead\0".as_ptr() as *const _,
                121,
                ret,
                fileSt.st_size as i32,
            )
        };
        return crate::types::V_ERR_GET_SIGNHEAD as i32;
    }
    if !FindSignature(file, signInfo) {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: find signature error\0".as_ptr() as *const _,
                b"GetSignHead\0".as_ptr() as *const _,
                125,
            )
        };
        return crate::types::V_ERR_GET_SIGNHEAD as i32;
    }
    let hapCoreDirOffset = unsafe { (*signInfo).hapCoreDirOffset };
    if hapCoreDirOffset < sign_head_size as i32 {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: hapCoreDirOffset error, %d\0".as_ptr() as *const _,
                b"GetSignHead\0".as_ptr() as *const _,
                129,
                hapCoreDirOffset,
            )
        };
        return crate::types::V_ERR_GET_SIGNHEAD as i32;
    }
    let ret = unsafe {
        crate::compat::lseek(
            (*file).fp,
            hapCoreDirOffset as i64 - sign_head_size,
            0,
        )
    };
    if ret < 0 {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: lseek error, %d\0".as_ptr() as *const _,
                b"GetSignHead\0".as_ptr() as *const _,
                134,
                ret as i32,
            )
        };
        return crate::types::V_ERR_GET_SIGNHEAD as i32;
    }
    let signHead = unsafe { libc::malloc(std::mem::size_of::<crate::types::HwSignHead>()) } as *mut crate::types::HwSignHead;
    if signHead.is_null() {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: signHead is null\0".as_ptr() as *const _,
                b"GetSignHead\0".as_ptr() as *const _,
                138,
            )
        };
        return crate::types::V_ERR as i32;
    }
    let readLen = unsafe {
        crate::compat::read(
            (*file).fp,
            signHead as *mut std::ffi::c_void,
            std::mem::size_of::<crate::types::HwSignHead>() as u64,
        )
    };
    if readLen as i64 != sign_head_size {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: readLen %d, %d\0".as_ptr() as *const _,
                b"GetSignHead\0".as_ptr() as *const _,
                141,
                readLen as i32,
                sign_head_size as i32,
            )
        };
        unsafe { libc::free(signHead as *mut std::ffi::c_void) };
        return crate::types::V_ERR_GET_SIGNHEAD as i32;
    }
    SignHeadN2H(signHead);
    let mut magicLow = crate::types::HAP_SIG_BLOCK_MAGIC_LO;
    let mut magicHigh = crate::types::HAP_SIG_BLOCK_MAGIC_HI;
    if unsafe { (*signHead).version } < crate::types::VERSION_FOR_NEW_MAGIC_NUM {
        magicLow = crate::types::HAP_SIG_BLOCK_MAGIC_LO_OLD;
        magicHigh = crate::types::HAP_SIG_BLOCK_MAGIC_HI_OLD;
    }
    if unsafe { (*signHead).magicLow } != magicLow || unsafe { (*signHead).magicHigh } != magicHigh {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: sign head magic invalid\0".as_ptr() as *const _,
                b"GetSignHead\0".as_ptr() as *const _,
                153,
            )
        };
        unsafe { libc::free(signHead as *mut std::ffi::c_void) };
        return crate::types::V_ERR_GET_SIGNHEAD as i32;
    }
    let _ = unsafe {
        crate::compat::HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_INFO as u32,
            0xD001100,
            b"appverify\0".as_ptr() as *const _,
            b"[%s:%d]: sign head: size: %llu, blockNum:0x%x\0".as_ptr() as *const _,
            b"GetSignHead\0".as_ptr() as *const _,
            157,
            unsafe { (*signHead).size } as u64,
            unsafe { (*signHead).blockNum } as u32,
        )
    };
    unsafe { (*signInfo).signHead = signHead };
    unsafe { (*signInfo).fullSignBlockOffset = hapCoreDirOffset - (*signHead).size as i32 };
    unsafe { (*signInfo).fileSize = fileSt.st_size as i32 };
    let fullSignBlockOffset = unsafe { (*signInfo).fullSignBlockOffset };
    if fullSignBlockOffset <= 0 || fullSignBlockOffset >= hapCoreDirOffset {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: fullSignBlockOffset invalid\0".as_ptr() as *const _,
                b"GetSignHead\0".as_ptr() as *const _,
                162,
            )
        };
        unsafe { libc::free(signHead as *mut std::ffi::c_void) };
        return crate::types::V_ERR_GET_SIGNHEAD as i32;
    }
    crate::types::V_OK as i32
}