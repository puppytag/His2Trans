extern "C" {
    pub type HdfSBuf;
    fn strcmp(
        _: *const core::ffi::c_char,
        _: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
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
    fn OsalMemCalloc(size: size_t) -> *mut core::ffi::c_void;
    fn OsalMemFree(mem: *mut core::ffi::c_void);
    fn memset_s(
        dest: *mut core::ffi::c_void,
        destMax: size_t,
        c: core::ffi::c_int,
        count: size_t,
    ) -> errno_t;
    fn memcpy_s(
        dest: *mut core::ffi::c_void,
        destMax: size_t,
        src: *const core::ffi::c_void,
        count: size_t,
    ) -> errno_t;
}
pub type size_t = core::ffi::c_uint;
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
pub type errno_t = core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioConfigData {
    pub cardServiceName: *const core::ffi::c_char,
    pub codecName: *const core::ffi::c_char,
    pub platformName: *const core::ffi::c_char,
    pub cpuDaiName: *const core::ffi::c_char,
    pub codecDaiName: *const core::ffi::c_char,
    pub dspName: *const core::ffi::c_char,
    pub dspDaiName: *const core::ffi::c_char,
}
pub type AudioPortDirection = core::ffi::c_uint;
pub const PORT_OUT_IN: AudioPortDirection = 3;
pub const PORT_IN: AudioPortDirection = 2;
pub const PORT_OUT: AudioPortDirection = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioPcmStream {
    pub portDirection: uint64_t,
    pub formats: uint64_t,
    pub rates: uint64_t,
    pub rateMin: uint64_t,
    pub rateMax: uint64_t,
    pub channelsMin: uint64_t,
    pub channelsMax: uint64_t,
    pub bufferBytesMax: uint64_t,
    pub periodBytesMin: uint64_t,
    pub periodBytesMax: uint64_t,
    pub periodsMin: uint64_t,
    pub periodsMax: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioPortInfo {
    pub render: AudioPcmStream,
    pub capture: AudioPcmStream,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioMixerControl {
    pub min: uint32_t,
    pub max: uint32_t,
    pub platformMax: int32_t,
    pub mask: uint32_t,
    pub reg: uint32_t,
    pub rreg: uint32_t,
    pub shift: uint32_t,
    pub rshift: uint32_t,
    pub invert: uint32_t,
    pub value: uint32_t,
}
pub type AudioRegOpsType = core::ffi::c_uint;
pub const AUDIO_GROUP_MAX: AudioRegOpsType = 12;
pub const AUDIO_SAPM_CFG_GROUP: AudioRegOpsType = 11;
pub const AUDIO_SAPM_COMP_GROUP: AudioRegOpsType = 10;
pub const AUDIO_CTRL_CFG_GROUP: AudioRegOpsType = 9;
pub const AUDIO_DAI_TRIGGER_GROUP: AudioRegOpsType = 8;
pub const AUDIO_DAI_PATAM_GROUP: AudioRegOpsType = 7;
pub const AUDIO_DAI_STARTUP_PATAM_GROUP: AudioRegOpsType = 6;
pub const AUDIO_CTRL_SAPM_PATAM_MUX_GROUP: AudioRegOpsType = 5;
pub const AUDIO_CTRL_SAPM_PATAM_GROUP: AudioRegOpsType = 4;
pub const AUDIO_CTRL_PATAM_MUX_GROUP: AudioRegOpsType = 3;
pub const AUDIO_CTRL_PATAM_GROUP: AudioRegOpsType = 2;
pub const AUDIO_INIT_GROUP: AudioRegOpsType = 1;
pub const AUDIO_RSET_GROUP: AudioRegOpsType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioIdInfo {
    pub chipName: *const core::ffi::c_char,
    pub chipIdRegister: uint32_t,
    pub chipIdSize: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioControlConfig {
    pub arrayIndex: uint16_t,
    pub iface: uint16_t,
    pub type_0: uint16_t,
    pub enable: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioAddrConfig {
    pub addr: uint32_t,
    pub value: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioSapmCtrlConfig {
    pub sapmType: uint8_t,
    pub compNameIndex: uint16_t,
    pub reg: uint32_t,
    pub mask: uint32_t,
    pub shift: uint8_t,
    pub invert: uint8_t,
    pub kcontrolNews: uint32_t,
    pub kcontrolsNum: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioEnumCtrlConfig {
    pub reg: uint32_t,
    pub reg2: uint32_t,
    pub shiftLeft: uint8_t,
    pub shiftRight: uint8_t,
    pub max: uint32_t,
    pub mask: uint32_t,
    pub texts: uint32_t,
    pub values: uint32_t,
    pub sapm: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioRegCfgGroupNode {
    pub itemNum: uint8_t,
    pub groupIndex: AudioRegOpsType,
    pub addrCfgItem: *mut AudioAddrConfig,
    pub regCfgItem: *mut AudioMixerControl,
    pub regEnumCfgItem: *mut AudioEnumCtrlConfig,
    pub ctrlCfgItem: *mut AudioControlConfig,
    pub sapmCompItem: *mut AudioSapmCtrlConfig,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioRegCfgData {
    pub audioIdInfo: AudioIdInfo,
    pub audioRegParams: [*mut AudioRegCfgGroupNode; 12],
}
pub const AUDIO_SAPM_COMP_INDEX_KCTLNUM: AudioSapmComponentIndex = 7;
pub const AUDIO_SAPM_COMP_INDEX_KCTL: AudioSapmComponentIndex = 6;
pub const AUDIO_SAPM_COMP_INDEX_INVERT: AudioSapmComponentIndex = 5;
pub const AUDIO_SAPM_COMP_INDEX_SHIFT: AudioSapmComponentIndex = 4;
pub const AUDIO_SAPM_COMP_INDEX_MASK: AudioSapmComponentIndex = 3;
pub const AUDIO_SAPM_COMP_INDEX_REG: AudioSapmComponentIndex = 2;
pub const AUDIO_SAPM_COMP_INDEX_NAME: AudioSapmComponentIndex = 1;
pub const AUDIO_SAPM_COMP_INDEX_TYPE: AudioSapmComponentIndex = 0;
pub const AUDIO_SAPM_COMP_INDEX_MAX: AudioSapmComponentIndex = 8;
pub const AUDIO_ENUM_REG_CFG_SAPM_INDEX: AudioEnumRegCfgIndex = 8;
pub const AUDIO_ENUM_REG_CFG_VALUE_INDEX: AudioEnumRegCfgIndex = 7;
pub const AUDIO_ENUM_REG_CFG_TEXTS_INDEX: AudioEnumRegCfgIndex = 6;
pub const AUDIO_ENUM_REG_CFG_MASK_INDEX: AudioEnumRegCfgIndex = 5;
pub const AUDIO_ENUM_REG_CFG_MAX_INDEX: AudioEnumRegCfgIndex = 4;
pub const AUDIO_ENUM_REG_CFG_RSHIFT_INDEX: AudioEnumRegCfgIndex = 3;
pub const AUDIO_ENUM_REG_CFG_SHIFT_INDEX: AudioEnumRegCfgIndex = 2;
pub const AUDIO_ENUM_REG_CFG_RREG_INDEX: AudioEnumRegCfgIndex = 1;
pub const AUDIO_ENUM_REG_CFG_REG_INDEX: AudioEnumRegCfgIndex = 0;
pub const AUDIO_ENUM_REG_CFG_INDEX_MAX: AudioEnumRegCfgIndex = 9;
pub const AUDIO_REG_CFG_VALUE_INDEX: AudioRegCfgIndex = 8;
pub const AUDIO_REG_CFG_INVERT_INDEX: AudioRegCfgIndex = 7;
pub const AUDIO_REG_CFG_MASK_INDEX: AudioRegCfgIndex = 6;
pub const AUDIO_REG_CFG_MAX_INDEX: AudioRegCfgIndex = 5;
pub const AUDIO_REG_CFG_MIN_INDEX: AudioRegCfgIndex = 4;
pub const AUDIO_REG_CFG_RSHIFT_INDEX: AudioRegCfgIndex = 3;
pub const AUDIO_REG_CFG_SHIFT_INDEX: AudioRegCfgIndex = 2;
pub const AUDIO_REG_CFG_RREG_INDEX: AudioRegCfgIndex = 1;
pub const AUDIO_REG_CFG_REG_INDEX: AudioRegCfgIndex = 0;
pub const AUDIO_REG_CFG_INDEX_MAX: AudioRegCfgIndex = 9;
pub const AUDIO_ADDR_CFG_VALUE_INDEX: AudioAddrCfgIndex = 1;
pub const AUDIO_ADDR_CFG_REG_INDEX: AudioAddrCfgIndex = 0;
pub const AUDIO_ADDR_CFG_INDEX_MAX: AudioAddrCfgIndex = 2;
pub const AUDIO_CTRL_CFG_ENABLE_INDEX: AudioCrtlCfgIndex = 3;
pub const AUDIO_CTRL_CFG_TYPE_INDEX: AudioCrtlCfgIndex = 2;
pub const AUDIO_CTRL_CFG_IFACE_INDEX: AudioCrtlCfgIndex = 1;
pub const AUDIO_CTRL_CFG_INDEX_INDEX: AudioCrtlCfgIndex = 0;
pub const AUDIO_CTRL_CFG_INDEX_MAX: AudioCrtlCfgIndex = 4;
pub type AudioRegCfgIndex = core::ffi::c_uint;
pub type AudioEnumRegCfgIndex = core::ffi::c_uint;
pub type AudioAddrCfgIndex = core::ffi::c_uint;
pub type AudioCrtlCfgIndex = core::ffi::c_uint;
pub type AudioSapmComponentIndex = core::ffi::c_uint;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const AUDIO_CONFIG_MAX_ITEM: core::ffi::c_int = 500 as core::ffi::c_int;
pub const HW_INFO: [core::ffi::c_char; 7] = unsafe {
    ::core::mem::transmute::<[u8; 7], [core::ffi::c_char; 7]>(*b"hwInfo\0")
};
pub const PORT_INFO_LIST_LENGHT: core::ffi::c_int = 12 as core::ffi::c_int;
static mut g_audioRegGroupName: [*mut core::ffi::c_char; 12] = [
    b"resetSeqConfig\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"initSeqConfig\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"ctrlParamsSeqConfig\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"ctrlParamsMuxSeqConfig\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"ctrlSapmParamsSeqConfig\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"ctrlSapmMuxParamsSeqConfig\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"daiStartupSeqConfig\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"daiParamsSeqConfig\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"daiTriggerSeqConfig\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"controlsConfig\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"sapmComponent\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"sapmConfig\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn AudioFillConfigData(
    mut device: *const HdfDeviceObject,
    mut configData: *mut AudioConfigData,
) -> int32_t {
    let mut node: *const DeviceResourceNode = 0 as *const DeviceResourceNode;
    let mut drsOps: *mut DeviceResourceIface = 0 as *mut DeviceResourceIface;
    if device.is_null() || configData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input para check error\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioFillConfigData\0"))
                .as_ptr(),
            92 as core::ffi::c_int,
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
            b"[%s][line:%d]: drs node is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioFillConfigData\0"))
                .as_ptr(),
            98 as core::ffi::c_int,
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
            b"[%s][line:%d]: AudioFillConfigData: invalid drs ops fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioFillConfigData\0"))
                .as_ptr(),
            103 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    ((*drsOps).GetString)
        .expect(
            "non-null function pointer",
        )(
        node,
        b"serviceName\0" as *const u8 as *const core::ffi::c_char,
        &mut (*configData).cardServiceName,
        0 as *const core::ffi::c_char,
    );
    ((*drsOps).GetString)
        .expect(
            "non-null function pointer",
        )(
        node,
        b"codecName\0" as *const u8 as *const core::ffi::c_char,
        &mut (*configData).codecName,
        0 as *const core::ffi::c_char,
    );
    ((*drsOps).GetString)
        .expect(
            "non-null function pointer",
        )(
        node,
        b"platformName\0" as *const u8 as *const core::ffi::c_char,
        &mut (*configData).platformName,
        0 as *const core::ffi::c_char,
    );
    ((*drsOps).GetString)
        .expect(
            "non-null function pointer",
        )(
        node,
        b"cpuDaiName\0" as *const u8 as *const core::ffi::c_char,
        &mut (*configData).cpuDaiName,
        0 as *const core::ffi::c_char,
    );
    ((*drsOps).GetString)
        .expect(
            "non-null function pointer",
        )(
        node,
        b"codecDaiName\0" as *const u8 as *const core::ffi::c_char,
        &mut (*configData).codecDaiName,
        0 as *const core::ffi::c_char,
    );
    ((*drsOps).GetString)
        .expect(
            "non-null function pointer",
        )(
        node,
        b"dspName\0" as *const u8 as *const core::ffi::c_char,
        &mut (*configData).dspName,
        0 as *const core::ffi::c_char,
    );
    ((*drsOps).GetString)
        .expect(
            "non-null function pointer",
        )(
        node,
        b"dspDaiName\0" as *const u8 as *const core::ffi::c_char,
        &mut (*configData).dspDaiName,
        0 as *const core::ffi::c_char,
    );
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        LOG_DOMAIN as core::ffi::c_uint,
        b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
        b"[%s][line:%d]: cardServiceName = %s\0" as *const u8
            as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 20],
            [core::ffi::c_char; 20],
        >(*b"AudioFillConfigData\0"))
            .as_ptr(),
        115 as core::ffi::c_int,
        (*configData).cardServiceName,
    );
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        LOG_DOMAIN as core::ffi::c_uint,
        b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
        b"[%s][line:%d]: codecName       = %s, codecDaiName = %s\0" as *const u8
            as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 20],
            [core::ffi::c_char; 20],
        >(*b"AudioFillConfigData\0"))
            .as_ptr(),
        116 as core::ffi::c_int,
        (*configData).codecName,
        (*configData).codecDaiName,
    );
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        LOG_DOMAIN as core::ffi::c_uint,
        b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
        b"[%s][line:%d]: platformName    = %s, cpuDaiNamei = %s\0" as *const u8
            as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 20],
            [core::ffi::c_char; 20],
        >(*b"AudioFillConfigData\0"))
            .as_ptr(),
        117 as core::ffi::c_int,
        (*configData).platformName,
        (*configData).cpuDaiName,
    );
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        LOG_DOMAIN as core::ffi::c_uint,
        b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
        b"[%s][line:%d]: dspName         = %s, dspDaiName = %s\0" as *const u8
            as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 20],
            [core::ffi::c_char; 20],
        >(*b"AudioFillConfigData\0"))
            .as_ptr(),
        118 as core::ffi::c_int,
        (*configData).dspName,
        (*configData).dspDaiName,
    );
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn GetAudioRegGroupNameIndex(
    mut name: *const core::ffi::c_char,
) -> uint32_t {
    let mut index: uint32_t = 0;
    if name.is_null() {
        return AUDIO_GROUP_MAX as core::ffi::c_int as uint32_t;
    }
    index = 0 as uint32_t;
    while index < AUDIO_GROUP_MAX as core::ffi::c_int as core::ffi::c_uint {
        if !(g_audioRegGroupName[index as usize]).is_null()
            && strcmp(name, g_audioRegGroupName[index as usize]) == 0 as core::ffi::c_int
        {
            break;
        }
        index = index.wrapping_add(1);
    }
    return index;
}
unsafe extern "C" fn GetRegArray(
    mut parser: *const DeviceResourceIface,
    mut regNode: *const DeviceResourceNode,
    mut group: *mut AudioRegCfgGroupNode,
    mut indexMax: uint32_t,
) -> *mut uint32_t {
    let mut ret: int32_t = 0;
    let mut index: int32_t = 0;
    let mut num: int32_t = 0;
    let mut buf: *mut uint32_t = 0 as *mut uint32_t;
    if group.is_null() || parser.is_null() || regNode.is_null()
        || indexMax == 0 as core::ffi::c_uint
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input para check error\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 12],
                [core::ffi::c_char; 12],
            >(*b"GetRegArray\0"))
                .as_ptr(),
            148 as core::ffi::c_int,
        );
        return 0 as *mut uint32_t;
    }
    index = (*group).groupIndex as int32_t;
    if index >= AUDIO_GROUP_MAX as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input indexMax=%d error\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 12],
                [core::ffi::c_char; 12],
            >(*b"GetRegArray\0"))
                .as_ptr(),
            154 as core::ffi::c_int,
            index,
        );
        return 0 as *mut uint32_t;
    }
    num = ((*parser).GetElemNum)
        .expect(
            "non-null function pointer",
        )(regNode, g_audioRegGroupName[index as usize]);
    if num <= 0 as core::ffi::c_int || num > AUDIO_CONFIG_MAX_ITEM {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: parser %s element num failed\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 12],
                [core::ffi::c_char; 12],
            >(*b"GetRegArray\0"))
                .as_ptr(),
            160 as core::ffi::c_int,
            g_audioRegGroupName[index as usize],
        );
        return 0 as *mut uint32_t;
    }
    (*group).itemNum = (num as uint32_t).wrapping_div(indexMax) as uint8_t;
    buf = OsalMemCalloc(
        (::core::mem::size_of::<uint32_t>() as size_t).wrapping_mul(num as size_t),
    ) as *mut uint32_t;
    if buf.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: malloc reg array buf failed!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 12],
                [core::ffi::c_char; 12],
            >(*b"GetRegArray\0"))
                .as_ptr(),
            168 as core::ffi::c_int,
        );
        return 0 as *mut uint32_t;
    }
    ret = ((*parser).GetUint32Array)
        .expect(
            "non-null function pointer",
        )(
        regNode,
        g_audioRegGroupName[index as usize],
        buf,
        num as uint32_t,
        0 as uint32_t,
    );
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: parser %s reg array failed\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 12],
                [core::ffi::c_char; 12],
            >(*b"GetRegArray\0"))
                .as_ptr(),
            174 as core::ffi::c_int,
            g_audioRegGroupName[index as usize],
        );
        OsalMemFree(buf as *mut core::ffi::c_void);
        return 0 as *mut uint32_t;
    }
    return buf;
}
unsafe extern "C" fn ParseAudioRegItem(
    mut parser: *const DeviceResourceIface,
    mut regNode: *const DeviceResourceNode,
    mut group: *mut AudioRegCfgGroupNode,
) -> int32_t {
    let mut step: int32_t = 0;
    let mut index: int32_t = 0;
    let mut buf: *mut int32_t = 0 as *mut int32_t;
    if group.is_null() || parser.is_null() || regNode.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input para check error\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"ParseAudioRegItem\0"))
                .as_ptr(),
            188 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    buf = GetRegArray(
        parser,
        regNode,
        group,
        AUDIO_REG_CFG_INDEX_MAX as core::ffi::c_int as uint32_t,
    ) as *mut int32_t;
    if buf.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: malloc reg array buf failed!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"ParseAudioRegItem\0"))
                .as_ptr(),
            194 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*group).regCfgItem = OsalMemCalloc(
        ((*group).itemNum as size_t)
            .wrapping_mul(::core::mem::size_of::<AudioMixerControl>() as size_t),
    ) as *mut AudioMixerControl;
    if ((*group).regCfgItem).is_null() {
        OsalMemFree(buf as *mut core::ffi::c_void);
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: malloc audio reg config item failed!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"ParseAudioRegItem\0"))
                .as_ptr(),
            202 as core::ffi::c_int,
        );
        return HDF_ERR_MALLOC_FAIL as core::ffi::c_int as int32_t;
    }
    index = 0 as core::ffi::c_int as int32_t;
    while index < (*group).itemNum as core::ffi::c_int {
        step = AUDIO_REG_CFG_INDEX_MAX as core::ffi::c_int as int32_t * index;
        (*((*group).regCfgItem).offset(index as isize)).reg = *buf
            .offset(
                (step as core::ffi::c_int + AUDIO_REG_CFG_REG_INDEX as core::ffi::c_int)
                    as isize,
            ) as uint32_t;
        (*((*group).regCfgItem).offset(index as isize)).rreg = *buf
            .offset(
                (step as core::ffi::c_int + AUDIO_REG_CFG_RREG_INDEX as core::ffi::c_int)
                    as isize,
            ) as uint32_t;
        (*((*group).regCfgItem).offset(index as isize)).shift = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_REG_CFG_SHIFT_INDEX as core::ffi::c_int) as isize,
            ) as uint32_t;
        (*((*group).regCfgItem).offset(index as isize)).rshift = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_REG_CFG_RSHIFT_INDEX as core::ffi::c_int) as isize,
            ) as uint32_t;
        (*((*group).regCfgItem).offset(index as isize)).min = *buf
            .offset(
                (step as core::ffi::c_int + AUDIO_REG_CFG_MIN_INDEX as core::ffi::c_int)
                    as isize,
            ) as uint32_t;
        (*((*group).regCfgItem).offset(index as isize)).max = *buf
            .offset(
                (step as core::ffi::c_int + AUDIO_REG_CFG_MAX_INDEX as core::ffi::c_int)
                    as isize,
            ) as uint32_t;
        (*((*group).regCfgItem).offset(index as isize)).mask = *buf
            .offset(
                (step as core::ffi::c_int + AUDIO_REG_CFG_MASK_INDEX as core::ffi::c_int)
                    as isize,
            ) as uint32_t;
        (*((*group).regCfgItem).offset(index as isize)).invert = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_REG_CFG_INVERT_INDEX as core::ffi::c_int) as isize,
            ) as uint32_t;
        (*((*group).regCfgItem).offset(index as isize)).value = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_REG_CFG_VALUE_INDEX as core::ffi::c_int) as isize,
            ) as uint32_t;
        index += 1;
    }
    OsalMemFree(buf as *mut core::ffi::c_void);
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn ParseAudioEnumRegItem(
    mut parser: *const DeviceResourceIface,
    mut regNode: *const DeviceResourceNode,
    mut group: *mut AudioRegCfgGroupNode,
) -> int32_t {
    let mut step: int32_t = 0;
    let mut index: int32_t = 0;
    let mut buf: *mut int32_t = 0 as *mut int32_t;
    if group.is_null() || parser.is_null() || regNode.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input para check error\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"ParseAudioEnumRegItem\0"))
                .as_ptr(),
            232 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    buf = GetRegArray(
        parser,
        regNode,
        group,
        AUDIO_ENUM_REG_CFG_INDEX_MAX as core::ffi::c_int as uint32_t,
    ) as *mut int32_t;
    if buf.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: malloc reg array buf failed!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"ParseAudioEnumRegItem\0"))
                .as_ptr(),
            238 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*group).regEnumCfgItem = OsalMemCalloc(
        ((*group).itemNum as size_t)
            .wrapping_mul(::core::mem::size_of::<AudioEnumCtrlConfig>() as size_t),
    ) as *mut AudioEnumCtrlConfig;
    if ((*group).regEnumCfgItem).is_null() {
        OsalMemFree(buf as *mut core::ffi::c_void);
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: malloc audio Enum reg config item is failed!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"ParseAudioEnumRegItem\0"))
                .as_ptr(),
            246 as core::ffi::c_int,
        );
        return HDF_ERR_MALLOC_FAIL as core::ffi::c_int as int32_t;
    }
    index = 0 as core::ffi::c_int as int32_t;
    while index < (*group).itemNum as core::ffi::c_int {
        step = AUDIO_ENUM_REG_CFG_INDEX_MAX as core::ffi::c_int as int32_t * index;
        (*((*group).regEnumCfgItem).offset(index as isize)).reg = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_ENUM_REG_CFG_REG_INDEX as core::ffi::c_int) as isize,
            ) as uint32_t;
        (*((*group).regEnumCfgItem).offset(index as isize)).reg2 = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_ENUM_REG_CFG_RREG_INDEX as core::ffi::c_int) as isize,
            ) as uint32_t;
        (*((*group).regEnumCfgItem).offset(index as isize)).shiftLeft = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_ENUM_REG_CFG_SHIFT_INDEX as core::ffi::c_int) as isize,
            ) as uint8_t;
        (*((*group).regEnumCfgItem).offset(index as isize)).shiftRight = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_ENUM_REG_CFG_RSHIFT_INDEX as core::ffi::c_int) as isize,
            ) as uint8_t;
        (*((*group).regEnumCfgItem).offset(index as isize)).max = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_ENUM_REG_CFG_MAX_INDEX as core::ffi::c_int) as isize,
            ) as uint32_t;
        (*((*group).regEnumCfgItem).offset(index as isize)).mask = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_ENUM_REG_CFG_MASK_INDEX as core::ffi::c_int) as isize,
            ) as uint32_t;
        (*((*group).regEnumCfgItem).offset(index as isize)).texts = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_ENUM_REG_CFG_TEXTS_INDEX as core::ffi::c_int) as isize,
            ) as uint32_t;
        (*((*group).regEnumCfgItem).offset(index as isize)).values = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_ENUM_REG_CFG_VALUE_INDEX as core::ffi::c_int) as isize,
            ) as uint32_t;
        (*((*group).regEnumCfgItem).offset(index as isize)).sapm = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_ENUM_REG_CFG_SAPM_INDEX as core::ffi::c_int) as isize,
            ) as uint32_t;
        index += 1;
    }
    OsalMemFree(buf as *mut core::ffi::c_void);
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn ParseAudioSapmItem(
    mut parser: *const DeviceResourceIface,
    mut regNode: *const DeviceResourceNode,
    mut group: *mut AudioRegCfgGroupNode,
) -> int32_t {
    let mut step: int32_t = 0;
    let mut index: int32_t = 0;
    let mut buf: *mut uint32_t = 0 as *mut uint32_t;
    if group.is_null() || parser.is_null() || regNode.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input para check error\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"ParseAudioSapmItem\0"))
                .as_ptr(),
            275 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    buf = GetRegArray(
        parser,
        regNode,
        group,
        AUDIO_SAPM_COMP_INDEX_MAX as core::ffi::c_int as uint32_t,
    );
    if buf.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: malloc reg array buf failed!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"ParseAudioSapmItem\0"))
                .as_ptr(),
            281 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*group).sapmCompItem = OsalMemCalloc(
        ((*group).itemNum as size_t)
            .wrapping_mul(::core::mem::size_of::<AudioSapmCtrlConfig>() as size_t),
    ) as *mut AudioSapmCtrlConfig;
    if ((*group).sapmCompItem).is_null() {
        OsalMemFree(buf as *mut core::ffi::c_void);
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: malloc audio reg config item failed!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"ParseAudioSapmItem\0"))
                .as_ptr(),
            289 as core::ffi::c_int,
        );
        return HDF_ERR_MALLOC_FAIL as core::ffi::c_int as int32_t;
    }
    index = 0 as core::ffi::c_int as int32_t;
    while index < (*group).itemNum as core::ffi::c_int {
        step = AUDIO_SAPM_COMP_INDEX_MAX as core::ffi::c_int as int32_t * index;
        (*((*group).sapmCompItem).offset(index as isize)).sapmType = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_SAPM_COMP_INDEX_TYPE as core::ffi::c_int) as isize,
            ) as uint8_t;
        (*((*group).sapmCompItem).offset(index as isize)).compNameIndex = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_SAPM_COMP_INDEX_NAME as core::ffi::c_int) as isize,
            ) as uint16_t;
        (*((*group).sapmCompItem).offset(index as isize)).reg = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_SAPM_COMP_INDEX_REG as core::ffi::c_int) as isize,
            );
        (*((*group).sapmCompItem).offset(index as isize)).mask = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_SAPM_COMP_INDEX_MASK as core::ffi::c_int) as isize,
            );
        (*((*group).sapmCompItem).offset(index as isize)).shift = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_SAPM_COMP_INDEX_SHIFT as core::ffi::c_int) as isize,
            ) as uint8_t;
        (*((*group).sapmCompItem).offset(index as isize)).invert = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_SAPM_COMP_INDEX_INVERT as core::ffi::c_int) as isize,
            ) as uint8_t;
        (*((*group).sapmCompItem).offset(index as isize)).kcontrolNews = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_SAPM_COMP_INDEX_KCTL as core::ffi::c_int) as isize,
            );
        (*((*group).sapmCompItem).offset(index as isize)).kcontrolsNum = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_SAPM_COMP_INDEX_KCTLNUM as core::ffi::c_int) as isize,
            );
        index += 1;
    }
    OsalMemFree(buf as *mut core::ffi::c_void);
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn ParseAudioCtrlItem(
    mut parser: *const DeviceResourceIface,
    mut regNode: *const DeviceResourceNode,
    mut group: *mut AudioRegCfgGroupNode,
) -> int32_t {
    let mut step: int32_t = 0;
    let mut index: int32_t = 0;
    let mut buf: *mut uint32_t = 0 as *mut uint32_t;
    if parser.is_null() || regNode.is_null() || group.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input para check error\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"ParseAudioCtrlItem\0"))
                .as_ptr(),
            317 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    buf = GetRegArray(
        parser,
        regNode,
        group,
        AUDIO_CTRL_CFG_INDEX_MAX as core::ffi::c_int as uint32_t,
    );
    if buf.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: malloc reg array buf failed!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"ParseAudioCtrlItem\0"))
                .as_ptr(),
            323 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*group).ctrlCfgItem = OsalMemCalloc(
        ((*group).itemNum as size_t)
            .wrapping_mul(::core::mem::size_of::<AudioControlConfig>() as size_t),
    ) as *mut AudioControlConfig;
    if ((*group).ctrlCfgItem).is_null() {
        OsalMemFree(buf as *mut core::ffi::c_void);
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: malloc audio ctrl config item failed!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"ParseAudioCtrlItem\0"))
                .as_ptr(),
            331 as core::ffi::c_int,
        );
        return HDF_ERR_MALLOC_FAIL as core::ffi::c_int as int32_t;
    }
    index = 0 as core::ffi::c_int as int32_t;
    while index < (*group).itemNum as core::ffi::c_int {
        step = AUDIO_CTRL_CFG_INDEX_MAX as core::ffi::c_int as int32_t * index;
        (*((*group).ctrlCfgItem).offset(index as isize)).arrayIndex = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_CTRL_CFG_INDEX_INDEX as core::ffi::c_int) as isize,
            ) as uint16_t;
        (*((*group).ctrlCfgItem).offset(index as isize)).iface = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_CTRL_CFG_IFACE_INDEX as core::ffi::c_int) as isize,
            ) as uint16_t;
        (*((*group).ctrlCfgItem).offset(index as isize)).type_0 = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_CTRL_CFG_TYPE_INDEX as core::ffi::c_int) as isize,
            ) as uint16_t;
        (*((*group).ctrlCfgItem).offset(index as isize)).enable = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_CTRL_CFG_ENABLE_INDEX as core::ffi::c_int) as isize,
            ) as uint8_t;
        index += 1;
    }
    OsalMemFree(buf as *mut core::ffi::c_void);
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn ParseAudioAddrItem(
    mut parser: *const DeviceResourceIface,
    mut regNode: *const DeviceResourceNode,
    mut group: *mut AudioRegCfgGroupNode,
) -> int32_t {
    let mut step: int32_t = 0;
    let mut index: int32_t = 0;
    let mut buf: *mut uint32_t = 0 as *mut uint32_t;
    if parser.is_null() || regNode.is_null() || group.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input para check error.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"ParseAudioAddrItem\0"))
                .as_ptr(),
            355 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    buf = GetRegArray(
        parser,
        regNode,
        group,
        AUDIO_ADDR_CFG_INDEX_MAX as core::ffi::c_int as uint32_t,
    );
    if buf.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: malloc reg array buf failed!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"ParseAudioAddrItem\0"))
                .as_ptr(),
            361 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*group).addrCfgItem = OsalMemCalloc(
        ((*group).itemNum as size_t)
            .wrapping_mul(::core::mem::size_of::<AudioAddrConfig>() as size_t),
    ) as *mut AudioAddrConfig;
    if ((*group).addrCfgItem).is_null() {
        OsalMemFree(buf as *mut core::ffi::c_void);
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: malloc audio addr config item failed!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"ParseAudioAddrItem\0"))
                .as_ptr(),
            368 as core::ffi::c_int,
        );
        return HDF_ERR_MALLOC_FAIL as core::ffi::c_int as int32_t;
    }
    index = 0 as core::ffi::c_int as int32_t;
    while index < (*group).itemNum as core::ffi::c_int {
        step = AUDIO_ADDR_CFG_INDEX_MAX as core::ffi::c_int as int32_t * index;
        (*((*group).addrCfgItem).offset(index as isize)).addr = *buf
            .offset(
                (step as core::ffi::c_int + AUDIO_ADDR_CFG_REG_INDEX as core::ffi::c_int)
                    as isize,
            );
        (*((*group).addrCfgItem).offset(index as isize)).value = *buf
            .offset(
                (step as core::ffi::c_int
                    + AUDIO_ADDR_CFG_VALUE_INDEX as core::ffi::c_int) as isize,
            );
        index += 1;
    }
    OsalMemFree(buf as *mut core::ffi::c_void);
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn ParseAudioRegGroup(
    mut parser: *const DeviceResourceIface,
    mut regCfgNode: *const DeviceResourceNode,
    mut groupNode: *mut *mut AudioRegCfgGroupNode,
    mut index: uint32_t,
) -> int32_t {
    let mut ret: int32_t = HDF_FAILURE as core::ffi::c_int as int32_t;
    let mut group: *mut AudioRegCfgGroupNode = 0 as *mut AudioRegCfgGroupNode;
    if parser.is_null() || regCfgNode.is_null() || groupNode.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input para check error\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"ParseAudioRegGroup\0"))
                .as_ptr(),
            388 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    group = OsalMemCalloc(::core::mem::size_of::<AudioRegCfgGroupNode>() as size_t)
        as *mut AudioRegCfgGroupNode;
    if group.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: malloc audio reg config group failed\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"ParseAudioRegGroup\0"))
                .as_ptr(),
            394 as core::ffi::c_int,
        );
        return HDF_ERR_MALLOC_FAIL as core::ffi::c_int as int32_t;
    }
    *groupNode = group;
    (**groupNode).groupIndex = index as AudioRegOpsType;
    match index {
        9 | 11 => {
            ret = ParseAudioCtrlItem(parser, regCfgNode, group);
        }
        0 | 1 => {
            ret = ParseAudioAddrItem(parser, regCfgNode, group);
        }
        7 | 8 | 2 | 4 | 6 => {
            ret = ParseAudioRegItem(parser, regCfgNode, group);
        }
        3 | 5 => {
            ret = ParseAudioEnumRegItem(parser, regCfgNode, group);
        }
        10 => {
            ret = ParseAudioSapmItem(parser, regCfgNode, group);
        }
        _ => {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: parse audio config index = %u not found!\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 19],
                    [core::ffi::c_char; 19],
                >(*b"ParseAudioRegGroup\0"))
                    .as_ptr(),
                424 as core::ffi::c_int,
                index,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    }
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: parse audio config item failed!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"ParseAudioRegGroup\0"))
                .as_ptr(),
            429 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn ReleaseAudioAllRegConfig(mut config: *mut AudioRegCfgData) {
    let mut index: int32_t = 0;
    if config.is_null() {
        return;
    }
    index = 0 as core::ffi::c_int as int32_t;
    while index < AUDIO_GROUP_MAX as core::ffi::c_int {
        if !((*config).audioRegParams[index as usize]).is_null() {
            if !((*(*config).audioRegParams[index as usize]).regCfgItem).is_null() {
                OsalMemFree(
                    (*(*config).audioRegParams[index as usize]).regCfgItem
                        as *mut core::ffi::c_void,
                );
                (*(*config).audioRegParams[index as usize]).regCfgItem = 0
                    as *mut AudioMixerControl;
            }
            OsalMemFree(
                (*config).audioRegParams[index as usize] as *mut core::ffi::c_void,
            );
            (*config).audioRegParams[index as usize] = 0 as *mut AudioRegCfgGroupNode;
        }
        index += 1;
    }
}
unsafe extern "C" fn ParseAudioAttr(
    mut parser: *const DeviceResourceIface,
    mut attrNode: *const DeviceResourceNode,
    mut config: *mut AudioIdInfo,
) -> int32_t {
    let mut ret: int32_t = 0;
    ret = ((*parser).GetString)
        .expect(
            "non-null function pointer",
        )(
        attrNode,
        b"chipName\0" as *const u8 as *const core::ffi::c_char,
        &mut (*config).chipName,
        0 as *const core::ffi::c_char,
    );
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: parser chipName reg audioIdInfo failed!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"ParseAudioAttr\0"))
                .as_ptr(),
            461 as core::ffi::c_int,
        );
        return HDF_SUCCESS as core::ffi::c_int as int32_t;
    }
    ret = ((*parser).GetUint32)
        .expect(
            "non-null function pointer",
        )(
        attrNode,
        b"chipIdRegister\0" as *const u8 as *const core::ffi::c_char,
        &mut (*config).chipIdRegister,
        0 as uint32_t,
    );
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: parser chipIdRegister reg audioIdInfo failed!\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"ParseAudioAttr\0"))
                .as_ptr(),
            467 as core::ffi::c_int,
        );
        return HDF_SUCCESS as core::ffi::c_int as int32_t;
    }
    ret = ((*parser).GetUint32)
        .expect(
            "non-null function pointer",
        )(
        attrNode,
        b"chipIdSize\0" as *const u8 as *const core::ffi::c_char,
        &mut (*config).chipIdSize,
        0 as uint32_t,
    );
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: parser chipIdSize reg audioIdInfo failed!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"ParseAudioAttr\0"))
                .as_ptr(),
            473 as core::ffi::c_int,
        );
        return HDF_SUCCESS as core::ffi::c_int as int32_t;
    }
    return ret;
}
unsafe extern "C" fn AudioSetPortInfoConfig(
    mut buf: *const uint64_t,
    mut info: *mut AudioPcmStream,
) -> int32_t {
    let mut ret: int32_t = 0;
    ret = memcpy_s(
        info as *mut core::ffi::c_void,
        ::core::mem::size_of::<AudioPcmStream>() as size_t,
        buf as *const core::ffi::c_void,
        (::core::mem::size_of::<uint64_t>() as size_t)
            .wrapping_mul(PORT_INFO_LIST_LENGHT as size_t),
    ) as int32_t;
    if ret != 0 as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: memcpy_s error ret = %d!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioSetPortInfoConfig\0"))
                .as_ptr(),
            485 as core::ffi::c_int,
            ret,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioSetPortInfoConfigStub(
    mut buf: *const uint64_t,
    mut configData: *mut AudioPortInfo,
) -> int32_t {
    match *buf.offset(0 as core::ffi::c_int as isize) {
        1 => return AudioSetPortInfoConfig(buf, &mut (*configData).render),
        2 => return AudioSetPortInfoConfig(buf, &mut (*configData).capture),
        _ => {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: portDirection = %llu element num failed\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 27],
                    [core::ffi::c_char; 27],
                >(*b"AudioSetPortInfoConfigStub\0"))
                    .as_ptr(),
                500 as core::ffi::c_int,
                *buf.offset(0 as core::ffi::c_int as isize),
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    };
}
unsafe extern "C" fn AudioGetPortInfoConfig(
    mut drsOps: *mut DeviceResourceIface,
    mut device: *const HdfDeviceObject,
    mut configData: *mut AudioPortInfo,
) -> int32_t {
    let mut num: uint32_t = 0;
    let mut ret: uint32_t = 0;
    let mut buf: *mut uint64_t = 0 as *mut uint64_t;
    num = ((*drsOps).GetElemNum)
        .expect(
            "non-null function pointer",
        )((*device).property as *const DeviceResourceNode, HW_INFO.as_ptr()) as uint32_t;
    if num == 0 as core::ffi::c_uint || num > AUDIO_CONFIG_MAX_ITEM as core::ffi::c_uint
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: parser %s element num failed\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioGetPortInfoConfig\0"))
                .as_ptr(),
            514 as core::ffi::c_int,
            b"hwInfo\0" as *const u8 as *const core::ffi::c_char,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    buf = OsalMemCalloc(
        (::core::mem::size_of::<uint64_t>() as size_t).wrapping_mul(num as size_t),
    ) as *mut uint64_t;
    if buf.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: malloc reg array buf failed!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioGetPortInfoConfig\0"))
                .as_ptr(),
            520 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    ret = ((*drsOps).GetUint64Array)
        .expect(
            "non-null function pointer",
        )(
        (*device).property as *const DeviceResourceNode,
        HW_INFO.as_ptr(),
        buf,
        num,
        0 as uint64_t,
    ) as uint32_t;
    if ret != HDF_SUCCESS as core::ffi::c_int as core::ffi::c_uint {
        OsalMemFree(buf as *mut core::ffi::c_void);
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: GetChildNode: Read portCfgNode fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioGetPortInfoConfig\0"))
                .as_ptr(),
            527 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    memset_s(
        configData as *mut core::ffi::c_void,
        ::core::mem::size_of::<AudioPortInfo>() as size_t,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<AudioPortInfo>() as size_t,
    );
    match num {
        12 => {
            AudioSetPortInfoConfigStub(buf, configData);
        }
        24 => {
            AudioSetPortInfoConfigStub(buf, configData);
            AudioSetPortInfoConfigStub(
                buf.offset(PORT_INFO_LIST_LENGHT as isize),
                configData,
            );
        }
        _ => {
            OsalMemFree(buf as *mut core::ffi::c_void);
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: configData->portDirection num is not matched! num = %d\0"
                    as *const u8 as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 23],
                    [core::ffi::c_char; 23],
                >(*b"AudioGetPortInfoConfig\0"))
                    .as_ptr(),
                543 as core::ffi::c_int,
                num,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    }
    OsalMemFree(buf as *mut core::ffi::c_void);
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioGetPortConfig(
    mut device: *const HdfDeviceObject,
    mut configData: *mut AudioPortInfo,
) -> int32_t {
    let mut ret: uint32_t = 0;
    let mut drsOps: *mut DeviceResourceIface = 0 as *mut DeviceResourceIface;
    if device.is_null() || ((*device).property).is_null() || configData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input para check error\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioGetPortConfig\0"))
                .as_ptr(),
            557 as core::ffi::c_int,
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
            b"[%s][line:%d]: AudioGetPortConfig: invalid drs ops fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioGetPortConfig\0"))
                .as_ptr(),
            563 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    ret = AudioGetPortInfoConfig(drsOps, device, configData) as uint32_t;
    if ret != HDF_SUCCESS as core::ffi::c_int as core::ffi::c_uint {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: parser chipIdRegister reg audioIdInfo failed!\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioGetPortConfig\0"))
                .as_ptr(),
            569 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioGetRegConfig(
    mut device: *const HdfDeviceObject,
    mut configData: *mut AudioRegCfgData,
) -> int32_t {
    let mut index: uint16_t = 0;
    let mut regCfgNode: *const DeviceResourceNode = 0 as *const DeviceResourceNode;
    let mut regAttr: *const DeviceResourceAttr = 0 as *const DeviceResourceAttr;
    let mut idNode: *const DeviceResourceNode = 0 as *const DeviceResourceNode;
    let mut drsOps: *mut DeviceResourceIface = 0 as *mut DeviceResourceIface;
    if device.is_null() || ((*device).property).is_null() || configData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input para check error\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"AudioGetRegConfig\0"))
                .as_ptr(),
            584 as core::ffi::c_int,
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
            b"[%s][line:%d]: AudioFillConfigData: invalid drs ops fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"AudioGetRegConfig\0"))
                .as_ptr(),
            590 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    idNode = ((*drsOps).GetChildNode)
        .expect(
            "non-null function pointer",
        )(
        (*device).property as *const DeviceResourceNode,
        b"idInfo\0" as *const u8 as *const core::ffi::c_char,
    );
    if !idNode.is_null() {
        if ParseAudioAttr(drsOps, idNode, &mut (*configData).audioIdInfo)
            != HDF_SUCCESS as core::ffi::c_int
        {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: audio reg node attr is null\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 18],
                    [core::ffi::c_char; 18],
                >(*b"AudioGetRegConfig\0"))
                    .as_ptr(),
                597 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    }
    regCfgNode = ((*drsOps).GetChildNode)
        .expect(
            "non-null function pointer",
        )(
        (*device).property as *const DeviceResourceNode,
        b"regConfig\0" as *const u8 as *const core::ffi::c_char,
    );
    if regCfgNode.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: AudioGetRegConfig: Read audioRegConfig fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"AudioGetRegConfig\0"))
                .as_ptr(),
            604 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    regAttr = (*regCfgNode).attrData;
    while !regAttr.is_null() {
        if regAttr.is_null() || ((*regAttr).name).is_null() {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: audio reg node attr is null\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 18],
                    [core::ffi::c_char; 18],
                >(*b"AudioGetRegConfig\0"))
                    .as_ptr(),
                610 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
        index = GetAudioRegGroupNameIndex((*regAttr).name) as uint16_t;
        if !(index as core::ffi::c_int >= AUDIO_GROUP_MAX as core::ffi::c_int) {
            if ParseAudioRegGroup(
                drsOps,
                regCfgNode,
                &mut *((*configData).audioRegParams).as_mut_ptr().offset(index as isize),
                index as uint32_t,
            ) != HDF_SUCCESS as core::ffi::c_int
            {
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    LOG_DOMAIN as core::ffi::c_uint,
                    b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s][line:%d]: parse audio register group failed\0" as *const u8
                        as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 18],
                        [core::ffi::c_char; 18],
                    >(*b"AudioGetRegConfig\0"))
                        .as_ptr(),
                    620 as core::ffi::c_int,
                );
                ReleaseAudioAllRegConfig(configData);
                return HDF_FAILURE as core::ffi::c_int as int32_t;
            }
        }
        regAttr = (*regAttr).next;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
pub const LOG_DOMAIN: core::ffi::c_int = 0xd002510 as core::ffi::c_int;
