fn PowerStateTokenOnFirstAcquire(sref: *mut crate::types::HdfSRef) {
    let mut stateToken: *mut crate::types::PowerStateToken = std::ptr::null_mut();
    if sref.is_null() {
        return;
    }
    unsafe {
        let __mptr = sref as *const crate::types::HdfSRef;
        let offset = std::mem::offset_of!(crate::types::PowerStateToken, wakeRef) as isize;
        stateToken = (__mptr as *mut u8).offset(-offset) as *mut crate::types::PowerStateToken;
    }
}