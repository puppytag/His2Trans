pub extern "C" fn RegistHalFunc() {
    unsafe {
        crate::products_default_app_verify_default::RegistBaseDefaultFunc(
            &mut crate::globals::g_productDiffFunc as *mut _ as *mut crate::types::ProductDiff
        );
        crate::products_ipcamera_app_verify_base::RegistProductFunc(
            &mut crate::globals::g_productDiffFunc as *mut _ as *mut crate::types::ProductDiff
        );
    }
}