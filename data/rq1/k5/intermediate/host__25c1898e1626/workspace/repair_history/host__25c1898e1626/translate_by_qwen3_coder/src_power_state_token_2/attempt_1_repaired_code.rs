fn PowerStateTokenOnLastRelease(sref: *mut crate::types::HdfSRef) {
    let mut stateToken: *mut crate::types::PowerStateToken = std::ptr::null_mut();
    let mut listener: *const crate::types::IPowerEventListener = std::ptr::null();
    if sref.is_null() {
        return;
    }
    unsafe {
        stateToken = (sref as *mut u8).offset(-((&(*(std::ptr::null::<crate::types::PowerStateToken>())).wakeRef) as *const crate::types::HdfSRef as isize)) as *mut crate::types::PowerStateToken;
        listener = (*stateToken).listener;
        if !listener.is_null() && (*listener).Suspend.is_some() {
            let _ = ((*listener).Suspend.unwrap())((*stateToken).deviceObject);
        }
        (*stateToken).psmState = crate::types::PSM_STATE_INACTIVE;
    }
}