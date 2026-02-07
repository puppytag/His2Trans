fn GetEocd(hapFile: *const crate::types::FileRead, hapEocd: *mut crate::types::HapEocd, eocdOffset: *mut i32) -> bool {
    use crate::types::*;
    
    let mut mmapInfo: MmapInfo = unsafe { std::mem::zeroed() };
    
    let min_eocd_size = std::mem::size_of::<MinEocd>() as i32;
    let short_size = std::mem::size_of::<i16>() as i32;
    let int_size = std::mem::size_of::<i32>() as i32;
    
    unsafe {
        if (*hapFile).len <= min_eocd_size {
            return false;
        }
        
        let ret = crate::src_app_file::HapMMap((*hapFile).len, 0, &mut mmapInfo, hapFile);
        if ret != V_OK as i32 {
            return false;
        }
        
        let file_start = (mmapInfo.mapAddr as isize + mmapInfo.readMoreLen as isize) as *mut i8;
        let file_len = (*hapFile).len;
        
        // Check for minimal EOCD at end of file
        let short_check_ptr = (file_start as isize + (file_len - short_size) as isize) as *const u8;
        let int_check_ptr = (file_start as isize + (file_len - min_eocd_size) as isize) as *const u8;
        
        if crate::src_app_common::HapGetShort(short_check_ptr, short_size) == 0 &&
           crate::src_app_common::HapGetInt(int_check_ptr, int_size) == HAP_EOCD_MAGIC as i32 {
            let src = (file_start as isize + (file_len - min_eocd_size) as isize) as *const ::core::ffi::c_void;
            let dst = &mut (*hapEocd).eocdHead as *mut MinEocd as *mut ::core::ffi::c_void;
            if memcpy_s(dst, min_eocd_size as u32, src, min_eocd_size as u32) != 0 {
                crate::src_app_file::HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
                return false;
            }
            crate::src_app_file::HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
            *eocdOffset = file_len - min_eocd_size;
            return true;
        }
        
        let max_read_len: i32 = if (file_len - min_eocd_size) as u32 > UINT16_MAX_VALUE {
            UINT16_MAX_VALUE as i32
        } else {
            file_len - min_eocd_size
        };
        
        let search_start = (file_start as isize + (file_len - min_eocd_size - max_read_len) as isize) as *mut i8;
        
        for i in 0..max_read_len {
            let short_pos = (search_start as isize + (i + min_eocd_size - short_size) as isize) as *const u8;
            let int_pos = (search_start as isize + i as isize) as *const u8;
            
            if crate::src_app_common::HapGetShort(short_pos, short_size) == (max_read_len - i) as i16 &&
               crate::src_app_common::HapGetInt(int_pos, int_size) == HAP_EOCD_MAGIC as i32 {
                let src = (search_start as isize + i as isize) as *const ::core::ffi::c_void;
                let dst = &mut (*hapEocd).eocdHead as *mut MinEocd as *mut ::core::ffi::c_void;
                if memcpy_s(dst, min_eocd_size as u32, src, min_eocd_size as u32) != 0 {
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