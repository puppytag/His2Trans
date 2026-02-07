fn GetEocd(hapFile: *const crate::types::FileRead, hapEocd: *mut crate::types::HapEocd, eocdOffset: *mut i32) -> bool {
    unsafe {
        let mut mmapInfo: crate::types::MmapInfo = std::mem::zeroed();
        
        let min_eocd_size = std::mem::size_of::<crate::types::MinEocd>() as i32;
        
        if (*hapFile).len <= min_eocd_size {
            return false;
        }
        
        let ret = crate::src_app_file::HapMMap((*hapFile).len, 0, &mut mmapInfo, hapFile);
        if ret != crate::types::V_OK as i32 {
            return false;
        }
        
        let file_start = mmapInfo.mapAddr.offset(mmapInfo.readMoreLen as isize);
        let file_len = (*hapFile).len;
        
        let short_size = std::mem::size_of::<i16>() as i32;
        let int_size = std::mem::size_of::<i32>() as i32;
        
        let short_check = crate::src_app_common::HapGetShort(
            file_start.offset((file_len - short_size) as isize) as *const u8,
            short_size
        );
        let magic_check = crate::src_app_common::HapGetInt(
            file_start.offset((file_len - min_eocd_size) as isize) as *const u8,
            int_size
        );
        
        if short_check == 0 && magic_check == crate::types::HAP_EOCD_MAGIC as i32 {
            let src = file_start.offset((file_len - min_eocd_size) as isize);
            let dest = &mut (*hapEocd).eocdHead as *mut crate::types::MinEocd as *mut ::core::ffi::c_void;
            let ret = memcpy_s(dest, min_eocd_size as u32, src as *const ::core::ffi::c_void, min_eocd_size as u32);
            if ret != 0 {
                crate::src_app_file::HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
                return false;
            }
            crate::src_app_file::HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
            *eocdOffset = file_len - min_eocd_size;
            return true;
        }
        
        let max_read_len: i32 = if (file_len - min_eocd_size) as u32 > crate::types::UINT16_MAX_VALUE {
            crate::types::UINT16_MAX_VALUE as i32
        } else {
            file_len - min_eocd_size
        };
        
        let search_start = file_start.offset((file_len - min_eocd_size - max_read_len) as isize);
        
        for i in 0..max_read_len {
            let comment_len_check = crate::src_app_common::HapGetShort(
                search_start.offset((i + min_eocd_size - short_size) as isize) as *const u8,
                short_size
            );
            let magic_check = crate::src_app_common::HapGetInt(
                search_start.offset(i as isize) as *const u8,
                int_size
            );
            
            if comment_len_check as i32 == (max_read_len - i) && magic_check == crate::types::HAP_EOCD_MAGIC as i32 {
                let src = search_start.offset(i as isize);
                let dest = &mut (*hapEocd).eocdHead as *mut crate::types::MinEocd as *mut ::core::ffi::c_void;
                let ret = memcpy_s(dest, min_eocd_size as u32, src as *const ::core::ffi::c_void, min_eocd_size as u32);
                if ret != 0 {
                    crate::src_app_file::HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
                    return false;
                }
                crate::src_app_file::HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
                *eocdOffset = file_len - min_eocd_size - (max_read_len - i);
                return true;
            }
        }
        
        crate::src_app_file::HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
        false
    }
}