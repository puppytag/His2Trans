fn VerifyDebugInfo(pf: *const crate::types::ProfileProf) -> i32 {
    unsafe {
        let debug_str: *const i8 = crate::types::DEBUG_TYPE.as_ptr() as *const i8;
        if libc::strcmp((*pf).type_, debug_str) != 0 {
            return crate::types::V_OK as i32;
        }
        let ret: i32;
        let udid_str: *const i8 = b"udid\0".as_ptr() as *const i8;
        if libc::strcmp((*pf).debugInfo.devIdType, udid_str) == 0 {
            ret = crate::src_app_provision::VerifyUdid(pf);
        } else {
            ret = crate::types::V_ERR as i32;
        }
        ret
    }
}