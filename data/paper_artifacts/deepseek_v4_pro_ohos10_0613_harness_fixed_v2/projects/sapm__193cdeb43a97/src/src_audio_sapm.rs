//! Module: src_audio_sapm
//!
//! Auto-generated skeleton - function bodies are unimplemented.

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// offset_of! is superseded by core::mem::offset_of! (Rust 1.77+)
// All call sites now use core::mem::offset_of! directly.

use crate::types::*;
use crate::globals::*;
use crate::compat::*;

// Safe wrapper macro for HiLogPrint to reduce unsafe blocks.
macro_rules! sapm_log {
    ($level:expr, $tag:expr, $fmt:expr, $func:expr, $line:expr $(,)?) => {
        unsafe {
            $crate::compat::HiLogPrint($crate::types::LOG_CORE, $level, 0xD002510u32, $tag, $fmt, $func, $line);
        }
    };
    ($level:expr, $tag:expr, $fmt:expr, $func:expr, $line:expr, $($extra:expr),+ $(,)?) => {
        unsafe {
            $crate::compat::HiLogPrint($crate::types::LOG_CORE, $level, 0xD002510u32, $tag, $fmt, $func, $line, $($extra),+);
        }
    };
}

// === C2R_FILE_STATICS_BEGIN ===
// File-scope `static` variables (internal linkage) from the original C TU.
// These are module-local by design (Scheme B).
/// C: static int32_t[26] g_audioSapmPowerUpSeq
type PowerSeq = [i32; 26];
fn g_audioSapmPowerUpSeq() -> &'static PowerSeq {
    static G: std::sync::OnceLock<PowerSeq> = std::sync::OnceLock::new();
    G.get_or_init(|| [
        0, // index  0 AUDIO_SAPM_INPUT: not specified, C default 0
        0, // index  1 AUDIO_SAPM_OUTPUT
        5, // index  2 AUDIO_SAPM_MUX -> 5
        0, // index  3 AUDIO_SAPM_DEMUX
        5, // index  4 AUDIO_SAPM_VIRT_MUX -> 5
        5, // index  5 AUDIO_SAPM_VALUE_MUX -> 5
        7, // index  6 AUDIO_SAPM_MIXER -> 7
        7, // index  7 AUDIO_SAPM_MIXER_NAMED_CTRL -> 7
        8, // index  8 AUDIO_SAPM_PGA -> 8
        10, // index  9 AUDIO_SAPM_OUT_DRV -> 10
        9, // index 10 AUDIO_SAPM_ADC -> 9
        6, // index 11 AUDIO_SAPM_DAC -> 6
        2, // index 12 AUDIO_SAPM_MICBIAS -> 2
        4, // index 13 AUDIO_SAPM_MIC -> 4
        10, // index 14 AUDIO_SAPM_HP -> 10
        10, // index 15 AUDIO_SAPM_SPK -> 10
        0, // index 16 AUDIO_SAPM_LINE
        0, // index 17 AUDIO_SAPM_ANALOG_SWITCH
        0, // index 18 AUDIO_SAPM_VMID
        0, // index 19 AUDIO_SAPM_PRE -> 0
        11, // index 20 AUDIO_SAPM_POST -> 11
        1, // index 21 AUDIO_SAPM_SUPPLY -> 1
        0, // index 22 AUDIO_SAPM_REGULATOR_SUPPLY
        0, // index 23 AUDIO_SAPM_CLOCK_SUPPLY
        3, // index 24 AUDIO_SAPM_AIF_IN -> 3
        3, // index 25 AUDIO_SAPM_AIF_OUT -> 3
    ])
}

/// C: static int32_t[26] g_audioSapmPowerDownSeq
fn g_audioSapmPowerDownSeq() -> &'static PowerSeq {
    static G: std::sync::OnceLock<PowerSeq> = std::sync::OnceLock::new();
    G.get_or_init(|| [
        0, // index  0 AUDIO_SAPM_INPUT
        0, // index  1 AUDIO_SAPM_OUTPUT
        9, // index  2 AUDIO_SAPM_MUX -> 9
        0, // index  3 AUDIO_SAPM_DEMUX
        9, // index  4 AUDIO_SAPM_VIRT_MUX -> 9
        9, // index  5 AUDIO_SAPM_VALUE_MUX -> 9
        5, // index  6 AUDIO_SAPM_MIXER -> 5
        5, // index  7 AUDIO_SAPM_MIXER_NAMED_CTRL -> 5
        4, // index  8 AUDIO_SAPM_PGA -> 4
        2, // index  9 AUDIO_SAPM_OUT_DRV -> 2
        1, // index 10 AUDIO_SAPM_ADC -> 1
        6, // index 11 AUDIO_SAPM_DAC -> 6
        8, // index 12 AUDIO_SAPM_MICBIAS -> 8
        7, // index 13 AUDIO_SAPM_MIC -> 7
        2, // index 14 AUDIO_SAPM_HP -> 2
        2, // index 15 AUDIO_SAPM_SPK -> 2
        0, // index 16 AUDIO_SAPM_LINE
        0, // index 17 AUDIO_SAPM_ANALOG_SWITCH
        0, // index 18 AUDIO_SAPM_VMID
        0, // index 19 AUDIO_SAPM_PRE -> 0
        12, // index 20 AUDIO_SAPM_POST -> 12
        11, // index 21 AUDIO_SAPM_SUPPLY -> 11
        0, // index 22 AUDIO_SAPM_REGULATOR_SUPPLY
        0, // index 23 AUDIO_SAPM_CLOCK_SUPPLY
        10, // index 24 AUDIO_SAPM_AIF_IN -> 10
        10, // index 25 AUDIO_SAPM_AIF_OUT -> 10
    ])
}

// === C2R_FILE_STATICS_END ===

// Helper to initialise a DListHead as an empty circular list.
#[inline]
unsafe fn dlist_head_init(head: *mut crate::types::DListHead) {
    (*head).next = head;
    (*head).prev = head;
}

fn ConnectedInputEndPoint(sapmComponent: *const crate::types::AudioSapmComponent)-> i32 {
    if sapmComponent.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let comp = unsafe { &*sapmComponent };

    let sapm_type = comp.sapmType;
    match sapm_type {
        crate::types::AUDIO_SAPM_DAC
        | crate::types::AUDIO_SAPM_AIF_IN
        | crate::types::AUDIO_SAPM_INPUT
        | crate::types::AUDIO_SAPM_MIC
        | crate::types::AUDIO_SAPM_LINE => {
            return 1;
        }
        _ => {}
    }

    let mut count: i32 = 0;

    // offset of listSink in AudioSapmpath
    let offset_of_list_sink = unsafe {
        std::ptr::addr_of!((*std::ptr::null::<crate::types::AudioSapmpath>()).listSink) as usize
    };

    let head = core::ptr::addr_of!(comp.sources) as *const crate::types::DListHead;
    let head_ref = unsafe { &*head };
    let mut pos: *const crate::types::DListHead = head_ref.next;

    while pos != head {
        let path = unsafe {
            (pos as *const u8).offset(-(offset_of_list_sink as isize))
                as *const crate::types::AudioSapmpath
        };
        if !path.is_null() {
            let path_ref = unsafe { &*path };
            let source = path_ref.source;
            let connect = path_ref.connect;
            if !source.is_null() && connect == 1 {
                count += crate::src_audio_sapm::ConnectedInputEndPoint(source);
            }
            pos = path_ref.listSink.next;
        } else {
            let pos_ref = unsafe { &*pos };
            pos = pos_ref.next;
        }
    }

    count
}

fn ConnectedOutputEndPoint(sapmComponent: *const crate::types::AudioSapmComponent)-> i32 {
    if sapmComponent.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let comp = unsafe { &*sapmComponent };

    // Check if this component itself is an output endpoint
    let sapm_type = comp.sapmType;
    match sapm_type {
        crate::types::AUDIO_SAPM_ADC
        | crate::types::AUDIO_SAPM_AIF_OUT
        | crate::types::AUDIO_SAPM_OUTPUT
        | crate::types::AUDIO_SAPM_HP
        | crate::types::AUDIO_SAPM_SPK
        | crate::types::AUDIO_SAPM_LINE => return 1,
        _ => {}
    }

    let mut count: i32 = 0;
    let head: *const crate::types::DListHead =
        std::ptr::addr_of!(comp.sinks);
    let head_ref = unsafe { &*head };
    let mut node: *const crate::types::DListHead = head_ref.next;

    // Compute byte offset of `listSource` inside `AudioSapmpath`
    let offset = unsafe {
        std::ptr::addr_of!((*std::ptr::null::<crate::types::AudioSapmpath>()).listSource) as usize
    };

    while node != head {
        // container_of: path = (char*)node - offset
        let path: *mut crate::types::AudioSapmpath =
            unsafe { (node as *const u8).sub(offset) as *mut crate::types::AudioSapmpath };
        if !path.is_null() {
            let path_ref = unsafe { &*path };
            let sink = path_ref.sink;
            let connect = path_ref.connect;
            if !sink.is_null() && connect == 1 {
                count += ConnectedOutputEndPoint(sink);
            }
            node = path_ref.listSource.next;
        } else {
            let node_ref = unsafe { &*node };
            node = node_ref.next;
        }
    }
    count
}

fn AudioSapmGenericCheckPower(sapmComponent: *const crate::types::AudioSapmComponent)-> i32 {
    if sapmComponent.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let input = crate::src_audio_sapm::ConnectedInputEndPoint(sapmComponent);
    if input == crate::types::HDF_FAILURE {
        return crate::types::HDF_FAILURE;
    }
    let output = crate::src_audio_sapm::ConnectedOutputEndPoint(sapmComponent);
    if output == crate::types::HDF_FAILURE {
        return crate::types::HDF_FAILURE;
    }
    if input == 0 || output == 0 {
        return 0;
    }
    return 1;
}

fn AudioSapmAdcPowerClock(sapmComponent: *mut crate::types::AudioSapmComponent)-> i32 {
    if sapmComponent.is_null() {
        eprintln!(
            "[{}][line:{}]: {} param sapmComponent is NULL.",
            "AudioSapmAdcPowerClock", 173, "HDF_AUDIO_SAPM"
        );
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let comp = unsafe { &mut *sapmComponent };
    let name = comp.componentName;
    let name_str = if name.is_null() {
        " (null)".to_owned()
    } else {
        let c_str = unsafe {
            std::ffi::CStr::from_ptr(name as *const std::os::raw::c_char)
        };
        c_str.to_str().unwrap_or(" (invalid utf8)").to_owned()
    };
    println!("{} standby mode entry!", name_str);
    crate::types::HDF_SUCCESS
}

fn AudioSapmDacPowerClock(sapmComponent: *mut crate::types::AudioSapmComponent)-> i32 {
    if sapmComponent.is_null() {
        let tag = std::ffi::CString::new("HDF_AUDIO_SAPM").unwrap();
        let fmt = std::ffi::CString::new("[%s][line:%d]: param sapmComponent is NULL.").unwrap();
        let func = std::ffi::CString::new("AudioSapmDacPowerClock").unwrap();
        let tag_ptr = tag.as_ptr();
        let fmt_ptr = fmt.as_ptr();
        let func_ptr = func.as_ptr();
        sapm_log!(crate::types::LOG_ERROR, tag_ptr, fmt_ptr, func_ptr, 184i32);
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let comp = unsafe { &mut *sapmComponent };
    let tag = std::ffi::CString::new("HDF_AUDIO_SAPM").unwrap();
    let fmt = std::ffi::CString::new("[%s][line:%d]: %s standby mode entry!").unwrap();
    let func = std::ffi::CString::new("AudioSapmDacPowerClock").unwrap();
    let tag_ptr = tag.as_ptr();
    let fmt_ptr = fmt.as_ptr();
    let func_ptr = func.as_ptr();
    let comp_name_ptr = comp.componentName as *const std::ffi::c_char;
    sapm_log!(crate::types::LOG_INFO, tag_ptr, fmt_ptr, func_ptr, 188i32, comp_name_ptr);
    return crate::types::HDF_SUCCESS;
}

fn AudioSapmAdcCheckPower(sapmComponent: *const crate::types::AudioSapmComponent)-> i32 {
    if sapmComponent.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let comp = unsafe { &*sapmComponent };
    let input = if comp.active == 0 {
        crate::src_audio_sapm::AudioSapmGenericCheckPower(sapmComponent)
    } else {
        crate::src_audio_sapm::ConnectedInputEndPoint(sapmComponent)
    };
    if input == crate::types::HDF_FAILURE {
        return crate::types::HDF_FAILURE;
    }
    input
}

fn AudioSapmDacCheckPower(sapmComponent: *const crate::types::AudioSapmComponent)-> i32 {
    if sapmComponent.is_null() {
        // HiLogPrint logging omitted due to unavailable extern
        return crate::types::HDF_FAILURE;
    }

    let comp = unsafe { &*sapmComponent };
    let output: i32;
    if comp.active == 0 {
        output = crate::src_audio_sapm::AudioSapmGenericCheckPower(sapmComponent);
    } else {
        output = crate::src_audio_sapm::ConnectedOutputEndPoint(sapmComponent);
    }
    if output == crate::types::HDF_FAILURE {
        // HiLogPrint logging omitted due to unavailable extern
        return crate::types::HDF_FAILURE;
    }
    output
}

// Thin ABI thunks for PowerCheck callbacks
unsafe extern "C" fn generic_check_power_thunk(ptr: *const crate::types::AudioSapmComponent) -> i32 {
    crate::src_audio_sapm::AudioSapmGenericCheckPower(ptr)
}
unsafe extern "C" fn adc_check_power_thunk(ptr: *const crate::types::AudioSapmComponent) -> i32 {
    crate::src_audio_sapm::AudioSapmAdcCheckPower(ptr)
}
unsafe extern "C" fn dac_check_power_thunk(ptr: *const crate::types::AudioSapmComponent) -> i32 {
    crate::src_audio_sapm::AudioSapmDacCheckPower(ptr)
}

// Thin ABI thunks for PowerClockOp callbacks
unsafe extern "C" fn adc_power_clock_thunk(ptr: *mut crate::types::AudioSapmComponent) -> i32 {
    crate::src_audio_sapm::AudioSapmAdcPowerClock(ptr)
}
unsafe extern "C" fn dac_power_clock_thunk(ptr: *mut crate::types::AudioSapmComponent) -> i32 {
    crate::src_audio_sapm::AudioSapmDacPowerClock(ptr)
}

// Thin ABI thunk for AudioSapmThread thread entry
unsafe extern "C" fn audio_sapm_thread_thunk(data: *mut std::ffi::c_void) -> i32 {
    crate::src_audio_sapm::AudioSapmThread(data)
}

fn AudioSampCheckPowerCallback(sapmComponent: *mut crate::types::AudioSapmComponent) {
    if sapmComponent.is_null() {
        // HiLogPrint omitted due to unresolved external symbol.
        return;
    }
    let comp = unsafe { &*sapmComponent };
    let sapm_type = comp.sapmType;
    let check_fn: unsafe extern "C" fn(*const crate::types::AudioSapmComponent) -> i32 = match sapm_type {
        crate::types::AUDIO_SAPM_ANALOG_SWITCH
        | crate::types::AUDIO_SAPM_MIXER
        | crate::types::AUDIO_SAPM_MIXER_NAMED_CTRL
        | crate::types::AUDIO_SAPM_MUX
        | crate::types::AUDIO_SAPM_VIRT_MUX
        | crate::types::AUDIO_SAPM_VALUE_MUX
        | crate::types::AUDIO_SAPM_PGA
        | crate::types::AUDIO_SAPM_OUT_DRV
        | crate::types::AUDIO_SAPM_INPUT
        | crate::types::AUDIO_SAPM_OUTPUT
        | crate::types::AUDIO_SAPM_MICBIAS
        | crate::types::AUDIO_SAPM_SPK
        | crate::types::AUDIO_SAPM_HP
        | crate::types::AUDIO_SAPM_MIC
        | crate::types::AUDIO_SAPM_LINE => {
            generic_check_power_thunk as unsafe extern "C" fn(*const crate::types::AudioSapmComponent) -> i32
        }
        crate::types::AUDIO_SAPM_ADC | crate::types::AUDIO_SAPM_AIF_OUT => {
            adc_check_power_thunk as unsafe extern "C" fn(*const crate::types::AudioSapmComponent) -> i32
        }
        crate::types::AUDIO_SAPM_DAC | crate::types::AUDIO_SAPM_AIF_IN => {
            dac_check_power_thunk as unsafe extern "C" fn(*const crate::types::AudioSapmComponent) -> i32
        }
        _ => {
            generic_check_power_thunk as unsafe extern "C" fn(*const crate::types::AudioSapmComponent) -> i32
        }
    };

    unsafe {
        (*sapmComponent).PowerCheck = Some(check_fn);
    }
}


fn AudioSampPowerClockCallback(sapmComponent: *mut crate::types::AudioSapmComponent) {
    if sapmComponent.is_null() {
        return;
    }
    let comp = unsafe { &*sapmComponent };
    let sapmType = comp.sapmType;
    let op: Option<unsafe extern "C" fn(*mut crate::types::AudioSapmComponent) -> i32> = match sapmType {
        crate::types::AUDIO_SAPM_ADC | crate::types::AUDIO_SAPM_AIF_OUT => {
            Some(adc_power_clock_thunk as unsafe extern "C" fn(*mut crate::types::AudioSapmComponent) -> i32)
        }
        crate::types::AUDIO_SAPM_DAC | crate::types::AUDIO_SAPM_AIF_IN => {
            Some(dac_power_clock_thunk as unsafe extern "C" fn(*mut crate::types::AudioSapmComponent) -> i32)
        }
        _ => None,
    };
    unsafe {
        (*sapmComponent).PowerClockOp = op;
    }
}

fn AudioSapmNewComponent(audioCard: *mut crate::types::AudioCard, component: *const crate::types::AudioSapmComponent) -> i32 {
    if audioCard.is_null() || component.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let card = unsafe { &mut *audioCard };
    let comp_in = unsafe { &*component };

    if card.rtd.is_null() {
        return crate::types::HDF_FAILURE;
    }
    if comp_in.componentName.is_null() {
        return crate::types::HDF_FAILURE;
    }

    let sapmComponent = unsafe {
        OsalMemCalloc(std::mem::size_of::<crate::types::AudioSapmComponent>() as size_t) as *mut crate::types::AudioSapmComponent
    };
    if sapmComponent.is_null() {
        return crate::types::HDF_FAILURE;
    }

    // Copy the component struct data
    unsafe {
        std::ptr::copy_nonoverlapping(
            component as *const u8,
            sapmComponent as *mut u8,
            std::mem::size_of::<crate::types::AudioSapmComponent>()
        );
    }

    // Allocate and copy component name
    let name_len = unsafe { libc::strlen(comp_in.componentName as *const i8) };
    let name_bytes = (name_len + 1) as usize;
    let new_name = unsafe { OsalMemCalloc(name_bytes as size_t) as *mut std::ffi::c_char };
    if new_name.is_null() {
        unsafe { OsalMemFree(sapmComponent as *mut std::ffi::c_void); }
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(
            comp_in.componentName as *const u8,
            new_name as *mut u8,
            name_bytes
        );
        (*sapmComponent).componentName = new_name;
    }

    unsafe {
        (*sapmComponent).codec = (*card.rtd).codec;
        (*sapmComponent).kcontrolsNum = comp_in.kcontrolsNum;
        (*sapmComponent).active = 0;
    }

    crate::src_audio_sapm::AudioSampCheckPowerCallback(sapmComponent);
    crate::src_audio_sapm::AudioSampPowerClockCallback(sapmComponent);

    // Initialize list heads in a consolidated unsafe block
    unsafe {
        dlist_head_init(std::ptr::addr_of_mut!((*sapmComponent).sources));
        dlist_head_init(std::ptr::addr_of_mut!((*sapmComponent).sinks));
        dlist_head_init(std::ptr::addr_of_mut!((*sapmComponent).list));
        dlist_head_init(std::ptr::addr_of_mut!((*sapmComponent).dirty));
    }

    // Insert into components list (unsafe linked list insertion)
    unsafe {
        let entry = std::ptr::addr_of_mut!((*sapmComponent).list);
        let head = std::ptr::addr_of_mut!(card.components);
        (*entry).next = (*head).next;
        (*entry).prev = head;
        (*(*head).next).prev = entry;
        (*head).next = entry;

        (*sapmComponent).connected = 1;
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioSapmNewComponents(audioCard: *mut crate::types::AudioCard, component: *const crate::types::AudioSapmComponent, cptMaxNum: i32) -> i32 {
    if audioCard.is_null() {
        return crate::types::HDF_FAILURE;
    }
    if component.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let components = unsafe { std::slice::from_raw_parts(component, cptMaxNum as usize) };
    for comp in components {
        let ret = crate::src_audio_sapm::AudioSapmNewComponent(audioCard, comp);
        if ret != crate::types::HDF_SUCCESS {
            return crate::types::HDF_FAILURE;
        }
    }
    crate::types::HDF_SUCCESS
}

fn MuxSetPathStatus(sapmComponent: *const crate::types::AudioSapmComponent, path: *mut crate::types::AudioSapmpath, enumKtl: *const crate::types::AudioEnumKcontrol, i: i32) {
    if sapmComponent.is_null() {
        return;
    }
    let comp = unsafe { &*sapmComponent };
    if comp.codec.is_null() {
        return;
    }
    if path.is_null() {
        return;
    }
    let path_mut = unsafe { &mut *path };
    if path_mut.name.is_null() {
        return;
    }
    if enumKtl.is_null() {
        return;
    }
    let ktl = unsafe { &*enumKtl };

    let shift = ktl.shiftLeft as u32;
    let mut val: u32 = 0;
    let ret = unsafe {
        AudioCodecReadReg(
            comp.codec as *const crate::types::CodecDevice,
            ktl.reg,
            &mut val,
        )
    };
    if ret != crate::types::HDF_SUCCESS {
        return;
    }

    let cur_value = (val >> shift) & ktl.mask;
    path_mut.connect = 0;

    if !ktl.texts.is_null() {
        let max = ktl.max;
        let texts = unsafe { std::slice::from_raw_parts(ktl.texts, max as usize) };
        for (idx, text_ptr) in texts.iter().enumerate() {
            let text_ptr = *text_ptr;
            if text_ptr.is_null() {
                continue;
            }
            let path_name = path_mut.name;
            if unsafe { libc::strcmp(path_name as *const i8, text_ptr) } == 0 && cur_value == idx as u32 {
                path_mut.connect = 1;
            }
        }
    } else {
        if cur_value != 0 {
            path_mut.connect = 1;
        }
    }
}

fn MuxValueSetPathStatus(
    sapmComponent: *const crate::types::AudioSapmComponent,
    path: *mut crate::types::AudioSapmpath,
    enumKtl: *const crate::types::AudioEnumKcontrol,
    _i: i32,
) {
    // Early null checks
    if sapmComponent.is_null() {
        return;
    }
    let comp = unsafe { &*sapmComponent };
    let codec = comp.codec;
    if codec.is_null() {
        return;
    }
    if path.is_null() {
        return;
    }
    let path_ref = unsafe { &*path };
    let path_name_ptr = path_ref.name;
    if path_name_ptr.is_null() {
        return;
    }
    if enumKtl.is_null() {
        return;
    }

    // Read all needed fields from enumKtl via a reference
    let ktl_ref = unsafe { &*enumKtl };
    let shift = ktl_ref.shiftLeft as u32;
    let reg = ktl_ref.reg;
    let mask = ktl_ref.mask;
    let values = ktl_ref.values;
    let texts = ktl_ref.texts;
    let max = ktl_ref.max;

    let mut val: u32 = 0;
    let ret = unsafe {
        crate::compat::AudioCodecReadReg(
            codec as *const crate::types::CodecDevice,
            reg,
            &mut val as *mut u32,
        )
    };
    if ret != crate::types::HDF_SUCCESS {
        return;
    }

    val = (val >> shift) & mask;
    let path_mut = unsafe { &mut *path };
    path_mut.connect = 0u8;

    if !values.is_null() && !texts.is_null() {
        let values_slice = unsafe { std::slice::from_raw_parts(values, max as usize) };
        let texts_slice = unsafe { std::slice::from_raw_parts(texts, max as usize) };
        let mut item: u32 = 0;
        for (i, v) in values_slice.iter().enumerate() {
            if val == *v {
                item = i as u32;
                break;
            }
        }

        for (i_local, text_ptr) in texts_slice.iter().enumerate() {
            let text_ptr = *text_ptr;
            if text_ptr.is_null() {
                continue;
            }
            let path_name_cstr = unsafe { std::ffi::CStr::from_ptr(path_name_ptr) };
            let text_cstr = unsafe { std::ffi::CStr::from_ptr(text_ptr) };
            if path_name_cstr == text_cstr && item == i_local as u32 {
                path_mut.connect = 1u8;
            }
        }
    } else {
        if val != 0 {
            path_mut.connect = 1u8;
        }
    }
}

fn MixerSetPathStatus(sapmComponent: *const crate::types::AudioSapmComponent, path: *mut crate::types::AudioSapmpath, mixerCtrl: *const crate::types::AudioMixerControl) {
    if sapmComponent.is_null() || path.is_null() || mixerCtrl.is_null() {
        return;
    }

    let sapm_ref = unsafe { &*sapmComponent };
    let reg: u32;
    let shift: u32;
    let mask: u32;
    let invert: u32;
    let mixer = unsafe { &*mixerCtrl };
    reg = mixer.reg;
    shift = mixer.shift;
    mask = mixer.mask;
    invert = mixer.invert;

    let codec = sapm_ref.codec;
    if codec.is_null() {
        return;
    }

    let dev_data = unsafe { (*codec).devData };
    if dev_data.is_null() {
        return;
    }

    let ret;
    let read_fn = unsafe { (*dev_data).Read };
    if let Some(read) = read_fn {
        let mut curValue: u32 = 0;
        ret = unsafe { read(codec as *const crate::types::CodecDevice, reg, &mut curValue as *mut u32) };
        if ret != crate::types::HDF_SUCCESS as i32 {
            return;
        }

        curValue = (curValue >> shift) & mask;
        let path_mut = unsafe { &mut *path };
        if (invert != 0 && curValue == 0) || (invert == 0 && curValue != 0) {
            path_mut.connect = 1u8;
        } else {
            path_mut.connect = 0u8;
        }
    }
}

fn AudioSapmSetPathStatus(sapmComponent: *const crate::types::AudioSapmComponent, path: *mut crate::types::AudioSapmpath, i: i32)-> i32 {
    if sapmComponent.is_null() || path.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let comp = unsafe { &*sapmComponent };
    let sapm_type = comp.sapmType;
    let news = unsafe { std::slice::from_raw_parts(comp.kcontrolNews, comp.kcontrolsNum as usize) };
    match sapm_type {
        crate::types::AUDIO_SAPM_MIXER
        | crate::types::AUDIO_SAPM_ANALOG_SWITCH
        | crate::types::AUDIO_SAPM_MIXER_NAMED_CTRL => {
            let kcontrol = &news[i as usize];
            let mixer_ptr = kcontrol.privateValue as *const crate::types::AudioMixerControl;
            crate::src_audio_sapm::MixerSetPathStatus(sapmComponent, path, mixer_ptr);
        }
        crate::types::AUDIO_SAPM_MUX => {
            let kcontrol = &news[i as usize];
            let enum_ptr = kcontrol.privateValue as *const crate::types::AudioEnumKcontrol;
            crate::src_audio_sapm::MuxSetPathStatus(sapmComponent, path, enum_ptr, i);
        }
        crate::types::AUDIO_SAPM_VALUE_MUX => {
            let kcontrol = &news[i as usize];
            let enum_ptr = kcontrol.privateValue as *const crate::types::AudioEnumKcontrol;
            crate::src_audio_sapm::MuxValueSetPathStatus(sapmComponent, path, enum_ptr, i);
        }
        _ => {
            let path_mut = unsafe { &mut *path };
            path_mut.connect = 1u8;
        }
    }
    crate::types::HDF_SUCCESS
}

fn AudioSapmConnectMux(audioCard: *mut crate::types::AudioCard, source: *mut crate::types::AudioSapmComponent, sink: *mut crate::types::AudioSapmComponent, path: *mut crate::types::AudioSapmpath, controlName: *const std::ffi::c_char)-> i32 {
    if audioCard.is_null() || source.is_null() || sink.is_null() || path.is_null() || controlName.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let sink_ref = unsafe { &*sink };
    if sink_ref.kcontrolNews.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let enumKtl: *mut crate::types::AudioEnumKcontrol = unsafe {
        let kcontrol = &mut *sink_ref.kcontrolNews;
        &mut kcontrol.privateValue as *mut std::ffi::c_ulong as *mut crate::types::AudioEnumKcontrol
    };
    if enumKtl.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let ktl = unsafe { &*enumKtl };
    if ktl.texts.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let max = ktl.max as usize;
    let texts = unsafe { std::slice::from_raw_parts(ktl.texts, max) };
    for i in 0..max {
        let text_ptr = texts[i];
        if unsafe { libc::strcmp(controlName, text_ptr) } == 0 {
            unsafe {
                // DListInsertHead: insert &path->list into &audioCard->paths
                {
                    let entry = std::ptr::addr_of_mut!((*path).list);
                    let head = std::ptr::addr_of_mut!((*audioCard).paths);
                    (*entry).next = (*head).next;
                    (*entry).prev = head;
                    (*(*head).next).prev = entry;
                    (*head).next = entry;
                }
                // DListInsertHead: insert &path->listSink into &sink->sources
                {
                    let entry = std::ptr::addr_of_mut!((*path).listSink);
                    let head = std::ptr::addr_of_mut!((*sink).sources);
                    (*entry).next = (*head).next;
                    (*entry).prev = head;
                    (*(*head).next).prev = entry;
                    (*head).next = entry;
                }
                // DListInsertHead: insert &path->listSource into &source->sinks
                {
                    let entry = std::ptr::addr_of_mut!((*path).listSource);
                    let head = std::ptr::addr_of_mut!((*source).sinks);
                    (*entry).next = (*head).next;
                    (*entry).prev = head;
                    (*(*head).next).prev = entry;
                    (*head).next = entry;
                }
                (*path).name = text_ptr as *mut std::ffi::c_char;
            }
            crate::src_audio_sapm::AudioSapmSetPathStatus(sink as *const _, path, i as i32);
            return crate::types::HDF_SUCCESS;
        }
    }

    crate::types::HDF_FAILURE
}

fn AudioSapmConnectMixer(
    audioCard: *mut crate::types::AudioCard,
    source: *mut crate::types::AudioSapmComponent,
    sink: *mut crate::types::AudioSapmComponent,
    path: *mut crate::types::AudioSapmpath,
    controlName: *const std::ffi::c_char,
) -> i32 {
    if audioCard.is_null() || source.is_null() || sink.is_null() || path.is_null() || controlName.is_null() {
        let func_name = b"AudioSapmConnectMixer\0".as_ptr() as *const std::ffi::c_char;
        let tag = b"HDF_AUDIO_SAPM\0".as_ptr() as *const std::ffi::c_char;
        let fmt = b"[%s][line:%d]: input params check error\0".as_ptr() as *const std::ffi::c_char;
        sapm_log!(crate::types::LOG_ERROR, tag, fmt, func_name, 634i32);
        return HDF_FAILURE as i32;
    }

    let sink_mut = unsafe { &mut *sink };
    let knum = sink_mut.kcontrolsNum;
    let kcontrol_news = unsafe { std::slice::from_raw_parts(sink_mut.kcontrolNews, knum as usize) };
    let mut i = 0i32;
    while i < knum {
        let kc_name = kcontrol_news[i as usize].name;
        if kc_name.is_null() {
            i += 1;
            continue;
        }
        let cmp = unsafe { libc::strcmp(controlName, kc_name) };
        if cmp == 0 {
            let name_len = unsafe { libc::strlen(kc_name) as usize };
            let new_name = unsafe { OsalMemCalloc((name_len + 1) as size_t) as *mut std::ffi::c_char };
            if new_name.is_null() {
                let func_name = b"AudioSapmConnectMixer\0".as_ptr() as *const std::ffi::c_char;
                let tag = b"HDF_AUDIO_SAPM\0".as_ptr() as *const std::ffi::c_char;
                let fmt = b"[%s][line:%d]: malloc path->name fail!\0".as_ptr() as *const std::ffi::c_char;
                sapm_log!(LOG_ERROR, tag, fmt, func_name, 646i32);
                return HDF_FAILURE as i32;
            }

            // Copy name and assign to path->name
            let copy_len = name_len + 1;
            let ret = unsafe {
                crate::compat::memcpy_s(
                    new_name as *mut std::ffi::c_void,
                    copy_len as size_t,
                    kc_name as *const std::ffi::c_void,
                    copy_len as size_t,
                )
            };
            if ret != 0 {
                unsafe { OsalMemFree(new_name as *mut std::ffi::c_void); }
                let func_name = b"AudioSapmConnectMixer\0".as_ptr() as *const std::ffi::c_char;
                let tag = b"HDF_AUDIO_SAPM\0".as_ptr() as *const std::ffi::c_char;
                let fmt = b"[%s][%d]: memcpy cpt->componentName fail!\0".as_ptr() as *const std::ffi::c_char;
                sapm_log!(LOG_ERROR, tag, fmt, func_name, 652i32);
                return HDF_FAILURE as i32;
            }

            // Assign path->name after copy succeeded
            unsafe { (*path).name = new_name; }

            // DListInsertHead(&path->list, &audioCard->paths)
            unsafe {
                let entry = &mut (*path).list as *mut DListHead;
                let head = &mut (*audioCard).paths as *mut DListHead;
                (*entry).next = (*head).next;
                (*entry).prev = head;
                (*(*head).next).prev = entry;
                (*head).next = entry;
            }

            // DListInsertHead(&path->listSink, &sink->sources)
            unsafe {
                let entry = &mut (*path).listSink as *mut DListHead;
                let head = &mut sink_mut.sources as *mut DListHead;
                (*entry).next = (*head).next;
                (*entry).prev = head;
                (*(*head).next).prev = entry;
                (*head).next = entry;
            }

            // DListInsertHead(&path->listSource, &source->sinks)
            unsafe {
                let entry = &mut (*path).listSource as *mut DListHead;
                let head = &mut (*source).sinks as *mut DListHead;
                (*entry).next = (*head).next;
                (*entry).prev = head;
                (*(*head).next).prev = entry;
                (*head).next = entry;
            }

            crate::src_audio_sapm::AudioSapmSetPathStatus(sink, path, i);
            return HDF_SUCCESS as i32;
        }
        i += 1;
    }

    HDF_FAILURE as i32
}

fn AudioSampStaticOrDynamicPath(audioCard: *mut crate::types::AudioCard, source: *mut crate::types::AudioSapmComponent, sink: *mut crate::types::AudioSapmComponent, path: *mut crate::types::AudioSapmpath, route: *const crate::types::AudioSapmRoute)-> i32 {
    let route_ref = unsafe { &*route };
    let sink_ref = unsafe { &*sink };
    let control = route_ref.control;
    let sink_type = sink_ref.sapmType;
    if control.is_null() {
        // direct path insertions (unsafe linked list ops)
        unsafe {
            let entry = &mut (*path).list as *mut crate::types::DListHead;
            let head = &mut (*audioCard).paths as *mut crate::types::DListHead;
            (*entry).next = (*head).next;
            (*entry).prev = head;
            (*(*head).next).prev = entry;
            (*head).next = entry;

            let entry = &mut (*path).listSink as *mut crate::types::DListHead;
            let head = &mut (*sink).sources as *mut crate::types::DListHead;
            (*entry).next = (*head).next;
            (*entry).prev = head;
            (*(*head).next).prev = entry;
            (*head).next = entry;

            let entry = &mut (*path).listSource as *mut crate::types::DListHead;
            let head = &mut (*source).sinks as *mut crate::types::DListHead;
            (*entry).next = (*head).next;
            (*entry).prev = head;
            (*(*head).next).prev = entry;
            (*head).next = entry;

            (*path).connect = 1;
        }
        return crate::types::HDF_SUCCESS;
    }

    match sink_type {
        crate::types::AUDIO_SAPM_MUX
        | crate::types::AUDIO_SAPM_VIRT_MUX
        | crate::types::AUDIO_SAPM_VALUE_MUX => {
            let ret = crate::src_audio_sapm::AudioSapmConnectMux(
                audioCard,
                source,
                sink,
                path,
                control,
            );
            if ret != crate::types::HDF_SUCCESS {
                return crate::types::HDF_FAILURE;
            }
        }
        crate::types::AUDIO_SAPM_ANALOG_SWITCH
        | crate::types::AUDIO_SAPM_MIXER
        | crate::types::AUDIO_SAPM_MIXER_NAMED_CTRL
        | crate::types::AUDIO_SAPM_PGA
        | crate::types::AUDIO_SAPM_SPK => {
            let ret = crate::src_audio_sapm::AudioSapmConnectMixer(
                audioCard,
                source,
                sink,
                path,
                control,
            );
            if ret != crate::types::HDF_SUCCESS {
                return crate::types::HDF_FAILURE;
            }
        }
        _ => {
            unsafe {
                let entry = &mut (*path).list as *mut crate::types::DListHead;
                let head = &mut (*audioCard).paths as *mut crate::types::DListHead;
                (*entry).next = (*head).next;
                (*entry).prev = head;
                (*(*head).next).prev = entry;
                (*head).next = entry;

                let entry = &mut (*path).listSink as *mut crate::types::DListHead;
                let head = &mut (*sink).sources as *mut crate::types::DListHead;
                (*entry).next = (*head).next;
                (*entry).prev = head;
                (*(*head).next).prev = entry;
                (*head).next = entry;

                let entry = &mut (*path).listSource as *mut crate::types::DListHead;
                let head = &mut (*source).sinks as *mut crate::types::DListHead;
                (*entry).next = (*head).next;
                (*entry).prev = head;
                (*(*head).next).prev = entry;
                (*head).next = entry;

                (*path).connect = 1;
            }
        }
    }
    crate::types::HDF_SUCCESS
}

fn AudioSampExtComponentsCheck(cptSource: *mut crate::types::AudioSapmComponent, cptSink: *mut crate::types::AudioSapmComponent) {
    if cptSource.is_null() || cptSink.is_null() {
        return;
    }
    let sink = unsafe { &mut *cptSink };
    let source = unsafe { &mut *cptSource };
    let sink_type = sink.sapmType;
    let source_type = source.sapmType;

    if sink_type == crate::types::AUDIO_SAPM_INPUT {
        if source_type == crate::types::AUDIO_SAPM_MICBIAS
            || source_type == crate::types::AUDIO_SAPM_MIC
            || source_type == crate::types::AUDIO_SAPM_LINE
            || source_type == crate::types::AUDIO_SAPM_OUTPUT
        {
            sink.external = 1;
        }
    }
    if source_type == crate::types::AUDIO_SAPM_OUTPUT {
        if sink_type == crate::types::AUDIO_SAPM_SPK
            || sink_type == crate::types::AUDIO_SAPM_HP
            || sink_type == crate::types::AUDIO_SAPM_LINE
            || sink_type == crate::types::AUDIO_SAPM_INPUT
        {
            source.external = 1;
        }
    }
}

fn AudioSapmAddRoute(audioCard: *mut crate::types::AudioCard, route: *const crate::types::AudioSapmRoute)-> i32 {
    use crate::types::*;
    use core::ffi::{c_char, c_void};

    if route.is_null() {
        return HDF_FAILURE;
    }
    let route_ref = unsafe { &*route };
    let (source_ptr, sink_ptr) = (route_ref.source, route_ref.sink);
    if source_ptr.is_null() || sink_ptr.is_null() {
        sapm_log!(crate::types::LOG_ERROR, b"HDF_AUDIO_SAPM\0".as_ptr() as *const c_char, b"[%s][line:%d]: input params check error: route is NULL.\0".as_ptr() as *const c_char, b"AudioSapmAddRoute\0".as_ptr() as *const c_char, 755i32);
        return HDF_FAILURE;
    }

    let mut cptSource: *mut AudioSapmComponent = core::ptr::null_mut();
    let mut cptSink: *mut AudioSapmComponent = core::ptr::null_mut();

    let offset = unsafe { std::ptr::addr_of!((*std::ptr::null::<crate::types::AudioSapmComponent>()).list) as usize };

    // Initialize sapmComponent to first entry
    let mut sapmComponent: *mut AudioSapmComponent = {
        let audioCard_ref = unsafe { &*audioCard };
        let next_ptr = audioCard_ref.components.next;
        unsafe { ((next_ptr as *const u8).sub(offset)) as *mut AudioSapmComponent }
    };

    let components_ptr: *const DListHead = unsafe { &(*audioCard).components as *const DListHead };

    while unsafe { &(*sapmComponent).list as *const DListHead != components_ptr } {
        let current = sapmComponent;
        let current_ref = unsafe { &*current };
        let next_ptr = current_ref.list.next;
        let next_sapmComponent = unsafe { ((next_ptr as *const u8).sub(offset)) as *mut AudioSapmComponent };

        let component_name = current_ref.componentName;
        if !component_name.is_null() {
            // source match
            if cptSource.is_null() {
                let route_source = route_ref.source;
                if unsafe { libc::strcmp(
                    component_name as *const c_char,
                    route_source,
                ) == 0 } {
                    cptSource = current;
                } else if cptSink.is_null() {
                    let route_sink = route_ref.sink;
                    if unsafe { libc::strcmp(
                        component_name as *const c_char,
                        route_sink,
                    ) == 0 } {
                        cptSink = current;
                    }
                }
            } else if cptSink.is_null() {
                let route_sink = route_ref.sink;
                if unsafe { libc::strcmp(
                    component_name as *const c_char,
                    route_sink,
                ) == 0 } {
                    cptSink = current;
                }
            }

            if !cptSource.is_null() && !cptSink.is_null() {
                break;
            }
        }

        sapmComponent = next_sapmComponent;
    }

    if cptSource.is_null() || cptSink.is_null() {
        let log_tag = b"HDF_AUDIO_SAPM\0".as_ptr() as *const c_char;
        let log_fmt = b"[%s][line:%d]: find component fail!\0".as_ptr() as *const c_char;
        let log_func = b"AudioSapmAddRoute\0".as_ptr() as *const c_char;
        sapm_log!(LOG_ERROR, log_tag, log_fmt, log_func, 775i32);
        return HDF_FAILURE;
    }

    let path = unsafe { OsalMemCalloc(core::mem::size_of::<AudioSapmpath>() as size_t) as *mut AudioSapmpath };
    if path.is_null() {
        let log_tag = b"HDF_AUDIO_SAPM\0".as_ptr() as *const c_char;
        let log_fmt = b"[%s][line:%d]: malloc path fail!\0".as_ptr() as *const c_char;
        let log_func = b"AudioSapmAddRoute\0".as_ptr() as *const c_char;
        sapm_log!(LOG_ERROR, log_tag, log_fmt, log_func, 781i32);
        return HDF_FAILURE;
    }

    // Initialize path fields and DListHeads
    unsafe {
        (*path).source = cptSource;
        (*path).sink = cptSink;

        dlist_head_init(std::ptr::addr_of_mut!((*path).list));
        dlist_head_init(std::ptr::addr_of_mut!((*path).listSink));
        dlist_head_init(std::ptr::addr_of_mut!((*path).listSource));
    }

    crate::src_audio_sapm::AudioSampExtComponentsCheck(cptSource, cptSink);

    let ret = crate::src_audio_sapm::AudioSampStaticOrDynamicPath(
        audioCard,
        cptSource,
        cptSink,
        path,
        route,
    );

    if ret != HDF_SUCCESS {
        unsafe { OsalMemFree(path as *mut c_void); }
        sapm_log!(crate::types::LOG_ERROR, b"HDF_AUDIO_SAPM\0".as_ptr() as *const c_char, b"[%s][line:%d]: static or dynamic path fail!\0".as_ptr() as *const c_char, b"AudioSapmAddRoute\0".as_ptr() as *const c_char, 796i32);
        return HDF_FAILURE;
    }

    HDF_SUCCESS
}

pub extern "C" fn AudioSapmAddRoutes(audioCard: *mut crate::types::AudioCard, route: *const crate::types::AudioSapmRoute, routeMaxNum: i32) -> i32 {
    if audioCard.is_null() {
        let func_name = b"AudioSapmAddRoutes\0".as_ptr() as *const ::core::ffi::c_char;
        let fmt = b"[%s][line:%d]: input params check error: audioCard is NULL.\0".as_ptr() as *const ::core::ffi::c_char;
        let tag = b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char;
        sapm_log!(crate::types::LOG_ERROR, tag, fmt, func_name, 808i32);
    }
    if route.is_null() {
        let func_name = b"AudioSapmAddRoutes\0".as_ptr() as *const ::core::ffi::c_char;
        let fmt = b"[%s][line:%d]: input params check error: route is NULL.\0".as_ptr() as *const ::core::ffi::c_char;
        let tag = b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char;
        sapm_log!(crate::types::LOG_ERROR, tag, fmt, func_name, 812i32);
        return HDF_FAILURE;
    }
    if routeMaxNum <= 0 {
        return HDF_SUCCESS;
    }
    let routes = unsafe { std::slice::from_raw_parts(route, routeMaxNum as usize) };
    for route_item in routes {
        let ret = crate::src_audio_sapm::AudioSapmAddRoute(audioCard, route_item as *const _);
        if ret != HDF_SUCCESS {
            let func_name = b"AudioSapmAddRoutes\0".as_ptr() as *const ::core::ffi::c_char;
            let fmt = b"[%s][line:%d]: AudioSapmAddRoute failed!\0".as_ptr() as *const ::core::ffi::c_char;
            let tag = b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char;
            sapm_log!(crate::types::LOG_ERROR, tag, fmt, func_name, 819i32);
            return HDF_FAILURE;
        }
    }
    return HDF_SUCCESS;
}

fn AudioSapmNewMixerControls(sapmComponent: *const crate::types::AudioSapmComponent, audioCard: *mut crate::types::AudioCard)-> i32 {
    if sapmComponent.is_null() {
        sapm_log!(crate::types::LOG_ERROR, b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char, b"[%s][line:%d]: input params check error: sapmComponent is NULL.\0".as_ptr() as *const ::core::ffi::c_char, b"AudioSapmNewMixerControls\0".as_ptr() as *const ::core::ffi::c_char, 834i32);
        return crate::types::HDF_FAILURE;
    }
    let comp = unsafe { &*sapmComponent };
    if comp.kcontrols.is_null() {
        sapm_log!(crate::types::LOG_ERROR, b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char, b"[%s][line:%d]: input params check error: sapmComponent->kcontrols is NULL.\0".as_ptr() as *const ::core::ffi::c_char, b"AudioSapmNewMixerControls\0".as_ptr() as *const ::core::ffi::c_char, 834i32);
        return crate::types::HDF_FAILURE;
    }
    if audioCard.is_null() {
        sapm_log!(
            crate::types::LOG_ERROR,
            b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s][line:%d]: input params check error: audioCard is NULL.\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"AudioSapmNewMixerControls\0".as_ptr() as *const ::core::ffi::c_char,
            838i32,
        );
        return crate::types::HDF_FAILURE;
    }

    let comp = unsafe { &*sapmComponent };
    let card = unsafe { &mut *audioCard };
    let sapm_comp_mut = sapmComponent as *mut crate::types::AudioSapmComponent;
    let kcontrols_num = comp.kcontrolsNum;
    let kcontrol_news = comp.kcontrolNews;
    let kcontrol_news_slice = if kcontrol_news.is_null() {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(kcontrol_news, kcontrols_num as usize) }
    };
    let kcontrols_slice = if comp.kcontrols.is_null() {
        &mut []
    } else {
        unsafe { std::slice::from_raw_parts_mut(comp.kcontrols, kcontrols_num as usize) }
    };

    // Compute offset of listSink inside AudioSapmpath (like container_of)
    let offset = {
        let null_ptr: *const crate::types::AudioSapmpath = ::core::ptr::null();
        unsafe {
            (::core::ptr::addr_of!((*null_ptr).listSink) as usize) - (null_ptr as usize)
        }
    };

    for i in 0..kcontrols_num {
        let sources_head = ::core::ptr::addr_of!(comp.sources);
        let mut list_sink_ptr = comp.sources.next;

        while (list_sink_ptr as *const u8) != (sources_head as *const u8) {
            let path = unsafe { (list_sink_ptr as *mut u8).sub(offset) as *mut crate::types::AudioSapmpath };
            let path_ref = unsafe { &mut *path };

            let path_name = path_ref.name;
            if kcontrol_news_slice.is_empty() {
                list_sink_ptr = path_ref.listSink.next;
                continue;
            }
            let kcontrol_news_i = &kcontrol_news_slice[i as usize];
            let kcontrol_news_i_name = kcontrol_news_i.name;

            if path_name.is_null() || kcontrol_news_i_name.is_null() {
                list_sink_ptr = path_ref.listSink.next;
                continue;
            }

            let cmp = unsafe { libc::strcmp(path_name, kcontrol_news_i_name) };
            if cmp != 0 {
                list_sink_ptr = path_ref.listSink.next;
                continue;
            }

            let kcontrol = unsafe {
                crate::compat::AudioAddControl(
                    audioCard,
                    &kcontrol_news_slice[i as usize] as *const crate::types::AudioKcontrol,
                )
            };
            if kcontrol.is_null() {
                let log_tag = b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char;
                let log_fmt = b"[%s][line:%d]: add control fail!\0".as_ptr() as *const ::core::ffi::c_char;
                let log_func = b"AudioSapmNewMixerControls\0".as_ptr() as *const ::core::ffi::c_char;
                sapm_log!(crate::types::LOG_ERROR, log_tag, log_fmt, log_func, 855i32);
                return crate::types::HDF_FAILURE;
            }

            unsafe {
                path_ref.kcontrol = kcontrol;
                kcontrols_slice[i as usize] = kcontrol;
            }

            // Inline DListInsertHead: insert &kcontrol->list into &audioCard->controls
            unsafe {
                let entry = ::core::ptr::addr_of_mut!((*kcontrol).list);
                let head = ::core::ptr::addr_of_mut!(card.controls);

                (*entry).next = (*head).next;
                (*entry).prev = head;
                (*(*head).next).prev = entry;
                (*head).next = entry;
            }

            list_sink_ptr = path_ref.listSink.next;        }
    }

    crate::types::HDF_SUCCESS
}

fn AudioSapmNewMuxControls(
    sapmComponent: *mut crate::types::AudioSapmComponent,
    audioCard: *mut crate::types::AudioCard,
) -> i32 {
    if sapmComponent.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let comp = unsafe { &mut *sapmComponent };
    let card = unsafe { &mut *audioCard };
    let (kcontrol_news, kcontrols_num, kcontrols) = {
        let kn = comp.kcontrolNews;
        let knum = comp.kcontrolsNum;
        let kctrl = comp.kcontrols;
        (kn, knum, kctrl)
    };
    if kcontrol_news.is_null() || audioCard.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let kcontrols_present = !kcontrols.is_null();
    if kcontrols_num != 1 {
        return crate::types::HDF_FAILURE;
    }
    let kctrl = unsafe { crate::compat::AudioAddControl(audioCard, kcontrol_news) };
    if kctrl.is_null() {
        return crate::types::HDF_FAILURE;
    }
    if !kcontrols_present {
        unsafe { crate::compat::OsalMemFree(kctrl as *mut ::core::ffi::c_void); }
        return crate::types::HDF_FAILURE;
    }
    // Write kcontrols slot and DListInsert
    unsafe {
        *comp.kcontrols = kctrl;
        let entry = &mut (*kctrl).list as *mut crate::types::DListHead;
        let head = &mut card.controls as *mut crate::types::DListHead;
        (*entry).next = (*head).next;
        (*entry).prev = head;
        (*(*head).next).prev = entry;
        (*head).next = entry;
    }
    crate::types::HDF_SUCCESS
}

fn AudioSapmPowerSeqInsert(newSapmComponent: *mut crate::types::AudioSapmComponent, list: *mut crate::types::DListHead, isPowerUp: i8) {
    let component_name_ok = if newSapmComponent.is_null() || list.is_null() {
        false
    } else {
        let comp_ref = unsafe { &*newSapmComponent };
        !comp_ref.componentName.is_null()
    };
    if !component_name_ok {
        return;
    }

    let seq = if isPowerUp != 0 {
        g_audioSapmPowerUpSeq()
    } else {
        g_audioSapmPowerDownSeq()
    };

    let new_sapm_type = unsafe { (*newSapmComponent).sapmType } as usize;
    let new_seq = seq[new_sapm_type];

    let offset = unsafe {
        std::ptr::addr_of!((*std::ptr::null::<crate::types::AudioSapmComponent>()).powerList) as usize
    };
    let list_ref = unsafe { &*list };
    let mut current = list_ref.next;
    while current != list {
        let sapmComponent = unsafe { (current as *mut u8).offset(-(offset as isize)) as *mut crate::types::AudioSapmComponent };
        let cur_sapm_type = unsafe { (*sapmComponent).sapmType } as usize;
        let cur_seq = seq[cur_sapm_type];

        if new_seq < cur_seq {
            let entry = unsafe { &mut (*newSapmComponent).powerList as *mut crate::types::DListHead };
            let head = unsafe { &mut (*sapmComponent).powerList as *mut crate::types::DListHead };
            unsafe {
                (*entry).next = head;
                (*entry).prev = (*head).prev;
                (*(*head).prev).next = entry;
                (*head).prev = entry;
            }
            return;
        }
        let sapmComp_ref = unsafe { &*sapmComponent };
        current = sapmComp_ref.powerList.next;
    }

    let entry = unsafe { &mut (*newSapmComponent).powerList as *mut crate::types::DListHead };
    unsafe {
        (*entry).next = list;
        (*entry).prev = (*list).prev;
        (*(*list).prev).next = entry;
        (*list).prev = entry;
    }
}

fn AudioSapmSetPower(audioCard: *mut crate::types::AudioCard, sapmComponent: *mut crate::types::AudioSapmComponent, power: u8, upList: *mut crate::types::DListHead, downList: *mut crate::types::DListHead) {
    if sapmComponent.is_null() {
        let tag: *const i8 = b"HDF_AUDIO_SAPM\0".as_ptr() as *const i8;
        let fmt: *const i8 = b"[%s][line:%d]: input param sapmComponent is NULL.\0".as_ptr() as *const i8;
        let func: *const i8 = b"AudioSapmSetPower\0".as_ptr() as *const i8;
        sapm_log!(crate::types::LOG_ERROR as u32, tag, fmt, func, 933i32);
        return;
    }

    let comp = unsafe { &mut *sapmComponent };

    let list_sink_offset = unsafe {
        std::ptr::addr_of!((*std::ptr::null::<crate::types::AudioSapmpath>()).listSink) as usize
    };
    let list_source_offset = unsafe {
        std::ptr::addr_of!((*std::ptr::null::<crate::types::AudioSapmpath>()).listSource) as usize
    };

    let sources_head = &comp.sources as *const crate::types::DListHead;
    let mut node: *mut crate::types::DListHead = comp.sources.next;
    while node != sources_head as *mut crate::types::DListHead {
        // container_of via listSink
        let path = (node as usize - list_sink_offset) as *mut crate::types::AudioSapmpath;
        let path_ref = unsafe { &*path };
        let (source, connect) = (path_ref.source, path_ref.connect);
        if !source.is_null() {
            let source_ref = unsafe { &*source };
            let source_power = source_ref.power;
            if source_power != power && connect != 0 {
                let source_dirty = &source_ref.dirty as *const crate::types::DListHead;
                let source_next = source_ref.dirty.next;
                if source_next == source_dirty as *mut crate::types::DListHead {
                    // DListInsertHead strategy: insert source->dirty at audioCard->sapmDirty
                    unsafe {
                        let entry = &mut (*source).dirty as *mut crate::types::DListHead;
                        let head = &(*audioCard).sapmDirty as *const _ as *mut crate::types::DListHead;
                        (*entry).next = head;
                        (*entry).prev = (*head).prev;
                        (*(*head).prev).next = entry;
                        (*head).prev = entry;
                    }
                }
            }
        }
        let path_ref_sink = unsafe { &*path };
        node = path_ref_sink.listSink.next;
    }

    let sinks_head = &comp.sinks as *const crate::types::DListHead;
    node = comp.sinks.next;
    while node != sinks_head as *mut crate::types::DListHead {
        // container_of via listSource
        let path = (node as usize - list_source_offset) as *mut crate::types::AudioSapmpath;
        let path_ref = unsafe { &*path };
        let (sink, connect) = (path_ref.sink, path_ref.connect);
        if !sink.is_null() {
            let sink_ref = unsafe { &*sink };
            let sink_power = sink_ref.power;
            if sink_power != power && connect != 0 {
                let sink_dirty = &sink_ref.dirty as *const crate::types::DListHead;
                let sink_next = sink_ref.dirty.next;
                if sink_next == sink_dirty as *mut crate::types::DListHead {
                    // DListInsertHead strategy: insert sink->dirty at audioCard->sapmDirty
                    unsafe {
                        let entry = &mut (*sink).dirty as *mut crate::types::DListHead;
                        let head = &(*audioCard).sapmDirty as *const _ as *mut crate::types::DListHead;
                        (*entry).next = head;
                        (*entry).prev = (*head).prev;
                        (*(*head).prev).next = entry;
                        (*head).prev = entry;
                    }
                }
            }
        }
        let path_ref_src = unsafe { &*path };
        node = path_ref_src.listSource.next;
    }

    if power != 0 {
        crate::src_audio_sapm::AudioSapmPowerSeqInsert(sapmComponent, upList, power as i8);
    } else {
        crate::src_audio_sapm::AudioSapmPowerSeqInsert(sapmComponent, downList, power as i8);
    }
}

fn AudioSapmPowerUpSeqRun(list: *const crate::types::DListHead) {
    if list.is_null() {
        let tag = b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char;
        let fmt = b"[%s][line:%d]: input param list is NULL.\0".as_ptr() as *const ::core::ffi::c_char;
        let fname = b"AudioSapmPowerUpSeqRun\0".as_ptr() as *const ::core::ffi::c_char;
        sapm_log!(crate::types::LOG_ERROR as u32, tag, fmt, fname, 972i32);
    }

    let offset: usize = unsafe {
        std::ptr::addr_of!((*std::ptr::null::<crate::types::AudioSapmComponent>()).powerList) as usize
    };

    let list_ref = unsafe { &*list };
    let mut current: *mut crate::types::DListHead = list_ref.next;
    let list_ptr: *const crate::types::DListHead = list;

    while current as *const _ != list_ptr {
        let component = unsafe {
            (current as *mut u8).offset(-(offset as isize)) as *mut crate::types::AudioSapmComponent
        };

        let comp = unsafe { &mut *component };
        let power_val = comp.power;
        let invert_val = comp.invert;
        let reg_val = comp.reg;
        let codec_val = comp.codec;
        let mask_val = comp.mask;
        let shift_val = comp.shift;
        let comp_name = comp.componentName;

        if power_val == 0 {
            let val: u32 = if invert_val != 0 { 0 } else { 1 };

            comp.power = 1;

            if reg_val != crate::types::AUDIO_NO_SAPM_REG {
                unsafe {
                    crate::compat::AudioUpdateCodecRegBits(
                        codec_val,
                        reg_val,
                        mask_val,
                        shift_val as u32,
                        val,
                    );
                }

                let tag = b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char;
                let fmt = b"[%s][line:%d]: Sapm Codec %s Power Up.\0".as_ptr() as *const ::core::ffi::c_char;
                let fname = b"AudioSapmPowerUpSeqRun\0".as_ptr() as *const ::core::ffi::c_char;
                sapm_log!(crate::types::LOG_INFO as u32, tag, fmt, fname, 989i32, comp_name as *const ::core::ffi::c_char);
            }
        }

        current = comp.powerList.next;
    }
}

fn AudioSapmPowerDownSeqRun(list: *const crate::types::DListHead) {
    if list.is_null() {
        let tag = b"HDF_AUDIO_SAPM\0" as *const u8 as *const std::ffi::c_char;
        let fmt = b"[%s][line:%d]: sapm input param list is NULL.\0" as *const u8 as *const std::ffi::c_char;
        let fname = b"AudioSapmPowerDownSeqRun\0" as *const u8 as *const std::ffi::c_char;
        sapm_log!(crate::types::LOG_ERROR as u32, tag, fmt, fname, 1005i32);
        return;
    }

    let offset = unsafe {
        std::ptr::addr_of!((*std::ptr::null::<crate::types::AudioSapmComponent>()).powerList) as usize
    };

    let list_ref = unsafe { &*list };
    let next = list_ref.next;
    let mut sapm_component: *mut crate::types::AudioSapmComponent = unsafe {
        ((next as *const u8).offset(-(offset as isize))) as *mut crate::types::AudioSapmComponent
    };

    while unsafe { &(*sapm_component).powerList as *const crate::types::DListHead != list } {
        let comp = unsafe { &mut *sapm_component };
        let (power, invert, reg, mask, shift, codec, comp_name) = {
            (comp.power, comp.invert, comp.reg, comp.mask, comp.shift, comp.codec, comp.componentName)
        };

        if power == 1 {
            let val: u32 = if invert != 0 { 1 } else { 0 };
            comp.power = 0;

            if reg != crate::types::AUDIO_NO_SAPM_REG {
                unsafe {
                    crate::compat::AudioUpdateCodecRegBits(codec, reg, mask, shift as u32, val);
                }

                let tag = b"HDF_AUDIO_SAPM\0" as *const u8 as *const std::ffi::c_char;
                let fmt = b"[%s][line:%d]: Sapm Codec %s Power Down.\0" as *const u8 as *const std::ffi::c_char;
                let fname = b"AudioSapmPowerDownSeqRun\0" as *const u8 as *const std::ffi::c_char;
                sapm_log!(crate::types::LOG_INFO as u32, tag, fmt, fname, 1032i32, comp_name as *const std::ffi::c_char);
            }
        }

        let next = comp.powerList.next;
        sapm_component = unsafe {
            ((next as *const u8).offset(-(offset as isize))) as *mut crate::types::AudioSapmComponent
        };
    }
}

fn AudioSapmPowerComponents(audioCard: *mut crate::types::AudioCard) {
    let mut sapmComponent: *mut crate::types::AudioSapmComponent = std::ptr::null_mut();
    let mut upList = crate::types::DListHead { next: std::ptr::null_mut(), prev: std::ptr::null_mut() };
    let up_ptr: *mut crate::types::DListHead = std::ptr::addr_of_mut!(upList);
    upList.next = up_ptr;
    upList.prev = up_ptr;
    let mut downList = crate::types::DListHead { next: std::ptr::null_mut(), prev: std::ptr::null_mut() };
    let down_ptr: *mut crate::types::DListHead = std::ptr::addr_of_mut!(downList);
    downList.next = down_ptr;
    downList.prev = down_ptr;

    if audioCard.is_null() {
        sapm_log!(crate::types::LOG_ERROR, b"HDF_AUDIO_SAPM\0".as_ptr() as *const i8, b"[%s][line:%d]: input param audioCard is NULL.\0".as_ptr() as *const i8, b"AudioSapmPowerComponents\0".as_ptr() as *const i8, 1040i32);
        return;
    }

    let card = unsafe { &mut *audioCard };

    let component_uninit = core::mem::MaybeUninit::<crate::types::AudioSapmComponent>::uninit();
    let component_base = component_uninit.as_ptr();
    let dirty_offset = unsafe {
        let dirty_ptr = std::ptr::addr_of!((*component_base).dirty);
        (dirty_ptr as usize) - (component_base as usize)
    };
    let list_offset = unsafe {
        let list_ptr = std::ptr::addr_of!((*component_base).list);
        (list_ptr as usize) - (component_base as usize)
    };

    let head_dirty: *const crate::types::DListHead = std::ptr::addr_of!(card.sapmDirty);
    let mut dirty_ptr: *const crate::types::DListHead = card.sapmDirty.next;
    sapmComponent = unsafe { (dirty_ptr as *const u8).sub(dirty_offset) as *mut crate::types::AudioSapmComponent };
    while std::ptr::addr_of!(unsafe { &*sapmComponent }.dirty) != head_dirty {
        let comp = unsafe { &mut *sapmComponent };
        let new_power: u8 =
            unsafe { comp.PowerCheck.unwrap()(comp as *const _) as u8 };
        comp.newPower = new_power;

        let current_power = comp.power;
        if new_power != current_power {
            let standby = card.sapmStandbyState;
            let clock_op = comp.PowerClockOp;
            let should_set = if standby && clock_op.is_some() {
                let ret = unsafe { clock_op.unwrap()(sapmComponent) };
                ret == crate::types::HDF_SUCCESS
            } else {
                true
            };
            if should_set {
                crate::src_audio_sapm::AudioSapmSetPower(
                    audioCard,
                    sapmComponent,
                    new_power,
                    std::ptr::addr_of_mut!(upList),
                    std::ptr::addr_of_mut!(downList),
                );
            }
        }

        let next_dirty = comp.dirty.next;
        sapmComponent = unsafe {
            (next_dirty as *const u8).sub(dirty_offset) as *mut crate::types::AudioSapmComponent
        };
    }

    let head_list: *const crate::types::DListHead = std::ptr::addr_of!(card.components);
    let mut list_ptr: *const crate::types::DListHead = card.components.next;
    sapmComponent = unsafe { (list_ptr as *const u8).sub(list_offset) as *mut crate::types::AudioSapmComponent };
    while std::ptr::addr_of!(unsafe { &*sapmComponent }.list) != head_list {
        let comp = unsafe { &mut *sapmComponent };
        unsafe {
            let dirty: *mut crate::types::DListHead = std::ptr::addr_of_mut!(comp.dirty);
            let prev = (*dirty).prev;
            let next = (*dirty).next;
            (*prev).next = next;
            (*next).prev = prev;
            (*dirty).prev = std::ptr::null_mut();
            (*dirty).next = std::ptr::null_mut();

            let head: *mut crate::types::DListHead = std::ptr::addr_of_mut!(comp.dirty);
            (*head).next = head;
            (*head).prev = head;
        }

        let next_list = comp.list.next;
        sapmComponent = unsafe {
            (next_list as *const u8).sub(list_offset) as *mut crate::types::AudioSapmComponent
        };
    }

    crate::src_audio_sapm::AudioSapmPowerDownSeqRun(std::ptr::addr_of!(downList) as *const _);
    crate::src_audio_sapm::AudioSapmPowerUpSeqRun(std::ptr::addr_of!(upList) as *const _);
}

fn ReadInitComponentPowerStatus(sapmComponent: *mut crate::types::AudioSapmComponent) {
    if sapmComponent.is_null() {
        let tag = b"HDF_AUDIO_SAPM\0".as_ptr() as *const std::ffi::c_char;
        let fmt = b"[%s][line:%d]: input param sapmComponent is NULL.\0".as_ptr() as *const std::ffi::c_char;
        let fname = b"ReadInitComponentPowerStatus\0".as_ptr() as *const std::ffi::c_char;
        sapm_log!(crate::types::LOG_ERROR, tag, fmt, fname, 1078i32);
        return;
    }
    let comp = unsafe { &mut *sapmComponent };
    if comp.codec.is_null() {
        let tag = b"HDF_AUDIO_SAPM\0".as_ptr() as *const std::ffi::c_char;
        let fmt = b"[%s][line:%d]: input param sapmComponent->codec is NULL.\0".as_ptr() as *const std::ffi::c_char;
        let fname = b"ReadInitComponentPowerStatus\0".as_ptr() as *const std::ffi::c_char;
        sapm_log!(crate::types::LOG_ERROR, tag, fmt, fname, 1078i32);
        return;
    }

    let reg = comp.reg;
    if reg != crate::types::AUDIO_NO_SAPM_REG {
        let codec_ptr = comp.codec;
        if codec_ptr.is_null() {
            return;
        }

        // Read dev_data, Read function pointer, shift, invert in one unsafe block
        let (dev_data, read_fn, shift, invert) = unsafe {
            let dev = (*codec_ptr).devData;
            let read = (*dev).Read;
            let shift_val = comp.shift;
            let invert_val = comp.invert;
            (dev, read, shift_val, invert_val)
        };

        if dev_data.is_null() || read_fn.is_none() {
            let tag = b"HDF_AUDIO_SAPM\0".as_ptr() as *const std::ffi::c_char;
            let fmt = b"[%s][line:%d]: read reg fail!\0".as_ptr() as *const std::ffi::c_char;
            let fname = b"ReadInitComponentPowerStatus\0".as_ptr() as *const std::ffi::c_char;
            sapm_log!(crate::types::LOG_ERROR, tag, fmt, fname, 1085i32);
            return;
        }

        let mut reg_val: u32 = 0;
        let ret = unsafe {
            read_fn.unwrap()(codec_ptr as *const crate::types::CodecDevice,
                              reg, &mut reg_val)
        };
        if ret != crate::types::HDF_SUCCESS {
            let tag = b"HDF_AUDIO_SAPM\0".as_ptr() as *const std::ffi::c_char;
            let fmt = b"[%s][line:%d]: read reg fail!\0".as_ptr() as *const std::ffi::c_char;
            let fname = b"ReadInitComponentPowerStatus\0".as_ptr() as *const std::ffi::c_char;
            sapm_log!(crate::types::LOG_ERROR, tag, fmt, fname, 1085i32);
            return;
        }

        reg_val &= 1u32 << shift;

        if invert != 0 {
            reg_val = if reg_val != 0 { 0 } else { 1 };
        }

        comp.power = if reg_val != 0 { 1 } else { 0 };
    }
}

fn AudioSapmThread(data: *mut std::ffi::c_void)-> i32 {
    let audioCard = data as *mut crate::types::AudioCard;
    let card = unsafe { &mut *audioCard };
    card.time = 0;
    loop {
        unsafe {
            OsalSleep(10u32);
        }
        crate::src_audio_sapm::AudioSapmTimerCallback(audioCard);
        card.time += 1;
    }
}

pub extern "C" fn AudioSapmSleep(audioCard: *mut crate::types::AudioCard) -> i32 {
    if audioCard.is_null() {
        sapm_log!(crate::types::LOG_ERROR, b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char, b"[%s][line:%d]: input param audioCard is NULL.\0".as_ptr() as *const ::core::ffi::c_char, b"AudioSapmSleep\0".as_ptr() as *const ::core::ffi::c_char, 1126i32);
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let _ = crate::src_audio_sapm::AudioSapmRefreshTime(audioCard, true);

    let sapmThreadName: *mut ::core::ffi::c_char = unsafe {
        OsalMemCalloc(60 as crate::types::size_t) as *mut ::core::ffi::c_char
    };
    if sapmThreadName.is_null() {
        return crate::types::HDF_ERR_MALLOC_FAIL;
    }

    let snprintf_ret: i32 = unsafe {
        snprintf_s(
            sapmThreadName,
            60 as crate::types::size_t,
            59 as crate::types::size_t,
            b"AudioSapmThread%u\0".as_ptr() as *const ::core::ffi::c_char,
            crate::globals::g_cardNum.load(core::sync::atomic::Ordering::Relaxed),
        )
    };
    if snprintf_ret < 0 {
            sapm_log!(crate::types::LOG_ERROR, b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char, b"[%s][line:%d]: snprintf_s failed.\0".as_ptr() as *const ::core::ffi::c_char, b"AudioSapmSleep\0".as_ptr() as *const ::core::ffi::c_char, 1136i32);
        return crate::types::HDF_FAILURE;
    }

    let mut audioSapmThread: crate::types::OsalThread = unsafe { core::mem::zeroed() };
    let thread_entry: crate::types::OsalThreadEntry = Some(audio_sapm_thread_thunk as unsafe extern "C" fn(*mut std::ffi::c_void) -> i32);
    let create_ret: i32 = unsafe {
        OsalThreadCreate(
            &mut audioSapmThread,
            thread_entry,
            audioCard as *mut ::core::ffi::c_void,
        )
    };
    if create_ret != crate::types::HDF_SUCCESS {
        sapm_log!(
            crate::types::LOG_ERROR,
            b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s][line:%d]: create sapm thread fail, ret=%d\0".as_ptr() as *const ::core::ffi::c_char,
            b"AudioSapmSleep\0".as_ptr() as *const ::core::ffi::c_char,
            1143i32,
            create_ret,
        );
        unsafe {
            OsalMemFree(sapmThreadName as *mut ::core::ffi::c_void);
        }
        return crate::types::HDF_FAILURE;
    }

    let mut threadCfg: crate::types::OsalThreadParam = unsafe { core::mem::zeroed() };
    threadCfg.name = sapmThreadName;
    threadCfg.priority = crate::types::OSAL_THREAD_PRI_DEFAULT;
    threadCfg.stackSize = 10000 as crate::types::size_t;

    let start_ret: i32 = unsafe { OsalThreadStart(&mut audioSapmThread, &threadCfg) };
    if start_ret != crate::types::HDF_SUCCESS {
        sapm_log!(
            crate::types::LOG_ERROR,
            b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s][line:%d]: start sapm thread fail, ret=%d\0".as_ptr() as *const ::core::ffi::c_char,
            b"AudioSapmSleep\0".as_ptr() as *const ::core::ffi::c_char,
            1154i32,
            start_ret,
        );
        unsafe {
            OsalThreadDestroy(&mut audioSapmThread);
            OsalMemFree(sapmThreadName as *mut ::core::ffi::c_void);
        }
        return crate::types::HDF_FAILURE;
    }

    crate::globals::g_cardNum.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    unsafe {
        (*audioCard).sapmStandbyState = false;
        (*audioCard).sapmSleepState = false;
        OsalMemFree(sapmThreadName as *mut ::core::ffi::c_void);
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioSapmNewControls(audioCard: *mut crate::types::AudioCard) -> i32 {
    use crate::types::*;
    let mut ret: i32 = HDF_SUCCESS as i32;
    if audioCard.is_null() {
        return HDF_ERR_INVALID_OBJECT as i32;
    }
    let (head, mut list_ptr, list_offset) = unsafe {
        let head = &mut (*audioCard).components as *mut DListHead;
        let list_ptr = (*head).next;
        let null_c: *const AudioSapmComponent = std::ptr::null();
        let offset = std::ptr::addr_of!((*null_c).list) as usize;
        (head, list_ptr, offset)
    };

    while list_ptr != head {
        let comp_ref = unsafe { &mut *( (list_ptr as *mut u8).sub(list_offset) as *mut AudioSapmComponent ) };
        // Read multiple fields via safe reference
        let new_cpt = comp_ref.newCpt;
        let knum = comp_ref.kcontrolsNum;
        let sapm_type = comp_ref.sapmType;
        let kcontrols_current = comp_ref.kcontrols;

        if new_cpt != 0 {
            let list_ptr_ref = unsafe { &*list_ptr };
            list_ptr = list_ptr_ref.next;
            continue;
        }

        if knum > 0 {
            let num_bytes = (knum as crate::types::size_t)
                .wrapping_mul(::core::mem::size_of::<*mut AudioKcontrol>() as crate::types::size_t);
            if num_bytes == 0 && knum > 0 {
                return HDF_FAILURE as i32;
            }
            let ptr = unsafe { OsalMemCalloc(num_bytes) };
            comp_ref.kcontrols = ptr as *mut *mut AudioKcontrol;
            if ptr.is_null() {
                return HDF_FAILURE as i32;
            }
        }

        match sapm_type {
            AUDIO_SAPM_ANALOG_SWITCH | AUDIO_SAPM_MIXER | AUDIO_SAPM_MIXER_NAMED_CTRL => {
                ret = crate::src_audio_sapm::AudioSapmNewMixerControls(
                    comp_ref as *const AudioSapmComponent,
                    audioCard,
                );
            }
            AUDIO_SAPM_MUX | AUDIO_SAPM_VIRT_MUX | AUDIO_SAPM_VALUE_MUX => {
                ret = crate::src_audio_sapm::AudioSapmNewMuxControls(
                    comp_ref,
                    audioCard,
                );
            }
            _ => {
                ret = HDF_SUCCESS as i32;
            }
        }

        if ret != HDF_SUCCESS as i32 {
            let kctrls = comp_ref.kcontrols;
            if !kctrls.is_null() {
                unsafe { OsalMemFree(kctrls as *mut ::core::ffi::c_void); }
            }
            return HDF_FAILURE as i32;
        }

        crate::src_audio_sapm::ReadInitComponentPowerStatus(comp_ref as *mut AudioSapmComponent);
        comp_ref.newCpt = 1u8;

        // DListInsert: insert comp_ref.dirty into &audioCard->sapmDirty
        unsafe {
            let dirty_head = &mut (*audioCard).sapmDirty as *mut DListHead;
            let entry = &mut comp_ref.dirty as *mut DListHead;
            (*entry).next = dirty_head;
            (*entry).prev = (*dirty_head).prev;
            (*(*dirty_head).prev).next = entry;
            (*dirty_head).prev = entry;
        }

        let list_ptr_ref2 = unsafe { &*list_ptr };
        list_ptr = list_ptr_ref2.next;
    }

    crate::src_audio_sapm::AudioSapmPowerComponents(audioCard);
    HDF_SUCCESS as i32
}

fn MixerUpdatePowerStatus(kcontrol: *const crate::types::AudioKcontrol, pathStatus: u32) -> i32 {
    if kcontrol.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let kc_ref = unsafe { &*kcontrol };
    if kc_ref.pri.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let audioCard = kc_ref.pri as *mut crate::types::AudioCard;
    let card = unsafe { &mut *audioCard };

    let head = &mut card.paths as *mut crate::types::DListHead;
    let offset = unsafe {
        std::ptr::addr_of!((*std::ptr::null::<crate::types::AudioSapmpath>()).list) as usize
    };
    let mut node = card.paths.next;
    loop {
        if node == head {
            break;
        }
        let path = unsafe {
            ((node as *mut u8).offset(-(offset as isize))) as *mut crate::types::AudioSapmpath
        };
        let path_ref = unsafe { &mut *path };
        if path_ref.kcontrol as *const crate::types::AudioKcontrol != kcontrol {
            let node_ref = unsafe { &*node };
            node = node_ref.next;
            continue;
        }
        if path_ref.sink.is_null() || path_ref.source.is_null() {
            return crate::types::HDF_FAILURE;
        }
        let sink_ref = unsafe { &*path_ref.sink };
        let sink_type = sink_ref.sapmType;
        if sink_type != crate::types::AUDIO_SAPM_MIXER
            && sink_type != crate::types::AUDIO_SAPM_MIXER_NAMED_CTRL
            && sink_type != crate::types::AUDIO_SAPM_PGA
            && sink_type != crate::types::AUDIO_SAPM_SPK
            && sink_type != crate::types::AUDIO_SAPM_ANALOG_SWITCH
        {
            return crate::types::HDF_DEV_ERR_NO_DEVICE;
        }

        path_ref.connect = pathStatus as u8;

        let source_ref = unsafe { &mut *path_ref.source };
        let entry1 = &mut source_ref.dirty as *mut crate::types::DListHead;
        let head_dirty = &mut card.sapmDirty as *mut crate::types::DListHead;
        unsafe {
            (*entry1).next = head_dirty;
            (*entry1).prev = (*head_dirty).prev;
            (*(*head_dirty).prev).next = entry1;
            (*head_dirty).prev = entry1;
        }
        let sink_ref = unsafe { &mut *path_ref.sink };
        let entry2 = &mut sink_ref.dirty as *mut crate::types::DListHead;
        unsafe {
            (*entry2).next = head_dirty;
            (*entry2).prev = (*head_dirty).prev;
            (*(*head_dirty).prev).next = entry2;
            (*head_dirty).prev = entry2;
        }

        break;
    }

    crate::src_audio_sapm::AudioSapmPowerComponents(audioCard);
    return crate::types::HDF_SUCCESS;
}

fn MuxUpdatePowerStatus(kcontrol: *const crate::types::AudioKcontrol, i: i32, enumKtl: *mut crate::types::AudioEnumKcontrol)-> i32 {
    if kcontrol.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let kc = unsafe { &*kcontrol };
    if kc.pri.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let audioCard = kc.pri as *mut crate::types::AudioCard;

    let list_offset = {
        let null_path: *const crate::types::AudioSapmpath = std::ptr::null();
        let list_field = unsafe { std::ptr::addr_of!((*null_path).list) as *const crate::types::DListHead };
        list_field as usize
    };

    let head_ptr: *mut crate::types::DListHead = unsafe { std::ptr::addr_of_mut!((*audioCard).paths) };
    let head_ptr_ref = unsafe { &*head_ptr };
    let mut current_node: *mut crate::types::DListHead = head_ptr_ref.next;

    while current_node != head_ptr {
        let path = unsafe {
            ((current_node as *mut u8).offset(-(list_offset as isize))) as *mut crate::types::AudioSapmpath
        };
        let path_mut = unsafe { &mut *path };

        if path_mut.kcontrol as *const crate::types::AudioKcontrol != kcontrol {
            current_node = path_mut.list.next;
            continue;
        }

        let enum_ktl = unsafe { &mut *enumKtl };
        let text_i = if enum_ktl.texts.is_null() {
            std::ptr::null()
        } else {
            let texts = unsafe { std::slice::from_raw_parts(enum_ktl.texts, (enum_ktl.max + 1) as usize) };
            texts[i as usize]
        };
        if path_mut.name.is_null() || text_i.is_null() {
            current_node = path_mut.list.next;
            continue;
        }

        let sink = path_mut.sink;
        let sink_ref = unsafe { &*sink };
        let sink_type = sink_ref.sapmType;
        if sink_type != crate::types::AUDIO_SAPM_MUX &&
           sink_type != crate::types::AUDIO_SAPM_VIRT_MUX &&
           sink_type != crate::types::AUDIO_SAPM_VALUE_MUX {
            return crate::types::HDF_DEV_ERR_NO_DEVICE;
        }

        let cmp = unsafe {
            libc::strcmp(path_mut.name as *const core::ffi::c_char, text_i)
        };
        if cmp == 0 {
            path_mut.connect = 1;
        } else {
            if path_mut.connect == 1 {
                path_mut.connect = 0;
            }
        }

        let source = path_mut.source;
        let source_ref = unsafe { &mut *source };
        let sink_ref = unsafe { &mut *sink };
        let source_dirty: *mut crate::types::DListHead = std::ptr::addr_of_mut!(source_ref.dirty);
        let sink_dirty: *mut crate::types::DListHead = std::ptr::addr_of_mut!(sink_ref.dirty);
        let sapm_dirty: *mut crate::types::DListHead = unsafe { std::ptr::addr_of_mut!((*audioCard).sapmDirty) };

        unsafe {
            (*source_dirty).next = sapm_dirty;
            (*source_dirty).prev = (*sapm_dirty).prev;
            (*(*sapm_dirty).prev).next = source_dirty;
            (*sapm_dirty).prev = source_dirty;
        }
        unsafe {
            (*sink_dirty).next = sapm_dirty;
            (*sink_dirty).prev = (*sapm_dirty).prev;
            (*(*sapm_dirty).prev).next = sink_dirty;
            (*sapm_dirty).prev = sink_dirty;
        }

        break;
    }

    crate::src_audio_sapm::AudioSapmPowerComponents(audioCard);
    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioCodecSapmGetCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemValue: *mut crate::types::AudioCtrlElemValue) -> i32 {
    let ret = unsafe { AudioCodecGetCtrlOps(kcontrol, elemValue) };
    if ret != HDF_SUCCESS {
        sapm_log!(crate::types::LOG_ERROR, b"HDF_AUDIO_SAPM\0".as_ptr() as *const i8, b"[%s][line:%d]: Audio codec sapm get control switch is fail!\0".as_ptr() as *const i8, b"AudioCodecSapmGetCtrlOps\0".as_ptr() as *const i8, 1306i32);
        return HDF_FAILURE;
    }
    HDF_SUCCESS
}

fn AudioSapmSetCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemValue: *const crate::types::AudioCtrlElemValue, value: *mut u32, pathStatus: *mut u32) -> i32 {
    if kcontrol.is_null()
        || elemValue.is_null()
        || value.is_null()
        || pathStatus.is_null()
    {
        sapm_log!(
            crate::types::LOG_ERROR,
            b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s][line:%d]: input params invalid.\0".as_ptr() as *const ::core::ffi::c_char,
            b"AudioSapmSetCtrlOps\0".as_ptr() as *const ::core::ffi::c_char,
            1321i32,
        );
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let kc_ref = unsafe { &*kcontrol };
    if kc_ref.privateValue == 0 {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let mixerCtrl_ptr = unsafe {
        let addr = kc_ref.privateValue as usize;
        addr as *mut crate::types::AudioMixerControl
    };
    let mixer_ref = unsafe { &*mixerCtrl_ptr };
    let elem_ref = unsafe { &*elemValue };

    let value_mut = unsafe { &mut *value };
    *value_mut = elem_ref.value[0];

    if *value_mut < mixer_ref.min || *value_mut > mixer_ref.max {
        sapm_log!(
            crate::types::LOG_ERROR,
            b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s][line:%d]: value is invalid.\0".as_ptr() as *const ::core::ffi::c_char,
            b"AudioSapmSetCtrlOps\0".as_ptr() as *const ::core::ffi::c_char,
            1328i32,
        );
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let path_status = unsafe { &mut *pathStatus };
    if *value_mut != 0 {
        *path_status = 1;
    } else {
        *path_status = 0;
    }

    if mixer_ref.invert != 0 {
        *value_mut = mixer_ref.max - *value_mut;
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioCodecSapmSetCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemValue: *const crate::types::AudioCtrlElemValue) -> i32 {
    let mut value: u32 = 0;
    let mut path_status: u32 = 0;
    let mut codec: *mut crate::types::CodecDevice = std::ptr::null_mut();
    let mut mixer_ctrl: *mut crate::types::AudioMixerControl = std::ptr::null_mut();

    // Validate kcontrol
    let kc_ref = if kcontrol.is_null() {
        sapm_log!(
            crate::types::LOG_ERROR,
            b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s][line:%d]: input params: kcontrol is NULL.\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"AudioCodecSapmSetCtrlOps\0".as_ptr() as *const ::core::ffi::c_char,
            1352i32,
        );
        return crate::types::HDF_ERR_INVALID_OBJECT;
    } else {
        unsafe { &*kcontrol }
    };
    if kc_ref.privateValue == 0 || kc_ref.pri.is_null() {
        sapm_log!(
            crate::types::LOG_ERROR,
            b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s][line:%d]: input params: kcontrol is NULL.\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"AudioCodecSapmSetCtrlOps\0".as_ptr() as *const ::core::ffi::c_char,
            1352i32,
        );
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    // Validate elemValue
    if elemValue.is_null() {
        sapm_log!(
            crate::types::LOG_ERROR,
            b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s][line:%d]: input params: elemValue is NULL.\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"AudioCodecSapmSetCtrlOps\0".as_ptr() as *const ::core::ffi::c_char,
            1356i32,
        );
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    // Cast privateValue to mixer control pointer
    mixer_ctrl = unsafe {
        (*kcontrol).privateValue as usize as *mut crate::types::AudioMixerControl
    };

    // Process the control set operation
    let value_ptr: *mut u32 = &mut value;
    let path_status_ptr: *mut u32 = &mut path_status;
    let ret = crate::src_audio_sapm::AudioSapmSetCtrlOps(
        kcontrol,
        elemValue,
        value_ptr,
        path_status_ptr,
    );
    if ret != crate::types::HDF_SUCCESS {
        sapm_log!(
            crate::types::LOG_ERROR,
            b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s][line:%d]: Audio sapm put control switch fail!\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"AudioCodecSapmSetCtrlOps\0".as_ptr() as *const ::core::ffi::c_char,
            1362i32,
        );
    }
    let _ = value; // suppress unused warning

    // Obtain the codec device
    codec = unsafe { AudioKcontrolGetCodec(kcontrol) };

    // Update power status
    if crate::src_audio_sapm::MixerUpdatePowerStatus(kcontrol, path_status)
        != crate::types::HDF_SUCCESS
    {
        sapm_log!(
            crate::types::LOG_ERROR,
            b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s][line:%d]: update power status is failure!\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"AudioCodecSapmSetCtrlOps\0".as_ptr() as *const ::core::ffi::c_char,
            1367i32,
        );
        return crate::types::HDF_FAILURE;
    }

    // Update mixer control value
    let mixer = unsafe { &mut *mixer_ctrl };
    let elem = unsafe { &*elemValue };
    mixer.value = elem.value[0];

    // Write the register update
    if unsafe { AudioCodecRegUpdate(codec, mixer_ctrl) } != crate::types::HDF_SUCCESS {
        sapm_log!(
            crate::types::LOG_ERROR,
            b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s][line:%d]: update reg bits fail!\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"AudioCodecSapmSetCtrlOps\0".as_ptr() as *const ::core::ffi::c_char,
            1373i32,
        );
        return crate::types::HDF_FAILURE;
    }

    crate::types::HDF_SUCCESS
}

fn AudioCodecCheckRegIsChange(enumCtrl: *mut crate::types::AudioEnumKcontrol, elemValue: *const crate::types::AudioCtrlElemValue, curValue: u32, change: *mut bool)-> i32 {
    if enumCtrl.is_null() || elemValue.is_null() || change.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let elem_ref = unsafe { &*elemValue };
    let ctrl_ref = unsafe { &mut *enumCtrl };
    let elem_value0 = elem_ref.value[0];
    let elem_value1 = elem_ref.value[1];
    let max = ctrl_ref.max;
    let values_ptr = ctrl_ref.values;
    let shift_left = ctrl_ref.shiftLeft;
    let shift_right = ctrl_ref.shiftRight;
    let mask_raw = ctrl_ref.mask;

    if elem_value0 > max {
        return crate::types::HDF_FAILURE;
    }
    let mut value: u32;
    let mut mask: u32;
    if !values_ptr.is_null() {
        let values = unsafe { std::slice::from_raw_parts(values_ptr, (max as usize).wrapping_add(1)) };
        value = values[elem_value0 as usize] << (shift_left as u32);
        mask = mask_raw << (shift_left as u32);
        if shift_left != shift_right {
            if elem_value1 > max {
                return crate::types::HDF_FAILURE;
            }
            value |= values[elem_value1 as usize] << (shift_right as u32);
            mask |= mask_raw << (shift_right as u32);
        }
    } else {
        value = elem_value0 << (shift_left as u32);
        mask = mask_raw << (shift_left as u32);
        if shift_left != shift_right {
            if elem_value1 > max {
                return crate::types::HDF_FAILURE;
            }
            value |= elem_value1 << (shift_right as u32);
            mask |= mask_raw << (shift_right as u32);
        }
    }
    let oldValue = curValue & mask;
    let new_change = oldValue != value;
    unsafe {
        *change = new_change;
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioCodecSapmSetEnumCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemValue: *const crate::types::AudioCtrlElemValue) -> i32 {
    let mut curValue: u32 = 0;
    let mut change: bool = false;
    let mut ret: i32;

    if kcontrol.is_null() {
        let domain = {
            ::core::ffi::CStr::from_bytes_with_nul(b"HDF_AUDIO_SAPM\0")
                .unwrap()
                .as_ptr()
        };
        let fmt = b"[%s][line:%d]: input params: kcontrol is NULL or elemValue is NULL\0".as_ptr() as *const ::core::ffi::c_char;
        let func = {
            ::core::ffi::CStr::from_bytes_with_nul(b"AudioCodecSapmSetEnumCtrlOps\0")
                .unwrap()
                .as_ptr()
        };
        sapm_log!(crate::types::LOG_ERROR, domain, fmt, func, 1438i32);
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let kc = unsafe { &*kcontrol };
    if kc.privateValue == 0 || elemValue.is_null() {
        let domain = {
            ::core::ffi::CStr::from_bytes_with_nul(b"HDF_AUDIO_SAPM\0")
                .unwrap()
                .as_ptr()
        };
        let fmt = b"[%s][line:%d]: input params: kcontrol is NULL or elemValue is NULL\0".as_ptr() as *const ::core::ffi::c_char;
        let func = {
            ::core::ffi::CStr::from_bytes_with_nul(b"AudioCodecSapmSetEnumCtrlOps\0")
                .unwrap()
                .as_ptr()
        };
        sapm_log!(crate::types::LOG_ERROR, domain, fmt, func, 1438i32);
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let codec = unsafe { AudioKcontrolGetCodec(kcontrol) };

    let enumCtrl =
        kc.privateValue as *mut crate::types::AudioEnumKcontrol;
    if enumCtrl.is_null() {
        let domain = {
            ::core::ffi::CStr::from_bytes_with_nul(b"HDF_AUDIO_SAPM\0")
                .unwrap()
                .as_ptr()
        };
        let fmt = {
            ::core::ffi::CStr::from_bytes_with_nul(
                b"[%s][line:%d]: privateValue is null\0",
            )
            .unwrap()
            .as_ptr()
        };
        let func = {
            ::core::ffi::CStr::from_bytes_with_nul(b"AudioCodecSapmSetEnumCtrlOps\0")
                .unwrap()
                .as_ptr()
        };
        sapm_log!(crate::types::LOG_ERROR, domain, fmt, func, 1445i32);
        return crate::types::HDF_FAILURE;
    }

    let en_ref = unsafe { &*enumCtrl };
    if unsafe { AudioCodecReadReg(codec, en_ref.reg, &mut curValue) }
        != crate::types::HDF_SUCCESS
    {
        let domain = {
            ::core::ffi::CStr::from_bytes_with_nul(b"HDF_AUDIO_SAPM\0")
                .unwrap()
                .as_ptr()
        };
        let fmt = {
            ::core::ffi::CStr::from_bytes_with_nul(
                b"[%s][line:%d]: Device read register is failure!\0",
            )
            .unwrap()
            .as_ptr()
        };
        let func = {
            ::core::ffi::CStr::from_bytes_with_nul(b"AudioCodecSapmSetEnumCtrlOps\0")
                .unwrap()
                .as_ptr()
        };
        sapm_log!(crate::types::LOG_ERROR, domain, fmt, func, 1450i32);
        return crate::types::HDF_FAILURE;
    }

    ret = crate::src_audio_sapm::AudioCodecCheckRegIsChange(
        enumCtrl,
        elemValue,
        curValue,
        &mut change,
    );
    if ret != crate::types::HDF_SUCCESS {
        let domain = {
            ::core::ffi::CStr::from_bytes_with_nul(b"HDF_AUDIO_SAPM\0")
                .unwrap()
                .as_ptr()
        };
        let fmt = {
            ::core::ffi::CStr::from_bytes_with_nul(
                b"[%s][line:%d]: AudioCodecCheckRegIsChange is failure!\0",
            )
            .unwrap()
            .as_ptr()
        };
        let func = {
            ::core::ffi::CStr::from_bytes_with_nul(b"AudioCodecSapmSetEnumCtrlOps\0")
                .unwrap()
                .as_ptr()
        };
        sapm_log!(crate::types::LOG_ERROR, domain, fmt, func, 1456i32);
        return crate::types::HDF_FAILURE;
    }

    if change {
        if crate::src_audio_sapm::MuxUpdatePowerStatus(
            kcontrol,
            unsafe { (*elemValue).value[0] } as i32,
            enumCtrl,
        ) != crate::types::HDF_SUCCESS
        {
            let domain = {
                ::core::ffi::CStr::from_bytes_with_nul(b"HDF_AUDIO_SAPM\0")
                    .unwrap()
                    .as_ptr()
            };
            let fmt = {
                ::core::ffi::CStr::from_bytes_with_nul(
                    b"[%s][line:%d]: update power status is failure!\0",
                )
                .unwrap()
                .as_ptr()
            };
            let func = {
                ::core::ffi::CStr::from_bytes_with_nul(b"AudioCodecSapmSetEnumCtrlOps\0")
                    .unwrap()
                    .as_ptr()
            };
            sapm_log!(crate::types::LOG_ERROR, domain, fmt, func, 1462i32);
            return crate::types::HDF_FAILURE;
        }

        ret = unsafe {
            AudioCodecMuxRegUpdate(
                codec,
                enumCtrl,
                (*elemValue).value.as_ptr(),
            )
        };
        if ret != crate::types::HDF_SUCCESS {
            let domain = {
                ::core::ffi::CStr::from_bytes_with_nul(b"HDF_AUDIO_SAPM\0")
                    .unwrap()
                    .as_ptr()
            };
            let fmt = {
                ::core::ffi::CStr::from_bytes_with_nul(
                    b"[%s][line:%d]: AudioCodecMuxRegUpdate is failure!\0",
                )
                .unwrap()
                .as_ptr()
            };
            let func = {
                ::core::ffi::CStr::from_bytes_with_nul(b"AudioCodecSapmSetEnumCtrlOps\0")
                    .unwrap()
                    .as_ptr()
            };
            sapm_log!(crate::types::LOG_ERROR, domain, fmt, func, 1468i32);
            return crate::types::HDF_FAILURE;
        }
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioCodecSapmGetEnumCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemValue: *mut crate::types::AudioCtrlElemValue) -> i32 {
    if unsafe { AudioCodecGetEnumCtrlOps(kcontrol, elemValue) } != crate::types::HDF_SUCCESS {
        return crate::types::HDF_FAILURE;
    }
    crate::types::HDF_SUCCESS
}

fn AudioSapmRefreshTime(audioCard: *mut crate::types::AudioCard, bRefresh: bool)-> i32 {
    if audioCard.is_null() {
        sapm_log!(crate::types::LOG_ERROR, b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char, b"[%s][line:%d]: input params is NULL.\0".as_ptr() as *const ::core::ffi::c_char, b"AudioSapmRefreshTime\0".as_ptr() as *const ::core::ffi::c_char, 1489i32);
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let card = unsafe { &mut *audioCard };
    if bRefresh {
        card.time = 0;
    }
    crate::types::HDF_SUCCESS
}

fn AudioSapmCheckTime(audioCard: *mut crate::types::AudioCard, timeoutStatus: *mut bool)-> i32 {
    if audioCard.is_null() || timeoutStatus.is_null() {
        sapm_log!(
            crate::types::LOG_ERROR,
            b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s][line:%d]: input params is NULL.\0".as_ptr() as *const ::core::ffi::c_char,
            b"AudioSapmCheckTime\0".as_ptr() as *const ::core::ffi::c_char,
            1504i32,
        );
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let card = unsafe { &mut *audioCard };
    let ret = crate::src_audio_sapm::AudioSapmRefreshTime(audioCard, false);
    if ret != crate::types::HDF_SUCCESS {
        sapm_log!(
            crate::types::LOG_ERROR,
            b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s][line:%d]: AudioSapmRefreshTime failed.\0".as_ptr() as *const ::core::ffi::c_char,
            b"AudioSapmCheckTime\0".as_ptr() as *const ::core::ffi::c_char,
            1510i32,
        );
        return crate::types::HDF_FAILURE;
    }
    let time = card.time;
    let timeout = time > 18u64;
    unsafe { *timeoutStatus = timeout; }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioSampPowerUp(card: *const crate::types::AudioCard) -> i32 {
    if card.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let mut upList = crate::types::DListHead {
        next: std::ptr::null_mut(),
        prev: std::ptr::null_mut(),
    };
    let up_ptr: *mut crate::types::DListHead = &mut upList;
    upList.next = up_ptr;
    upList.prev = up_ptr;
    let components_head = unsafe { std::ptr::addr_of!((*card).components) };
    let head_ptr = components_head as *mut crate::types::DListHead;
    let head_ref = unsafe { &*head_ptr };
    let mut pos = head_ref.next;
    while pos != head_ptr {
        let sapmComponent = unsafe {
                let offset = std::ptr::addr_of!((*std::ptr::null::<crate::types::AudioSapmComponent>()).list) as isize;
            (pos as *mut u8).offset(-offset) as *mut crate::types::AudioSapmComponent
        };
        if sapmComponent.is_null() {
            break;
        }
        let comp = unsafe { &*sapmComponent };
        if comp.power == 0 {
            crate::src_audio_sapm::AudioSapmPowerSeqInsert(sapmComponent, std::ptr::addr_of_mut!(upList), 1_i8);
        }
        let pos_ref = unsafe { &*pos };
        pos = pos_ref.next;
    }
    crate::src_audio_sapm::AudioSapmPowerUpSeqRun(&upList);
    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioSampSetPowerMonitor(card: *mut crate::types::AudioCard, powerMonitorState: bool) -> i32 {
    if card.is_null() {
        let func_name = b"AudioSampSetPowerMonitor\0";
        sapm_log!(
            crate::types::LOG_ERROR,
            b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s][line:%d]: input params is null.\0".as_ptr() as *const ::core::ffi::c_char,
            func_name.as_ptr() as *const ::core::ffi::c_char,
            1545i32,
        );
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let card_mut = unsafe { &mut *card };
    card_mut.sapmMonitorState = powerMonitorState;
    if !powerMonitorState {
        card_mut.sapmSleepState = false;
        card_mut.sapmStandbyState = false;
        card_mut.sapmStandbyStartTimeFlag = false;
        card_mut.sapmSleepStartTimeFlag = false;
    }
    crate::types::HDF_SUCCESS
}

fn AudioSapmEnterSleep(audioCard: *mut crate::types::AudioCard) {
    // HiLogPrint entry omitted (unresolved)
    // Initialize downList as an empty circular list (safe local init)
    let mut downList = crate::types::DListHead {
        next: std::ptr::null_mut(),
        prev: std::ptr::null_mut(),
    };
    downList.next = &mut downList;
    downList.prev = &mut downList;

    if audioCard.is_null() {
        // HiLogPrint error omitted
        return;
    }
    let card = unsafe { &mut *audioCard };

    let list_offset = unsafe {
        // Compute offset of `list` field in AudioSapmComponent
        std::ptr::addr_of!((*std::ptr::null::<crate::types::AudioSapmComponent>()).list) as usize
    };
    let components_list = std::ptr::addr_of_mut!(card.components);
    let components_ref = unsafe { &*components_list };
    let mut node = components_ref.next;
    while node != components_list {
        if node.is_null() {
            break;
        }
        let sapmComponent = unsafe {
            ((node as *mut u8).offset(-(list_offset as isize))) as *mut crate::types::AudioSapmComponent
        };
        if sapmComponent.is_null() {
            break;
        }
        let comp = unsafe { &*sapmComponent };
        let power = comp.power;
        if power == 1 {
            crate::src_audio_sapm::AudioSapmPowerSeqInsert(
                sapmComponent,
                &mut downList,
                0i8,
            );
        }
        let node_ref = unsafe { &*node };
        node = node_ref.next;
    }

    crate::src_audio_sapm::AudioSapmPowerDownSeqRun(&downList);
    card.sapmStandbyState = false;
    card.sapmSleepState = true;
}

fn AudioSapmEnterStandby(audioCard: *mut crate::types::AudioCard) -> bool {
    if audioCard.is_null() {
        let log_tag = b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char;
        let log_fmt = b"[%s][line:%d]: audioCard is null.\0".as_ptr() as *const ::core::ffi::c_char;
        let log_func = b"AudioSapmEnterStandby\0".as_ptr() as *const ::core::ffi::c_char;
        sapm_log!(crate::types::LOG_ERROR, log_tag, log_fmt, log_func, 1589i32);
        return false;
    }
    let card = unsafe { &mut *audioCard };

    let flag = card.sapmStandbyStartTimeFlag;
    if !flag {
        let ret = crate::src_audio_sapm::AudioSapmRefreshTime(audioCard, true);
        if ret != crate::types::HDF_SUCCESS {
            let log_tag = b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char;
            let log_fmt = b"[%s][line:%d]: AudioSapmRefreshTime failed.\0".as_ptr() as *const ::core::ffi::c_char;
            let log_func = b"AudioSapmEnterStandby\0".as_ptr() as *const ::core::ffi::c_char;
            sapm_log!(crate::types::LOG_ERROR, log_tag, log_fmt, log_func, 1595i32);
            return false;
        }
        card.sapmStandbyStartTimeFlag = true;
    }

    let standby_mode = card.standbyMode;
    if standby_mode != crate::types::AUDIO_SAPM_TURN_STANDBY_NOW {
        let mut timeout_status: bool = false;
        let ret = crate::src_audio_sapm::AudioSapmCheckTime(audioCard, &mut timeout_status);
        if ret != crate::types::HDF_SUCCESS {
            sapm_log!(
                crate::types::LOG_ERROR,
                b"HDF_AUDIO_SAPM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: AudioSapmCheckTime failed.\0".as_ptr() as *const ::core::ffi::c_char,
                b"AudioSapmEnterStandby\0".as_ptr() as *const ::core::ffi::c_char,
                1603i32,
            );
            return false;
        }
        if !timeout_status {
            return false;
        }
    }

    let standby_state = card.sapmStandbyState;
    if !standby_state {
        let list_offset = unsafe {
            let null_comp: *const crate::types::AudioSapmComponent = std::ptr::null();
            (std::ptr::addr_of!((*null_comp).list)) as *const crate::types::DListHead as usize
        };

        let mut current_component: *mut crate::types::AudioSapmComponent = {
            let next = card.components.next;
            ((next as usize) - list_offset) as *mut crate::types::AudioSapmComponent
        };

        loop {
            let comp = unsafe { &*current_component };
            let is_end = (std::ptr::addr_of!(comp.list)) as *const _
                == std::ptr::addr_of!(card.components) as *const _;
            if is_end {
                break;
            }

            let power_clock_op = comp.PowerClockOp;
            if let Some(op) = power_clock_op {
                unsafe { op(current_component); }
            }

            let next = comp.list.next;
            current_component = {
                ((next as usize) - list_offset) as *mut crate::types::AudioSapmComponent
            };
        }

        card.sapmStandbyState = true;
    }

    true
}

fn AudioSapmTimerCallback(audioCard: *mut crate::types::AudioCard) {
    if audioCard.is_null() {
        return;
    }
    let card = unsafe { &mut *audioCard };
    let sleep_state = card.sapmSleepState;
    if sleep_state {
        return;
    }
    let monitor_state = card.sapmMonitorState;
    if !monitor_state {
        return;
    }
    let standby_entry = crate::src_audio_sapm::AudioSapmEnterStandby(audioCard);
    if !standby_entry {
        return;
    }
    let sleep_flag = card.sapmSleepStartTimeFlag;
    if !sleep_flag {
        if crate::src_audio_sapm::AudioSapmRefreshTime(audioCard, true) != crate::types::HDF_SUCCESS {
            return;
        }
        card.sapmSleepStartTimeFlag = true;
    }
    let mut timeout_status: bool = false;
    if crate::src_audio_sapm::AudioSapmCheckTime(audioCard, &mut timeout_status) != crate::types::HDF_SUCCESS {
        return;
    }
    if !timeout_status {
        return;
    }
    crate::src_audio_sapm::AudioSapmEnterSleep(audioCard);
}
