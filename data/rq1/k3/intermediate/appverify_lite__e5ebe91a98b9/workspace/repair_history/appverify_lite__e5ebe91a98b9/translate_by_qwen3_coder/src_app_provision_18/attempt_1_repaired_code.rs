fn VerifyDebugInfo(pf: *const crate::types::ProfileProf) -> i32 {
    unsafe {
        let type_ptr = (*pf).type_;
        if libc::strcmp(type_ptr, b"debug\0".as_ptr() as *const i8) != 0 {
            let _ = HiLogPrint(
                crate::types::LOG_CORE as i32,
                crate::types::LOG_INFO as i32,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: not debug app, return ok\0".as_ptr() as *const i8,
                b"VerifyDebugInfo\0".as_ptr() as *const i8,
                415,
            );
            return crate::types::V_OK as i32;
        }
        let dev_id_type_ptr = (*pf).debugInfo.devIdType;
        let _ = HiLogPrint(
            crate::types::LOG_CORE as i32,
            crate::types::LOG_INFO as i32,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: devid type: %s\0".as_ptr() as *const i8,
            b"VerifyDebugInfo\0".as_ptr() as *const i8,
            418,
            dev_id_type_ptr,
        );
        let mut ret: i32;
        if libc::strcmp(dev_id_type_ptr, b"udid\0".as_ptr() as *const i8) == 0 {
            ret = crate::src_app_provision::VerifyUdid(pf);
        } else {
            let _ = HiLogPrint(
                crate::types::LOG_CORE as i32,
                crate::types::LOG_ERROR as i32,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: devid type invalid\0".as_ptr() as *const i8,
                b"VerifyDebugInfo\0".as_ptr() as *const i8,
                423,
            );
            ret = crate::types::V_ERR as i32;
        }
        ret
    }
}