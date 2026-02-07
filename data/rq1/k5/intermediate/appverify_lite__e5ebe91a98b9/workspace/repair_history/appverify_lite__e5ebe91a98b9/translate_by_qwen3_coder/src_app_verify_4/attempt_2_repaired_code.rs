fn GetSignHead(file: *const crate::types::FileRead, signInfo: *mut crate::types::SignatureInfo) -> i32 {
    use crate::types::*;
    use crate::compat::*;
    use crate::globals::*;
    use ::core::ffi::c_void;
    let mut file_st: stat = unsafe { ::core::mem::zeroed() };
    let ret = unsafe { libc::fstat((*file).fp, &mut file_st as *mut stat) };
    if ret != 0 || (file_st.st_size as i64) < (::core::mem::size_of::<HwSignHead>() as i64) {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: fstat error, %d, filelen: %d\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 121, ret, file_st.st_size as i32); }
        return V_ERR_GET_SIGNHEAD as i32;
    }
    if !FindSignature(file, signInfo) {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: find signature error\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 125); }
        return V_ERR_GET_SIGNHEAD as i32;
    }
    let hap_core_dir_offset = unsafe { (*signInfo).hapCoreDirOffset };
    if hap_core_dir_offset < ::core::mem::size_of::<HwSignHead>() as i32 {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: hapCoreDirOffset error, %d\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 129, hap_core_dir_offset); }
        return V_ERR_GET_SIGNHEAD as i32;
    }
    let seek_ret = unsafe { libc::lseek((*file).fp, (hap_core_dir_offset as i64) - (::core::mem::size_of::<HwSignHead>() as i64), 0) };
    if seek_ret < 0 {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: lseek error, %d\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 134, seek_ret as i32); }
        return V_ERR_GET_SIGNHEAD as i32;
    }
    let sign_head = unsafe { libc::malloc(::core::mem::size_of::<HwSignHead>()) as *mut HwSignHead };
    if sign_head.is_null() {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: signHead is null\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 138); }
        return V_ERR as i32;
    }
    let read_len = unsafe { libc::read((*file).fp, sign_head as *mut c_void, ::core::mem::size_of::<HwSignHead>()) };
    if read_len != ::core::mem::size_of::<HwSignHead>() as isize {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: readLen %d, %d\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 141, read_len as i32, ::core::mem::size_of::<HwSignHead>() as i32); }
        unsafe { libc::free(sign_head as *mut c_void); }
        return V_ERR_GET_SIGNHEAD as i32;
    }
    crate::src_app_verify::SignHeadN2H(sign_head);
    let mut magic_low: u64 = HAP_SIG_BLOCK_MAGIC_LO;
    let mut magic_high: u64 = HAP_SIG_BLOCK_MAGIC_HI;
    if unsafe { (*sign_head).version } < VERSION_FOR_NEW_MAGIC_NUM {
        magic_low = HAP_SIG_BLOCK_MAGIC_LO_OLD;
        magic_high = HAP_SIG_BLOCK_MAGIC_HI_OLD;
    }
    if unsafe { (*sign_head).magicLow } != magic_low || unsafe { (*sign_head).magicHigh } != magic_high {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: sign head magic invalid\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 153); }
        unsafe { libc::free(sign_head as *mut c_void); }
        return V_ERR_GET_SIGNHEAD as i32;
    }
    unsafe { let _ = HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: sign head: size: %llu, blockNum:0x%x\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 157, (*sign_head).size, (*sign_head).blockNum); }
    unsafe {
        (*signInfo).signHead = sign_head;
        (*signInfo).fullSignBlockOffset = hap_core_dir_offset - (*sign_head).size as i32;
        (*signInfo).fileSize = file_st.st_size as i32;
    }
    let full_sign_block_offset = unsafe { (*signInfo).fullSignBlockOffset };
    if full_sign_block_offset <= 0 || full_sign_block_offset >= hap_core_dir_offset {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: fullSignBlockOffset invalid\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 162); }
        unsafe { libc::free(sign_head as *mut c_void); }
        return V_ERR_GET_SIGNHEAD as i32;
    }
    V_OK as i32
}