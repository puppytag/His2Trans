fn CmpCert(certA: *const crate::types::mbedtls_x509_crt, binSignCert: *const crate::types::CertInfo) -> i32 {
    use crate::compat::*;
    use crate::globals::*;
    if certA.is_null() {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: certA is null\0".as_ptr() as *const i8, "CmpCert\0".as_ptr() as *const i8, 797);
        return V_ERR as i32;
    }
    if binSignCert.is_null() {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: binSignCert is null\0".as_ptr() as *const i8, "CmpCert\0".as_ptr() as *const i8, 798);
        return V_ERR as i32;
    }
    let certA_ref = unsafe { &*certA };
    let binSignCert_ref = unsafe { &*binSignCert };
    if certA_ref.subject_raw.len as u64 != binSignCert_ref.subjectLen as u64 ||
        unsafe { libc::memcmp(certA_ref.subject_raw.p as *const core::ffi::c_void, binSignCert_ref.subject as *const core::ffi::c_void, certA_ref.subject_raw.len as usize) } != 0 {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: cert subject diff\0".as_ptr() as *const i8, "CmpCert\0".as_ptr() as *const i8, 802);
        return V_ERR as i32;
    }
    if certA_ref.issuer_raw.len as u64 != binSignCert_ref.issuerLen as u64 ||
        unsafe { libc::memcmp(certA_ref.issuer_raw.p as *const core::ffi::c_void, binSignCert_ref.issuer as *const core::ffi::c_void, certA_ref.issuer_raw.len as usize) } != 0 {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: cert issuer diff\0".as_ptr() as *const i8, "CmpCert\0".as_ptr() as *const i8, 808);
        return V_ERR as i32;
    }
    let pk_type = unsafe { mbedtls_pk_get_type(&certA_ref.pk) };
    if pk_type != binSignCert_ref.pkType {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: pk type diff\0".as_ptr() as *const i8, "CmpCert\0".as_ptr() as *const i8, 814);
        return V_ERR as i32;
    }
    let mut lenA: i32 = 0;
    let bufA = crate::src_app_verify::GetPkBuf(&certA_ref.pk, &mut lenA);
    if bufA.is_null() {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: bufA is null\0".as_ptr() as *const i8, "CmpCert\0".as_ptr() as *const i8, 819);
        return V_ERR as i32;
    }
    if lenA != binSignCert_ref.pkLen {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: pkA len diff %d, %d\0".as_ptr() as *const i8, "CmpCert\0".as_ptr() as *const i8, 822, lenA, binSignCert_ref.pkLen);
        unsafe { libc::free(bufA as *mut core::ffi::c_void) };
        return V_ERR as i32;
    }
    if unsafe { libc::memcmp(bufA as *const core::ffi::c_void, binSignCert_ref.pkBuf as *const core::ffi::c_void, lenA as usize) } != 0 {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: pk content different\0".as_ptr() as *const i8, "CmpCert\0".as_ptr() as *const i8, 828);
        unsafe { libc::free(bufA as *mut core::ffi::c_void) };
        return V_ERR as i32;
    }
    unsafe { libc::free(bufA as *mut core::ffi::c_void) };
    let _ = HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: compare cert consistent\0".as_ptr() as *const i8, "CmpCert\0".as_ptr() as *const i8, 833);
    V_OK as i32
}