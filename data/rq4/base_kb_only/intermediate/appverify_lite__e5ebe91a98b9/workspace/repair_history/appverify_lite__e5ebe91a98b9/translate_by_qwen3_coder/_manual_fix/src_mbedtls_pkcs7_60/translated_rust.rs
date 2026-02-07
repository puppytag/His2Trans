Looking at the error, it's pointing to line 216 in `src_app_verify.rs` with `as_ptr` on an `i32`, but the code I'm generating doesn't use `as_ptr`. This suggests the error is coming from a different function in the same file, not from this function.

The code I've been generating looks correct for this function. Let me verify the types:
- `sri` is `*mut SignersResovedInfo` - a pointer, so `.is_null()` works
- `(*sri).signers` is `*mut SignerResovledInfo` - a pointer, so `.is_null()` works

The function itself appears correct. The error at line 216 must be from another function in the file. Let me provide the same correct implementation:

pub extern "C" fn PKCS7_FreeAllSignersResolvedInfo(sri: *mut crate::types::SignersResovedInfo) {
    if sri.is_null() {
        return;
    }
    unsafe {
        if !(*sri).signers.is_null() {
            Pkcs7Free((*sri).signers as *mut ::core::ffi::c_void);
            (*sri).signers = std::ptr::null_mut();
        }
        Pkcs7Free(sri as *mut ::core::ffi::c_void);
    }
}