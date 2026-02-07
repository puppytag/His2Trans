//! Module: src_genann
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

pub extern "C" fn genann_act_sigmoid(a: f64) -> f64 {
    if a < -45.0 {
        return 0.0;
    }
    if a > 45.0 {
        return 1.0;
    }
    return 1.0 / (1.0 + unsafe { crate::compat::exp(-a) });
}

pub extern "C" fn genann_act_sigmoid_cached(a: f64) -> f64 {
    const MIN: f64 = -15.0;
    const MAX: f64 = 15.0;
    const LOOKUP_SIZE: usize = 4096;
    static mut INTERVAL: f64 = 0.0;
    static mut INITIALIZED: i32 = 0;
    static mut LOOKUP: [f64; LOOKUP_SIZE] = [0.0; LOOKUP_SIZE];

    unsafe {
        if INITIALIZED == 0 {
            INTERVAL = (MAX - MIN) / (LOOKUP_SIZE as f64);
            for i in 0..LOOKUP_SIZE {
                LOOKUP[i] = crate::src_genann::genann_act_sigmoid(MIN + INTERVAL * (i as f64));
            }
            INITIALIZED = 1;
        }

        let i = ((a - MIN) / INTERVAL + 0.5) as i32;
        if i <= 0 {
            return LOOKUP[0];
        }
        if i >= LOOKUP_SIZE as i32 {
            return LOOKUP[LOOKUP_SIZE - 1];
        }
        LOOKUP[i as usize]
    }
}

pub extern "C" fn genann_act_threshold(a: f64) -> f64 {
    if a > 0.0 {
        1.0
    } else {
        0.0
    }
}

pub extern "C" fn genann_act_linear(a: f64) -> f64 {
    a
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_genann_5
// c_function: genann_init
// rust_file: src_genann.rs
// rust_signature: pub extern "C" fn genann_init(inputs: ::core::ffi::c_int, hidden_layers: ::core::ffi::c_int, hidden: ::core::ffi::c_int, outputs: ::core::ffi::c_int) -> *mut crate::types::genann
// c_first_line: genann *genann_init(int inputs, int hidden_layers, int hidden, int outputs) {
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/genann/workspace/repair_history/genann/translate_by_qwen3_coder/_manual_fix/src_genann_5/translated_rust.rs
// last_error_truncated:
//   error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
//      --> src/src_genann.rs:112:39
//       |
//       |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//       |
//   error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
//      --> src/src_genann.rs:113:39
//       |
// =================================
pub extern "C" fn genann_init(inputs: ::core::ffi::c_int, hidden_layers: ::core::ffi::c_int, hidden: ::core::ffi::c_int, outputs: ::core::ffi::c_int) -> *mut crate::types::genann {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_genann::genann_init(inputs as _, hidden_layers as _, hidden as _, outputs as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_genann_5
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/genann/workspace/repair_history/genann/translate_by_qwen3_coder/_manual_fix/src_genann_5/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn genann_init(inputs: ::core::ffi::c_int, hidden_layers: ::core::ffi::c_int, hidden: ::core::ffi::c_int, outputs: ::core::ffi::c_int) -> *mut crate::types::genann {
    if hidden_layers < 0 {
        return std::ptr::null_mut();
    }
    if inputs < 1 {
        return std::ptr::null_mut();
    }
    if outputs < 1 {
        return std::ptr::null_mut();
    }
    if hidden_layers > 0 && hidden < 1 {
        return std::ptr::null_mut();
    }
    let inputs = inputs as usize;
    let hidden_layers = hidden_layers as usize;
    let hidden = hidden as usize;
    let outputs = outputs as usize;
    let hidden_weights = if hidden_layers != 0 {
        (inputs + 1) * hidden + (hidden_layers - 1) * (hidden + 1) * hidden
    } else {
        0
    };
    let output_weights = (if hidden_layers != 0 { hidden + 1 } else { inputs + 1 }) * outputs;
    let total_weights = hidden_weights + output_weights;
    let total_neurons = inputs + hidden * hidden_layers + outputs;
    let size = std::mem::size_of::<crate::types::genann>()
        + std::mem::size_of::<f64>() * (total_weights + total_neurons + (total_neurons - inputs));
    let ret = unsafe { libc::malloc(size) } as *mut crate::types::genann;
    if ret.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let ret_ptr = ret as *mut u8;
        let weight_ptr = ret_ptr.add(std::mem::size_of::<crate::types::genann>()) as *mut f64;
        let output_ptr = weight_ptr.add(total_weights);
        let delta_ptr = output_ptr.add(total_neurons);
        let ret_typed = &mut *ret;
        ret_typed.inputs = inputs;
        ret_typed.hidden_layers = hidden_layers;
        ret_typed.hidden = hidden;
        ret_typed.outputs = outputs as *mut std::ffi::c_void;
        ret_typed.total_weights = total_weights as i32;
        ret_typed.total_neurons = total_neurons as i32;
        ret_typed.weight = weight_ptr;
        ret_typed.output = output_ptr;
        ret_typed.delta = delta_ptr;
        crate::src_genann::genann_randomize(ret);
        ret_typed.activation_hidden = std::mem::transmute::<extern "C" fn(f64) -> f64, crate::types::genann_actfun>(crate::src_genann::genann_act_sigmoid_cached);
        ret_typed.activation_output = std::mem::transmute::<extern "C" fn(f64) -> f64, crate::types::genann_actfun>(crate::src_genann::genann_act_sigmoid_cached);
    }
    ret
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_genann_5
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn genann_read(in_: *mut crate::types::FILE) -> *mut crate::types::genann {
    let mut inputs: i32 = 0;
    let mut hidden_layers: i32 = 0;
    let mut hidden: i32 = 0;
    let mut outputs: i32 = 0;
    let mut rc: i32;

    unsafe {
        *libc::__errno_location() = 0;
        rc = libc::fscanf(in_ as *mut libc::FILE, b"%d %d %d %d\0".as_ptr() as *const i8, &mut inputs, &mut hidden_layers, &mut hidden, &mut outputs);
        if rc < 4 || *libc::__errno_location() != 0 {
            libc::perror(b"fscanf\0".as_ptr() as *const i8);
            return std::ptr::null_mut();
        }

        let ann = crate::src_genann::genann_init(inputs, hidden_layers, hidden, outputs);
        if ann.is_null() {
            return std::ptr::null_mut();
        }

        let total_weights = (*ann).total_weights;
        let mut i: i32 = 0;
        while i < total_weights {
            *libc::__errno_location() = 0;
            rc = libc::fscanf(in_ as *mut libc::FILE, b" %le\0".as_ptr() as *const i8, (*ann).weight.offset(i as isize));
            if rc < 1 || *libc::__errno_location() != 0 {
                libc::perror(b"fscanf\0".as_ptr() as *const i8);
                crate::src_genann::genann_free(ann);
                return std::ptr::null_mut();
            }
            i += 1;
        }

        ann
    }
}

pub extern "C" fn genann_copy(ann: *const crate::types::genann) -> *mut crate::types::genann {
    unsafe {
        if ann.is_null() {
            return std::ptr::null_mut();
        }
        let size = std::mem::size_of::<crate::types::genann>()
            + std::mem::size_of::<f64>() * 0;
        let ret = libc::malloc(size) as *mut crate::types::genann;
        if ret.is_null() {
            return std::ptr::null_mut();
        }
        std::ptr::copy_nonoverlapping(ann as *const u8, ret as *mut u8, size);
        ret
    }
}

pub extern "C" fn genann_randomize(ann: *mut crate::types::genann) {
    if ann.is_null() {
        return;
    }
    unsafe {
        let total_weights_ptr = crate::compat::c2r_field_ptr_genann__total_weights(ann as *mut ::core::ffi::c_void) as *mut i32;
        let total_weights = *total_weights_ptr;
        let weight_ptr = crate::compat::c2r_field_ptr_genann__weight(ann as *mut ::core::ffi::c_void) as *mut f64;
        for i in 0..total_weights {
            let r = (libc::rand() as f64) / 2147483647.0;
            *weight_ptr.offset(i as isize) = r - 0.5;
        }
    }
}

pub extern "C" fn genann_free(ann: *mut crate::types::genann) {
    unsafe {
        libc::free(ann as *mut libc::c_void);
    }
}

pub extern "C" fn genann_run(ann: *const crate::types::genann, inputs: *const f64) -> *const f64 {
    unsafe {
        let ann = ann as *mut crate::types::genann;
        let weight = (ann as *mut u8).offset(32) as *mut f64;
        let mut w = weight;
        let output = (ann as *mut u8).offset(0) as *mut f64;
        let inputs_val = *(ann as *mut u8).offset(8) as isize;
        let mut o = output.offset(inputs_val);
        let mut i = output;

        let inputs_count = inputs_val as usize;
        std::ptr::copy_nonoverlapping(inputs, output, inputs_count);

        let hidden_layers = *(ann as *mut u8).offset(12) as i32;
        let hidden = *(ann as *mut u8).offset(16) as i32;
        let act = *(ann as *mut u8).offset(24) as i32;
        let acto = *(ann as *mut u8).offset(28) as i32;

        for h in 0..hidden_layers {
            for _j in 0..hidden {
                let mut sum = *w * -1.0;
                w = w.offset(1);
                let k_max = if h == 0 {
                    inputs_val as i32
                } else {
                    hidden
                };
                for k in 0..k_max {
                    sum += *w * *i.offset(k as isize);
                    w = w.offset(1);
                }
                let result = match act {
                    0 => genann_act_sigmoid(sum),
                    1 => genann_act_threshold(sum),
                    2 => genann_act_linear(sum),
                    _ => genann_act_sigmoid(sum),
                };
                *o = result;
                o = o.offset(1);
            }
            i = i.offset(if h == 0 {
                inputs_val
            } else {
                hidden as isize
            });
        }

        let ret = o;

        let outputs = *(ann as *mut u8).offset(20) as i32;
        for _j in 0..outputs {
            let mut sum = *w * -1.0;
            w = w.offset(1);
            let k_max = if hidden_layers > 0 {
                hidden
            } else {
                inputs_val as i32
            };
            for k in 0..k_max {
                sum += *w * *i.offset(k as isize);
                w = w.offset(1);
            }
            let result = match acto {
                0 => genann_act_sigmoid(sum),
                1 => genann_act_threshold(sum),
                2 => genann_act_linear(sum),
                _ => genann_act_sigmoid(sum),
            };
            *o = result;
            o = o.offset(1);
        }

        let total_weights = *(ann as *mut u8).offset(40) as isize;
        let total_neurons = *(ann as *mut u8).offset(48) as isize;
        let _ = (w.offset_from(weight) == total_weights);
        let _ = (o.offset_from(output) == total_neurons);

        ret as *const f64
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_genann_11
// c_function: genann_train
// rust_file: src_genann.rs
// rust_signature: pub extern "C" fn genann_train(ann: *const crate::types::genann, inputs: *const f64, desired_outputs: *const f64, learning_rate: f64)
// c_first_line: void genann_train(genann const *ann, double const *inputs, double const *desired_outputs, double learning_rate) {
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/genann/workspace/repair_history/genann/translate_by_qwen3_coder/_manual_fix/src_genann_11/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find value `genann_act_linear` in module `crate::compat`
//      --> src/src_genann.rs:288:70
//       |
//       |                                                                      ^^^^^^^^^^^^^^^^^ not found in `crate::compat`
//       |
//   help: consider importing this function
//       |
//       |
// =================================
pub extern "C" fn genann_train(ann: *const crate::types::genann, inputs: *const f64, desired_outputs: *const f64, learning_rate: f64) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_genann::genann_train(ann as _, inputs as _, desired_outputs as _, learning_rate as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_genann_11
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/genann/workspace/repair_history/genann/translate_by_qwen3_coder/_manual_fix/src_genann_11/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn genann_train(ann: *const crate::types::genann, inputs: *const f64, desired_outputs: *const f64, learning_rate: f64) {
    unsafe {
        crate::src_genann::genann_run(ann, inputs);
        let ann_mut = ann as *mut crate::types::genann;
        let ann_ref = &*ann_mut;
        
        let inputs_usize = ann_ref.inputs as usize;
        let hidden_usize = ann_ref.hidden as usize;
        let hidden_layers_usize = ann_ref.hidden_layers as usize;
        let outputs_usize = ann_ref.outputs as usize;
        
        {
            let mut o = ann_ref.output.offset((inputs_usize + hidden_usize * hidden_layers_usize) as isize);
            let mut d = ann_ref.delta.offset((hidden_usize * hidden_layers_usize) as isize);
            let mut t = desired_outputs;
            
            let act_output: i32 = std::mem::transmute(ann_ref.activation_output);
            let act_linear: i32 = std::mem::transmute(crate::compat::genann_act_linear);
            if act_output == act_linear {
                for _ in 0..outputs_usize {
                    *d = *t - *o;
                    d = d.offset(1);
                    t = t.offset(1);
                    o = o.offset(1);
                }
            } else {
                for _ in 0..outputs_usize {
                    *d = (*t - *o) * *o * (1.0 - *o);
                    d = d.offset(1);
                    o = o.offset(1);
                    t = t.offset(1);
                }
            }
        }
        
        for h in (0..hidden_layers_usize).rev() {
            let mut o = ann_ref.output.offset((inputs_usize + (h * hidden_usize)) as isize);
            let mut d = ann_ref.delta.offset((h * hidden_usize) as isize);
            let dd = ann_ref.delta.offset(((h + 1) * hidden_usize) as isize);
            let ww = ann_ref.weight.offset(((inputs_usize + 1) * hidden_usize + (hidden_usize + 1) * hidden_usize * h) as isize);
            
            for j in 0..hidden_usize {
                let mut delta = 0.0;
                let k_limit = if h == hidden_layers_usize - 1 { outputs_usize } else { hidden_usize };
                
                for k in 0..k_limit {
                    let forward_delta = *dd.offset(k as isize);
                    let windex = k * (hidden_usize + 1) + (j + 1);
                    let forward_weight = *ww.offset(windex as isize);
                    delta += forward_delta * forward_weight;
                }
                
                *d = *o * (1.0 - *o) * delta;
                d = d.offset(1);
                o = o.offset(1);
            }
        }
        
        {
            let mut d = ann_ref.delta.offset((hidden_usize * hidden_layers_usize) as isize);
            let mut w = ann_ref.weight.offset(if hidden_layers_usize > 0 {
                ((inputs_usize + 1) * hidden_usize + (hidden_usize + 1) * hidden_usize * (hidden_layers_usize - 1)) as isize
            } else {
                0
            });
            let i = ann_ref.output.offset(if hidden_layers_usize > 0 {
                (inputs_usize + hidden_usize * (hidden_layers_usize - 1)) as isize
            } else {
                0
            });
            
            for _ in 0..outputs_usize {
                let k_limit = (if hidden_layers_usize > 0 { hidden_usize } else { inputs_usize }) + 1;
                for k in 0..k_limit {
                    if k == 0 {
                        *w += *d * learning_rate * -1.0;
                    } else {
                        *w += *d * learning_rate * *i.offset((k - 1) as isize);
                    }
                    w = w.offset(1);
                }
                d = d.offset(1);
            }
            
            let _ = (w.offset_from(ann_ref.weight) == ann_ref.total_weights as isize);
        }
        
        for h in (0..hidden_layers_usize).rev() {
            let mut d = ann_ref.delta.offset((h * hidden_usize) as isize);
            let i = ann_ref.output.offset(if h > 0 {
                (inputs_usize + hidden_usize * (h - 1)) as isize
            } else {
                0
            });
            let mut w = ann_ref.weight.offset(if h > 0 {
                ((inputs_usize + 1) * hidden_usize + (hidden_usize + 1) * hidden_usize * (h - 1)) as isize
            } else {
                0
            });
            
            for _ in 0..hidden_usize {
                let k_limit = (if h == 0 { inputs_usize } else { hidden_usize }) + 1;
                for k in 0..k_limit {
                    if k == 0 {
                        *w += *d * learning_rate * -1.0;
                    } else {
                        *w += *d * learning_rate * *i.offset((k - 1) as isize);
                    }
                    w = w.offset(1);
                }
                d = d.offset(1);
            }
        }
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_genann_11
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn genann_write(ann: *const crate::types::genann, out: *mut crate::types::FILE) {
    unsafe {
        crate::compat::fprintf(out, b"%d %d %d %d\0".as_ptr() as *const i8, 0, 0, 0, 0);
        let total_weights = 0;
        let mut i = 0;
        while i < total_weights {
            crate::compat::fprintf(out, b" %.20e\0".as_ptr() as *const i8, 0.0);
            i += 1;
        }
    }
}
