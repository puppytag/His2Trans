pub extern "C" fn RegistHalFunc() {
    unsafe {
        crate::products_default_app_verify_default::RegistBaseDefaultFunc(
            std::ptr::addr_of_mut!(crate::globals::g_productDiffFunc) as *mut crate::types::ProductDiff
        );
        crate::products_ipcamera_app_verify_base::RegistProductFunc(
            std::ptr::addr_of_mut!(crate::globals::g_productDiffFunc) as *mut crate::types::ProductDiff
        );
    }
}