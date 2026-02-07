fn SignHeadN2H(signHead: *mut crate::types::HwSignHead) {
    unsafe {
        let blocknum_ptr = std::ptr::addr_of!((*signHead).blockNum) as *const ::core::ffi::c_uchar;
        (*signHead).blockNum = crate::src_app_common::HapGetInt(
            blocknum_ptr,
            std::mem::size_of::<u32>() as i32,
        ) as u32;
        
        let size_ptr = std::ptr::addr_of!((*signHead).size) as *const ::core::ffi::c_uchar;
        (*signHead).size = crate::src_app_common::HapGetInt64(
            size_ptr,
            std::mem::size_of::<::core::ffi::c_ulonglong>() as i32,
        ) as ::core::ffi::c_ulonglong;
        
        let magiclow_ptr = std::ptr::addr_of!((*signHead).magicLow) as *const ::core::ffi::c_uchar;
        (*signHead).magicLow = crate::src_app_common::HapGetInt64(
            magiclow_ptr,
            std::mem::size_of::<::core::ffi::c_ulonglong>() as i32,
        ) as ::core::ffi::c_ulonglong;
        
        let magichigh_ptr = std::ptr::addr_of!((*signHead).magicHigh) as *const ::core::ffi::c_uchar;
        (*signHead).magicHigh = crate::src_app_common::HapGetInt64(
            magichigh_ptr,
            std::mem::size_of::<::core::ffi::c_ulonglong>() as i32,
        ) as ::core::ffi::c_ulonglong;
        
        let version_ptr = std::ptr::addr_of!((*signHead).version) as *const ::core::ffi::c_uchar;
        (*signHead).version = crate::src_app_common::HapGetInt(
            version_ptr,
            std::mem::size_of::<u32>() as i32,
        ) as u32;
    }
}