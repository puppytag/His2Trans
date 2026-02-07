函数: src_app_verify_41
文件: src_app_verify
尝试次数: 1/5
============================================================
修复后的代码:
============================================================
pub extern "C" fn APPVERI_AppVerify(filePath: *const ::core::ffi::c_char, verifyRst: *mut crate::types::VerifyResult) -> i32 {
    use crate::types::{FileRead, SignatureInfo, VerifyResult, V_ERR_FILE_OPEN, V_ERR_FILE_STAT, V_ERR_MALLOC, V_OK};
    use crate::src_app_file::InitVerify;
    use crate::src_app_provision::ProfFreeData;
    use crate::src_app_verify::{GetSignHead, VerifyIntegrity};
    use ::core::ffi::c_void;
    use libc::{close, fstat, free, malloc};

    if filePath.is_null() || verifyRst.is_null() {
        return V_ERR_FILE_OPEN as i32;
    }
    let mut handle: i32 = 0;
    let mut file = FileRead {
        fp: 0,
        offset: 0,
        len: 0,
    };
    if unsafe { InitVerify(&mut file as *mut FileRead, filePath, &mut handle as *mut i32) } != V_OK as i32 {
        unsafe { close(handle); }
        return V_ERR_FILE_OPEN as i32;
    }
    let mut signInfo = SignatureInfo {
        signHead: std::ptr::null_mut(),
        fullSignBlockOffset: 0,
        hapCoreDirOffset: 0,
        hapEocdOffset: 0,
        hapEocdSize: 0,
        fileSize: 0,
        version: 0,
        certType: 0,
    };
    let ret = unsafe { GetSignHead(&file as *const FileRead, &mut signInfo as *mut SignatureInfo) };
    if ret != V_OK as i32 {
        unsafe { close(handle); }
        return ret;
    }
    let mut signHead = signInfo.signHead;
    let ret = unsafe { VerifyIntegrity(&mut signInfo as *mut SignatureInfo, handle, &mut (*verifyRst).profile as *mut crate::types::ProfileProf) };
    if ret != V_OK as i32 {
        unsafe { close(handle); }
        if !signHead.is_null() {
            unsafe { free(signHead as *mut c_void); }
            signHead = std::ptr::null_mut();
        }
        return ret;
    }
    let fileSt = unsafe { malloc(std::mem::size_of::<libc::stat>()) } as *mut libc::stat;
    if fileSt.is_null() {
        unsafe { close(handle); }
        if !signHead.is_null() {
            unsafe { free(signHead as *mut c_void); }
            signHead = std::ptr::null_mut();
        }
        unsafe { ProfFreeData(&mut (*verifyRst).profile as *mut crate::types::ProfileProf); }
        return V_ERR_MALLOC as i32;
    }
    let ret = unsafe { fstat(handle, fileSt) };
    if ret != V_OK as i32 {
        unsafe { close(handle); }
        if !signHead.is_null() {
            unsafe { free(signHead as *mut c_void); }
            signHead = std::ptr::null_mut();
        }
        unsafe { ProfFreeData(&mut (*verifyRst).profile as *mut crate::types::ProfileProf); }
        if !fileSt.is_null() {
            unsafe { free(fileSt as *mut c_void); }
        }
        return V_ERR_FILE_STAT as i32;
    }
    unsafe { close(handle); }
    if !signHead.is_null() {
        unsafe { free(signHead as *mut c_void); }
    }
    if !fileSt.is_null() {
        unsafe { free(fileSt as *mut c_void); }
    }
    return ret;
}