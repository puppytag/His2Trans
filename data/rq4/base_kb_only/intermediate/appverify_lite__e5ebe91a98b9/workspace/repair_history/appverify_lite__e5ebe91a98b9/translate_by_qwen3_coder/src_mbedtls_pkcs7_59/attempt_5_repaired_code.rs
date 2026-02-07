fn GetSignerSignningCertDepth(signer: *const crate::types::SignerInfo) -> i32 {
    if IsIncludeRoot(signer) {
        unsafe { (*signer).certPath.depth }
    } else {
        unsafe { (*signer).certPath.depth + 1 }
    }
}