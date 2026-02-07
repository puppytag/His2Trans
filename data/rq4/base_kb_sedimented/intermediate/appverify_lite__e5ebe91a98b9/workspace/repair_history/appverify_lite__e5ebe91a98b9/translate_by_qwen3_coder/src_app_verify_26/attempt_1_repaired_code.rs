fn VerifyProfGetContent(fp: i32, signInfo: *const crate::types::SignatureInfo, certType: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    let mut profBuf: *mut std::ffi::c_char = std::ptr::null_mut();
    let mut len: i32 = 0;
    let mut blockHead: crate::types::BlockHead = unsafe { std::mem::zeroed() };
    let mut rawLen: i32 = 0;
    
    let rawBuf = crate::src_app_verify::GetSignBlockByType(
        signInfo,
        fp,
        crate::types::PROFILE_BLOCK_WITHSIGN_TYPE as i32,
        &mut rawLen,
        &mut blockHead
    );
    
    if rawBuf.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: rawBuf is null\0".as_ptr() as *const std::ffi::c_char,
                b"VerifyProfGetContent\0".as_ptr() as *const std::ffi::c_char,
                762i32
            );
        }
        return crate::types::V_ERR_GET_PROFILE_DATA as i32;
    }
    
    unsafe {
        crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const std::ffi::c_char,
            b"[%s:%d]: certType %d\0".as_ptr() as *const std::ffi::c_char,
            b"VerifyProfGetContent\0".as_ptr() as *const std::ffi::c_char,
            763i32,
            certType
        );
    }
    
    if certType == 0 {
        profBuf = rawBuf;
        len = rawLen;
    } else {
        let ret_inner = crate::src_app_verify::VerifyProfileSignGetRaw(
            rawBuf as *const std::ffi::c_char,
            rawLen,
            &mut profBuf,
            &mut len
        );
        
        if !rawBuf.is_null() {
            unsafe { libc::free(rawBuf as *mut std::ffi::c_void); }
        }
        
        if ret_inner != crate::types::V_OK as i32 {
            unsafe {
                crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const std::ffi::c_char,
                    b"[%s:%d]: ret not ok\0".as_ptr() as *const std::ffi::c_char,
                    b"VerifyProfGetContent\0".as_ptr() as *const std::ffi::c_char,
                    772i32
                );
            }
            return ret_inner;
        }
    }
    
    let mut ret = crate::src_app_provision::ParseProfile(profBuf as *const std::ffi::c_char, len, pf);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: GetSignBlock error\0".as_ptr() as *const std::ffi::c_char,
                b"VerifyProfGetContent\0".as_ptr() as *const std::ffi::c_char,
                777i32
            );
        }
        if !profBuf.is_null() {
            unsafe { libc::free(profBuf as *mut std::ffi::c_void); }
        }
        return crate::types::V_ERR_GET_PARSE_PROFILE as i32;
    }
    
    if !profBuf.is_null() {
        unsafe { libc::free(profBuf as *mut std::ffi::c_void); }
    }
    
    ret = crate::src_app_provision::VerifyProfileContent(pf as *const crate::types::ProfileProf);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: ret not ok\0".as_ptr() as *const std::ffi::c_char,
                b"VerifyProfGetContent\0".as_ptr() as *const std::ffi::c_char,
                784i32
            );
        }
        crate::src_app_provision::ProfFreeData(pf);
        return ret;
    }
    
    ret = crate::src_app_verify::GetAppid(pf);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: ret not ok\0".as_ptr() as *const std::ffi::c_char,
                b"VerifyProfGetContent\0".as_ptr() as *const std::ffi::c_char,
                787i32
            );
        }
        crate::src_app_provision::ProfFreeData(pf);
        return ret;
    }
    
    crate::types::V_OK as i32
}