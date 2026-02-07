#![allow(deref_nullptr)]
#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
// === C2R_C2RUST_EXTERN_TYPES_BEGIN ===
// Auto-generated: downgraded c2rust `extern type` to stable-safe opaque structs.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceResourceNode {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HdfSBuf {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DevHostService {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PowerStateToken {
    _unused: [u8; 0],
}

// === C2R_C2RUST_EXTERN_TYPES_END ===

extern "C" {
    pub fn HdfObjectManagerGetObject(objectId: core::ffi::c_int) -> *mut HdfObject;
    pub fn HdfObjectManagerFreeObject(object: *mut HdfObject);
    pub fn OsalMemCalloc(size: size_t) -> *mut core::ffi::c_void;
    pub fn HdfDeviceNodeFreeInstance(devNode: *mut HdfDeviceNode);
    pub fn OsalMemFree(mem: *mut core::ffi::c_void);
    pub fn HiLogPrint(
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
pub struct DListHead {
    pub next: *mut DListHead,
    pub prev: *mut DListHead,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfObject {
    pub objectId: int32_t,
}
pub type devid_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfDeviceNode {
    pub super_0: IDeviceNode,
    pub entry: DListHead,
    pub powerToken: *mut PowerStateToken,
    pub hostService: *mut DevHostService,
    pub deviceObject: HdfDeviceObject,
    pub token: *mut IHdfDeviceToken,
    pub driver: *mut HdfDriver,
    pub device: *mut HdfDevice,
    pub servName: *mut core::ffi::c_char,
    pub servInfo: *const core::ffi::c_char,
    pub driverName: *mut core::ffi::c_char,
    pub devId: devid_t,
    pub policy: uint16_t,
    pub permission: uint16_t,
    pub devStatus: uint8_t,
    pub servStatus: bool,
    pub interfaceDesc: *mut core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfDevice {
    pub super_0: IHdfDevice,
    pub node: DListHead,
    pub devNodes: DListHead,
    pub deviceId: devid_t,
    pub devidIndex: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IHdfDevice {
    pub object: HdfObject,
    pub Attach: Option<
        unsafe extern "C" fn(*mut IHdfDevice, *mut HdfDeviceNode) -> core::ffi::c_int,
    >,
    pub Detach: Option<
        unsafe extern "C" fn(*mut IHdfDevice, *mut HdfDeviceNode) -> core::ffi::c_int,
    >,
    pub GetDeviceNode: Option<
        unsafe extern "C" fn(*mut IHdfDevice, devid_t) -> *mut HdfDeviceNode,
    >,
    pub DetachWithDevid: Option<
        unsafe extern "C" fn(*mut IHdfDevice, devid_t) -> core::ffi::c_int,
    >,
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
pub struct IHdfDeviceToken {
    pub object: HdfObject,
    pub devid: devid_t,
    pub servName: *const core::ffi::c_char,
    pub deviceName: *const core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IDeviceNode {
    pub object: HdfObject,
    pub PublishService: Option<
        unsafe extern "C" fn(*mut HdfDeviceNode) -> core::ffi::c_int,
    >,
    pub RemoveService: Option<
        unsafe extern "C" fn(*mut HdfDeviceNode) -> core::ffi::c_int,
    >,
    pub LaunchNode: Option<unsafe extern "C" fn(*mut HdfDeviceNode) -> core::ffi::c_int>,
    pub UnlaunchNode: Option<unsafe extern "C" fn(*mut HdfDeviceNode) -> ()>,
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
pub const HDF_OBJECT_ID_DEVICE: C2RustUnnamed_0 = 5;
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const HDF_OBJECT_ID_MAX: C2RustUnnamed_0 = 9;
pub const HDF_OBJECT_ID_REMOTE_SERVICE: C2RustUnnamed_0 = 8;
pub const HDF_OBJECT_ID_DEVICE_SERVICE: C2RustUnnamed_0 = 7;
pub const HDF_OBJECT_ID_DEVICE_TOKEN: C2RustUnnamed_0 = 6;
pub const HDF_OBJECT_ID_DRIVER_LOADER: C2RustUnnamed_0 = 4;
pub const HDF_OBJECT_ID_DRIVER_INSTALLER: C2RustUnnamed_0 = 3;
pub const HDF_OBJECT_ID_DEVHOST_SERVICE: C2RustUnnamed_0 = 2;
pub const HDF_OBJECT_ID_DEVSVC_MANAGER: C2RustUnnamed_0 = 1;
pub const HDF_OBJECT_ID_DEVMGR_SERVICE: C2RustUnnamed_0 = 0;
pub unsafe extern "C" fn UpdateDeivceNodeIdIndex(
    mut device: *mut HdfDevice,
    mut nodeDevid: devid_t,
) {
    if ((*device).devidIndex as core::ffi::c_uint)
        < nodeDevid
            & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                - 1 as core::ffi::c_int) as core::ffi::c_uint
    {
        (*device).devidIndex = (nodeDevid
            & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                - 1 as core::ffi::c_int) as core::ffi::c_uint) as uint16_t;
    }
}
pub unsafe extern "C" fn FindUsableDevNodeId(mut device: *mut HdfDevice) -> devid_t {
    let mut nodeId: uint16_t = 129 as uint16_t;
    let mut find: bool = 0 as core::ffi::c_int != 0;
    let mut devNode: *mut HdfDeviceNode = 0 as *mut HdfDeviceNode;
    while nodeId as core::ffi::c_int <= (*device).devidIndex as core::ffi::c_int {
        find = 0 as core::ffi::c_int != 0;
        devNode = ((*device).devNodes.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut HdfDeviceNode)).entry as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_long as *mut HdfDeviceNode;
        while &mut (*devNode).entry as *mut DListHead
            != &mut (*device).devNodes as *mut DListHead
        {
            if (*devNode).devId
                & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                    - 1 as core::ffi::c_int) as core::ffi::c_uint
                == nodeId as core::ffi::c_uint
            {
                find = 1 as core::ffi::c_int != 0;
                break;
            } else {
                devNode = ((*devNode).entry.next as *mut core::ffi::c_char)
                    .offset_from(
                        &mut (*(0 as *mut HdfDeviceNode)).entry as *mut DListHead
                            as *mut core::ffi::c_char,
                    ) as core::ffi::c_long as *mut HdfDeviceNode;
            }
        }
        if !find {
            return nodeId as devid_t;
        }
        nodeId = nodeId.wrapping_add(1);
    }
    return nodeId as devid_t;
}
pub unsafe extern "C" fn AcquireNodeDeivceId(
    mut device: *mut HdfDevice,
    mut devid: *mut devid_t,
) -> core::ffi::c_int {
    let mut nodeId: devid_t = 0;
    let mut usableId: devid_t = 0;
    if (*device).devidIndex as core::ffi::c_int
        >= ((1 as core::ffi::c_int) << 8 as core::ffi::c_int) - 1 as core::ffi::c_int
    {
        return HDF_FAILURE as core::ffi::c_int;
    }
    if ((*device).devidIndex as core::ffi::c_int) < 129 as core::ffi::c_int {
        (*device).devidIndex = 129 as uint16_t;
        nodeId = (*device).devidIndex as devid_t;
    } else {
        usableId = FindUsableDevNodeId(device);
        if usableId <= (*device).devidIndex as core::ffi::c_uint {
            nodeId = usableId;
        } else {
            (*device).devidIndex = ((*device).devidIndex).wrapping_add(1);
            nodeId = (*device).devidIndex as devid_t;
        }
    }
    if devid.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"hdf_device\0" as *const u8 as *const core::ffi::c_char,
            b"params invalid *devid\0" as *const u8 as *const core::ffi::c_char,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int;
    }
    *devid = ((((*device).deviceId >> 16 as core::ffi::c_int + 8 as core::ffi::c_int)
        as uint16_t as core::ffi::c_int)
        << 16 as core::ffi::c_int + 8 as core::ffi::c_int) as devid_t
        | ((*device).deviceId >> 8 as core::ffi::c_int
            & (((1 as core::ffi::c_int) << 16 as core::ffi::c_int)
                - 1 as core::ffi::c_int) as devid_t) << 8 as core::ffi::c_int | nodeId;
    return HDF_SUCCESS as core::ffi::c_int;
}
#[inline]
pub unsafe extern "C" fn DListHeadInit(mut head: *mut DListHead) {
    (*head).next = head;
    (*head).prev = head;
}
pub unsafe extern "C" fn HdfDeviceAttach(
    mut devInst: *mut IHdfDevice,
    mut devNode: *mut HdfDeviceNode,
) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int = 0;
    let mut device: *mut HdfDevice = devInst as *mut HdfDevice;
    let mut nodeIf: *mut IDeviceNode = devNode as *mut IDeviceNode;
    if device.is_null() || nodeIf.is_null() || ((*nodeIf).LaunchNode).is_none() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"hdf_device\0" as *const u8 as *const core::ffi::c_char,
            b"failed to attach device, input params invalid\0" as *const u8
                as *const core::ffi::c_char,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int;
    }
    if (*devNode).devId == 0 as core::ffi::c_uint
        && AcquireNodeDeivceId(device, &mut (*devNode).devId)
            != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"hdf_device\0" as *const u8 as *const core::ffi::c_char,
            b"failed to attach device, invalid device id\0" as *const u8
                as *const core::ffi::c_char,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int;
    }
    (*(*devNode).token).devid = (*devNode).devId;
    ret = ((*nodeIf).LaunchNode).expect("non-null function pointer")(devNode);
    if ret == HDF_SUCCESS as core::ffi::c_int {
        DListInsertTail(&mut (*devNode).entry, &mut (*device).devNodes);
        UpdateDeivceNodeIdIndex(device, (*devNode).devId);
    }
    return ret;
}
#[inline]
pub unsafe extern "C" fn DListRemove(mut entry: *mut DListHead) {
    (*(*entry).prev).next = (*entry).next;
    (*(*entry).next).prev = (*entry).prev;
    (*entry).prev = 0 as *mut DListHead;
    (*entry).next = 0 as *mut DListHead;
}
pub unsafe extern "C" fn HdfDeviceDetach(
    mut devInst: *mut IHdfDevice,
    mut devNode: *mut HdfDeviceNode,
) -> core::ffi::c_int {
    let mut device: *mut HdfDevice = 0 as *mut HdfDevice;
    if devInst.is_null() || devNode.is_null() {
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int;
    }
    device = (devInst as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut HdfDevice)).super_0 as *mut IHdfDevice
                as *mut core::ffi::c_char,
        ) as core::ffi::c_long as *mut HdfDevice;
    if (*device).deviceId >> 8 as core::ffi::c_int
        & (((1 as core::ffi::c_int) << 16 as core::ffi::c_int) - 1 as core::ffi::c_int)
            as core::ffi::c_uint
        != (*devNode).devId >> 8 as core::ffi::c_int
            & (((1 as core::ffi::c_int) << 16 as core::ffi::c_int)
                - 1 as core::ffi::c_int) as core::ffi::c_uint
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"hdf_device\0" as *const u8 as *const core::ffi::c_char,
            b"%{public}s: device detach unknown devnode \0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"HdfDeviceDetach\0"))
                .as_ptr(),
        );
        return HDF_DEV_ERR_NO_DEVICE as core::ffi::c_int;
    }
    if !((*devNode).entry.next).is_null() {
        DListRemove(&mut (*devNode).entry);
    }
    if ((*devNode).super_0.UnlaunchNode).is_some() {
        ((*devNode).super_0.UnlaunchNode).expect("non-null function pointer")(devNode);
    }
    return HDF_SUCCESS as core::ffi::c_int;
}
#[inline]
pub unsafe extern "C" fn DListInsertTail(
    mut entry: *mut DListHead,
    mut head: *mut DListHead,
) {
    (*entry).next = head;
    (*entry).prev = (*head).prev;
    (*(*head).prev).next = entry;
    (*head).prev = entry;
}
pub unsafe extern "C" fn HdfDeviceGetDeviceNode(
    mut device: *mut IHdfDevice,
    mut devid: devid_t,
) -> *mut HdfDeviceNode {
    let mut devNode: *mut HdfDeviceNode = 0 as *mut HdfDeviceNode;
    let mut dev: *mut HdfDevice = (device as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut HdfDevice)).super_0 as *mut IHdfDevice
                as *mut core::ffi::c_char,
        ) as core::ffi::c_long as *mut HdfDevice;
    devNode = ((*dev).devNodes.next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut HdfDeviceNode)).entry as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_long as *mut HdfDeviceNode;
    while &mut (*devNode).entry as *mut DListHead
        != &mut (*dev).devNodes as *mut DListHead
    {
        if (*devNode).devId == devid {
            return devNode;
        }
        devNode = ((*devNode).entry.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut HdfDeviceNode)).entry as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_long as *mut HdfDeviceNode;
    }
    return 0 as *mut HdfDeviceNode;
}
pub unsafe extern "C" fn HdfDeviceDetachWithDevid(
    mut device: *mut IHdfDevice,
    mut devid: devid_t,
) -> core::ffi::c_int {
    let mut dev: *mut HdfDevice = (device as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut HdfDevice)).super_0 as *mut IHdfDevice
                as *mut core::ffi::c_char,
        ) as core::ffi::c_long as *mut HdfDevice;
    let mut devNode: *mut HdfDeviceNode = HdfDeviceGetDeviceNode(device, devid);
    if devNode.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"hdf_device\0" as *const u8 as *const core::ffi::c_char,
            b"devNode is NULL\0" as *const u8 as *const core::ffi::c_char,
        );
        return HDF_DEV_ERR_NO_DEVICE as core::ffi::c_int;
    }
    return HdfDeviceDetach(device, devNode);
}
pub unsafe extern "C" fn HdfDeviceConstruct(mut device: *mut HdfDevice) {
    (*device).super_0.Attach = Some(
        HdfDeviceAttach
            as unsafe extern "C" fn(
                *mut IHdfDevice,
                *mut HdfDeviceNode,
            ) -> core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(*mut IHdfDevice, *mut HdfDeviceNode) -> core::ffi::c_int,
        >;
    (*device).super_0.Detach = Some(
        HdfDeviceDetach
            as unsafe extern "C" fn(
                *mut IHdfDevice,
                *mut HdfDeviceNode,
            ) -> core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(*mut IHdfDevice, *mut HdfDeviceNode) -> core::ffi::c_int,
        >;
    (*device).super_0.DetachWithDevid = Some(
        HdfDeviceDetachWithDevid
            as unsafe extern "C" fn(*mut IHdfDevice, devid_t) -> core::ffi::c_int,
    ) as Option<unsafe extern "C" fn(*mut IHdfDevice, devid_t) -> core::ffi::c_int>;
    (*device).super_0.GetDeviceNode = Some(
        HdfDeviceGetDeviceNode
            as unsafe extern "C" fn(*mut IHdfDevice, devid_t) -> *mut HdfDeviceNode,
    ) as Option<unsafe extern "C" fn(*mut IHdfDevice, devid_t) -> *mut HdfDeviceNode>;
    DListHeadInit(&mut (*device).devNodes);
}
pub unsafe extern "C" fn HdfDeviceDestruct(mut device: *mut HdfDevice) {
    let mut devNode: *mut HdfDeviceNode = 0 as *mut HdfDeviceNode;
    let mut tmp: *mut HdfDeviceNode = 0 as *mut HdfDeviceNode;
    devNode = ((*device).devNodes.next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut HdfDeviceNode)).entry as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_long as *mut HdfDeviceNode;
    tmp = ((*devNode).entry.next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut HdfDeviceNode)).entry as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_long as *mut HdfDeviceNode;
    while &mut (*devNode).entry as *mut DListHead
        != &mut (*device).devNodes as *mut DListHead
    {
        HdfDeviceNodeFreeInstance(devNode);
        devNode = tmp;
        tmp = ((*devNode).entry.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut HdfDeviceNode)).entry as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_long as *mut HdfDeviceNode;
    }
    DListHeadInit(&mut (*device).devNodes);
}
pub unsafe extern "C" fn HdfDeviceCreate() -> *mut HdfObject {
    let mut device: *mut HdfDevice = OsalMemCalloc(
        ::core::mem::size_of::<HdfDevice>() as size_t,
    ) as *mut HdfDevice;
    if !device.is_null() {
        HdfDeviceConstruct(device);
    }
    return device as *mut HdfObject;
}
pub unsafe extern "C" fn HdfDeviceRelease(mut object: *mut HdfObject) {
    let mut device: *mut HdfDevice = object as *mut HdfDevice;
    if !device.is_null() {
        HdfDeviceDestruct(device);
        OsalMemFree(device as *mut core::ffi::c_void);
    }
}
pub unsafe extern "C" fn HdfDeviceNewInstance() -> *mut HdfDevice {
    return HdfObjectManagerGetObject(HDF_OBJECT_ID_DEVICE as core::ffi::c_int)
        as *mut HdfDevice;
}
pub unsafe extern "C" fn HdfDeviceFreeInstance(mut device: *mut HdfDevice) {
    if !device.is_null() {
        HdfObjectManagerFreeObject(&mut (*device).super_0.object);
    }
}
