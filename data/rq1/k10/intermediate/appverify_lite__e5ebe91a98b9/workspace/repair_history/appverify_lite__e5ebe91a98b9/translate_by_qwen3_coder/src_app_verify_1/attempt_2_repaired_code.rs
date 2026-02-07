fn SignHeadN2H(signHead: *mut crate::types::HwSignHead) {
    unsafe {
        let block_num_ptr = (&mut (*signHead).blockNum as *mut u32) as *mut ::core::ffi::c_uchar;
        (*signHead).blockNum = crate::src_app_common::HapGetInt(block_num_ptr, std::mem::size_of::<u32>() as i32) as u32;
        let size_ptr = (&mut (*signHead).size as *mut ::core::ffi::c_ulonglong) as *mut ::core::ffi::c_uchar;
        (*signHead).size = crate::src_app_common::HapGetInt64(size_ptr, std::mem::size_of::<::core::ffi::c_ulonglong>() as i32) as ::core::ffi::c_ulonglong;
        let magic_low_ptr = (&mut (*signHead).magicLow as *mut ::core::ffi::c_ulonglong) as *mut ::core::ffi::c_uchar;
        (*signHead).magicLow = crate::src_app_common::HapGetInt64(magic_low_ptr, std::mem::size_of::<::core::ffi::c_ulonglong>() as i32) as ::core::ffi::c_ulonglong;
        let magic_high_ptr = (&mut (*signHead).magicHigh as *mut ::core::ffi::c_ulonglong) as *mut ::core::ffi::c_uchar;
        (*signHead).magicHigh = crate::src_app_common::HapGetInt64(magic_high_ptr, std::mem::size_of::<::core::ffi::c_ulonglong>() as i32) as ::core::ffi::c_ulonglong;
        let version_ptr = (&mut (*signHead).version as *mut u32) as *mut ::core::ffi::c_uchar;
        (*signHead).version = crate::src_app_common::HapGetInt(version_ptr, std::mem::size_of::<u32>() as i32) as u32;
    }
}