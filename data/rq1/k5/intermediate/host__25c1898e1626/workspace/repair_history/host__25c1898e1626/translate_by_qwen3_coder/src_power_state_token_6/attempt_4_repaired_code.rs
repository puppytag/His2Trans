fn PowerStateTokenConstruct(powerStateToken: *mut crate::types::PowerStateToken, deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) -> i32 {
    unsafe {
        let tokenIf = &mut (*powerStateToken).super_ as *mut crate::types::IPowerStateToken;
        let srefListener = libc::calloc(1, std::mem::size_of::<crate::types::IHdfSRefListener>()) as *mut crate::types::IHdfSRefListener;
        if srefListener.is_null() {
            return crate::types::HDF_ERR_MALLOC_FAIL;
        }
        (*tokenIf).AcquireWakeLock = Some(core::mem::transmute(PowerStateTokenAcquireWakeLock as *const ()));
        (*tokenIf).ReleaseWakeLock = Some(core::mem::transmute(PowerStateTokenReleaseWakeLock as *const ()));
        (*srefListener).OnFirstAcquire = Some(core::mem::transmute(PowerStateTokenOnFirstAcquire as *const ()));
        (*srefListener).OnLastRelease = Some(core::mem::transmute(PowerStateTokenOnLastRelease as *const ()));
        (*powerStateToken).psmState = crate::types::PSM_STATE_IDLE;
        (*powerStateToken).listener = listener;
        (*powerStateToken).deviceObject = deviceObject;
        crate::compat::HdfSRefConstruct(&mut (*powerStateToken).wakeRef as *mut crate::types::HdfSRef, srefListener);
        crate::types::HDF_SUCCESS
    }
}