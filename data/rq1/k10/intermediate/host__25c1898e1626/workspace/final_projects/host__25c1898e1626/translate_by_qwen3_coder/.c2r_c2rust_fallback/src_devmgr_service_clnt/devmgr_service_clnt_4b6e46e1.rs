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
    pub type HdfSBuf;
    pub type DeviceResourceNode;
    fn HdfObjectManagerGetObject(objectId: core::ffi::c_int) -> *mut HdfObject;
    fn HdfObjectManagerFreeObject(object: *mut HdfObject);
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
pub type uint16_t = core::ffi::c_ushort;
pub type uint32_t = core::ffi::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfObject {
    pub objectId: int32_t,
}
pub type devid_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfSListNode {
    pub next: *mut HdfSListNode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfDeviceInfo {
    pub node: HdfSListNode,
    pub isDynamic: bool,
    pub status: uint16_t,
    pub deviceType: uint16_t,
    pub deviceId: uint32_t,
    pub policy: uint16_t,
    pub priority: uint16_t,
    pub preload: uint16_t,
    pub permission: uint16_t,
    pub moduleName: *const core::ffi::c_char,
    pub svcName: *const core::ffi::c_char,
    pub deviceMatchAttr: *const core::ffi::c_char,
    pub deviceName: *const core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IDevHostService {
    pub object: HdfObject,
    pub AddDevice: Option<
        unsafe extern "C" fn(
            *mut IDevHostService,
            *const HdfDeviceInfo,
        ) -> core::ffi::c_int,
    >,
    pub DelDevice: Option<
        unsafe extern "C" fn(*mut IDevHostService, devid_t) -> core::ffi::c_int,
    >,
    pub StartService: Option<
        unsafe extern "C" fn(*mut IDevHostService) -> core::ffi::c_int,
    >,
    pub PmNotify: Option<
        unsafe extern "C" fn(*mut IDevHostService, uint32_t) -> core::ffi::c_int,
    >,
    pub Dump: Option<
        unsafe extern "C" fn(
            *mut IDevHostService,
            *mut HdfSBuf,
            *mut HdfSBuf,
        ) -> core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IHdfDeviceToken {
    pub object: HdfObject,
    pub devid: devid_t,
    pub servName: *const core::ffi::c_char,
    pub deviceName: *const core::ffi::c_char,
}
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
pub struct HdfDeviceObject {
    pub service: *mut IDeviceIoService,
    pub property: *const DeviceResourceNode,
    pub deviceClass: DeviceClass,
    pub priv_0: *mut core::ffi::c_void,
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
pub type HdfPowerState = core::ffi::c_uint;
pub const POWER_STATE_MAX: HdfPowerState = 4;
pub const POWER_STATE_SUSPEND: HdfPowerState = 3;
pub const POWER_STATE_RESUME: HdfPowerState = 2;
pub const POWER_STATE_DOZE_SUSPEND: HdfPowerState = 1;
pub const POWER_STATE_DOZE_RESUME: HdfPowerState = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IDevmgrService {
    pub base: HdfObject,
    pub object: HdfDeviceObject,
    pub AttachDeviceHost: Option<
        unsafe extern "C" fn(
            *mut IDevmgrService,
            uint16_t,
            *mut IDevHostService,
        ) -> core::ffi::c_int,
    >,
    pub AttachDevice: Option<
        unsafe extern "C" fn(
            *mut IDevmgrService,
            *mut IHdfDeviceToken,
        ) -> core::ffi::c_int,
    >,
    pub DetachDevice: Option<
        unsafe extern "C" fn(*mut IDevmgrService, devid_t) -> core::ffi::c_int,
    >,
    pub LoadDevice: Option<
        unsafe extern "C" fn(
            *mut IDevmgrService,
            *const core::ffi::c_char,
        ) -> core::ffi::c_int,
    >,
    pub UnloadDevice: Option<
        unsafe extern "C" fn(
            *mut IDevmgrService,
            *const core::ffi::c_char,
        ) -> core::ffi::c_int,
    >,
    pub StartService: Option<
        unsafe extern "C" fn(*mut IDevmgrService) -> core::ffi::c_int,
    >,
    pub PowerStateChange: Option<
        unsafe extern "C" fn(*mut IDevmgrService, HdfPowerState) -> core::ffi::c_int,
    >,
    pub ListAllDevice: Option<
        unsafe extern "C" fn(*mut IDevmgrService, *mut HdfSBuf) -> core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DevmgrServiceClnt {
    pub devMgrSvcIf: *mut IDevmgrService,
}
pub const HDF_OBJECT_ID_DEVMGR_SERVICE: C2RustUnnamed_0 = 0;
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
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const HDF_OBJECT_ID_MAX: C2RustUnnamed_0 = 9;
pub const HDF_OBJECT_ID_REMOTE_SERVICE: C2RustUnnamed_0 = 8;
pub const HDF_OBJECT_ID_DEVICE_SERVICE: C2RustUnnamed_0 = 7;
pub const HDF_OBJECT_ID_DEVICE_TOKEN: C2RustUnnamed_0 = 6;
pub const HDF_OBJECT_ID_DEVICE: C2RustUnnamed_0 = 5;
pub const HDF_OBJECT_ID_DRIVER_LOADER: C2RustUnnamed_0 = 4;
pub const HDF_OBJECT_ID_DRIVER_INSTALLER: C2RustUnnamed_0 = 3;
pub const HDF_OBJECT_ID_DEVHOST_SERVICE: C2RustUnnamed_0 = 2;
pub const HDF_OBJECT_ID_DEVSVC_MANAGER: C2RustUnnamed_0 = 1;
#[no_mangle]
pub unsafe extern "C" fn DevmgrServiceClntAttachDeviceHost(
    mut hostId: uint16_t,
    mut hostService: *mut IDevHostService,
) -> core::ffi::c_int {
    let mut devMgrSvcIf: *mut IDevmgrService = 0 as *mut IDevmgrService;
    let mut inst: *mut DevmgrServiceClnt = DevmgrServiceClntGetInstance();
    if inst.is_null() || ((*inst).devMgrSvcIf).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"devmgr_service_clnt\0" as *const u8 as *const core::ffi::c_char,
            b"failed to attach device host, get device manager service client is null\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return HDF_FAILURE as core::ffi::c_int;
    }
    devMgrSvcIf = (*inst).devMgrSvcIf;
    if ((*devMgrSvcIf).AttachDeviceHost).is_none() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"devmgr_service_clnt\0" as *const u8 as *const core::ffi::c_char,
            b"failed to attach device host, attach device host function is null\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return HDF_FAILURE as core::ffi::c_int;
    }
    return ((*devMgrSvcIf).AttachDeviceHost)
        .expect("non-null function pointer")(devMgrSvcIf, hostId, hostService);
}
#[no_mangle]
pub unsafe extern "C" fn DevmgrServiceClntAttachDevice(
    mut deviceToken: *mut IHdfDeviceToken,
) -> core::ffi::c_int {
    let mut devMgrSvcIf: *mut IDevmgrService = 0 as *mut IDevmgrService;
    let mut inst: *mut DevmgrServiceClnt = DevmgrServiceClntGetInstance();
    if inst.is_null() || ((*inst).devMgrSvcIf).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"devmgr_service_clnt\0" as *const u8 as *const core::ffi::c_char,
            b"devmgr client failed to attach device, inst is null\0" as *const u8
                as *const core::ffi::c_char,
        );
        return HDF_FAILURE as core::ffi::c_int;
    }
    devMgrSvcIf = (*inst).devMgrSvcIf;
    if ((*devMgrSvcIf).AttachDevice).is_none() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"devmgr_service_clnt\0" as *const u8 as *const core::ffi::c_char,
            b"devmgr client failed to attach device, dmsOps->AttachDevice is nul\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return HDF_FAILURE as core::ffi::c_int;
    }
    return ((*devMgrSvcIf).AttachDevice)
        .expect("non-null function pointer")(devMgrSvcIf, deviceToken);
}
#[no_mangle]
pub unsafe extern "C" fn DevmgrServiceClntDetachDevice(
    mut devid: devid_t,
) -> core::ffi::c_int {
    let mut devMgrSvcIf: *mut IDevmgrService = 0 as *mut IDevmgrService;
    let mut inst: *mut DevmgrServiceClnt = DevmgrServiceClntGetInstance();
    if inst.is_null() || ((*inst).devMgrSvcIf).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"devmgr_service_clnt\0" as *const u8 as *const core::ffi::c_char,
            b"devmgr client failed to deatch device, inst is null\0" as *const u8
                as *const core::ffi::c_char,
        );
        return HDF_FAILURE as core::ffi::c_int;
    }
    devMgrSvcIf = (*inst).devMgrSvcIf;
    if ((*devMgrSvcIf).DetachDevice).is_none() {
        return HDF_FAILURE as core::ffi::c_int;
    }
    return ((*devMgrSvcIf).DetachDevice)
        .expect("non-null function pointer")(devMgrSvcIf, devid);
}
#[no_mangle]
pub unsafe extern "C" fn DevmgrServiceClntGetInstance() -> *mut DevmgrServiceClnt {
    static mut instance: DevmgrServiceClnt = {
        let mut init = DevmgrServiceClnt {
            devMgrSvcIf: 0 as *const IDevmgrService as *mut IDevmgrService,
        };
        init
    };
    if (instance.devMgrSvcIf).is_null() {
        instance.devMgrSvcIf = HdfObjectManagerGetObject(
            HDF_OBJECT_ID_DEVMGR_SERVICE as core::ffi::c_int,
        ) as *mut IDevmgrService;
    }
    return &mut instance;
}
#[no_mangle]
pub unsafe extern "C" fn DevmgrServiceClntFreeInstance(
    mut inst: *mut DevmgrServiceClnt,
) {
    if !inst.is_null() && !((*inst).devMgrSvcIf).is_null() {
        HdfObjectManagerFreeObject((*inst).devMgrSvcIf as *mut HdfObject);
        (*inst).devMgrSvcIf = 0 as *mut IDevmgrService;
    }
}
