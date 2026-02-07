pub extern "C" fn PKCS7_FreeRes(pkcs7: *mut crate::types::Pkcs7) {
    if pkcs7.is_null() {
        return;
    }
    FreeSignedDataDigestAlgs(pkcs7);
    FreeSignersInfo(pkcs7);
    FreeSignedDataCerts(pkcs7);
    FreeSignedDataCrl(pkcs7);
    UnLoadRootCert();
    UnLoadSelfSignedCert();
}