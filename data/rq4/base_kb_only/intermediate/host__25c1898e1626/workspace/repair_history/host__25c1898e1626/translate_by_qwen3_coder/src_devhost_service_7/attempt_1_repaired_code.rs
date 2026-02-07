fn ApplyDevicesPowerState(device: *mut crate::types::HdfDevice, state: u32) -> ::core::ffi::c_int {
    use crate::types::*;
    
    let mut deviceNode: *mut HdfDeviceNode;
    let mut ret: ::core::ffi::c_int;
    
    // Inline IsPowerWakeState logic - need to define these constants or use values
    const POWER_STATE_DOZE_RESUME: u32 = 1;
    const POWER_STATE_RESUME: u32 = 3;
    let is_wake_state = state == POWER_STATE_DOZE_RESUME || state == POWER_STATE_RESUME;
    
    // Calculate offset of entry field in HdfDeviceNode using offset_of pattern
    let entry_offset = {
        let dummy = std::mem::MaybeUninit::<HdfDeviceNode>::uninit();
        let base = dummy.as_ptr();
        unsafe { std::ptr::addr_of!((*base).entry) as usize - base as usize }
    };
    
    unsafe {
        if is_wake_state {
            // Forward iteration: DLIST_FOR_EACH_ENTRY
            let head = std::ptr::addr_of_mut!((*device).devNodes);
            let mut current = (*head).next;
            
            while current != head {
                deviceNode = (current as *mut u8).sub(entry_offset) as *mut HdfDeviceNode;
                
                if !(*deviceNode).powerToken.is_null() {
                    ret = crate::src_power_state_token::PowerStateChange((*deviceNode).powerToken, state);
                    if ret != HDF_SUCCESS {
                        let _ = ret;
                    }
                }
                
                current = (*current).next;
            }
        } else {
            // Reverse iteration: DLIST_FOR_EACH_ENTRY_REVERSE
            let head = std::ptr::addr_of_mut!((*device).devNodes);
            let mut current = (*head).prev;
            
            while current != head {
                deviceNode = (current as *mut u8).sub(entry_offset) as *mut HdfDeviceNode;
                
                if !(*deviceNode).powerToken.is_null() {
                    ret = crate::src_power_state_token::PowerStateChange((*deviceNode).powerToken, state);
                    if ret != HDF_SUCCESS {
                        let _ = ret;
                    }
                }
                
                current = (*current).prev;
            }
        }
    }
    
    HDF_SUCCESS
}