extern "C" {
    pub type HdfSBuf;
    fn strcmp(
        _: *const core::ffi::c_char,
        _: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn HdfDeviceSetClass(
        deviceObject: *mut HdfDeviceObject,
        deviceClass: DeviceClass,
    ) -> bool;
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
    fn AudioFillConfigData(
        device: *const HdfDeviceObject,
        configData: *mut AudioConfigData,
    ) -> int32_t;
    fn AudioBindDaiLink(
        audioCard: *mut AudioCard,
        configData: *const AudioConfigData,
    ) -> int32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfDriverEntry {
    pub moduleVersion: int32_t,
    pub moduleName: *const core::ffi::c_char,
    pub Bind: Option<unsafe extern "C" fn(*mut HdfDeviceObject) -> int32_t>,
    pub Init: Option<unsafe extern "C" fn(*mut HdfDeviceObject) -> int32_t>,
    pub Release: Option<unsafe extern "C" fn(*mut HdfDeviceObject) -> ()>,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioHost {
    pub service: IDeviceIoService,
    pub device: *mut HdfDeviceObject,
    pub priv_0: *mut core::ffi::c_void,
}
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
pub const NULL_0: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;
#[no_mangle]
pub static mut g_cardManager: DListHead = unsafe {
    {
        let mut init = DListHead {
            next: &g_cardManager as *const DListHead as *mut DListHead,
            prev: &g_cardManager as *const DListHead as *mut DListHead,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn GetAllCardInstance() -> *const DListHead {
    if DListIsEmpty(&mut g_cardManager) {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: g_cardManager is empty.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"GetAllCardInstance\0"))
                .as_ptr(),
            24 as core::ffi::c_int,
        );
        return 0 as *const DListHead;
    }
    return &mut g_cardManager;
}
#[no_mangle]
pub unsafe extern "C" fn GetCardInstance(
    mut serviceName: *const core::ffi::c_char,
) -> *mut AudioCard {
    let mut audioCard: *mut AudioCard = 0 as *mut AudioCard;
    if serviceName.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: serviceName is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"GetCardInstance\0"))
                .as_ptr(),
            37 as core::ffi::c_int,
        );
        return 0 as *mut AudioCard;
    }
    if DListIsEmpty(&mut g_cardManager) {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: g_cardManager is empty.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"GetCardInstance\0"))
                .as_ptr(),
            42 as core::ffi::c_int,
        );
        return 0 as *mut AudioCard;
    }
    audioCard = (g_cardManager.next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut AudioCard)).list as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_int as *mut AudioCard;
    while &mut (*audioCard).list as *mut DListHead
        != &mut g_cardManager as *mut DListHead
    {
        if ((*audioCard).configData.cardServiceName).is_null() {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: cardServiceName is NULL.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"GetCardInstance\0"))
                    .as_ptr(),
                48 as core::ffi::c_int,
            );
            return 0 as *mut AudioCard;
        }
        if strcmp((*audioCard).configData.cardServiceName, serviceName)
            == 0 as core::ffi::c_int
        {
            return audioCard;
        }
        audioCard = ((*audioCard).list.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut AudioCard)).list as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_int as *mut AudioCard;
    }
    HiLogPrint(
        LOG_CORE,
        LOG_ERROR,
        LOG_DOMAIN as core::ffi::c_uint,
        b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
        b"[%s][line:%d]: serviceName is %s.\0" as *const u8 as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 16],
            [core::ffi::c_char; 16],
        >(*b"GetCardInstance\0"))
            .as_ptr(),
        56 as core::ffi::c_int,
        serviceName,
    );
    return 0 as *mut AudioCard;
}
unsafe extern "C" fn AudioCodecDevInit(mut audioCard: *mut AudioCard) -> int32_t {
    let mut rtd: *mut AudioRuntimeDeivces = 0 as *mut AudioRuntimeDeivces;
    let mut codec: *mut CodecDevice = 0 as *mut CodecDevice;
    if audioCard.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: audioCard is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"AudioCodecDevInit\0"))
                .as_ptr(),
            67 as core::ffi::c_int,
        );
        return HDF_ERR_IO as core::ffi::c_int as int32_t;
    }
    rtd = (*audioCard).rtd as *mut AudioRuntimeDeivces;
    if rtd.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: rtd is NULL.\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"AudioCodecDevInit\0"))
                .as_ptr(),
            72 as core::ffi::c_int,
        );
        return HDF_ERR_IO as core::ffi::c_int as int32_t;
    }
    codec = (*rtd).codec as *mut CodecDevice;
    if !codec.is_null() && !((*codec).devData).is_null()
        && ((*(*codec).devData).Init).is_some()
    {
        let mut ret: int32_t = ((*(*codec).devData).Init)
            .expect("non-null function pointer")(audioCard, codec);
        if ret != HDF_SUCCESS as core::ffi::c_int {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: codec initialization fail ret=%d\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 18],
                    [core::ffi::c_char; 18],
                >(*b"AudioCodecDevInit\0"))
                    .as_ptr(),
                80 as core::ffi::c_int,
                ret,
            );
            return HDF_ERR_IO as core::ffi::c_int as int32_t;
        }
    }
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        LOG_DOMAIN as core::ffi::c_uint,
        b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
        b"[%s][line:%d]: success.\0" as *const u8 as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 18],
            [core::ffi::c_char; 18],
        >(*b"AudioCodecDevInit\0"))
            .as_ptr(),
        85 as core::ffi::c_int,
    );
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioPlatformDevInit(mut audioCard: *const AudioCard) -> int32_t {
    let mut rtd: *mut AudioRuntimeDeivces = 0 as *mut AudioRuntimeDeivces;
    let mut platform: *mut PlatformDevice = 0 as *mut PlatformDevice;
    if audioCard.is_null() {
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
            >(*b"AudioPlatformDevInit\0"))
                .as_ptr(),
            95 as core::ffi::c_int,
        );
        return HDF_ERR_IO as core::ffi::c_int as int32_t;
    }
    rtd = (*audioCard).rtd as *mut AudioRuntimeDeivces;
    if rtd.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Platform rtd is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"AudioPlatformDevInit\0"))
                .as_ptr(),
            102 as core::ffi::c_int,
        );
        return HDF_ERR_IO as core::ffi::c_int as int32_t;
    }
    platform = (*rtd).platform as *mut PlatformDevice;
    if !platform.is_null() && !((*platform).devData).is_null()
        && ((*(*platform).devData).PlatformInit).is_some()
    {
        let mut ret: int32_t = ((*(*platform).devData).PlatformInit)
            .expect("non-null function pointer")(audioCard, platform);
        if ret != HDF_SUCCESS as core::ffi::c_int {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: platform initialization fail ret=%d\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 21],
                    [core::ffi::c_char; 21],
                >(*b"AudioPlatformDevInit\0"))
                    .as_ptr(),
                110 as core::ffi::c_int,
                ret,
            );
            return HDF_ERR_IO as core::ffi::c_int as int32_t;
        }
    }
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        LOG_DOMAIN as core::ffi::c_uint,
        b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
        b"[%s][line:%d]: success.\0" as *const u8 as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 21],
            [core::ffi::c_char; 21],
        >(*b"AudioPlatformDevInit\0"))
            .as_ptr(),
        115 as core::ffi::c_int,
    );
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioDspDevInit(mut audioCard: *const AudioCard) -> int32_t {
    let mut rtd: *mut AudioRuntimeDeivces = 0 as *mut AudioRuntimeDeivces;
    let mut dsp: *mut DspDevice = 0 as *mut DspDevice;
    if audioCard.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: audioCard is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"AudioDspDevInit\0"))
                .as_ptr(),
            125 as core::ffi::c_int,
        );
        return HDF_ERR_IO as core::ffi::c_int as int32_t;
    }
    rtd = (*audioCard).rtd as *mut AudioRuntimeDeivces;
    if rtd.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: audioCard rtd object is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"AudioDspDevInit\0"))
                .as_ptr(),
            132 as core::ffi::c_int,
        );
        return HDF_ERR_IO as core::ffi::c_int as int32_t;
    }
    dsp = (*rtd).dsp as *mut DspDevice;
    if !dsp.is_null() && !((*dsp).devData).is_null()
        && ((*(*dsp).devData).DspInit).is_some()
    {
        let mut ret: int32_t = ((*(*dsp).devData).DspInit)
            .expect("non-null function pointer")(dsp);
        if ret != HDF_SUCCESS as core::ffi::c_int {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: dsp initialization fail ret=%d\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"AudioDspDevInit\0"))
                    .as_ptr(),
                141 as core::ffi::c_int,
                ret,
            );
            return HDF_ERR_IO as core::ffi::c_int as int32_t;
        }
    }
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        LOG_DOMAIN as core::ffi::c_uint,
        b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
        b"[%s][line:%d]: success.\0" as *const u8 as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 16],
            [core::ffi::c_char; 16],
        >(*b"AudioDspDevInit\0"))
            .as_ptr(),
        146 as core::ffi::c_int,
    );
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioCodecDaiDevInit(mut audioCard: *mut AudioCard) -> int32_t {
    let mut rtd: *mut AudioRuntimeDeivces = 0 as *mut AudioRuntimeDeivces;
    let mut codecDai: *mut DaiDevice = 0 as *mut DaiDevice;
    if audioCard.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: audioCard is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"AudioCodecDaiDevInit\0"))
                .as_ptr(),
            156 as core::ffi::c_int,
        );
        return HDF_ERR_IO as core::ffi::c_int as int32_t;
    }
    rtd = (*audioCard).rtd as *mut AudioRuntimeDeivces;
    if rtd.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: CodecDai rtd is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"AudioCodecDaiDevInit\0"))
                .as_ptr(),
            163 as core::ffi::c_int,
        );
        return HDF_ERR_IO as core::ffi::c_int as int32_t;
    }
    codecDai = (*rtd).codecDai as *mut DaiDevice;
    if !codecDai.is_null() && !((*codecDai).devData).is_null()
        && ((*(*codecDai).devData).DaiInit).is_some()
    {
        let mut ret: int32_t = ((*(*codecDai).devData).DaiInit)
            .expect("non-null function pointer")(audioCard, codecDai);
        if ret != HDF_SUCCESS as core::ffi::c_int {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: codec dai initialization fail ret=%d\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 21],
                    [core::ffi::c_char; 21],
                >(*b"AudioCodecDaiDevInit\0"))
                    .as_ptr(),
                171 as core::ffi::c_int,
                ret,
            );
            return HDF_ERR_IO as core::ffi::c_int as int32_t;
        }
    }
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        LOG_DOMAIN as core::ffi::c_uint,
        b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
        b"[%s][line:%d]: success.\0" as *const u8 as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 21],
            [core::ffi::c_char; 21],
        >(*b"AudioCodecDaiDevInit\0"))
            .as_ptr(),
        176 as core::ffi::c_int,
    );
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioCpuDaiDevInit(mut audioCard: *mut AudioCard) -> int32_t {
    let mut rtd: *mut AudioRuntimeDeivces = 0 as *mut AudioRuntimeDeivces;
    let mut cpuDai: *mut DaiDevice = 0 as *mut DaiDevice;
    if audioCard.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: audioCard is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioCpuDaiDevInit\0"))
                .as_ptr(),
            186 as core::ffi::c_int,
        );
        return HDF_ERR_IO as core::ffi::c_int as int32_t;
    }
    rtd = (*audioCard).rtd as *mut AudioRuntimeDeivces;
    if rtd.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: cpuDai rtd is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioCpuDaiDevInit\0"))
                .as_ptr(),
            193 as core::ffi::c_int,
        );
        return HDF_ERR_IO as core::ffi::c_int as int32_t;
    }
    cpuDai = (*rtd).cpuDai;
    if !cpuDai.is_null() && !((*cpuDai).devData).is_null()
        && ((*(*cpuDai).devData).DaiInit).is_some()
    {
        let mut ret: int32_t = ((*(*cpuDai).devData).DaiInit)
            .expect("non-null function pointer")(audioCard, cpuDai);
        if ret != HDF_SUCCESS as core::ffi::c_int {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: cpu dai initialization fail ret=%d\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 19],
                    [core::ffi::c_char; 19],
                >(*b"AudioCpuDaiDevInit\0"))
                    .as_ptr(),
                201 as core::ffi::c_int,
                ret,
            );
            return HDF_ERR_IO as core::ffi::c_int as int32_t;
        }
    }
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        LOG_DOMAIN as core::ffi::c_uint,
        b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
        b"[%s][line:%d]: success.\0" as *const u8 as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 19],
            [core::ffi::c_char; 19],
        >(*b"AudioCpuDaiDevInit\0"))
            .as_ptr(),
        206 as core::ffi::c_int,
    );
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioDspDaiDevInit(mut audioCard: *mut AudioCard) -> int32_t {
    let mut rtd: *mut AudioRuntimeDeivces = 0 as *mut AudioRuntimeDeivces;
    let mut dspDai: *mut DaiDevice = 0 as *mut DaiDevice;
    let mut ret: int32_t = 0;
    if audioCard.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: audioCard is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioDspDaiDevInit\0"))
                .as_ptr(),
            217 as core::ffi::c_int,
        );
        return HDF_ERR_IO as core::ffi::c_int as int32_t;
    }
    rtd = (*audioCard).rtd as *mut AudioRuntimeDeivces;
    if rtd.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: dspDai rtd is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioDspDaiDevInit\0"))
                .as_ptr(),
            224 as core::ffi::c_int,
        );
        return HDF_ERR_IO as core::ffi::c_int as int32_t;
    }
    dspDai = (*rtd).dspDai;
    if !dspDai.is_null() && !((*dspDai).devData).is_null()
        && ((*(*dspDai).devData).DaiInit).is_some()
    {
        ret = ((*(*dspDai).devData).DaiInit)
            .expect("non-null function pointer")(audioCard, dspDai);
        if ret != HDF_SUCCESS as core::ffi::c_int {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: dsp dai initialization fail ret=%d\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 19],
                    [core::ffi::c_char; 19],
                >(*b"AudioDspDaiDevInit\0"))
                    .as_ptr(),
                232 as core::ffi::c_int,
                ret,
            );
            return HDF_ERR_IO as core::ffi::c_int as int32_t;
        }
    }
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        LOG_DOMAIN as core::ffi::c_uint,
        b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
        b"[%s][line:%d]: success.\0" as *const u8 as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 19],
            [core::ffi::c_char; 19],
        >(*b"AudioDspDaiDevInit\0"))
            .as_ptr(),
        237 as core::ffi::c_int,
    );
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioInitDaiLink(mut audioCard: *mut AudioCard) -> int32_t {
    if audioCard.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: audioCard is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioInitDaiLink\0"))
                .as_ptr(),
            244 as core::ffi::c_int,
        );
        return HDF_ERR_IO as core::ffi::c_int as int32_t;
    }
    if AudioPlatformDevInit(audioCard) != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Platform init fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioInitDaiLink\0"))
                .as_ptr(),
            250 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioCpuDaiDevInit(audioCard) != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: CpuDai init fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioInitDaiLink\0"))
                .as_ptr(),
            255 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioCodecDevInit(audioCard) != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: codec Device init fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioInitDaiLink\0"))
                .as_ptr(),
            259 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioCodecDaiDevInit(audioCard) != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: CodecDai Device init fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioInitDaiLink\0"))
                .as_ptr(),
            264 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioDspDevInit(audioCard) != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Dsp Device init fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioInitDaiLink\0"))
                .as_ptr(),
            269 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioDspDaiDevInit(audioCard) != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: DspDai Device init fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioInitDaiLink\0"))
                .as_ptr(),
            274 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioHostCreateAndBind(
    mut device: *mut HdfDeviceObject,
) -> *mut AudioHost {
    let mut audioHost: *mut AudioHost = 0 as *mut AudioHost;
    if device.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: device is NULL!\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioHostCreateAndBind\0"))
                .as_ptr(),
            287 as core::ffi::c_int,
        );
        return 0 as *mut AudioHost;
    }
    audioHost = OsalMemCalloc(::core::mem::size_of::<AudioHost>() as size_t)
        as *mut AudioHost;
    if audioHost.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Malloc audio host fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioHostCreateAndBind\0"))
                .as_ptr(),
            293 as core::ffi::c_int,
        );
        return 0 as *mut AudioHost;
    }
    (*audioHost).device = device;
    (*device).service = &mut (*audioHost).service as *mut IDeviceIoService;
    return audioHost;
}
unsafe extern "C" fn AudioDriverBind(mut device: *mut HdfDeviceObject) -> int32_t {
    let mut audioHost: *mut AudioHost = 0 as *mut AudioHost;
    if device.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: device is NULL.\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"AudioDriverBind\0"))
                .as_ptr(),
            308 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    audioHost = AudioHostCreateAndBind(device);
    if audioHost.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: audioHost create failed!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"AudioDriverBind\0"))
                .as_ptr(),
            314 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        LOG_DOMAIN as core::ffi::c_uint,
        b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
        b"[%s][line:%d]: success.\0" as *const u8 as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 16],
            [core::ffi::c_char; 16],
        >(*b"AudioDriverBind\0"))
            .as_ptr(),
        318 as core::ffi::c_int,
    );
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioCardInit(
    mut device: *mut HdfDeviceObject,
    mut audioHost: *mut AudioHost,
) -> int32_t {
    let mut ret: int32_t = 0;
    let mut audioCard: *mut AudioCard = 0 as *mut AudioCard;
    if device.is_null() || audioHost.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: device or audioHost is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"AudioCardInit\0"))
                .as_ptr(),
            327 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    audioCard = OsalMemCalloc(::core::mem::size_of::<AudioCard>() as size_t)
        as *mut AudioCard;
    if audioCard.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Malloc audioCard fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"AudioCardInit\0"))
                .as_ptr(),
            333 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*audioHost).priv_0 = audioCard as *mut core::ffi::c_void;
    ret = AudioFillConfigData(device, &mut (*audioCard).configData);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: AudioFillConfigData fail ret=%d\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"AudioCardInit\0"))
                .as_ptr(),
            340 as core::ffi::c_int,
            ret,
        );
        return HDF_ERR_IO as core::ffi::c_int as int32_t;
    }
    (*audioCard).device = device;
    (*audioCard).standbyMode = AUDIO_SAPM_TURN_STANDBY_LATER;
    ret = AudioBindDaiLink(audioCard, &mut (*audioCard).configData);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: AudioBindDaiLink fail ret=%d\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"AudioCardInit\0"))
                .as_ptr(),
            350 as core::ffi::c_int,
            ret,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if ((*audioCard).rtd).is_null() || (*(*audioCard).rtd).complete == 0 {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: AudioBindDaiLink fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"AudioCardInit\0"))
                .as_ptr(),
            354 as core::ffi::c_int,
        );
        return HDF_ERR_IO as core::ffi::c_int as int32_t;
    }
    DListHeadInit(&mut (*audioCard).list);
    DListHeadInit(&mut (*audioCard).controls);
    DListHeadInit(&mut (*audioCard).components);
    DListHeadInit(&mut (*audioCard).paths);
    DListHeadInit(&mut (*audioCard).sapmDirty);
    ret = AudioInitDaiLink(audioCard);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: AudioInitDaiLink fail ret=%d\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"AudioCardInit\0"))
                .as_ptr(),
            367 as core::ffi::c_int,
            ret,
        );
        return HDF_ERR_IO as core::ffi::c_int as int32_t;
    }
    DListInsertHead(&mut (*audioCard).list, &mut g_cardManager);
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioDriverInit(mut device: *mut HdfDeviceObject) -> int32_t {
    let mut audioHost: *mut AudioHost = 0 as *mut AudioHost;
    if device.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: device is NULL.\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"AudioDriverInit\0"))
                .as_ptr(),
            383 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if !HdfDeviceSetClass(device, DEVICE_CLASS_AUDIO) {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: set audio class failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"AudioDriverInit\0"))
                .as_ptr(),
            387 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    audioHost = (*device).service as *mut AudioHost;
    if AudioCardInit(device, audioHost) != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: audio card init failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"AudioDriverInit\0"))
                .as_ptr(),
            392 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        LOG_DOMAIN as core::ffi::c_uint,
        b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
        b"[%s][line:%d]: success.\0" as *const u8 as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 16],
            [core::ffi::c_char; 16],
        >(*b"AudioDriverInit\0"))
            .as_ptr(),
        395 as core::ffi::c_int,
    );
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioDriverRelease(mut device: *mut HdfDeviceObject) {
    let mut audioHost: *mut AudioHost = 0 as *mut AudioHost;
    let mut audioCard: *mut AudioCard = 0 as *mut AudioCard;
    let mut componentHead: *mut DListHead = 0 as *mut DListHead;
    let mut controlHead: *mut DListHead = 0 as *mut DListHead;
    let mut componentReq: *mut AudioSapmComponent = 0 as *mut AudioSapmComponent;
    let mut componentTmp: *mut AudioSapmComponent = 0 as *mut AudioSapmComponent;
    let mut ctrlReq: *mut AudioKcontrol = 0 as *mut AudioKcontrol;
    let mut ctrlTmp: *mut AudioKcontrol = 0 as *mut AudioKcontrol;
    if device.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: device is NULL.\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioDriverRelease\0"))
                .as_ptr(),
            412 as core::ffi::c_int,
        );
        return;
    }
    audioHost = (*device).service as *mut AudioHost;
    if audioHost.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: audioHost is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioDriverRelease\0"))
                .as_ptr(),
            417 as core::ffi::c_int,
        );
        return;
    }
    if !((*audioHost).priv_0).is_null() {
        audioCard = (*audioHost).priv_0 as *mut AudioCard;
        componentHead = &mut (*audioCard).components;
        componentReq = ((*componentHead).next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut AudioSapmComponent)).list as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_int as *mut AudioSapmComponent;
        componentTmp = ((*componentReq).list.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut AudioSapmComponent)).list as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_int as *mut AudioSapmComponent;
        while &mut (*componentReq).list as *mut DListHead != componentHead {
            DListRemove(&mut (*componentReq).list);
            OsalMemFree((*componentReq).componentName as *mut core::ffi::c_void);
            OsalMemFree(componentReq as *mut core::ffi::c_void);
            componentReq = componentTmp;
            componentTmp = ((*componentReq).list.next as *mut core::ffi::c_char)
                .offset_from(
                    &mut (*(0 as *mut AudioSapmComponent)).list as *mut DListHead
                        as *mut core::ffi::c_char,
                ) as core::ffi::c_int as *mut AudioSapmComponent;
        }
        controlHead = &mut (*audioCard).controls;
        ctrlReq = ((*controlHead).next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut AudioKcontrol)).list as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_int as *mut AudioKcontrol;
        ctrlTmp = ((*ctrlReq).list.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut AudioKcontrol)).list as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_int as *mut AudioKcontrol;
        while &mut (*ctrlReq).list as *mut DListHead != controlHead {
            DListRemove(&mut (*ctrlReq).list);
            OsalMemFree((*ctrlReq).privateData);
            OsalMemFree(ctrlReq as *mut core::ffi::c_void);
            ctrlReq = ctrlTmp;
            ctrlTmp = ((*ctrlReq).list.next as *mut core::ffi::c_char)
                .offset_from(
                    &mut (*(0 as *mut AudioKcontrol)).list as *mut DListHead
                        as *mut core::ffi::c_char,
                ) as core::ffi::c_int as *mut AudioKcontrol;
        }
        DListRemove(&mut (*audioCard).list);
        OsalMemFree((*audioCard).rtd as *mut core::ffi::c_void);
        OsalMemFree((*audioHost).priv_0);
    }
    OsalMemFree(audioHost as *mut core::ffi::c_void);
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        LOG_DOMAIN as core::ffi::c_uint,
        b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
        b"[%s][line:%d]: success.\0" as *const u8 as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 19],
            [core::ffi::c_char; 19],
        >(*b"AudioDriverRelease\0"))
            .as_ptr(),
        445 as core::ffi::c_int,
    );
}
#[no_mangle]
pub static mut g_audioDriverEntry: HdfDriverEntry = unsafe {
    {
        let mut init = HdfDriverEntry {
            moduleVersion: 1 as int32_t,
            moduleName: b"HDF_AUDIO\0" as *const u8 as *const core::ffi::c_char,
            Bind: Some(
                AudioDriverBind as unsafe extern "C" fn(*mut HdfDeviceObject) -> int32_t,
            ),
            Init: Some(
                AudioDriverInit as unsafe extern "C" fn(*mut HdfDeviceObject) -> int32_t,
            ),
            Release: Some(
                AudioDriverRelease as unsafe extern "C" fn(*mut HdfDeviceObject) -> (),
            ),
        };
        init
    }
};
#[no_mangle]
#[used]
#[link_section = ".hdf.driver"]
pub static mut g_audioDriverEntryHdfEntry: size_t = 0;
pub const LOG_DOMAIN: core::ffi::c_int = 0xd002510 as core::ffi::c_int;
unsafe extern "C" fn run_static_initializers() {
    g_audioDriverEntryHdfEntry = &mut g_audioDriverEntry as *mut HdfDriverEntry
        as size_t;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
