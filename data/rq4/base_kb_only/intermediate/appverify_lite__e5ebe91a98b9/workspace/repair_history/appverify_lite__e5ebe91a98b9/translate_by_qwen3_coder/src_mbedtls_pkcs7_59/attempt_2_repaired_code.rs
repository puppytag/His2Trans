fn GetSignerSignningCertDepth(signer: *const crate::types::SignerInfo) -> i32 {
    if crate::src_mbedtls_pkcs7::IsIncludeRoot(signer) {
        unsafe { (*signer).certPath.depth }
    } else {
        unsafe { (*signer).certPath.depth + 1 }
    }
}