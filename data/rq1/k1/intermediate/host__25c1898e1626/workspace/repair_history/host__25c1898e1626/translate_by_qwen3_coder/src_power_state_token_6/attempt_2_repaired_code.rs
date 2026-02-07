fn PowerStateTokenConstruct(powerStateToken: *mut crate::types::PowerStateToken, deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) -> i32 {
    unsafe {
        let tokenIf = &mut (*powerStateToken).super_ as *mut crate::types::IPowerStateToken;
        let srefListener = libc::malloc(std::mem::size_of::<crate::types::IHdfSRefListener>()) as *mut crate::types::IHdfSRefListener;
        if srefListener.is_null() {
            return crate::types::HDF_ERR_MALLOC_FAIL;
        }
        std::ptr::write_bytes(srefListener as *mut u8, 0, std::mem::size_of::<crate::types::IHdfSRefListener>());
        (*tokenIf).AcquireWakeLock = Some(PowerStateTokenAcquireWakeLock as unsafe extern "C" fn(*mut crate::types::IPowerStateToken));
        (*tokenIf).ReleaseWakeLock = Some(PowerStateTokenReleaseWakeLock as unsafe extern "C" fn(*mut crate::types::IPowerStateToken));
        (*srefListener).OnFirstAcquire = Some(PowerStateTokenOnFirstAcquire as unsafe extern "C" fn(*mut crate::types::HdfSRef));
        (*srefListener).OnLastRelease = Some(PowerStateTokenOnLastRelease as unsafe extern "C" fn(*mut crate::types::HdfSRef));
        (*powerStateToken).psmState = crate::types::PSM_STATE_IDLE;
        (*powerStateToken).listener = listener;
        (*powerStateToken).deviceObject = deviceObject;
        crate::compat::HdfSRefConstruct(&mut (*powerStateToken).wakeRef as *mut crate::types::HdfSRef, srefListener);
        crate::types::HDF_SUCCESS
    }
}