fn VerifyProfGetContent(fp: i32, signInfo: *const crate::types::SignatureInfo, certType: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    let mut profBuf: *mut std::ffi::c_char = std::ptr::null_mut();
    let mut len: i32 = 0;
    let mut blockHead: crate::types::BlockHead = unsafe { std::mem::zeroed() };
    let mut rawLen: i32 = 0;
    
    let rawBuf = unsafe {
        crate::src_app_verify::GetSignBlockByType(
            signInfo,
            fp,
            crate::types::PROFILE_BLOCK_WITHSIGN_TYPE as i32,
            &mut rawLen,
            &mut blockHead,
        )
    };
    
    if rawBuf.is_null() {
        return crate::types::V_ERR_GET_PROFILE_DATA as i32;
    }
    
    let mut rawBuf_mut = rawBuf;
    
    if certType == 0 {
        profBuf = rawBuf_mut;
        len = rawLen;
    } else {
        let ret_sign = crate::src_app_verify::VerifyProfileSignGetRaw(
            rawBuf_mut as *const std::ffi::c_char,
            rawLen,
            &mut profBuf,
            &mut len,
        );
        
        if !rawBuf_mut.is_null() {
            unsafe { libc::free(rawBuf_mut as *mut std::ffi::c_void) };
            rawBuf_mut = std::ptr::null_mut();
        }
        
        if ret_sign != crate::types::V_OK as i32 {
            return ret_sign;
        }
    }
    let _ = rawBuf_mut;
    
    let ret_parse = unsafe {
        crate::src_app_provision::ParseProfile(profBuf as *const std::ffi::c_char, len, pf)
    };
    
    if ret_parse != crate::types::V_OK as i32 {
        if !profBuf.is_null() {
            unsafe { libc::free(profBuf as *mut std::ffi::c_void) };
        }
        return crate::types::V_ERR_GET_PARSE_PROFILE as i32;
    }
    
    if !profBuf.is_null() {
        unsafe { libc::free(profBuf as *mut std::ffi::c_void) };
    }
    
    let ret_content = unsafe {
        crate::src_app_provision::VerifyProfileContent(pf as *const crate::types::ProfileProf)
    };
    
    if ret_content != crate::types::V_OK as i32 {
        unsafe { crate::src_app_provision::ProfFreeData(pf) };
        return ret_content;
    }
    
    let ret_appid = unsafe { crate::src_app_verify::GetAppid(pf) };
    
    if ret_appid != crate::types::V_OK as i32 {
        unsafe { crate::src_app_provision::ProfFreeData(pf) };
        return ret_appid;
    }
    
    crate::types::V_OK as i32
}