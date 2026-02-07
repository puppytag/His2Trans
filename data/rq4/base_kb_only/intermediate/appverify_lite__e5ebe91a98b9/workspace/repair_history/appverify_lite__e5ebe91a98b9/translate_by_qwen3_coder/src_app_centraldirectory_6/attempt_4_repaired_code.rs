fn GetEocd(hapFile: *const crate::types::FileRead, hapEocd: *mut crate::types::HapEocd, eocdOffset: *mut i32) -> bool {
    let mut mmapInfo: crate::types::MmapInfo = unsafe { std::mem::zeroed() };
    
    let min_eocd_size = std::mem::size_of::<crate::types::MinEocd>() as i32;
    let short_size = std::mem::size_of::<i16>() as i32;
    let int_size = std::mem::size_of::<i32>() as i32;
    
    let hap_len = unsafe { (*hapFile).len };
    
    if hap_len <= min_eocd_size {
        return false;
    }
    
    let ret = crate::src_app_file::HapMMap(hap_len, 0, &mut mmapInfo, hapFile);
    if ret != crate::types::V_OK as i32 {
        return false;
    }
    
    let file_start = unsafe { mmapInfo.mapAddr.offset(mmapInfo.readMoreLen as isize) };
    
    let short_offset = hap_len - short_size;
    let eocd_offset_pos = hap_len - min_eocd_size;
    
    let short_val = crate::src_app_common::HapGetShort(
        unsafe { file_start.offset(short_offset as isize) as *const u8 },
        short_size
    );
    let magic_val = crate::src_app_common::HapGetInt(
        unsafe { file_start.offset(eocd_offset_pos as isize) as *const u8 },
        int_size
    );
    
    if short_val == 0 && magic_val == crate::types::HAP_EOCD_MAGIC as i32 {
        let src = unsafe { file_start.offset(eocd_offset_pos as isize) };
        let copy_ret = unsafe {
            crate::compat::memcpy_s(
                &mut (*hapEocd).eocdHead as *mut crate::types::MinEocd as *mut core::ffi::c_void,
                min_eocd_size as u32,
                src as *const core::ffi::c_void,
                min_eocd_size as u32
            )
        };
        if copy_ret != 0 {
            crate::src_app_file::HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
            return false;
        }
        crate::src_app_file::HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
        unsafe { *eocdOffset = eocd_offset_pos };
        return true;
    }
    
    let max_search = hap_len - min_eocd_size;
    let max_read_len: i32 = if (max_search as u32) > crate::types::UINT16_MAX_VALUE {
        crate::types::UINT16_MAX_VALUE as i32
    } else {
        max_search
    };
    
    let search_start = unsafe { file_start.offset((hap_len - min_eocd_size - max_read_len) as isize) };
    
    for i in 0..max_read_len {
        let comment_len_check = crate::src_app_common::HapGetShort(
            unsafe { search_start.offset((i + min_eocd_size - short_size) as isize) as *const u8 },
            short_size
        );
        let magic_check = crate::src_app_common::HapGetInt(
            unsafe { search_start.offset(i as isize) as *const u8 },
            int_size
        );
        
        if comment_len_check as i32 == (max_read_len - i) && magic_check == crate::types::HAP_EOCD_MAGIC as i32 {
            let src = unsafe { search_start.offset(i as isize) };
            let copy_ret = unsafe {
                crate::compat::memcpy_s(
                    &mut (*hapEocd).eocdHead as *mut crate::types::MinEocd as *mut core::ffi::c_void,
                    min_eocd_size as u32,
                    src as *const core::ffi::c_void,
                    min_eocd_size as u32
                )
            };
            if copy_ret != 0 {
                crate::src_app_file::HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
                return false;
            }
            crate::src_app_file::HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
            unsafe { *eocdOffset = hap_len - min_eocd_size - (max_read_len - i) };
            return true;
        }
    }
    
    crate::src_app_file::HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
    false
}