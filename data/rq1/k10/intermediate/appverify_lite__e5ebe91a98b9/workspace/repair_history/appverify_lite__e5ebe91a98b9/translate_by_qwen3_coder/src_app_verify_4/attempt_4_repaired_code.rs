fn GetSignHead(file: *const crate::types::FileRead, signInfo: *mut crate::types::SignatureInfo) -> i32 {
    use crate::types::{HAP_SIG_BLOCK_MAGIC_HI, HAP_SIG_BLOCK_MAGIC_HI_OLD, HAP_SIG_BLOCK_MAGIC_LO, HAP_SIG_BLOCK_MAGIC_LO_OLD, VERSION_FOR_NEW_MAGIC_NUM, V_OK, V_ERR_GET_SIGNHEAD, V_ERR, LOG_CORE, LOG_ERROR, LOG_INFO, SEEK_SET};
    use crate::compat::*;
    use crate::globals::*;
    use ::libc::{fstat, lseek, read, free, malloc};
    use ::core::mem::size_of;
    let mut file_st = crate::compat::stat {
        st_dev: 0,
        __st_dev_padding: 0,
        __st_ino_truncated: 0,
        st_mode: 0,
        st_nlink: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        __st_rdev_padding: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        __st_atim32: crate::compat::stat__bindgen_ty_1 { tv_sec: 0, tv_nsec: 0 },
        __st_mtim32: crate::compat::stat__bindgen_ty_1 { tv_sec: 0, tv_nsec: 0 },
        __st_ctim32: crate::compat::stat__bindgen_ty_1 { tv_sec: 0, tv_nsec: 0 },
        st_ino: 0,
        st_atim: crate::compat::timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: crate::compat::timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: crate::compat::timespec { tv_sec: 0, tv_nsec: 0 },
    };
    let fp = unsafe { (*file).fp };
    let mut ret = unsafe { fstat(fp, &mut file_st) };
    if ret != 0 || file_st.st_size < size_of::<crate::types::HwSignHead>() as i64 {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: fstat error, %d, filelen: %d\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 121, ret, file_st.st_size as i32) };
        return V_ERR_GET_SIGNHEAD as i32;
    }
    if !crate::src_app_centraldirectory::FindSignature(file, signInfo) {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: find signature error\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 125) };
        return V_ERR_GET_SIGNHEAD as i32;
    }
    let hap_core_dir_offset = unsafe { (*signInfo).hapCoreDirOffset };
    if hap_core_dir_offset < size_of::<crate::types::HwSignHead>() as i32 {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: hapCoreDirOffset error, %d\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 129, hap_core_dir_offset) };
        return V_ERR_GET_SIGNHEAD as i32;
    }
    ret = unsafe { lseek(fp, hap_core_dir_offset as i64 - size_of::<crate::types::HwSignHead>() as i64, SEEK_SET as i32) } as i32;
    if ret < 0 {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: lseek error, %d\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 134, ret) };
        return V_ERR_GET_SIGNHEAD as i32;
    }
    let sign_head = unsafe { malloc(size_of::<crate::types::HwSignHead>() as usize) as *mut crate::types::HwSignHead };
    if sign_head.is_null() {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: signHead is null\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 138) };
        return V_ERR as i32;
    }
    let read_len = unsafe { read(fp, sign_head as *mut ::core::ffi::c_void, size_of::<crate::types::HwSignHead>() as usize) } as i32;
    if read_len != size_of::<crate::types::HwSignHead>() as i32 {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: readLen %d, %d\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 141, read_len, size_of::<crate::types::HwSignHead>() as i32) };
        unsafe { free(sign_head as *mut ::core::ffi::c_void); }
        return V_ERR_GET_SIGNHEAD as i32;
    }
    crate::src_app_verify::SignHeadN2H(sign_head);
    let mut magic_low = HAP_SIG_BLOCK_MAGIC_LO;
    let mut magic_high = HAP_SIG_BLOCK_MAGIC_HI;
    if unsafe { (*sign_head).version } < VERSION_FOR_NEW_MAGIC_NUM {
        magic_low = HAP_SIG_BLOCK_MAGIC_LO_OLD;
        magic_high = HAP_SIG_BLOCK_MAGIC_HI_OLD;
    }
    if unsafe { (*sign_head).magicLow } != magic_low || unsafe { (*sign_head).magicHigh } != magic_high {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: sign head magic invalid\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 153) };
        unsafe { free(sign_head as *mut ::core::ffi::c_void); }
        return V_ERR_GET_SIGNHEAD as i32;
    }
    let _ = unsafe { HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: sign head: size: %llu, blockNum:0x%x\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 157, unsafe { (*sign_head).size }, unsafe { (*sign_head).blockNum }) };
    unsafe { (*signInfo).signHead = sign_head; }
    unsafe { (*signInfo).fullSignBlockOffset = hap_core_dir_offset - (*sign_head).size as i32; }
    unsafe { (*signInfo).fileSize = file_st.st_size as i32; }
    let full_sign_block_offset = unsafe { (*signInfo).fullSignBlockOffset };
    if full_sign_block_offset <= 0 || full_sign_block_offset >= hap_core_dir_offset {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: fullSignBlockOffset invalid\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 162) };
        unsafe { free(sign_head as *mut ::core::ffi::c_void); }
        return V_ERR_GET_SIGNHEAD as i32;
    }
    V_OK as i32
}