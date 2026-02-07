extern "C" {
    pub type HdfSBuf;
    fn strcmp(
        _: *const core::ffi::c_char,
        _: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn strlen(_: *const core::ffi::c_char) -> size_t;
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
    fn AudioCodecRegUpdate(
        codec: *mut CodecDevice,
        mixerCtrl: *mut AudioMixerControl,
    ) -> int32_t;
    fn AudioCodecMuxRegUpdate(
        codec: *mut CodecDevice,
        enumCtrl: *mut AudioEnumKcontrol,
        value: *const uint32_t,
    ) -> int32_t;
    fn AudioUpdateCodecRegBits(
        codec: *mut CodecDevice,
        reg: uint32_t,
        mask: uint32_t,
        shift: uint32_t,
        value: uint32_t,
    ) -> int32_t;
    fn AudioKcontrolGetCodec(kcontrol: *const AudioKcontrol) -> *mut CodecDevice;
    fn AudioAddControl(
        audioCard: *const AudioCard,
        ctl: *const AudioKcontrol,
    ) -> *mut AudioKcontrol;
    fn AudioCodecReadReg(
        codec: *const CodecDevice,
        reg: uint32_t,
        val: *mut uint32_t,
    ) -> int32_t;
    fn AudioCodecGetCtrlOps(
        kcontrol: *const AudioKcontrol,
        elemValue: *mut AudioCtrlElemValue,
    ) -> int32_t;
    fn AudioCodecGetEnumCtrlOps(
        kcontrol: *const AudioKcontrol,
        elemValue: *mut AudioCtrlElemValue,
    ) -> int32_t;
    fn OsalSleep(sec: uint32_t);
    fn OsalThreadCreate(
        thread: *mut OsalThread,
        threadEntry: OsalThreadEntry,
        entryPara: *mut core::ffi::c_void,
    ) -> int32_t;
    fn OsalThreadStart(
        thread: *mut OsalThread,
        param: *const OsalThreadParam,
    ) -> int32_t;
    fn OsalThreadDestroy(thread: *mut OsalThread) -> int32_t;
}
pub type size_t = core::ffi::c_uint;
pub type uintptr_t = core::ffi::c_uint;
pub type int8_t = core::ffi::c_schar;
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
pub struct AudioCtrlElemValue {
    pub id: AudioCtrlElemId,
    pub value: [uint32_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioEnumKcontrol {
    pub reg: uint32_t,
    pub reg2: uint32_t,
    pub shiftLeft: uint8_t,
    pub shiftRight: uint8_t,
    pub max: uint32_t,
    pub mask: uint32_t,
    pub texts: *const *const core::ffi::c_char,
    pub values: *const uint32_t,
    pub sapm: *mut core::ffi::c_void,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioSapmpath {
    pub name: *mut core::ffi::c_char,
    pub source: *mut AudioSapmComponent,
    pub sink: *mut AudioSapmComponent,
    pub kcontrol: *mut AudioKcontrol,
    pub connect: uint8_t,
    pub walked: uint8_t,
    pub weak: uint8_t,
    pub connected: Option<
        unsafe extern "C" fn(*mut AudioSapmComponent, *mut AudioSapmComponent) -> int32_t,
    >,
    pub listSource: DListHead,
    pub listSink: DListHead,
    pub list: DListHead,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OsalThread {
    pub realThread: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OsalThreadParam {
    pub name: *mut core::ffi::c_char,
    pub stackSize: size_t,
    pub priority: OSAL_THREAD_PRIORITY,
    pub policy: core::ffi::c_int,
}
pub type OSAL_THREAD_PRIORITY = core::ffi::c_uint;
pub const OSAL_THREAD_PRI_HIGHEST: OSAL_THREAD_PRIORITY = 3;
pub const OSAL_THREAD_PRI_HIGH: OSAL_THREAD_PRIORITY = 2;
pub const OSAL_THREAD_PRI_DEFAULT: OSAL_THREAD_PRIORITY = 1;
pub const OSAL_THREAD_PRI_LOW: OSAL_THREAD_PRIORITY = 0;
pub type OsalThreadEntry = Option<
    unsafe extern "C" fn(*mut core::ffi::c_void) -> core::ffi::c_int,
>;
pub const LOG_DOMAIN: core::ffi::c_int = 0xd002510 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[inline]
unsafe extern "C" fn DListHeadInit(mut head: *mut DListHead) {
    (*head).next = head;
    (*head).prev = head;
}
#[inline]
unsafe extern "C" fn DListIsEmpty(mut head: *const DListHead) -> bool {
    return if (*head).next == head as *mut DListHead { true_0 } else { false_0 } != 0;
}
#[inline]
unsafe extern "C" fn DListRemove(mut entry: *mut DListHead) {
    (*(*entry).prev).next = (*entry).next;
    (*(*entry).next).prev = (*entry).prev;
    (*entry).prev = 0 as *mut DListHead;
    (*entry).next = 0 as *mut DListHead;
}
#[inline]
unsafe extern "C" fn DListInsertHead(
    mut entry: *mut DListHead,
    mut head: *mut DListHead,
) {
    (*entry).next = (*head).next;
    (*entry).prev = head;
    (*(*head).next).prev = entry;
    (*head).next = entry;
}
#[inline]
unsafe extern "C" fn DListInsertTail(
    mut entry: *mut DListHead,
    mut head: *mut DListHead,
) {
    (*entry).next = head;
    (*entry).prev = (*head).prev;
    (*(*head).prev).next = entry;
    (*head).prev = entry;
}
pub const EOK: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AUDIO_NO_SAPM_REG: core::ffi::c_int = 0xffff as core::ffi::c_int;
pub const SAPM_POLL_TIME: core::ffi::c_int = 10 as core::ffi::c_int;
pub const SAPM_SLEEP_TIMES: core::ffi::c_int = 3 as core::ffi::c_int
    * 60 as core::ffi::c_int / 10 as core::ffi::c_int;
pub const SAPM_THREAD_NAME: core::ffi::c_int = 60 as core::ffi::c_int;
pub const SAPM_POWER_DOWN: core::ffi::c_int = 0 as core::ffi::c_int;
pub const SAPM_POWER_UP: core::ffi::c_int = 1 as core::ffi::c_int;
pub const SAPM_STACK_SIZE: core::ffi::c_int = 10000 as core::ffi::c_int;
pub const CONNECT_CODEC_PIN: core::ffi::c_int = 1 as core::ffi::c_int;
pub const EXIST_EXTERNAL_WIDGET: core::ffi::c_int = 1 as core::ffi::c_int;
pub const CONNECT_SINK_AND_SOURCE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const UNCONNECT_SINK_AND_SOURCE: core::ffi::c_int = 0 as core::ffi::c_int;
#[no_mangle]
pub static mut g_cardNum: uint32_t = 0 as uint32_t;
static mut g_audioSapmPowerUpSeq: [int32_t; 26] = [
    0,
    0,
    5 as core::ffi::c_int,
    0,
    5 as core::ffi::c_int,
    5 as core::ffi::c_int,
    7 as core::ffi::c_int,
    7 as core::ffi::c_int,
    8 as core::ffi::c_int,
    10 as core::ffi::c_int,
    9 as core::ffi::c_int,
    6 as core::ffi::c_int,
    2 as core::ffi::c_int,
    4 as core::ffi::c_int,
    10 as core::ffi::c_int,
    10 as core::ffi::c_int,
    0,
    0,
    0,
    0 as core::ffi::c_int,
    11 as core::ffi::c_int,
    1 as core::ffi::c_int,
    0,
    0,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
];
static mut g_audioSapmPowerDownSeq: [int32_t; 26] = [
    0,
    0,
    9 as core::ffi::c_int,
    0,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    5 as core::ffi::c_int,
    5 as core::ffi::c_int,
    4 as core::ffi::c_int,
    2 as core::ffi::c_int,
    1 as core::ffi::c_int,
    6 as core::ffi::c_int,
    8 as core::ffi::c_int,
    7 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    0,
    0,
    0,
    0 as core::ffi::c_int,
    12 as core::ffi::c_int,
    11 as core::ffi::c_int,
    0,
    0,
    10 as core::ffi::c_int,
    10 as core::ffi::c_int,
];
unsafe extern "C" fn ConnectedInputEndPoint(
    mut sapmComponent: *const AudioSapmComponent,
) -> int32_t {
    let mut path: *mut AudioSapmpath = 0 as *mut AudioSapmpath;
    let mut count: int32_t = 0 as int32_t;
    let endPointVal: int32_t = 1 as int32_t;
    if sapmComponent.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input param sapmComponent is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"ConnectedInputEndPoint\0"))
                .as_ptr(),
            88 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    match (*sapmComponent).sapmType as core::ffi::c_uint {
        11 | 24 | 0 | 13 | 16 => return endPointVal,
        _ => {}
    }
    path = ((*sapmComponent).sources.next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut AudioSapmpath)).listSink as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_int as *mut AudioSapmpath;
    while &mut (*path).listSink as *mut DListHead
        != &(*sapmComponent).sources as *const DListHead as *mut DListHead
    {
        if !((*path).source).is_null()
            && (*path).connect as core::ffi::c_int == CONNECT_SINK_AND_SOURCE
        {
            count += ConnectedInputEndPoint((*path).source) as core::ffi::c_int;
        }
        path = ((*path).listSink.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut AudioSapmpath)).listSink as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_int as *mut AudioSapmpath;
    }
    return count;
}
unsafe extern "C" fn ConnectedOutputEndPoint(
    mut sapmComponent: *const AudioSapmComponent,
) -> int32_t {
    let mut path: *mut AudioSapmpath = 0 as *mut AudioSapmpath;
    let mut count: int32_t = 0 as int32_t;
    let endPointVal: int32_t = 1 as int32_t;
    if sapmComponent.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input param sapmComponent is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"ConnectedOutputEndPoint\0"))
                .as_ptr(),
            118 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    match (*sapmComponent).sapmType as core::ffi::c_uint {
        10 | 25 | 1 | 14 | 15 | 16 => return endPointVal,
        _ => {}
    }
    path = ((*sapmComponent).sinks.next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut AudioSapmpath)).listSource as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_int as *mut AudioSapmpath;
    while &mut (*path).listSource as *mut DListHead
        != &(*sapmComponent).sinks as *const DListHead as *mut DListHead
    {
        if !((*path).sink).is_null()
            && (*path).connect as core::ffi::c_int == CONNECT_SINK_AND_SOURCE
        {
            count += ConnectedOutputEndPoint((*path).sink) as core::ffi::c_int;
        }
        path = ((*path).listSource.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut AudioSapmpath)).listSource as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_int as *mut AudioSapmpath;
    }
    return count;
}
unsafe extern "C" fn AudioSapmGenericCheckPower(
    mut sapmComponent: *const AudioSapmComponent,
) -> int32_t {
    let mut input: int32_t = 0;
    let mut output: int32_t = 0;
    if sapmComponent.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input param cpt is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"AudioSapmGenericCheckPower\0"))
                .as_ptr(),
            148 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    input = ConnectedInputEndPoint(sapmComponent);
    if input == HDF_FAILURE as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input endpoint fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"AudioSapmGenericCheckPower\0"))
                .as_ptr(),
            154 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    output = ConnectedOutputEndPoint(sapmComponent);
    if output == HDF_FAILURE as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: output endpoint fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"AudioSapmGenericCheckPower\0"))
                .as_ptr(),
            159 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if input == 0 as core::ffi::c_int || output == 0 as core::ffi::c_int {
        return SAPM_POWER_DOWN;
    }
    return SAPM_POWER_UP;
}
unsafe extern "C" fn AudioSapmAdcPowerClock(
    mut sapmComponent: *mut AudioSapmComponent,
) -> int32_t {
    if sapmComponent.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: param sapmComponent is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioSapmAdcPowerClock\0"))
                .as_ptr(),
            173 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        LOG_DOMAIN as core::ffi::c_uint,
        b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
        b"[%s][line:%d]: %s standby mode entry!\0" as *const u8
            as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 23],
            [core::ffi::c_char; 23],
        >(*b"AudioSapmAdcPowerClock\0"))
            .as_ptr(),
        177 as core::ffi::c_int,
        (*sapmComponent).componentName,
    );
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioSapmDacPowerClock(
    mut sapmComponent: *mut AudioSapmComponent,
) -> int32_t {
    if sapmComponent.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: param sapmComponent is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioSapmDacPowerClock\0"))
                .as_ptr(),
            184 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        LOG_DOMAIN as core::ffi::c_uint,
        b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
        b"[%s][line:%d]: %s standby mode entry!\0" as *const u8
            as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 23],
            [core::ffi::c_char; 23],
        >(*b"AudioSapmDacPowerClock\0"))
            .as_ptr(),
        188 as core::ffi::c_int,
        (*sapmComponent).componentName,
    );
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioSapmAdcCheckPower(
    mut sapmComponent: *const AudioSapmComponent,
) -> int32_t {
    let mut input: int32_t = 0;
    if sapmComponent.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input param sapmComponent is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioSapmAdcCheckPower\0"))
                .as_ptr(),
            197 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if (*sapmComponent).active as core::ffi::c_int == 0 as core::ffi::c_int {
        input = AudioSapmGenericCheckPower(sapmComponent);
    } else {
        input = ConnectedInputEndPoint(sapmComponent);
    }
    if input == HDF_FAILURE as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input endpoint fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioSapmAdcCheckPower\0"))
                .as_ptr(),
            207 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return input;
}
unsafe extern "C" fn AudioSapmDacCheckPower(
    mut sapmComponent: *const AudioSapmComponent,
) -> int32_t {
    let mut output: int32_t = 0;
    if sapmComponent.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input sapmComponent cpt is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioSapmDacCheckPower\0"))
                .as_ptr(),
            218 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if (*sapmComponent).active as core::ffi::c_int == 0 as core::ffi::c_int {
        output = AudioSapmGenericCheckPower(sapmComponent);
    } else {
        output = ConnectedOutputEndPoint(sapmComponent);
    }
    if output == HDF_FAILURE as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: output endpoint fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioSapmDacCheckPower\0"))
                .as_ptr(),
            228 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return output;
}
unsafe extern "C" fn AudioSampCheckPowerCallback(
    mut sapmComponent: *mut AudioSapmComponent,
) {
    if sapmComponent.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input param cpt is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"AudioSampCheckPowerCallback\0"))
                .as_ptr(),
            237 as core::ffi::c_int,
        );
        return;
    }
    match (*sapmComponent).sapmType as core::ffi::c_uint {
        17 | 6 | 7 => {
            (*sapmComponent).PowerCheck = Some(
                AudioSapmGenericCheckPower
                    as unsafe extern "C" fn(*const AudioSapmComponent) -> int32_t,
            ) as Option<unsafe extern "C" fn(*const AudioSapmComponent) -> int32_t>;
        }
        2 | 4 | 5 => {
            (*sapmComponent).PowerCheck = Some(
                AudioSapmGenericCheckPower
                    as unsafe extern "C" fn(*const AudioSapmComponent) -> int32_t,
            ) as Option<unsafe extern "C" fn(*const AudioSapmComponent) -> int32_t>;
        }
        10 | 25 => {
            (*sapmComponent).PowerCheck = Some(
                AudioSapmAdcCheckPower
                    as unsafe extern "C" fn(*const AudioSapmComponent) -> int32_t,
            ) as Option<unsafe extern "C" fn(*const AudioSapmComponent) -> int32_t>;
        }
        11 | 24 => {
            (*sapmComponent).PowerCheck = Some(
                AudioSapmDacCheckPower
                    as unsafe extern "C" fn(*const AudioSapmComponent) -> int32_t,
            ) as Option<unsafe extern "C" fn(*const AudioSapmComponent) -> int32_t>;
        }
        8 | 9 | 0 | 1 | 12 | 15 | 14 | 13 | 16 => {
            (*sapmComponent).PowerCheck = Some(
                AudioSapmGenericCheckPower
                    as unsafe extern "C" fn(*const AudioSapmComponent) -> int32_t,
            ) as Option<unsafe extern "C" fn(*const AudioSapmComponent) -> int32_t>;
        }
        _ => {
            (*sapmComponent).PowerCheck = Some(
                AudioSapmGenericCheckPower
                    as unsafe extern "C" fn(*const AudioSapmComponent) -> int32_t,
            ) as Option<unsafe extern "C" fn(*const AudioSapmComponent) -> int32_t>;
        }
    };
}
unsafe extern "C" fn AudioSampPowerClockCallback(
    mut sapmComponent: *mut AudioSapmComponent,
) {
    if sapmComponent.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input param cpt is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"AudioSampPowerClockCallback\0"))
                .as_ptr(),
            282 as core::ffi::c_int,
        );
        return;
    }
    match (*sapmComponent).sapmType as core::ffi::c_uint {
        17 | 6 | 7 => {
            (*sapmComponent).PowerClockOp = None;
        }
        2 | 4 | 5 => {
            (*sapmComponent).PowerClockOp = None;
        }
        10 | 25 => {
            (*sapmComponent).PowerClockOp = Some(
                AudioSapmAdcPowerClock
                    as unsafe extern "C" fn(*mut AudioSapmComponent) -> int32_t,
            ) as Option<unsafe extern "C" fn(*mut AudioSapmComponent) -> int32_t>;
        }
        11 | 24 => {
            (*sapmComponent).PowerClockOp = Some(
                AudioSapmDacPowerClock
                    as unsafe extern "C" fn(*mut AudioSapmComponent) -> int32_t,
            ) as Option<unsafe extern "C" fn(*mut AudioSapmComponent) -> int32_t>;
        }
        8 | 9 | 0 | 1 | 12 | 13 | 15 | 14 | 16 => {
            (*sapmComponent).PowerClockOp = None;
        }
        _ => {
            (*sapmComponent).PowerClockOp = None;
        }
    };
}
unsafe extern "C" fn AudioSapmNewComponent(
    mut audioCard: *mut AudioCard,
    mut component: *const AudioSapmComponent,
) -> int32_t {
    let mut sapmComponent: *mut AudioSapmComponent = 0 as *mut AudioSapmComponent;
    if audioCard.is_null() || ((*audioCard).rtd).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params check error: audioCard is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioSapmNewComponent\0"))
                .as_ptr(),
            329 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if component.is_null() || ((*component).componentName).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: params component or component->componentName is null.\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioSapmNewComponent\0"))
                .as_ptr(),
            333 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    sapmComponent = OsalMemCalloc(::core::mem::size_of::<AudioSapmComponent>() as size_t)
        as *mut AudioSapmComponent;
    if sapmComponent.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: malloc cpt fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioSapmNewComponent\0"))
                .as_ptr(),
            339 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if memcpy_s(
        sapmComponent as *mut core::ffi::c_void,
        ::core::mem::size_of::<AudioSapmComponent>() as size_t,
        component as *const core::ffi::c_void,
        ::core::mem::size_of::<AudioSapmComponent>() as size_t,
    ) != EOK
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: memcpy cpt fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioSapmNewComponent\0"))
                .as_ptr(),
            344 as core::ffi::c_int,
        );
        OsalMemFree(sapmComponent as *mut core::ffi::c_void);
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*sapmComponent).componentName = OsalMemCalloc(
        (strlen((*component).componentName) as size_t).wrapping_add(1 as size_t),
    ) as *mut core::ffi::c_char;
    if ((*sapmComponent).componentName).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: malloc cpt->componentName fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioSapmNewComponent\0"))
                .as_ptr(),
            351 as core::ffi::c_int,
        );
        OsalMemFree(sapmComponent as *mut core::ffi::c_void);
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if memcpy_s(
        (*sapmComponent).componentName as *mut core::ffi::c_void,
        (strlen((*component).componentName) as size_t).wrapping_add(1 as size_t),
        (*component).componentName as *const core::ffi::c_void,
        (strlen((*component).componentName) as size_t).wrapping_add(1 as size_t),
    ) != EOK
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: memcpy cpt->componentName fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioSapmNewComponent\0"))
                .as_ptr(),
            357 as core::ffi::c_int,
        );
        OsalMemFree((*sapmComponent).componentName as *mut core::ffi::c_void);
        OsalMemFree(sapmComponent as *mut core::ffi::c_void);
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*sapmComponent).codec = (*(*audioCard).rtd).codec as *mut CodecDevice;
    (*sapmComponent).kcontrolsNum = (*component).kcontrolsNum;
    (*sapmComponent).active = 0 as uint8_t;
    AudioSampCheckPowerCallback(sapmComponent);
    AudioSampPowerClockCallback(sapmComponent);
    DListHeadInit(&mut (*sapmComponent).sources);
    DListHeadInit(&mut (*sapmComponent).sinks);
    DListHeadInit(&mut (*sapmComponent).list);
    DListHeadInit(&mut (*sapmComponent).dirty);
    DListInsertHead(&mut (*sapmComponent).list, &mut (*audioCard).components);
    (*sapmComponent).connected = CONNECT_CODEC_PIN as uint8_t;
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSapmNewComponents(
    mut audioCard: *mut AudioCard,
    mut component: *const AudioSapmComponent,
    mut cptMaxNum: int32_t,
) -> int32_t {
    let mut i: int32_t = 0;
    let mut ret: int32_t = 0;
    if audioCard.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params check error: audioCard is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioSapmNewComponents\0"))
                .as_ptr(),
            387 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if component.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params check error: component is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioSapmNewComponents\0"))
                .as_ptr(),
            391 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    i = 0 as core::ffi::c_int as int32_t;
    while i < cptMaxNum {
        ret = AudioSapmNewComponent(audioCard, component);
        if ret != HDF_SUCCESS as core::ffi::c_int {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: AudioSapmNewComponent fail!\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 23],
                    [core::ffi::c_char; 23],
                >(*b"AudioSapmNewComponents\0"))
                    .as_ptr(),
                398 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
        component = component.offset(1);
        i += 1;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn MuxSetPathStatus(
    mut sapmComponent: *const AudioSapmComponent,
    mut path: *mut AudioSapmpath,
    mut enumKtl: *const AudioEnumKcontrol,
    mut i: int32_t,
) {
    let mut ret: int32_t = 0;
    let mut val: uint32_t = 0 as uint32_t;
    let mut curValue: int32_t = 0;
    let mut shift: uint32_t = 0;
    if sapmComponent.is_null() || ((*sapmComponent).codec).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input MuxSet params check error\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"MuxSetPathStatus\0"))
                .as_ptr(),
            416 as core::ffi::c_int,
        );
        return;
    }
    if path.is_null() || ((*path).name).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params check error: path is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"MuxSetPathStatus\0"))
                .as_ptr(),
            420 as core::ffi::c_int,
        );
        return;
    }
    if enumKtl.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params check error: enumKtl is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"MuxSetPathStatus\0"))
                .as_ptr(),
            424 as core::ffi::c_int,
        );
        return;
    }
    shift = (*enumKtl).shiftLeft as uint32_t;
    ret = AudioCodecReadReg((*sapmComponent).codec, (*enumKtl).reg, &mut val);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: codec read reg fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"MuxSetPathStatus\0"))
                .as_ptr(),
            431 as core::ffi::c_int,
        );
        return;
    }
    curValue = (val >> shift & (*enumKtl).mask) as int32_t;
    (*path).connect = UNCONNECT_SINK_AND_SOURCE as uint8_t;
    if !((*enumKtl).texts).is_null() {
        i = 0 as core::ffi::c_int as int32_t;
        while (i as core::ffi::c_uint) < (*enumKtl).max {
            if (*((*enumKtl).texts).offset(i as isize)).is_null() {
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    LOG_DOMAIN as core::ffi::c_uint,
                    b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s][line:%d]: enumKtl->texts[%d] is NULL\0" as *const u8
                        as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 17],
                        [core::ffi::c_char; 17],
                    >(*b"MuxSetPathStatus\0"))
                        .as_ptr(),
                    441 as core::ffi::c_int,
                    i,
                );
            } else if strcmp((*path).name, *((*enumKtl).texts).offset(i as isize))
                == 0 as core::ffi::c_int && curValue == i
            {
                (*path).connect = CONNECT_SINK_AND_SOURCE as uint8_t;
            }
            i += 1;
        }
    } else if curValue != 0 {
        (*path).connect = CONNECT_SINK_AND_SOURCE as uint8_t;
    }
}
unsafe extern "C" fn MuxValueSetPathStatus(
    mut sapmComponent: *const AudioSapmComponent,
    mut path: *mut AudioSapmpath,
    mut enumKtl: *const AudioEnumKcontrol,
    mut i: int32_t,
) {
    let mut ret: int32_t = 0;
    let mut val: uint32_t = 0 as uint32_t;
    let mut item: uint32_t = 0;
    let mut shift: uint32_t = 0;
    if sapmComponent.is_null() || ((*sapmComponent).codec).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input muxValueSet params check error\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"MuxValueSetPathStatus\0"))
                .as_ptr(),
            466 as core::ffi::c_int,
        );
        return;
    }
    if path.is_null() || ((*path).name).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input MuxValueSet params check error: path is NULL.\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"MuxValueSetPathStatus\0"))
                .as_ptr(),
            470 as core::ffi::c_int,
        );
        return;
    }
    if enumKtl.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input MuxValueSet params check error: enumKtl is NULL.\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"MuxValueSetPathStatus\0"))
                .as_ptr(),
            474 as core::ffi::c_int,
        );
        return;
    }
    shift = (*enumKtl).shiftLeft as uint32_t;
    ret = AudioCodecReadReg((*sapmComponent).codec, (*enumKtl).reg, &mut val);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: muxValueSet read reg fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"MuxValueSetPathStatus\0"))
                .as_ptr(),
            480 as core::ffi::c_int,
        );
        return;
    }
    val = val >> shift & (*enumKtl).mask;
    (*path).connect = UNCONNECT_SINK_AND_SOURCE as uint8_t;
    if !((*enumKtl).values).is_null() && !((*enumKtl).texts).is_null() {
        item = 0 as uint32_t;
        while item < (*enumKtl).max {
            if val == *((*enumKtl).values).offset(item as isize) {
                break;
            }
            item = item.wrapping_add(1);
        }
        i = 0 as core::ffi::c_int as int32_t;
        while (i as core::ffi::c_uint) < (*enumKtl).max {
            if !(*((*enumKtl).texts).offset(i as isize)).is_null() {
                if strcmp((*path).name, *((*enumKtl).texts).offset(i as isize))
                    == 0 as core::ffi::c_int && item == i as core::ffi::c_uint
                {
                    (*path).connect = CONNECT_SINK_AND_SOURCE as uint8_t;
                }
            }
            i += 1;
        }
    } else if val != 0 {
        (*path).connect = CONNECT_SINK_AND_SOURCE as uint8_t;
    }
}
unsafe extern "C" fn MixerSetPathStatus(
    mut sapmComponent: *const AudioSapmComponent,
    mut path: *mut AudioSapmpath,
    mut mixerCtrl: *const AudioMixerControl,
) {
    let mut ret: int32_t = 0;
    let mut reg: uint32_t = 0;
    let mut mask: uint32_t = 0;
    let mut shift: uint32_t = 0;
    let mut invert: uint32_t = 0;
    let mut curValue: uint32_t = 0 as uint32_t;
    if sapmComponent.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params check error: sapmComponent is NULL.\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"MixerSetPathStatus\0"))
                .as_ptr(),
            522 as core::ffi::c_int,
        );
        return;
    }
    if path.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params check error: path is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"MixerSetPathStatus\0"))
                .as_ptr(),
            526 as core::ffi::c_int,
        );
        return;
    }
    if mixerCtrl.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params check error: mixerCtrl is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"MixerSetPathStatus\0"))
                .as_ptr(),
            530 as core::ffi::c_int,
        );
        return;
    }
    reg = (*mixerCtrl).reg;
    shift = (*mixerCtrl).shift;
    mask = (*mixerCtrl).mask;
    invert = (*mixerCtrl).invert;
    if !((*sapmComponent).codec).is_null() {
        ret = AudioCodecReadReg((*sapmComponent).codec, reg, &mut curValue);
        if ret != HDF_SUCCESS as core::ffi::c_int {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: read reg fail!\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 19],
                    [core::ffi::c_char; 19],
                >(*b"MixerSetPathStatus\0"))
                    .as_ptr(),
                542 as core::ffi::c_int,
            );
            return;
        }
    } else {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: codec is null!\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"MixerSetPathStatus\0"))
                .as_ptr(),
            546 as core::ffi::c_int,
        );
        return;
    }
    curValue = curValue >> shift & mask;
    if invert != 0 && curValue == 0 || invert == 0 && curValue != 0 {
        (*path).connect = CONNECT_SINK_AND_SOURCE as uint8_t;
    } else {
        (*path).connect = UNCONNECT_SINK_AND_SOURCE as uint8_t;
    };
}
unsafe extern "C" fn AudioSapmSetPathStatus(
    mut sapmComponent: *const AudioSapmComponent,
    mut path: *mut AudioSapmpath,
    mut i: int32_t,
) -> int32_t {
    if sapmComponent.is_null() || path.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params check error\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioSapmSetPathStatus\0"))
                .as_ptr(),
            564 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    match (*sapmComponent).sapmType as core::ffi::c_uint {
        6 | 17 | 7 => {
            MixerSetPathStatus(
                sapmComponent,
                path,
                (*((*sapmComponent).kcontrolNews).offset(i as isize)).privateValue
                    as uintptr_t as *mut AudioMixerControl,
            );
        }
        2 => {
            MuxSetPathStatus(
                sapmComponent,
                path,
                (*((*sapmComponent).kcontrolNews).offset(i as isize)).privateValue
                    as uintptr_t as *mut AudioEnumKcontrol,
                i,
            );
        }
        5 => {
            MuxValueSetPathStatus(
                sapmComponent,
                path,
                (*((*sapmComponent).kcontrolNews).offset(i as isize)).privateValue
                    as uintptr_t as *mut AudioEnumKcontrol,
                i,
            );
        }
        _ => {
            (*path).connect = CONNECT_SINK_AND_SOURCE as uint8_t;
        }
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioSapmConnectMux(
    mut audioCard: *mut AudioCard,
    mut source: *mut AudioSapmComponent,
    mut sink: *mut AudioSapmComponent,
    mut path: *mut AudioSapmpath,
    mut controlName: *const core::ffi::c_char,
) -> int32_t {
    let mut i: int32_t = 0;
    let mut enumKtl: *mut AudioEnumKcontrol = 0 as *mut AudioEnumKcontrol;
    if audioCard.is_null() || source.is_null() || sink.is_null() || path.is_null()
        || controlName.is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params check error\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioSapmConnectMux\0"))
                .as_ptr(),
            598 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if ((*sink).kcontrolNews).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params sink kcontrolNews is null.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioSapmConnectMux\0"))
                .as_ptr(),
            603 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    enumKtl = &mut (*((*sink).kcontrolNews).offset(0 as core::ffi::c_int as isize))
        .privateValue as *mut core::ffi::c_ulong as *mut AudioEnumKcontrol;
    if enumKtl.is_null() || ((*enumKtl).texts).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: kcontrolNews privateValue is null.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioSapmConnectMux\0"))
                .as_ptr(),
            608 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    i = 0 as core::ffi::c_int as int32_t;
    while (i as core::ffi::c_uint) < (*enumKtl).max {
        if strcmp(controlName, *((*enumKtl).texts).offset(i as isize))
            == 0 as core::ffi::c_int
        {
            DListInsertHead(&mut (*path).list, &mut (*audioCard).paths);
            DListInsertHead(&mut (*path).listSink, &mut (*sink).sources);
            DListInsertHead(&mut (*path).listSource, &mut (*source).sinks);
            (*path).name = *((*enumKtl).texts).offset(i as isize)
                as *mut core::ffi::c_char;
            AudioSapmSetPathStatus(sink, path, i);
            return HDF_SUCCESS as core::ffi::c_int as int32_t;
        }
        i += 1;
    }
    return HDF_FAILURE as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioSapmConnectMixer(
    mut audioCard: *mut AudioCard,
    mut source: *mut AudioSapmComponent,
    mut sink: *mut AudioSapmComponent,
    mut path: *mut AudioSapmpath,
    mut controlName: *const core::ffi::c_char,
) -> int32_t {
    let mut i: int32_t = 0;
    if audioCard.is_null() || source.is_null() || sink.is_null() || path.is_null()
        || controlName.is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params check error\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioSapmConnectMixer\0"))
                .as_ptr(),
            634 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    i = 0 as core::ffi::c_int as int32_t;
    while i < (*sink).kcontrolsNum {
        if !((*((*sink).kcontrolNews).offset(i as isize)).name).is_null() {
            if strcmp(controlName, (*((*sink).kcontrolNews).offset(i as isize)).name)
                == 0 as core::ffi::c_int
            {
                (*path).name = OsalMemCalloc(
                    (strlen((*((*sink).kcontrolNews).offset(i as isize)).name) as size_t)
                        .wrapping_add(1 as size_t),
                ) as *mut core::ffi::c_char;
                if ((*path).name).is_null() {
                    HiLogPrint(
                        LOG_CORE,
                        LOG_ERROR,
                        LOG_DOMAIN as core::ffi::c_uint,
                        b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
                        b"[%s][line:%d]: malloc path->name fail!\0" as *const u8
                            as *const core::ffi::c_char,
                        (::core::mem::transmute::<
                            [u8; 22],
                            [core::ffi::c_char; 22],
                        >(*b"AudioSapmConnectMixer\0"))
                            .as_ptr(),
                        646 as core::ffi::c_int,
                    );
                    return HDF_FAILURE as core::ffi::c_int as int32_t;
                }
                if memcpy_s(
                    (*path).name as *mut core::ffi::c_void,
                    (strlen((*((*sink).kcontrolNews).offset(i as isize)).name) as size_t)
                        .wrapping_add(1 as size_t),
                    (*((*sink).kcontrolNews).offset(i as isize)).name
                        as *const core::ffi::c_void,
                    (strlen((*((*sink).kcontrolNews).offset(i as isize)).name) as size_t)
                        .wrapping_add(1 as size_t),
                ) != EOK
                {
                    OsalMemFree((*path).name as *mut core::ffi::c_void);
                    HiLogPrint(
                        LOG_CORE,
                        LOG_ERROR,
                        LOG_DOMAIN as core::ffi::c_uint,
                        b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
                        b"[%s][line:%d]: memcpy cpt->componentName fail!\0" as *const u8
                            as *const core::ffi::c_char,
                        (::core::mem::transmute::<
                            [u8; 22],
                            [core::ffi::c_char; 22],
                        >(*b"AudioSapmConnectMixer\0"))
                            .as_ptr(),
                        652 as core::ffi::c_int,
                    );
                    return HDF_FAILURE as core::ffi::c_int as int32_t;
                }
                DListInsertHead(&mut (*path).list, &mut (*audioCard).paths);
                DListInsertHead(&mut (*path).listSink, &mut (*sink).sources);
                DListInsertHead(&mut (*path).listSource, &mut (*source).sinks);
                AudioSapmSetPathStatus(sink, path, i);
                return HDF_SUCCESS as core::ffi::c_int as int32_t;
            }
        }
        i += 1;
    }
    return HDF_FAILURE as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioSampStaticOrDynamicPath(
    mut audioCard: *mut AudioCard,
    mut source: *mut AudioSapmComponent,
    mut sink: *mut AudioSapmComponent,
    mut path: *mut AudioSapmpath,
    mut route: *const AudioSapmRoute,
) -> int32_t {
    let mut ret: int32_t = 0;
    if ((*route).control).is_null() {
        DListInsertHead(&mut (*path).list, &mut (*audioCard).paths);
        DListInsertHead(&mut (*path).listSink, &mut (*sink).sources);
        DListInsertHead(&mut (*path).listSource, &mut (*source).sinks);
        (*path).connect = CONNECT_SINK_AND_SOURCE as uint8_t;
        return HDF_SUCCESS as core::ffi::c_int as int32_t;
    }
    match (*sink).sapmType as core::ffi::c_uint {
        2 | 4 | 5 => {
            ret = AudioSapmConnectMux(audioCard, source, sink, path, (*route).control);
            if ret != HDF_SUCCESS as core::ffi::c_int {
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    LOG_DOMAIN as core::ffi::c_uint,
                    b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s][line:%d]: connect mux fail!\0" as *const u8
                        as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 29],
                        [core::ffi::c_char; 29],
                    >(*b"AudioSampStaticOrDynamicPath\0"))
                        .as_ptr(),
                    688 as core::ffi::c_int,
                );
                return HDF_FAILURE as core::ffi::c_int as int32_t;
            }
        }
        17 | 6 | 7 | 8 | 15 => {
            ret = AudioSapmConnectMixer(audioCard, source, sink, path, (*route).control);
            if ret != HDF_SUCCESS as core::ffi::c_int {
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    LOG_DOMAIN as core::ffi::c_uint,
                    b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s][line:%d]: connect mixer fail!\0" as *const u8
                        as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 29],
                        [core::ffi::c_char; 29],
                    >(*b"AudioSampStaticOrDynamicPath\0"))
                        .as_ptr(),
                    699 as core::ffi::c_int,
                );
                return HDF_FAILURE as core::ffi::c_int as int32_t;
            }
        }
        14 | 13 | 16 => {
            DListInsertHead(&mut (*path).list, &mut (*audioCard).paths);
            DListInsertHead(&mut (*path).listSink, &mut (*sink).sources);
            DListInsertHead(&mut (*path).listSource, &mut (*source).sinks);
            (*path).connect = CONNECT_SINK_AND_SOURCE as uint8_t;
        }
        _ => {
            DListInsertHead(&mut (*path).list, &mut (*audioCard).paths);
            DListInsertHead(&mut (*path).listSink, &mut (*sink).sources);
            DListInsertHead(&mut (*path).listSource, &mut (*source).sinks);
            (*path).connect = CONNECT_SINK_AND_SOURCE as uint8_t;
        }
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioSampExtComponentsCheck(
    mut cptSource: *mut AudioSapmComponent,
    mut cptSink: *mut AudioSapmComponent,
) {
    if cptSource.is_null() || cptSink.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params check error\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"AudioSampExtComponentsCheck\0"))
                .as_ptr(),
            725 as core::ffi::c_int,
        );
        return;
    }
    if (*cptSink).sapmType as core::ffi::c_uint
        == AUDIO_SAPM_INPUT as core::ffi::c_int as core::ffi::c_uint
    {
        if (*cptSource).sapmType as core::ffi::c_uint
            == AUDIO_SAPM_MICBIAS as core::ffi::c_int as core::ffi::c_uint
            || (*cptSource).sapmType as core::ffi::c_uint
                == AUDIO_SAPM_MIC as core::ffi::c_int as core::ffi::c_uint
            || (*cptSource).sapmType as core::ffi::c_uint
                == AUDIO_SAPM_LINE as core::ffi::c_int as core::ffi::c_uint
            || (*cptSource).sapmType as core::ffi::c_uint
                == AUDIO_SAPM_OUTPUT as core::ffi::c_int as core::ffi::c_uint
        {
            (*cptSink).external = EXIST_EXTERNAL_WIDGET as uint8_t;
        }
    }
    if (*cptSource).sapmType as core::ffi::c_uint
        == AUDIO_SAPM_OUTPUT as core::ffi::c_int as core::ffi::c_uint
    {
        if (*cptSink).sapmType as core::ffi::c_uint
            == AUDIO_SAPM_SPK as core::ffi::c_int as core::ffi::c_uint
            || (*cptSink).sapmType as core::ffi::c_uint
                == AUDIO_SAPM_HP as core::ffi::c_int as core::ffi::c_uint
            || (*cptSink).sapmType as core::ffi::c_uint
                == AUDIO_SAPM_LINE as core::ffi::c_int as core::ffi::c_uint
            || (*cptSink).sapmType as core::ffi::c_uint
                == AUDIO_SAPM_INPUT as core::ffi::c_int as core::ffi::c_uint
        {
            (*cptSource).external = EXIST_EXTERNAL_WIDGET as uint8_t;
        }
    }
}
unsafe extern "C" fn AudioSapmAddRoute(
    mut audioCard: *mut AudioCard,
    mut route: *const AudioSapmRoute,
) -> int32_t {
    let mut path: *mut AudioSapmpath = 0 as *mut AudioSapmpath;
    let mut cptSource: *mut AudioSapmComponent = 0 as *mut AudioSapmComponent;
    let mut cptSink: *mut AudioSapmComponent = 0 as *mut AudioSapmComponent;
    let mut sapmComponent: *mut AudioSapmComponent = 0 as *mut AudioSapmComponent;
    let mut ret: int32_t = 0;
    if route.is_null() || ((*route).source).is_null() || ((*route).sink).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params check error: route is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"AudioSapmAddRoute\0"))
                .as_ptr(),
            755 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    sapmComponent = ((*audioCard).components.next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut AudioSapmComponent)).list as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_int as *mut AudioSapmComponent;
    while &mut (*sapmComponent).list as *mut DListHead
        != &mut (*audioCard).components as *mut DListHead
    {
        if !((*sapmComponent).componentName).is_null() {
            if cptSource.is_null()
                && strcmp((*sapmComponent).componentName, (*route).source)
                    == 0 as core::ffi::c_int
            {
                cptSource = sapmComponent;
            } else {
                if cptSink.is_null()
                    && strcmp((*sapmComponent).componentName, (*route).sink)
                        == 0 as core::ffi::c_int
                {
                    cptSink = sapmComponent;
                }
                if !cptSource.is_null() && !cptSink.is_null() {
                    break;
                }
            }
        }
        sapmComponent = ((*sapmComponent).list.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut AudioSapmComponent)).list as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_int as *mut AudioSapmComponent;
    }
    if cptSource.is_null() || cptSink.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: find component fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"AudioSapmAddRoute\0"))
                .as_ptr(),
            775 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    path = OsalMemCalloc(::core::mem::size_of::<AudioSapmpath>() as size_t)
        as *mut AudioSapmpath;
    if path.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: malloc path fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"AudioSapmAddRoute\0"))
                .as_ptr(),
            781 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*path).source = cptSource;
    (*path).sink = cptSink;
    DListHeadInit(&mut (*path).list);
    DListHeadInit(&mut (*path).listSink);
    DListHeadInit(&mut (*path).listSource);
    AudioSampExtComponentsCheck(cptSource, cptSink);
    ret = AudioSampStaticOrDynamicPath(audioCard, cptSource, cptSink, path, route);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        OsalMemFree(path as *mut core::ffi::c_void);
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: static or dynamic path fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"AudioSapmAddRoute\0"))
                .as_ptr(),
            796 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSapmAddRoutes(
    mut audioCard: *mut AudioCard,
    mut route: *const AudioSapmRoute,
    mut routeMaxNum: int32_t,
) -> int32_t {
    let mut i: int32_t = 0;
    let mut ret: int32_t = 0;
    if audioCard.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params check error: audioCard is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioSapmAddRoutes\0"))
                .as_ptr(),
            808 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if route.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params check error: route is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioSapmAddRoutes\0"))
                .as_ptr(),
            812 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    i = 0 as core::ffi::c_int as int32_t;
    while i < routeMaxNum {
        ret = AudioSapmAddRoute(audioCard, route);
        if ret != HDF_SUCCESS as core::ffi::c_int {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: AudioSapmAddRoute failed!\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 19],
                    [core::ffi::c_char; 19],
                >(*b"AudioSapmAddRoutes\0"))
                    .as_ptr(),
                819 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
        route = route.offset(1);
        i += 1;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioSapmNewMixerControls(
    mut sapmComponent: *const AudioSapmComponent,
    mut audioCard: *mut AudioCard,
) -> int32_t {
    let mut path: *mut AudioSapmpath = 0 as *mut AudioSapmpath;
    let mut i: int32_t = 0;
    if sapmComponent.is_null() || ((*sapmComponent).kcontrols).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params check error: sapmComponent is NULL.\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"AudioSapmNewMixerControls\0"))
                .as_ptr(),
            834 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if audioCard.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params check error: audioCard is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"AudioSapmNewMixerControls\0"))
                .as_ptr(),
            838 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    i = 0 as core::ffi::c_int as int32_t;
    while i < (*sapmComponent).kcontrolsNum {
        path = ((*sapmComponent).sources.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut AudioSapmpath)).listSink as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_int as *mut AudioSapmpath;
        while &mut (*path).listSink as *mut DListHead
            != &(*sapmComponent).sources as *const DListHead as *mut DListHead
        {
            if !(((*path).name).is_null() || ((*sapmComponent).kcontrolNews).is_null()
                || ((*((*sapmComponent).kcontrolNews).offset(i as isize)).name)
                    .is_null())
            {
                if !(strcmp(
                    (*path).name,
                    (*((*sapmComponent).kcontrolNews).offset(i as isize)).name,
                ) != 0 as core::ffi::c_int)
                {
                    (*path).kcontrol = AudioAddControl(
                        audioCard,
                        &mut *((*sapmComponent).kcontrolNews).offset(i as isize),
                    );
                    if ((*path).kcontrol).is_null() {
                        HiLogPrint(
                            LOG_CORE,
                            LOG_ERROR,
                            LOG_DOMAIN as core::ffi::c_uint,
                            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
                            b"[%s][line:%d]: add control fail!\0" as *const u8
                                as *const core::ffi::c_char,
                            (::core::mem::transmute::<
                                [u8; 26],
                                [core::ffi::c_char; 26],
                            >(*b"AudioSapmNewMixerControls\0"))
                                .as_ptr(),
                            855 as core::ffi::c_int,
                        );
                        return HDF_FAILURE as core::ffi::c_int as int32_t;
                    }
                    let ref mut fresh1 = *((*sapmComponent).kcontrols)
                        .offset(i as isize);
                    *fresh1 = (*path).kcontrol;
                    DListInsertHead(
                        &mut (**((*sapmComponent).kcontrols).offset(i as isize)).list,
                        &mut (*audioCard).controls,
                    );
                }
            }
            path = ((*path).listSink.next as *mut core::ffi::c_char)
                .offset_from(
                    &mut (*(0 as *mut AudioSapmpath)).listSink as *mut DListHead
                        as *mut core::ffi::c_char,
                ) as core::ffi::c_int as *mut AudioSapmpath;
        }
        i += 1;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioSapmNewMuxControls(
    mut sapmComponent: *mut AudioSapmComponent,
    mut audioCard: *mut AudioCard,
) -> int32_t {
    let mut kctrl: *mut AudioKcontrol = 0 as *mut AudioKcontrol;
    if sapmComponent.is_null() || ((*sapmComponent).kcontrolNews).is_null()
        || audioCard.is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input param is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"AudioSapmNewMuxControls\0"))
                .as_ptr(),
            871 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if (*sapmComponent).kcontrolsNum != 1 as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: incorrect number of controls.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"AudioSapmNewMuxControls\0"))
                .as_ptr(),
            876 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    kctrl = AudioAddControl(
        audioCard,
        &mut *((*sapmComponent).kcontrolNews).offset(0 as core::ffi::c_int as isize),
    );
    if kctrl.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: add control fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"AudioSapmNewMuxControls\0"))
                .as_ptr(),
            882 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if ((*sapmComponent).kcontrols).is_null() {
        OsalMemFree(kctrl as *mut core::ffi::c_void);
        kctrl = 0 as *mut AudioKcontrol;
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: sapmComponent->kcontrols is NULL!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"AudioSapmNewMuxControls\0"))
                .as_ptr(),
            889 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    let ref mut fresh0 = *((*sapmComponent).kcontrols)
        .offset(0 as core::ffi::c_int as isize);
    *fresh0 = kctrl;
    DListInsertHead(
        &mut (**((*sapmComponent).kcontrols).offset(0 as core::ffi::c_int as isize))
            .list,
        &mut (*audioCard).controls,
    );
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioSapmPowerSeqInsert(
    mut newSapmComponent: *mut AudioSapmComponent,
    mut list: *mut DListHead,
    mut isPowerUp: int8_t,
) {
    let mut sapmComponent: *mut AudioSapmComponent = 0 as *mut AudioSapmComponent;
    let mut seq: *mut int32_t = 0 as *mut int32_t;
    if newSapmComponent.is_null() || list.is_null()
        || ((*newSapmComponent).componentName).is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input param newCpt is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"AudioSapmPowerSeqInsert\0"))
                .as_ptr(),
            905 as core::ffi::c_int,
        );
        return;
    }
    if isPowerUp != 0 {
        seq = g_audioSapmPowerUpSeq.as_mut_ptr();
    } else {
        seq = g_audioSapmPowerDownSeq.as_mut_ptr();
    }
    sapmComponent = ((*list).next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut AudioSapmComponent)).powerList as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_int as *mut AudioSapmComponent;
    while &mut (*sapmComponent).powerList as *mut DListHead != list {
        if *seq.offset((*newSapmComponent).sapmType as isize)
            - *seq.offset((*sapmComponent).sapmType as isize) < 0 as core::ffi::c_int
        {
            DListInsertTail(
                &mut (*newSapmComponent).powerList,
                &mut (*sapmComponent).powerList,
            );
            return;
        }
        sapmComponent = ((*sapmComponent).powerList.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut AudioSapmComponent)).powerList as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_int as *mut AudioSapmComponent;
    }
    DListInsertTail(&mut (*newSapmComponent).powerList, list);
}
unsafe extern "C" fn AudioSapmSetPower(
    mut audioCard: *mut AudioCard,
    mut sapmComponent: *mut AudioSapmComponent,
    mut power: uint8_t,
    mut upList: *mut DListHead,
    mut downList: *mut DListHead,
) {
    let mut path: *mut AudioSapmpath = 0 as *mut AudioSapmpath;
    if sapmComponent.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input param sapmComponent is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"AudioSapmSetPower\0"))
                .as_ptr(),
            933 as core::ffi::c_int,
        );
        return;
    }
    path = ((*sapmComponent).sources.next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut AudioSapmpath)).listSink as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_int as *mut AudioSapmpath;
    while &mut (*path).listSink as *mut DListHead
        != &mut (*sapmComponent).sources as *mut DListHead
    {
        if !((*path).source).is_null() {
            if (*(*path).source).power as core::ffi::c_int != power as core::ffi::c_int
                && (*path).connect as core::ffi::c_int != 0
            {
                if DListIsEmpty(&mut (*(*path).source).dirty) {
                    DListInsertTail(
                        &mut (*(*path).source).dirty,
                        &mut (*audioCard).sapmDirty,
                    );
                }
            }
        }
        path = ((*path).listSink.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut AudioSapmpath)).listSink as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_int as *mut AudioSapmpath;
    }
    path = ((*sapmComponent).sinks.next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut AudioSapmpath)).listSource as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_int as *mut AudioSapmpath;
    while &mut (*path).listSource as *mut DListHead
        != &mut (*sapmComponent).sinks as *mut DListHead
    {
        if !((*path).sink).is_null() {
            if (*(*path).sink).power as core::ffi::c_int != power as core::ffi::c_int
                && (*path).connect as core::ffi::c_int != 0
            {
                if DListIsEmpty(&mut (*(*path).sink).dirty) {
                    DListInsertTail(
                        &mut (*(*path).sink).dirty,
                        &mut (*audioCard).sapmDirty,
                    );
                }
            }
        }
        path = ((*path).listSource.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut AudioSapmpath)).listSource as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_int as *mut AudioSapmpath;
    }
    if power != 0 {
        AudioSapmPowerSeqInsert(sapmComponent, upList, power as int8_t);
    } else {
        AudioSapmPowerSeqInsert(sapmComponent, downList, power as int8_t);
    };
}
unsafe extern "C" fn AudioSapmPowerUpSeqRun(mut list: *const DListHead) {
    let mut val: uint32_t = 0;
    let mut mixerControl: AudioMixerControl = AudioMixerControl {
        min: 0,
        max: 0,
        platformMax: 0,
        mask: 0,
        reg: 0,
        rreg: 0,
        shift: 0,
        rshift: 0,
        invert: 0,
        value: 0,
    };
    let mut sapmComponent: *mut AudioSapmComponent = 0 as *mut AudioSapmComponent;
    if list.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input param list is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioSapmPowerUpSeqRun\0"))
                .as_ptr(),
            972 as core::ffi::c_int,
        );
        return;
    }
    sapmComponent = ((*list).next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut AudioSapmComponent)).powerList as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_int as *mut AudioSapmComponent;
    while &mut (*sapmComponent).powerList as *mut DListHead != list as *mut DListHead {
        if (*sapmComponent).power as core::ffi::c_int == SAPM_POWER_DOWN {
            val = SAPM_POWER_UP as uint32_t;
            if (*sapmComponent).invert != 0 {
                val = (val == 0) as core::ffi::c_int as uint32_t;
            }
            (*sapmComponent).power = SAPM_POWER_UP as uint8_t;
            if (*sapmComponent).reg != AUDIO_NO_SAPM_REG as core::ffi::c_uint {
                mixerControl.reg = (*sapmComponent).reg;
                mixerControl.mask = (*sapmComponent).mask;
                mixerControl.shift = (*sapmComponent).shift as uint32_t;
                AudioUpdateCodecRegBits(
                    (*sapmComponent).codec,
                    mixerControl.reg,
                    mixerControl.mask,
                    mixerControl.shift,
                    val,
                );
                HiLogPrint(
                    LOG_CORE,
                    LOG_INFO,
                    LOG_DOMAIN as core::ffi::c_uint,
                    b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s][line:%d]: Sapm Codec %s Power Up.\0" as *const u8
                        as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 23],
                        [core::ffi::c_char; 23],
                    >(*b"AudioSapmPowerUpSeqRun\0"))
                        .as_ptr(),
                    989 as core::ffi::c_int,
                    (*sapmComponent).componentName,
                );
            }
        }
        sapmComponent = ((*sapmComponent).powerList.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut AudioSapmComponent)).powerList as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_int as *mut AudioSapmComponent;
    }
}
unsafe extern "C" fn AudioSapmPowerDownSeqRun(mut list: *const DListHead) {
    let mut val: uint32_t = 0;
    let mut mixerControl: AudioMixerControl = AudioMixerControl {
        min: 0,
        max: 0,
        platformMax: 0,
        mask: 0,
        reg: 0,
        rreg: 0,
        shift: 0,
        rshift: 0,
        invert: 0,
        value: 0,
    };
    let mut sapmComponent: *mut AudioSapmComponent = 0 as *mut AudioSapmComponent;
    if list.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: sapm input param list is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"AudioSapmPowerDownSeqRun\0"))
                .as_ptr(),
            1005 as core::ffi::c_int,
        );
        return;
    }
    sapmComponent = ((*list).next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut AudioSapmComponent)).powerList as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_int as *mut AudioSapmComponent;
    while &mut (*sapmComponent).powerList as *mut DListHead != list as *mut DListHead {
        if (*sapmComponent).power as core::ffi::c_int == SAPM_POWER_UP {
            val = SAPM_POWER_DOWN as uint32_t;
            if (*sapmComponent).invert != 0 {
                val = (val == 0) as core::ffi::c_int as uint32_t;
            }
            (*sapmComponent).power = SAPM_POWER_DOWN as uint8_t;
            if (*sapmComponent).reg != AUDIO_NO_SAPM_REG as core::ffi::c_uint {
                mixerControl.mask = (*sapmComponent).mask;
                mixerControl.reg = (*sapmComponent).reg;
                mixerControl.shift = (*sapmComponent).shift as uint32_t;
                AudioUpdateCodecRegBits(
                    (*sapmComponent).codec,
                    mixerControl.reg,
                    mixerControl.mask,
                    mixerControl.shift,
                    val,
                );
                HiLogPrint(
                    LOG_CORE,
                    LOG_INFO,
                    LOG_DOMAIN as core::ffi::c_uint,
                    b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s][line:%d]: Sapm Codec %s Power Down.\0" as *const u8
                        as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 25],
                        [core::ffi::c_char; 25],
                    >(*b"AudioSapmPowerDownSeqRun\0"))
                        .as_ptr(),
                    1023 as core::ffi::c_int,
                    (*sapmComponent).componentName,
                );
            }
        }
        sapmComponent = ((*sapmComponent).powerList.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut AudioSapmComponent)).powerList as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_int as *mut AudioSapmComponent;
    }
}
unsafe extern "C" fn AudioSapmPowerComponents(mut audioCard: *mut AudioCard) {
    let mut ret: int32_t = 0;
    let mut sapmComponent: *mut AudioSapmComponent = 0 as *mut AudioSapmComponent;
    let mut upList: DListHead = DListHead {
        next: 0 as *mut DListHead,
        prev: 0 as *mut DListHead,
    };
    let mut downList: DListHead = DListHead {
        next: 0 as *mut DListHead,
        prev: 0 as *mut DListHead,
    };
    if audioCard.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input param audioCard is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"AudioSapmPowerComponents\0"))
                .as_ptr(),
            1040 as core::ffi::c_int,
        );
        return;
    }
    DListHeadInit(&mut upList);
    DListHeadInit(&mut downList);
    let mut current_block_13: u64;
    sapmComponent = ((*audioCard).sapmDirty.next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut AudioSapmComponent)).dirty as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_int as *mut AudioSapmComponent;
    while &mut (*sapmComponent).dirty as *mut DListHead
        != &mut (*audioCard).sapmDirty as *mut DListHead
    {
        (*sapmComponent).newPower = ((*sapmComponent).PowerCheck)
            .expect("non-null function pointer")(sapmComponent) as uint8_t;
        if !((*sapmComponent).newPower as core::ffi::c_int
            == (*sapmComponent).power as core::ffi::c_int)
        {
            if (*audioCard).sapmStandbyState as core::ffi::c_int != 0
                && ((*sapmComponent).PowerClockOp).is_some()
            {
                ret = ((*sapmComponent).PowerClockOp)
                    .expect("non-null function pointer")(sapmComponent);
                if ret != HDF_SUCCESS as core::ffi::c_int {
                    current_block_13 = 11812396948646013369;
                } else {
                    current_block_13 = 12039483399334584727;
                }
            } else {
                current_block_13 = 12039483399334584727;
            }
            match current_block_13 {
                11812396948646013369 => {}
                _ => {
                    AudioSapmSetPower(
                        audioCard,
                        sapmComponent,
                        (*sapmComponent).newPower,
                        &mut upList,
                        &mut downList,
                    );
                }
            }
        }
        sapmComponent = ((*sapmComponent).dirty.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut AudioSapmComponent)).dirty as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_int as *mut AudioSapmComponent;
    }
    sapmComponent = ((*audioCard).components.next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut AudioSapmComponent)).list as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_int as *mut AudioSapmComponent;
    while &mut (*sapmComponent).list as *mut DListHead
        != &mut (*audioCard).components as *mut DListHead
    {
        DListRemove(&mut (*sapmComponent).dirty);
        DListHeadInit(&mut (*sapmComponent).dirty);
        sapmComponent = ((*sapmComponent).list.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut AudioSapmComponent)).list as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_int as *mut AudioSapmComponent;
    }
    AudioSapmPowerDownSeqRun(&mut downList);
    AudioSapmPowerUpSeqRun(&mut upList);
}
unsafe extern "C" fn ReadInitComponentPowerStatus(
    mut sapmComponent: *mut AudioSapmComponent,
) {
    let mut ret: int32_t = 0;
    let mut regVal: uint32_t = 0 as uint32_t;
    if sapmComponent.is_null() || ((*sapmComponent).codec).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input param sapmComponent is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"ReadInitComponentPowerStatus\0"))
                .as_ptr(),
            1078 as core::ffi::c_int,
        );
        return;
    }
    if (*sapmComponent).reg != AUDIO_NO_SAPM_REG as core::ffi::c_uint {
        ret = AudioCodecReadReg(
            (*sapmComponent).codec,
            (*sapmComponent).reg,
            &mut regVal,
        );
        if ret != HDF_SUCCESS as core::ffi::c_int {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: read reg fail!\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 29],
                    [core::ffi::c_char; 29],
                >(*b"ReadInitComponentPowerStatus\0"))
                    .as_ptr(),
                1085 as core::ffi::c_int,
            );
            return;
        }
        regVal
            &= ((1 as core::ffi::c_int) << (*sapmComponent).shift as core::ffi::c_int)
                as core::ffi::c_uint;
        if (*sapmComponent).invert != 0 {
            regVal = (regVal == 0) as core::ffi::c_int as uint32_t;
        }
        if regVal != 0 {
            (*sapmComponent).power = SAPM_POWER_UP as uint8_t;
        } else {
            (*sapmComponent).power = SAPM_POWER_DOWN as uint8_t;
        }
    }
}
unsafe extern "C" fn AudioSapmThread(
    mut data: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let mut audioCard: *mut AudioCard = data as *mut AudioCard;
    (*audioCard).time = 0 as uint64_t;
    loop {
        OsalSleep(SAPM_POLL_TIME as uint32_t);
        AudioSapmTimerCallback(audioCard);
        (*audioCard).time = ((*audioCard).time).wrapping_add(1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioSapmSleep(mut audioCard: *mut AudioCard) -> int32_t {
    let mut ret: int32_t = 0;
    let mut sapmThreadName: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut threadCfg: OsalThreadParam = OsalThreadParam {
        name: 0 as *mut core::ffi::c_char,
        stackSize: 0,
        priority: OSAL_THREAD_PRI_LOW,
        policy: 0,
    };
    let mut audioSapmThread: OsalThread = OsalThread {
        realThread: 0 as *mut core::ffi::c_void,
    };
    if audioCard.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input param audioCard is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"AudioSapmSleep\0"))
                .as_ptr(),
            1126 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    AudioSapmRefreshTime(audioCard, true_0 != 0);
    sapmThreadName = OsalMemCalloc(SAPM_THREAD_NAME as size_t) as *mut core::ffi::c_char;
    if sapmThreadName.is_null() {
        return HDF_ERR_MALLOC_FAIL as core::ffi::c_int as int32_t;
    }
    if snprintf_s(
        sapmThreadName,
        SAPM_THREAD_NAME as size_t,
        (SAPM_THREAD_NAME - 1 as core::ffi::c_int) as size_t,
        b"AudioSapmThread%u\0" as *const u8 as *const core::ffi::c_char,
        g_cardNum,
    ) < 0 as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: snprintf_s failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"AudioSapmSleep\0"))
                .as_ptr(),
            1136 as core::ffi::c_int,
        );
        OsalMemFree(sapmThreadName as *mut core::ffi::c_void);
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    ret = OsalThreadCreate(
        &mut audioSapmThread,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> core::ffi::c_int>,
            OsalThreadEntry,
        >(
            Some(
                AudioSapmThread
                    as unsafe extern "C" fn(*mut core::ffi::c_void) -> core::ffi::c_int,
            ),
        ),
        audioCard as *mut core::ffi::c_void,
    );
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: create sapm thread fail, ret=%d\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"AudioSapmSleep\0"))
                .as_ptr(),
            1143 as core::ffi::c_int,
            ret,
        );
        OsalMemFree(sapmThreadName as *mut core::ffi::c_void);
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    memset_s(
        &mut threadCfg as *mut OsalThreadParam as *mut core::ffi::c_void,
        ::core::mem::size_of::<OsalThreadParam>() as size_t,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<OsalThreadParam>() as size_t,
    );
    threadCfg.name = sapmThreadName;
    threadCfg.priority = OSAL_THREAD_PRI_DEFAULT;
    threadCfg.stackSize = SAPM_STACK_SIZE as size_t;
    ret = OsalThreadStart(&mut audioSapmThread, &mut threadCfg);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: start sapm thread fail, ret=%d\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"AudioSapmSleep\0"))
                .as_ptr(),
            1154 as core::ffi::c_int,
            ret,
        );
        OsalThreadDestroy(&mut audioSapmThread);
        OsalMemFree(sapmThreadName as *mut core::ffi::c_void);
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    g_cardNum = g_cardNum.wrapping_add(1);
    (*audioCard).sapmStandbyState = false_0 != 0;
    (*audioCard).sapmSleepState = false_0 != 0;
    OsalMemFree(sapmThreadName as *mut core::ffi::c_void);
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSapmNewControls(mut audioCard: *mut AudioCard) -> int32_t {
    let mut sapmComponent: *mut AudioSapmComponent = 0 as *mut AudioSapmComponent;
    let mut ret: int32_t = HDF_SUCCESS as core::ffi::c_int as int32_t;
    if audioCard.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input param audioCard is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"AudioSapmNewControls\0"))
                .as_ptr(),
            1174 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    sapmComponent = ((*audioCard).components.next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut AudioSapmComponent)).list as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_int as *mut AudioSapmComponent;
    while &mut (*sapmComponent).list as *mut DListHead
        != &mut (*audioCard).components as *mut DListHead
    {
        if !((*sapmComponent).newCpt != 0) {
            if (*sapmComponent).kcontrolsNum > 0 as core::ffi::c_int {
                (*sapmComponent).kcontrols = OsalMemCalloc(
                    (::core::mem::size_of::<*mut AudioKcontrol>() as size_t)
                        .wrapping_mul((*sapmComponent).kcontrolsNum as size_t),
                ) as *mut *mut AudioKcontrol;
                if ((*sapmComponent).kcontrols).is_null() {
                    HiLogPrint(
                        LOG_CORE,
                        LOG_ERROR,
                        LOG_DOMAIN as core::ffi::c_uint,
                        b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
                        b"[%s][line:%d]: malloc kcontrols fail!\0" as *const u8
                            as *const core::ffi::c_char,
                        (::core::mem::transmute::<
                            [u8; 21],
                            [core::ffi::c_char; 21],
                        >(*b"AudioSapmNewControls\0"))
                            .as_ptr(),
                        1185 as core::ffi::c_int,
                    );
                    return HDF_FAILURE as core::ffi::c_int as int32_t;
                }
            }
            match (*sapmComponent).sapmType as core::ffi::c_uint {
                17 | 6 | 7 => {
                    ret = AudioSapmNewMixerControls(sapmComponent, audioCard);
                }
                2 | 4 | 5 => {
                    ret = AudioSapmNewMuxControls(sapmComponent, audioCard);
                }
                _ => {
                    ret = HDF_SUCCESS as core::ffi::c_int as int32_t;
                }
            }
            if ret != HDF_SUCCESS as core::ffi::c_int {
                OsalMemFree((*sapmComponent).kcontrols as *mut core::ffi::c_void);
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    LOG_DOMAIN as core::ffi::c_uint,
                    b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s][line:%d]: sapm new mixer or mux controls fail!\0" as *const u8
                        as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 21],
                        [core::ffi::c_char; 21],
                    >(*b"AudioSapmNewControls\0"))
                        .as_ptr(),
                    1207 as core::ffi::c_int,
                );
                return HDF_FAILURE as core::ffi::c_int as int32_t;
            }
            ReadInitComponentPowerStatus(sapmComponent);
            (*sapmComponent).newCpt = 1 as uint8_t;
            DListInsertTail(&mut (*sapmComponent).dirty, &mut (*audioCard).sapmDirty);
        }
        sapmComponent = ((*sapmComponent).list.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut AudioSapmComponent)).list as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_int as *mut AudioSapmComponent;
    }
    AudioSapmPowerComponents(audioCard);
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn MixerUpdatePowerStatus(
    mut kcontrol: *const AudioKcontrol,
    mut pathStatus: uint32_t,
) -> int32_t {
    let mut audioCard: *mut AudioCard = 0 as *mut AudioCard;
    let mut path: *mut AudioSapmpath = 0 as *mut AudioSapmpath;
    if kcontrol.is_null() || ((*kcontrol).pri).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Mixer input param kcontrol is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"MixerUpdatePowerStatus\0"))
                .as_ptr(),
            1227 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    audioCard = (*kcontrol).pri as uintptr_t as *mut AudioCard;
    path = ((*audioCard).paths.next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut AudioSapmpath)).list as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_int as *mut AudioSapmpath;
    while &mut (*path).list as *mut DListHead
        != &mut (*audioCard).paths as *mut DListHead
    {
        if (*path).kcontrol != kcontrol as *mut AudioKcontrol {
            path = ((*path).list.next as *mut core::ffi::c_char)
                .offset_from(
                    &mut (*(0 as *mut AudioSapmpath)).list as *mut DListHead
                        as *mut core::ffi::c_char,
                ) as core::ffi::c_int as *mut AudioSapmpath;
        } else {
            if ((*path).sink).is_null() || ((*path).source).is_null() {
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    LOG_DOMAIN as core::ffi::c_uint,
                    b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s][line:%d]: get path sink or source fail!\0" as *const u8
                        as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 23],
                        [core::ffi::c_char; 23],
                    >(*b"MixerUpdatePowerStatus\0"))
                        .as_ptr(),
                    1237 as core::ffi::c_int,
                );
                return HDF_FAILURE as core::ffi::c_int as int32_t;
            }
            if (*(*path).sink).sapmType as core::ffi::c_uint
                != AUDIO_SAPM_MIXER as core::ffi::c_int as core::ffi::c_uint
                && (*(*path).sink).sapmType as core::ffi::c_uint
                    != AUDIO_SAPM_MIXER_NAMED_CTRL as core::ffi::c_int
                        as core::ffi::c_uint
                && (*(*path).sink).sapmType as core::ffi::c_uint
                    != AUDIO_SAPM_PGA as core::ffi::c_int as core::ffi::c_uint
                && (*(*path).sink).sapmType as core::ffi::c_uint
                    != AUDIO_SAPM_SPK as core::ffi::c_int as core::ffi::c_uint
                && (*(*path).sink).sapmType as core::ffi::c_uint
                    != AUDIO_SAPM_ANALOG_SWITCH as core::ffi::c_int as core::ffi::c_uint
            {
                return HDF_DEV_ERR_NO_DEVICE as core::ffi::c_int as int32_t;
            }
            (*path).connect = pathStatus as uint8_t;
            DListInsertTail(&mut (*(*path).source).dirty, &mut (*audioCard).sapmDirty);
            DListInsertTail(&mut (*(*path).sink).dirty, &mut (*audioCard).sapmDirty);
            break;
        }
    }
    AudioSapmPowerComponents(audioCard);
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn MuxUpdatePowerStatus(
    mut kcontrol: *const AudioKcontrol,
    mut i: int32_t,
    mut enumKtl: *mut AudioEnumKcontrol,
) -> int32_t {
    let mut audioCard: *mut AudioCard = 0 as *mut AudioCard;
    let mut path: *mut AudioSapmpath = 0 as *mut AudioSapmpath;
    if kcontrol.is_null() || ((*kcontrol).pri).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Mux input param kcontrol is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"MuxUpdatePowerStatus\0"))
                .as_ptr(),
            1266 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    audioCard = (*kcontrol).pri as uintptr_t as *mut AudioCard;
    path = ((*audioCard).paths.next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut AudioSapmpath)).list as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_int as *mut AudioSapmpath;
    while &mut (*path).list as *mut DListHead
        != &mut (*audioCard).paths as *mut DListHead
    {
        if !((*path).kcontrol != kcontrol as *mut AudioKcontrol) {
            if !(((*path).name).is_null()
                || (*((*enumKtl).texts).offset(i as isize)).is_null())
            {
                if (*(*path).sink).sapmType as core::ffi::c_uint
                    != AUDIO_SAPM_MUX as core::ffi::c_int as core::ffi::c_uint
                    && (*(*path).sink).sapmType as core::ffi::c_uint
                        != AUDIO_SAPM_VIRT_MUX as core::ffi::c_int as core::ffi::c_uint
                    && (*(*path).sink).sapmType as core::ffi::c_uint
                        != AUDIO_SAPM_VALUE_MUX as core::ffi::c_int as core::ffi::c_uint
                {
                    HiLogPrint(
                        LOG_CORE,
                        LOG_ERROR,
                        LOG_DOMAIN as core::ffi::c_uint,
                        b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
                        b"[%s][line:%d]: no mux device.\0" as *const u8
                            as *const core::ffi::c_char,
                        (::core::mem::transmute::<
                            [u8; 21],
                            [core::ffi::c_char; 21],
                        >(*b"MuxUpdatePowerStatus\0"))
                            .as_ptr(),
                        1282 as core::ffi::c_int,
                    );
                    return HDF_DEV_ERR_NO_DEVICE as core::ffi::c_int as int32_t;
                }
                if strcmp((*path).name, *((*enumKtl).texts).offset(i as isize))
                    == 0 as core::ffi::c_int
                {
                    (*path).connect = 1 as uint8_t;
                } else if (*path).connect as core::ffi::c_int == 1 as core::ffi::c_int {
                    (*path).connect = 0 as uint8_t;
                }
                DListInsertTail(
                    &mut (*(*path).source).dirty,
                    &mut (*audioCard).sapmDirty,
                );
                DListInsertTail(&mut (*(*path).sink).dirty, &mut (*audioCard).sapmDirty);
                break;
            }
        }
        path = ((*path).list.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut AudioSapmpath)).list as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_int as *mut AudioSapmpath;
    }
    AudioSapmPowerComponents(audioCard);
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioCodecSapmGetCtrlOps(
    mut kcontrol: *const AudioKcontrol,
    mut elemValue: *mut AudioCtrlElemValue,
) -> int32_t {
    if AudioCodecGetCtrlOps(kcontrol, elemValue) != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio codec sapm get control switch is fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"AudioCodecSapmGetCtrlOps\0"))
                .as_ptr(),
            1306 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioSapmSetCtrlOps(
    mut kcontrol: *const AudioKcontrol,
    mut elemValue: *const AudioCtrlElemValue,
    mut value: *mut uint32_t,
    mut pathStatus: *mut uint32_t,
) -> int32_t {
    let mut mixerCtrl: *mut AudioMixerControl = 0 as *mut AudioMixerControl;
    let mut iFlag: int32_t = (kcontrol.is_null()
        || (*kcontrol).privateValue <= 0 as core::ffi::c_ulong || elemValue.is_null()
        || value.is_null() || pathStatus.is_null()) as core::ffi::c_int;
    if iFlag != 0 {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params invalid.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioSapmSetCtrlOps\0"))
                .as_ptr(),
            1321 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    mixerCtrl = (*kcontrol).privateValue as uintptr_t as *mut AudioMixerControl;
    *value = (*elemValue).value[0 as core::ffi::c_int as usize];
    if *value < (*mixerCtrl).min || *value > (*mixerCtrl).max {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: value is invalid.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioSapmSetCtrlOps\0"))
                .as_ptr(),
            1328 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if *value != 0 {
        *pathStatus = CONNECT_SINK_AND_SOURCE as uint32_t;
    } else {
        *pathStatus = UNCONNECT_SINK_AND_SOURCE as uint32_t;
    }
    if (*mixerCtrl).invert != 0 {
        *value = ((*mixerCtrl).max).wrapping_sub(*value);
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioCodecSapmSetCtrlOps(
    mut kcontrol: *const AudioKcontrol,
    mut elemValue: *const AudioCtrlElemValue,
) -> int32_t {
    let mut value: uint32_t = 0;
    let mut pathStatus: uint32_t = 0 as uint32_t;
    let mut codec: *mut CodecDevice = 0 as *mut CodecDevice;
    let mut mixerCtrl: *mut AudioMixerControl = 0 as *mut AudioMixerControl;
    if kcontrol.is_null() || (*kcontrol).privateValue <= 0 as core::ffi::c_ulong
        || ((*kcontrol).pri).is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params: kcontrol is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"AudioCodecSapmSetCtrlOps\0"))
                .as_ptr(),
            1352 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if elemValue.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params: elemValue is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"AudioCodecSapmSetCtrlOps\0"))
                .as_ptr(),
            1356 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    mixerCtrl = (*kcontrol).privateValue as uintptr_t as *mut AudioMixerControl;
    if AudioSapmSetCtrlOps(kcontrol, elemValue, &mut value, &mut pathStatus)
        != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio sapm put control switch fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"AudioCodecSapmSetCtrlOps\0"))
                .as_ptr(),
            1362 as core::ffi::c_int,
        );
    }
    codec = AudioKcontrolGetCodec(kcontrol);
    if MixerUpdatePowerStatus(kcontrol, pathStatus) != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: update power status is failure!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"AudioCodecSapmSetCtrlOps\0"))
                .as_ptr(),
            1367 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*mixerCtrl).value = (*elemValue).value[0 as core::ffi::c_int as usize];
    if AudioCodecRegUpdate(codec, mixerCtrl) != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: update reg bits fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"AudioCodecSapmSetCtrlOps\0"))
                .as_ptr(),
            1373 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioCodecCheckRegIsChange(
    mut enumCtrl: *mut AudioEnumKcontrol,
    mut elemValue: *const AudioCtrlElemValue,
    mut curValue: uint32_t,
    mut change: *mut bool,
) -> int32_t {
    let mut value: uint32_t = 0;
    let mut mask: uint32_t = 0;
    let mut oldValue: uint32_t = 0;
    if enumCtrl.is_null() || elemValue.is_null() || change.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input para is null!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"AudioCodecCheckRegIsChange\0"))
                .as_ptr(),
            1387 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    *change = false_0 != 0;
    if (*elemValue).value[0 as core::ffi::c_int as usize] > (*enumCtrl).max {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: elemValue value[0] out of range!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"AudioCodecCheckRegIsChange\0"))
                .as_ptr(),
            1393 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if !((*enumCtrl).values).is_null() {
        value = *((*enumCtrl).values)
            .offset((*elemValue).value[0 as core::ffi::c_int as usize] as isize)
            << (*enumCtrl).shiftLeft as core::ffi::c_int;
        mask = (*enumCtrl).mask << (*enumCtrl).shiftLeft as core::ffi::c_int;
        if (*enumCtrl).shiftLeft as core::ffi::c_int
            != (*enumCtrl).shiftRight as core::ffi::c_int
        {
            if (*elemValue).value[1 as core::ffi::c_int as usize] > (*enumCtrl).max {
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    LOG_DOMAIN as core::ffi::c_uint,
                    b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s][line:%d]: elemValue value[1] out of range!\0" as *const u8
                        as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 27],
                        [core::ffi::c_char; 27],
                    >(*b"AudioCodecCheckRegIsChange\0"))
                        .as_ptr(),
                    1402 as core::ffi::c_int,
                );
                return HDF_FAILURE as core::ffi::c_int as int32_t;
            }
            value
                |= (*((*enumCtrl).values)
                    .offset((*elemValue).value[1 as core::ffi::c_int as usize] as isize)
                    << (*enumCtrl).shiftRight as core::ffi::c_int) as core::ffi::c_uint;
            mask
                |= ((*enumCtrl).mask << (*enumCtrl).shiftRight as core::ffi::c_int)
                    as core::ffi::c_uint;
        }
    } else {
        value = (*elemValue).value[0 as core::ffi::c_int as usize]
            << (*enumCtrl).shiftLeft as core::ffi::c_int;
        mask = (*enumCtrl).mask << (*enumCtrl).shiftLeft as core::ffi::c_int;
        if (*enumCtrl).shiftLeft as core::ffi::c_int
            != (*enumCtrl).shiftRight as core::ffi::c_int
        {
            if (*elemValue).value[1 as core::ffi::c_int as usize] > (*enumCtrl).max {
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    LOG_DOMAIN as core::ffi::c_uint,
                    b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s][line:%d]: elemValue value[1] out of range!\0" as *const u8
                        as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 27],
                        [core::ffi::c_char; 27],
                    >(*b"AudioCodecCheckRegIsChange\0"))
                        .as_ptr(),
                    1413 as core::ffi::c_int,
                );
                return HDF_FAILURE as core::ffi::c_int as int32_t;
            }
            value
                |= ((*elemValue).value[1 as core::ffi::c_int as usize]
                    << (*enumCtrl).shiftRight as core::ffi::c_int) as core::ffi::c_uint;
            mask
                |= ((*enumCtrl).mask << (*enumCtrl).shiftRight as core::ffi::c_int)
                    as core::ffi::c_uint;
        }
    }
    oldValue = curValue & mask;
    if oldValue != value {
        *change = true_0 != 0;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioCodecSapmSetEnumCtrlOps(
    mut kcontrol: *const AudioKcontrol,
    mut elemValue: *const AudioCtrlElemValue,
) -> int32_t {
    let mut curValue: uint32_t = 0;
    let mut change: bool = false;
    let mut ret: int32_t = 0;
    let mut codec: *mut CodecDevice = 0 as *mut CodecDevice;
    let mut enumCtrl: *mut AudioEnumKcontrol = 0 as *mut AudioEnumKcontrol;
    if kcontrol.is_null() || (*kcontrol).privateValue <= 0 as core::ffi::c_ulong
        || elemValue.is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params: kcontrol is NULL or elemValue is NULL\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"AudioCodecSapmSetEnumCtrlOps\0"))
                .as_ptr(),
            1438 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    codec = AudioKcontrolGetCodec(kcontrol);
    enumCtrl = (*kcontrol).privateValue as uintptr_t as *mut AudioEnumKcontrol;
    if enumCtrl.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: privateValue is null\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"AudioCodecSapmSetEnumCtrlOps\0"))
                .as_ptr(),
            1445 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioCodecReadReg(codec, (*enumCtrl).reg, &mut curValue)
        != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Device read register is failure!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"AudioCodecSapmSetEnumCtrlOps\0"))
                .as_ptr(),
            1450 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    ret = AudioCodecCheckRegIsChange(enumCtrl, elemValue, curValue, &mut change);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: AudioCodecCheckRegIsChange is failure!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"AudioCodecSapmSetEnumCtrlOps\0"))
                .as_ptr(),
            1456 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if change {
        if MuxUpdatePowerStatus(
            kcontrol,
            (*elemValue).value[0 as core::ffi::c_int as usize] as int32_t,
            enumCtrl,
        ) != HDF_SUCCESS as core::ffi::c_int
        {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: update power status is failure!\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 29],
                    [core::ffi::c_char; 29],
                >(*b"AudioCodecSapmSetEnumCtrlOps\0"))
                    .as_ptr(),
                1462 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
        ret = AudioCodecMuxRegUpdate(codec, enumCtrl, ((*elemValue).value).as_ptr());
        if ret != HDF_SUCCESS as core::ffi::c_int {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: AudioCodecMuxRegUpdate is failure!\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 29],
                    [core::ffi::c_char; 29],
                >(*b"AudioCodecSapmSetEnumCtrlOps\0"))
                    .as_ptr(),
                1468 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioCodecSapmGetEnumCtrlOps(
    mut kcontrol: *const AudioKcontrol,
    mut elemValue: *mut AudioCtrlElemValue,
) -> int32_t {
    if AudioCodecGetEnumCtrlOps(kcontrol, elemValue) != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio codec sapm get control switch is fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"AudioCodecSapmGetEnumCtrlOps\0"))
                .as_ptr(),
            1479 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioSapmRefreshTime(
    mut audioCard: *mut AudioCard,
    mut bRefresh: bool,
) -> int32_t {
    if audioCard.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"AudioSapmRefreshTime\0"))
                .as_ptr(),
            1489 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if bRefresh {
        (*audioCard).time = 0 as uint64_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioSapmCheckTime(
    mut audioCard: *mut AudioCard,
    mut timeoutStatus: *mut bool,
) -> int32_t {
    let mut ret: int32_t = 0;
    if audioCard.is_null() || timeoutStatus.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioSapmCheckTime\0"))
                .as_ptr(),
            1504 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    ret = AudioSapmRefreshTime(audioCard, false_0 != 0);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: AudioSapmRefreshTime failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioSapmCheckTime\0"))
                .as_ptr(),
            1510 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    *timeoutStatus = if (*audioCard).time > SAPM_SLEEP_TIMES as core::ffi::c_ulonglong {
        true_0
    } else {
        false_0
    } != 0;
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSampPowerUp(mut card: *const AudioCard) -> int32_t {
    let mut upList: DListHead = DListHead {
        next: 0 as *mut DListHead,
        prev: 0 as *mut DListHead,
    };
    let mut sapmComponent: *mut AudioSapmComponent = 0 as *mut AudioSapmComponent;
    if card.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params is null.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioSampPowerUp\0"))
                .as_ptr(),
            1524 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    DListHeadInit(&mut upList);
    sapmComponent = ((*card).components.next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut AudioSapmComponent)).list as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_int as *mut AudioSapmComponent;
    while &mut (*sapmComponent).list as *mut DListHead
        != &(*card).components as *const DListHead as *mut DListHead
    {
        if sapmComponent.is_null() {
            break;
        }
        if (*sapmComponent).power as core::ffi::c_int == SAPM_POWER_DOWN {
            AudioSapmPowerSeqInsert(sapmComponent, &mut upList, SAPM_POWER_UP as int8_t);
        }
        sapmComponent = ((*sapmComponent).list.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut AudioSapmComponent)).list as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_int as *mut AudioSapmComponent;
    }
    AudioSapmPowerUpSeqRun(&mut upList);
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSampSetPowerMonitor(
    mut card: *mut AudioCard,
    mut powerMonitorState: bool,
) -> int32_t {
    if card.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input params is null.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"AudioSampSetPowerMonitor\0"))
                .as_ptr(),
            1545 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    (*card).sapmMonitorState = powerMonitorState;
    if powerMonitorState as core::ffi::c_int == false_0 {
        (*card).sapmSleepState = false_0 != 0;
        (*card).sapmStandbyState = false_0 != 0;
        (*card).sapmStandbyStartTimeFlag = false_0 != 0;
        (*card).sapmSleepStartTimeFlag = false_0 != 0;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioSapmEnterSleep(mut audioCard: *mut AudioCard) {
    let mut downList: DListHead = DListHead {
        next: 0 as *mut DListHead,
        prev: 0 as *mut DListHead,
    };
    let mut sapmComponent: *mut AudioSapmComponent = 0 as *mut AudioSapmComponent;
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        LOG_DOMAIN as core::ffi::c_uint,
        b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
        b"[%s][line:%d]: entry!\0" as *const u8 as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 20],
            [core::ffi::c_char; 20],
        >(*b"AudioSapmEnterSleep\0"))
            .as_ptr(),
        1563 as core::ffi::c_int,
    );
    DListHeadInit(&mut downList);
    if audioCard.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: audioCard is null.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioSapmEnterSleep\0"))
                .as_ptr(),
            1567 as core::ffi::c_int,
        );
        return;
    }
    sapmComponent = ((*audioCard).components.next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut AudioSapmComponent)).list as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_int as *mut AudioSapmComponent;
    while &mut (*sapmComponent).list as *mut DListHead
        != &mut (*audioCard).components as *mut DListHead
    {
        if sapmComponent.is_null() {
            break;
        }
        if (*sapmComponent).power as core::ffi::c_int == SAPM_POWER_UP {
            AudioSapmPowerSeqInsert(
                sapmComponent,
                &mut downList,
                SAPM_POWER_DOWN as int8_t,
            );
        }
        sapmComponent = ((*sapmComponent).list.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut AudioSapmComponent)).list as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_int as *mut AudioSapmComponent;
    }
    AudioSapmPowerDownSeqRun(&mut downList);
    (*audioCard).sapmStandbyState = false_0 != 0;
    (*audioCard).sapmSleepState = true_0 != 0;
}
unsafe extern "C" fn AudioSapmEnterStandby(mut audioCard: *mut AudioCard) -> bool {
    let mut timeoutStatus: bool = false;
    let mut sapmComponent: *mut AudioSapmComponent = 0 as *mut AudioSapmComponent;
    if audioCard.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: audioCard is null.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioSapmEnterStandby\0"))
                .as_ptr(),
            1589 as core::ffi::c_int,
        );
        return false_0 != 0;
    }
    if (*audioCard).sapmStandbyStartTimeFlag as core::ffi::c_int == false_0 {
        if AudioSapmRefreshTime(audioCard, true_0 != 0)
            != HDF_SUCCESS as core::ffi::c_int
        {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: AudioSapmRefreshTime failed.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 22],
                    [core::ffi::c_char; 22],
                >(*b"AudioSapmEnterStandby\0"))
                    .as_ptr(),
                1595 as core::ffi::c_int,
            );
            return false_0 != 0;
        }
        (*audioCard).sapmStandbyStartTimeFlag = true_0 != 0;
    }
    if (*audioCard).standbyMode as core::ffi::c_uint
        != AUDIO_SAPM_TURN_STANDBY_NOW as core::ffi::c_int as core::ffi::c_uint
    {
        if AudioSapmCheckTime(audioCard, &mut timeoutStatus)
            != HDF_SUCCESS as core::ffi::c_int
        {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: AudioSapmCheckTime failed.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 22],
                    [core::ffi::c_char; 22],
                >(*b"AudioSapmEnterStandby\0"))
                    .as_ptr(),
                1603 as core::ffi::c_int,
            );
            return false_0 != 0;
        }
        if !timeoutStatus {
            return false_0 != 0;
        }
    }
    if (*audioCard).sapmStandbyState as core::ffi::c_int == false_0 {
        sapmComponent = ((*audioCard).components.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut AudioSapmComponent)).list as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_int as *mut AudioSapmComponent;
        while &mut (*sapmComponent).list as *mut DListHead
            != &mut (*audioCard).components as *mut DListHead
        {
            if ((*sapmComponent).PowerClockOp).is_some() {
                ((*sapmComponent).PowerClockOp)
                    .expect("non-null function pointer")(sapmComponent);
            }
            sapmComponent = ((*sapmComponent).list.next as *mut core::ffi::c_char)
                .offset_from(
                    &mut (*(0 as *mut AudioSapmComponent)).list as *mut DListHead
                        as *mut core::ffi::c_char,
                ) as core::ffi::c_int as *mut AudioSapmComponent;
        }
        (*audioCard).sapmStandbyState = true_0 != 0;
    }
    return true_0 != 0;
}
unsafe extern "C" fn AudioSapmTimerCallback(mut audioCard: *mut AudioCard) {
    let mut timeoutStatus: bool = false;
    let mut standbyEntry: bool = false;
    if audioCard.is_null() {
        return;
    }
    if (*audioCard).sapmSleepState as core::ffi::c_int == true_0 {
        return;
    }
    if (*audioCard).sapmMonitorState as core::ffi::c_int == false_0 {
        return;
    }
    standbyEntry = AudioSapmEnterStandby(audioCard);
    if !standbyEntry {
        return;
    }
    if (*audioCard).sapmSleepStartTimeFlag as core::ffi::c_int == false_0 {
        if AudioSapmRefreshTime(audioCard, true_0 != 0)
            != HDF_SUCCESS as core::ffi::c_int
        {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: AudioSapmRefreshTime failed.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 23],
                    [core::ffi::c_char; 23],
                >(*b"AudioSapmTimerCallback\0"))
                    .as_ptr(),
                1651 as core::ffi::c_int,
            );
            return;
        }
        (*audioCard).sapmSleepStartTimeFlag = true_0 != 0;
    }
    if AudioSapmCheckTime(audioCard, &mut timeoutStatus)
        != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_SAPM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: AudioSapmCheckTime failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioSapmTimerCallback\0"))
                .as_ptr(),
            1658 as core::ffi::c_int,
        );
        return;
    }
    if !timeoutStatus {
        return;
    }
    AudioSapmEnterSleep(audioCard);
}
pub const NULL_0: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;
