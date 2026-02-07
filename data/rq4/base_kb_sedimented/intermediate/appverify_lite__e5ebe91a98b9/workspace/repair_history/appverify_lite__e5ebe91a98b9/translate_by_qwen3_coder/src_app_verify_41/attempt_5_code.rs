函数: src_app_verify_41
文件: src_app_verify
尝试次数: 5/5
============================================================
翻译代码（修复前）:
============================================================
pub extern "C" fn APPVERI_AppVerify(filePath: *const ::core::ffi::c_char, verifyRst: *mut crate::types::VerifyResult) -> i32 {
    if filePath.is_null() || verifyRst.is_null() {
        return crate::types::V_ERR_FILE_OPEN as i32;
    }
    
    let mut handle: i32 = 0;
    let mut file: crate::types::FileRead = unsafe { std::mem::zeroed() };
    
    if crate::src_app_file::InitVerify(&mut file, filePath, &mut handle) != crate::types::V_OK as i32 {
        unsafe { libc::close(handle) };
        return crate::types::V_ERR_FILE_OPEN as i32;
    }
    
    let mut signInfo: crate::types::SignatureInfo = unsafe { std::mem::zeroed() };
    let mut ret = crate::src_app_verify::GetSignHead(&file as *const _, &mut signInfo);
    if ret != crate::types::V_OK as i32 {
        unsafe { libc::close(handle) };
        return ret;
    }
    
    let mut signHead = unsafe { signInfo.signHead };
    ret = crate::src_app_verify::VerifyIntegrity(&mut signInfo, handle, unsafe { &mut (*verifyRst).profile });
    if ret != crate::types::V_OK as i32 {
        unsafe { libc::close(handle) };
        if !signHead.is_null() {
            unsafe { libc::free(signHead as *mut ::core::ffi::c_void) };
            signHead = std::ptr::null_mut();
        }
        let _ = signHead;
        return ret;
    }
    
    let fileSt: *mut libc::stat = unsafe { libc::malloc(std::mem::size_of::<libc::stat>()) as *mut libc::stat };
    if fileSt.is_null() {
        unsafe { libc::close(handle) };
        if !signHead.is_null() {
            unsafe { libc::free(signHead as *mut ::core::ffi::c_void) };
        }
        crate::src_app_provision::ProfFreeData(unsafe { &mut (*verifyRst).profile });
        return crate::types::V_ERR_MALLOC as i32;
    }
    
    ret = unsafe { libc::fstat(handle, fileSt) };
    if ret != crate::types::V_OK as i32 {
        unsafe { libc::close(handle) };
        if !signHead.is_null() {
            unsafe { libc::free(signHead as *mut ::core::ffi::c_void) };
        }
        crate::src_app_provision::ProfFreeData(unsafe { &mut (*verifyRst).profile });
        if !fileSt.is_null() {
            unsafe { libc::free(fileSt as *mut ::core::ffi::c_void) };
        }
        return crate::types::V_ERR_FILE_STAT as i32;
    }
    
    unsafe { libc::close(handle) };
    if !signHead.is_null() {
        unsafe { libc::free(signHead as *mut ::core::ffi::c_void) };
    }
    if !fileSt.is_null() {
        unsafe { libc::free(fileSt as *mut ::core::ffi::c_void) };
    }
    
    ret
}