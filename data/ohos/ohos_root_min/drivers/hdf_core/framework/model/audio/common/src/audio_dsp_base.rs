extern "C" {
    pub type HdfSBuf;
    fn DeviceResourceGetIfaceInstance(
        type_0: DeviceResourceType,
    ) -> *mut DeviceResourceIface;
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
pub type uint8_t = core::ffi::c_uchar;
pub type uint16_t = core::ffi::c_ushort;
pub type uint32_t = core::ffi::c_uint;
pub type uint64_t = core::ffi::c_ulonglong;
pub type C2RustUnnamed = core::ffi::c_int;
pub const HDF_DEV_ERR_NETDOWN: C2RustUnnamed = -211;
pub const HDF_DEV_ERR_OP: C2RustUnnamed = -210;
pub const HDF_DEV_ERR_NORANGE: C2RustUnnamed = -208;
pub const HDF_DEV_ERR_NODATA: C2RustUnnamed = -207;
pub const HDF_DEV_ERR_ATTACHDEV_FAIL: C2RustUnnamed = -206;
pub const HDF_DEV_ERR_PUBLISH_FAIL: C2RustUnnamed = -205;
pub const HDF_DEV_ERR_DEV_INIT_FAIL: C2RustUnnamed = -204;
pub const HDF_DEV_ERR_NO_DEVICE_SERVICE: C2RustUnnamed = -203;
pub const HDF_DEV_ERR_NO_DEVICE: C2RustUnnamed = -202;
pub const HDF_DEV_ERR_NO_MEMORY: C2RustUnnamed = -201;
pub const HDF_PAL_ERR_INNER: C2RustUnnamed = -104;
pub const HDF_PAL_ERR_DEV_CREATE: C2RustUnnamed = -103;
pub const HDF_ERR_BSP_PLT_API_ERR: C2RustUnnamed = -102;
pub const HDF_BSP_ERR_OP: C2RustUnnamed = -101;
pub const HDF_ERR_OUT_OF_RANGE: C2RustUnnamed = -20;
pub const HDF_ERR_NOPERM: C2RustUnnamed = -19;
pub const HDF_ERR_BAD_FD: C2RustUnnamed = -18;
pub const HDF_ERR_IO: C2RustUnnamed = -17;
pub const HDF_ERR_DEVICE_BUSY: C2RustUnnamed = -16;
pub const HDF_ERR_QUEUE_FULL: C2RustUnnamed = -15;
pub const HDF_ERR_THREAD_CREATE_FAIL: C2RustUnnamed = -10;
pub const HDF_ERR_TIMEOUT: C2RustUnnamed = -7;
pub const HDF_ERR_MALLOC_FAIL: C2RustUnnamed = -6;
pub const HDF_ERR_INVALID_OBJECT: C2RustUnnamed = -4;
pub const HDF_ERR_INVALID_PARAM: C2RustUnnamed = -3;
pub const HDF_ERR_NOT_SUPPORT: C2RustUnnamed = -2;
pub const HDF_FAILURE: C2RustUnnamed = -1;
pub const HDF_SUCCESS: C2RustUnnamed = 0;
pub type DeviceClass = core::ffi::c_uint;
pub const DEVICE_CLASS_MAX: DeviceClass = 1024;
pub const DEVICE_CLASS_HIMEDIACOMM: DeviceClass = 512;
pub const DEVICE_CLASS_USERAUTH: DeviceClass = 256;
pub const DEVICE_CLASS_USB: DeviceClass = 128;
pub const DEVICE_CLASS_CAMERA: DeviceClass = 64;
pub const DEVICE_CLASS_AUDIO: DeviceClass = 32;
pub const DEVICE_CLASS_DISPLAY: DeviceClass = 16;
pub const DEVICE_CLASS_INPUT: DeviceClass = 8;
pub const DEVICE_CLASS_SENSOR: DeviceClass = 4;
pub const DEVICE_CLASS_PLAT: DeviceClass = 2;
pub const DEVICE_CLASS_DEFAULT: DeviceClass = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfObject {
    pub objectId: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfDeviceObject {
    pub service: *mut IDeviceIoService,
    pub property: *const DeviceResourceNode,
    pub deviceClass: DeviceClass,
    pub priv_0: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceResourceNode {
    pub name: *const core::ffi::c_char,
    pub hashValue: uint32_t,
    pub attrData: *mut DeviceResourceAttr,
    pub parent: *mut DeviceResourceNode,
    pub child: *mut DeviceResourceNode,
    pub sibling: *mut DeviceResourceNode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceResourceAttr {
    pub name: *const core::ffi::c_char,
    pub value: *const core::ffi::c_char,
    pub next: *mut DeviceResourceAttr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IDeviceIoService {
    pub object: HdfObject,
    pub Open: Option<unsafe extern "C" fn(*mut HdfDeviceIoClient) -> int32_t>,
    pub Dispatch: Option<
        unsafe extern "C" fn(
            *mut HdfDeviceIoClient,
            core::ffi::c_int,
            *mut HdfSBuf,
            *mut HdfSBuf,
        ) -> int32_t,
    >,
    pub Release: Option<unsafe extern "C" fn(*mut HdfDeviceIoClient) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfDeviceIoClient {
    pub device: *mut HdfDeviceObject,
    pub priv_0: *mut core::ffi::c_void,
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
pub type DeviceResourceType = core::ffi::c_uint;
pub const INVALID: DeviceResourceType = 1;
pub const HDF_CONFIG_SOURCE: DeviceResourceType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceResourceIface {
    pub GetRootNode: Option<unsafe extern "C" fn() -> *const DeviceResourceNode>,
    pub GetBool: Option<
        unsafe extern "C" fn(*const DeviceResourceNode, *const core::ffi::c_char) -> bool,
    >,
    pub GetUint8: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            *mut uint8_t,
            uint8_t,
        ) -> int32_t,
    >,
    pub GetUint8ArrayElem: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            uint32_t,
            *mut uint8_t,
            uint8_t,
        ) -> int32_t,
    >,
    pub GetUint8Array: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            *mut uint8_t,
            uint32_t,
            uint8_t,
        ) -> int32_t,
    >,
    pub GetUint16: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            *mut uint16_t,
            uint16_t,
        ) -> int32_t,
    >,
    pub GetUint16ArrayElem: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            uint32_t,
            *mut uint16_t,
            uint16_t,
        ) -> int32_t,
    >,
    pub GetUint16Array: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            *mut uint16_t,
            uint32_t,
            uint16_t,
        ) -> int32_t,
    >,
    pub GetUint32: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            *mut uint32_t,
            uint32_t,
        ) -> int32_t,
    >,
    pub GetUint32ArrayElem: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            uint32_t,
            *mut uint32_t,
            uint32_t,
        ) -> int32_t,
    >,
    pub GetUint32Array: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            *mut uint32_t,
            uint32_t,
            uint32_t,
        ) -> int32_t,
    >,
    pub GetUint64: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            *mut uint64_t,
            uint64_t,
        ) -> int32_t,
    >,
    pub GetUint64ArrayElem: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            uint32_t,
            *mut uint64_t,
            uint64_t,
        ) -> int32_t,
    >,
    pub GetUint64Array: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            *mut uint64_t,
            uint32_t,
            uint64_t,
        ) -> int32_t,
    >,
    pub GetString: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            *mut *const core::ffi::c_char,
            *const core::ffi::c_char,
        ) -> int32_t,
    >,
    pub GetStringArrayElem: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            uint32_t,
            *mut *const core::ffi::c_char,
            *const core::ffi::c_char,
        ) -> int32_t,
    >,
    pub GetElemNum: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
        ) -> int32_t,
    >,
    pub GetNodeByMatchAttr: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
        ) -> *const DeviceResourceNode,
    >,
    pub GetChildNode: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
        ) -> *const DeviceResourceNode,
    >,
    pub GetNodeByRefAttr: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
        ) -> *const DeviceResourceNode,
    >,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn DspGetServiceName(
    mut device: *const HdfDeviceObject,
    mut drvDspName: *mut *const core::ffi::c_char,
) -> int32_t {
    let mut node: *const DeviceResourceNode = 0 as *const DeviceResourceNode;
    let mut drsOps: *mut DeviceResourceIface = 0 as *mut DeviceResourceIface;
    let mut ret: int32_t = 0;
    if device.is_null() || drvDspName.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: device is NULL.\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"DspGetServiceName\0"))
                .as_ptr(),
            21 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    node = (*device).property as *const DeviceResourceNode;
    if node.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: device property is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"DspGetServiceName\0"))
                .as_ptr(),
            27 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    drsOps = DeviceResourceGetIfaceInstance(HDF_CONFIG_SOURCE);
    if drsOps.is_null() || ((*drsOps).GetString).is_none() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: from resource get drsops failed!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"DspGetServiceName\0"))
                .as_ptr(),
            32 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    ret = ((*drsOps).GetString)
        .expect(
            "non-null function pointer",
        )(
        node,
        b"serviceName\0" as *const u8 as *const core::ffi::c_char,
        drvDspName,
        0 as *const core::ffi::c_char,
    );
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: read DspServiceName fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"DspGetServiceName\0"))
                .as_ptr(),
            38 as core::ffi::c_int,
        );
        return ret;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn DspGetDaiName(
    mut device: *const HdfDeviceObject,
    mut drvDaiName: *mut *const core::ffi::c_char,
) -> int32_t {
    let mut node: *const DeviceResourceNode = 0 as *const DeviceResourceNode;
    let mut drsOps: *mut DeviceResourceIface = 0 as *mut DeviceResourceIface;
    let mut ret: int32_t = 0;
    if device.is_null() || drvDaiName.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input para is null pointer.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"DspGetDaiName\0"))
                .as_ptr(),
            52 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    node = (*device).property as *const DeviceResourceNode;
    if node.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: drs node is null pointer.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"DspGetDaiName\0"))
                .as_ptr(),
            58 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    drsOps = DeviceResourceGetIfaceInstance(HDF_CONFIG_SOURCE);
    if drsOps.is_null() || ((*drsOps).GetString).is_none() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: drs ops fail!\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"DspGetDaiName\0"))
                .as_ptr(),
            63 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    ret = ((*drsOps).GetString)
        .expect(
            "non-null function pointer",
        )(
        node,
        b"dspDaiName\0" as *const u8 as *const core::ffi::c_char,
        drvDaiName,
        0 as *const core::ffi::c_char,
    );
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: read dspDaiName fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"DspGetDaiName\0"))
                .as_ptr(),
            69 as core::ffi::c_int,
        );
        return ret;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
pub const LOG_DOMAIN: core::ffi::c_int = 0xd002510 as core::ffi::c_int;
