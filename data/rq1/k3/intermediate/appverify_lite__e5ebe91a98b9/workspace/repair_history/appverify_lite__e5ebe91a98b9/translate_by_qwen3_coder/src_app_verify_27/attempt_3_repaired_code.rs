fn CmpCert(certA: *const crate::types::mbedtls_x509_crt, binSignCert: *const crate::types::CertInfo) -> i32 {
    use crate::compat::*;
    use crate::globals::*;
    unsafe {
        if certA.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: certA is null\0".as_ptr() as *const i8, "CmpCert\0".as_ptr() as *const i8, 797);
            return V_ERR as i32;
        }
        if binSignCert.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: binSignCert is null\0".as_ptr() as *const i8, "CmpCert\0".as_ptr() as *const i8, 798);
            return V_ERR as i32;
        }
        let cert = &*certA;
        let bin = &*binSignCert;
        if cert.subject_raw.len != bin.subjectLen as u32 ||
            memcmp(cert.subject_raw.p as *const _, bin.subject as *const _, cert.subject_raw.len as u32) != 0 {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: cert subject diff\0".as_ptr() as *const i8, "CmpCert\0".as_ptr() as *const i8, 802);
            return V_ERR as i32;
        }
        if cert.issuer_raw.len != bin.issuerLen as u32 ||
            memcmp(cert.issuer_raw.p as *const _, bin.issuer as *const _, cert.issuer_raw.len as u32) != 0 {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: cert issuer diff\0".as_ptr() as *const i8, "CmpCert\0".as_ptr() as *const i8, 808);
            return V_ERR as i32;
        }
        if mbedtls_pk_get_type(&cert.pk) != bin.pkType {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: pk type diff\0".as_ptr() as *const i8, "CmpCert\0".as_ptr() as *const i8, 814);
            return V_ERR as i32;
        }
        let mut lenA: i32 = 0;
        let bufA = crate::src_app_verify::GetPkBuf(&cert.pk, &mut lenA);
        if bufA.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: bufA is null\0".as_ptr() as *const i8, "CmpCert\0".as_ptr() as *const i8, 819);
            return V_ERR as i32;
        }
        if lenA != bin.pkLen {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: pkA len diff %d, %d\0".as_ptr() as *const i8, "CmpCert\0".as_ptr() as *const i8, 822, lenA, bin.pkLen);
            libc::free(bufA as *mut _);
            return V_ERR as i32;
        }
        if memcmp(bufA as *const _, bin.pkBuf as *const _, lenA as u32) != 0 {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: pk content different\0".as_ptr() as *const i8, "CmpCert\0".as_ptr() as *const i8, 828);
            libc::free(bufA as *mut _);
            return V_ERR as i32;
        }
        libc::free(bufA as *mut _);
        let _ = HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: compare cert consistent\0".as_ptr() as *const i8, "CmpCert\0".as_ptr() as *const i8, 833);
        V_OK as i32
    }
}