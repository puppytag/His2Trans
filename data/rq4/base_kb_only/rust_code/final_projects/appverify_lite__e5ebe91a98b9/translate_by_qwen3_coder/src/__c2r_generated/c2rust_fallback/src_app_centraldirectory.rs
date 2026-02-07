#![allow(deref_nullptr)]
#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    pub fn malloc(_: size_t) -> *mut core::ffi::c_void;
    pub fn free(_: *mut core::ffi::c_void);
    pub fn lseek(_: core::ffi::c_int, _: off_t, _: core::ffi::c_int) -> off_t;
    pub fn read(_: core::ffi::c_int, _: *mut core::ffi::c_void, _: size_t) -> ssize_t;
    pub fn HapMMap(
        bufCapacity: int32_t,
        offset: int32_t,
        mmapInfo: *mut MmapInfo,
        file: *const FileRead,
    ) -> int32_t;
    pub fn HapMUnMap(mapAddr: *mut core::ffi::c_char, mmapSize: int32_t);
    pub fn HapGetInt(buf: *const core::ffi::c_uchar, len: int32_t) -> int32_t;
    pub fn HapGetShort(buf: *const core::ffi::c_uchar, len: int32_t) -> core::ffi::c_short;
    pub fn HapPutInt32(buf: *mut core::ffi::c_uchar, len: int32_t, value: int32_t);
    pub fn memset_s(
        dest: *mut core::ffi::c_void,
        destMax: size_t,
        c: core::ffi::c_int,
        count: size_t,
    ) -> errno_t;
    pub fn memcpy_s(
        dest: *mut core::ffi::c_void,
        destMax: size_t,
        src: *const core::ffi::c_void,
        count: size_t,
    ) -> errno_t;
    pub fn HiLogPrint(
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
pub type size_t = core::ffi::c_uint;
pub type ssize_t = core::ffi::c_int;
pub type off_t = core::ffi::c_longlong;
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
pub type ReadFileErrorCode = core::ffi::c_int;
pub const MMAP_PARAM_INVALID: ReadFileErrorCode = -6;
pub const MMAP_FAILED: ReadFileErrorCode = -5;
pub const READ_OFFSET_OUT_OF_RANGE: ReadFileErrorCode = -4;
pub const MMAP_COPY_FAILED: ReadFileErrorCode = -3;
pub const FILE_IS_CLOSE: ReadFileErrorCode = -2;
pub const DEST_BUFFER_IS_NULL: ReadFileErrorCode = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MmapInfo {
    pub mmapPosition: int32_t,
    pub readMoreLen: int32_t,
    pub mmapSize: int32_t,
    pub mapAddr: *mut core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FileRead {
    pub fp: int32_t,
    pub offset: int32_t,
    pub len: int32_t,
}
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
pub type errno_t = core::ffi::c_int;
pub unsafe extern "C" fn HapPutByte(
    mut hapBuffer: *const HapBuf,
    mut offset: int32_t,
    mut value: core::ffi::c_char,
) {
    if hapBuffer.is_null() || ((*hapBuffer).buffer).is_null() {
        return;
    }
    if offset >= 0 as core::ffi::c_int
        && (*hapBuffer).len - offset
            >= ::core::mem::size_of::<core::ffi::c_char>() as core::ffi::c_int
    {
        *((*hapBuffer).buffer as *mut core::ffi::c_char).offset(offset as isize) = value;
    }
}
pub unsafe extern "C" fn HapPutData(
    mut hapBuffer: *const HapBuf,
    mut offset: int32_t,
    mut data: *const core::ffi::c_uchar,
    mut len: int32_t,
) {
    if hapBuffer.is_null() || ((*hapBuffer).buffer).is_null() {
        return;
    }
    if !data.is_null() && offset >= 0 as core::ffi::c_int && len > 0 as core::ffi::c_int
        && (*hapBuffer).len - offset >= len
    {
        if memcpy_s(
            ((*hapBuffer).buffer as *mut core::ffi::c_uchar).offset(offset as isize)
                as *mut core::ffi::c_void,
            ((*hapBuffer).len - offset) as size_t,
            data as *const core::ffi::c_void,
            len as size_t,
        ) != 0 as core::ffi::c_int
        {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd001100 as core::ffi::c_uint,
                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                b"[%s:%d]: memcpy_s fail\0" as *const u8 as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 11],
                    [core::ffi::c_char; 11],
                >(*b"HapPutData\0"))
                    .as_ptr(),
                50 as core::ffi::c_int,
            );
        }
    }
}
pub unsafe extern "C" fn HapSetInt32(
    mut buffer: *const HapBuf,
    mut offset: int32_t,
    mut value: int32_t,
) {
    if value < 0 as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: int32 value of out range: %d\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 12],
                [core::ffi::c_char; 12],
            >(*b"HapSetInt32\0"))
                .as_ptr(),
            58 as core::ffi::c_int,
            value,
        );
        return;
    }
    if buffer.is_null() || ((*buffer).buffer).is_null() {
        return;
    }
    if offset >= 0 as core::ffi::c_int
        && (*buffer).len - offset
            >= ::core::mem::size_of::<int32_t>() as core::ffi::c_int
    {
        HapPutInt32(
            ((*buffer).buffer as *mut core::ffi::c_uchar).offset(offset as isize),
            (*buffer).len - offset,
            value,
        );
    }
}
pub unsafe extern "C" fn CreateHapBuffer(
    mut hapBuffer: *mut HapBuf,
    mut len: int32_t,
) -> bool {
    if hapBuffer.is_null() || len <= 0 as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: create buf fail, buf is null\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"CreateHapBuffer\0"))
                .as_ptr(),
            73 as core::ffi::c_int,
        );
        return 0 as core::ffi::c_int != 0;
    }
    (*hapBuffer).buffer = malloc(len as size_t);
    if ((*hapBuffer).buffer).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: create buf fail\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"CreateHapBuffer\0"))
                .as_ptr(),
            78 as core::ffi::c_int,
        );
        return 0 as core::ffi::c_int != 0;
    }
    (*hapBuffer).len = len;
    return 1 as core::ffi::c_int != 0;
}
pub unsafe extern "C" fn ClearHapBuffer(mut hapBuffer: *mut HapBuf) {
    if hapBuffer.is_null() || ((*hapBuffer).buffer).is_null() {
        return;
    }
    memset_s(
        (*hapBuffer).buffer,
        (*hapBuffer).len as size_t,
        0 as core::ffi::c_int,
        (*hapBuffer).len as size_t,
    );
    if !((*hapBuffer).buffer).is_null() {
        free((*hapBuffer).buffer);
        (*hapBuffer).buffer = 0 as *mut core::ffi::c_void;
    }
    (*hapBuffer).buffer = 0 as *mut core::ffi::c_void;
    (*hapBuffer).len = 0 as core::ffi::c_int as int32_t;
}
pub unsafe extern "C" fn GetEocd(
    mut hapFile: *const FileRead,
    mut hapEocd: *mut HapEocd,
    mut eocdOffset: *mut int32_t,
) -> bool {
    let mut mmapInfo: MmapInfo = {
        let mut init = MmapInfo {
            mmapPosition: 0 as int32_t,
            readMoreLen: 0,
            mmapSize: 0,
            mapAddr: 0 as *mut core::ffi::c_char,
        };
        init
    };
    if (*hapFile).len as usize <= ::core::mem::size_of::<MinEocd>() as usize {
        return 0 as core::ffi::c_int != 0;
    }
    let mut ret: int32_t = HapMMap((*hapFile).len, 0 as int32_t, &mut mmapInfo, hapFile);
    if ret != V_OK as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: mmap not ok\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<[u8; 8], [core::ffi::c_char; 8]>(*b"GetEocd\0"))
                .as_ptr(),
            104 as core::ffi::c_int,
        );
        return 0 as core::ffi::c_int != 0;
    }
    let mut fileStart: *mut core::ffi::c_char = (mmapInfo.mapAddr)
        .offset(mmapInfo.readMoreLen as isize);
    if HapGetShort(
        (fileStart as *mut core::ffi::c_uchar)
            .offset((*hapFile).len as isize)
            .offset(-(::core::mem::size_of::<core::ffi::c_short>() as usize as isize)),
        ::core::mem::size_of::<core::ffi::c_short>() as int32_t,
    ) as core::ffi::c_int == 0 as core::ffi::c_int
        && HapGetInt(
            (fileStart as *mut core::ffi::c_uchar)
                .offset((*hapFile).len as isize)
                .offset(-(::core::mem::size_of::<MinEocd>() as usize as isize)),
            ::core::mem::size_of::<core::ffi::c_int>() as int32_t,
        ) == 0x6054b50 as core::ffi::c_int
    {
        if memcpy_s(
            &mut (*hapEocd).eocdHead as *mut MinEocd as *mut core::ffi::c_void,
            ::core::mem::size_of::<MinEocd>() as size_t,
            fileStart
                .offset((*hapFile).len as isize)
                .offset(-(::core::mem::size_of::<MinEocd>() as usize as isize))
                as *const core::ffi::c_void,
            ::core::mem::size_of::<MinEocd>() as size_t,
        ) != 0 as core::ffi::c_int
        {
            HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd001100 as core::ffi::c_uint,
                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                b"[%s:%d]: copy error\0" as *const u8 as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 8],
                    [core::ffi::c_char; 8],
                >(*b"GetEocd\0"))
                    .as_ptr(),
                113 as core::ffi::c_int,
            );
            return 0 as core::ffi::c_int != 0;
        }
        HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
        *eocdOffset = ((*hapFile).len as usize)
            .wrapping_sub(::core::mem::size_of::<MinEocd>() as usize) as int32_t;
        return 1 as core::ffi::c_int != 0;
    }
    let mut maxReadLen: int32_t = (if ((*hapFile).len as usize)
        .wrapping_sub(::core::mem::size_of::<MinEocd>() as usize) as core::ffi::c_uint
        > 0xffff as core::ffi::c_uint
    {
        0xffff as usize
    } else {
        ((*hapFile).len as usize)
            .wrapping_sub(::core::mem::size_of::<MinEocd>() as usize)
    }) as int32_t;
    fileStart = fileStart
        .offset(
            ((*hapFile).len as usize)
                .wrapping_sub(::core::mem::size_of::<MinEocd>() as usize)
                .wrapping_sub(maxReadLen as usize) as isize,
        );
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        0xd001100 as core::ffi::c_uint,
        b"appverify\0" as *const u8 as *const core::ffi::c_char,
        b"[%s:%d]: maxReadLen %d\0" as *const u8 as *const core::ffi::c_char,
        (::core::mem::transmute::<[u8; 8], [core::ffi::c_char; 8]>(*b"GetEocd\0"))
            .as_ptr(),
        124 as core::ffi::c_int,
        maxReadLen,
    );
    let mut i: int32_t = 0 as int32_t;
    while i < maxReadLen {
        if HapGetShort(
            (fileStart as *mut core::ffi::c_uchar)
                .offset(i as isize)
                .offset(::core::mem::size_of::<MinEocd>() as usize as isize)
                .offset(
                    -(::core::mem::size_of::<core::ffi::c_short>() as usize as isize),
                ),
            ::core::mem::size_of::<core::ffi::c_short>() as int32_t,
        ) as core::ffi::c_int == maxReadLen - i
            && HapGetInt(
                (fileStart as *mut core::ffi::c_uchar).offset(i as isize),
                ::core::mem::size_of::<core::ffi::c_int>() as int32_t,
            ) == 0x6054b50 as core::ffi::c_int
        {
            if memcpy_s(
                &mut (*hapEocd).eocdHead as *mut MinEocd as *mut core::ffi::c_void,
                ::core::mem::size_of::<MinEocd>() as size_t,
                fileStart.offset(i as isize) as *const core::ffi::c_void,
                ::core::mem::size_of::<MinEocd>() as size_t,
            ) != 0 as core::ffi::c_int
            {
                HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    0xd001100 as core::ffi::c_uint,
                    b"appverify\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s:%d]: copy error\0" as *const u8 as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 8],
                        [core::ffi::c_char; 8],
                    >(*b"GetEocd\0"))
                        .as_ptr(),
                    132 as core::ffi::c_int,
                );
                return 0 as core::ffi::c_int != 0;
            }
            HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
            HiLogPrint(
                LOG_CORE,
                LOG_INFO,
                0xd001100 as core::ffi::c_uint,
                b"appverify\0" as *const u8 as *const core::ffi::c_char,
                b"[%s:%d]: comment num %d\0" as *const u8 as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 8],
                    [core::ffi::c_char; 8],
                >(*b"GetEocd\0"))
                    .as_ptr(),
                136 as core::ffi::c_int,
                maxReadLen - i,
            );
            *eocdOffset = ((*hapFile).len as usize)
                .wrapping_sub(::core::mem::size_of::<MinEocd>() as usize)
                .wrapping_sub((maxReadLen - i) as usize) as int32_t;
            return 1 as core::ffi::c_int != 0;
        }
        i += 1;
    }
    HiLogPrint(
        LOG_CORE,
        LOG_ERROR,
        0xd001100 as core::ffi::c_uint,
        b"appverify\0" as *const u8 as *const core::ffi::c_char,
        b"[%s:%d]: can not find eocd\0" as *const u8 as *const core::ffi::c_char,
        (::core::mem::transmute::<[u8; 8], [core::ffi::c_char; 8]>(*b"GetEocd\0"))
            .as_ptr(),
        141 as core::ffi::c_int,
    );
    HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
    return 0 as core::ffi::c_int != 0;
}
pub unsafe extern "C" fn FindSignature(
    mut hapFile: *const FileRead,
    mut signInfo: *mut SignatureInfo,
) -> bool {
    if hapFile.is_null() || signInfo.is_null() {
        return 0 as core::ffi::c_int != 0;
    }
    let mut eocdOffset: int32_t = 0 as int32_t;
    let mut hapEocd: HapEocd = {
        let mut init = HapEocd {
            eocdHead: {
                let mut init = MinEocd {
                    magic: 0 as int32_t,
                    diskNum: 0,
                    startNum: 0,
                    coreDirNumOnDisk: 0,
                    coreDirNum: 0,
                    coreDirSize: 0,
                    coreDirOffset: 0,
                    commentLen: 0,
                };
                init
            },
            comment: 0 as *mut core::ffi::c_char,
        };
        init
    };
    if !GetEocd(hapFile, &mut hapEocd, &mut eocdOffset) {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: find Eocd fail\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"FindSignature\0"))
                .as_ptr(),
            154 as core::ffi::c_int,
        );
        return 0 as core::ffi::c_int != 0;
    }
    (*signInfo).hapEocdOffset = eocdOffset;
    (*signInfo).hapEocdSize = (*hapFile).len - eocdOffset;
    (*signInfo).hapCoreDirOffset = HapGetInt(
        &mut hapEocd.eocdHead.coreDirOffset as *mut int32_t as *mut core::ffi::c_uchar,
        ::core::mem::size_of::<core::ffi::c_int>() as int32_t,
    );
    if (*signInfo).hapCoreDirOffset <= 0 as core::ffi::c_int
        || (*signInfo).hapCoreDirOffset >= eocdOffset
        || (*signInfo).hapEocdSize <= 0 as core::ffi::c_int
        || (*signInfo).hapEocdOffset <= 0 as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: core dir error\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"FindSignature\0"))
                .as_ptr(),
            162 as core::ffi::c_int,
        );
        return 0 as core::ffi::c_int != 0;
    }
    return 1 as core::ffi::c_int != 0;
}
pub unsafe extern "C" fn ReadFileFullyFromOffset(
    mut buffer: *const HapBuf,
    mut offset: int32_t,
    mut file: *const FileRead,
) -> int32_t {
    if buffer.is_null() || ((*buffer).buffer).is_null() || file.is_null() {
        return DEST_BUFFER_IS_NULL as core::ffi::c_int as int32_t;
    }
    if offset < 0 as core::ffi::c_int || offset > (*file).len {
        return READ_OFFSET_OUT_OF_RANGE as core::ffi::c_int as int32_t;
    }
    lseek((*file).fp as core::ffi::c_int, offset as off_t, 0 as core::ffi::c_int);
    let mut readLen: int32_t = read(
        (*file).fp as core::ffi::c_int,
        (*buffer).buffer,
        (*buffer).len as size_t,
    ) as int32_t;
    if readLen != (*buffer).len {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd001100 as core::ffi::c_uint,
            b"appverify\0" as *const u8 as *const core::ffi::c_char,
            b"[%s:%d]: file read error %d --- %d\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"ReadFileFullyFromOffset\0"))
                .as_ptr(),
            179 as core::ffi::c_int,
            readLen,
            (*buffer).len,
        );
        return READ_OFFSET_OUT_OF_RANGE as core::ffi::c_int as int32_t;
    }
    return (*buffer).len;
}
