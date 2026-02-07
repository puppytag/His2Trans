函数: src_app_verify_41
文件: src_app_verify
尝试次数: 1/5
============================================================
修复后的代码:
============================================================
pub extern "C" fn APPVERI_AppVerify(filePath: *const ::core::ffi::c_char, verifyRst: *mut crate::types::VerifyResult) -> i32 {
    if filePath.is_null() || verifyRst.is_null() {
        return crate::types::V_ERR_FILE_OPEN as i32;
    }
    let mut handle: i32 = 0;
    let mut file = crate::types::FileRead {
        fp: 0,
        offset: 0,
        len: 0,
    };
    if crate::src_app_file::InitVerify(&mut file as *mut crate::types::FileRead, filePath, &mut handle as *mut i32) != crate::types::V_OK as i32 {
        unsafe {
            libc::close(handle);
        }
        return crate::types::V_ERR_FILE_OPEN as i32;
    }
    let mut signInfo = crate::types::SignatureInfo {
        signHead: std::ptr::null_mut(),
        fullSignBlockOffset: 0,
        hapCoreDirOffset: 0,
        hapEocdOffset: 0,
        hapEocdSize: 0,
        fileSize: 0,
        version: 0,
        certType: 0,
    };
    let mut ret = crate::src_app_verify::GetSignHead(&file as *const crate::types::FileRead, &mut signInfo as *mut crate::types::SignatureInfo);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: get sign head error\0".as_ptr() as *const ::core::ffi::c_char, b"APPVERI_AppVerify\0".as_ptr() as *const ::core::ffi::c_char, 1188) };
        unsafe {
            libc::close(handle);
        }
        return ret;
    }
    let mut signHead = signInfo.signHead;
    ret = crate::src_app_verify::VerifyIntegrity(&mut signInfo as *mut crate::types::SignatureInfo, handle, unsafe { &mut (*verifyRst).profile } as *mut crate::types::ProfileProf);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: verify integrity failed\0".as_ptr() as *const ::core::ffi::c_char, b"APPVERI_AppVerify\0".as_ptr() as *const ::core::ffi::c_char, 1195) };
        unsafe {
            libc::close(handle);
            if !signHead.is_null() {
                libc::free(signHead as *mut std::ffi::c_void);
                signHead = std::ptr::null_mut();
            }
        }
        return ret;
    }
    let fileSt = unsafe { libc::malloc(std::mem::size_of::<libc::stat>()) as *mut libc::stat };
    if fileSt.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_INFO as u32, 0xD001100, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: malloc error\0".as_ptr() as *const ::core::ffi::c_char, b"APPVERI_AppVerify\0".as_ptr() as *const ::core::ffi::c_char, 1202) };
        unsafe {
            libc::close(handle);
            if !signHead.is_null() {
                libc::free(signHead as *mut std::ffi::c_void);
                signHead = std::ptr::null_mut();
            }
            crate::src_app_provision::ProfFreeData(&mut (*verifyRst).profile as *mut crate::types::ProfileProf);
        }
        return crate::types::V_ERR_MALLOC as i32;
    }
    ret = unsafe { libc::fstat(handle, fileSt) };
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: fstat error\0".as_ptr() as *const ::core::ffi::c_char, b"APPVERI_AppVerify\0".as_ptr() as *const ::core::ffi::c_char, 1210) };
        unsafe {
            libc::close(handle);
            if !signHead.is_null() {
                libc::free(signHead as *mut std::ffi::c_void);
                signHead = std::ptr::null_mut();
            }
            crate::src_app_provision::ProfFreeData(&mut (*verifyRst).profile as *mut crate::types::ProfileProf);
            if !fileSt.is_null() {
                libc::free(fileSt as *mut std::ffi::c_void);
            }
        }
        return crate::types::V_ERR_FILE_STAT as i32;
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_INFO as u32, 0xD001100, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: file len: %d\0".as_ptr() as *const ::core::ffi::c_char, b"APPVERI_AppVerify\0".as_ptr() as *const ::core::ffi::c_char, 1217, unsafe { (*fileSt).st_size as i32 }) };
    unsafe {
        libc::close(handle);
        if !signHead.is_null() {
            libc::free(signHead as *mut std::ffi::c_void);
        }
        if !fileSt.is_null() {
            libc::free(fileSt as *mut std::ffi::c_void);
        }
    }
    ret
}