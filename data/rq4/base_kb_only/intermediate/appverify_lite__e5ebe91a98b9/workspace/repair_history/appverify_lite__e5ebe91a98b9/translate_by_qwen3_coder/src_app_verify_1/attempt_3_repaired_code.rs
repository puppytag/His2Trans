fn SignHeadN2H(signHead: *mut crate::types::HwSignHead) {
    unsafe {
        let blocknum_ptr = std::ptr::addr_of_mut!((*signHead).blockNum) as *const ::core::ffi::c_uchar;
        let blocknum_val = crate::src_app_common::HapGetInt(
            blocknum_ptr,
            std::mem::size_of::<u32>() as i32,
        ) as u32;
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*signHead).blockNum), blocknum_val);
        
        let size_ptr = std::ptr::addr_of_mut!((*signHead).size) as *const ::core::ffi::c_uchar;
        let size_val = crate::src_app_common::HapGetInt64(
            size_ptr,
            std::mem::size_of::<::core::ffi::c_ulonglong>() as i32,
        ) as ::core::ffi::c_ulonglong;
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*signHead).size), size_val);
        
        let magiclow_ptr = std::ptr::addr_of_mut!((*signHead).magicLow) as *const ::core::ffi::c_uchar;
        let magiclow_val = crate::src_app_common::HapGetInt64(
            magiclow_ptr,
            std::mem::size_of::<::core::ffi::c_ulonglong>() as i32,
        ) as ::core::ffi::c_ulonglong;
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*signHead).magicLow), magiclow_val);
        
        let magichigh_ptr = std::ptr::addr_of_mut!((*signHead).magicHigh) as *const ::core::ffi::c_uchar;
        let magichigh_val = crate::src_app_common::HapGetInt64(
            magichigh_ptr,
            std::mem::size_of::<::core::ffi::c_ulonglong>() as i32,
        ) as ::core::ffi::c_ulonglong;
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*signHead).magicHigh), magichigh_val);
        
        let version_ptr = std::ptr::addr_of_mut!((*signHead).version) as *const ::core::ffi::c_uchar;
        let version_val = crate::src_app_common::HapGetInt(
            version_ptr,
            std::mem::size_of::<u32>() as i32,
        ) as u32;
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*signHead).version), version_val);
    }
}