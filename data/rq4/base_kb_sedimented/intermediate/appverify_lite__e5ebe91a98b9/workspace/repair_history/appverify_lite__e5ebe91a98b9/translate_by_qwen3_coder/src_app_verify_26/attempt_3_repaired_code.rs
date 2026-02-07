fn VerifyProfGetContent(fp: i32, signInfo: *const crate::types::SignatureInfo, certType: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    let mut profBuf: *mut std::ffi::c_char = std::ptr::null_mut();
    let mut len: i32 = 0;
    let mut blockHead: crate::types::BlockHead = unsafe { std::mem::zeroed() };
    let mut rawLen: i32 = 0;
    
    let rawBuf = GetSignBlockByType(
        signInfo,
        fp,
        crate::types::PROFILE_BLOCK_WITHSIGN_TYPE as i32,
        &mut rawLen,
        &mut blockHead
    );
    
    if rawBuf.is_null() {
        return crate::types::V_ERR_GET_PROFILE_DATA as i32;
    }
    
    if certType == 0 {
        profBuf = rawBuf;
        len = rawLen;
    } else {
        let ret_inner = VerifyProfileSignGetRaw(
            rawBuf as *const std::ffi::c_char,
            rawLen,
            &mut profBuf,
            &mut len
        );
        
        if !rawBuf.is_null() {
            unsafe { libc::free(rawBuf as *mut std::ffi::c_void); }
        }
        
        if ret_inner != crate::types::V_OK as i32 {
            return ret_inner;
        }
    }
    
    let mut ret = ParseProfile(profBuf as *const std::ffi::c_char, len, pf);
    if ret != crate::types::V_OK as i32 {
        if !profBuf.is_null() {
            unsafe { libc::free(profBuf as *mut std::ffi::c_void); }
        }
        return crate::types::V_ERR_GET_PARSE_PROFILE as i32;
    }
    
    if !profBuf.is_null() {
        unsafe { libc::free(profBuf as *mut std::ffi::c_void); }
    }
    
    ret = VerifyProfileContent(pf as *const crate::types::ProfileProf);
    if ret != crate::types::V_OK as i32 {
        ProfFreeData(pf);
        return ret;
    }
    
    ret = GetAppid(pf);
    if ret != crate::types::V_OK as i32 {
        ProfFreeData(pf);
        return ret;
    }
    
    crate::types::V_OK as i32
}