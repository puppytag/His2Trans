//! Module: src_app_provision
//!
//! Auto-generated skeleton - function bodies are unimplemented.

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::types::*;
use crate::globals::*;
use crate::compat::*;

fn ProfInit(pf: *mut crate::types::ProfileProf) {
    if pf.is_null() {
        return;
    }
    // Only the write_bytes on a raw pointer requires unsafe; null-check is safe.
    unsafe {
        std::ptr::write_bytes(pf as *mut u8, 0u8, std::mem::size_of::<crate::types::ProfileProf>());
    }
}

fn GetStringTag(root: *const crate::types::cJSON, tag: *const std::ffi::c_char) -> *mut std::ffi::c_char {
    let jsonObj = find_child_by_tag(root, tag);
    if jsonObj.is_null() {
        return std::ptr::null_mut();
    }
    let src = unsafe { (*jsonObj).valuestring };
    if src.is_null() {
        return std::ptr::null_mut();
    }
    let objLen = unsafe { libc::strlen(src) } as i32;
    if objLen < 0 {
        return std::ptr::null_mut();
    }
    let alloc_size = (objLen + 1) as usize;
    let value = unsafe { libc::malloc(alloc_size) } as *mut std::ffi::c_char;
    if value.is_null() {
        return std::ptr::null_mut();
    }
    let src_len = objLen as usize;
    if src_len + 1 > alloc_size {
        return std::ptr::null_mut();
    }
    unsafe { std::ptr::copy_nonoverlapping(src as *const u8, value as *mut u8, src_len + 1); }
    value
}

fn find_child_by_tag(root: *const crate::types::cJSON, tag: *const std::ffi::c_char) -> *mut crate::types::cJSON {
    // Safe wrapper around cJSON child walk; ensures minimal unsafe scope.
    let mut item = unsafe { (*root).child };
    while !item.is_null() {
        let item_str = unsafe { (*item).string };
        if !item_str.is_null() && unsafe { libc::strcmp(item_str, tag) == 0 } {
            return item;
        }
        item = unsafe { (*item).next };
    }
    std::ptr::null_mut()
}

fn FreeStringAttay(array: *mut *mut std::ffi::c_char, num: i32) {
    if array.is_null() {
        return;
    }
    for i in 0..num {
        let ptr = unsafe { *array.offset(i as isize) };
        if !ptr.is_null() {
            unsafe {
                libc::free(ptr as *mut ::core::ffi::c_void);
            }
            unsafe { *array.offset(i as isize) = std::ptr::null_mut(); }
        }
    }
    unsafe {
        libc::free(array as *mut ::core::ffi::c_void);
    }
}

fn GetStringArrayTag(root: *const crate::types::cJSON, tag: *const std::ffi::c_char, numReturn: *mut i32) -> *mut *mut std::ffi::c_char {
    if root.is_null() || numReturn.is_null() {
        return std::ptr::null_mut();
    }
    // Find matching child object using a helper (safe)
    let json_obj = find_child_by_tag(root, tag);
    if json_obj.is_null() {
        return std::ptr::null_mut();
    }
    // unsafe: traverse cJSON child list to count children
    let num = {
        let mut n: i32 = 0;
        let mut item_ptr = unsafe { (*json_obj).child };
        while !item_ptr.is_null() {
            n += 1;
            item_ptr = unsafe { (*item_ptr).next };
        }
        n
    };
    if num == 0 {
        // unsafe: raw pointer deref for out-param
        unsafe { *numReturn = 0; }
        return std::ptr::null_mut();
    }
    // unsafe: FFI malloc call
    let value = unsafe {
        libc::malloc((num as usize) * std::mem::size_of::<*mut std::ffi::c_char>())
    } as *mut *mut std::ffi::c_char;
    if value.is_null() {
        // unsafe: raw pointer deref for out-param
        unsafe { *numReturn = 0; }
        return std::ptr::null_mut();
    }
    // unsafe: raw pointer write_bytes
    unsafe { std::ptr::write_bytes(value as *mut u8, 0, (num as usize) * std::mem::size_of::<*mut std::ffi::c_char>()); }
    // Fill array. unsafe: traverse cJSON child list, read valuestring, FFI strlen/malloc/copy
    let mut cur_item = unsafe { (*json_obj).child };
    for i in 0..num {
        if cur_item.is_null() || unsafe { (*cur_item).valuestring.is_null() } {
            crate::src_app_provision::FreeStringAttay(value, num);
            return std::ptr::null_mut();
        }
        let len = unsafe { libc::strlen((*cur_item).valuestring as *const i8) } as usize;
        let str_copy = unsafe { libc::malloc(len + 1) } as *mut std::ffi::c_char;
        if str_copy.is_null() {
            crate::src_app_provision::FreeStringAttay(value, num);
            return std::ptr::null_mut();
        }
        // unsafe: raw pointer copy_nonoverlapping and write
        unsafe { std::ptr::copy_nonoverlapping((*cur_item).valuestring as *const u8, str_copy as *mut u8, len + 1); }
        unsafe { *value.offset(i as isize) = str_copy; }
        cur_item = unsafe { (*cur_item).next };
    }
    // unsafe: raw pointer deref for out-param
    unsafe { *numReturn = num; }
    value
}

fn find_child_by_name(root: *const crate::types::cJSON, name: &[u8]) -> *mut crate::types::cJSON {
    let target = unsafe { ::core::ffi::CStr::from_ptr(name.as_ptr() as *const ::core::ffi::c_char) };
    let mut c = unsafe { (*root).child };
    while !c.is_null() {
        let c_ref = unsafe { &*c };
        let matches = if !c_ref.string.is_null() {
            let cur_cstr = unsafe { ::core::ffi::CStr::from_ptr(c_ref.string) };
            cur_cstr == target
        } else {
            false
        };
        if matches {
            return c;
        }
        c = c_ref.next;
    }
    std::ptr::null_mut()
}

fn GetProfValidity(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfValidity) -> i32 {
    use core::ffi::CStr;
    let json_obj = find_child_by_name(root, b"validity\0");
    if json_obj.is_null() {
        return V_ERR as i32;
    }
    let not_before = find_child_by_name(json_obj, b"not-before\0");
    if not_before.is_null() {
        return V_ERR as i32;
    }
    unsafe { (*profVal).notBefore = (*not_before).valueint; }
    let not_after = find_child_by_name(json_obj, b"not-after\0");
    if not_after.is_null() {
        return V_ERR as i32;
    }
    unsafe { (*profVal).notAfter = (*not_after).valueint; }
    V_OK as i32
}


fn GetProfBundleInfo(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfBundleInfo)-> i32 {
    // Find "bundle-info" child node using safe traversal with minimal unsafe derefs
    let root_ref = unsafe { &*root };
    let key = b"bundle-info\0";
    let key_ptr = key.as_ptr() as *const ::core::ffi::c_char;
    let mut cur = root_ref.child;
    let mut found: *mut crate::types::cJSON = ::core::ptr::null_mut();
    while !cur.is_null() {
        let cur_ref = unsafe { &*cur };
        let cur_string = cur_ref.string;
        let cmp = unsafe { libc::strcmp(key_ptr, cur_string) };
        if cmp == 0 {
            found = cur;
            break;
        }
        cur = cur_ref.next;
    }
    let json_obj = found;

    if json_obj.is_null() {
        eprintln!("[{}:{}] failed to get bundle-info", "GetProfBundleInfo", 156);
        return V_ERR as i32;
    }

    // Unsafe: deref raw pointer to obtain mutable reference
    let pf = unsafe { &mut *profVal };

    pf.developerId = crate::src_app_provision::GetStringTag(
        json_obj as *const crate::types::cJSON,
        b"developer-id\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if pf.developerId.is_null() {
        eprintln!("[{}:{}] profVal->developerId is null", "GetProfBundleInfo", 161);
        return V_ERR as i32;
    }

    let dev_cert_i8 = crate::src_app_provision::GetStringTag(
        json_obj as *const crate::types::cJSON,
        b"development-certificate\0".as_ptr() as *const ::core::ffi::c_char,
    );
    pf.devCert = dev_cert_i8 as *mut ::core::ffi::c_uchar;
    if pf.devCert.is_null() {
        eprintln!("[{}:{}] get development-certificate failed", "GetProfBundleInfo", 165);
        pf.devCert = unsafe { libc::malloc(1) as *mut ::core::ffi::c_uchar };
        if pf.devCert.is_null() {
            eprintln!("[{}:{}] profVal->devCert is null", "GetProfBundleInfo", 167);
            return V_ERR as i32;
        }
        unsafe { *pf.devCert = 0u8; }
    }

    let rel_cert_i8 = crate::src_app_provision::GetStringTag(
        json_obj as *const crate::types::cJSON,
        b"distribution-certificate\0".as_ptr() as *const ::core::ffi::c_char,
    );
    pf.releaseCert = rel_cert_i8 as *mut ::core::ffi::c_uchar;
    if pf.releaseCert.is_null() {
        eprintln!("[{}:{}] get distribution-certificate failed", "GetProfBundleInfo", 173);
        pf.releaseCert = unsafe { libc::malloc(1) as *mut ::core::ffi::c_uchar };
        if pf.releaseCert.is_null() {
            eprintln!("[{}:{}] profVal->releaseCert is null", "GetProfBundleInfo", 175);
            return V_ERR as i32;
        }
        unsafe { *pf.releaseCert = 0u8; }
    }

    pf.bundleName = crate::src_app_provision::GetStringTag(
        json_obj as *const crate::types::cJSON,
        b"bundle-name\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if pf.bundleName.is_null() {
        eprintln!("[{}:{}] profVal->bundleName is null", "GetProfBundleInfo", 180);
        return V_ERR as i32;
    }

    pf.appFeature = crate::src_app_provision::GetStringTag(
        json_obj as *const crate::types::cJSON,
        b"app-feature\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if pf.appFeature.is_null() {
        eprintln!("[{}:{}] profVal->appFeature is null", "GetProfBundleInfo", 183);
        return V_ERR as i32;
    }

    V_OK as i32
}

fn GetProfPermission(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfPermission)-> i32 {
    if root.is_null() {
        return crate::types::V_ERR as i32;
    }
    let root_ref = unsafe { &*root };
    let mut current = root_ref.child;
    let key = std::ffi::CStr::from_bytes_with_nul(b"permissions\x00").unwrap();
    let mut found: *mut crate::types::cJSON = std::ptr::null_mut();
    while !current.is_null() {
        let cur_ref = unsafe { &*current };
        if !cur_ref.string.is_null() {
            let cur_key = unsafe { std::ffi::CStr::from_ptr(cur_ref.string as *const i8) };
            if cur_key == key {
                found = current;
                break;
            }
        }
        current = cur_ref.next;
    }
    if found.is_null() {
        return crate::types::V_ERR as i32;
    }
    let prof_ref = unsafe { &mut *profVal };
    let feature_tag = std::ffi::CStr::from_bytes_with_nul(b"feature-permissions\x00").unwrap();
    prof_ref.permission = crate::src_app_provision::GetStringArrayTag(
        found as *const crate::types::cJSON,
        feature_tag.as_ptr(),
        &mut prof_ref.permissionNum as *mut i32,
    );
    let restr_tag = std::ffi::CStr::from_bytes_with_nul(b"restricted-permissions\x00").unwrap();
    prof_ref.restricPermission = crate::src_app_provision::GetStringArrayTag(
        found as *const crate::types::cJSON,
        restr_tag.as_ptr(),
        &mut prof_ref.restricNum as *mut i32,
    );
    crate::types::V_OK as i32
}

fn GetProfDebugInfo(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfDebugInfo)-> i32 {
    if root.is_null() {
        return 0; 
    }
    let json_obj = {
        let mut result: *mut crate::types::cJSON = std::ptr::null_mut();
        let root_ref = unsafe { &*root };
        let mut cur = root_ref.child;
        while !cur.is_null() {
            let cur_ref = unsafe { &*cur };
            let cur_string = cur_ref.string;
            if !cur_string.is_null() {
                let name = unsafe { std::ffi::CStr::from_ptr(cur_string as *const std::ffi::c_char) };
                if name.to_bytes() == b"debug\x2Dinfo" {
                    result = cur;
                    break;
                }
            }
            cur = cur_ref.next;
        }
        result
    };
    if json_obj.is_null() {
        return 0;
    }
    let dev_id_type = crate::src_app_provision::GetStringTag(
        json_obj,
        b"device-id-type\x00".as_ptr() as *const std::ffi::c_char,
    );
    let prof_ref = unsafe { &mut *profVal };
    prof_ref.devIdType = dev_id_type;
    if dev_id_type.is_null() {
        return 0;
    }
    let mut devid_num: i32 = 0;
    let device_id_array = crate::src_app_provision::GetStringArrayTag(
        json_obj,
        b"device-ids\x00".as_ptr() as *const std::ffi::c_char,
        &mut devid_num,
    );
    prof_ref.deviceId = device_id_array;
    prof_ref.devidNum = devid_num;
    0
}

fn GetProfIssuerInfo(root: *const crate::types::cJSON, pf: *mut crate::types::ProfileProf)-> i32 {
    let issuer_ptr = crate::src_app_provision::GetStringTag(
        root,
        b"issuer\x00".as_ptr() as *const std::ffi::c_char,
    );
    let pf_ref = unsafe { &mut *pf };
    pf_ref.issuer = issuer_ptr;
    if !issuer_ptr.is_null() {
        return crate::types::V_OK as i32;
    }
    let app_store_ptr = crate::types::APP_STORE.as_ptr() as *const std::ffi::c_char;
    let len = unsafe { libc::strlen(app_store_ptr) };
    let malloc_size = (len).wrapping_add(1) as usize;
    let allocated = unsafe { libc::malloc(malloc_size) as *mut std::ffi::c_char };
    if allocated.is_null() {
        return crate::types::V_ERR as i32;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(
            app_store_ptr as *const u8,
            allocated as *mut u8,
            malloc_size,
        );
    }
    pf_ref.issuer = allocated;
    crate::types::V_OK as i32
}
// === C2R_FAILED_TRANSLATION_BEGIN func_key: src_app_provision_9 ===
// fn GetProfIssuerInfo(root: *const crate::cJSON, pf: *mut crate::ProfileProf) -> i32 {
//     unsafe {
//         let tag = std::ffi::CStr::from_bytes_with_nul(b"issuer\0").unwrap().as_ptr();
//         let issuer_ptr = crate::src_app_provision::GetStringTag(root, tag);
//         (*pf).issuer = issuer_ptr;
//         if issuer_ptr.is_null() {
//             let app_store_ptr = crate::globals::APP_STORE.as_ptr() as *const ::core::ffi::c_char;
//             let len = libc::strlen(app_store_ptr);
//             let allocated_ptr = libc::malloc((len + 1) as usize) as *mut ::core::ffi::c_char;
//             (*pf).issuer = allocated_ptr;
//             if allocated_ptr.is_null() {
//                 return -1;
//             }
//             std::ptr::copy_nonoverlapping(
//                 app_store_ptr as *const u8,
//                 allocated_ptr as *mut u8,
//                 (len + 1) as usize,
//             );
//             return 0;
//         }
//         0
//     }
// }
// === C2R_FAILED_TRANSLATION_END func_key: src_app_provision_9 ===


fn FreeProfBundle(pfval: *mut crate::types::ProfBundleInfo) {
    if pfval.is_null() { return; }
    let pf = unsafe { &mut *pfval };
    // Narrow unsafe per free call; null checks and field reassignments are safe.
    if !pf.appFeature.is_null() {
        unsafe { libc::free(pf.appFeature as *mut ::core::ffi::c_void); }
    }
    pf.appFeature = ::core::ptr::null_mut();
    if !pf.bundleName.is_null() {
        unsafe { libc::free(pf.bundleName as *mut ::core::ffi::c_void); }
    }
    pf.bundleName = ::core::ptr::null_mut();
    if !pf.devCert.is_null() {
        unsafe { libc::free(pf.devCert as *mut ::core::ffi::c_void); }
    }
    pf.devCert = ::core::ptr::null_mut();
    if !pf.developerId.is_null() {
        unsafe { libc::free(pf.developerId as *mut ::core::ffi::c_void); }
    }
    pf.developerId = ::core::ptr::null_mut();
    if !pf.releaseCert.is_null() {
        unsafe { libc::free(pf.releaseCert as *mut ::core::ffi::c_void); }
    }
    pf.releaseCert = ::core::ptr::null_mut();
}

fn FreeProfPerssion(pfval: *mut crate::types::ProfPermission) {
    let pf = unsafe { &mut *pfval };
    crate::src_app_provision::FreeStringAttay(pf.permission, pf.permissionNum);
    pf.permissionNum = 0;
    pf.permission = std::ptr::null_mut();
    crate::src_app_provision::FreeStringAttay(pf.restricPermission, pf.restricNum);
    pf.restricNum = 0;
    pf.restricPermission = std::ptr::null_mut();
}

fn FreeProfDebuginfo(pfval: *mut crate::types::ProfDebugInfo) {
    if pfval.is_null() { return; }
    let pf = unsafe { &mut *pfval };
    if !pf.devIdType.is_null() {
        unsafe { libc::free(pf.devIdType as *mut libc::c_void); }
    }
    pf.devIdType = std::ptr::null_mut();
    crate::src_app_provision::FreeStringAttay(pf.deviceId, pf.devidNum);
    pf.devidNum = 0;
    pf.deviceId = std::ptr::null_mut();
}

pub extern "C" fn ProfFreeData(pf: *mut crate::types::ProfileProf) {
    if pf.is_null() {
        return;
    }
    let pf_ref = unsafe { &mut *pf };
    if !pf_ref.versionName.is_null() {
        unsafe { libc::free(pf_ref.versionName as *mut ::core::ffi::c_void); }
        pf_ref.versionName = std::ptr::null_mut();
    }
    if !pf_ref.uuid.is_null() {
        unsafe { libc::free(pf_ref.uuid as *mut ::core::ffi::c_void); }
        pf_ref.uuid = std::ptr::null_mut();
    }
    if !pf_ref.type_.is_null() {
        unsafe { libc::free(pf_ref.type_ as *mut ::core::ffi::c_void); }
        pf_ref.type_ = std::ptr::null_mut();
    }
    if !pf_ref.appDistType.is_null() {
        unsafe { libc::free(pf_ref.appDistType as *mut ::core::ffi::c_void); }
        pf_ref.appDistType = std::ptr::null_mut();
    }
    crate::src_app_provision::FreeProfBundle(&mut pf_ref.bundleInfo as *mut crate::types::ProfBundleInfo);
    crate::src_app_provision::FreeProfPerssion(&mut pf_ref.permission as *mut crate::types::ProfPermission);
    crate::src_app_provision::FreeProfDebuginfo(&mut pf_ref.debugInfo as *mut crate::types::ProfDebugInfo);
    if !pf_ref.issuer.is_null() {
        unsafe { libc::free(pf_ref.issuer as *mut ::core::ffi::c_void); }
        pf_ref.issuer = std::ptr::null_mut();
    }
    if !pf_ref.appid.is_null() {
        unsafe { libc::free(pf_ref.appid as *mut ::core::ffi::c_void); }
        pf_ref.appid = std::ptr::null_mut();
    }
}

pub extern "C" fn ParseProfile(buf: *const ::core::ffi::c_char, len: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    if pf.is_null() || buf.is_null() {
        return crate::types::V_ERR as i32;
    }
    crate::src_app_provision::ProfInit(pf);

    let pf_ref = unsafe { &mut *pf };
    let mut pfStr: *const ::core::ffi::c_char = ::core::ptr::null();
    {
        let mut p = buf;
        loop {
            let ch = unsafe { *p };
            if ch == 0 {
                break;
            }
            if ch == b'{' as i8 {
                pfStr = p;
                break;
            }
            p = unsafe { p.offset(1) };
        }
    }
    if pfStr.is_null() {
        return crate::types::V_ERR as i32;
    }

    extern "C" {
        fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut crate::types::cJSON;
        fn cJSON_GetObjectItem(object: *const crate::types::cJSON, string: *const ::core::ffi::c_char) -> *mut crate::types::cJSON;
        fn cJSON_Delete(c: *mut crate::types::cJSON);
        fn malloc(size: usize) -> *mut ::core::ffi::c_void;
    }

    let root = unsafe { cJSON_Parse(pfStr) };
    if root.is_null() {
        return crate::types::V_ERR as i32;
    }

    let mut ret_val = crate::types::V_OK as i32;
    loop {
        let jsonObj = unsafe { cJSON_GetObjectItem(root, b"version-code\0".as_ptr() as *const i8) };
        if jsonObj.is_null() {
            ret_val = crate::types::V_ERR as i32;
            break;
        }
        let jsonObj_ref = unsafe { &*jsonObj };
        pf_ref.versionCode = jsonObj_ref.valueint;

        pf_ref.versionName = crate::src_app_provision::GetStringTag(root, b"version-name\0".as_ptr() as *const i8);
        if pf_ref.versionName.is_null() {
            ret_val = crate::types::V_ERR as i32;
            break;
        }

        pf_ref.uuid = crate::src_app_provision::GetStringTag(root, b"uuid\0".as_ptr() as *const i8);
        if pf_ref.uuid.is_null() {
            ret_val = crate::types::V_ERR as i32;
            break;
        }

        pf_ref.type_ = crate::src_app_provision::GetStringTag(root, b"type\0".as_ptr() as *const i8);
        if pf_ref.type_.is_null() {
            ret_val = crate::types::V_ERR as i32;
            break;
        }

        let appDistType = crate::src_app_provision::GetStringTag(root, b"app-distribution-type\0".as_ptr() as *const i8);
        if appDistType.is_null() {
            let buf = unsafe { malloc(1) as *mut i8 };
            if buf.is_null() {
                ret_val = crate::types::V_ERR as i32;
                break;
            }
            unsafe { *buf = 0; }
            pf_ref.appDistType = buf;
        } else {
            pf_ref.appDistType = appDistType;
        }

        let ret = crate::src_app_provision::GetProfValidity(root, &mut pf_ref.validity as *mut crate::types::ProfValidity);
        if ret != crate::types::V_OK as i32 {
            ret_val = crate::types::V_ERR as i32;
            break;
        }

        let ret = crate::src_app_provision::GetProfBundleInfo(root, &mut pf_ref.bundleInfo as *mut crate::types::ProfBundleInfo);
        if ret != crate::types::V_OK as i32 {
            ret_val = crate::types::V_ERR as i32;
            break;
        }

        let ret = crate::src_app_provision::GetProfPermission(root, &mut pf_ref.permission as *mut crate::types::ProfPermission);
        if ret != crate::types::V_OK as i32 {
            ret_val = crate::types::V_ERR as i32;
            break;
        }

        let ret = crate::src_app_provision::GetProfDebugInfo(root, &mut pf_ref.debugInfo as *mut crate::types::ProfDebugInfo);
        if ret != crate::types::V_OK as i32 {
            ret_val = crate::types::V_ERR as i32;
            break;
        }

        let ret = crate::src_app_provision::GetProfIssuerInfo(root, pf);
        if ret != crate::types::V_OK as i32 {
            ret_val = crate::types::V_ERR as i32;
            break;
        }

        break;
    }

    if ret_val != crate::types::V_OK as i32 {
        unsafe { cJSON_Delete(root); }
        crate::src_app_provision::ProfFreeData(pf);
        return crate::types::V_ERR as i32;
    }

    unsafe { cJSON_Delete(root); }
    crate::types::V_OK as i32
}

fn VerifyAppTypeAndDistribution(pf: *const crate::types::ProfileProf) -> i32 {
    let type_cstr = unsafe { ::std::ffi::CStr::from_ptr((*pf).type_ as *const ::core::ffi::c_char) };
    if type_cstr.to_bytes() != b"debug" && type_cstr.to_bytes() != b"release" {
        return crate::types::V_ERR as i32;
    }
    if type_cstr.to_bytes() == b"release" {
        let app_dist_type = unsafe { ::std::ffi::CStr::from_ptr((*pf).appDistType as *const ::core::ffi::c_char) };
        let app_gallery = crate::globals::app_gallery_cstr();
        let enterprise = crate::globals::enterprise_cstr();
        let enterprise_normal = crate::globals::enterprise_normal_cstr();
        let enterprise_mdm = crate::globals::enterprise_mdm_cstr();
        let internaltesting = crate::globals::internaltesting_cstr();
        let os_integration = crate::globals::os_integration_cstr();
        if app_dist_type.to_bytes() != app_gallery.to_bytes()
            && app_dist_type.to_bytes() != enterprise.to_bytes()
            && app_dist_type.to_bytes() != enterprise_normal.to_bytes()
            && app_dist_type.to_bytes() != enterprise_mdm.to_bytes()
            && app_dist_type.to_bytes() != internaltesting.to_bytes()
            && app_dist_type.to_bytes() != os_integration.to_bytes()
        {
            return crate::types::V_ERR as i32;
        }
    }
    crate::types::V_OK as i32
}

fn VerifyAppBundleInfo(pf: *const crate::types::ProfileProf)-> i32 {
    if pf.is_null() {
        return crate::types::V_ERR as i32;
    }
    let type_ptr = unsafe { (*pf).type_ };
    if type_ptr.is_null() {
        return crate::types::V_ERR as i32;
    }
    let type_cstr = unsafe { std::ffi::CStr::from_ptr(type_ptr) };
    if type_cstr.to_bytes() == b"debug" {
        let dev_cert = unsafe { (*pf).bundleInfo.devCert };
        if dev_cert.is_null() || unsafe { ::libc::strlen(dev_cert as *const i8) } == 0 {
            return crate::types::V_ERR as i32;
        }
    } else if type_cstr.to_bytes() == b"release" {
        let release_cert = unsafe { (*pf).bundleInfo.releaseCert };
        if release_cert.is_null() || unsafe { ::libc::strlen(release_cert as *const i8) } == 0 {
            return crate::types::V_ERR as i32;
        }
    } else {
        return crate::types::V_ERR as i32;
    }
    crate::types::V_OK as i32
}

fn VerifyUdid(pf: *const crate::types::ProfileProf)-> i32 {
    if pf.is_null() {
        return crate::types::V_ERR as i32;
    }
    let size: u32 = (crate::types::UDID_VERIFY_BYTES + 1) as u32;
    // Move safety checks before unsafe, and narrow unsafe to FFI calls and pointer derefs.
    let devid_num = unsafe { (*pf).debugInfo.devidNum };
    if devid_num > crate::types::MAX_UDID_NUM as i32 {
        println!("[VerifyUdid:383]: udid num exceed maximum");
        return crate::types::V_ERR as i32;
    }
    let udid = unsafe { libc::malloc(size as usize) as *mut u8 };
    if udid.is_null() {
        println!("[VerifyUdid:388]: udid is null");
        return crate::types::V_ERR as i32;
    }
    unsafe { std::ptr::write_bytes(udid, 0, size as usize); }
    let result = crate::src_app_verify_hal::InquiryDeviceUdid(udid, size as i32);
    if result != 0 {
        unsafe { libc::free(udid as *mut libc::c_void); }
        println!("[VerifyUdid:395]: get udid fail, ret: {}", result);
        return crate::types::V_ERR as i32;
    }
    for i in 0..devid_num {
        let s1 = unsafe { *((*pf).debugInfo.deviceId.add(i as usize)) };
        if unsafe { libc::strcmp(s1 as *const i8, udid as *const i8) } == 0 {
            println!("[VerifyUdid:400]: find right udid");
            unsafe { libc::free(udid as *mut libc::c_void); }
            return crate::types::V_OK as i32;
        }
    }
    println!("[VerifyUdid:406]: udid invalid");
    unsafe { libc::free(udid as *mut libc::c_void); }
    return crate::types::V_ERR as i32;
}

fn VerifyDebugInfo(pf: *const crate::types::ProfileProf)-> i32 {
    // Check if the app type is "debug"
    let type_cstr = unsafe { std::ffi::CStr::from_ptr((*pf).type_ as *const i8) };
    if type_cstr.to_bytes() != b"debug" {
        return crate::types::V_OK as i32;
    }
    let dev_id_type = unsafe { std::ffi::CStr::from_ptr((*pf).debugInfo.devIdType as *const i8) };
    if dev_id_type.to_bytes() == b"udid" {
        crate::src_app_provision::VerifyUdid(pf)
    } else {
        crate::types::V_ERR as i32
    }
}

pub extern "C" fn VerifyProfileContent(pf: *const crate::types::ProfileProf) -> i32 {
    if pf.is_null() {
        return crate::types::V_ERR as i32;
    }
    let ret = crate::src_app_provision::VerifyAppTypeAndDistribution(pf);
    if ret != (crate::types::V_OK as i32) {
        return crate::types::V_ERR_INVALID_DISP_TYPE as i32;
    }
    let ret = crate::src_app_provision::VerifyAppBundleInfo(pf);
    if ret != (crate::types::V_OK as i32) {
        return crate::types::V_ERR_INVALID_APP_BUNDLE as i32;
    }
    let ret = crate::src_app_provision::VerifyDebugInfo(pf);
    if ret != (crate::types::V_OK as i32) {
        return crate::types::V_ERR_INVALID_DEVID as i32;
    }
    crate::types::V_OK as i32
}
