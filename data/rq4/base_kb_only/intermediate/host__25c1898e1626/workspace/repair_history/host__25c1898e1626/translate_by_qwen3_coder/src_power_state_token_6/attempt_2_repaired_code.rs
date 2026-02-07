fn PowerStateTokenConstruct(powerStateToken: *mut crate::types::PowerStateToken, deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) -> i32 {
    unsafe {
        let tokenIf: *mut crate::types::IPowerStateToken = &mut (*powerStateToken).super_;
        let srefListener: *mut crate::types::IHdfSRefListener = crate::compat::OsalMemCalloc(
            std::mem::size_of::<crate::types::IHdfSRefListener>() as u32
        ) as *mut crate::types::IHdfSRefListener;
        
        if srefListener.is_null() {
            return crate::types::HDF_ERR_MALLOC_FAIL;
        }
        
        (*tokenIf).AcquireWakeLock = Some(std::mem::transmute::<fn(*mut crate::types::IPowerStateToken), unsafe extern "C" fn(*mut crate::types::IPowerStateToken)>(PowerStateTokenAcquireWakeLock));
        (*tokenIf).ReleaseWakeLock = Some(std::mem::transmute::<fn(*mut crate::types::IPowerStateToken), unsafe extern "C" fn(*mut crate::types::IPowerStateToken)>(PowerStateTokenReleaseWakeLock));
        
        (*srefListener).OnFirstAcquire = Some(std::mem::transmute::<fn(*mut crate::types::HdfSRef), unsafe extern "C" fn(*mut crate::types::HdfSRef)>(PowerStateTokenOnFirstAcquire));
        (*srefListener).OnLastRelease = Some(std::mem::transmute::<fn(*mut crate::types::HdfSRef), unsafe extern "C" fn(*mut crate::types::HdfSRef)>(PowerStateTokenOnLastRelease));
        
        (*powerStateToken).psmState = crate::types::PSM_STATE_IDLE;
        (*powerStateToken).listener = listener;
        (*powerStateToken).deviceObject = deviceObject;
        crate::compat::HdfSRefConstruct(&mut (*powerStateToken).wakeRef, srefListener);
        
        crate::types::HDF_SUCCESS
    }
}