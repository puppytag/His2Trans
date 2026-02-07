fn PowerStateTokenOnFirstAcquire(sref: *mut crate::types::HdfSRef) {
    if sref.is_null() {
        return;
    }
    
    // Calculate offset of wakeRef field in PowerStateToken using offset_of pattern
    // that doesn't dereference null pointer
    let offset = std::mem::offset_of!(crate::types::PowerStateToken, wakeRef);
    
    // Get PowerStateToken from HdfSRef using container_of pattern
    let stateToken: *mut crate::types::PowerStateToken = unsafe {
        (sref as *mut u8).sub(offset) as *mut crate::types::PowerStateToken
    };
    
    if stateToken.is_null() {
        return;
    }
    
    unsafe {
        let listener = (*stateToken).listener;
        if !listener.is_null() {
            if let Some(resume_fn) = (*listener).Resume {
                resume_fn((*stateToken).deviceObject);
            }
        }
    }
}