#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type mbedtls_md_info_t;
    fn malloc(_: size_t) -> *mut core::ffi::c_void;
    fn free(_: *mut core::ffi::c_void);
    fn lseek(_: core::ffi::c_int, _: off_t, _: core::ffi::c_int) -> off_t;
    fn read(_: core::ffi::c_int, _: *mut core::ffi::c_void, _: size_t) -> ssize_t;
    fn CreateHapBuffer(hapBuffer: *mut HapBuf, len: int32_t) -> bool;
    fn HapSetInt32(buffer: *const HapBuf, offset: int32_t, value: int32_t);
    fn ClearHapBuffer(hapBuffer: *mut HapBuf);
    fn HapPutByte(hapBuffer: *const HapBuf, offset: int32_t, value: core::ffi::c_char);
    fn HapPutData(
        hapBuffer: *const HapBuf,
        offset: int32_t,
        data: *const core::ffi::c_uchar,
        len: int32_t,
    );
    fn HapPutInt32(buf: *mut core::ffi::c_uchar, len: int32_t, value: int32_t);
    fn GetHashUnitLen(hashAlg: int32_t) -> int32_t;
    fn GetSignBlockByType(
        signInfo: *const SignatureInfo,
        fp: int32_t,
        blockType: int32_t,
        len: *mut int32_t,
        blockHead: *mut BlockHead,
    ) -> *mut core::ffi::c_char;
    fn memset_s(
        dest: *mut core::ffi::c_void,
        destMax: size_t,
        c: core::ffi::c_int,
        count: size_t,
    ) -> errno_t;
    fn mbedtls_md_info_from_type(md_type: mbedtls_md_type_t) -> *const mbedtls_md_info_t;
    fn mbedtls_md_init(ctx: *mut mbedtls_md_context_t);
    fn memcpy_s(
        dest: *mut core::ffi::c_void,
        destMax: size_t,
        src: *const core::ffi::c_void,
        count: size_t,
    ) -> errno_t;
    fn mbedtls_md_free(ctx: *mut mbedtls_md_context_t);
    fn HiLogPrint(
        type_0: LogType,
        level: LogLevel,
        domain: core::ffi::c_uint,
        tag: *const core::ffi::c_char,
        fmt: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn mbedtls_md_setup(
        ctx: *mut mbedtls_md_context_t,
        md_info: *const mbedtls_md_info_t,
        hmac: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn mbedtls_md_get_size(md_info: *const mbedtls_md_info_t) -> core::ffi::c_uchar;
    fn mbedtls_md_starts(ctx: *mut mbedtls_md_context_t) -> core::ffi::c_int;
    fn mbedtls_md_update(
        ctx: *mut mbedtls_md_context_t,
        input: *const core::ffi::c_uchar,
        ilen: size_t,
    ) -> core::ffi::c_int;
    fn mbedtls_md_finish(
        ctx: *mut mbedtls_md_context_t,
        output: *mut core::ffi::c_uchar,
    ) -> core::ffi::c_int;
}
pub type int32_t = core::ffi::c_int;
pub type uint32_t = core::ffi::c_uint;
pub type size_t = core::ffi::c_uint;
pub type ssize_t = core::ffi::c_int;
pub type off_t = core::ffi::c_longlong;
pub type mbedtls_md_type_t = core::ffi::c_uint;
pub const MBEDTLS_MD_SHA3_512: mbedtls_md_type_t = 19;
pub const MBEDTLS_MD_SHA3_384: mbedtls_md_type_t = 18;
pub const MBEDTLS_MD_SHA3_256: mbedtls_md_type_t = 17;
pub const MBEDTLS_MD_SHA3_224: mbedtls_md_type_t = 16;
pub const MBEDTLS_MD_SHA512: mbedtls_md_type_t = 11;
pub const MBEDTLS_MD_SHA384: mbedtls_md_type_t = 10;
pub const MBEDTLS_MD_SHA256: mbedtls_md_type_t = 9;
pub const MBEDTLS_MD_SHA224: mbedtls_md_type_t = 8;
pub const MBEDTLS_MD_SHA1: mbedtls_md_type_t = 5;
pub const MBEDTLS_MD_RIPEMD160: mbedtls_md_type_t = 4;
pub const MBEDTLS_MD_MD5: mbedtls_md_type_t = 1;
pub const MBEDTLS_MD_NONE: mbedtls_md_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_md_context_t {
    pub private_md_info: *const mbedtls_md_info_t,
    pub private_md_ctx: *mut core::ffi::c_void,
    pub private_hmac_ctx: *mut core::ffi::c_void,
}
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
pub type LogType = core::ffi::c_uint;
pub const LOG_TYPE_MAX: LogType = 4;
pub const LOG_CORE: LogType = 3;
pub const LOG_INIT: LogType = 1;
pub const LOG_TYPE_MIN: LogType = 0;
pub type LogLevel = core::ffi::c_uint;
pub const LOG_FATAL: LogLevel = 7;
pub const LOG_ERROR: LogLevel = 6;
pub const LOG_WARN: LogLevel = 5;
pub const LOG_INFO: LogLevel = 4;
pub const LOG_DEBUG: LogLevel = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HapBuf {
    pub buffer: *mut core::ffi::c_void,
    pub len: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed(4))]
pub struct HwSignHead {
    pub blockNum: uint32_t,
    pub size: core::ffi::c_ulonglong,
    pub magicLow: core::ffi::c_ulonglong,
    pub magicHigh: core::ffi::c_ulonglong,
    pub version: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed(2))]
pub struct MinEocd {
    pub magic: int32_t,
    pub diskNum: core::ffi::c_short,
    pub startNum: core::ffi::c_short,
    pub coreDirNumOnDisk: core::ffi::c_short,
    pub coreDirNum: core::ffi::c_short,
    pub coreDirSize: int32_t,
    pub coreDirOffset: int32_t,
    pub commentLen: core::ffi::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HapEocd {
    pub eocdHead: MinEocd,
    pub comment: *mut core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SignatureInfo {
    pub signHead: *mut HwSignHead,
    pub fullSignBlockOffset: int32_t,
    pub hapCoreDirOffset: int32_t,
    pub hapEocdOffset: int32_t,
    pub hapEocdSize: int32_t,
    pub fileSize: int32_t,
    pub version: int32_t,
    pub certType: int32_t,
}
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const PROPERTY_BLOCK_TYPE: C2RustUnnamed_0 = 536870915;
pub const PROFILE_BLOCK_WITHSIGN_TYPE: C2RustUnnamed_0 = 536870914;
pub const KEY_ROTATION_BLOCK_TYPE: C2RustUnnamed_0 = 536870913;
pub const SIGNATURE_BLOCK_TYPE: C2RustUnnamed_0 = 536870912;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockHead {
    pub type_0: uint32_t,
    pub length: uint32_t,
    pub offset: uint32_t,
}
pub type errno_t = core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn GetDigestAlgorithmId(mut signAlgorithm: uint32_t) -> int32_t {
    match signAlgorithm as core::ffi::c_uint & 0xf as core::ffi::c_uint {
        1 | 4 => return MBEDTLS_MD_SHA256 as core::ffi::c_int as int32_t,
        2 | 5 => return MBEDTLS_MD_SHA384 as core::ffi::c_int as int32_t,
        3 | 6 => return MBEDTLS_MD_SHA512 as core::ffi::c_int as int32_t,
        _ => {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd001100 as core::ffi::c_uint,
                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                b"[%s:%d]: signAlgorithm: %u error\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 21],
                    [core::ffi::c_char; 21],
                >(*b"GetDigestAlgorithmId\0"))
                    .as_ptr(),
                38 as core::ffi::c_int,
                signAlgorithm,
            );
            return V_ERR as core::ffi::c_uint as int32_t;
        }
    };
}
unsafe extern "C" fn ComputeBlockHash(
    mut block: *const core::ffi::c_char,
    mut blockLen: int32_t,
    mut alg: int32_t,
    mut result: *const HapBuf,
    mut offset: *mut int32_t,
) -> int32_t {
    let mut current_block: u64;
    let mut mdInfo: *const mbedtls_md_info_t = mbedtls_md_info_from_type(
        alg as mbedtls_md_type_t,
    );
    if mdInfo.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: mdInfo is null\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"ComputeBlockHash\0"))
                .as_ptr(),
            46 as core::ffi::c_int,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    let mut pos: int32_t = 0 as int32_t;
    let mut rawBufLen: int32_t = blockLen;
    let mut mdCtx: *mut mbedtls_md_context_t = malloc(
        ::core::mem::size_of::<mbedtls_md_context_t>() as size_t,
    ) as *mut mbedtls_md_context_t;
    if mdCtx.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: mdCtx is null\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"ComputeBlockHash\0"))
                .as_ptr(),
            50 as core::ffi::c_int,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        0xd001100 as core::ffi::c_uint,
        b"appverify\0" as *const u8 as *const core::ffi::c_char,
        b"[%s:%d]: alg: %d wholelen: %d\0" as *const u8 as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 17],
            [core::ffi::c_char; 17],
        >(*b"ComputeBlockHash\0"))
            .as_ptr(),
        51 as core::ffi::c_int,
        alg,
        rawBufLen,
    );
    loop {
        if !(rawBufLen > 0 as core::ffi::c_int) {
            current_block = 12556861819962772176;
            break;
        }
        mbedtls_md_init(mdCtx);
        let mut readLen: int32_t = if rawBufLen
            > 1024 as core::ffi::c_int * 1024 as core::ffi::c_int
        {
            1024 as int32_t * 1024 as int32_t
        } else {
            rawBufLen
        };
        let mut ret: int32_t = mbedtls_md_setup(mdCtx, mdInfo, 0 as core::ffi::c_int)
            as int32_t;
        if ret != V_OK as core::ffi::c_int {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd001100 as core::ffi::c_uint,
                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                b"[%s:%d]: ret not ok\0" as *const u8 as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"ComputeBlockHash\0"))
                    .as_ptr(),
                56 as core::ffi::c_int,
            );
            current_block = 13109744992413637920;
            break;
        } else {
            let mut hlen: size_t = mbedtls_md_get_size(mdInfo) as size_t;
            if hlen == 0 as core::ffi::c_uint || hlen > 64 as core::ffi::c_uint {
                current_block = 13109744992413637920;
                break;
            }
            ret = mbedtls_md_starts(mdCtx) as int32_t;
            if ret != V_OK as core::ffi::c_int {
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    0xd001100 as core::ffi::c_uint,
                    b"appverify\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s:%d]: ret not ok\0" as *const u8 as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 17],
                        [core::ffi::c_char; 17],
                    >(*b"ComputeBlockHash\0"))
                        .as_ptr(),
                    62 as core::ffi::c_int,
                );
                current_block = 13109744992413637920;
                break;
            } else {
                let mut chunkContentPrefix: [core::ffi::c_uchar; 5] = [
                    0xa5 as core::ffi::c_int as core::ffi::c_uchar,
                    0 as core::ffi::c_int as core::ffi::c_uchar,
                    0 as core::ffi::c_int as core::ffi::c_uchar,
                    0 as core::ffi::c_int as core::ffi::c_uchar,
                    0 as core::ffi::c_int as core::ffi::c_uchar,
                ];
                if memcpy_s(
                    chunkContentPrefix
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as isize)
                        as *mut core::ffi::c_void,
                    (5 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                    &mut readLen as *mut int32_t as *const core::ffi::c_void,
                    ::core::mem::size_of::<core::ffi::c_int>() as size_t,
                ) != 0 as core::ffi::c_int
                {
                    HiLogPrint(
                        LOG_CORE,
                        LOG_ERROR,
                        0xd001100 as core::ffi::c_uint,
                        b"appverify\0" as *const u8 as *const core::ffi::c_char,
                        b"[%s:%d]: memcpy_s fail\0" as *const u8
                            as *const core::ffi::c_char,
                        (::core::mem::transmute::<
                            [u8; 17],
                            [core::ffi::c_char; 17],
                        >(*b"ComputeBlockHash\0"))
                            .as_ptr(),
                        65 as core::ffi::c_int,
                    );
                    current_block = 13109744992413637920;
                    break;
                } else {
                    ret = mbedtls_md_update(
                        mdCtx,
                        chunkContentPrefix.as_mut_ptr(),
                        5 as size_t,
                    ) as int32_t;
                    if ret != V_OK as core::ffi::c_int {
                        HiLogPrint(
                            LOG_CORE,
                            LOG_ERROR,
                            0xd001100 as core::ffi::c_uint,
                            b"appverify\0" as *const u8 as *const core::ffi::c_char,
                            b"[%s:%d]: ret not ok\0" as *const u8
                                as *const core::ffi::c_char,
                            (::core::mem::transmute::<
                                [u8; 17],
                                [core::ffi::c_char; 17],
                            >(*b"ComputeBlockHash\0"))
                                .as_ptr(),
                            69 as core::ffi::c_int,
                        );
                        current_block = 13109744992413637920;
                        break;
                    } else {
                        HiLogPrint(
                            LOG_CORE,
                            LOG_INFO,
                            0xd001100 as core::ffi::c_uint,
                            b"appverify\0" as *const u8 as *const core::ffi::c_char,
                            b"[%s:%d]: content: %d, %d\0" as *const u8
                                as *const core::ffi::c_char,
                            (::core::mem::transmute::<
                                [u8; 17],
                                [core::ffi::c_char; 17],
                            >(*b"ComputeBlockHash\0"))
                                .as_ptr(),
                            70 as core::ffi::c_int,
                            rawBufLen,
                            pos,
                        );
                        ret = mbedtls_md_update(
                            mdCtx,
                            (block as *mut core::ffi::c_uchar).offset(pos as isize),
                            readLen as size_t,
                        ) as int32_t;
                        if ret != V_OK as core::ffi::c_int {
                            HiLogPrint(
                                LOG_CORE,
                                LOG_ERROR,
                                0xd001100 as core::ffi::c_uint,
                                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                                b"[%s:%d]: ret not ok\0" as *const u8
                                    as *const core::ffi::c_char,
                                (::core::mem::transmute::<
                                    [u8; 17],
                                    [core::ffi::c_char; 17],
                                >(*b"ComputeBlockHash\0"))
                                    .as_ptr(),
                                72 as core::ffi::c_int,
                            );
                            current_block = 13109744992413637920;
                            break;
                        } else {
                            rawBufLen -= readLen as core::ffi::c_int;
                            pos += readLen as core::ffi::c_int;
                            let mut outbuf: *mut core::ffi::c_uchar = malloc(
                                hlen as size_t,
                            ) as *mut core::ffi::c_uchar;
                            if outbuf.is_null() {
                                HiLogPrint(
                                    LOG_CORE,
                                    LOG_ERROR,
                                    0xd001100 as core::ffi::c_uint,
                                    b"appverify\0" as *const u8 as *const core::ffi::c_char,
                                    b"[%s:%d]: outbuf is null\0" as *const u8
                                        as *const core::ffi::c_char,
                                    (::core::mem::transmute::<
                                        [u8; 17],
                                        [core::ffi::c_char; 17],
                                    >(*b"ComputeBlockHash\0"))
                                        .as_ptr(),
                                    76 as core::ffi::c_int,
                                );
                                current_block = 13109744992413637920;
                                break;
                            } else {
                                ret = mbedtls_md_finish(mdCtx, outbuf) as int32_t;
                                HapPutData(result, *offset, outbuf, hlen as int32_t);
                                *offset = (*offset as core::ffi::c_uint)
                                    .wrapping_add(hlen as core::ffi::c_uint) as int32_t
                                    as int32_t;
                                memset_s(
                                    outbuf as *mut core::ffi::c_void,
                                    hlen,
                                    0 as core::ffi::c_int,
                                    hlen,
                                );
                                if !outbuf.is_null() {
                                    free(outbuf as *mut core::ffi::c_void);
                                    outbuf = 0 as *mut core::ffi::c_uchar;
                                }
                                if ret != V_OK as core::ffi::c_int {
                                    HiLogPrint(
                                        LOG_CORE,
                                        LOG_ERROR,
                                        0xd001100 as core::ffi::c_uint,
                                        b"appverify\0" as *const u8 as *const core::ffi::c_char,
                                        b"[%s:%d]: ret not ok\0" as *const u8
                                            as *const core::ffi::c_char,
                                        (::core::mem::transmute::<
                                            [u8; 17],
                                            [core::ffi::c_char; 17],
                                        >(*b"ComputeBlockHash\0"))
                                            .as_ptr(),
                                        82 as core::ffi::c_int,
                                    );
                                    current_block = 13109744992413637920;
                                    break;
                                } else {
                                    mbedtls_md_free(mdCtx);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        13109744992413637920 => {
            mbedtls_md_free(mdCtx);
            if !mdCtx.is_null() {
                free(mdCtx as *mut core::ffi::c_void);
                mdCtx = 0 as *mut mbedtls_md_context_t;
            }
            return V_ERR as core::ffi::c_uint as int32_t;
        }
        _ => {
            if !mdCtx.is_null() {
                free(mdCtx as *mut core::ffi::c_void);
                mdCtx = 0 as *mut mbedtls_md_context_t;
            }
            return V_OK as core::ffi::c_int as int32_t;
        }
    };
}
unsafe extern "C" fn GetChunkSumCount(
    mut fileSize: int32_t,
    mut coreDirectorySize: int32_t,
    mut eocdSize: int32_t,
    mut rootHashLen: int32_t,
) -> int32_t {
    let mut chunkSize: int32_t = 1024 as int32_t * 1024 as int32_t;
    let mut maxSize: int32_t = 0x7fffffff as int32_t - chunkSize;
    if fileSize > maxSize || coreDirectorySize > maxSize || eocdSize > maxSize {
        return 0 as int32_t;
    }
    let mut count: int32_t = (fileSize - 1 as int32_t + chunkSize) / chunkSize
        + (coreDirectorySize - 1 as int32_t + chunkSize) / chunkSize
        + (eocdSize - 1 as int32_t + chunkSize) / chunkSize;
    if rootHashLen < 0 as core::ffi::c_int
        || (0x7fffffff as int32_t - 5 as int32_t) / count < rootHashLen
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: overflow count: %d, chunkDigestLen: %d\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"GetChunkSumCount\0"))
                .as_ptr(),
            103 as core::ffi::c_int,
            count,
            rootHashLen,
        );
        return 0 as int32_t;
    }
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        0xd001100 as core::ffi::c_uint,
        b"appverify\0" as *const u8 as *const core::ffi::c_char,
        b"[%s:%d]: get sum count %d\0" as *const u8 as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 17],
            [core::ffi::c_char; 17],
        >(*b"GetChunkSumCount\0"))
            .as_ptr(),
        106 as core::ffi::c_int,
        count,
    );
    return count;
}
unsafe extern "C" fn ComputeDigestsWithOptionalBlock(
    digestAlgorithm: int32_t,
    mut fp: int32_t,
    mut signInfo: *const SignatureInfo,
    mut chunkDigest: *const HapBuf,
    mut fianlDigest: *const HapBuf,
) -> int32_t {
    let mut readLen: int32_t = 0;
    let mut rst: int32_t = V_ERR as core::ffi::c_uint as int32_t;
    let mut rawBuf: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut outbuf: *mut core::ffi::c_uchar = 0 as *mut core::ffi::c_uchar;
    let mut rootHashLen: int32_t = GetHashUnitLen(digestAlgorithm);
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        0xd001100 as core::ffi::c_uint,
        b"appverify\0" as *const u8 as *const core::ffi::c_char,
        b"[%s:%d]: rootHashLen %d\0" as *const u8 as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 32],
            [core::ffi::c_char; 32],
        >(*b"ComputeDigestsWithOptionalBlock\0"))
            .as_ptr(),
        117 as core::ffi::c_int,
        rootHashLen,
    );
    if rootHashLen <= 0 as core::ffi::c_int || rootHashLen > 64 as core::ffi::c_int {
        return rst;
    }
    let mut mdInfo: *const mbedtls_md_info_t = mbedtls_md_info_from_type(
        digestAlgorithm as mbedtls_md_type_t,
    );
    if mdInfo.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: mdInfo is null\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 32],
                [core::ffi::c_char; 32],
            >(*b"ComputeDigestsWithOptionalBlock\0"))
                .as_ptr(),
            122 as core::ffi::c_int,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    let mut mdCtx: *mut mbedtls_md_context_t = malloc(
        ::core::mem::size_of::<mbedtls_md_context_t>() as size_t,
    ) as *mut mbedtls_md_context_t;
    if mdCtx.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: mdCtx is null\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 32],
                [core::ffi::c_char; 32],
            >(*b"ComputeDigestsWithOptionalBlock\0"))
                .as_ptr(),
            124 as core::ffi::c_int,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    mbedtls_md_init(mdCtx);
    let mut ret: int32_t = mbedtls_md_setup(mdCtx, mdInfo, 0 as core::ffi::c_int)
        as int32_t;
    let mut rawLen: int32_t = 0 as int32_t;
    let mut blockHead: BlockHead = {
        let mut init = BlockHead {
            type_0: 0 as uint32_t,
            length: 0,
            offset: 0,
        };
        init
    };
    if ret != V_OK as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: ret not ok\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 32],
                [core::ffi::c_char; 32],
            >(*b"ComputeDigestsWithOptionalBlock\0"))
                .as_ptr(),
            130 as core::ffi::c_int,
        );
    } else {
        ret = mbedtls_md_starts(mdCtx) as int32_t;
        if ret != V_OK as core::ffi::c_int {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd001100 as core::ffi::c_uint,
                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                b"[%s:%d]: ret not ok\0" as *const u8 as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 32],
                    [core::ffi::c_char; 32],
                >(*b"ComputeDigestsWithOptionalBlock\0"))
                    .as_ptr(),
                132 as core::ffi::c_int,
            );
        } else {
            readLen = (*chunkDigest).len;
            HiLogPrint(
                LOG_CORE,
                LOG_INFO,
                0xd001100 as core::ffi::c_uint,
                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                b"[%s:%d]: readLen %d\0" as *const u8 as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 32],
                    [core::ffi::c_char; 32],
                >(*b"ComputeDigestsWithOptionalBlock\0"))
                    .as_ptr(),
                134 as core::ffi::c_int,
                readLen,
            );
            ret = mbedtls_md_update(
                mdCtx,
                (*chunkDigest).buffer as *const core::ffi::c_uchar,
                readLen as size_t,
            ) as int32_t;
            if ret != V_OK as core::ffi::c_int {
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    0xd001100 as core::ffi::c_uint,
                    b"appverify\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s:%d]: ret not ok\0" as *const u8 as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 32],
                        [core::ffi::c_char; 32],
                    >(*b"ComputeDigestsWithOptionalBlock\0"))
                        .as_ptr(),
                    136 as core::ffi::c_int,
                );
            } else {
                rawBuf = GetSignBlockByType(
                    signInfo,
                    fp,
                    PROFILE_BLOCK_WITHSIGN_TYPE as core::ffi::c_int as int32_t,
                    &mut rawLen,
                    &mut blockHead,
                );
                if rawBuf.is_null() {
                    HiLogPrint(
                        LOG_CORE,
                        LOG_ERROR,
                        0xd001100 as core::ffi::c_uint,
                        b"appverify\0" as *const u8 as *const core::ffi::c_char,
                        b"[%s:%d]: rawBuf is null\0" as *const u8
                            as *const core::ffi::c_char,
                        (::core::mem::transmute::<
                            [u8; 32],
                            [core::ffi::c_char; 32],
                        >(*b"ComputeDigestsWithOptionalBlock\0"))
                            .as_ptr(),
                        139 as core::ffi::c_int,
                    );
                } else {
                    readLen = rawLen;
                    HiLogPrint(
                        LOG_CORE,
                        LOG_INFO,
                        0xd001100 as core::ffi::c_uint,
                        b"appverify\0" as *const u8 as *const core::ffi::c_char,
                        b"[%s:%d]: signBuf %0x %d\0" as *const u8
                            as *const core::ffi::c_char,
                        (::core::mem::transmute::<
                            [u8; 32],
                            [core::ffi::c_char; 32],
                        >(*b"ComputeDigestsWithOptionalBlock\0"))
                            .as_ptr(),
                        141 as core::ffi::c_int,
                        *rawBuf.offset(0 as core::ffi::c_int as isize)
                            as core::ffi::c_int,
                        readLen,
                    );
                    ret = mbedtls_md_update(
                        mdCtx,
                        rawBuf as *mut core::ffi::c_uchar,
                        readLen as size_t,
                    ) as int32_t;
                    if ret != V_OK as core::ffi::c_int {
                        HiLogPrint(
                            LOG_CORE,
                            LOG_ERROR,
                            0xd001100 as core::ffi::c_uint,
                            b"appverify\0" as *const u8 as *const core::ffi::c_char,
                            b"[%s:%d]: ret not ok\0" as *const u8
                                as *const core::ffi::c_char,
                            (::core::mem::transmute::<
                                [u8; 32],
                                [core::ffi::c_char; 32],
                            >(*b"ComputeDigestsWithOptionalBlock\0"))
                                .as_ptr(),
                            143 as core::ffi::c_int,
                        );
                    } else {
                        outbuf = malloc(rootHashLen as size_t)
                            as *mut core::ffi::c_uchar;
                        if outbuf.is_null() {
                            HiLogPrint(
                                LOG_CORE,
                                LOG_ERROR,
                                0xd001100 as core::ffi::c_uint,
                                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                                b"[%s:%d]: outbuf is null\0" as *const u8
                                    as *const core::ffi::c_char,
                                (::core::mem::transmute::<
                                    [u8; 32],
                                    [core::ffi::c_char; 32],
                                >(*b"ComputeDigestsWithOptionalBlock\0"))
                                    .as_ptr(),
                                145 as core::ffi::c_int,
                            );
                        } else {
                            ret = mbedtls_md_finish(mdCtx, outbuf) as int32_t;
                            if ret != V_OK as core::ffi::c_int {
                                HiLogPrint(
                                    LOG_CORE,
                                    LOG_ERROR,
                                    0xd001100 as core::ffi::c_uint,
                                    b"appverify\0" as *const u8 as *const core::ffi::c_char,
                                    b"[%s:%d]: ret not ok\0" as *const u8
                                        as *const core::ffi::c_char,
                                    (::core::mem::transmute::<
                                        [u8; 32],
                                        [core::ffi::c_char; 32],
                                    >(*b"ComputeDigestsWithOptionalBlock\0"))
                                        .as_ptr(),
                                    147 as core::ffi::c_int,
                                );
                            } else {
                                HapPutData(fianlDigest, 0 as int32_t, outbuf, rootHashLen);
                                memset_s(
                                    outbuf as *mut core::ffi::c_void,
                                    rootHashLen as size_t,
                                    0 as core::ffi::c_int,
                                    rootHashLen as size_t,
                                );
                                rst = V_OK as core::ffi::c_int as int32_t;
                            }
                        }
                    }
                }
            }
        }
    }
    mbedtls_md_free(mdCtx);
    if !mdCtx.is_null() {
        free(mdCtx as *mut core::ffi::c_void);
        mdCtx = 0 as *mut mbedtls_md_context_t;
    }
    if !rawBuf.is_null() {
        free(rawBuf as *mut core::ffi::c_void);
        rawBuf = 0 as *mut core::ffi::c_char;
    }
    if !outbuf.is_null() {
        free(outbuf as *mut core::ffi::c_void);
        outbuf = 0 as *mut core::ffi::c_uchar;
    }
    return rst;
}
unsafe extern "C" fn HapUpdateDigistHead(
    mut digestAlgorithm: int32_t,
    mut mdCtx: *mut mbedtls_md_context_t,
    mut mdInfo: *const mbedtls_md_info_t,
    mut readLen: int32_t,
    mut hlen: *mut size_t,
) -> int32_t {
    mbedtls_md_init(mdCtx);
    let mut ret: int32_t = mbedtls_md_setup(mdCtx, mdInfo, 0 as core::ffi::c_int)
        as int32_t;
    if ret != 0 as core::ffi::c_int {
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    *hlen = mbedtls_md_get_size(mdInfo) as size_t;
    if *hlen == 0 as core::ffi::c_uint || *hlen > 64 as core::ffi::c_uint {
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    ret = mbedtls_md_starts(mdCtx) as int32_t;
    if ret != 0 as core::ffi::c_int {
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    let mut chunkContentPrefix: [core::ffi::c_uchar; 5] = [
        0xa5 as core::ffi::c_int as core::ffi::c_uchar,
        0 as core::ffi::c_int as core::ffi::c_uchar,
        0 as core::ffi::c_int as core::ffi::c_uchar,
        0 as core::ffi::c_int as core::ffi::c_uchar,
        0 as core::ffi::c_int as core::ffi::c_uchar,
    ];
    if memcpy_s(
        chunkContentPrefix.as_mut_ptr().offset(1 as core::ffi::c_int as isize)
            as *mut core::ffi::c_void,
        (5 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
        &mut readLen as *mut int32_t as *const core::ffi::c_void,
        ::core::mem::size_of::<core::ffi::c_int>() as size_t,
    ) != 0 as core::ffi::c_int
    {
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    ret = mbedtls_md_update(mdCtx, chunkContentPrefix.as_mut_ptr(), 5 as size_t)
        as int32_t;
    if ret != 0 as core::ffi::c_int {
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    return V_OK as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn UpdateSmallBlock(
    mut readLen: int32_t,
    fp: int32_t,
    mut mdCtx: *mut mbedtls_md_context_t,
) -> int32_t {
    let mut readLenLeft: int32_t = readLen;
    while readLenLeft > 0 as core::ffi::c_int {
        let mut onceRead: int32_t = if readLenLeft
            > 1024 as core::ffi::c_int * 64 as core::ffi::c_int
        {
            1024 as int32_t * 64 as int32_t
        } else {
            readLenLeft
        };
        let mut onceBuf: *mut core::ffi::c_uchar = malloc(onceRead as size_t)
            as *mut core::ffi::c_uchar;
        if onceBuf.is_null() {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd001100 as core::ffi::c_uint,
                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                b"[%s:%d]: onceBuf is null\0" as *const u8 as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"UpdateSmallBlock\0"))
                    .as_ptr(),
                193 as core::ffi::c_int,
            );
            return V_ERR as core::ffi::c_uint as int32_t;
        }
        let mut len: int32_t = read(
            fp as core::ffi::c_int,
            onceBuf as *mut core::ffi::c_void,
            (::core::mem::size_of::<core::ffi::c_char>() as usize)
                .wrapping_mul(onceRead as usize) as size_t,
        ) as int32_t;
        if len != onceRead {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd001100 as core::ffi::c_uint,
                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                b"[%s:%d]: fread err: %d, %d\0" as *const u8 as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"UpdateSmallBlock\0"))
                    .as_ptr(),
                196 as core::ffi::c_int,
                len,
                onceRead,
            );
            if !onceBuf.is_null() {
                free(onceBuf as *mut core::ffi::c_void);
                onceBuf = 0 as *mut core::ffi::c_uchar;
            }
            return V_ERR as core::ffi::c_uint as int32_t;
        }
        let mut ret: int32_t = mbedtls_md_update(mdCtx, onceBuf, onceRead as size_t)
            as int32_t;
        if !onceBuf.is_null() {
            free(onceBuf as *mut core::ffi::c_void);
            onceBuf = 0 as *mut core::ffi::c_uchar;
        }
        if ret != V_OK as core::ffi::c_int {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd001100 as core::ffi::c_uint,
                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                b"[%s:%d]: ret not ok\0" as *const u8 as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"UpdateSmallBlock\0"))
                    .as_ptr(),
                202 as core::ffi::c_int,
            );
            return ret;
        }
        readLenLeft -= onceRead as core::ffi::c_int;
    }
    return V_OK as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn ComputerFileHash(
    mut signInfo: *const SignatureInfo,
    mut digestAlgorithm: int32_t,
    fp: int32_t,
    mut chunkDigest: *const HapBuf,
    mut offset: *mut int32_t,
) -> int32_t {
    let mut current_block: u64;
    let mut mdCtx: *mut mbedtls_md_context_t = malloc(
        ::core::mem::size_of::<mbedtls_md_context_t>() as size_t,
    ) as *mut mbedtls_md_context_t;
    if mdCtx.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: mdCtx is null\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"ComputerFileHash\0"))
                .as_ptr(),
            212 as core::ffi::c_int,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    lseek(fp as core::ffi::c_int, 0 as off_t, 0 as core::ffi::c_int);
    let mut pos: int32_t = 0 as int32_t;
    let mut rawBufLen: int32_t = (*signInfo).fullSignBlockOffset;
    loop {
        if !(rawBufLen > 0 as core::ffi::c_int) {
            current_block = 18435049525520518667;
            break;
        }
        let mut hlen: size_t = 0 as size_t;
        let mut readLen: int32_t = if rawBufLen
            > 1024 as core::ffi::c_int * 1024 as core::ffi::c_int
        {
            1024 as int32_t * 1024 as int32_t
        } else {
            rawBufLen
        };
        let mut mdInfo: *const mbedtls_md_info_t = mbedtls_md_info_from_type(
            digestAlgorithm as mbedtls_md_type_t,
        );
        if mdInfo.is_null() {
            if !mdCtx.is_null() {
                free(mdCtx as *mut core::ffi::c_void);
                mdCtx = 0 as *mut mbedtls_md_context_t;
            }
            return V_ERR as core::ffi::c_uint as int32_t;
        }
        let mut ret: int32_t = HapUpdateDigistHead(
            digestAlgorithm,
            mdCtx,
            mdInfo,
            readLen,
            &mut hlen,
        );
        if ret != V_OK as core::ffi::c_int {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd001100 as core::ffi::c_uint,
                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                b"[%s:%d]: ret not ok\0" as *const u8 as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"ComputerFileHash\0"))
                    .as_ptr(),
                225 as core::ffi::c_int,
            );
            current_block = 11618802771708950166;
            break;
        } else {
            HiLogPrint(
                LOG_CORE,
                LOG_INFO,
                0xd001100 as core::ffi::c_uint,
                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                b"[%s:%d]: content: %d, %d\0" as *const u8 as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"ComputerFileHash\0"))
                    .as_ptr(),
                226 as core::ffi::c_int,
                rawBufLen,
                pos,
            );
            ret = UpdateSmallBlock(readLen, fp, mdCtx);
            if ret != V_OK as core::ffi::c_int {
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    0xd001100 as core::ffi::c_uint,
                    b"appverify\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s:%d]: ret not ok\0" as *const u8 as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 17],
                        [core::ffi::c_char; 17],
                    >(*b"ComputerFileHash\0"))
                        .as_ptr(),
                    228 as core::ffi::c_int,
                );
                current_block = 11618802771708950166;
                break;
            } else {
                rawBufLen -= readLen as core::ffi::c_int;
                pos += readLen as core::ffi::c_int;
                let mut outbuf: *mut core::ffi::c_uchar = malloc(hlen as size_t)
                    as *mut core::ffi::c_uchar;
                if outbuf.is_null() {
                    HiLogPrint(
                        LOG_CORE,
                        LOG_ERROR,
                        0xd001100 as core::ffi::c_uint,
                        b"appverify\0" as *const u8 as *const core::ffi::c_char,
                        b"[%s:%d]: outbuf is null\0" as *const u8
                            as *const core::ffi::c_char,
                        (::core::mem::transmute::<
                            [u8; 17],
                            [core::ffi::c_char; 17],
                        >(*b"ComputerFileHash\0"))
                            .as_ptr(),
                        232 as core::ffi::c_int,
                    );
                    current_block = 11618802771708950166;
                    break;
                } else {
                    ret = mbedtls_md_finish(mdCtx, outbuf) as int32_t;
                    HapPutData(chunkDigest, *offset, outbuf, hlen as int32_t);
                    memset_s(
                        outbuf as *mut core::ffi::c_void,
                        hlen,
                        0 as core::ffi::c_int,
                        hlen,
                    );
                    *offset = (*offset as core::ffi::c_uint)
                        .wrapping_add(hlen as core::ffi::c_uint) as int32_t as int32_t;
                    if !outbuf.is_null() {
                        free(outbuf as *mut core::ffi::c_void);
                        outbuf = 0 as *mut core::ffi::c_uchar;
                    }
                    if ret != V_OK as core::ffi::c_int {
                        HiLogPrint(
                            LOG_CORE,
                            LOG_ERROR,
                            0xd001100 as core::ffi::c_uint,
                            b"appverify\0" as *const u8 as *const core::ffi::c_char,
                            b"[%s:%d]: ret not ok\0" as *const u8
                                as *const core::ffi::c_char,
                            (::core::mem::transmute::<
                                [u8; 17],
                                [core::ffi::c_char; 17],
                            >(*b"ComputerFileHash\0"))
                                .as_ptr(),
                            238 as core::ffi::c_int,
                        );
                        current_block = 11618802771708950166;
                        break;
                    } else {
                        mbedtls_md_free(mdCtx);
                    }
                }
            }
        }
    }
    match current_block {
        11618802771708950166 => {
            mbedtls_md_free(mdCtx);
            if !mdCtx.is_null() {
                free(mdCtx as *mut core::ffi::c_void);
                mdCtx = 0 as *mut mbedtls_md_context_t;
            }
            return V_ERR as core::ffi::c_uint as int32_t;
        }
        _ => {
            if !mdCtx.is_null() {
                free(mdCtx as *mut core::ffi::c_void);
                mdCtx = 0 as *mut mbedtls_md_context_t;
            }
            return V_OK as core::ffi::c_int as int32_t;
        }
    };
}
unsafe extern "C" fn ComputerCoreDirHash(
    mut signInfo: *const SignatureInfo,
    mut digestAlgorithm: int32_t,
    fp: int32_t,
    mut chunkDigest: *const HapBuf,
    mut offset: *mut int32_t,
) -> int32_t {
    let mut centralDirSize: int32_t = (*signInfo).hapEocdOffset
        - (*signInfo).hapCoreDirOffset;
    if centralDirSize <= 0 as core::ffi::c_int {
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    let mut dirBuf: *mut core::ffi::c_char = malloc(centralDirSize as size_t)
        as *mut core::ffi::c_char;
    if dirBuf.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: dirBuf is null\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"ComputerCoreDirHash\0"))
                .as_ptr(),
            257 as core::ffi::c_int,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    lseek(
        fp as core::ffi::c_int,
        (*signInfo).hapCoreDirOffset as off_t,
        0 as core::ffi::c_int,
    );
    let mut len: int32_t = read(
        fp as core::ffi::c_int,
        dirBuf as *mut core::ffi::c_void,
        (::core::mem::size_of::<core::ffi::c_char>() as usize)
            .wrapping_mul(centralDirSize as usize) as size_t,
    ) as int32_t;
    if len != centralDirSize {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: fread err: %d, %d\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"ComputerCoreDirHash\0"))
                .as_ptr(),
            261 as core::ffi::c_int,
            len,
            centralDirSize,
        );
        if !dirBuf.is_null() {
            free(dirBuf as *mut core::ffi::c_void);
            dirBuf = 0 as *mut core::ffi::c_char;
        }
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    let mut ret: int32_t = ComputeBlockHash(
        dirBuf,
        centralDirSize,
        digestAlgorithm,
        chunkDigest,
        offset,
    );
    memset_s(
        dirBuf as *mut core::ffi::c_void,
        centralDirSize as size_t,
        0 as core::ffi::c_int,
        centralDirSize as size_t,
    );
    if !dirBuf.is_null() {
        free(dirBuf as *mut core::ffi::c_void);
        dirBuf = 0 as *mut core::ffi::c_char;
    }
    if ret != V_OK as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: ret not ok\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"ComputerCoreDirHash\0"))
                .as_ptr(),
            268 as core::ffi::c_int,
        );
        return ret;
    }
    return V_OK as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn ComputerEocdHash(
    mut signInfo: *const SignatureInfo,
    mut digestAlgorithm: int32_t,
    fp: int32_t,
    mut chunkDigest: *const HapBuf,
    mut offset: *mut int32_t,
) -> int32_t {
    if (*signInfo).hapEocdSize <= 0 as core::ffi::c_int {
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    let mut eocdBuf: *mut HapEocd = malloc((*signInfo).hapEocdSize as size_t)
        as *mut HapEocd;
    if eocdBuf.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: eocdBuf is null\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"ComputerEocdHash\0"))
                .as_ptr(),
            279 as core::ffi::c_int,
        );
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    lseek(
        fp as core::ffi::c_int,
        (*signInfo).hapEocdOffset as off_t,
        0 as core::ffi::c_int,
    );
    let mut len: int32_t = read(
        fp as core::ffi::c_int,
        eocdBuf as *mut core::ffi::c_void,
        (*signInfo).hapEocdSize as size_t,
    ) as int32_t;
    if len != (*signInfo).hapEocdSize {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: fread err: %d, %d\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"ComputerEocdHash\0"))
                .as_ptr(),
            283 as core::ffi::c_int,
            len,
            (*signInfo).hapEocdSize,
        );
        if !eocdBuf.is_null() {
            free(eocdBuf as *mut core::ffi::c_void);
            eocdBuf = 0 as *mut HapEocd;
        }
        return V_ERR as core::ffi::c_uint as int32_t;
    }
    HapPutInt32(
        &mut (*eocdBuf).eocdHead.coreDirOffset as *mut int32_t
            as *mut core::ffi::c_uchar,
        ::core::mem::size_of::<core::ffi::c_int>() as int32_t,
        (*signInfo).fullSignBlockOffset,
    );
    let mut ret: int32_t = ComputeBlockHash(
        eocdBuf as *mut core::ffi::c_char,
        len,
        digestAlgorithm,
        chunkDigest,
        offset,
    );
    memset_s(
        eocdBuf as *mut core::ffi::c_void,
        (*signInfo).hapEocdSize as size_t,
        0 as core::ffi::c_int,
        (*signInfo).hapEocdSize as size_t,
    );
    if !eocdBuf.is_null() {
        free(eocdBuf as *mut core::ffi::c_void);
        eocdBuf = 0 as *mut HapEocd;
    }
    if ret != V_OK as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: ret not ok\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"ComputerEocdHash\0"))
                .as_ptr(),
            291 as core::ffi::c_int,
        );
        return ret;
    }
    return V_OK as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn VerifyIntegrityChunk(
    mut digestAlgorithm: int32_t,
    fp: int32_t,
    mut signInfo: *const SignatureInfo,
    mut actualDigest: *const HapBuf,
) -> bool {
    if signInfo.is_null() || actualDigest.is_null() || ((*actualDigest).buffer).is_null()
    {
        return 0 as core::ffi::c_int != 0;
    }
    let mut centralDirSize: int32_t = (*signInfo).hapEocdOffset
        - (*signInfo).hapCoreDirOffset;
    let mut rootHashLen: int32_t = GetHashUnitLen(digestAlgorithm);
    if rootHashLen < 0 as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: alg error\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"VerifyIntegrityChunk\0"))
                .as_ptr(),
            304 as core::ffi::c_int,
        );
        return 0 as core::ffi::c_int != 0;
    }
    let mut sumCount: int32_t = GetChunkSumCount(
        (*signInfo).fullSignBlockOffset,
        centralDirSize,
        (*signInfo).hapEocdSize,
        rootHashLen,
    );
    if sumCount == 0 as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: sum count error\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"VerifyIntegrityChunk\0"))
                .as_ptr(),
            310 as core::ffi::c_int,
        );
        return 0 as core::ffi::c_int != 0;
    }
    let mut sumOfChunksLen: int32_t = 5 as int32_t + sumCount * rootHashLen;
    let mut chunkDigest: HapBuf = {
        let mut init = HapBuf {
            buffer: 0 as *mut core::ffi::c_void,
            len: 0,
        };
        init
    };
    if !CreateHapBuffer(&mut chunkDigest, sumOfChunksLen) {
        return 0 as core::ffi::c_int != 0;
    }
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        0xd001100 as core::ffi::c_uint,
        b"appverify\0" as *const u8 as *const core::ffi::c_char,
        b"[%s:%d]: alg: %d\0" as *const u8 as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 21],
            [core::ffi::c_char; 21],
        >(*b"VerifyIntegrityChunk\0"))
            .as_ptr(),
        318 as core::ffi::c_int,
        digestAlgorithm,
    );
    HapPutByte(&mut chunkDigest, 0 as int32_t, 0x5a as core::ffi::c_char);
    HapSetInt32(&mut chunkDigest, 1 as int32_t, sumCount);
    let mut offset: int32_t = 5 as int32_t;
    let mut ret: int32_t = 0;
    ret = ComputerFileHash(signInfo, digestAlgorithm, fp, &mut chunkDigest, &mut offset);
    if ret != V_OK as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: ret not ok\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"VerifyIntegrityChunk\0"))
                .as_ptr(),
            324 as core::ffi::c_int,
        );
    } else {
        ret = ComputerCoreDirHash(
            signInfo,
            digestAlgorithm,
            fp,
            &mut chunkDigest,
            &mut offset,
        );
        if ret != V_OK as core::ffi::c_int {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd001100 as core::ffi::c_uint,
                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                b"[%s:%d]: ret not ok\0" as *const u8 as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 21],
                    [core::ffi::c_char; 21],
                >(*b"VerifyIntegrityChunk\0"))
                    .as_ptr(),
                326 as core::ffi::c_int,
            );
        } else {
            ret = ComputerEocdHash(
                signInfo,
                digestAlgorithm,
                fp,
                &mut chunkDigest,
                &mut offset,
            );
            if ret != V_OK as core::ffi::c_int {
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    0xd001100 as core::ffi::c_uint,
                    b"appverify\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s:%d]: ret not ok\0" as *const u8 as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 21],
                        [core::ffi::c_char; 21],
                    >(*b"VerifyIntegrityChunk\0"))
                        .as_ptr(),
                    328 as core::ffi::c_int,
                );
            } else {
                ret = ComputeDigestsWithOptionalBlock(
                    digestAlgorithm,
                    fp,
                    signInfo,
                    &mut chunkDigest,
                    actualDigest,
                );
                if ret != V_OK as core::ffi::c_int {
                    HiLogPrint(
                        LOG_CORE,
                        LOG_ERROR,
                        0xd001100 as core::ffi::c_uint,
                        b"appverify\0" as *const u8 as *const core::ffi::c_char,
                        b"[%s:%d]: ret not ok\0" as *const u8
                            as *const core::ffi::c_char,
                        (::core::mem::transmute::<
                            [u8; 21],
                            [core::ffi::c_char; 21],
                        >(*b"VerifyIntegrityChunk\0"))
                            .as_ptr(),
                        330 as core::ffi::c_int,
                    );
                } else {
                    ClearHapBuffer(&mut chunkDigest);
                    HiLogPrint(
                        LOG_CORE,
                        LOG_INFO,
                        0xd001100 as core::ffi::c_uint,
                        b"appverify\0" as *const u8 as *const core::ffi::c_char,
                        b"[%s:%d]: finish\0" as *const u8 as *const core::ffi::c_char,
                        (::core::mem::transmute::<
                            [u8; 21],
                            [core::ffi::c_char; 21],
                        >(*b"VerifyIntegrityChunk\0"))
                            .as_ptr(),
                        332 as core::ffi::c_int,
                    );
                    return 1 as core::ffi::c_int != 0;
                }
            }
        }
    }
    HiLogPrint(
        LOG_CORE,
        LOG_ERROR,
        0xd001100 as core::ffi::c_uint,
        b"appverify\0" as *const u8 as *const core::ffi::c_char,
        b"[%s:%d]: exit\0" as *const u8 as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 21],
            [core::ffi::c_char; 21],
        >(*b"VerifyIntegrityChunk\0"))
            .as_ptr(),
        335 as core::ffi::c_int,
    );
    ClearHapBuffer(&mut chunkDigest);
    return 0 as core::ffi::c_int != 0;
}
