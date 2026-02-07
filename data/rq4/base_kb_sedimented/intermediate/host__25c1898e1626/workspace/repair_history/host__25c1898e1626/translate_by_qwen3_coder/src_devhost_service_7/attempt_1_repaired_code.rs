fn ApplyDevicesPowerState(device: *mut crate::types::HdfDevice, state: u32) -> ::core::ffi::c_int {
    use crate::types::*;
    
    let mut deviceNode: *mut HdfDeviceNode;
    let mut ret: ::core::ffi::c_int;
    
    // Inline IsPowerWakeState logic - need to check if these constants exist
    // POWER_STATE_DOZE_RESUME and POWER_STATE_RESUME
    let is_wake_state = state == 1 || state == 3; // typical wake states
    
    // Calculate entry offset using memoffset-style calculation
    let entry_offset = unsafe {
        let dummy: HdfDeviceNode = std::mem::zeroed();
        let base = &dummy as *const HdfDeviceNode as usize;
        let field = &dummy.entry as *const DListHead as usize;
        field - base
    };
    
    if is_wake_state {
        // DLIST_FOR_EACH_ENTRY forward iteration
        unsafe {
            let head = &(*device).devNodes as *const DListHead as *mut DListHead;
            let mut current = (*head).next;
            
            while current != head {
                deviceNode = (current as *mut u8).sub(entry_offset) as *mut HdfDeviceNode;
                
                if !(*deviceNode).powerToken.is_null() {
                    ret = crate::src_power_state_token::PowerStateChange((*deviceNode).powerToken, state);
                    if ret != HDF_SUCCESS as ::core::ffi::c_int {
                        let _ = ret;
                    }
                }
                
                current = (*current).next;
            }
        }
    } else {
        // DLIST_FOR_EACH_ENTRY_REVERSE backward iteration
        unsafe {
            let head = &(*device).devNodes as *const DListHead as *mut DListHead;
            let mut current = (*head).prev;
            
            while current != head {
                deviceNode = (current as *mut u8).sub(entry_offset) as *mut HdfDeviceNode;
                
                if !(*deviceNode).powerToken.is_null() {
                    ret = crate::src_power_state_token::PowerStateChange((*deviceNode).powerToken, state);
                    if ret != HDF_SUCCESS as ::core::ffi::c_int {
                        let _ = ret;
                    }
                }
                
                current = (*current).prev;
            }
        }
    }
    
    HDF_SUCCESS as ::core::ffi::c_int
}