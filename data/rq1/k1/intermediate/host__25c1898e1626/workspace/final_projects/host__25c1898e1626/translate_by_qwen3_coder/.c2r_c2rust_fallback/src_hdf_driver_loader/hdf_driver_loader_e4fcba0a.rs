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
    pub type DeviceResourceNode;
    pub type HdfSBuf;
    fn HdfRegisterDriverEntry(entry: *const HdfDriverEntry) -> int32_t;
    fn HdfDriverManagerGetDriver(driverName: *const core::ffi::c_char) -> *mut HdfDriver;
    fn HdfObjectManagerGetObject(objectId: core::ffi::c_int) -> *mut HdfObject;
    fn HiLogPrint(
        type_0: LogType,
        level: LogLevel,
        domain: core::ffi::c_uint,
        tag: *const core::ffi::c_char,
        fmt: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
}
pub type size_t = core::ffi::c_uint;
pub type int32_t = core::ffi::c_int;
pub type uint8_t = core::ffi::c_uchar;
pub type uint16_t = core::ffi::c_ushort;
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
pub struct DListHead {
    pub next: *mut DListHead,
    pub prev: *mut DListHead,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfObject {
    pub objectId: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfDriver {
    pub entry: *const HdfDriverEntry,
    pub type_0: uint16_t,
    pub bus: uint16_t,
    pub node: DListHead,
    pub priv_0: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfDriverEntry {
    pub moduleVersion: int32_t,
    pub moduleName: *const core::ffi::c_char,
    pub Bind: Option<unsafe extern "C" fn(*mut HdfDeviceObject) -> int32_t>,
    pub Init: Option<unsafe extern "C" fn(*mut HdfDeviceObject) -> int32_t>,
    pub Release: Option<unsafe extern "C" fn(*mut HdfDeviceObject) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfDeviceObject {
    pub service: *mut IDeviceIoService,
    pub property: *const DeviceResourceNode,
    pub deviceClass: DeviceClass,
    pub priv_0: *mut core::ffi::c_void,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IDriverLoader {
    pub object: HdfObject,
    pub GetDriver: Option<
        unsafe extern "C" fn(*const core::ffi::c_char) -> *mut HdfDriver,
    >,
    pub ReclaimDriver: Option<unsafe extern "C" fn(*mut HdfDriver) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfDriverLoader {
    pub super_0: IDriverLoader,
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
pub const HDF_OBJECT_ID_DRIVER_LOADER: C2RustUnnamed_0 = 4;
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const HDF_OBJECT_ID_MAX: C2RustUnnamed_0 = 9;
pub const HDF_OBJECT_ID_REMOTE_SERVICE: C2RustUnnamed_0 = 8;
pub const HDF_OBJECT_ID_DEVICE_SERVICE: C2RustUnnamed_0 = 7;
pub const HDF_OBJECT_ID_DEVICE_TOKEN: C2RustUnnamed_0 = 6;
pub const HDF_OBJECT_ID_DEVICE: C2RustUnnamed_0 = 5;
pub const HDF_OBJECT_ID_DRIVER_INSTALLER: C2RustUnnamed_0 = 3;
pub const HDF_OBJECT_ID_DEVHOST_SERVICE: C2RustUnnamed_0 = 2;
pub const HDF_OBJECT_ID_DEVSVC_MANAGER: C2RustUnnamed_0 = 1;
pub const HDF_OBJECT_ID_DEVMGR_SERVICE: C2RustUnnamed_0 = 0;
#[no_mangle]
pub unsafe extern "C" fn HdfDriverLoaderGetDriver(
    mut moduleName: *const core::ffi::c_char,
) -> *mut HdfDriver {
    if moduleName.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"driver_loader\0" as *const u8 as *const core::ffi::c_char,
            b"%{public}s: failed to get device entry, moduleName is NULL\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"HdfDriverLoaderGetDriver\0"))
                .as_ptr(),
        );
        return 0 as *mut HdfDriver;
    }
    return HdfDriverManagerGetDriver(moduleName);
}
#[no_mangle]
pub unsafe extern "C" fn HdfDriverLoaderReclaimDriver(mut driver: *mut HdfDriver) {}
#[no_mangle]
pub unsafe extern "C" fn HdfDriverLoaderConstruct(mut inst: *mut HdfDriverLoader) {
    if !inst.is_null() {
        (*inst).super_0.GetDriver = Some(
            HdfDriverLoaderGetDriver
                as unsafe extern "C" fn(*const core::ffi::c_char) -> *mut HdfDriver,
        ) as Option<unsafe extern "C" fn(*const core::ffi::c_char) -> *mut HdfDriver>;
        (*inst).super_0.ReclaimDriver = Some(
            HdfDriverLoaderReclaimDriver as unsafe extern "C" fn(*mut HdfDriver) -> (),
        ) as Option<unsafe extern "C" fn(*mut HdfDriver) -> ()>;
    }
}
#[no_mangle]
pub unsafe extern "C" fn HdfDriverLoaderCreate() -> *mut HdfObject {
    static mut isDriverLoaderInit: bool = 0 as core::ffi::c_int != 0;
    static mut driverLoader: HdfDriverLoader = HdfDriverLoader {
        super_0: IDriverLoader {
            object: HdfObject { objectId: 0 },
            GetDriver: None,
            ReclaimDriver: None,
        },
    };
    if !isDriverLoaderInit {
        if HdfDriverEntryConstruct() != HDF_SUCCESS as core::ffi::c_int {
            return 0 as *mut HdfObject;
        }
        HdfDriverLoaderConstruct(&mut driverLoader);
        isDriverLoaderInit = 1 as core::ffi::c_int != 0;
    }
    return &mut driverLoader as *mut HdfDriverLoader as *mut HdfObject;
}
#[no_mangle]
pub unsafe extern "C" fn HdfDriverLoaderGetInstance() -> *mut IDriverLoader {
    static mut instance: *mut IDriverLoader = 0 as *const IDriverLoader
        as *mut IDriverLoader;
    if instance.is_null() {
        instance = HdfObjectManagerGetObject(
            HDF_OBJECT_ID_DRIVER_LOADER as core::ffi::c_int,
        ) as *mut IDriverLoader;
    }
    return instance;
}
