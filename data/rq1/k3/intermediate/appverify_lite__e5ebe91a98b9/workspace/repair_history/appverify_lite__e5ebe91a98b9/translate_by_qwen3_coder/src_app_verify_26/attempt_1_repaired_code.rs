fn VerifyProfGetContent(fp: i32, signInfo: *const crate::types::SignatureInfo, certType: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    let mut prof_buf: *mut std::ffi::c_char = std::ptr::null_mut();
    let mut len: i32 = 0;
    let mut block_head = crate::types::BlockHead {
        type_: 0,
        length: 0,
        offset: 0,
    };
    let mut raw_len: i32 = 0;
    let raw_buf = crate::src_app_verify::GetSignBlockByType(
        signInfo,
        fp,
        crate::types::PROFILE_BLOCK_WITHSIGN_TYPE as i32,
        &mut raw_len,
        &mut block_head,
    );
    if raw_buf.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: rawBuf is null\0".as_ptr() as *const std::ffi::c_char,
                b"VerifyProfGetContent\0".as_ptr() as *const std::ffi::c_char,
                762,
            );
        }
        return crate::types::V_ERR_GET_PROFILE_DATA as i32;
    }
    unsafe {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const std::ffi::c_char,
            b"[%s:%d]: certType %d\0".as_ptr() as *const std::ffi::c_char,
            b"VerifyProfGetContent\0".as_ptr() as *const std::ffi::c_char,
            763,
            certType,
        );
    }
    if certType == 0 {
        prof_buf = raw_buf;
        len = raw_len;
    } else {
        let mut content_len: i32 = 0;
        let ret = crate::src_app_verify::VerifyProfileSignGetRaw(
            raw_buf as *const std::ffi::c_char,
            raw_len,
            &mut prof_buf,
            &mut content_len,
        );
        if !raw_buf.is_null() {
            unsafe {
                libc::free(raw_buf as *mut std::ffi::c_void);
            }
        }
        if ret != crate::types::V_OK as i32 {
            unsafe {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const std::ffi::c_char,
                    b"[%s:%d]: ret not ok\0".as_ptr() as *const std::ffi::c_char,
                    b"VerifyProfGetContent\0".as_ptr() as *const std::ffi::c_char,
                    772,
                );
            }
            return ret;
        }
        len = content_len;
    }
    let ret = crate::src_app_provision::ParseProfile(prof_buf as *const std::ffi::c_char, len, pf);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: GetSignBlock error\0".as_ptr() as *const std::ffi::c_char,
                b"VerifyProfGetContent\0".as_ptr() as *const std::ffi::c_char,
                777,
            );
        }
        if !prof_buf.is_null() {
            unsafe {
                libc::free(prof_buf as *mut std::ffi::c_void);
            }
        }
        return crate::types::V_ERR_GET_PARSE_PROFILE as i32;
    }
    if !prof_buf.is_null() {
        unsafe {
            libc::free(prof_buf as *mut std::ffi::c_void);
        }
    }
    let ret = crate::src_app_provision::VerifyProfileContent(pf as *const crate::types::ProfileProf);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: ret not ok\0".as_ptr() as *const std::ffi::c_char,
                b"VerifyProfGetContent\0".as_ptr() as *const std::ffi::c_char,
                784,
            );
        }
        crate::src_app_provision::ProfFreeData(pf);
        return ret;
    }
    let ret = crate::src_app_verify::GetAppid(pf);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: ret not ok\0".as_ptr() as *const std::ffi::c_char,
                b"VerifyProfGetContent\0".as_ptr() as *const std::ffi::c_char,
                787,
            );
        }
        crate::src_app_provision::ProfFreeData(pf);
        return ret;
    }
    crate::types::V_OK as i32
}