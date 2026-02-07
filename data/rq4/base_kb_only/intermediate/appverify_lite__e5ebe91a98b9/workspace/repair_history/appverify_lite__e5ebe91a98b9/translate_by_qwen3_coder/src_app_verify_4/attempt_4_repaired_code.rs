fn GetSignHead(file: *const crate::types::FileRead, signInfo: *mut crate::types::SignatureInfo) -> i32 {
    use crate::types::*;
    
    unsafe {
        let mut fileSt: libc::stat = std::mem::zeroed();
        let ret = libc::fstat((*file).fp, &mut fileSt);
        if ret != 0 || fileSt.st_size < std::mem::size_of::<HwSignHead>() as i64 {
            return V_ERR_GET_SIGNHEAD as i32;
        }
        
        if !crate::src_app_centraldirectory::FindSignature(file, signInfo) {
            return V_ERR_GET_SIGNHEAD as i32;
        }
        
        if (*signInfo).hapCoreDirOffset < std::mem::size_of::<HwSignHead>() as i32 {
            return V_ERR_GET_SIGNHEAD as i32;
        }
        
        let seek_offset = (*signInfo).hapCoreDirOffset - std::mem::size_of::<HwSignHead>() as i32;
        let ret = libc::lseek((*file).fp, seek_offset as libc::off_t, libc::SEEK_SET);
        if ret < 0 {
            return V_ERR_GET_SIGNHEAD as i32;
        }
        
        let signHead = libc::malloc(std::mem::size_of::<HwSignHead>()) as *mut HwSignHead;
        if signHead.is_null() {
            return V_ERR as i32;
        }
        
        let readLen = libc::read((*file).fp, signHead as *mut core::ffi::c_void, std::mem::size_of::<HwSignHead>());
        if readLen != std::mem::size_of::<HwSignHead>() as isize {
            libc::free(signHead as *mut core::ffi::c_void);
            return V_ERR_GET_SIGNHEAD as i32;
        }
        
        SignHeadN2H(signHead);
        
        let mut magicLow: u64 = HAP_SIG_BLOCK_MAGIC_LO;
        let mut magicHigh: u64 = HAP_SIG_BLOCK_MAGIC_HI;
        
        if (*signHead).version < VERSION_FOR_NEW_MAGIC_NUM {
            magicLow = HAP_SIG_BLOCK_MAGIC_LO_OLD;
            magicHigh = HAP_SIG_BLOCK_MAGIC_HI_OLD;
        }
        
        if (*signHead).magicLow != magicLow || (*signHead).magicHigh != magicHigh {
            libc::free(signHead as *mut core::ffi::c_void);
            return V_ERR_GET_SIGNHEAD as i32;
        }
        
        (*signInfo).signHead = signHead;
        (*signInfo).fullSignBlockOffset = (*signInfo).hapCoreDirOffset - (*signHead).size as i32;
        (*signInfo).fileSize = fileSt.st_size as i32;
        
        if (*signInfo).fullSignBlockOffset <= 0 || (*signInfo).fullSignBlockOffset >= (*signInfo).hapCoreDirOffset {
            libc::free(signHead as *mut core::ffi::c_void);
            return V_ERR_GET_SIGNHEAD as i32;
        }
        
        V_OK as i32
    }
}