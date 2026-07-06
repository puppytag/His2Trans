//! Module: src_audio_parse
//!
//! Auto-generated skeleton - function bodies are unimplemented.

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::types::*;
use crate::globals::*;
use crate::compat::*;

// === C2R_FILE_STATICS_BEGIN ===
// File-scope `static` variables (internal linkage) from the original C TU.
// These are module-local by design (Scheme B).

/// Safe Sync wrapper for raw C string pointers used in statics.
#[derive(Copy, Clone)]
pub(crate) struct CCharPtr(pub *const ::core::ffi::c_char);
// SAFETY: The inner pointer is read-only and never mutated after initialization;
// the pointed string literals have static lifetime.
unsafe impl Send for CCharPtr {}
unsafe impl Sync for CCharPtr {}

/// C: static char *[12] g_audioRegGroupName
static g_audioRegGroupName: [CCharPtr; 12usize] = [
    CCharPtr(b"resetSeqConfig\0".as_ptr() as *const ::core::ffi::c_char),
    CCharPtr(b"initSeqConfig\0".as_ptr() as *const ::core::ffi::c_char),
    CCharPtr(b"ctrlParamsSeqConfig\0".as_ptr() as *const ::core::ffi::c_char),
    CCharPtr(b"ctrlParamsMuxSeqConfig\0".as_ptr() as *const ::core::ffi::c_char),
    CCharPtr(b"ctrlSapmParamsSeqConfig\0".as_ptr() as *const ::core::ffi::c_char),
    CCharPtr(b"ctrlSapmMuxParamsSeqConfig\0".as_ptr() as *const ::core::ffi::c_char),
    CCharPtr(b"daiStartupSeqConfig\0".as_ptr() as *const ::core::ffi::c_char),
    CCharPtr(b"daiParamsSeqConfig\0".as_ptr() as *const ::core::ffi::c_char),
    CCharPtr(b"daiTriggerSeqConfig\0".as_ptr() as *const ::core::ffi::c_char),
    CCharPtr(b"controlsConfig\0".as_ptr() as *const ::core::ffi::c_char),
    CCharPtr(b"sapmComponent\0".as_ptr() as *const ::core::ffi::c_char),
    CCharPtr(b"sapmConfig\0".as_ptr() as *const ::core::ffi::c_char),
];

// === C2R_FILE_STATICS_END ===

pub extern "C" fn AudioFillConfigData(device: *const crate::types::HdfDeviceObject, configData: *mut crate::types::AudioConfigData) -> i32 {
    if device.is_null() || configData.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: Input para check error\0".as_ptr() as *const i8,
                b"AudioFillConfigData\0".as_ptr() as *const i8,
                92i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    let dev = unsafe { &*device };
    let node = dev.property;
    if node.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: drs node is NULL.\0".as_ptr() as *const i8,
                b"AudioFillConfigData\0".as_ptr() as *const i8,
                98i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    let drsOps = unsafe { DeviceResourceGetIfaceInstance(crate::types::HDF_CONFIG_SOURCE) };
    if drsOps.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: AudioFillConfigData: invalid drs ops fail!\0".as_ptr() as *const i8,
                b"AudioFillConfigData\0".as_ptr() as *const i8,
                103i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    let iface = unsafe { &*drsOps };
    let get_string = iface.GetString;
    if get_string.is_none() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: AudioFillConfigData: invalid drs ops fail!\0".as_ptr() as *const i8,
                b"AudioFillConfigData\0".as_ptr() as *const i8,
                103i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    let get_string = get_string.unwrap();

    let config = unsafe { &mut *configData };
    unsafe {
        let _ = get_string(
            node,
            b"serviceName\0".as_ptr() as *const i8,
            &mut config.cardServiceName,
            std::ptr::null(),
        );
        let _ = get_string(
            node,
            b"codecName\0".as_ptr() as *const i8,
            &mut config.codecName,
            std::ptr::null(),
        );
        let _ = get_string(
            node,
            b"platformName\0".as_ptr() as *const i8,
            &mut config.platformName,
            std::ptr::null(),
        );
        let _ = get_string(
            node,
            b"cpuDaiName\0".as_ptr() as *const i8,
            &mut config.cpuDaiName,
            std::ptr::null(),
        );
        let _ = get_string(
            node,
            b"codecDaiName\0".as_ptr() as *const i8,
            &mut config.codecDaiName,
            std::ptr::null(),
        );
        let _ = get_string(
            node,
            b"dspName\0".as_ptr() as *const i8,
            &mut config.dspName,
            std::ptr::null(),
        );
        let _ = get_string(
            node,
            b"dspDaiName\0".as_ptr() as *const i8,
            &mut config.dspDaiName,
            std::ptr::null(),
        );
    }

    unsafe {
        HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD002510u32,
            b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
            b"[%s][line:%d]: cardServiceName = %s\0".as_ptr() as *const i8,
            b"AudioFillConfigData\0".as_ptr() as *const i8,
            115i32,
            config.cardServiceName,
        );
    }
    unsafe {
        HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD002510u32,
            b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
            b"[%s][line:%d]: codecName       = %s, codecDaiName = %s\0".as_ptr() as *const i8,
            b"AudioFillConfigData\0".as_ptr() as *const i8,
            116i32,
            config.codecName,
            config.codecDaiName,
        );
    }
    unsafe {
        HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD002510u32,
            b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
            b"[%s][line:%d]: platformName    = %s, cpuDaiNamei = %s\0".as_ptr() as *const i8,
            b"AudioFillConfigData\0".as_ptr() as *const i8,
            117i32,
            config.platformName,
            config.cpuDaiName,
        );
    }
    unsafe {
        HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD002510u32,
            b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
            b"[%s][line:%d]: dspName         = %s, dspDaiName = %s\0".as_ptr() as *const i8,
            b"AudioFillConfigData\0".as_ptr() as *const i8,
            118i32,
            config.dspName,
            config.dspDaiName,
        );
    }

    crate::types::HDF_SUCCESS
}

fn GetAudioRegGroupNameIndex(name: *const std::ffi::c_char)-> u32 {
    if name.is_null() {
        return AUDIO_GROUP_MAX;
    }
    for index in 0..AUDIO_GROUP_MAX {
        let entry = g_audioRegGroupName[index as usize].0;
        if !entry.is_null() {
            if unsafe { libc::strcmp(name, entry) } == 0 {
                return index;
            }
        }
    }
    AUDIO_GROUP_MAX
}

fn GetRegArray(parser: *const crate::types::DeviceResourceIface, regNode: *const crate::types::DeviceResourceNode, group: *mut crate::types::AudioRegCfgGroupNode, indexMax: u32)-> *mut u32 {
    if group.is_null() || parser.is_null() || regNode.is_null() || indexMax == 0 {
        let tag = std::ffi::CString::new("HDF_AUDIO_KADM").unwrap();
        let fmt = std::ffi::CString::new("[%s][line:%d]: Input para check error").unwrap();
        let func_name = std::ffi::CString::new("GetRegArray").unwrap();
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, tag.as_ptr(), fmt.as_ptr(), func_name.as_ptr(), 148i32) };
        return std::ptr::null_mut();
    }

    let index: u32 = unsafe { (*group).groupIndex as u32 };
    if index >= AUDIO_GROUP_MAX {
        let tag = std::ffi::CString::new("HDF_AUDIO_KADM").unwrap();
        let fmt = std::ffi::CString::new("[%s][line:%d]: Input indexMax=%d error").unwrap();
        let func_name = std::ffi::CString::new("GetRegArray").unwrap();
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, tag.as_ptr(), fmt.as_ptr(), func_name.as_ptr(), 154i32, index as i32) };
        return std::ptr::null_mut();
    }

    let name: *const std::os::raw::c_char = g_audioRegGroupName[index as usize].0;
    let get_elem_num = unsafe { (*parser).GetElemNum.unwrap() };
    let num: i32 = unsafe { get_elem_num(regNode, name) };

    if num <= 0 || num > 500 {
        let tag = std::ffi::CString::new("HDF_AUDIO_KADM").unwrap();
        let fmt = std::ffi::CString::new("[%s][line:%d]: parser %s element num failed").unwrap();
        let func_name = std::ffi::CString::new("GetRegArray").unwrap();
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, tag.as_ptr(), fmt.as_ptr(), func_name.as_ptr(), 160i32, name) };
        return std::ptr::null_mut();
    }

    let item_count: u32 = (num as u32) / indexMax;
    unsafe { (*group).itemNum = item_count as u8; }

    let size: usize = std::mem::size_of::<u32>() * (num as usize);
    let buf: *mut u32 = unsafe { OsalMemCalloc(size.try_into().unwrap()) as *mut u32 };
    if buf.is_null() {
        let tag = std::ffi::CString::new("HDF_AUDIO_KADM").unwrap();
        let fmt = std::ffi::CString::new("[%s][line:%d]: malloc reg array buf failed!").unwrap();
        let func_name = std::ffi::CString::new("GetRegArray").unwrap();
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, tag.as_ptr(), fmt.as_ptr(), func_name.as_ptr(), 168i32) };
        return std::ptr::null_mut();
    }

    let get_uint32_array = unsafe { (*parser).GetUint32Array.unwrap() };
    let ret: i32 = unsafe { get_uint32_array(regNode, name, buf, num as u32, 0u32) };
    if ret != HDF_SUCCESS {
        let tag = std::ffi::CString::new("HDF_AUDIO_KADM").unwrap();
        let fmt = std::ffi::CString::new("[%s][line:%d]: parser %s reg array failed").unwrap();
        let func_name = std::ffi::CString::new("GetRegArray").unwrap();
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, tag.as_ptr(), fmt.as_ptr(), func_name.as_ptr(), 174i32, name) };
        unsafe { OsalMemFree(buf as *mut std::ffi::c_void) };
        return std::ptr::null_mut();
    }

    buf
}

fn ParseAudioRegItem(parser: *const crate::types::DeviceResourceIface, regNode: *const crate::types::DeviceResourceNode, group: *mut crate::types::AudioRegCfgGroupNode)-> i32 {
    if group.is_null() || parser.is_null() || regNode.is_null() {
        return HDF_FAILURE;
    }
    let group_ref = unsafe { &mut *group };
    let buf = GetRegArray(parser, regNode, group, AUDIO_REG_CFG_INDEX_MAX as u32);
    if buf.is_null() {
        return HDF_FAILURE;
    }
    let item_size = core::mem::size_of::<crate::types::AudioMixerControl>();
    let count = group_ref.itemNum as usize;
    let total_size = count * item_size;
    let ptr = unsafe { OsalMemCalloc(total_size.try_into().unwrap()) as *mut crate::types::AudioMixerControl };
    group_ref.regCfgItem = ptr;
    if ptr.is_null() {
        unsafe { OsalMemFree(buf as *mut core::ffi::c_void); }
        return HDF_ERR_MALLOC_FAIL;
    }
    for index in 0..count {
        let step = AUDIO_REG_CFG_INDEX_MAX as usize * index;
        let item = unsafe { &mut *ptr.add(index) };
        let buf_slice = unsafe { core::slice::from_raw_parts(buf, step + AUDIO_REG_CFG_INDEX_MAX as usize) };
        item.reg = buf_slice[step + AUDIO_REG_CFG_REG_INDEX as usize];
        item.rreg = buf_slice[step + AUDIO_REG_CFG_RREG_INDEX as usize];
        item.shift = buf_slice[step + AUDIO_REG_CFG_SHIFT_INDEX as usize];
        item.rshift = buf_slice[step + AUDIO_REG_CFG_RSHIFT_INDEX as usize];
        item.min = buf_slice[step + AUDIO_REG_CFG_MIN_INDEX as usize];
        item.max = buf_slice[step + AUDIO_REG_CFG_MAX_INDEX as usize];
        item.mask = buf_slice[step + AUDIO_REG_CFG_MASK_INDEX as usize];
        item.invert = buf_slice[step + AUDIO_REG_CFG_INVERT_INDEX as usize];
        item.value = buf_slice[step + AUDIO_REG_CFG_VALUE_INDEX as usize];
    }
    unsafe { OsalMemFree(buf as *mut core::ffi::c_void); }
    HDF_SUCCESS
}

fn ParseAudioEnumRegItem(parser: *const crate::types::DeviceResourceIface, regNode: *const crate::types::DeviceResourceNode, group: *mut crate::types::AudioRegCfgGroupNode)-> i32 {
    if group.is_null() || parser.is_null() || regNode.is_null() {
        return HDF_FAILURE;
    }

    let group_ref = unsafe { &mut *group };
    let buf = crate::src_audio_parse::GetRegArray(parser, regNode, group, AUDIO_ENUM_REG_CFG_INDEX_MAX as u32);
    if buf.is_null() {
        return HDF_FAILURE;
    }

    let item_count = group_ref.itemNum as usize;
    let new_item_ptr = unsafe {
        libc::calloc(item_count, std::mem::size_of::<crate::types::AudioEnumCtrlConfig>()) as *mut crate::types::AudioEnumCtrlConfig
    };
    if new_item_ptr.is_null() {
        unsafe { OsalMemFree(buf as *mut core::ffi::c_void); }
        return HDF_ERR_MALLOC_FAIL;
    }
    group_ref.regEnumCfgItem = new_item_ptr;

    for index in 0..item_count {
        let step = AUDIO_ENUM_REG_CFG_INDEX_MAX as usize * index;
        let item = unsafe { &mut *new_item_ptr.add(index) };
        let buf_slice = unsafe { core::slice::from_raw_parts(buf, step + AUDIO_ENUM_REG_CFG_INDEX_MAX as usize) };
        item.reg = buf_slice[step + AUDIO_ENUM_REG_CFG_REG_INDEX as usize];
        item.reg2 = buf_slice[step + AUDIO_ENUM_REG_CFG_RREG_INDEX as usize];
        item.shiftLeft = buf_slice[step + AUDIO_ENUM_REG_CFG_SHIFT_INDEX as usize] as u8;
        item.shiftRight = buf_slice[step + AUDIO_ENUM_REG_CFG_RSHIFT_INDEX as usize] as u8;
        item.max = buf_slice[step + AUDIO_ENUM_REG_CFG_MAX_INDEX as usize];
        item.mask = buf_slice[step + AUDIO_ENUM_REG_CFG_MASK_INDEX as usize];
        item.texts = buf_slice[step + AUDIO_ENUM_REG_CFG_TEXTS_INDEX as usize];
        item.values = buf_slice[step + AUDIO_ENUM_REG_CFG_VALUE_INDEX as usize];
        item.sapm = buf_slice[step + AUDIO_ENUM_REG_CFG_SAPM_INDEX as usize];
    }

    unsafe { OsalMemFree(buf as *mut core::ffi::c_void); }

    HDF_SUCCESS
}

fn ParseAudioSapmItem(parser: *const crate::types::DeviceResourceIface, regNode: *const crate::types::DeviceResourceNode, group: *mut crate::types::AudioRegCfgGroupNode)-> i32 {
    if parser.is_null() || regNode.is_null() || group.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const core::ffi::c_char,
                b"[%s][line:%d]: Input para check error\0".as_ptr() as *const core::ffi::c_char,
                b"ParseAudioSapmItem\0".as_ptr() as *const core::ffi::c_char,
                275,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    let group_ref = unsafe { &mut *group };
    let buf = crate::src_audio_parse::GetRegArray(
        parser,
        regNode,
        group,
        crate::types::AUDIO_SAPM_COMP_INDEX_MAX.try_into().unwrap(),
    );
    if buf.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const core::ffi::c_char,
                b"[%s][line:%d]: malloc reg array buf failed!\0".as_ptr()
                    as *const core::ffi::c_char,
                b"ParseAudioSapmItem\0".as_ptr() as *const core::ffi::c_char,
                281,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    let item_num = group_ref.itemNum;
    let alloc_size = (item_num as usize) * core::mem::size_of::<crate::types::AudioSapmCtrlConfig>();
    let sapm_ptr = unsafe {
        crate::compat::OsalMemCalloc(alloc_size.try_into().unwrap()) as *mut crate::types::AudioSapmCtrlConfig
    };
    if sapm_ptr.is_null() {
        unsafe {
            crate::compat::OsalMemFree(buf as *mut core::ffi::c_void);
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const core::ffi::c_char,
                b"[%s][line:%d]: malloc audio reg config item failed!\0".as_ptr()
                    as *const core::ffi::c_char,
                b"ParseAudioSapmItem\0".as_ptr() as *const core::ffi::c_char,
                289,
            );
        }
        return crate::types::HDF_ERR_MALLOC_FAIL;
    }
    group_ref.sapmCompItem = sapm_ptr;

    for index in 0..item_num {
        let step = (crate::types::AUDIO_SAPM_COMP_INDEX_MAX as usize) * (index as usize);
        let sapm_item = unsafe { &mut *sapm_ptr.offset(index as isize) };
        let buf_slice = unsafe { core::slice::from_raw_parts(buf, step + crate::types::AUDIO_SAPM_COMP_INDEX_MAX as usize) };
        let base = step;
        let stype = buf_slice[base + crate::types::AUDIO_SAPM_COMP_INDEX_TYPE as usize] as u8;
        let comp_name_idx = buf_slice[base + crate::types::AUDIO_SAPM_COMP_INDEX_NAME as usize] as u16;
        let reg = buf_slice[base + crate::types::AUDIO_SAPM_COMP_INDEX_REG as usize];
        let mask = buf_slice[base + crate::types::AUDIO_SAPM_COMP_INDEX_MASK as usize];
        let shift = buf_slice[base + crate::types::AUDIO_SAPM_COMP_INDEX_SHIFT as usize] as u8;
        let invert = buf_slice[base + crate::types::AUDIO_SAPM_COMP_INDEX_INVERT as usize] as u8;
        let kctl_news = buf_slice[base + crate::types::AUDIO_SAPM_COMP_INDEX_KCTL as usize];
        let kctl_num = buf_slice[base + crate::types::AUDIO_SAPM_COMP_INDEX_KCTLNUM as usize];
        sapm_item.sapmType = stype;
        sapm_item.compNameIndex = comp_name_idx;
        sapm_item.reg = reg;
        sapm_item.mask = mask;
        sapm_item.shift = shift;
        sapm_item.invert = invert;
        sapm_item.kcontrolNews = kctl_news;
        sapm_item.kcontrolsNum = kctl_num;
    }

    unsafe {
        crate::compat::OsalMemFree(buf as *mut core::ffi::c_void);
    }
    crate::types::HDF_SUCCESS
}

fn ParseAudioCtrlItem(parser: *const crate::types::DeviceResourceIface, regNode: *const crate::types::DeviceResourceNode, group: *mut crate::types::AudioRegCfgGroupNode)-> i32 {
    if parser.is_null() || regNode.is_null() || group.is_null() {
        return crate::types::HDF_FAILURE;
    }

    let group_ref = unsafe { &mut *group };
    let buf = crate::src_audio_parse::GetRegArray(
        parser,
        regNode,
        group,
        crate::types::AUDIO_CTRL_CFG_INDEX_MAX as u32,
    );
    if buf.is_null() {
        return crate::types::HDF_FAILURE;
    }

    let item_num = group_ref.itemNum as usize;
    let ctrl_ptr = unsafe {
        OsalMemCalloc(
            (item_num * std::mem::size_of::<crate::types::AudioControlConfig>()).try_into().unwrap(),
        ) as *mut crate::types::AudioControlConfig
    };
    if ctrl_ptr.is_null() {
        unsafe { OsalMemFree(buf as *mut ::core::ffi::c_void); }
        return crate::types::HDF_ERR_MALLOC_FAIL;
    }

    group_ref.ctrlCfgItem = ctrl_ptr;

    for index in 0..item_num {
        let step = (crate::types::AUDIO_CTRL_CFG_INDEX_MAX as usize) * index;
        let item = unsafe { &mut *ctrl_ptr.add(index) };
        let buf_slice = unsafe { core::slice::from_raw_parts(buf, step + crate::types::AUDIO_CTRL_CFG_INDEX_MAX as usize) };
        let array_index = buf_slice[step + crate::types::AUDIO_CTRL_CFG_INDEX_INDEX as usize];
        let iface_id = buf_slice[step + crate::types::AUDIO_CTRL_CFG_IFACE_INDEX as usize];
        let type_id = buf_slice[step + crate::types::AUDIO_CTRL_CFG_TYPE_INDEX as usize];
        let enable_val = buf_slice[step + crate::types::AUDIO_CTRL_CFG_ENABLE_INDEX as usize];
        item.arrayIndex = array_index as u16;
        item.iface = iface_id as u16;
        item.type_ = type_id as u16;
        item.enable = enable_val as u8;
    }

    unsafe { OsalMemFree(buf as *mut ::core::ffi::c_void); }

    crate::types::HDF_SUCCESS
}

fn ParseAudioAddrItem(parser: *const crate::types::DeviceResourceIface, regNode: *const crate::types::DeviceResourceNode, group: *mut crate::types::AudioRegCfgGroupNode)-> i32 {
    if parser.is_null() || regNode.is_null() || group.is_null() {
        return crate::types::HDF_FAILURE;
    }

    let group_ref = unsafe { &mut *group };
    let buf: *mut u32 = crate::src_audio_parse::GetRegArray(
        parser,
        regNode,
        group,
        crate::types::AUDIO_ADDR_CFG_INDEX_MAX as u32,
    );
    if buf.is_null() {
        return crate::types::HDF_FAILURE;
    }

    let item_num: u8 = group_ref.itemNum;
    let addr_cfg_item = unsafe {
        OsalMemCalloc(
            ((item_num as usize) * ::core::mem::size_of::<crate::types::AudioAddrConfig>()).try_into().unwrap(),
        ) as *mut crate::types::AudioAddrConfig
    };
    if addr_cfg_item.is_null() {
        unsafe { OsalMemFree(buf as *mut ::core::ffi::c_void); }
        return crate::types::HDF_ERR_MALLOC_FAIL;
    }

    group_ref.addrCfgItem = addr_cfg_item;

    for index in 0..(item_num as i32) {
        let step: i32 = crate::types::AUDIO_ADDR_CFG_INDEX_MAX * index;
        let item_ptr = unsafe { &mut *addr_cfg_item.offset(index as isize) };
        let buf_slice = unsafe { core::slice::from_raw_parts(buf, (step + crate::types::AUDIO_ADDR_CFG_INDEX_MAX) as usize) };
        let addr_val = buf_slice[(step + crate::types::AUDIO_ADDR_CFG_REG_INDEX) as usize];
        let value_val = buf_slice[(step + crate::types::AUDIO_ADDR_CFG_VALUE_INDEX) as usize];
        item_ptr.addr = addr_val;
        item_ptr.value = value_val;
    }

    unsafe { OsalMemFree(buf as *mut ::core::ffi::c_void); }

    crate::types::HDF_SUCCESS
}

fn ParseAudioRegGroup(parser: *const crate::types::DeviceResourceIface, regCfgNode: *const crate::types::DeviceResourceNode, groupNode: *mut *mut crate::types::AudioRegCfgGroupNode, index: u32)-> i32 {
    if parser.is_null() || regCfgNode.is_null() || groupNode.is_null() {
        // Logging omitted: HiLogPrint not available
        return crate::types::HDF_FAILURE;
    }

    let group = unsafe {
        OsalMemCalloc(core::mem::size_of::<crate::types::AudioRegCfgGroupNode>().try_into().unwrap())
            as *mut crate::types::AudioRegCfgGroupNode
    };
    if group.is_null() {
        // Logging omitted: HiLogPrint not available
        return crate::types::HDF_ERR_MALLOC_FAIL;
    }

    unsafe {
        *groupNode = group;
        (*group).groupIndex = index;
    }

    let ret = match index {
        crate::types::AUDIO_CTRL_CFG_GROUP | crate::types::AUDIO_SAPM_CFG_GROUP => {
            crate::src_audio_parse::ParseAudioCtrlItem(parser, regCfgNode, group)
        }
        crate::types::AUDIO_RSET_GROUP | crate::types::AUDIO_INIT_GROUP => {
            crate::src_audio_parse::ParseAudioAddrItem(parser, regCfgNode, group)
        }
        crate::types::AUDIO_DAI_PATAM_GROUP
        | crate::types::AUDIO_DAI_TRIGGER_GROUP
        | crate::types::AUDIO_CTRL_PATAM_GROUP
        | crate::types::AUDIO_CTRL_SAPM_PATAM_GROUP
        | crate::types::AUDIO_DAI_STARTUP_PATAM_GROUP => {
            crate::src_audio_parse::ParseAudioRegItem(parser, regCfgNode, group)
        }
        crate::types::AUDIO_CTRL_PATAM_MUX_GROUP | crate::types::AUDIO_CTRL_SAPM_PATAM_MUX_GROUP => {
            crate::src_audio_parse::ParseAudioEnumRegItem(parser, regCfgNode, group)
        }
        crate::types::AUDIO_SAPM_COMP_GROUP => {
            crate::src_audio_parse::ParseAudioSapmItem(parser, regCfgNode, group)
        }
        _ => {
            // Logging omitted: HiLogPrint not available
            return crate::types::HDF_FAILURE;
        }
    };

    if ret != crate::types::HDF_SUCCESS {
        // Logging omitted: HiLogPrint not available
        return crate::types::HDF_FAILURE;
    }

    crate::types::HDF_SUCCESS
}

fn ReleaseAudioAllRegConfig(config: *mut crate::types::AudioRegCfgData) {
    if config.is_null() {
        return;
    }
    let max = crate::types::AUDIO_GROUP_MAX as usize;
    for index in 0..max {
        unsafe {
            let node_ptr = (*config).audioRegParams[index];
            if !node_ptr.is_null() {
                if !(*node_ptr).regCfgItem.is_null() {
                    OsalMemFree((*node_ptr).regCfgItem as *mut ::core::ffi::c_void);
                    (*node_ptr).regCfgItem = std::ptr::null_mut();
                }
                OsalMemFree(node_ptr as *mut ::core::ffi::c_void);
                (*config).audioRegParams[index] = std::ptr::null_mut();
            }
        }
    }
}

fn ParseAudioAttr(parser: *const crate::types::DeviceResourceIface, attrNode: *const crate::types::DeviceResourceNode, config: *mut crate::types::AudioIdInfo)-> i32 {
    unsafe {
        let parser_ref = &*parser;
        let config_ref = &mut *config;

        // GetString
        let ret = parser_ref.GetString.unwrap()(
            attrNode,
            b"chipName\0".as_ptr() as *const ::core::ffi::c_char,
            &mut config_ref.chipName as *mut *const ::core::ffi::c_char,
            ::core::ptr::null(),
        );
        if ret != crate::types::HDF_SUCCESS {
            // HiLogPrint unavailable; original logs then returns HDF_SUCCESS
            return crate::types::HDF_SUCCESS;
        }

        // GetUint32 chipIdRegister
        let ret = parser_ref.GetUint32.unwrap()(
            attrNode,
            b"chipIdRegister\0".as_ptr() as *const ::core::ffi::c_char,
            &mut config_ref.chipIdRegister as *mut u32,
            0,
        );
        if ret != crate::types::HDF_SUCCESS {
            return crate::types::HDF_SUCCESS;
        }

        // GetUint32 chipIdSize
        let ret = parser_ref.GetUint32.unwrap()(
            attrNode,
            b"chipIdSize\0".as_ptr() as *const ::core::ffi::c_char,
            &mut config_ref.chipIdSize as *mut u32,
            0,
        );
        if ret != crate::types::HDF_SUCCESS {
            return crate::types::HDF_SUCCESS;
        }

        ret
    }
}

fn AudioSetPortInfoConfig(buf: *const u64, info: *mut crate::types::AudioPcmStream)-> i32 {
    let dest_size = core::mem::size_of::<crate::types::AudioPcmStream>();
    let count = 12 * core::mem::size_of::<u64>();
    let ret: i32;
    if count <= dest_size && !info.is_null() && !buf.is_null() {
        unsafe {
            std::ptr::copy_nonoverlapping(buf as *const u8, info as *mut u8, count);
        }
        ret = 0;
    } else {
        ret = -1;
    }
    if ret != 0 {
        // HiLogPrint is an unresolved external symbol; omitted.
        return crate::types::HDF_FAILURE;
    }
    crate::types::HDF_SUCCESS
}

fn AudioSetPortInfoConfigStub(buf: *const u64, configData: *mut crate::types::AudioPortInfo)-> i32 {
    unsafe {
        let direction = *buf;
        if direction == crate::types::PORT_OUT as u64 {
            crate::src_audio_parse::AudioSetPortInfoConfig(buf, std::ptr::addr_of_mut!((*configData).render))
        } else if direction == crate::types::PORT_IN as u64 {
            crate::src_audio_parse::AudioSetPortInfoConfig(buf, std::ptr::addr_of_mut!((*configData).capture))
        } else {
            crate::types::HDF_FAILURE
        }
    }
}

fn AudioGetPortInfoConfig(drsOps: *mut crate::types::DeviceResourceIface, device: *const crate::types::HdfDeviceObject, configData: *mut crate::types::AudioPortInfo)-> i32 {
    let property = unsafe { (*device).property };
    let hw_info = b"hwInfo\0" as *const u8 as *const std::ffi::c_char;

    let get_elem_num = unsafe { (*drsOps).GetElemNum.unwrap() };
    let num: i32 = unsafe { get_elem_num(property, hw_info) };
    if num <= 0 || num > crate::types::AUDIO_CONFIG_MAX_ITEM as i32 {
        return crate::types::HDF_FAILURE;
    }

    let mut buf: [u64; crate::types::AUDIO_CONFIG_MAX_ITEM as usize] = [0u64; crate::types::AUDIO_CONFIG_MAX_ITEM as usize];
    let buf_ptr = buf.as_mut_ptr();

    let get_uint64_array = unsafe { (*drsOps).GetUint64Array.unwrap() };
    let ret = unsafe { get_uint64_array(property, hw_info, buf_ptr as *mut u64, num as u32, 0) };
    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_FAILURE;
    }

    unsafe {
        std::ptr::write_bytes(configData as *mut u8, 0, std::mem::size_of::<crate::types::AudioPortInfo>());
    }

    let buf_ptr_const = buf.as_ptr();
    match num {
        12 => unsafe {
            crate::src_audio_parse::AudioSetPortInfoConfigStub(buf_ptr_const, configData);
        },
        24 => unsafe {
            crate::src_audio_parse::AudioSetPortInfoConfigStub(buf_ptr_const, configData);
            crate::src_audio_parse::AudioSetPortInfoConfigStub(buf_ptr_const.add(12), configData);
        },
        _ => return crate::types::HDF_FAILURE,
    }

    crate::types::HDF_SUCCESS
}

#[no_mangle]
pub extern "C" fn AudioGetPortConfig(device: *const crate::types::HdfDeviceObject, configData: *mut crate::types::AudioPortInfo) -> i32 {
    if device.is_null() || unsafe { (*device).property.is_null() } || configData.is_null() {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: Input para check error\0".as_ptr() as *const ::core::ffi::c_char,
                b"AudioGetPortConfig\0".as_ptr() as *const ::core::ffi::c_char,
                557i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    let drsOps = unsafe { DeviceResourceGetIfaceInstance(crate::types::HDF_CONFIG_SOURCE) };
    if drsOps.is_null() || unsafe { (*drsOps).GetString.is_none() } {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: AudioGetPortConfig: invalid drs ops fail!\0".as_ptr() as *const ::core::ffi::c_char,
                b"AudioGetPortConfig\0".as_ptr() as *const ::core::ffi::c_char,
                563i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    let ret = crate::src_audio_parse::AudioGetPortInfoConfig(drsOps, device, configData);
    if ret != crate::types::HDF_SUCCESS {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: parser chipIdRegister reg audioIdInfo failed!\0".as_ptr() as *const ::core::ffi::c_char,
                b"AudioGetPortConfig\0".as_ptr() as *const ::core::ffi::c_char,
                569i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    return crate::types::HDF_SUCCESS;
}

pub extern "C" fn AudioGetRegConfig(device: *const crate::types::HdfDeviceObject, configData: *mut crate::types::AudioRegCfgData) -> i32 {
    if device.is_null() || unsafe { (*device).property.is_null() } || configData.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510_u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: Input para check error\0".as_ptr() as *const i8,
                b"AudioGetRegConfig\0".as_ptr() as *const i8,
                584i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    let drsOps = unsafe { DeviceResourceGetIfaceInstance(crate::types::HDF_CONFIG_SOURCE) } as *mut crate::types::DeviceResourceIface;
    if drsOps.is_null() || unsafe { (*drsOps).GetString.is_none() } {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510_u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: AudioFillConfigData: invalid drs ops fail!\0".as_ptr() as *const i8,
                b"AudioGetRegConfig\0".as_ptr() as *const i8,
                590i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    let idNode = if let Some(func) = unsafe { (*drsOps).GetChildNode } {
        unsafe { func((*device).property, b"idInfo\0".as_ptr() as *const i8) }
    } else {
        std::ptr::null()
    };

    if !idNode.is_null() {
        let ret = crate::src_audio_parse::ParseAudioAttr(
            drsOps as *const crate::types::DeviceResourceIface,
            idNode,
            unsafe { &mut (*configData).audioIdInfo as *mut _ },
        );
        if ret != crate::types::HDF_SUCCESS {
            unsafe {
                HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510_u32,
                    b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                    b"[%s][line:%d]: audio reg node attr is null\0".as_ptr() as *const i8,
                    b"AudioGetRegConfig\0".as_ptr() as *const i8,
                    597i32,
                );
            }
            return crate::types::HDF_FAILURE;
        }
    }

    let regCfgNode = if let Some(func) = unsafe { (*drsOps).GetChildNode } {
        unsafe { func((*device).property, b"regConfig\0".as_ptr() as *const i8) }
    } else {
        std::ptr::null()
    };
    if regCfgNode.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510_u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: AudioGetRegConfig: Read audioRegConfig fail!\0".as_ptr() as *const i8,
                b"AudioGetRegConfig\0".as_ptr() as *const i8,
                604i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    let mut regAttr = unsafe { (*regCfgNode).attrData };
    while !regAttr.is_null() {
        if regAttr.is_null() || unsafe { (*regAttr).name.is_null() } {
            unsafe {
                HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510_u32,
                    b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                    b"[%s][line:%d]: audio reg node attr is null\0".as_ptr() as *const i8,
                    b"AudioGetRegConfig\0".as_ptr() as *const i8,
                    610i32,
                );
            }
            return crate::types::HDF_FAILURE;
        }

        let index = crate::src_audio_parse::GetAudioRegGroupNameIndex(unsafe { (*regAttr).name });
        if index >= crate::types::AUDIO_GROUP_MAX {
            regAttr = unsafe { (*regAttr).next };
            continue;
        }

        let group_node_ptr = unsafe {
            &mut (*configData).audioRegParams[index as usize] as *mut *mut crate::types::AudioRegCfgGroupNode
        };
        let ret = crate::src_audio_parse::ParseAudioRegGroup(
            drsOps as *const crate::types::DeviceResourceIface,
            regCfgNode,
            group_node_ptr,
            index,
        );
        if ret != crate::types::HDF_SUCCESS {
            unsafe {
                HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510_u32,
                    b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                    b"[%s][line:%d]: parse audio register group failed\0".as_ptr() as *const i8,
                    b"AudioGetRegConfig\0".as_ptr() as *const i8,
                    620i32,
                );
            }
            crate::src_audio_parse::ReleaseAudioAllRegConfig(configData);
            return crate::types::HDF_FAILURE;
        }

        regAttr = unsafe { (*regAttr).next };
    }

    crate::types::HDF_SUCCESS
}
