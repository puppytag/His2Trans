fn PowerStateTokenOnFirstAcquire(sref: *mut crate::types::HdfSRef) {
    let mut stateToken: *mut crate::types::PowerStateToken = std::ptr::null_mut();
    if sref.is_null() {
        return;
    }
    unsafe {
        let offset = std::mem::offset_of!(crate::types::PowerStateToken, wakeRef) as isize;
        stateToken = (sref as *mut u8).offset(-offset) as *mut crate::types::PowerStateToken;
        if !(*stateToken).listener.is_null() && (*(*stateToken).listener).Resume.is_some() {
            let _ = (*(*stateToken).listener).Resume.unwrap()((*stateToken).deviceObject);
        }
        (*stateToken).psmState = crate::types::PSM_STATE_ACTIVE as crate::types::HdfPsmState;
    }
}