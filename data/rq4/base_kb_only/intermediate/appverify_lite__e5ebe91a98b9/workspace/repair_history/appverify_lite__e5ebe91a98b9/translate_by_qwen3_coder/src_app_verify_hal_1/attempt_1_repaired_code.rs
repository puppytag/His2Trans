pub extern "C" fn RegistHalFunc() {
    unsafe {
        crate::compat::RegistBaseDefaultFunc(&mut crate::globals::g_productDiffFunc);
        crate::products_ipcamera_app_verify_base::RegistProductFunc(&mut crate::globals::g_productDiffFunc);
    }
}