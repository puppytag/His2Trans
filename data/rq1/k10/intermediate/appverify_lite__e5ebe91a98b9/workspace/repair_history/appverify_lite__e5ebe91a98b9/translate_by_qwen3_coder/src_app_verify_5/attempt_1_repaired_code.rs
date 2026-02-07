fn FindBlockHead(signInfo: *const crate::types::SignatureInfo, fp: i32, blockType: i32, block: *mut crate::types::BlockHead) -> i32 {
    use crate::types::{MAX_BLOCK_NUM, V_ERR, V_OK, SEEK_SET};
    unsafe {
        let signH = (*signInfo).signHead;
        if signH.is_null() {
            return V_ERR as i32;
        }
        let _ = libc::lseek(fp, (*signInfo).fullSignBlockOffset as libc::off_t, SEEK_SET as i32);
        let mut num = (*signH).blockNum as i32;
        if num > MAX_BLOCK_NUM as i32 {
            return V_ERR as i32;
        }
        while num > 0 {
            num -= 1;
            let read_len = libc::read(fp, block as *mut libc::c_void, std::mem::size_of::<crate::types::BlockHead>() as libc::size_t) as i32;
            if read_len != std::mem::size_of::<crate::types::BlockHead>() as i32 {
                return V_ERR as i32;
            }
            let type_val = crate::src_app_common::HapGetInt((&(*block).type_) as *const u32 as *const libc::c_uchar, std::mem::size_of::<u32>() as i32);
            if type_val == blockType {
                crate::src_app_verify::BlockHeadN2H(block);
                return V_OK as i32;
            }
        }
        V_ERR as i32
    }
}