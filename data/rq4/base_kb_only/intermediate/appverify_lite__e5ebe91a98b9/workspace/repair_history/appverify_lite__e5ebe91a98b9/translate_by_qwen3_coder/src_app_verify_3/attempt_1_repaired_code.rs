fn ContentN2H(content: *mut crate::types::ContentInfo) {
    unsafe {
        (*content).blockNum = crate::src_app_common::HapGetInt(
            &(*content).blockNum as *const i32 as *const u8,
            std::mem::size_of::<i32>() as i32,
        );
        (*content).size = crate::src_app_common::HapGetInt(
            &(*content).size as *const i32 as *const u8,
            std::mem::size_of::<i32>() as i32,
        );
        (*content).algId = crate::src_app_common::HapGetInt(
            &(*content).algId as *const i32 as *const u8,
            std::mem::size_of::<i32>() as i32,
        );
        (*content).length = crate::src_app_common::HapGetInt(
            &(*content).length as *const i32 as *const u8,
            std::mem::size_of::<i32>() as i32,
        );
    }
}