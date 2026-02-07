fn PowerStateTokenConstruct(powerStateToken: *mut crate::types::PowerStateToken, deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) -> i32 {
    unsafe {
        let tokenIf = &mut (*powerStateToken).super_ as *mut crate::types::IPowerStateToken;
        let srefListener = libc::calloc(1, std::mem::size_of::<crate::types::IHdfSRefListener>()) as *mut crate::types::IHdfSRefListener;
        if srefListener.is_null() {
            return crate::types::HDF_ERR_MALLOC_FAIL;
        }
        (*tokenIf).AcquireWakeLock = Some(crate::compat::PowerStateTokenAcquireWakeLock);
        (*tokenIf).ReleaseWakeLock = Some(crate::compat::PowerStateTokenReleaseWakeLock);
        (*srefListener).OnFirstAcquire = Some(crate::compat::PowerStateTokenOnFirstAcquire);
        (*srefListener).OnLastRelease = Some(crate::compat::PowerStateTokenOnLastRelease);
        (*powerStateToken).psmState = crate::types::PSM_STATE_IDLE;
        (*powerStateToken).listener = listener;
        (*powerStateToken).deviceObject = deviceObject;
        crate::compat::HdfSRefConstruct(&mut (*powerStateToken).wakeRef as *mut crate::types::HdfSRef, srefListener);
        crate::types::HDF_SUCCESS
    }
}