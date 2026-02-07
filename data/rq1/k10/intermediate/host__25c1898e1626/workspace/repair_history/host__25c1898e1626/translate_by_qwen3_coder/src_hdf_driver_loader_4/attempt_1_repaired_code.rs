pub extern "C" fn HdfDriverLoaderConstruct(inst: *mut crate::types::HdfDriverLoader) {
    if !inst.is_null() {
        unsafe {
            (*inst).super_.GetDriver = Some(HdfDriverLoaderGetDriver);
            (*inst).super_.ReclaimDriver = Some(HdfDriverLoaderReclaimDriver);
        }
    }
}