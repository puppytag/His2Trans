fn PowerStateTokenConstruct(powerStateToken: *mut crate::types::PowerStateToken, deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) -> i32 {
    use crate::compat::*;
    use crate::globals::*;
    unsafe {
        let tokenIf = &mut (*powerStateToken).super_;
        let srefListener = OsalMemCalloc(std::mem::size_of::<crate::types::IHdfSRefListener>() as u32) as *mut crate::types::IHdfSRefListener;
        if srefListener.is_null() {
            return crate::types::HDF_ERR_MALLOC_FAIL;
        }
        (*tokenIf).AcquireWakeLock = Some(PowerStateTokenAcquireWakeLock as unsafe extern "C" fn(*mut crate::types::IPowerStateToken));
        (*tokenIf).ReleaseWakeLock = Some(PowerStateTokenReleaseWakeLock as unsafe extern "C" fn(*mut crate::types::IPowerStateToken));
        (*srefListener).OnFirstAcquire = Some(PowerStateTokenOnFirstAcquire as unsafe extern "C" fn(*mut crate::types::HdfSRef));
        (*srefListener).OnLastRelease = Some(PowerStateTokenOnLastRelease as unsafe extern "C" fn(*mut crate::types::HdfSRef));
        (*powerStateToken).psmState = crate::types::PSM_STATE_IDLE;
        (*powerStateToken).listener = listener;
        (*powerStateToken).deviceObject = deviceObject;
        HdfSRefConstruct(&mut (*powerStateToken).wakeRef, srefListener);
        crate::types::HDF_SUCCESS
    }
}