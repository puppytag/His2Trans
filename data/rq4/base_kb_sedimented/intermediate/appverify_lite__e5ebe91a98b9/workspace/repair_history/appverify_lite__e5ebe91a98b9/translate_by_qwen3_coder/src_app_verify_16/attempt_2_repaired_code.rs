fn GetAppSingerCertType(pkcs7Handle: *mut crate::types::Pkcs7, certType: *mut i32) -> i32 {
    let sri = PKCS7_GetAllSignersResolvedInfo(pkcs7Handle as *const crate::types::Pkcs7);
    if sri.is_null() || unsafe { (*sri).nrOfSigners } == 0 {
        PKCS7_FreeAllSignersResolvedInfo(sri);
        return crate::types::V_ERR as i32;
    }
    let ret = GetAppCertTypeBySignInfo(unsafe { (*sri).signers }, certType);
    if ret != crate::types::V_OK as i32 {
        PKCS7_FreeAllSignersResolvedInfo(sri);
        return crate::types::V_ERR as i32;
    }
    PKCS7_FreeAllSignersResolvedInfo(sri);
    crate::types::V_OK as i32
}