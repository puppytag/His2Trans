fn GetEocd(hapFile: *const crate::types::FileRead, hapEocd: *mut crate::types::HapEocd, eocdOffset: *mut i32) -> bool {
    use crate::src_app_common::{HapGetInt, HapGetShort};
    use crate::src_app_file::{HapMMap, HapMUnMap};
    let mut mmapInfo = crate::types::MmapInfo {
        mmapPosition: 0,
        readMoreLen: 0,
        mmapSize: 0,
        mapAddr: std::ptr::null_mut(),
    };
    unsafe {
        if (*hapFile).len <= std::mem::size_of::<crate::types::MinEocd>() as i32 {
            return false;
        }
        let ret = HapMMap((*hapFile).len, 0, &mut mmapInfo as *mut _, hapFile);
        if ret != crate::types::V_OK as i32 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: mmap not ok\0".as_ptr() as *const _,
                b"GetEocd\0".as_ptr() as *const _,
                104,
            );
            return false;
        }
        let fileStart = (mmapInfo.mapAddr as *mut u8).offset(mmapInfo.readMoreLen as isize) as *mut i8;
        if HapGetShort(
            fileStart.offset((*hapFile).len as isize - std::mem::size_of::<i16>() as isize) as *const _,
            std::mem::size_of::<i16>() as i32,
        ) == 0
            && HapGetInt(
                fileStart.offset((*hapFile).len as isize - std::mem::size_of::<crate::types::MinEocd>() as isize) as *const _,
                std::mem::size_of::<i32>() as i32,
            ) == crate::types::HAP_EOCD_MAGIC as i32
        {
            if crate::compat::memcpy_s(
                &mut (*hapEocd).eocdHead as *mut _ as *mut _,
                std::mem::size_of::<crate::types::MinEocd>() as u64,
                fileStart.offset((*hapFile).len as isize - std::mem::size_of::<crate::types::MinEocd>() as isize) as *const _,
                std::mem::size_of::<crate::types::MinEocd>() as u64,
            ) != crate::types::EOK as i32
            {
                HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE as u32,
                    crate::types::LOG_ERROR as u32,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const _,
                    b"[%s:%d]: copy error\0".as_ptr() as *const _,
                    b"GetEocd\0".as_ptr() as *const _,
                    113,
                );
                return false;
            }
            HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
            *eocdOffset = (*hapFile).len - std::mem::size_of::<crate::types::MinEocd>() as i32;
            return true;
        }
        let maxReadLen = if ((*hapFile).len - std::mem::size_of::<crate::types::MinEocd>() as i32) as u32 > crate::types::UINT16_MAX_VALUE {
            crate::types::UINT16_MAX_VALUE as i32
        } else {
            (*hapFile).len - std::mem::size_of::<crate::types::MinEocd>() as i32
        };
        let fileStart = fileStart.offset(((*hapFile).len - std::mem::size_of::<crate::types::MinEocd>() as i32 - maxReadLen) as isize);
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_INFO as u32,
            0xD001100,
            b"appverify\0".as_ptr() as *const _,
            b"[%s:%d]: maxReadLen %d\0".as_ptr() as *const _,
            b"GetEocd\0".as_ptr() as *const _,
            124,
            maxReadLen,
        );
        for i in 0..maxReadLen {
            if HapGetShort(
                fileStart.offset((i + std::mem::size_of::<crate::types::MinEocd>() as i32 - std::mem::size_of::<i16>() as i32) as isize) as *const _,
                std::mem::size_of::<i16>() as i32,
            ) == (maxReadLen - i) as i16
                && HapGetInt(
                    fileStart.offset(i as isize) as *const _,
                    std::mem::size_of::<i32>() as i32,
                ) == crate::types::HAP_EOCD_MAGIC as i32
            {
                if crate::compat::memcpy_s(
                    &mut (*hapEocd).eocdHead as *mut _ as *mut _,
                    std::mem::size_of::<crate::types::MinEocd>() as u64,
                    fileStart.offset(i as isize) as *const _,
                    std::mem::size_of::<crate::types::MinEocd>() as u64,
                ) != crate::types::EOK as i32
                {
                    HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
                    let _ = crate::compat::HiLogPrint(
                        crate::types::LOG_CORE as u32,
                        crate::types::LOG_ERROR as u32,
                        0xD001100,
                        b"appverify\0".as_ptr() as *const _,
                        b"[%s:%d]: copy error\0".as_ptr() as *const _,
                        b"GetEocd\0".as_ptr() as *const _,
                        132,
                    );
                    return false;
                }
                HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE as u32,
                    crate::types::LOG_INFO as u32,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const _,
                    b"[%s:%d]: comment num %d\0".as_ptr() as *const _,
                    b"GetEocd\0".as_ptr() as *const _,
                    136,
                    maxReadLen - i,
                );
                *eocdOffset = (*hapFile).len - std::mem::size_of::<crate::types::MinEocd>() as i32 - (maxReadLen - i);
                return true;
            }
        }
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_ERROR as u32,
            0xD001100,
            b"appverify\0".as_ptr() as *const _,
            b"[%s:%d]: can not find eocd\0".as_ptr() as *const _,
            b"GetEocd\0".as_ptr() as *const _,
            141,
        );
        HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
        false
    }
}