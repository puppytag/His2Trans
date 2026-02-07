#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn InquiryDeviceUdid(udid: *mut core::ffi::c_uchar, size: int32_t) -> int32_t;
    fn strcmp(
        _: *const core::ffi::c_char,
        _: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn malloc(_: size_t) -> *mut core::ffi::c_void;
    fn free(_: *mut core::ffi::c_void);
    fn strchr(
        _: *const core::ffi::c_char,
        _: core::ffi::c_int,
    ) -> *mut core::ffi::c_char;
    fn strlen(_: *const core::ffi::c_char) -> size_t;
    fn memset_s(
        dest: *mut core::ffi::c_void,
        destMax: size_t,
        c: core::ffi::c_int,
        count: size_t,
    ) -> errno_t;
    fn cJSON_Parse(value: *const core::ffi::c_char) -> *mut cJSON;
    fn cJSON_Delete(item: *mut cJSON);
    fn strcpy_s(
        strDest: *mut core::ffi::c_char,
        destMax: size_t,
        strSrc: *const core::ffi::c_char,
    ) -> errno_t;
    fn cJSON_GetArraySize(array: *const cJSON) -> core::ffi::c_int;
    fn cJSON_GetArrayItem(array: *const cJSON, index: core::ffi::c_int) -> *mut cJSON;
    fn cJSON_GetObjectItem(
        object: *const cJSON,
        string: *const core::ffi::c_char,
    ) -> *mut cJSON;
    fn HiLogPrint(
        type_0: LogType,
        level: LogLevel,
        domain: core::ffi::c_uint,
        tag: *const core::ffi::c_char,
        fmt: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
}
pub type int32_t = core::ffi::c_int;
pub type uint32_t = core::ffi::c_uint;
pub type C2RustUnnamed = core::ffi::c_uint;
pub const V_ERR: C2RustUnnamed = 4294967295;
pub const V_ERR_MALLOC: C2RustUnnamed = 4009754656;
pub const V_ERR_MEMCPY: C2RustUnnamed = 4009754655;
pub const V_ERR_MEMSET: C2RustUnnamed = 4009754654;
pub const V_ERR_FILE_LENGTH: C2RustUnnamed = 4009754653;
pub const V_ERR_FILE_STAT: C2RustUnnamed = 4009754652;
pub const V_ERR_FILE_OPEN: C2RustUnnamed = 4009754651;
pub const V_ERR_INVALID_DEVID: C2RustUnnamed = 4009754650;
pub const V_ERR_INVALID_DATE: C2RustUnnamed = 4009754649;
pub const V_ERR_INVALID_APP_BUNDLE: C2RustUnnamed = 4009754648;
pub const V_ERR_INVALID_DISP_TYPE: C2RustUnnamed = 4009754647;
pub const V_ERR_GET_APPID: C2RustUnnamed = 4009754646;
pub const V_ERR_GET_CERT_PK: C2RustUnnamed = 4009754645;
pub const V_ERR_VERFIY_PROF_CERT: C2RustUnnamed = 4009754644;
pub const V_ERR_PROF_CONTENT_INVALID: C2RustUnnamed = 4009754643;
pub const V_ERR_GET_PARSE_PROFILE: C2RustUnnamed = 4009754642;
pub const V_ERR_GET_PROFILE_DATA: C2RustUnnamed = 4009754641;
pub const V_ERR_GET_CERT_TYPE: C2RustUnnamed = 4009754640;
pub const V_ERR_VERIFY_SIGNATURE: C2RustUnnamed = 4009754639;
pub const V_ERR_VERIFY_CERT_CHAIN: C2RustUnnamed = 4009754638;
pub const V_ERR_PARSE_PKC7_DATA: C2RustUnnamed = 4009754637;
pub const V_ERR_CALC_BLOCK_HASH: C2RustUnnamed = 4009754636;
pub const V_ERR_GET_ROOT_HASH: C2RustUnnamed = 4009754634;
pub const V_ERR_INVALID_HASH_ALG: C2RustUnnamed = 4009754633;
pub const V_ERR_INVALID_CONTENT_TAG: C2RustUnnamed = 4009754632;
pub const V_ERR_GET_HASH_DIFF: C2RustUnnamed = 4009754631;
pub const V_ERR_GET_SIGN_BLOCK: C2RustUnnamed = 4009754630;
pub const V_ERR_GET_SIGNHEAD: C2RustUnnamed = 4009754629;
pub const V_ERR_INTEGRITY: C2RustUnnamed = 4009754628;
pub const V_ERR_UNTRUSTED_CERT: C2RustUnnamed = 4009754627;
pub const V_ERR_GET_CERT_INFO: C2RustUnnamed = 4009754626;
pub const V_OK: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProfValidity {
    pub notBefore: int32_t,
    pub notAfter: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProfBundleInfo {
    pub developerId: *mut core::ffi::c_char,
    pub devCert: *mut core::ffi::c_uchar,
    pub releaseCert: *mut core::ffi::c_uchar,
    pub bundleName: *mut core::ffi::c_char,
    pub appFeature: *mut core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProfPermission {
    pub restricNum: int32_t,
    pub restricPermission: *mut *mut core::ffi::c_char,
    pub permissionNum: int32_t,
    pub permission: *mut *mut core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProfDebugInfo {
    pub devIdType: *mut core::ffi::c_char,
    pub devidNum: int32_t,
    pub deviceId: *mut *mut core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProfileProf {
    pub versionCode: int32_t,
    pub versionName: *mut core::ffi::c_char,
    pub uuid: *mut core::ffi::c_char,
    pub type_0: *mut core::ffi::c_char,
    pub appDistType: *mut core::ffi::c_char,
    pub validity: ProfValidity,
    pub bundleInfo: ProfBundleInfo,
    pub permission: ProfPermission,
    pub debugInfo: ProfDebugInfo,
    pub issuer: *mut core::ffi::c_char,
    pub appid: *mut core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cJSON {
    pub next: *mut cJSON,
    pub prev: *mut cJSON,
    pub child: *mut cJSON,
    pub type_0: core::ffi::c_int,
    pub valuestring: *mut core::ffi::c_char,
    pub valueint: core::ffi::c_int,
    pub valuedouble: core::ffi::c_double,
    pub string: *mut core::ffi::c_char,
}
pub type LogLevel = core::ffi::c_uint;
pub const LOG_FATAL: LogLevel = 7;
pub const LOG_ERROR: LogLevel = 6;
pub const LOG_WARN: LogLevel = 5;
pub const LOG_INFO: LogLevel = 4;
pub const LOG_DEBUG: LogLevel = 3;
pub type LogType = core::ffi::c_uint;
pub const LOG_TYPE_MAX: LogType = 4;
pub const LOG_CORE: LogType = 3;
pub const LOG_INIT: LogType = 1;
pub const LOG_TYPE_MIN: LogType = 0;
pub type errno_t = core::ffi::c_int;
pub type size_t = core::ffi::c_uint;
#[no_mangle]
pub static mut APP_GALLERY: [core::ffi::c_char; 12] = unsafe {
    ::core::mem::transmute::<[u8; 12], [core::ffi::c_char; 12]>(*b"app_gallery\0")
};
#[no_mangle]
pub static mut ENTERPRISE: [core::ffi::c_char; 11] = unsafe {
    ::core::mem::transmute::<[u8; 11], [core::ffi::c_char; 11]>(*b"enterprise\0")
};
#[no_mangle]
pub static mut ENTERPRISE_NORMAL: [core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [core::ffi::c_char; 18]>(*b"enterprise_normal\0")
};
#[no_mangle]
pub static mut ENTERPRISE_MDM: [core::ffi::c_char; 15] = unsafe {
    ::core::mem::transmute::<[u8; 15], [core::ffi::c_char; 15]>(*b"enterprise_mdm\0")
};
#[no_mangle]
pub static mut OS_INTEGRATION: [core::ffi::c_char; 15] = unsafe {
    ::core::mem::transmute::<[u8; 15], [core::ffi::c_char; 15]>(*b"os_integration\0")
};
#[no_mangle]
pub static mut INTERNALTESTING: [core::ffi::c_char; 16] = unsafe {
    ::core::mem::transmute::<[u8; 16], [core::ffi::c_char; 16]>(*b"internaltesting\0")
};
unsafe extern "C" fn ProfInit(mut pf: *mut ProfileProf) {
    let mut ret: errno_t = memset_s(
        pf as *mut core::ffi::c_void,
        ::core::mem::size_of::<ProfileProf>() as size_t,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<ProfileProf>() as size_t,
    );
    if ret != 0 as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: memset failed\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<[u8; 9], [core::ffi::c_char; 9]>(*b"ProfInit\0"))
                .as_ptr(),
            35 as core::ffi::c_int,
        );
        return;
    }
}
unsafe extern "C" fn GetStringTag(
    mut root: *const cJSON,
    mut tag: *const core::ffi::c_char,
) -> *mut core::ffi::c_char {
    let mut jsonObj: *mut cJSON = cJSON_GetObjectItem(root, tag);
    if jsonObj.is_null() || ((*jsonObj).valuestring).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: failed to get %s\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"GetStringTag\0"))
                .as_ptr(),
            45 as core::ffi::c_int,
            tag,
        );
        return 0 as *mut core::ffi::c_char;
    }
    let mut objLen: int32_t = strlen((*jsonObj).valuestring) as int32_t;
    if objLen < 0 as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: len error\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"GetStringTag\0"))
                .as_ptr(),
            50 as core::ffi::c_int,
        );
        return 0 as *mut core::ffi::c_char;
    }
    let mut value: *mut core::ffi::c_char = malloc(
        (objLen as core::ffi::c_int + 1 as core::ffi::c_int) as size_t,
    ) as *mut core::ffi::c_char;
    if value.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: malloc error: %d\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"GetStringTag\0"))
                .as_ptr(),
            55 as core::ffi::c_int,
            objLen as core::ffi::c_int + 1 as core::ffi::c_int,
        );
        return 0 as *mut core::ffi::c_char;
    }
    let mut ret: errno_t = strcpy_s(
        value,
        (objLen as core::ffi::c_int + 1 as core::ffi::c_int) as size_t,
        (*jsonObj).valuestring,
    );
    if ret != 0 as core::ffi::c_int {
        if !value.is_null() {
            free(value as *mut core::ffi::c_void);
            value = 0 as *mut core::ffi::c_char;
        }
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: strcpy error: %d\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"GetStringTag\0"))
                .as_ptr(),
            61 as core::ffi::c_int,
            ret,
        );
        return 0 as *mut core::ffi::c_char;
    }
    return value;
}
unsafe extern "C" fn FreeStringAttay(
    mut array: *mut *mut core::ffi::c_char,
    mut num: int32_t,
) {
    if array.is_null() {
        return;
    }
    let mut i: int32_t = 0 as int32_t;
    while i < num {
        if !(*array.offset(i as isize)).is_null() {
            if !(*array.offset(i as isize)).is_null() {
                free(*array.offset(i as isize) as *mut core::ffi::c_void);
                let ref mut fresh0 = *array.offset(i as isize);
                *fresh0 = 0 as *mut core::ffi::c_char;
            }
        }
        i += 1;
    }
    if !array.is_null() {
        free(array as *mut core::ffi::c_void);
        array = 0 as *mut *mut core::ffi::c_char;
    }
}
unsafe extern "C" fn GetStringArrayTag(
    mut root: *const cJSON,
    mut tag: *const core::ffi::c_char,
    mut numReturn: *mut int32_t,
) -> *mut *mut core::ffi::c_char {
    let mut current_block: u64;
    let mut jsonObj: *mut cJSON = cJSON_GetObjectItem(root, tag);
    if jsonObj.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: failed to get %s\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"GetStringArrayTag\0"))
                .as_ptr(),
            85 as core::ffi::c_int,
            tag,
        );
        return 0 as *mut *mut core::ffi::c_char;
    }
    let mut num: int32_t = cJSON_GetArraySize(jsonObj) as int32_t;
    if num == 0 as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: array num 0\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"GetStringArrayTag\0"))
                .as_ptr(),
            90 as core::ffi::c_int,
        );
        *numReturn = 0 as core::ffi::c_int as int32_t;
        return 0 as *mut *mut core::ffi::c_char;
    }
    let mut value: *mut *mut core::ffi::c_char = malloc(
        (::core::mem::size_of::<*mut core::ffi::c_char>() as usize)
            .wrapping_mul(num as usize) as size_t,
    ) as *mut *mut core::ffi::c_char;
    if value.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: value is null\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"GetStringArrayTag\0"))
                .as_ptr(),
            96 as core::ffi::c_int,
        );
        *numReturn = 0 as core::ffi::c_int as int32_t;
        return 0 as *mut *mut core::ffi::c_char;
    }
    memset_s(
        value as *mut core::ffi::c_void,
        (::core::mem::size_of::<*mut core::ffi::c_char>() as usize)
            .wrapping_mul(num as usize) as size_t,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<*mut core::ffi::c_char>() as usize)
            .wrapping_mul(num as usize) as size_t,
    );
    let mut i: int32_t = 0 as int32_t;
    loop {
        if !(i < num) {
            current_block = 3437258052017859086;
            break;
        }
        let mut item: *mut cJSON = cJSON_GetArrayItem(jsonObj, i as core::ffi::c_int);
        if item.is_null() {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd001100 as core::ffi::c_uint,
                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                b"[%s:%d]: item is null\0" as *const u8 as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 18],
                    [core::ffi::c_char; 18],
                >(*b"GetStringArrayTag\0"))
                    .as_ptr(),
                104 as core::ffi::c_int,
            );
            current_block = 2740216068686416728;
            break;
        } else {
            if ((*item).valuestring).is_null() {
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    0xd001100 as core::ffi::c_uint,
                    b"appverify\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s:%d]: valuestring is NULL\0" as *const u8
                        as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 18],
                        [core::ffi::c_char; 18],
                    >(*b"GetStringArrayTag\0"))
                        .as_ptr(),
                    106 as core::ffi::c_int,
                );
                FreeStringAttay(value, num);
                return 0 as *mut *mut core::ffi::c_char;
            }
            let mut len: int32_t = strlen((*item).valuestring) as int32_t;
            let ref mut fresh1 = *value.offset(i as isize);
            *fresh1 = malloc((len as core::ffi::c_int + 1 as core::ffi::c_int) as size_t)
                as *mut core::ffi::c_char;
            if (*value.offset(i as isize)).is_null() {
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    0xd001100 as core::ffi::c_uint,
                    b"appverify\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s:%d]: value[i] is null\0" as *const u8
                        as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 18],
                        [core::ffi::c_char; 18],
                    >(*b"GetStringArrayTag\0"))
                        .as_ptr(),
                    112 as core::ffi::c_int,
                );
                current_block = 2740216068686416728;
                break;
            } else {
                let mut ret: errno_t = strcpy_s(
                    *value.offset(i as isize),
                    (len as core::ffi::c_int + 1 as core::ffi::c_int) as size_t,
                    (*item).valuestring,
                );
                if ret != 0 as core::ffi::c_int {
                    HiLogPrint(
                        LOG_CORE,
                        LOG_ERROR,
                        0xd001100 as core::ffi::c_uint,
                        b"appverify\0" as *const u8 as *const core::ffi::c_char,
                        b"[%s:%d]: str cpy error : %d\0" as *const u8
                            as *const core::ffi::c_char,
                        (::core::mem::transmute::<
                            [u8; 18],
                            [core::ffi::c_char; 18],
                        >(*b"GetStringArrayTag\0"))
                            .as_ptr(),
                        116 as core::ffi::c_int,
                        ret,
                    );
                    FreeStringAttay(value, num);
                    return 0 as *mut *mut core::ffi::c_char;
                }
                i += 1;
            }
        }
    }
    match current_block {
        2740216068686416728 => {
            FreeStringAttay(value, num);
            return 0 as *mut *mut core::ffi::c_char;
        }
        _ => {
            *numReturn = num;
            return value;
        }
    };
}
unsafe extern "C" fn GetProfValidity(
    mut root: *const cJSON,
    mut profVal: *mut ProfValidity,
) -> int32_t {
    let mut jsonObj: *mut cJSON = cJSON_GetObjectItem(
        root,
        b"validity\0" as *const u8 as *const core::ffi::c_char,
    );
    if jsonObj.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: failed to get validity\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"GetProfValidity\0"))
                .as_ptr(),
            132 as core::ffi::c_int,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    let mut notBefore: *mut cJSON = cJSON_GetObjectItem(
        jsonObj,
        b"not-before\0" as *const u8 as *const core::ffi::c_char,
    );
    if notBefore.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: failed to get not-before\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"GetProfValidity\0"))
                .as_ptr(),
            138 as core::ffi::c_int,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    (*profVal).notBefore = (*notBefore).valueint as int32_t;
    let mut notAfter: *mut cJSON = cJSON_GetObjectItem(
        jsonObj,
        b"not-after\0" as *const u8 as *const core::ffi::c_char,
    );
    if notAfter.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: failed to get not-after\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"GetProfValidity\0"))
                .as_ptr(),
            145 as core::ffi::c_int,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    (*profVal).notAfter = (*notAfter).valueint as int32_t;
    return V_OK as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn GetProfBundleInfo(
    mut root: *const cJSON,
    mut profVal: *mut ProfBundleInfo,
) -> int32_t {
    let mut jsonObj: *mut cJSON = cJSON_GetObjectItem(
        root,
        b"bundle-info\0" as *const u8 as *const core::ffi::c_char,
    );
    if jsonObj.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: failed to get bundle-info\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"GetProfBundleInfo\0"))
                .as_ptr(),
            156 as core::ffi::c_int,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    (*profVal).developerId = GetStringTag(
        jsonObj,
        b"developer-id\0" as *const u8 as *const core::ffi::c_char,
    );
    if ((*profVal).developerId).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: profVal->developerId is null\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"GetProfBundleInfo\0"))
                .as_ptr(),
            161 as core::ffi::c_int,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    (*profVal).devCert = GetStringTag(
        jsonObj,
        b"development-certificate\0" as *const u8 as *const core::ffi::c_char,
    ) as *mut core::ffi::c_uchar;
    if ((*profVal).devCert).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: get development-certificat failed\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"GetProfBundleInfo\0"))
                .as_ptr(),
            165 as core::ffi::c_int,
        );
        (*profVal).devCert = malloc(
            ::core::mem::size_of::<core::ffi::c_char>() as size_t,
        ) as *mut core::ffi::c_uchar;
        if ((*profVal).devCert).is_null() {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd001100 as core::ffi::c_uint,
                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                b"[%s:%d]: profVal->devCert is null\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 18],
                    [core::ffi::c_char; 18],
                >(*b"GetProfBundleInfo\0"))
                    .as_ptr(),
                167 as core::ffi::c_int,
            );
            return V_ERR as core::ffi::c_uint as int32_t;
        }
        *((*profVal).devCert).offset(0 as core::ffi::c_int as isize) = '\0' as i32
            as core::ffi::c_uchar;
    }
    (*profVal).releaseCert = GetStringTag(
        jsonObj,
        b"distribution-certificate\0" as *const u8 as *const core::ffi::c_char,
    ) as *mut core::ffi::c_uchar;
    if ((*profVal).releaseCert).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: get distribution-certificat failed\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"GetProfBundleInfo\0"))
                .as_ptr(),
            173 as core::ffi::c_int,
        );
        (*profVal).releaseCert = malloc(
            ::core::mem::size_of::<core::ffi::c_char>() as size_t,
        ) as *mut core::ffi::c_uchar;
        if ((*profVal).releaseCert).is_null() {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd001100 as core::ffi::c_uint,
                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                b"[%s:%d]: profVal->releaseCert is null\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 18],
                    [core::ffi::c_char; 18],
                >(*b"GetProfBundleInfo\0"))
                    .as_ptr(),
                175 as core::ffi::c_int,
            );
            return V_ERR as core::ffi::c_uint as int32_t;
        }
        *((*profVal).releaseCert).offset(0 as core::ffi::c_int as isize) = '\0' as i32
            as core::ffi::c_uchar;
    }
    (*profVal).bundleName = GetStringTag(
        jsonObj,
        b"bundle-name\0" as *const u8 as *const core::ffi::c_char,
    );
    if ((*profVal).bundleName).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: profVal->bundleName is null\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"GetProfBundleInfo\0"))
                .as_ptr(),
            180 as core::ffi::c_int,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    (*profVal).appFeature = GetStringTag(
        jsonObj,
        b"app-feature\0" as *const u8 as *const core::ffi::c_char,
    );
    if ((*profVal).appFeature).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: profVal->appFeature is null\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"GetProfBundleInfo\0"))
                .as_ptr(),
            183 as core::ffi::c_int,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    return V_OK as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn GetProfPermission(
    mut root: *const cJSON,
    mut profVal: *mut ProfPermission,
) -> int32_t {
    let mut jsonObj: *mut cJSON = cJSON_GetObjectItem(
        root,
        b"permissions\0" as *const u8 as *const core::ffi::c_char,
    );
    if jsonObj.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: failed to get permissions\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"GetProfPermission\0"))
                .as_ptr(),
            192 as core::ffi::c_int,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    (*profVal).permission = GetStringArrayTag(
        jsonObj,
        b"feature-permissions\0" as *const u8 as *const core::ffi::c_char,
        &mut (*profVal).permissionNum,
    );
    (*profVal).restricPermission = GetStringArrayTag(
        jsonObj,
        b"restricted-permissions\0" as *const u8 as *const core::ffi::c_char,
        &mut (*profVal).restricNum,
    );
    return V_OK as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn GetProfDebugInfo(
    mut root: *const cJSON,
    mut profVal: *mut ProfDebugInfo,
) -> int32_t {
    let mut jsonObj: *mut cJSON = cJSON_GetObjectItem(
        root,
        b"debug-info\0" as *const u8 as *const core::ffi::c_char,
    );
    if jsonObj.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: failed to get debug-info\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"GetProfDebugInfo\0"))
                .as_ptr(),
            204 as core::ffi::c_int,
        );
        return V_OK as core::ffi::c_int as int32_t;
    }
    (*profVal).devIdType = GetStringTag(
        jsonObj,
        b"device-id-type\0" as *const u8 as *const core::ffi::c_char,
    );
    if ((*profVal).devIdType).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: failed to get device-id-type\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"GetProfDebugInfo\0"))
                .as_ptr(),
            209 as core::ffi::c_int,
        );
        return V_OK as core::ffi::c_int as int32_t;
    }
    (*profVal).deviceId = GetStringArrayTag(
        jsonObj,
        b"device-ids\0" as *const u8 as *const core::ffi::c_char,
        &mut (*profVal).devidNum,
    );
    return V_OK as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn GetProfIssuerInfo(
    mut root: *const cJSON,
    mut pf: *mut ProfileProf,
) -> int32_t {
    (*pf).issuer = GetStringTag(
        root,
        b"issuer\0" as *const u8 as *const core::ffi::c_char,
    );
    if ((*pf).issuer).is_null() {
        let mut len: int32_t = strlen(
            b"Huawei App Store\0" as *const u8 as *const core::ffi::c_char,
        ) as int32_t;
        (*pf).issuer = malloc(
            (len as core::ffi::c_int + 1 as core::ffi::c_int) as size_t,
        ) as *mut core::ffi::c_char;
        if ((*pf).issuer).is_null() {
            return V_ERR as core::ffi::c_uint as int32_t;
        }
        let mut ret: errno_t = strcpy_s(
            (*pf).issuer,
            (len as core::ffi::c_int + 1 as core::ffi::c_int) as size_t,
            b"Huawei App Store\0" as *const u8 as *const core::ffi::c_char,
        );
        if ret != 0 as core::ffi::c_int {
            if !((*pf).issuer).is_null() {
                free((*pf).issuer as *mut core::ffi::c_void);
                (*pf).issuer = 0 as *mut core::ffi::c_char;
            }
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd001100 as core::ffi::c_uint,
                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                b"[%s:%d]: str cpy error: %d\0" as *const u8 as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 18],
                    [core::ffi::c_char; 18],
                >(*b"GetProfIssuerInfo\0"))
                    .as_ptr(),
                228 as core::ffi::c_int,
                ret,
            );
        }
        return ret as int32_t;
    }
    return V_OK as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn FreeProfBundle(mut pfval: *mut ProfBundleInfo) {
    if !((*pfval).appFeature).is_null() {
        free((*pfval).appFeature as *mut core::ffi::c_void);
        (*pfval).appFeature = 0 as *mut core::ffi::c_char;
    }
    if !((*pfval).bundleName).is_null() {
        free((*pfval).bundleName as *mut core::ffi::c_void);
        (*pfval).bundleName = 0 as *mut core::ffi::c_char;
    }
    if !((*pfval).devCert).is_null() {
        free((*pfval).devCert as *mut core::ffi::c_void);
        (*pfval).devCert = 0 as *mut core::ffi::c_uchar;
    }
    if !((*pfval).developerId).is_null() {
        free((*pfval).developerId as *mut core::ffi::c_void);
        (*pfval).developerId = 0 as *mut core::ffi::c_char;
    }
    if !((*pfval).releaseCert).is_null() {
        free((*pfval).releaseCert as *mut core::ffi::c_void);
        (*pfval).releaseCert = 0 as *mut core::ffi::c_uchar;
    }
}
unsafe extern "C" fn FreeProfPerssion(mut pfval: *mut ProfPermission) {
    FreeStringAttay((*pfval).permission, (*pfval).permissionNum);
    (*pfval).permissionNum = 0 as core::ffi::c_int as int32_t;
    (*pfval).permission = 0 as *mut *mut core::ffi::c_char;
    FreeStringAttay((*pfval).restricPermission, (*pfval).restricNum);
    (*pfval).restricNum = 0 as core::ffi::c_int as int32_t;
    (*pfval).restricPermission = 0 as *mut *mut core::ffi::c_char;
}
unsafe extern "C" fn FreeProfDebuginfo(mut pfval: *mut ProfDebugInfo) {
    if !((*pfval).devIdType).is_null() {
        free((*pfval).devIdType as *mut core::ffi::c_void);
        (*pfval).devIdType = 0 as *mut core::ffi::c_char;
    }
    FreeStringAttay((*pfval).deviceId, (*pfval).devidNum);
    (*pfval).devidNum = 0 as core::ffi::c_int as int32_t;
    (*pfval).deviceId = 0 as *mut *mut core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn ProfFreeData(mut pf: *mut ProfileProf) {
    if pf.is_null() {
        return;
    }
    if !((*pf).versionName).is_null() {
        free((*pf).versionName as *mut core::ffi::c_void);
        (*pf).versionName = 0 as *mut core::ffi::c_char;
    }
    if !((*pf).uuid).is_null() {
        free((*pf).uuid as *mut core::ffi::c_void);
        (*pf).uuid = 0 as *mut core::ffi::c_char;
    }
    if !((*pf).type_0).is_null() {
        free((*pf).type_0 as *mut core::ffi::c_void);
        (*pf).type_0 = 0 as *mut core::ffi::c_char;
    }
    if !((*pf).appDistType).is_null() {
        free((*pf).appDistType as *mut core::ffi::c_void);
        (*pf).appDistType = 0 as *mut core::ffi::c_char;
    }
    FreeProfBundle(&mut (*pf).bundleInfo);
    FreeProfPerssion(&mut (*pf).permission);
    FreeProfDebuginfo(&mut (*pf).debugInfo);
    if !((*pf).issuer).is_null() {
        free((*pf).issuer as *mut core::ffi::c_void);
        (*pf).issuer = 0 as *mut core::ffi::c_char;
    }
    if !((*pf).appid).is_null() {
        free((*pf).appid as *mut core::ffi::c_void);
        (*pf).appid = 0 as *mut core::ffi::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ParseProfile(
    mut buf: *const core::ffi::c_char,
    mut len: int32_t,
    mut pf: *mut ProfileProf,
) -> int32_t {
    let mut current_block: u64;
    if pf.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: pf is null\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"ParseProfile\0"))
                .as_ptr(),
            288 as core::ffi::c_int,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    if buf.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: buf is null\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"ParseProfile\0"))
                .as_ptr(),
            289 as core::ffi::c_int,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    ProfInit(pf);
    let mut ret: int32_t = 0;
    let mut pfStr: *mut core::ffi::c_char = strchr(buf, '{' as i32);
    if pfStr.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: pfStr is null\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"ParseProfile\0"))
                .as_ptr(),
            293 as core::ffi::c_int,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    let mut root: *mut cJSON = cJSON_Parse(pfStr);
    if root.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: root is null\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"ParseProfile\0"))
                .as_ptr(),
            296 as core::ffi::c_int,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    let mut jsonObj: *mut cJSON = cJSON_GetObjectItem(
        root,
        b"version-code\0" as *const u8 as *const core::ffi::c_char,
    );
    if jsonObj.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: jsonObj is null\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"ParseProfile\0"))
                .as_ptr(),
            299 as core::ffi::c_int,
        );
    } else {
        (*pf).versionCode = (*jsonObj).valueint as int32_t;
        (*pf).versionName = GetStringTag(
            root,
            b"version-name\0" as *const u8 as *const core::ffi::c_char,
        );
        if ((*pf).versionName).is_null() {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd001100 as core::ffi::c_uint,
                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                b"[%s:%d]: pf->versionName is null\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 13],
                    [core::ffi::c_char; 13],
                >(*b"ParseProfile\0"))
                    .as_ptr(),
                303 as core::ffi::c_int,
            );
        } else {
            (*pf).uuid = GetStringTag(
                root,
                b"uuid\0" as *const u8 as *const core::ffi::c_char,
            );
            if ((*pf).uuid).is_null() {
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    0xd001100 as core::ffi::c_uint,
                    b"appverify\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s:%d]: pf->uuid is null\0" as *const u8
                        as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 13],
                        [core::ffi::c_char; 13],
                    >(*b"ParseProfile\0"))
                        .as_ptr(),
                    306 as core::ffi::c_int,
                );
            } else {
                (*pf).type_0 = GetStringTag(
                    root,
                    b"type\0" as *const u8 as *const core::ffi::c_char,
                );
                if ((*pf).type_0).is_null() {
                    HiLogPrint(
                        LOG_CORE,
                        LOG_ERROR,
                        0xd001100 as core::ffi::c_uint,
                        b"appverify\0" as *const u8 as *const core::ffi::c_char,
                        b"[%s:%d]: pf->type is null\0" as *const u8
                            as *const core::ffi::c_char,
                        (::core::mem::transmute::<
                            [u8; 13],
                            [core::ffi::c_char; 13],
                        >(*b"ParseProfile\0"))
                            .as_ptr(),
                        309 as core::ffi::c_int,
                    );
                } else {
                    (*pf).appDistType = GetStringTag(
                        root,
                        b"app-distribution-type\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                    if ((*pf).appDistType).is_null() {
                        (*pf).appDistType = malloc(
                            ::core::mem::size_of::<core::ffi::c_char>() as size_t,
                        ) as *mut core::ffi::c_char;
                        if ((*pf).appDistType).is_null() {
                            HiLogPrint(
                                LOG_CORE,
                                LOG_ERROR,
                                0xd001100 as core::ffi::c_uint,
                                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                                b"[%s:%d]: pf->appDistType is null\0" as *const u8
                                    as *const core::ffi::c_char,
                                (::core::mem::transmute::<
                                    [u8; 13],
                                    [core::ffi::c_char; 13],
                                >(*b"ParseProfile\0"))
                                    .as_ptr(),
                                314 as core::ffi::c_int,
                            );
                            current_block = 10416150603132289894;
                        } else {
                            *((*pf).appDistType)
                                .offset(0 as core::ffi::c_int as isize) = '\0' as i32
                                as core::ffi::c_char;
                            current_block = 1847472278776910194;
                        }
                    } else {
                        current_block = 1847472278776910194;
                    }
                    match current_block {
                        10416150603132289894 => {}
                        _ => {
                            ret = GetProfValidity(root, &mut (*pf).validity);
                            if ret != V_OK as core::ffi::c_int {
                                HiLogPrint(
                                    LOG_CORE,
                                    LOG_ERROR,
                                    0xd001100 as core::ffi::c_uint,
                                    b"appverify\0" as *const u8 as *const core::ffi::c_char,
                                    b"[%s:%d]: ret not ok\0" as *const u8
                                        as *const core::ffi::c_char,
                                    (::core::mem::transmute::<
                                        [u8; 13],
                                        [core::ffi::c_char; 13],
                                    >(*b"ParseProfile\0"))
                                        .as_ptr(),
                                    319 as core::ffi::c_int,
                                );
                            } else {
                                ret = GetProfBundleInfo(root, &mut (*pf).bundleInfo);
                                if ret != V_OK as core::ffi::c_int {
                                    HiLogPrint(
                                        LOG_CORE,
                                        LOG_ERROR,
                                        0xd001100 as core::ffi::c_uint,
                                        b"appverify\0" as *const u8 as *const core::ffi::c_char,
                                        b"[%s:%d]: ret not ok\0" as *const u8
                                            as *const core::ffi::c_char,
                                        (::core::mem::transmute::<
                                            [u8; 13],
                                            [core::ffi::c_char; 13],
                                        >(*b"ParseProfile\0"))
                                            .as_ptr(),
                                        322 as core::ffi::c_int,
                                    );
                                } else {
                                    ret = GetProfPermission(root, &mut (*pf).permission);
                                    if ret != V_OK as core::ffi::c_int {
                                        HiLogPrint(
                                            LOG_CORE,
                                            LOG_ERROR,
                                            0xd001100 as core::ffi::c_uint,
                                            b"appverify\0" as *const u8 as *const core::ffi::c_char,
                                            b"[%s:%d]: ret not ok\0" as *const u8
                                                as *const core::ffi::c_char,
                                            (::core::mem::transmute::<
                                                [u8; 13],
                                                [core::ffi::c_char; 13],
                                            >(*b"ParseProfile\0"))
                                                .as_ptr(),
                                            325 as core::ffi::c_int,
                                        );
                                    } else {
                                        ret = GetProfDebugInfo(root, &mut (*pf).debugInfo);
                                        if ret != V_OK as core::ffi::c_int {
                                            HiLogPrint(
                                                LOG_CORE,
                                                LOG_ERROR,
                                                0xd001100 as core::ffi::c_uint,
                                                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                                                b"[%s:%d]: ret not ok\0" as *const u8
                                                    as *const core::ffi::c_char,
                                                (::core::mem::transmute::<
                                                    [u8; 13],
                                                    [core::ffi::c_char; 13],
                                                >(*b"ParseProfile\0"))
                                                    .as_ptr(),
                                                328 as core::ffi::c_int,
                                            );
                                        } else {
                                            ret = GetProfIssuerInfo(root, pf);
                                            if ret != V_OK as core::ffi::c_int {
                                                HiLogPrint(
                                                    LOG_CORE,
                                                    LOG_ERROR,
                                                    0xd001100 as core::ffi::c_uint,
                                                    b"appverify\0" as *const u8 as *const core::ffi::c_char,
                                                    b"[%s:%d]: ret not ok\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    (::core::mem::transmute::<
                                                        [u8; 13],
                                                        [core::ffi::c_char; 13],
                                                    >(*b"ParseProfile\0"))
                                                        .as_ptr(),
                                                    331 as core::ffi::c_int,
                                                );
                                            } else {
                                                HiLogPrint(
                                                    LOG_CORE,
                                                    LOG_INFO,
                                                    0xd001100 as core::ffi::c_uint,
                                                    b"appverify\0" as *const u8 as *const core::ffi::c_char,
                                                    b"[%s:%d]: parse profile json success\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    (::core::mem::transmute::<
                                                        [u8; 13],
                                                        [core::ffi::c_char; 13],
                                                    >(*b"ParseProfile\0"))
                                                        .as_ptr(),
                                                    333 as core::ffi::c_int,
                                                );
                                                cJSON_Delete(root);
                                                return V_OK as core::ffi::c_int as int32_t;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    cJSON_Delete(root);
    ProfFreeData(pf);
    return V_ERR as core::ffi::c_uint as int32_t;
}
unsafe extern "C" fn VerifyAppTypeAndDistribution(
    mut pf: *const ProfileProf,
) -> int32_t {
    if strcmp((*pf).type_0, b"debug\0" as *const u8 as *const core::ffi::c_char)
        != 0 as core::ffi::c_int
        && strcmp((*pf).type_0, b"release\0" as *const u8 as *const core::ffi::c_char)
            != 0 as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: invalid app type: %s\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"VerifyAppTypeAndDistribution\0"))
                .as_ptr(),
            346 as core::ffi::c_int,
            (*pf).type_0,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    if strcmp((*pf).type_0, b"release\0" as *const u8 as *const core::ffi::c_char)
        == 0 as core::ffi::c_int
    {
        if strcmp((*pf).appDistType, APP_GALLERY.as_ptr()) != 0 as core::ffi::c_int
            && strcmp((*pf).appDistType, ENTERPRISE.as_ptr()) != 0 as core::ffi::c_int
            && strcmp((*pf).appDistType, ENTERPRISE_NORMAL.as_ptr())
                != 0 as core::ffi::c_int
            && strcmp((*pf).appDistType, ENTERPRISE_MDM.as_ptr())
                != 0 as core::ffi::c_int
            && strcmp((*pf).appDistType, INTERNALTESTING.as_ptr())
                != 0 as core::ffi::c_int
            && strcmp((*pf).appDistType, OS_INTEGRATION.as_ptr())
                != 0 as core::ffi::c_int
        {
            HiLogPrint(
                LOG_CORE,
                LOG_INFO,
                0xd001100 as core::ffi::c_uint,
                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                b"[%s:%d]: invalid app dis type: %s\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 29],
                    [core::ffi::c_char; 29],
                >(*b"VerifyAppTypeAndDistribution\0"))
                    .as_ptr(),
                353 as core::ffi::c_int,
                (*pf).appDistType,
            );
            return V_ERR as core::ffi::c_uint as int32_t;
        }
    }
    return V_OK as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn VerifyAppBundleInfo(mut pf: *const ProfileProf) -> int32_t {
    if strcmp((*pf).type_0, b"debug\0" as *const u8 as *const core::ffi::c_char)
        == 0 as core::ffi::c_int
    {
        if strlen((*pf).bundleInfo.devCert as *mut core::ffi::c_char)
            == 0 as core::ffi::c_uint
        {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd001100 as core::ffi::c_uint,
                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                b"[%s:%d]: debug app, dev cert null\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 20],
                    [core::ffi::c_char; 20],
                >(*b"VerifyAppBundleInfo\0"))
                    .as_ptr(),
                364 as core::ffi::c_int,
            );
            return V_ERR as core::ffi::c_uint as int32_t;
        }
    } else if strcmp((*pf).type_0, b"release\0" as *const u8 as *const core::ffi::c_char)
        == 0 as core::ffi::c_int
    {
        if strlen((*pf).bundleInfo.releaseCert as *mut core::ffi::c_char)
            == 0 as core::ffi::c_uint
        {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd001100 as core::ffi::c_uint,
                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                b"[%s:%d]: debug app, dev cert null\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 20],
                    [core::ffi::c_char; 20],
                >(*b"VerifyAppBundleInfo\0"))
                    .as_ptr(),
                369 as core::ffi::c_int,
            );
            return V_ERR as core::ffi::c_uint as int32_t;
        }
    } else {
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: invalid app type: %s\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"VerifyAppBundleInfo\0"))
                .as_ptr(),
            373 as core::ffi::c_int,
            (*pf).type_0,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    return V_OK as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn VerifyUdid(mut pf: *const ProfileProf) -> int32_t {
    let mut size: uint32_t = (64 as core::ffi::c_int + 1 as core::ffi::c_int)
        as uint32_t;
    if (*pf).debugInfo.devidNum > 100 as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: udid num exceed maximum\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 11],
                [core::ffi::c_char; 11],
            >(*b"VerifyUdid\0"))
                .as_ptr(),
            383 as core::ffi::c_int,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    let mut udid: *mut core::ffi::c_uchar = malloc(size as size_t)
        as *mut core::ffi::c_uchar;
    if udid.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: udid is null\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 11],
                [core::ffi::c_char; 11],
            >(*b"VerifyUdid\0"))
                .as_ptr(),
            388 as core::ffi::c_int,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    memset_s(
        udid as *mut core::ffi::c_void,
        size as size_t,
        0 as core::ffi::c_int,
        size as size_t,
    );
    let mut result: int32_t = InquiryDeviceUdid(udid, size as int32_t);
    if result != 0 as core::ffi::c_int {
        free(udid as *mut core::ffi::c_void);
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: get udid fail, ret: %d\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 11],
                [core::ffi::c_char; 11],
            >(*b"VerifyUdid\0"))
                .as_ptr(),
            395 as core::ffi::c_int,
            result,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    let mut i: int32_t = 0 as int32_t;
    while i < (*pf).debugInfo.devidNum {
        if strcmp(
            *((*pf).debugInfo.deviceId).offset(i as isize) as *const core::ffi::c_char,
            udid as *const core::ffi::c_char,
        ) == 0 as core::ffi::c_int
        {
            HiLogPrint(
                LOG_CORE,
                LOG_INFO,
                0xd001100 as core::ffi::c_uint,
                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                b"[%s:%d]: find right udid\0" as *const u8 as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 11],
                    [core::ffi::c_char; 11],
                >(*b"VerifyUdid\0"))
                    .as_ptr(),
                400 as core::ffi::c_int,
            );
            free(udid as *mut core::ffi::c_void);
            udid = 0 as *mut core::ffi::c_uchar;
            return V_OK as core::ffi::c_int as int32_t;
        }
        i += 1;
    }
    HiLogPrint(
        LOG_CORE,
        LOG_ERROR,
        0xd001100 as core::ffi::c_uint,
        b"appverify\0" as *const u8 as *const core::ffi::c_char,
        b"[%s:%d]: udid invalid\0" as *const u8 as *const core::ffi::c_char,
        (::core::mem::transmute::<[u8; 11], [core::ffi::c_char; 11]>(*b"VerifyUdid\0"))
            .as_ptr(),
        406 as core::ffi::c_int,
    );
    free(udid as *mut core::ffi::c_void);
    udid = 0 as *mut core::ffi::c_uchar;
    return V_ERR as core::ffi::c_uint as int32_t;
}
unsafe extern "C" fn VerifyDebugInfo(mut pf: *const ProfileProf) -> int32_t {
    if strcmp((*pf).type_0, b"debug\0" as *const u8 as *const core::ffi::c_char)
        != 0 as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: not debug app, return ok\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"VerifyDebugInfo\0"))
                .as_ptr(),
            415 as core::ffi::c_int,
        );
        return V_OK as core::ffi::c_int as int32_t;
    }
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        0xd001100 as core::ffi::c_uint,
        b"appverify\0" as *const u8 as *const core::ffi::c_char,
        b"[%s:%d]: devid type: %s\0" as *const u8 as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 16],
            [core::ffi::c_char; 16],
        >(*b"VerifyDebugInfo\0"))
            .as_ptr(),
        418 as core::ffi::c_int,
        (*pf).debugInfo.devIdType,
    );
    let mut ret: int32_t = 0;
    if strcmp(
        (*pf).debugInfo.devIdType,
        b"udid\0" as *const u8 as *const core::ffi::c_char,
    ) == 0 as core::ffi::c_int
    {
        ret = VerifyUdid(pf);
    } else {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: devid type invalid\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"VerifyDebugInfo\0"))
                .as_ptr(),
            423 as core::ffi::c_int,
        );
        ret = V_ERR as core::ffi::c_uint as int32_t;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn VerifyProfileContent(mut pf: *const ProfileProf) -> int32_t {
    if pf.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: pf is null\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"VerifyProfileContent\0"))
                .as_ptr(),
            431 as core::ffi::c_int,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    let mut ret: int32_t = VerifyAppTypeAndDistribution(pf);
    if ret != V_OK as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: invalid profile distribution type : %s\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"VerifyProfileContent\0"))
                .as_ptr(),
            434 as core::ffi::c_int,
            (*pf).appDistType,
        );
        return V_ERR_INVALID_DISP_TYPE as core::ffi::c_uint as int32_t;
    }
    ret = VerifyAppBundleInfo(pf);
    if ret != V_OK as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: invalid profile app bundle info\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"VerifyProfileContent\0"))
                .as_ptr(),
            439 as core::ffi::c_int,
        );
        return V_ERR_INVALID_APP_BUNDLE as core::ffi::c_uint as int32_t;
    }
    ret = VerifyDebugInfo(pf);
    if ret != V_OK as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: validate debug info\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"VerifyProfileContent\0"))
                .as_ptr(),
            445 as core::ffi::c_int,
        );
        return V_ERR_INVALID_DEVID as core::ffi::c_uint as int32_t;
    }
    return V_OK as core::ffi::c_int as int32_t;
}
