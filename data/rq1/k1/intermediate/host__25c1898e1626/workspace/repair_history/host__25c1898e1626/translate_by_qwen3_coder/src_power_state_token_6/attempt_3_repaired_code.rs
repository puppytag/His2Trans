fn PowerStateTokenConstruct(powerStateToken: *mut crate::types::PowerStateToken, deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) -> i32 {
    unsafe {
        let tokenIf = &mut (*powerStateToken).super_ as *mut crate::types::IPowerStateToken;
        let srefListener = libc::malloc(std::mem::size_of::<crate::types::IHdfSRefListener>()) as *mut crate::types::IHdfSRefListener;
        if srefListener.is_null() {
            return crate::types::HDF_ERR_MALLOC_FAIL;
        }
        std::ptr::write_bytes(srefListener as *mut u8, 0, std::mem::size_of::<crate::types::IHdfSRefListener>());
        (*tokenIf).AcquireWakeLock = Some(std::mem::transmute(PowerStateTokenAcquireWakeLock as *const ()));
        (*tokenIf).ReleaseWakeLock = Some(std::mem::transmute(PowerStateTokenReleaseWakeLock as *const ()));
        (*srefListener).OnFirstAcquire = Some(std::mem::transmute(PowerStateTokenOnFirstAcquire as *const ()));
        (*srefListener).OnLastRelease = Some(std::mem::transmute(PowerStateTokenOnLastRelease as *const ()));
        (*powerStateToken).psmState = crate::types::PSM_STATE_IDLE;
        (*powerStateToken).listener = listener;
        (*powerStateToken).deviceObject = deviceObject;
        crate::compat::HdfSRefConstruct(&mut (*powerStateToken).wakeRef as *mut crate::types::HdfSRef, srefListener);
        crate::types::HDF_SUCCESS
    }
}