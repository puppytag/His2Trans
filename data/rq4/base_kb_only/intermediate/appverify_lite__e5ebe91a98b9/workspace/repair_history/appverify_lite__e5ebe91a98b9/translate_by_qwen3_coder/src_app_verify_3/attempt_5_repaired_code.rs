fn ContentN2H(content: *mut crate::types::ContentInfo) {
    unsafe {
        let blocknum_ptr = &mut (*content).blockNum as *mut i32 as *const ::core::ffi::c_uchar;
        (*content).blockNum = HapGetInt(blocknum_ptr, std::mem::size_of::<i32>() as i32);
        
        let size_ptr = &mut (*content).size as *mut i32 as *const ::core::ffi::c_uchar;
        (*content).size = HapGetInt(size_ptr, std::mem::size_of::<i32>() as i32);
        
        let algid_ptr = &mut (*content).algId as *mut i32 as *const ::core::ffi::c_uchar;
        (*content).algId = HapGetInt(algid_ptr, std::mem::size_of::<i32>() as i32);
        
        let length_ptr = &mut (*content).length as *mut i32 as *const ::core::ffi::c_uchar;
        (*content).length = HapGetInt(length_ptr, std::mem::size_of::<i32>() as i32);
    }
}