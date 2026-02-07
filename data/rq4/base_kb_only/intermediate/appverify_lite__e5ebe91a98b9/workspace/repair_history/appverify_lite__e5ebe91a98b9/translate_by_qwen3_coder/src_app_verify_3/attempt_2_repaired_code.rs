fn ContentN2H(content: *mut crate::types::ContentInfo) {
    unsafe {
        (*content).blockNum = HapGetInt(
            &(*content).blockNum as *const i32 as *const u8,
            std::mem::size_of::<i32>() as i32,
        );
        (*content).size = HapGetInt(
            &(*content).size as *const i32 as *const u8,
            std::mem::size_of::<i32>() as i32,
        );
        (*content).algId = HapGetInt(
            &(*content).algId as *const i32 as *const u8,
            std::mem::size_of::<i32>() as i32,
        );
        (*content).length = HapGetInt(
            &(*content).length as *const i32 as *const u8,
            std::mem::size_of::<i32>() as i32,
        );
    }
}