extern "C" {
    pub type HdfSBuf;
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
    fn snprintf_s(
        strDest: *mut core::ffi::c_char,
        destMax: size_t,
        count: size_t,
        format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn AudioSampPowerUp(card: *const AudioCard) -> int32_t;
    fn AudioSampSetPowerMonitor(
        card: *mut AudioCard,
        powerMonitorState: bool,
    ) -> int32_t;
    fn AudioDmaBufAlloc(data: *mut PlatformData, streamType: AudioStreamType) -> int32_t;
    fn AudioDmaBufFree(data: *mut PlatformData, streamType: AudioStreamType) -> int32_t;
    fn AudioDmaRequestChannel(
        data: *mut PlatformData,
        streamType: AudioStreamType,
    ) -> int32_t;
    fn AudioDmaConfigChannel(
        data: *mut PlatformData,
        streamType: AudioStreamType,
    ) -> int32_t;
    fn AudioDmaPrep(data: *mut PlatformData, streamType: AudioStreamType) -> int32_t;
    fn AudioDmaSubmit(data: *mut PlatformData, streamType: AudioStreamType) -> int32_t;
    fn AudioDmaPending(data: *mut PlatformData, streamType: AudioStreamType) -> int32_t;
    fn AudioDmaPause(data: *mut PlatformData, streamType: AudioStreamType) -> int32_t;
    fn AudioDmaResume(data: *mut PlatformData, streamType: AudioStreamType) -> int32_t;
    fn AudioDmaPointer(
        data: *mut PlatformData,
        streamType: AudioStreamType,
        pointer: *mut uint32_t,
    ) -> int32_t;
    fn HdfDeviceObjectSetServInfo(
        dev: *mut HdfDeviceObject,
        info: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn HdfDeviceObjectUpdate(dev: *mut HdfDeviceObject) -> core::ffi::c_int;
    fn OsalMSleep(ms: uint32_t);
    fn LOS_ArchCopyFromUser(
        dst: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        len: size_t,
    ) -> size_t;
    fn LOS_ArchCopyToUser(
        dst: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        len: size_t,
    ) -> size_t;
}
pub type size_t = core::ffi::c_uint;
pub type uintptr_t = core::ffi::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DListHead {
    pub next: *mut DListHead,
    pub prev: *mut DListHead,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OsalMutex {
    pub realMutex: *mut core::ffi::c_void,
}
pub type errno_t = core::ffi::c_int;
pub type AudioFormat = core::ffi::c_uint;
pub const AUDIO_FORMAT_TYPE_G726: AudioFormat = 33554435;
pub const AUDIO_FORMAT_TYPE_G711U: AudioFormat = 33554434;
pub const AUDIO_FORMAT_TYPE_G711A: AudioFormat = 33554433;
pub const AUDIO_FORMAT_TYPE_AAC_HE_V2: AudioFormat = 16777222;
pub const AUDIO_FORMAT_TYPE_AAC_HE_V1: AudioFormat = 16777221;
pub const AUDIO_FORMAT_TYPE_AAC_ELD: AudioFormat = 16777220;
pub const AUDIO_FORMAT_TYPE_AAC_LD: AudioFormat = 16777219;
pub const AUDIO_FORMAT_TYPE_AAC_LC: AudioFormat = 16777218;
pub const AUDIO_FORMAT_TYPE_AAC_MAIN: AudioFormat = 16777217;
pub const AUDIO_FORMAT_TYPE_PCM_32_BIT: AudioFormat = 4;
pub const AUDIO_FORMAT_TYPE_PCM_24_BIT: AudioFormat = 3;
pub const AUDIO_FORMAT_TYPE_PCM_16_BIT: AudioFormat = 2;
pub const AUDIO_FORMAT_TYPE_PCM_8_BIT: AudioFormat = 1;
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
pub type AudioSapmTurnStandbyMode = core::ffi::c_uint;
pub const AUDIO_SAPM_TURN_STANDBY_BUTT: AudioSapmTurnStandbyMode = 2;
pub const AUDIO_SAPM_TURN_STANDBY_NOW: AudioSapmTurnStandbyMode = 1;
pub const AUDIO_SAPM_TURN_STANDBY_LATER: AudioSapmTurnStandbyMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioCard {
    pub device: *mut HdfDeviceObject,
    pub rtd: *mut AudioRuntimeDeivces,
    pub configData: AudioConfigData,
    pub sapmComponents: *const AudioSapmComponent,
    pub sapmComponentsNum: int32_t,
    pub sapmRoutes: *const AudioSapmRoute,
    pub sapmRoutesNum: int32_t,
    pub list: DListHead,
    pub controls: DListHead,
    pub components: DListHead,
    pub paths: DListHead,
    pub sapmDirty: DListHead,
    pub standbyMode: AudioSapmTurnStandbyMode,
    pub time: uint64_t,
    pub sapmSleepState: bool,
    pub sapmStandbyState: bool,
    pub sapmMonitorState: bool,
    pub sapmStandbyStartTimeFlag: bool,
    pub sapmSleepStartTimeFlag: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioSapmRoute {
    pub sink: *const core::ffi::c_char,
    pub control: *const core::ffi::c_char,
    pub source: *const core::ffi::c_char,
    pub Connected: Option<
        unsafe extern "C" fn(
            *mut AudioSapmComponent,
            *mut AudioSapmComponent,
        ) -> uint32_t,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioSapmComponent {
    pub sapmType: AudioSapmType,
    pub componentName: *mut core::ffi::c_char,
    pub streamName: *mut core::ffi::c_char,
    pub sapm: *mut AudioSapmContext,
    pub codec: *mut CodecDevice,
    pub platform: *mut PlatformDevice,
    pub reg: uint32_t,
    pub shift: uint8_t,
    pub invert: uint8_t,
    pub mask: uint32_t,
    pub connected: uint8_t,
    pub external: uint8_t,
    pub active: uint8_t,
    pub newPower: uint8_t,
    pub power: uint8_t,
    pub newCpt: uint8_t,
    pub eventFlags: uint16_t,
    pub Event: Option<
        unsafe extern "C" fn(
            *mut AudioSapmComponent,
            *mut AudioKcontrol,
            int32_t,
        ) -> int32_t,
    >,
    pub PowerCheck: Option<unsafe extern "C" fn(*const AudioSapmComponent) -> int32_t>,
    pub kcontrolsNum: int32_t,
    pub kcontrolNews: *mut AudioKcontrol,
    pub kcontrols: *mut *mut AudioKcontrol,
    pub list: DListHead,
    pub sources: DListHead,
    pub sinks: DListHead,
    pub powerList: DListHead,
    pub dirty: DListHead,
    pub PowerClockOp: Option<unsafe extern "C" fn(*mut AudioSapmComponent) -> int32_t>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioKcontrol {
    pub name: *mut core::ffi::c_char,
    pub iface: int32_t,
    pub Info: Option<
        unsafe extern "C" fn(*const AudioKcontrol, *mut AudioCtrlElemInfo) -> int32_t,
    >,
    pub Get: Option<
        unsafe extern "C" fn(*const AudioKcontrol, *mut AudioCtrlElemValue) -> int32_t,
    >,
    pub Set: Option<
        unsafe extern "C" fn(*const AudioKcontrol, *const AudioCtrlElemValue) -> int32_t,
    >,
    pub privateData: *mut core::ffi::c_void,
    pub pri: *mut core::ffi::c_void,
    pub privateValue: core::ffi::c_ulong,
    pub list: DListHead,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioCtrlElemValue {
    pub id: AudioCtrlElemId,
    pub value: [uint32_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioCtrlElemId {
    pub cardServiceName: *const core::ffi::c_char,
    pub iface: int32_t,
    pub itemName: *const core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioCtrlElemInfo {
    pub id: AudioCtrlElemId,
    pub count: uint32_t,
    pub type_0: int32_t,
    pub min: int32_t,
    pub max: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlatformDevice {
    pub devPlatformName: *const core::ffi::c_char,
    pub devData: *mut PlatformData,
    pub device: *mut HdfDeviceObject,
    pub list: DListHead,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlatformData {
    pub drvPlatformName: *const core::ffi::c_char,
    pub PlatformInit: Option<
        unsafe extern "C" fn(*const AudioCard, *const PlatformDevice) -> int32_t,
    >,
    pub ops: *mut AudioDmaOps,
    pub renderBufInfo: CircleBufInfo,
    pub captureBufInfo: CircleBufInfo,
    pub renderPcmInfo: PcmInfo,
    pub capturePcmInfo: PcmInfo,
    pub platformInitFlag: bool,
    pub mmapData: AudioMmapData,
    pub mmapLoopCount: uint32_t,
    pub dmaPrv: *mut core::ffi::c_void,
    pub regConfig: *mut AudioRegCfgData,
    pub regCfgGroup: *mut *mut AudioRegCfgGroupNode,
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
pub struct AudioControlConfig {
    pub arrayIndex: uint16_t,
    pub iface: uint16_t,
    pub type_0: uint16_t,
    pub enable: uint8_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioAddrConfig {
    pub addr: uint32_t,
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
pub struct AudioRegCfgData {
    pub audioIdInfo: AudioIdInfo,
    pub audioRegParams: [*mut AudioRegCfgGroupNode; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioIdInfo {
    pub chipName: *const core::ffi::c_char,
    pub chipIdRegister: uint32_t,
    pub chipIdSize: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioMmapData {
    pub memoryAddress: *mut core::ffi::c_void,
    pub memoryFd: int32_t,
    pub totalBufferFrames: int32_t,
    pub transferFrameSize: int32_t,
    pub isShareable: int32_t,
    pub offset: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PcmInfo {
    pub streamType: AudioStreamType,
    pub channels: uint32_t,
    pub rate: uint32_t,
    pub bitWidth: uint32_t,
    pub frameSize: uint32_t,
    pub isBigEndian: bool,
    pub isSignedData: bool,
    pub startThreshold: uint32_t,
    pub stopThreshold: uint32_t,
    pub silenceThreshold: uint32_t,
    pub totalStreamSize: uint32_t,
    pub interleaved: uint32_t,
}
pub type AudioStreamType = core::ffi::c_uint;
pub const AUDIO_RENDER_STREAM: AudioStreamType = 1;
pub const AUDIO_CAPTURE_STREAM: AudioStreamType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CircleBufInfo {
    pub cirBufSize: uint32_t,
    pub trafBufSize: uint32_t,
    pub period: uint32_t,
    pub periodSize: uint32_t,
    pub periodCount: uint32_t,
    pub phyAddr: core::ffi::c_ulong,
    pub virtAddr: *mut uint32_t,
    pub wbufOffSet: uint32_t,
    pub wptrOffSet: uint32_t,
    pub rbufOffSet: uint32_t,
    pub rptrOffSet: uint32_t,
    pub runStatus: PcmStatus,
    pub chnId: uint32_t,
    pub enable: uint32_t,
    pub buffMutex: OsalMutex,
    pub framesPosition: uint32_t,
    pub pointer: uint32_t,
    pub periodsMax: uint32_t,
    pub periodsMin: uint32_t,
    pub cirBufMax: uint32_t,
    pub curTrafSize: uint32_t,
    pub oneMsBytes: uint32_t,
    pub trafCompCount: uint32_t,
}
pub type PcmStatus = core::ffi::c_uint;
pub const PCM_START: PcmStatus = 2;
pub const PCM_PAUSE: PcmStatus = 1;
pub const PCM_STOP: PcmStatus = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioDmaOps {
    pub DmaBufAlloc: Option<
        unsafe extern "C" fn(*mut PlatformData, AudioStreamType) -> int32_t,
    >,
    pub DmaBufFree: Option<
        unsafe extern "C" fn(*mut PlatformData, AudioStreamType) -> int32_t,
    >,
    pub DmaRequestChannel: Option<
        unsafe extern "C" fn(*const PlatformData, AudioStreamType) -> int32_t,
    >,
    pub DmaConfigChannel: Option<
        unsafe extern "C" fn(*const PlatformData, AudioStreamType) -> int32_t,
    >,
    pub DmaPrep: Option<
        unsafe extern "C" fn(*const PlatformData, AudioStreamType) -> int32_t,
    >,
    pub DmaSubmit: Option<
        unsafe extern "C" fn(*const PlatformData, AudioStreamType) -> int32_t,
    >,
    pub DmaPending: Option<
        unsafe extern "C" fn(*mut PlatformData, AudioStreamType) -> int32_t,
    >,
    pub DmaPause: Option<
        unsafe extern "C" fn(*mut PlatformData, AudioStreamType) -> int32_t,
    >,
    pub DmaResume: Option<
        unsafe extern "C" fn(*const PlatformData, AudioStreamType) -> int32_t,
    >,
    pub DmaPointer: Option<
        unsafe extern "C" fn(
            *mut PlatformData,
            AudioStreamType,
            *mut uint32_t,
        ) -> int32_t,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CodecDevice {
    pub devCodecName: *const core::ffi::c_char,
    pub devData: *mut CodecData,
    pub device: *mut HdfDeviceObject,
    pub list: DListHead,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CodecData {
    pub drvCodecName: *const core::ffi::c_char,
    pub Init: Option<
        unsafe extern "C" fn(*mut AudioCard, *const CodecDevice) -> int32_t,
    >,
    pub Read: Option<
        unsafe extern "C" fn(*const CodecDevice, uint32_t, *mut uint32_t) -> int32_t,
    >,
    pub Write: Option<
        unsafe extern "C" fn(*const CodecDevice, uint32_t, uint32_t) -> int32_t,
    >,
    pub controls: *mut AudioKcontrol,
    pub numControls: core::ffi::c_int,
    pub sapmComponents: *mut AudioSapmComponent,
    pub numSapmComponent: core::ffi::c_int,
    pub sapmRoutes: *const AudioSapmRoute,
    pub numSapmRoutes: core::ffi::c_int,
    pub virtualAddress: core::ffi::c_ulong,
    pub regConfig: *mut AudioRegCfgData,
    pub regCfgGroup: *mut *mut AudioRegCfgGroupNode,
    pub mutex: OsalMutex,
    pub privateParam: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioSapmContext {
    pub componentNum: int32_t,
    pub biasLevel: AudioBiasLevel,
    pub suspendBiasLevel: AudioBiasLevel,
    pub codec: *mut CodecDevice,
    pub platform: *mut PlatformDevice,
    pub card: *mut AudioCard,
    pub targetBiasLevel: AudioBiasLevel,
    pub list: DListHead,
}
pub type AudioBiasLevel = core::ffi::c_uint;
pub const AUDIO_BIAS_ON: AudioBiasLevel = 3;
pub const AUDIO_BIAS_PREPARE: AudioBiasLevel = 2;
pub const AUDIO_BIAS_STANDBY: AudioBiasLevel = 1;
pub const AUDIO_BIAS_OFF: AudioBiasLevel = 0;
pub type AudioSapmType = core::ffi::c_uint;
pub const AUDIO_SAPM_SINK: AudioSapmType = 27;
pub const AUDIO_SAPM_SIGGEN: AudioSapmType = 26;
pub const AUDIO_SAPM_AIF_OUT: AudioSapmType = 25;
pub const AUDIO_SAPM_AIF_IN: AudioSapmType = 24;
pub const AUDIO_SAPM_CLOCK_SUPPLY: AudioSapmType = 23;
pub const AUDIO_SAPM_REGULATOR_SUPPLY: AudioSapmType = 22;
pub const AUDIO_SAPM_SUPPLY: AudioSapmType = 21;
pub const AUDIO_SAPM_POST: AudioSapmType = 20;
pub const AUDIO_SAPM_PRE: AudioSapmType = 19;
pub const AUDIO_SAPM_VMID: AudioSapmType = 18;
pub const AUDIO_SAPM_ANALOG_SWITCH: AudioSapmType = 17;
pub const AUDIO_SAPM_LINE: AudioSapmType = 16;
pub const AUDIO_SAPM_SPK: AudioSapmType = 15;
pub const AUDIO_SAPM_HP: AudioSapmType = 14;
pub const AUDIO_SAPM_MIC: AudioSapmType = 13;
pub const AUDIO_SAPM_MICBIAS: AudioSapmType = 12;
pub const AUDIO_SAPM_DAC: AudioSapmType = 11;
pub const AUDIO_SAPM_ADC: AudioSapmType = 10;
pub const AUDIO_SAPM_OUT_DRV: AudioSapmType = 9;
pub const AUDIO_SAPM_PGA: AudioSapmType = 8;
pub const AUDIO_SAPM_MIXER_NAMED_CTRL: AudioSapmType = 7;
pub const AUDIO_SAPM_MIXER: AudioSapmType = 6;
pub const AUDIO_SAPM_VALUE_MUX: AudioSapmType = 5;
pub const AUDIO_SAPM_VIRT_MUX: AudioSapmType = 4;
pub const AUDIO_SAPM_DEMUX: AudioSapmType = 3;
pub const AUDIO_SAPM_MUX: AudioSapmType = 2;
pub const AUDIO_SAPM_OUTPUT: AudioSapmType = 1;
pub const AUDIO_SAPM_INPUT: AudioSapmType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioRuntimeDeivces {
    pub codec: *mut CodecDevice,
    pub platform: *mut PlatformDevice,
    pub codecDai: *mut DaiDevice,
    pub cpuDai: *mut DaiDevice,
    pub dsp: *mut DspDevice,
    pub dspDai: *mut DaiDevice,
    pub complete: uint8_t,
    pub frameBits: uint32_t,
    pub dmaArea: *mut uint8_t,
    pub bufferSize: core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DaiDevice {
    pub devDaiName: *const core::ffi::c_char,
    pub devData: *mut DaiData,
    pub device: *mut HdfDeviceObject,
    pub list: DListHead,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DaiData {
    pub drvDaiName: *const core::ffi::c_char,
    pub DaiInit: Option<
        unsafe extern "C" fn(*mut AudioCard, *const DaiDevice) -> int32_t,
    >,
    pub Read: Option<
        unsafe extern "C" fn(*const DaiDevice, uint32_t, *mut uint32_t) -> int32_t,
    >,
    pub Write: Option<
        unsafe extern "C" fn(*const DaiDevice, uint32_t, uint32_t) -> int32_t,
    >,
    pub ops: *const AudioDaiOps,
    pub portInfo: AudioPortInfo,
    pub pcmInfo: PcmInfo,
    pub controls: *mut AudioKcontrol,
    pub numControls: core::ffi::c_int,
    pub daiInitFlag: bool,
    pub regVirtualAddr: core::ffi::c_ulong,
    pub regConfig: *mut AudioRegCfgData,
    pub regCfgGroup: *mut *mut AudioRegCfgGroupNode,
    pub mutex: OsalMutex,
    pub privateParam: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioPortInfo {
    pub render: AudioPcmStream,
    pub capture: AudioPcmStream,
}
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
pub struct AudioDaiOps {
    pub Startup: Option<
        unsafe extern "C" fn(*const AudioCard, *const DaiDevice) -> int32_t,
    >,
    pub HwParams: Option<
        unsafe extern "C" fn(*const AudioCard, *const AudioPcmHwParams) -> int32_t,
    >,
    pub Trigger: Option<
        unsafe extern "C" fn(
            *const AudioCard,
            core::ffi::c_int,
            *const DaiDevice,
        ) -> int32_t,
    >,
    pub Shutdown: Option<
        unsafe extern "C" fn(*const AudioCard, *const DaiDevice) -> int32_t,
    >,
    pub MuteStream: Option<
        unsafe extern "C" fn(
            *const AudioCard,
            *const DaiDevice,
            bool,
            int32_t,
        ) -> int32_t,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioPcmHwParams {
    pub streamType: AudioStreamType,
    pub channels: uint32_t,
    pub rate: uint32_t,
    pub periodSize: uint32_t,
    pub periodCount: uint32_t,
    pub format: AudioFormat,
    pub cardServiceName: *const core::ffi::c_char,
    pub period: uint32_t,
    pub frameSize: uint32_t,
    pub isBigEndian: bool,
    pub isSignedData: bool,
    pub startThreshold: uint32_t,
    pub stopThreshold: uint32_t,
    pub silenceThreshold: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DspDevice {
    pub devDspName: *const core::ffi::c_char,
    pub devData: *mut DspData,
    pub device: *mut HdfDeviceObject,
    pub list: DListHead,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DspData {
    pub drvDspName: *const core::ffi::c_char,
    pub DspInit: Option<unsafe extern "C" fn(*const DspDevice) -> int32_t>,
    pub Read: Option<
        unsafe extern "C" fn(
            *const DspDevice,
            *const core::ffi::c_void,
            uint32_t,
        ) -> int32_t,
    >,
    pub Write: Option<
        unsafe extern "C" fn(
            *const DspDevice,
            *const core::ffi::c_void,
            uint32_t,
        ) -> int32_t,
    >,
    pub Decode: Option<
        unsafe extern "C" fn(
            *const AudioCard,
            *const uint8_t,
            *const DspDevice,
        ) -> int32_t,
    >,
    pub Encode: Option<
        unsafe extern "C" fn(
            *const AudioCard,
            *const uint8_t,
            *const DspDevice,
        ) -> int32_t,
    >,
    pub Equalizer: Option<
        unsafe extern "C" fn(
            *const AudioCard,
            *const uint8_t,
            *const DspDevice,
        ) -> int32_t,
    >,
}
pub type CriBuffStatus = core::ffi::c_int;
pub const ENUM_CIR_BUFF_EMPTY: CriBuffStatus = -3;
pub const ENUM_CIR_BUFF_FULL: CriBuffStatus = -2;
pub const ENUM_CIR_BUFF_NORMAL: CriBuffStatus = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioTxData {
    pub status: int32_t,
    pub buf: *mut core::ffi::c_char,
    pub frames: core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioRxData {
    pub status: int32_t,
    pub buf: *mut core::ffi::c_char,
    pub bufSize: core::ffi::c_ulong,
    pub frames: core::ffi::c_ulong,
}
pub type DataBitWidth = core::ffi::c_uint;
pub const DATA_BIT_WIDTH32: DataBitWidth = 32;
pub const DATA_BIT_WIDTH24: DataBitWidth = 24;
pub const DATA_BIT_WIDTH20: DataBitWidth = 20;
pub const DATA_BIT_WIDTH18: DataBitWidth = 18;
pub const DATA_BIT_WIDTH16: DataBitWidth = 16;
pub const DATA_BIT_WIDTH8: DataBitWidth = 8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioEvent {
    pub eventType: uint32_t,
    pub deviceType: uint32_t,
}
pub const AUDIO_DRV_PCM_IOCTL_RENDER_RESUME: StreamDispMethodCmd = 11;
pub const AUDIO_DRV_PCM_IOCTL_RENDER_PAUSE: StreamDispMethodCmd = 9;
pub const AUDIO_DRV_PCM_IOCTL_RENDER_STOP: StreamDispMethodCmd = 6;
pub const AUDIO_DRV_PCM_IOCTL_RENDER_START: StreamDispMethodCmd = 5;
pub const AUDIO_DRV_PCM_IOCTL_CAPTURE_RESUME: StreamDispMethodCmd = 12;
pub const AUDIO_DRV_PCM_IOCTL_CAPTURE_PAUSE: StreamDispMethodCmd = 10;
pub const AUDIO_DRV_PCM_IOCTL_CAPTURE_STOP: StreamDispMethodCmd = 8;
pub const AUDIO_DRV_PCM_IOCTL_CAPTURE_START: StreamDispMethodCmd = 7;
pub type StreamDispMethodCmd = core::ffi::c_uint;
pub const AUDIO_DRV_PCM_IOCTL_BUTT: StreamDispMethodCmd = 24;
pub const AUDIO_DRV_PCM_IOCTL_DSPEQUALIZER: StreamDispMethodCmd = 23;
pub const AUDIO_DRV_PCM_IOCTL_DSPENCODE: StreamDispMethodCmd = 22;
pub const AUDIO_DRV_PCM_IOCTL_DSPDECODE: StreamDispMethodCmd = 21;
pub const AUDIO_DRV_PCM_IOCTL_CAPTURE_CLOSE: StreamDispMethodCmd = 20;
pub const AUDIO_DRV_PCM_IOCTL_CAPTURE_OPEN: StreamDispMethodCmd = 19;
pub const AUDIO_DRV_PCM_IOCTL_RENDER_CLOSE: StreamDispMethodCmd = 18;
pub const AUDIO_DRV_PCM_IOCTL_RENDER_OPEN: StreamDispMethodCmd = 17;
pub const AUDIO_DRV_PCM_IOCTL_MMAP_POSITION_CAPTURE: StreamDispMethodCmd = 16;
pub const AUDIO_DRV_PCM_IOCTL_MMAP_POSITION: StreamDispMethodCmd = 15;
pub const AUDIO_DRV_PCM_IOCTL_MMAP_BUFFER_CAPTURE: StreamDispMethodCmd = 14;
pub const AUDIO_DRV_PCM_IOCTL_MMAP_BUFFER: StreamDispMethodCmd = 13;
pub const AUDIO_DRV_PCM_IOCTL_READ: StreamDispMethodCmd = 4;
pub const AUDIO_DRV_PCM_IOCTL_WRITE: StreamDispMethodCmd = 3;
pub const AUDIO_DRV_PCM_IOCTL_CAPTURE_PREPARE: StreamDispMethodCmd = 2;
pub const AUDIO_DRV_PCM_IOCTL_RENDER_PREPARE: StreamDispMethodCmd = 1;
pub const AUDIO_DRV_PCM_IOCTL_HW_PARAMS: StreamDispMethodCmd = 0;
pub const EOK: core::ffi::c_int = 0 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const BITSTOBYTE: core::ffi::c_int = 8 as core::ffi::c_int;
pub const MAX_BUFF_SIZE: core::ffi::c_int = 128 as core::ffi::c_int
    * 1024 as core::ffi::c_int;
pub const MIN_BUFF_SIZE: core::ffi::c_int = 16 as core::ffi::c_int
    * 1024 as core::ffi::c_int;
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;
pub const MAX_PERIOD_SIZE: core::ffi::c_int = 8 as core::ffi::c_int
    * 1024 as core::ffi::c_int;
pub const MIN_PERIOD_SIZE: core::ffi::c_int = 2 as core::ffi::c_int
    * 1024 as core::ffi::c_int;
pub const SECOND_TO_MILLISECOND: core::ffi::c_int = 1000 as core::ffi::c_int;
#[no_mangle]
pub static mut PERIOD_COUNT: int32_t = 4 as int32_t;
#[no_mangle]
pub static mut RENDER_TRAF_BUF_SIZE: int32_t = 1024 as int32_t;
#[no_mangle]
pub static mut TIME_OUT_CONST: int32_t = 50 as int32_t;
#[no_mangle]
pub static mut SLEEP_TIME: int32_t = 5 as int32_t;
pub const AUDIO_PNP_MSG_LEN: core::ffi::c_int = 256 as core::ffi::c_int;
pub const INTERLEAVED: core::ffi::c_int = 1 as core::ffi::c_int;
#[no_mangle]
pub static mut MIN_PERIOD_SILENCE_THRESHOLD: int32_t = 4 as int32_t * 1024 as int32_t;
#[no_mangle]
pub static mut MAX_PERIOD_SILENCE_THRESHOLD: int32_t = 16 as int32_t * 1024 as int32_t;
#[no_mangle]
pub unsafe extern "C" fn SysReadl(mut addr: core::ffi::c_ulong) -> uint32_t {
    return *(addr as uintptr_t as *mut uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn SysWritel(mut addr: core::ffi::c_ulong, mut value: uint32_t) {
    ::core::ptr::write_volatile((addr as uintptr_t as *mut uint32_t), value);
}
#[no_mangle]
pub unsafe extern "C" fn PlatformDataFromCard(
    mut card: *const AudioCard,
) -> *mut PlatformData {
    if card.is_null() || ((*card).rtd).is_null() || ((*(*card).rtd).platform).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: param is null.\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"PlatformDataFromCard\0"))
                .as_ptr(),
            45 as core::ffi::c_int,
        );
        return 0 as *mut PlatformData;
    }
    return (*(*(*card).rtd).platform).devData as *mut PlatformData;
}
#[no_mangle]
pub unsafe extern "C" fn AudioBytesToFrames(
    mut frameBits: uint32_t,
    mut size: uint32_t,
) -> uint32_t {
    if size == 0 as core::ffi::c_uint {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: size is null.\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioBytesToFrames\0"))
                .as_ptr(),
            54 as core::ffi::c_int,
        );
        return 0 as uint32_t;
    } else {
        return frameBits.wrapping_div(size)
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioDataBigEndianChange(
    mut srcData: *mut core::ffi::c_char,
    mut audioLen: uint32_t,
    mut bitWidth: DataBitWidth,
) -> int32_t {
    let mut i: uint64_t = 0;
    let mut framesize: uint16_t = 0;
    let mut changeData: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut pData: *mut uint32_t = 0 as *mut uint32_t;
    if srcData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: srcData is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"AudioDataBigEndianChange\0"))
                .as_ptr(),
            69 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    changeData = srcData;
    pData = changeData as *mut uint32_t;
    match bitWidth as core::ffi::c_uint {
        8 => return HDF_SUCCESS as core::ffi::c_int as int32_t,
        24 => {
            framesize = 3 as uint16_t;
            i = 0 as uint64_t;
            while i < audioLen as core::ffi::c_ulonglong {
                *pData = (*pData >> 0x10 as core::ffi::c_int & 0xff as core::ffi::c_uint
                    | *pData & 0xff00ff00 as core::ffi::c_uint
                    | *pData << 0x10 as core::ffi::c_int & 0xff0000 as core::ffi::c_uint)
                    as uint32_t;
                changeData = changeData.offset(framesize as core::ffi::c_int as isize);
                pData = changeData as *mut uint32_t;
                i = (i as core::ffi::c_ulonglong)
                    .wrapping_add(framesize as core::ffi::c_ulonglong) as uint64_t
                    as uint64_t;
            }
        }
        16 | _ => {
            framesize = 4 as uint16_t;
            i = 0 as uint64_t;
            while i < audioLen as core::ffi::c_ulonglong {
                *pData = (*pData << 0x8 as core::ffi::c_int
                    & 0xff00ff00 as core::ffi::c_uint
                    | *pData >> 0x8 as core::ffi::c_int & 0xff00ff as core::ffi::c_uint)
                    as uint32_t;
                pData = pData.offset(1);
                i = (i as core::ffi::c_ulonglong)
                    .wrapping_add(framesize as core::ffi::c_ulonglong) as uint64_t
                    as uint64_t;
            }
        }
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioFormatToBitWidth(
    mut format: AudioFormat,
    mut bitWidth: *mut uint32_t,
) -> int32_t {
    if bitWidth.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: bitWidth is null.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioFormatToBitWidth\0"))
                .as_ptr(),
            106 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    match format as core::ffi::c_uint {
        1 => {
            *bitWidth = DATA_BIT_WIDTH8 as core::ffi::c_int as uint32_t;
        }
        2 => {
            *bitWidth = DATA_BIT_WIDTH16 as core::ffi::c_int as uint32_t;
        }
        3 => {
            *bitWidth = DATA_BIT_WIDTH24 as core::ffi::c_int as uint32_t;
        }
        4 => {
            *bitWidth = DATA_BIT_WIDTH32 as core::ffi::c_int as uint32_t;
        }
        _ => {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: format: %d is not define.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 22],
                    [core::ffi::c_char; 22],
                >(*b"AudioFormatToBitWidth\0"))
                    .as_ptr(),
                127 as core::ffi::c_int,
                format as core::ffi::c_uint,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSetPcmInfo(
    mut platformData: *mut PlatformData,
    mut param: *const AudioPcmHwParams,
) -> int32_t {
    if platformData.is_null() || param.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: platform is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"AudioSetPcmInfo\0"))
                .as_ptr(),
            136 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    (*platformData).renderBufInfo.chnId = 0 as uint32_t;
    (*platformData).captureBufInfo.chnId = 0 as uint32_t;
    if (*param).streamType as core::ffi::c_uint
        == AUDIO_RENDER_STREAM as core::ffi::c_int as core::ffi::c_uint
    {
        if AudioFormatToBitWidth(
            (*param).format,
            &mut (*platformData).renderPcmInfo.bitWidth,
        ) != HDF_SUCCESS as core::ffi::c_int
        {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: renderPcmInfo is failure\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"AudioSetPcmInfo\0"))
                    .as_ptr(),
                144 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
        (*platformData).renderPcmInfo.rate = (*param).rate;
        (*platformData).renderPcmInfo.frameSize = ((*param).channels
            as core::ffi::c_uint)
            .wrapping_mul((*platformData).renderPcmInfo.bitWidth as core::ffi::c_uint)
            .wrapping_div(BITSTOBYTE as core::ffi::c_uint) as uint32_t;
        (*platformData).renderPcmInfo.channels = (*param).channels;
        (*platformData).renderPcmInfo.isBigEndian = (*param).isBigEndian;
        (*platformData).renderPcmInfo.isSignedData = (*param).isSignedData;
        (*platformData).renderPcmInfo.startThreshold = (*param).startThreshold;
        (*platformData).renderPcmInfo.stopThreshold = (*param).stopThreshold;
        (*platformData).renderPcmInfo.silenceThreshold = (*param).silenceThreshold;
        (*platformData).renderPcmInfo.interleaved = INTERLEAVED as uint32_t;
        (*platformData).renderPcmInfo.streamType = (*param).streamType;
    } else if (*param).streamType as core::ffi::c_uint
        == AUDIO_CAPTURE_STREAM as core::ffi::c_int as core::ffi::c_uint
    {
        if AudioFormatToBitWidth(
            (*param).format,
            &mut (*platformData).capturePcmInfo.bitWidth,
        ) != HDF_SUCCESS as core::ffi::c_int
        {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: capturePcmInfo is failure\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"AudioSetPcmInfo\0"))
                    .as_ptr(),
                162 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
        (*platformData).capturePcmInfo.rate = (*param).rate;
        (*platformData).capturePcmInfo.frameSize = ((*param).channels
            as core::ffi::c_uint)
            .wrapping_mul((*platformData).capturePcmInfo.bitWidth as core::ffi::c_uint)
            .wrapping_div(BITSTOBYTE as core::ffi::c_uint) as uint32_t;
        (*platformData).capturePcmInfo.channels = (*param).channels;
        (*platformData).capturePcmInfo.isBigEndian = (*param).isBigEndian;
        (*platformData).capturePcmInfo.isSignedData = (*param).isSignedData;
        (*platformData).capturePcmInfo.startThreshold = (*param).startThreshold;
        (*platformData).capturePcmInfo.stopThreshold = (*param).stopThreshold;
        (*platformData).capturePcmInfo.silenceThreshold = (*param).silenceThreshold;
        (*platformData).capturePcmInfo.interleaved = INTERLEAVED as uint32_t;
        (*platformData).capturePcmInfo.streamType = (*param).streamType;
    } else {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: streamType is invalid.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"AudioSetPcmInfo\0"))
                .as_ptr(),
            179 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSetRenderBufInfo(
    mut data: *mut PlatformData,
    mut param: *const AudioPcmHwParams,
) -> int32_t {
    let mut size: uint32_t = 0;
    if data.is_null() || param.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: platform is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioSetRenderBufInfo\0"))
                .as_ptr(),
            191 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    if (*param).period < MIN_PERIOD_SIZE as core::ffi::c_uint
        || (*param).period > MAX_PERIOD_SIZE as core::ffi::c_uint
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: periodSize is invalid %d.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioSetRenderBufInfo\0"))
                .as_ptr(),
            196 as core::ffi::c_int,
            (*param).period,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*data).renderBufInfo.periodSize = ((*param).period as core::ffi::c_uint)
        .wrapping_mul((*data).renderPcmInfo.bitWidth as core::ffi::c_uint)
        .wrapping_mul((*data).renderPcmInfo.channels as core::ffi::c_uint)
        .wrapping_div(BITSTOBYTE as core::ffi::c_uint) as uint32_t;
    (*data).renderBufInfo.periodCount = PERIOD_COUNT as uint32_t;
    (*data).renderBufInfo.trafBufSize = RENDER_TRAF_BUF_SIZE as uint32_t;
    size = ((*data).renderBufInfo.periodCount)
        .wrapping_mul((*data).renderBufInfo.periodSize);
    if size < MIN_BUFF_SIZE as core::ffi::c_uint
        || size > MAX_BUFF_SIZE as core::ffi::c_uint
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: buffSize is invalid.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioSetRenderBufInfo\0"))
                .as_ptr(),
            206 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*data).renderBufInfo.cirBufSize = size;
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSetCaptureBufInfo(
    mut data: *mut PlatformData,
    mut param: *const AudioPcmHwParams,
) -> int32_t {
    if data.is_null() || param.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: platform is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioSetCaptureBufInfo\0"))
                .as_ptr(),
            217 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    if (*param).period < MIN_PERIOD_SIZE as core::ffi::c_uint
        || (*param).period > MAX_PERIOD_SIZE as core::ffi::c_uint
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: periodSize is invalid %d.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioSetCaptureBufInfo\0"))
                .as_ptr(),
            222 as core::ffi::c_int,
            (*param).period,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*data).captureBufInfo.periodSize = ((*param).period as core::ffi::c_uint)
        .wrapping_mul((*data).capturePcmInfo.bitWidth as core::ffi::c_uint)
        .wrapping_mul((*data).capturePcmInfo.channels as core::ffi::c_uint)
        .wrapping_div(BITSTOBYTE as core::ffi::c_uint) as uint32_t;
    (*data).captureBufInfo.periodCount = PERIOD_COUNT as uint32_t;
    if (*param).silenceThreshold < MIN_PERIOD_SILENCE_THRESHOLD as core::ffi::c_uint
        || (*param).silenceThreshold > MAX_PERIOD_SILENCE_THRESHOLD as core::ffi::c_uint
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: silenceThreshold is invalid %d.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioSetCaptureBufInfo\0"))
                .as_ptr(),
            231 as core::ffi::c_int,
            (*param).silenceThreshold,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*data).captureBufInfo.trafBufSize = (*param).silenceThreshold;
    (*data).captureBufInfo.cirBufSize = ((*data).captureBufInfo.periodSize)
        .wrapping_mul((*data).captureBufInfo.periodCount);
    if (*data).captureBufInfo.cirBufSize > (*data).captureBufInfo.cirBufMax {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: cirBufSize is invalid %d.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioSetCaptureBufInfo\0"))
                .as_ptr(),
            237 as core::ffi::c_int,
            (*data).captureBufInfo.cirBufSize,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioDmaBuffStatus(
    mut card: *const AudioCard,
    mut streamType: AudioStreamType,
) -> int32_t {
    let mut dataAvailable: uint32_t = 0;
    let mut residual: uint32_t = 0;
    let mut pointer: uint32_t = 0 as uint32_t;
    let mut wptr: uint32_t = 0;
    let mut rptr: uint32_t = 0;
    let mut data: *mut PlatformData = PlatformDataFromCard(card) as *mut PlatformData;
    if data.is_null() || ((*data).ops).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: PlatformDataFromCard failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioDmaBuffStatus\0"))
                .as_ptr(),
            253 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioPcmPointer(card, &mut pointer, streamType) != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: get Pointer failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioDmaBuffStatus\0"))
                .as_ptr(),
            258 as core::ffi::c_int,
        );
        return ENUM_CIR_BUFF_FULL as core::ffi::c_int as int32_t;
    }
    if streamType as core::ffi::c_uint
        == AUDIO_RENDER_STREAM as core::ffi::c_int as core::ffi::c_uint
    {
        (*data).renderBufInfo.pointer = pointer;
        rptr = ((*data).renderBufInfo.pointer)
            .wrapping_mul((*data).renderPcmInfo.frameSize);
        dataAvailable = ((*data).renderBufInfo.wbufOffSet)
            .wrapping_sub(rptr)
            .wrapping_rem((*data).renderBufInfo.cirBufSize);
        residual = ((*data).renderBufInfo.cirBufSize).wrapping_sub(dataAvailable);
        if residual > (*data).renderBufInfo.trafBufSize {
            return ENUM_CIR_BUFF_NORMAL as core::ffi::c_int as int32_t
        } else {
            return ((*data).renderBufInfo.trafBufSize)
                .wrapping_sub(residual)
                .wrapping_div((*data).renderBufInfo.oneMsBytes) as int32_t
        }
    } else if streamType as core::ffi::c_uint
        == AUDIO_CAPTURE_STREAM as core::ffi::c_int as core::ffi::c_uint
    {
        rptr = (*data).captureBufInfo.rptrOffSet;
        wptr = pointer.wrapping_mul((*data).capturePcmInfo.frameSize);
        (*data).captureBufInfo.pointer = pointer;
        if wptr >= rptr {
            dataAvailable = wptr.wrapping_sub(rptr);
            if dataAvailable < (*data).captureBufInfo.trafBufSize {
                return ((*data).captureBufInfo.trafBufSize)
                    .wrapping_sub(dataAvailable)
                    .wrapping_div((*data).captureBufInfo.oneMsBytes) as int32_t;
            }
        }
        return ENUM_CIR_BUFF_NORMAL as core::ffi::c_int as int32_t;
    } else {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: streamType is invalid.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioDmaBuffStatus\0"))
                .as_ptr(),
            292 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioPcmWrite(
    mut card: *const AudioCard,
    mut txData: *mut AudioTxData,
) -> int32_t {
    let mut data: *mut PlatformData = 0 as *mut PlatformData;
    let mut status: int32_t = 0;
    let mut wPtr: uint32_t = 0;
    let mut ret: int32_t = 0;
    if card.is_null() || txData.is_null() || ((*txData).buf).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input param is null.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"AudioPcmWrite\0"))
                .as_ptr(),
            305 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    data = PlatformDataFromCard(card) as *mut PlatformData;
    if data.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: from PlatformDataFromCard get platformData is NULL.\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"AudioPcmWrite\0"))
                .as_ptr(),
            311 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*data).renderBufInfo.trafBufSize = ((*txData).frames)
        .wrapping_mul((*data).renderPcmInfo.frameSize as core::ffi::c_ulong) as uint32_t;
    if (*data).renderPcmInfo.isBigEndian {
        if AudioDataBigEndianChange(
            (*txData).buf,
            (*data).renderBufInfo.trafBufSize,
            (*data).renderPcmInfo.bitWidth as DataBitWidth,
        ) != HDF_SUCCESS as core::ffi::c_int
        {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: AudioDataBigEndianChange: failed.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 14],
                    [core::ffi::c_char; 14],
                >(*b"AudioPcmWrite\0"))
                    .as_ptr(),
                322 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    }
    status = AudioDmaBuffStatus(card, AUDIO_RENDER_STREAM);
    if status != ENUM_CIR_BUFF_NORMAL as core::ffi::c_int {
        (*txData).status = status;
        return HDF_SUCCESS as core::ffi::c_int as int32_t;
    }
    if (*data).renderBufInfo.trafBufSize > (*data).renderBufInfo.cirBufSize {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: transferFrameSize is tool big.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"AudioPcmWrite\0"))
                .as_ptr(),
            335 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    wPtr = ((*data).renderBufInfo.wbufOffSet)
        .wrapping_rem((*data).renderBufInfo.cirBufSize);
    if ((*data).renderBufInfo.virtAddr).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: render buffer is null.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"AudioPcmWrite\0"))
                .as_ptr(),
            341 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    ret = memcpy_s(
        ((*data).renderBufInfo.virtAddr as *mut core::ffi::c_char).offset(wPtr as isize)
            as *mut core::ffi::c_void,
        (*data).renderBufInfo.trafBufSize as size_t,
        (*txData).buf as *const core::ffi::c_void,
        (*data).renderBufInfo.trafBufSize as size_t,
    ) as int32_t;
    if ret != 0 as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: memcpy_s failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"AudioPcmWrite\0"))
                .as_ptr(),
            347 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*txData).status = ENUM_CIR_BUFF_NORMAL as core::ffi::c_int as int32_t;
    (*data).renderBufInfo.wptrOffSet = wPtr
        .wrapping_add((*data).renderBufInfo.trafBufSize);
    (*data).renderBufInfo.wbufOffSet = ((*data).renderBufInfo.wbufOffSet
        as core::ffi::c_uint)
        .wrapping_add((*data).renderBufInfo.trafBufSize as core::ffi::c_uint) as uint32_t
        as uint32_t;
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn PcmReadData(
    mut data: *mut PlatformData,
    mut rxData: *mut AudioRxData,
) -> int32_t {
    let mut wptr: uint32_t = 0;
    let mut rptr: uint32_t = 0;
    let mut validDataSize: uint32_t = 0;
    if data.is_null() || rxData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input param is null.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 12],
                [core::ffi::c_char; 12],
            >(*b"PcmReadData\0"))
                .as_ptr(),
            364 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*rxData).buf = ((*data).captureBufInfo.virtAddr as *mut core::ffi::c_char)
        .offset((*data).captureBufInfo.rptrOffSet as isize);
    wptr = ((*data).captureBufInfo.pointer)
        .wrapping_mul((*data).capturePcmInfo.frameSize);
    rptr = (*data).captureBufInfo.rptrOffSet;
    (*data).captureBufInfo.curTrafSize = (*data).captureBufInfo.trafBufSize;
    if rptr > wptr {
        validDataSize = ((*data).captureBufInfo.cirBufSize).wrapping_sub(rptr);
        if validDataSize < (*data).captureBufInfo.trafBufSize {
            (*data).captureBufInfo.curTrafSize = validDataSize;
        }
    }
    if (*data).capturePcmInfo.isBigEndian {
        if ((*rxData).buf).is_null()
            || AudioDataBigEndianChange(
                (*rxData).buf,
                (*data).captureBufInfo.curTrafSize,
                (*data).capturePcmInfo.bitWidth as DataBitWidth,
            ) != HDF_SUCCESS as core::ffi::c_int
        {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: AudioDataBigEndianChange: failed.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 12],
                    [core::ffi::c_char; 12],
                >(*b"PcmReadData\0"))
                    .as_ptr(),
                383 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    }
    (*rxData).frames = ((*data).captureBufInfo.curTrafSize)
        .wrapping_div((*data).capturePcmInfo.frameSize) as core::ffi::c_ulong;
    (*rxData).bufSize = (*data).captureBufInfo.curTrafSize as core::ffi::c_ulong;
    (*rxData).status = ENUM_CIR_BUFF_NORMAL as core::ffi::c_int as int32_t;
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioPcmRead(
    mut card: *const AudioCard,
    mut rxData: *mut AudioRxData,
) -> int32_t {
    let mut data: *mut PlatformData = 0 as *mut PlatformData;
    let mut status: int32_t = 0;
    if card.is_null() || rxData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input param is null.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"AudioPcmRead\0"))
                .as_ptr(),
            401 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    data = PlatformDataFromCard(card) as *mut PlatformData;
    if data.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: from PlatformDataFromCard get platformData is NULL.\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"AudioPcmRead\0"))
                .as_ptr(),
            407 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if ((*data).captureBufInfo.virtAddr).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: capture buffer is null.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"AudioPcmRead\0"))
                .as_ptr(),
            411 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    status = AudioDmaBuffStatus(card, AUDIO_CAPTURE_STREAM);
    if status != ENUM_CIR_BUFF_NORMAL as core::ffi::c_int {
        (*rxData).status = status;
        (*rxData).buf = ((*data).captureBufInfo.virtAddr as *mut core::ffi::c_char)
            .offset((*data).captureBufInfo.rptrOffSet as isize);
        (*rxData).frames = 0 as core::ffi::c_ulong;
        (*rxData).bufSize = 0 as core::ffi::c_ulong;
        return HDF_SUCCESS as core::ffi::c_int as int32_t;
    }
    if PcmReadData(data, rxData) != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Pcm Read Data fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"AudioPcmRead\0"))
                .as_ptr(),
            428 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*data).captureBufInfo.rptrOffSet = ((*data).captureBufInfo.rptrOffSet
        as core::ffi::c_uint)
        .wrapping_add((*data).captureBufInfo.curTrafSize as core::ffi::c_uint)
        as uint32_t as uint32_t;
    if (*data).captureBufInfo.rptrOffSet >= (*data).captureBufInfo.cirBufSize {
        (*data).captureBufInfo.rptrOffSet = 0 as uint32_t;
    }
    (*data).captureBufInfo.rbufOffSet = ((*data).captureBufInfo.rbufOffSet
        as core::ffi::c_uint)
        .wrapping_add((*data).captureBufInfo.curTrafSize as core::ffi::c_uint)
        as uint32_t as uint32_t;
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn MmapWriteData(
    mut data: *mut PlatformData,
    mut tmpBuf: *mut core::ffi::c_char,
) -> int32_t {
    let mut wPtr: uint32_t = 0;
    let mut ret: int32_t = 0;
    if (*data).renderBufInfo.trafBufSize > (*data).renderBufInfo.cirBufSize {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: transferFrameSize is tool big.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"MmapWriteData\0"))
                .as_ptr(),
            447 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    wPtr = ((*data).renderBufInfo.wbufOffSet)
        .wrapping_rem((*data).renderBufInfo.cirBufSize);
    ret = CopyFromUser(
        tmpBuf as *mut core::ffi::c_void,
        ((*data).mmapData.memoryAddress as *mut core::ffi::c_char)
            .offset((*data).mmapData.offset as isize) as *const core::ffi::c_void,
        (*data).renderBufInfo.trafBufSize as size_t,
    ) as int32_t;
    if ret != EOK {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: CopyFromUser failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"MmapWriteData\0"))
                .as_ptr(),
            455 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    ret = memcpy_s(
        ((*data).renderBufInfo.virtAddr as *mut core::ffi::c_char).offset(wPtr as isize)
            as *mut core::ffi::c_void,
        (*data).renderBufInfo.trafBufSize as size_t,
        tmpBuf as *const core::ffi::c_void,
        (*data).renderBufInfo.trafBufSize as size_t,
    ) as int32_t;
    if ret != 0 as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: memcpy_s failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"MmapWriteData\0"))
                .as_ptr(),
            462 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*data).renderBufInfo.wptrOffSet = wPtr
        .wrapping_add((*data).renderBufInfo.trafBufSize);
    (*data).renderBufInfo.wbufOffSet = ((*data).renderBufInfo.wbufOffSet
        as core::ffi::c_uint)
        .wrapping_add((*data).renderBufInfo.trafBufSize as core::ffi::c_uint) as uint32_t
        as uint32_t;
    (*data).renderBufInfo.framesPosition = ((*data).renderBufInfo.framesPosition
        as core::ffi::c_uint)
        .wrapping_add(
            ((*data).renderBufInfo.trafBufSize)
                .wrapping_div((*data).renderPcmInfo.frameSize) as core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    (*data).mmapData.offset = ((*data).mmapData.offset as core::ffi::c_uint)
        .wrapping_add((*data).renderBufInfo.trafBufSize as core::ffi::c_uint) as uint32_t
        as uint32_t;
    (*data).mmapLoopCount = ((*data).mmapLoopCount).wrapping_add(1);
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioRenderPlatformDataInit(
    mut data: *mut PlatformData,
    mut totalSize: *mut uint32_t,
    mut lastBuffSize: *mut uint32_t,
    mut loopTimes: *mut uint32_t,
) -> int32_t {
    if data.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: PlatformDataFromCard failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"AudioRenderPlatformDataInit\0"))
                .as_ptr(),
            478 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if ((*data).renderBufInfo.virtAddr).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: render buffer is null.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"AudioRenderPlatformDataInit\0"))
                .as_ptr(),
            482 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if (*data).renderBufInfo.runStatus as core::ffi::c_uint
        != PCM_START as core::ffi::c_int as core::ffi::c_uint
    {
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: render did not start.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"AudioRenderPlatformDataInit\0"))
                .as_ptr(),
            486 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    *totalSize = ((*data).mmapData.totalBufferFrames as uint32_t)
        .wrapping_mul((*data).renderPcmInfo.frameSize);
    *lastBuffSize = (if (*totalSize).wrapping_rem(MIN_PERIOD_SIZE as core::ffi::c_uint)
        == 0 as core::ffi::c_uint
    {
        MIN_PERIOD_SIZE as core::ffi::c_uint
    } else {
        (*totalSize).wrapping_rem(MIN_PERIOD_SIZE as core::ffi::c_uint)
    }) as uint32_t;
    *loopTimes = (if *lastBuffSize == MIN_PERIOD_SIZE as core::ffi::c_uint {
        (*totalSize).wrapping_div(MIN_PERIOD_SIZE as core::ffi::c_uint)
    } else {
        (*totalSize)
            .wrapping_div(MIN_PERIOD_SIZE as core::ffi::c_uint)
            .wrapping_add(1 as core::ffi::c_uint)
    }) as uint32_t;
    (*data).mmapLoopCount = 0 as uint32_t;
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioMmapWriteTransfer(mut card: *const AudioCard) -> int32_t {
    let mut timeout: uint32_t = 0 as uint32_t;
    let mut totalSize: uint32_t = 0;
    let mut lastBuffSize: uint32_t = 0;
    let mut loopTimes: uint32_t = 0;
    let mut tmpBuf: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut data: *mut PlatformData = PlatformDataFromCard(card) as *mut PlatformData;
    if AudioRenderPlatformDataInit(
        data,
        &mut totalSize,
        &mut lastBuffSize,
        &mut loopTimes,
    ) == HDF_FAILURE as core::ffi::c_int
    {
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    tmpBuf = OsalMemCalloc(MIN_PERIOD_SIZE as size_t) as *mut core::ffi::c_char;
    if tmpBuf.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: tmpBuf is null.\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioMmapWriteTransfer\0"))
                .as_ptr(),
            512 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    while (*data).mmapLoopCount < loopTimes
        && (*data).renderBufInfo.runStatus as core::ffi::c_uint
            != PCM_STOP as core::ffi::c_int as core::ffi::c_uint
    {
        if (*data).renderBufInfo.runStatus as core::ffi::c_uint
            == PCM_PAUSE as core::ffi::c_int as core::ffi::c_uint
        {
            OsalMSleep(SLEEP_TIME as uint32_t);
        } else if AudioDmaBuffStatus(card, AUDIO_RENDER_STREAM)
            != ENUM_CIR_BUFF_NORMAL as core::ffi::c_int
        {
            OsalMSleep(SLEEP_TIME as uint32_t);
            timeout = timeout.wrapping_add(1);
            if timeout >= TIME_OUT_CONST as core::ffi::c_uint {
                OsalMemFree(tmpBuf as *mut core::ffi::c_void);
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    LOG_DOMAIN as core::ffi::c_uint,
                    b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s][line:%d]: timeout failed.\0" as *const u8
                        as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 23],
                        [core::ffi::c_char; 23],
                    >(*b"AudioMmapWriteTransfer\0"))
                        .as_ptr(),
                    527 as core::ffi::c_int,
                );
                return HDF_FAILURE as core::ffi::c_int as int32_t;
            }
        } else {
            timeout = 0 as uint32_t;
            (*data).renderBufInfo.trafBufSize = (if (*data).mmapLoopCount
                < (loopTimes as core::ffi::c_uint).wrapping_sub(1 as core::ffi::c_uint)
            {
                MIN_PERIOD_SIZE as core::ffi::c_uint
            } else {
                lastBuffSize as core::ffi::c_uint
            }) as uint32_t;
            if MmapWriteData(data, tmpBuf) != HDF_SUCCESS as core::ffi::c_int {
                OsalMemFree(tmpBuf as *mut core::ffi::c_void);
                return HDF_FAILURE as core::ffi::c_int as int32_t;
            }
        }
    }
    if (*data).mmapLoopCount > loopTimes {
        (*data).renderBufInfo.runStatus = PCM_STOP;
    }
    OsalMemFree(tmpBuf as *mut core::ffi::c_void);
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioPcmMmapWrite(
    mut card: *const AudioCard,
    mut txMmapData: *const AudioMmapData,
) -> int32_t {
    let mut ret: int32_t = 0;
    let mut data: *mut PlatformData = 0 as *mut PlatformData;
    data = PlatformDataFromCard(card) as *mut PlatformData;
    if data.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: PlatformDataFromCard failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"AudioPcmMmapWrite\0"))
                .as_ptr(),
            557 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if txMmapData.is_null() || ((*txMmapData).memoryAddress).is_null()
        || (*txMmapData).transferFrameSize <= 0 as core::ffi::c_int
        || (*txMmapData).totalBufferFrames <= 0 as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: param is invalid.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"AudioPcmMmapWrite\0"))
                .as_ptr(),
            563 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    (*data).mmapData.offset = (*txMmapData).offset;
    (*data).mmapData.memoryAddress = (*txMmapData).memoryAddress;
    (*data).mmapData.totalBufferFrames = (*txMmapData).totalBufferFrames;
    (*data).mmapData.transferFrameSize = (*txMmapData).transferFrameSize;
    (*data).mmapLoopCount = 0 as uint32_t;
    ret = AudioMmapWriteTransfer(card);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: AudioMmapTransfer fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"AudioPcmMmapWrite\0"))
                .as_ptr(),
            579 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn MmapReadData(
    mut data: *mut PlatformData,
    mut rxMmapData: *const AudioMmapData,
    mut offset: uint32_t,
) -> int32_t {
    let mut wPtr: uint32_t = 0;
    let mut rPtr: uint32_t = 0;
    let mut validDataSize: uint32_t = 0;
    if data.is_null() || rxMmapData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: data is null.\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"MmapReadData\0"))
                .as_ptr(),
            594 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    rPtr = (*data).captureBufInfo.rptrOffSet;
    wPtr = ((*data).captureBufInfo.pointer)
        .wrapping_mul((*data).capturePcmInfo.frameSize);
    if rPtr > wPtr {
        validDataSize = ((*data).captureBufInfo.cirBufSize).wrapping_sub(rPtr);
        if validDataSize < (*data).captureBufInfo.trafBufSize {
            (*data).captureBufInfo.curTrafSize = validDataSize;
        }
    }
    if (*data).capturePcmInfo.isBigEndian {
        if AudioDataBigEndianChange(
            ((*data).captureBufInfo.virtAddr as *mut core::ffi::c_char)
                .offset(rPtr as isize),
            (*data).captureBufInfo.curTrafSize,
            (*data).capturePcmInfo.bitWidth as DataBitWidth,
        ) != HDF_SUCCESS as core::ffi::c_int
        {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: AudioDataBigEndianChange: failed.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 13],
                    [core::ffi::c_char; 13],
                >(*b"MmapReadData\0"))
                    .as_ptr(),
                609 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    }
    if CopyToUser(
        ((*rxMmapData).memoryAddress as *mut core::ffi::c_char).offset(offset as isize)
            as *mut core::ffi::c_void,
        ((*data).captureBufInfo.virtAddr as *mut core::ffi::c_char).offset(rPtr as isize)
            as *const core::ffi::c_void,
        (*data).captureBufInfo.curTrafSize as size_t,
    ) != 0 as core::ffi::c_uint
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: CopyToUser failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"MmapReadData\0"))
                .as_ptr(),
            616 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*data).captureBufInfo.rptrOffSet = ((*data).captureBufInfo.rptrOffSet
        as core::ffi::c_uint)
        .wrapping_add((*data).captureBufInfo.curTrafSize as core::ffi::c_uint)
        as uint32_t as uint32_t;
    if (*data).captureBufInfo.rptrOffSet >= (*data).captureBufInfo.cirBufSize {
        (*data).captureBufInfo.rptrOffSet = 0 as uint32_t;
    }
    (*data).captureBufInfo.framesPosition = ((*data).captureBufInfo.framesPosition
        as core::ffi::c_uint)
        .wrapping_add(
            ((*data).captureBufInfo.curTrafSize)
                .wrapping_div((*data).capturePcmInfo.frameSize) as core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    (*data).captureBufInfo.rbufOffSet = ((*data).captureBufInfo.rbufOffSet
        as core::ffi::c_uint)
        .wrapping_add((*data).captureBufInfo.curTrafSize as core::ffi::c_uint)
        as uint32_t as uint32_t;
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioCapturePlatformDataInit(
    mut data: *mut PlatformData,
    mut rxMmapData: *const AudioMmapData,
    mut totalSize: *mut uint32_t,
) -> int32_t {
    (*data).captureBufInfo.pointer = 0 as uint32_t;
    (*data).captureBufInfo.curTrafSize = (*data).captureBufInfo.trafBufSize;
    if ((*data).captureBufInfo.virtAddr).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: capture buffer is null.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"AudioCapturePlatformDataInit\0"))
                .as_ptr(),
            636 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if (*data).captureBufInfo.runStatus as core::ffi::c_uint
        != PCM_START as core::ffi::c_int as core::ffi::c_uint
    {
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: capture did not start.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"AudioCapturePlatformDataInit\0"))
                .as_ptr(),
            641 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    *totalSize = ((*rxMmapData).totalBufferFrames as uint32_t)
        .wrapping_mul((*data).capturePcmInfo.frameSize);
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioMmapReadTransfer(
    mut card: *const AudioCard,
    mut rxMmapData: *const AudioMmapData,
) -> int32_t {
    let mut offset: uint32_t = 0 as uint32_t;
    let mut status: int32_t = 0;
    let mut timeout: uint32_t = 0 as uint32_t;
    let mut data: *mut PlatformData = 0 as *mut PlatformData;
    let mut totalSize: uint32_t = 0;
    if card.is_null() || rxMmapData.is_null() || ((*rxMmapData).memoryAddress).is_null()
        || (*rxMmapData).totalBufferFrames <= 0 as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: param is invalid.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioMmapReadTransfer\0"))
                .as_ptr(),
            659 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    data = PlatformDataFromCard(card) as *mut PlatformData;
    if data.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: PlatformDataFromCard failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioMmapReadTransfer\0"))
                .as_ptr(),
            665 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioCapturePlatformDataInit(data, rxMmapData, &mut totalSize)
        != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: AudioCapturePlatformDataInit failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioMmapReadTransfer\0"))
                .as_ptr(),
            669 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    loop {
        if (*data).captureBufInfo.runStatus as core::ffi::c_uint
            == PCM_PAUSE as core::ffi::c_int as core::ffi::c_uint
        {
            OsalMSleep(SLEEP_TIME as uint32_t);
        } else {
            status = AudioDmaBuffStatus(card, AUDIO_CAPTURE_STREAM);
            if status != ENUM_CIR_BUFF_NORMAL as core::ffi::c_int {
                OsalMSleep(SLEEP_TIME as uint32_t);
                timeout = timeout.wrapping_add(1);
                if timeout >= TIME_OUT_CONST as core::ffi::c_uint {
                    HiLogPrint(
                        LOG_CORE,
                        LOG_ERROR,
                        LOG_DOMAIN as core::ffi::c_uint,
                        b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                        b"[%s][line:%d]: timeout failed.\0" as *const u8
                            as *const core::ffi::c_char,
                        (::core::mem::transmute::<
                            [u8; 22],
                            [core::ffi::c_char; 22],
                        >(*b"AudioMmapReadTransfer\0"))
                            .as_ptr(),
                        685 as core::ffi::c_int,
                    );
                    return HDF_FAILURE as core::ffi::c_int as int32_t;
                }
            } else {
                timeout = 0 as uint32_t;
                if MmapReadData(data, rxMmapData, offset)
                    != HDF_SUCCESS as core::ffi::c_int
                {
                    HiLogPrint(
                        LOG_CORE,
                        LOG_ERROR,
                        LOG_DOMAIN as core::ffi::c_uint,
                        b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                        b"[%s][line:%d]: MmapReadData fail.\0" as *const u8
                            as *const core::ffi::c_char,
                        (::core::mem::transmute::<
                            [u8; 22],
                            [core::ffi::c_char; 22],
                        >(*b"AudioMmapReadTransfer\0"))
                            .as_ptr(),
                        694 as core::ffi::c_int,
                    );
                    return HDF_FAILURE as core::ffi::c_int as int32_t;
                }
                offset = (offset as core::ffi::c_uint)
                    .wrapping_add(
                        (*data).captureBufInfo.curTrafSize as core::ffi::c_uint,
                    ) as uint32_t as uint32_t;
            }
        }
        if !(offset < totalSize
            && (*data).captureBufInfo.runStatus as core::ffi::c_uint
                != 0 as core::ffi::c_uint)
        {
            break;
        }
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioPcmMmapRead(
    mut card: *const AudioCard,
    mut rxMmapData: *const AudioMmapData,
) -> int32_t {
    let mut ret: int32_t = 0;
    let mut data: *mut PlatformData = 0 as *mut PlatformData;
    ret = (card.is_null() || rxMmapData.is_null()
        || ((*rxMmapData).memoryAddress).is_null()
        || (*rxMmapData).totalBufferFrames <= 0 as core::ffi::c_int) as core::ffi::c_int
        as int32_t;
    if ret != 0 {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: param is invalid.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioPcmMmapRead\0"))
                .as_ptr(),
            711 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    data = PlatformDataFromCard(card) as *mut PlatformData;
    if data.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: PlatformDataFromCard failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioPcmMmapRead\0"))
                .as_ptr(),
            717 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    ret = AudioMmapReadTransfer(card, rxMmapData);
    if ret != 0 {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: AudioMmapReadTransfer fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioPcmMmapRead\0"))
                .as_ptr(),
            723 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioRenderBuffInit(
    mut platformData: *mut PlatformData,
) -> int32_t {
    let mut ret: int32_t = 0;
    if platformData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input para is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioRenderBuffInit\0"))
                .as_ptr(),
            736 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if !((*platformData).renderBufInfo.virtAddr).is_null() {
        return HDF_SUCCESS as core::ffi::c_int as int32_t;
    }
    (*platformData).renderBufInfo.cirBufMax = MAX_BUFF_SIZE as uint32_t;
    (*platformData).renderBufInfo.phyAddr = 0 as core::ffi::c_ulong;
    ret = AudioDmaBufAlloc(platformData, AUDIO_RENDER_STREAM);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Dma Buf Alloc fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioRenderBuffInit\0"))
                .as_ptr(),
            749 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if ((*platformData).renderBufInfo.virtAddr).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: mem alloc failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioRenderBuffInit\0"))
                .as_ptr(),
            754 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    memset_s(
        (*platformData).renderBufInfo.virtAddr as *mut core::ffi::c_void,
        (*platformData).renderBufInfo.cirBufMax as size_t,
        0 as core::ffi::c_int,
        (*platformData).renderBufInfo.cirBufMax as size_t,
    );
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioRenderBuffFree(
    mut platformData: *mut PlatformData,
) -> int32_t {
    let mut ret: int32_t = 0;
    if platformData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input param is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioRenderBuffFree\0"))
                .as_ptr(),
            769 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    ret = AudioDmaBufFree(platformData, AUDIO_RENDER_STREAM);
    if ret != 0 {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Dma Buf Alloc fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioRenderBuffFree\0"))
                .as_ptr(),
            775 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*platformData).renderBufInfo.virtAddr = 0 as *mut uint32_t;
    (*platformData).renderBufInfo.phyAddr = 0 as core::ffi::c_ulong;
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioCaptureBuffInit(
    mut platformData: *mut PlatformData,
) -> int32_t {
    let mut ret: int32_t = 0;
    if platformData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input param is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"AudioCaptureBuffInit\0"))
                .as_ptr(),
            789 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if !((*platformData).captureBufInfo.virtAddr).is_null() {
        return HDF_SUCCESS as core::ffi::c_int as int32_t;
    }
    (*platformData).captureBufInfo.cirBufMax = MAX_BUFF_SIZE as uint32_t;
    (*platformData).captureBufInfo.phyAddr = 0 as core::ffi::c_ulong;
    ret = AudioDmaBufAlloc(platformData, AUDIO_CAPTURE_STREAM);
    if ret != 0 {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Dma Buf Alloc fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"AudioCaptureBuffInit\0"))
                .as_ptr(),
            801 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if ((*platformData).captureBufInfo.virtAddr).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: mem alloc failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"AudioCaptureBuffInit\0"))
                .as_ptr(),
            806 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    memset_s(
        (*platformData).captureBufInfo.virtAddr as *mut core::ffi::c_void,
        (*platformData).captureBufInfo.cirBufMax as size_t,
        0 as core::ffi::c_int,
        (*platformData).captureBufInfo.cirBufMax as size_t,
    );
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioCaptureBuffFree(
    mut platformData: *mut PlatformData,
) -> int32_t {
    let mut ret: int32_t = 0;
    if platformData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input param is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"AudioCaptureBuffFree\0"))
                .as_ptr(),
            821 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    ret = AudioDmaBufFree(platformData, AUDIO_CAPTURE_STREAM);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Dma Buf Alloc fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"AudioCaptureBuffFree\0"))
                .as_ptr(),
            827 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*platformData).captureBufInfo.virtAddr = 0 as *mut uint32_t;
    (*platformData).captureBufInfo.phyAddr = 0 as core::ffi::c_ulong;
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioRenderOpen(mut card: *const AudioCard) -> int32_t {
    let mut data: *mut PlatformData = PlatformDataFromCard(card) as *mut PlatformData;
    if data.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: PlatformDataFromCard failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"AudioRenderOpen\0"))
                .as_ptr(),
            840 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if ((*data).renderBufInfo.virtAddr).is_null() {
        if AudioRenderBuffInit(data) != HDF_SUCCESS as core::ffi::c_int {
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioRenderClose(mut card: *const AudioCard) -> int32_t {
    let mut data: *mut PlatformData = PlatformDataFromCard(card) as *mut PlatformData;
    if data.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: PlatformDataFromCard failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioRenderClose\0"))
                .as_ptr(),
            856 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return AudioRenderBuffFree(data);
}
#[no_mangle]
pub unsafe extern "C" fn AudioCaptureOpen(mut card: *const AudioCard) -> int32_t {
    let mut platformData: *mut PlatformData = 0 as *mut PlatformData;
    if card.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: capture open param card is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioCaptureOpen\0"))
                .as_ptr(),
            867 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    platformData = PlatformDataFromCard(card) as *mut PlatformData;
    if platformData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: PlatformDataFromCard failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioCaptureOpen\0"))
                .as_ptr(),
            873 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if ((*platformData).captureBufInfo.virtAddr).is_null() {
        if AudioCaptureBuffInit(platformData) != HDF_SUCCESS as core::ffi::c_int {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: AudioCaptureBuffInit: fail.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"AudioCaptureOpen\0"))
                    .as_ptr(),
                879 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioCaptureClose(mut card: *const AudioCard) -> int32_t {
    let mut platformData: *mut PlatformData = 0 as *mut PlatformData;
    if card.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: capture close param card is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"AudioCaptureClose\0"))
                .as_ptr(),
            890 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    platformData = PlatformDataFromCard(card) as *mut PlatformData;
    if platformData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: PlatformDataFromCard failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"AudioCaptureClose\0"))
                .as_ptr(),
            896 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return AudioCaptureBuffFree(platformData);
}
unsafe extern "C" fn AudioPcmPending(
    mut card: *mut AudioCard,
    mut streamType: AudioStreamType,
) -> int32_t {
    let mut data: *mut PlatformData = PlatformDataFromCard(card) as *mut PlatformData;
    if data.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: PlatformDataFromCard failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"AudioPcmPending\0"))
                .as_ptr(),
            906 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioDmaSubmit(data, streamType) != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: DmaPending fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"AudioPcmPending\0"))
                .as_ptr(),
            911 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioDmaPending(data, streamType) != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: DmaPending fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"AudioPcmPending\0"))
                .as_ptr(),
            916 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioSampPowerUp(card) != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: PowerUp fail.\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"AudioPcmPending\0"))
                .as_ptr(),
            921 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioSampSetPowerMonitor(card, false_0 != 0) != HDF_SUCCESS as core::ffi::c_int {
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioPcmPause(
    mut card: *mut AudioCard,
    mut streamType: AudioStreamType,
) -> int32_t {
    let mut data: *mut PlatformData = PlatformDataFromCard(card) as *mut PlatformData;
    if data.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: PlatformDataFromCard failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"AudioPcmPause\0"))
                .as_ptr(),
            935 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioDmaPause(data, streamType) != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: DmaPause fail.\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"AudioPcmPause\0"))
                .as_ptr(),
            940 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioSampSetPowerMonitor(card, true_0 != 0) != HDF_SUCCESS as core::ffi::c_int {
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioPcmResume(
    mut card: *mut AudioCard,
    mut streamType: AudioStreamType,
) -> int32_t {
    let mut data: *mut PlatformData = PlatformDataFromCard(card) as *mut PlatformData;
    if data.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: PlatformDataFromCard failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"AudioPcmResume\0"))
                .as_ptr(),
            955 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioDmaResume(data, streamType) != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: DmaPause fail.\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"AudioPcmResume\0"))
                .as_ptr(),
            960 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioSampPowerUp(card) != HDF_SUCCESS as core::ffi::c_int {
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioSampSetPowerMonitor(card, false_0 != 0) != HDF_SUCCESS as core::ffi::c_int {
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioRenderTrigger(
    mut card: *mut AudioCard,
    mut cmd: int32_t,
) -> int32_t {
    let mut data: *mut PlatformData = PlatformDataFromCard(card) as *mut PlatformData;
    if data.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: PlatformDataFromCard failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioRenderTrigger\0"))
                .as_ptr(),
            978 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    match cmd {
        5 => {
            if AudioPcmPending(card, AUDIO_RENDER_STREAM)
                != HDF_SUCCESS as core::ffi::c_int
            {
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    LOG_DOMAIN as core::ffi::c_uint,
                    b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s][line:%d]: AudioPcmPending fail.\0" as *const u8
                        as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 19],
                        [core::ffi::c_char; 19],
                    >(*b"AudioRenderTrigger\0"))
                        .as_ptr(),
                    984 as core::ffi::c_int,
                );
                return HDF_FAILURE as core::ffi::c_int as int32_t;
            }
            (*data).renderBufInfo.runStatus = PCM_START;
        }
        6 => {
            if AudioPcmPause(card, AUDIO_RENDER_STREAM)
                != HDF_SUCCESS as core::ffi::c_int
            {
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    LOG_DOMAIN as core::ffi::c_uint,
                    b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s][line:%d]: AudioPcmPause fail.\0" as *const u8
                        as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 19],
                        [core::ffi::c_char; 19],
                    >(*b"AudioRenderTrigger\0"))
                        .as_ptr(),
                    992 as core::ffi::c_int,
                );
                return HDF_FAILURE as core::ffi::c_int as int32_t;
            }
            (*data).renderBufInfo.runStatus = PCM_STOP;
        }
        9 => {
            if AudioPcmPause(card, AUDIO_RENDER_STREAM)
                != HDF_SUCCESS as core::ffi::c_int
            {
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    LOG_DOMAIN as core::ffi::c_uint,
                    b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s][line:%d]: AudioPcmPause fail.\0" as *const u8
                        as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 19],
                        [core::ffi::c_char; 19],
                    >(*b"AudioRenderTrigger\0"))
                        .as_ptr(),
                    1000 as core::ffi::c_int,
                );
                return HDF_FAILURE as core::ffi::c_int as int32_t;
            }
            (*data).renderBufInfo.runStatus = PCM_PAUSE;
        }
        11 => {
            if AudioPcmResume(card, AUDIO_RENDER_STREAM)
                != HDF_SUCCESS as core::ffi::c_int
            {
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    LOG_DOMAIN as core::ffi::c_uint,
                    b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s][line:%d]: AudioPcmResume fail.\0" as *const u8
                        as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 19],
                        [core::ffi::c_char; 19],
                    >(*b"AudioRenderTrigger\0"))
                        .as_ptr(),
                    1008 as core::ffi::c_int,
                );
                return HDF_FAILURE as core::ffi::c_int as int32_t;
            }
            (*data).renderBufInfo.runStatus = PCM_START;
        }
        _ => {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: invalude cmd id: %d.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 19],
                    [core::ffi::c_char; 19],
                >(*b"AudioRenderTrigger\0"))
                    .as_ptr(),
                1015 as core::ffi::c_int,
                cmd,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioCaptureTrigger(
    mut card: *mut AudioCard,
    mut cmd: int32_t,
) -> int32_t {
    let mut data: *mut PlatformData = PlatformDataFromCard(card) as *mut PlatformData;
    if data.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: PlatformDataFromCard failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioCaptureTrigger\0"))
                .as_ptr(),
            1025 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    match cmd {
        7 => {
            if AudioPcmPending(card, AUDIO_CAPTURE_STREAM)
                != HDF_SUCCESS as core::ffi::c_int
            {
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    LOG_DOMAIN as core::ffi::c_uint,
                    b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s][line:%d]: AudioPcmPending fail.\0" as *const u8
                        as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 20],
                        [core::ffi::c_char; 20],
                    >(*b"AudioCaptureTrigger\0"))
                        .as_ptr(),
                    1032 as core::ffi::c_int,
                );
                return HDF_FAILURE as core::ffi::c_int as int32_t;
            }
            (*data).captureBufInfo.runStatus = PCM_START;
        }
        8 => {
            if AudioPcmPause(card, AUDIO_CAPTURE_STREAM)
                != HDF_SUCCESS as core::ffi::c_int
            {
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    LOG_DOMAIN as core::ffi::c_uint,
                    b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s][line:%d]: AudioPcmPause fail.\0" as *const u8
                        as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 20],
                        [core::ffi::c_char; 20],
                    >(*b"AudioCaptureTrigger\0"))
                        .as_ptr(),
                    1040 as core::ffi::c_int,
                );
                return HDF_FAILURE as core::ffi::c_int as int32_t;
            }
            (*data).captureBufInfo.runStatus = PCM_STOP;
        }
        10 => {
            if AudioPcmPause(card, AUDIO_CAPTURE_STREAM)
                != HDF_SUCCESS as core::ffi::c_int
            {
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    LOG_DOMAIN as core::ffi::c_uint,
                    b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s][line:%d]: AudioPcmPause fail.\0" as *const u8
                        as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 20],
                        [core::ffi::c_char; 20],
                    >(*b"AudioCaptureTrigger\0"))
                        .as_ptr(),
                    1048 as core::ffi::c_int,
                );
                return HDF_FAILURE as core::ffi::c_int as int32_t;
            }
            (*data).captureBufInfo.runStatus = PCM_PAUSE;
        }
        12 => {
            if AudioPcmResume(card, AUDIO_CAPTURE_STREAM)
                != HDF_SUCCESS as core::ffi::c_int
            {
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    LOG_DOMAIN as core::ffi::c_uint,
                    b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s][line:%d]: AudioPcmResume fail.\0" as *const u8
                        as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 20],
                        [core::ffi::c_char; 20],
                    >(*b"AudioCaptureTrigger\0"))
                        .as_ptr(),
                    1056 as core::ffi::c_int,
                );
                return HDF_FAILURE as core::ffi::c_int as int32_t;
            }
            (*data).captureBufInfo.runStatus = PCM_START;
        }
        _ => {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: invalude cmd id: %d.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 20],
                    [core::ffi::c_char; 20],
                >(*b"AudioCaptureTrigger\0"))
                    .as_ptr(),
                1063 as core::ffi::c_int,
                cmd,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioDmaConfig(
    mut data: *mut PlatformData,
    mut streamType: AudioStreamType,
) -> int32_t {
    let mut ret: int32_t = 0;
    if data.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: platform is invalid.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"AudioDmaConfig\0"))
                .as_ptr(),
            1073 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    if streamType as core::ffi::c_uint
        == AUDIO_RENDER_STREAM as core::ffi::c_int as core::ffi::c_uint
    {
        ret = AudioDmaConfigChannel(data, AUDIO_RENDER_STREAM);
        if ret != 0 {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: Dma Config Channel fail.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 15],
                    [core::ffi::c_char; 15],
                >(*b"AudioDmaConfig\0"))
                    .as_ptr(),
                1080 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    } else if streamType as core::ffi::c_uint
        == AUDIO_CAPTURE_STREAM as core::ffi::c_int as core::ffi::c_uint
    {
        ret = AudioDmaConfigChannel(data, AUDIO_CAPTURE_STREAM);
        if ret != 0 {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: Dma Config Channel fail.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 15],
                    [core::ffi::c_char; 15],
                >(*b"AudioDmaConfig\0"))
                    .as_ptr(),
                1086 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    } else {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: streamType(%d) is invalid.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"AudioDmaConfig\0"))
                .as_ptr(),
            1090 as core::ffi::c_int,
            streamType as core::ffi::c_uint,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioPcmTransferBytes(
    mut data: *mut PlatformData,
    mut streamType: AudioStreamType,
) -> int32_t {
    if data.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: param is NULL.\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioPcmTransferBytes\0"))
                .as_ptr(),
            1099 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    if streamType as core::ffi::c_uint
        == AUDIO_RENDER_STREAM as core::ffi::c_int as core::ffi::c_uint
    {
        (*data).renderBufInfo.oneMsBytes = ((*data).renderPcmInfo.rate
            as core::ffi::c_uint)
            .wrapping_mul((*data).renderPcmInfo.frameSize as core::ffi::c_uint)
            .wrapping_div(SECOND_TO_MILLISECOND as core::ffi::c_uint) as uint32_t;
        if (*data).renderBufInfo.oneMsBytes == 0 as core::ffi::c_uint {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: render pcm info is error.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 22],
                    [core::ffi::c_char; 22],
                >(*b"AudioPcmTransferBytes\0"))
                    .as_ptr(),
                1107 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    } else if streamType as core::ffi::c_uint
        == AUDIO_CAPTURE_STREAM as core::ffi::c_int as core::ffi::c_uint
    {
        (*data).captureBufInfo.oneMsBytes = ((*data).capturePcmInfo.rate
            as core::ffi::c_uint)
            .wrapping_mul((*data).capturePcmInfo.frameSize as core::ffi::c_uint)
            .wrapping_div(SECOND_TO_MILLISECOND as core::ffi::c_uint) as uint32_t;
        if (*data).captureBufInfo.oneMsBytes == 0 as core::ffi::c_uint {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: capture pcm info is error.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 22],
                    [core::ffi::c_char; 22],
                >(*b"AudioPcmTransferBytes\0"))
                    .as_ptr(),
                1115 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    } else {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: streamType(%d) is invalid.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioPcmTransferBytes\0"))
                .as_ptr(),
            1120 as core::ffi::c_int,
            streamType as core::ffi::c_uint,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioSetRenderHwParams(
    mut param: *const AudioPcmHwParams,
    mut data: *mut PlatformData,
) -> int32_t {
    if param.is_null() || data.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input param is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioSetRenderHwParams\0"))
                .as_ptr(),
            1130 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    if AudioDmaRequestChannel(data, AUDIO_RENDER_STREAM)
        != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Dma Request Channel fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioSetRenderHwParams\0"))
                .as_ptr(),
            1135 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioSetRenderBufInfo(data, param) != HDF_SUCCESS as core::ffi::c_int {
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioDmaConfig(data, AUDIO_RENDER_STREAM) != HDF_SUCCESS as core::ffi::c_int {
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioPcmTransferBytes(data, AUDIO_RENDER_STREAM)
        != HDF_SUCCESS as core::ffi::c_int
    {
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioSetCaptureHwParams(
    mut param: *const AudioPcmHwParams,
    mut data: *mut PlatformData,
) -> int32_t {
    if param.is_null() || data.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input param is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"AudioSetCaptureHwParams\0"))
                .as_ptr(),
            1157 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    if AudioDmaRequestChannel(data, AUDIO_CAPTURE_STREAM)
        != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Dma Request Channel fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"AudioSetCaptureHwParams\0"))
                .as_ptr(),
            1162 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioSetCaptureBufInfo(data, param) != HDF_SUCCESS as core::ffi::c_int {
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioDmaConfig(data, AUDIO_CAPTURE_STREAM) != HDF_SUCCESS as core::ffi::c_int {
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioPcmTransferBytes(data, AUDIO_CAPTURE_STREAM)
        != HDF_SUCCESS as core::ffi::c_int
    {
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioHwParams(
    mut card: *const AudioCard,
    mut param: *const AudioPcmHwParams,
) -> int32_t {
    let chnlCntMin: int32_t = 1 as int32_t;
    let chnlCntMax: int32_t = 2 as int32_t;
    let mut platformData: *mut PlatformData = 0 as *mut PlatformData;
    if card.is_null() || param.is_null() || ((*param).cardServiceName).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input param is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"AudioHwParams\0"))
                .as_ptr(),
            1188 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    if (*param).channels < chnlCntMin as core::ffi::c_uint
        || (*param).channels > chnlCntMax as core::ffi::c_uint
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: channels param is invalid.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"AudioHwParams\0"))
                .as_ptr(),
            1193 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    platformData = PlatformDataFromCard(card) as *mut PlatformData;
    if platformData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: platformData is null.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"AudioHwParams\0"))
                .as_ptr(),
            1199 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioSetPcmInfo(platformData, param) != HDF_SUCCESS as core::ffi::c_int {
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if (*param).streamType as core::ffi::c_uint
        == AUDIO_RENDER_STREAM as core::ffi::c_int as core::ffi::c_uint
    {
        if AudioSetRenderHwParams(param, platformData) != HDF_SUCCESS as core::ffi::c_int
        {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: set render hardware params is failed.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 14],
                    [core::ffi::c_char; 14],
                >(*b"AudioHwParams\0"))
                    .as_ptr(),
                1209 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    } else if (*param).streamType as core::ffi::c_uint
        == AUDIO_CAPTURE_STREAM as core::ffi::c_int as core::ffi::c_uint
    {
        if AudioSetCaptureHwParams(param, platformData)
            != HDF_SUCCESS as core::ffi::c_int
        {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: set capture hardware params is failed.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 14],
                    [core::ffi::c_char; 14],
                >(*b"AudioHwParams\0"))
                    .as_ptr(),
                1214 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    } else {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: param streamType is invalid.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"AudioHwParams\0"))
                .as_ptr(),
            1218 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioRenderPrepare(mut card: *const AudioCard) -> int32_t {
    let mut ret: int32_t = 0;
    let mut platformData: *mut PlatformData = 0 as *mut PlatformData;
    if card.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: audio render param card is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioRenderPrepare\0"))
                .as_ptr(),
            1230 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    platformData = PlatformDataFromCard(card) as *mut PlatformData;
    if platformData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: audio render PlatformDataFromCard failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioRenderPrepare\0"))
                .as_ptr(),
            1236 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if ((*platformData).renderBufInfo.virtAddr).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: render buff is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioRenderPrepare\0"))
                .as_ptr(),
            1241 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    memset_s(
        (*platformData).renderBufInfo.virtAddr as *mut core::ffi::c_void,
        (*platformData).renderBufInfo.cirBufSize as size_t,
        0 as core::ffi::c_int,
        (*platformData).renderBufInfo.cirBufSize as size_t,
    );
    (*platformData).renderBufInfo.wbufOffSet = 0 as uint32_t;
    (*platformData).renderBufInfo.wptrOffSet = 0 as uint32_t;
    (*platformData).renderBufInfo.framesPosition = 0 as uint32_t;
    (*platformData).renderBufInfo.pointer = 0 as uint32_t;
    (*platformData).renderPcmInfo.totalStreamSize = 0 as uint32_t;
    (*platformData).renderBufInfo.rbufOffSet = 0 as uint32_t;
    (*platformData).renderBufInfo.trafCompCount = 0 as uint32_t;
    ret = AudioDmaPrep(platformData, AUDIO_RENDER_STREAM);
    if ret != 0 {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Dma prepare fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioRenderPrepare\0"))
                .as_ptr(),
            1256 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioCapturePrepare(mut card: *const AudioCard) -> int32_t {
    let mut ret: int32_t = 0;
    let mut platformData: *mut PlatformData = 0 as *mut PlatformData;
    if card.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: audio capture param card is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioCapturePrepare\0"))
                .as_ptr(),
            1268 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    platformData = PlatformDataFromCard(card) as *mut PlatformData;
    if platformData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: audio capture PlatformDataFromCard failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioCapturePrepare\0"))
                .as_ptr(),
            1274 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    if ((*platformData).captureBufInfo.virtAddr).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: capture buff is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioCapturePrepare\0"))
                .as_ptr(),
            1279 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    memset_s(
        (*platformData).captureBufInfo.virtAddr as *mut core::ffi::c_void,
        (*platformData).captureBufInfo.cirBufSize as size_t,
        0 as core::ffi::c_int,
        (*platformData).captureBufInfo.cirBufSize as size_t,
    );
    (*platformData).captureBufInfo.rbufOffSet = 0 as uint32_t;
    (*platformData).captureBufInfo.rptrOffSet = 0 as uint32_t;
    (*platformData).captureBufInfo.chnId = 0 as uint32_t;
    (*platformData).captureBufInfo.framesPosition = 0 as uint32_t;
    (*platformData).captureBufInfo.pointer = 0 as uint32_t;
    (*platformData).capturePcmInfo.totalStreamSize = 0 as uint32_t;
    (*platformData).captureBufInfo.wbufOffSet = 0 as uint32_t;
    (*platformData).captureBufInfo.trafCompCount = 0 as uint32_t;
    ret = AudioDmaPrep(platformData, AUDIO_CAPTURE_STREAM);
    if ret != 0 {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Dma prepare fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioCapturePrepare\0"))
                .as_ptr(),
            1295 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioPcmPointer(
    mut card: *const AudioCard,
    mut pointer: *mut uint32_t,
    mut streamType: AudioStreamType,
) -> int32_t {
    let mut ret: int32_t = 0;
    let mut data: *mut PlatformData = 0 as *mut PlatformData;
    if card.is_null() || pointer.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: param card is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"AudioPcmPointer\0"))
                .as_ptr(),
            1307 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    data = PlatformDataFromCard(card) as *mut PlatformData;
    if data.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: PlatformDataFromCard failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"AudioPcmPointer\0"))
                .as_ptr(),
            1313 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    ret = AudioDmaPointer(data, streamType, pointer);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Dma Pointer fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"AudioPcmPointer\0"))
                .as_ptr(),
            1319 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioCapSilenceThresholdEvent(
    mut device: *mut HdfDeviceObject,
    mut reportMsg: *const AudioEvent,
) -> int32_t {
    let mut ret: int32_t = 0;
    let mut msgBuf: [core::ffi::c_char; 256] = [
        0 as core::ffi::c_int as core::ffi::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    if device.is_null() || reportMsg.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: device is NULL!\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"AudioCapSilenceThresholdEvent\0"))
                .as_ptr(),
            1332 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    ret = snprintf_s(
        msgBuf.as_mut_ptr(),
        AUDIO_PNP_MSG_LEN as size_t,
        (AUDIO_PNP_MSG_LEN - 1 as core::ffi::c_int) as size_t,
        b"EVENT_TYPE=0x%x;DEVICE_TYPE=0x%x\0" as *const u8 as *const core::ffi::c_char,
        (*reportMsg).eventType,
        (*reportMsg).deviceType,
    ) as int32_t;
    if ret < 0 as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: snprintf_s fail\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"AudioCapSilenceThresholdEvent\0"))
                .as_ptr(),
            1339 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if HdfDeviceObjectSetServInfo(device, msgBuf.as_mut_ptr())
        != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: HdfDeviceObjectSetServInfo fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"AudioCapSilenceThresholdEvent\0"))
                .as_ptr(),
            1343 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    ret = HdfDeviceObjectUpdate(device) as int32_t;
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: HdfDeviceObjectUpdate fail\n\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"AudioCapSilenceThresholdEvent\0"))
                .as_ptr(),
            1349 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
pub const LOG_DOMAIN: core::ffi::c_int = 0xd002510 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn CopyToUser(
    mut to: *mut core::ffi::c_void,
    mut from: *const core::ffi::c_void,
    mut len: size_t,
) -> size_t {
    return LOS_ArchCopyToUser(to, from, len);
}
#[inline]
unsafe extern "C" fn CopyFromUser(
    mut to: *mut core::ffi::c_void,
    mut from: *const core::ffi::c_void,
    mut len: size_t,
) -> size_t {
    return LOS_ArchCopyFromUser(to, from, len);
}
