fn GetSignHead(file: *const crate::types::FileRead, signInfo: *mut crate::types::SignatureInfo) -> i32 {
    use crate::types::{HAP_SIG_BLOCK_MAGIC_HI, HAP_SIG_BLOCK_MAGIC_LO, HAP_SIG_BLOCK_MAGIC_HI_OLD, HAP_SIG_BLOCK_MAGIC_LO_OLD, VERSION_FOR_NEW_MAGIC_NUM, V_OK, V_ERR_GET_SIGNHEAD, V_ERR, LOG_CORE, LOG_ERROR, LOG_INFO, SEEK_SET};
    use crate::compat::*;
    use crate::globals::*;
    let mut file_st = std::mem::MaybeUninit::<libc::stat>::uninit();
    let ret = unsafe { libc::fstat((*file).fp, file_st.as_mut_ptr()) };
    if ret != 0 {
        let _ = unsafe { HiLogPrint(LOG_CORE as i32, LOG_ERROR as i32, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: fstat error, %d, filelen: %d\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 121, ret, 0) };
        return V_ERR_GET_SIGNHEAD as i32;
    }
    let file_st = unsafe { file_st.assume_init() };
    if (file_st.st_size as i64) < std::mem::size_of::<crate::types::HwSignHead>() as i64 {
        let _ = unsafe { HiLogPrint(LOG_CORE as i32, LOG_ERROR as i32, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: fstat error, %d, filelen: %d\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 121, ret, file_st.st_size as i32) };
        return V_ERR_GET_SIGNHEAD as i32;
    }
    if !FindSignature(file, signInfo) {
        let _ = unsafe { HiLogPrint(LOG_CORE as i32, LOG_ERROR as i32, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: find signature error\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 125) };
        return V_ERR_GET_SIGNHEAD as i32;
    }
    let hap_core_dir_offset = unsafe { (*signInfo).hapCoreDirOffset };
    if hap_core_dir_offset < std::mem::size_of::<crate::types::HwSignHead>() as i32 {
        let _ = unsafe { HiLogPrint(LOG_CORE as i32, LOG_ERROR as i32, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: hapCoreDirOffset error, %d\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 129, hap_core_dir_offset) };
        return V_ERR_GET_SIGNHEAD as i32;
    }
    let ret = unsafe { libc::lseek((*file).fp, (hap_core_dir_offset as i64) - (std::mem::size_of::<crate::types::HwSignHead>() as i64), SEEK_SET as i32) };
    if ret < 0 {
        let _ = unsafe { HiLogPrint(LOG_CORE as i32, LOG_ERROR as i32, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: lseek error, %d\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 134, ret as i32) };
        return V_ERR_GET_SIGNHEAD as i32;
    }
    let sign_head = unsafe { libc::malloc(std::mem::size_of::<crate::types::HwSignHead>()) as *mut crate::types::HwSignHead };
    if sign_head.is_null() {
        let _ = unsafe { HiLogPrint(LOG_CORE as i32, LOG_ERROR as i32, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: signHead is null\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 138) };
        return V_ERR as i32;
    }
    let read_len = unsafe { libc::read((*file).fp, sign_head as *mut std::ffi::c_void, std::mem::size_of::<crate::types::HwSignHead>()) };
    if read_len != std::mem::size_of::<crate::types::HwSignHead>() as isize {
        let _ = unsafe { HiLogPrint(LOG_CORE as i32, LOG_ERROR as i32, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: readLen %d, %d\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 141, read_len as i32, std::mem::size_of::<crate::types::HwSignHead>() as i32) };
        unsafe { libc::free(sign_head as *mut std::ffi::c_void); }
        return V_ERR_GET_SIGNHEAD as i32;
    }
    crate::src_app_verify::SignHeadN2H(sign_head);
    let mut magic_low = HAP_SIG_BLOCK_MAGIC_LO;
    let mut magic_high = HAP_SIG_BLOCK_MAGIC_HI;
    if unsafe { (*sign_head).version } < VERSION_FOR_NEW_MAGIC_NUM {
        magic_low = HAP_SIG_BLOCK_MAGIC_LO_OLD;
        magic_high = HAP_SIG_BLOCK_MAGIC_HI_OLD;
    }
    if unsafe { (*sign_head).magicLow != magic_low || (*sign_head).magicHigh != magic_high } {
        let _ = unsafe { HiLogPrint(LOG_CORE as i32, LOG_ERROR as i32, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: sign head magic invalid\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 153) };
        unsafe { libc::free(sign_head as *mut std::ffi::c_void); }
        return V_ERR_GET_SIGNHEAD as i32;
    }
    let _ = unsafe { HiLogPrint(LOG_CORE as i32, LOG_INFO as i32, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: sign head: size: %llu, blockNum:0x%x\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 157, (*sign_head).size, (*sign_head).blockNum) };
    unsafe { (*signInfo).signHead = sign_head; }
    unsafe { (*signInfo).fullSignBlockOffset = hap_core_dir_offset - (*sign_head).size as i32; }
    unsafe { (*signInfo).fileSize = file_st.st_size as i32; }
    let full_sign_block_offset = unsafe { (*signInfo).fullSignBlockOffset };
    if full_sign_block_offset <= 0 || full_sign_block_offset >= hap_core_dir_offset {
        let _ = unsafe { HiLogPrint(LOG_CORE as i32, LOG_ERROR as i32, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: fullSignBlockOffset invalid\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 162) };
        unsafe { libc::free(sign_head as *mut std::ffi::c_void); }
        return V_ERR_GET_SIGNHEAD as i32;
    }
    V_OK as i32
}