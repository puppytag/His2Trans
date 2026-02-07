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
    fn HdfStringMakeHashKey(key: *const core::ffi::c_char, mask: uint32_t) -> uint32_t;
    fn HdfServiceObserverRecordObtain(
        serviceKey: uint32_t,
    ) -> *mut HdfServiceObserverRecord;
    fn HdfServiceSubscriberObtain(
        callback: SubscriberCallback,
        devid: devid_t,
    ) -> *mut HdfServiceSubscriber;
    fn HdfServiceObserverRecordRecycle(observerRecord: *mut HdfServiceObserverRecord);
    fn HdfServiceObserverRecordCompare(
        listEntry: *mut HdfSListNode,
        serviceKey: uint32_t,
    ) -> bool;
    fn HdfServiceObserverRecordNotifySubscribers(
        record: *mut HdfServiceObserverRecord,
        deviceId: devid_t,
        policy: uint16_t,
    );
    fn HdfServiceObserverRecordDelete(listEntry: *mut HdfSListNode);
    fn HdfSListInit(list: *mut HdfSList);
    fn HdfSListSearch(
        list: *const HdfSList,
        keyValue: uint32_t,
        comparer: HdfSListSearchComparer,
    ) -> *mut HdfSListNode;
    fn OsalMutexInit(mutex: *mut OsalMutex) -> int32_t;
    fn HdfSListAdd(list: *mut HdfSList, link: *mut HdfSListNode);
    fn OsalMutexDestroy(mutex: *mut OsalMutex) -> int32_t;
    fn OsalMutexLock(mutex: *mut OsalMutex) -> int32_t;
    fn HdfSListRemove(list: *mut HdfSList, link: *mut HdfSListNode);
    fn HdfSListFlush(list: *mut HdfSList, deleter: HdfSListDeleter);
    fn OsalMutexUnlock(mutex: *mut OsalMutex) -> int32_t;
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
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const SERVICE_POLICY_INVALID: C2RustUnnamed_0 = 5;
pub const SERVICE_POLICY_PRIVATE: C2RustUnnamed_0 = 4;
pub const SERVICE_POLICY_FRIENDLY: C2RustUnnamed_0 = 3;
pub const SERVICE_POLICY_CAPACITY: C2RustUnnamed_0 = 2;
pub const SERVICE_POLICY_PUBLIC: C2RustUnnamed_0 = 1;
pub const SERVICE_POLICY_NONE: C2RustUnnamed_0 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubscriberCallback {
    pub deviceObject: *mut HdfDeviceObject,
    pub OnServiceConnected: Option<
        unsafe extern "C" fn(*mut HdfDeviceObject, *const HdfObject) -> int32_t,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfSListNode {
    pub next: *mut HdfSListNode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfSList {
    pub root: *mut HdfSListNode,
}
pub type HdfSListDeleter = Option<unsafe extern "C" fn(*mut HdfSListNode) -> ()>;
pub type HdfSListSearchComparer = Option<
    unsafe extern "C" fn(*mut HdfSListNode, uint32_t) -> bool,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OsalMutex {
    pub realMutex: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfServiceObserver {
    pub services: HdfSList,
    pub observerMutex: OsalMutex,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfServiceObserverRecord {
    pub entry: HdfSListNode,
    pub serviceKey: uint32_t,
    pub policy: uint16_t,
    pub devId: devid_t,
    pub obsRecMutex: OsalMutex,
    pub subscribers: HdfSList,
    pub publisher: *mut HdfObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfServiceSubscriber {
    pub entry: HdfSListNode,
    pub state: uint32_t,
    pub devId: uint32_t,
    pub callback: SubscriberCallback,
}
pub const HDF_SUBSCRIBER_STATE_READY: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_1 = core::ffi::c_uint;
pub const HDF_SUBSCRIBER_STATE_PENDING: C2RustUnnamed_1 = 0;
#[no_mangle]
pub unsafe extern "C" fn HdfServiceObserverConstruct(
    mut observer: *mut HdfServiceObserver,
) -> bool {
    if observer.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"service_observer\0" as *const u8 as *const core::ffi::c_char,
            b"observer is null\0" as *const u8 as *const core::ffi::c_char,
        );
        return 0 as core::ffi::c_int != 0;
    }
    if OsalMutexInit(&mut (*observer).observerMutex) != HDF_SUCCESS as core::ffi::c_int {
        return 0 as core::ffi::c_int != 0;
    }
    HdfSListInit(&mut (*observer).services);
    return 1 as core::ffi::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn HdfServiceObserverDestruct(
    mut observer: *mut HdfServiceObserver,
) {
    if !observer.is_null() {
        HdfSListFlush(
            &mut (*observer).services,
            Some(
                HdfServiceObserverRecordDelete
                    as unsafe extern "C" fn(*mut HdfSListNode) -> (),
            ),
        );
        OsalMutexDestroy(&mut (*observer).observerMutex);
    }
}
#[no_mangle]
pub unsafe extern "C" fn HdfServiceObserverSubscribeService(
    mut observer: *mut HdfServiceObserver,
    mut svcName: *const core::ffi::c_char,
    mut deviceId: devid_t,
    mut callback: SubscriberCallback,
) -> core::ffi::c_int {
    let mut serviceRecord: *mut HdfServiceObserverRecord = 0
        as *mut HdfServiceObserverRecord;
    let mut subscriber: *mut HdfServiceSubscriber = 0 as *mut HdfServiceSubscriber;
    let mut serviceKey: uint32_t = HdfStringMakeHashKey(svcName, 0 as uint32_t);
    if observer.is_null() || svcName.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"service_observer\0" as *const u8 as *const core::ffi::c_char,
            b"observer or svcName or callback.OnServiceConnected is null\0" as *const u8
                as *const core::ffi::c_char,
        );
        return HDF_FAILURE as core::ffi::c_int;
    }
    serviceRecord = HdfSListSearch(
        &mut (*observer).services,
        serviceKey,
        Some(
            HdfServiceObserverRecordCompare
                as unsafe extern "C" fn(*mut HdfSListNode, uint32_t) -> bool,
        ),
    ) as *mut HdfServiceObserverRecord;
    if serviceRecord.is_null() {
        serviceRecord = HdfServiceObserverRecordObtain(serviceKey);
        if serviceRecord.is_null() {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd002510 as core::ffi::c_uint,
                b"service_observer\0" as *const u8 as *const core::ffi::c_char,
                b"failed to subscribe service, serviceRecord is null\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return HDF_FAILURE as core::ffi::c_int;
        }
        subscriber = HdfServiceSubscriberObtain(callback, deviceId);
        if subscriber.is_null() {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd002510 as core::ffi::c_uint,
                b"service_observer\0" as *const u8 as *const core::ffi::c_char,
                b"failed to subscribe service, subscriber is null\0" as *const u8
                    as *const core::ffi::c_char,
            );
            HdfServiceObserverRecordRecycle(serviceRecord);
            return HDF_FAILURE as core::ffi::c_int;
        }
        OsalMutexLock(&mut (*observer).observerMutex);
        HdfSListAdd(&mut (*observer).services, &mut (*serviceRecord).entry);
        OsalMutexUnlock(&mut (*observer).observerMutex);
    } else {
        subscriber = HdfServiceSubscriberObtain(callback, deviceId);
        if subscriber.is_null() {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd002510 as core::ffi::c_uint,
                b"service_observer\0" as *const u8 as *const core::ffi::c_char,
                b"failed to subscribe service, subscriber obtain null\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return HDF_FAILURE as core::ffi::c_int;
        }
    }
    if !((*serviceRecord).publisher).is_null()
        && ((*subscriber).callback.OnServiceConnected).is_some()
        && ((*serviceRecord).policy as core::ffi::c_int
            != SERVICE_POLICY_PRIVATE as core::ffi::c_int
            || (*serviceRecord).devId == deviceId)
    {
        (*subscriber).state = HDF_SUBSCRIBER_STATE_READY as core::ffi::c_int as uint32_t;
        ((*subscriber).callback.OnServiceConnected)
            .expect(
                "non-null function pointer",
            )((*subscriber).callback.deviceObject, (*serviceRecord).publisher);
    }
    OsalMutexLock(&mut (*serviceRecord).obsRecMutex);
    HdfSListAdd(&mut (*serviceRecord).subscribers, &mut (*subscriber).entry);
    OsalMutexUnlock(&mut (*serviceRecord).obsRecMutex);
    return HDF_SUCCESS as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn HdfServiceObserverPublishService(
    mut observer: *mut HdfServiceObserver,
    mut svcName: *const core::ffi::c_char,
    mut deviceId: devid_t,
    mut policy: uint16_t,
    mut service: *mut HdfObject,
) -> core::ffi::c_int {
    let mut serviceRecord: *mut HdfServiceObserverRecord = 0
        as *mut HdfServiceObserverRecord;
    let mut serviceKey: uint32_t = HdfStringMakeHashKey(svcName, 0 as uint32_t);
    if observer.is_null() || svcName.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"service_observer\0" as *const u8 as *const core::ffi::c_char,
            b"observer or svcName is null\0" as *const u8 as *const core::ffi::c_char,
        );
        return HDF_FAILURE as core::ffi::c_int;
    }
    serviceRecord = HdfSListSearch(
        &mut (*observer).services,
        serviceKey,
        Some(
            HdfServiceObserverRecordCompare
                as unsafe extern "C" fn(*mut HdfSListNode, uint32_t) -> bool,
        ),
    ) as *mut HdfServiceObserverRecord;
    if serviceRecord.is_null() {
        serviceRecord = HdfServiceObserverRecordObtain(serviceKey);
        if serviceRecord.is_null() {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd002510 as core::ffi::c_uint,
                b"service_observer\0" as *const u8 as *const core::ffi::c_char,
                b"PublishService failed, serviceRecord is null\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return HDF_FAILURE as core::ffi::c_int;
        }
        (*serviceRecord).publisher = service;
        (*serviceRecord).devId = deviceId;
        (*serviceRecord).policy = policy;
        OsalMutexLock(&mut (*observer).observerMutex);
        HdfSListAdd(&mut (*observer).services, &mut (*serviceRecord).entry);
        OsalMutexUnlock(&mut (*observer).observerMutex);
    } else {
        (*serviceRecord).publisher = service;
        HdfServiceObserverRecordNotifySubscribers(serviceRecord, deviceId, policy);
    }
    return HDF_SUCCESS as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn HdfServiceObserverRemoveRecord(
    mut observer: *mut HdfServiceObserver,
    mut svcName: *const core::ffi::c_char,
) {
    let mut serviceRecord: *mut HdfServiceObserverRecord = 0
        as *mut HdfServiceObserverRecord;
    let mut serviceKey: uint32_t = HdfStringMakeHashKey(svcName, 0 as uint32_t);
    if observer.is_null() || svcName.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_WARN,
            0xd002510 as core::ffi::c_uint,
            b"service_observer\0" as *const u8 as *const core::ffi::c_char,
            b"observer or svcName is null\0" as *const u8 as *const core::ffi::c_char,
        );
        return;
    }
    serviceRecord = HdfSListSearch(
        &mut (*observer).services,
        serviceKey,
        Some(
            HdfServiceObserverRecordCompare
                as unsafe extern "C" fn(*mut HdfSListNode, uint32_t) -> bool,
        ),
    ) as *mut HdfServiceObserverRecord;
    if !serviceRecord.is_null() {
        OsalMutexLock(&mut (*observer).observerMutex);
        HdfSListRemove(&mut (*observer).services, &mut (*serviceRecord).entry);
        OsalMutexUnlock(&mut (*observer).observerMutex);
        HdfServiceObserverRecordRecycle(serviceRecord);
    }
}
