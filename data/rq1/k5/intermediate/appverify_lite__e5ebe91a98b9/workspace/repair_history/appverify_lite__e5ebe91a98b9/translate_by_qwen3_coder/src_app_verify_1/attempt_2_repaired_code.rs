fn SignHeadN2H(signHead: *mut crate::types::HwSignHead) {
    unsafe {
        let block_num = std::ptr::read_unaligned((*signHead).blockNum as *const u32);
        (*signHead).blockNum = crate::src_app_common::HapGetInt(
            &block_num as *const u32 as *const ::core::ffi::c_uchar,
            std::mem::size_of::<u32>() as i32,
        ) as u32;
        let size_val = std::ptr::read_unaligned((*signHead).size as *const ::core::ffi::c_ulonglong);
        (*signHead).size = crate::src_app_common::HapGetInt64(
            &size_val as *const ::core::ffi::c_ulonglong as *const ::core::ffi::c_uchar,
            std::mem::size_of::<::core::ffi::c_ulonglong>() as i32,
        ) as ::core::ffi::c_ulonglong;
        let magic_low = std::ptr::read_unaligned((*signHead).magicLow as *const ::core::ffi::c_ulonglong);
        (*signHead).magicLow = crate::src_app_common::HapGetInt64(
            &magic_low as *const ::core::ffi::c_ulonglong as *const ::core::ffi::c_uchar,
            std::mem::size_of::<::core::ffi::c_ulonglong>() as i32,
        ) as ::core::ffi::c_ulonglong;
        let magic_high = std::ptr::read_unaligned((*signHead).magicHigh as *const ::core::ffi::c_ulonglong);
        (*signHead).magicHigh = crate::src_app_common::HapGetInt64(
            &magic_high as *const ::core::ffi::c_ulonglong as *const ::core::ffi::c_uchar,
            std::mem::size_of::<::core::ffi::c_ulonglong>() as i32,
        ) as ::core::ffi::c_ulonglong;
        let version_val = std::ptr::read_unaligned((*signHead).version as *const u32);
        (*signHead).version = crate::src_app_common::HapGetInt(
            &version_val as *const u32 as *const ::core::ffi::c_uchar,
            std::mem::size_of::<u32>() as i32,
        ) as u32;
    }
}