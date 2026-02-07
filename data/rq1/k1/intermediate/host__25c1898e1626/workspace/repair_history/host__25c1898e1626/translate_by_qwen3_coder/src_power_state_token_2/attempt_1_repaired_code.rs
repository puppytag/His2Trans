fn PowerStateTokenOnLastRelease(sref: *mut crate::types::HdfSRef) {
    let mut stateToken: *mut crate::types::PowerStateToken = std::ptr::null_mut();
    let mut listener: *const crate::types::IPowerEventListener = std::ptr::null();
    if sref.is_null() {
        return;
    }
    unsafe {
        let offset = std::mem::offset_of!(crate::types::PowerStateToken, wakeRef) as isize;
        stateToken = (sref as *mut u8).offset(-offset) as *mut crate::types::PowerStateToken;
        listener = (*stateToken).listener;
        if !listener.is_null() && (*listener).Suspend.is_some() {
            let _ = ((*listener).Suspend.unwrap())((*stateToken).deviceObject);
        }
        (*stateToken).psmState = crate::types::PSM_STATE_IDLE;
    }
}