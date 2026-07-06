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
use core::sync::atomic::{AtomicU64, Ordering};

pub extern "C" fn genann_act_sigmoid(a: f64) -> f64 {
    if a < -45.0 {
        return 0.0;
    }
    if a > 45.0 {
        return 1.0;
    }
    1.0 / (1.0 + (-a).exp())
}

pub extern "C" fn genann_act_sigmoid_cached(a: f64) -> f64 {
    use std::sync::OnceLock;
    const MIN: f64 = -15.0;
    const MAX: f64 = 15.0;
    const LOOKUP_SIZE: usize = 4096;

    struct Cache {
        interval: f64,
        lookup: [f64; LOOKUP_SIZE],
    }

    fn init_cache() -> Cache {
        let interval = (MAX - MIN) / LOOKUP_SIZE as f64;
        let mut lookup = [0.0; LOOKUP_SIZE];
        for i in 0..LOOKUP_SIZE {
            lookup[i] = genann_act_sigmoid(MIN + interval * i as f64);
        }
        Cache { interval, lookup }
    }

    static CACHE: OnceLock<Cache> = OnceLock::new();
    let cache = CACHE.get_or_init(init_cache);

    let i = ((a - MIN) / cache.interval + 0.5) as i32;

    if i <= 0 {
        cache.lookup[0]
    } else if i >= LOOKUP_SIZE as i32 {
        cache.lookup[LOOKUP_SIZE - 1]
    } else {
        cache.lookup[i as usize]
    }
}

pub extern "C" fn genann_act_threshold(a: f64) -> f64 {
    if a > 0.0 { 1.0 } else { 0.0 }
}

pub extern "C" fn genann_act_linear(a: f64) -> f64 {
    a
}

pub fn genann_init(inputs: ::core::ffi::c_int, hidden_layers: ::core::ffi::c_int, hidden: ::core::ffi::c_int, outputs: ::core::ffi::c_int) -> Option<Box<crate::types::genann>> {
    if hidden_layers < 0 { return None; }
    if inputs < 1 { return None; }
    if outputs < 1 { return None; }
    if hidden_layers > 0 && hidden < 1 { return None; }

    let hidden_weights = if hidden_layers != 0 {
        (inputs + 1) * hidden + (hidden_layers - 1) * (hidden + 1) * hidden
    } else {
        0
    };
    let output_weights = if hidden_layers != 0 {
        (hidden + 1) * outputs
    } else {
        (inputs + 1) * outputs
    };
    let total_weights = hidden_weights + output_weights;
    let total_neurons = inputs + hidden * hidden_layers + outputs;

    let weight = vec![0.0; total_weights as usize];
    let output = vec![0.0; total_neurons as usize];
    let delta_len = (total_neurons - inputs) as usize;
    let delta = vec![0.0; delta_len];

    let mut ann = Box::new(crate::types::genann {
        inputs,
        hidden_layers,
        hidden,
        outputs,
        activation_hidden: crate::src_genann::genann_act_sigmoid_cached,
        activation_output: crate::src_genann::genann_act_sigmoid_cached,
        total_weights,
        total_neurons,
        weight,
        output,
        delta,
    });

    crate::src_genann::genann_randomize(&mut *ann);

    Some(ann)
}

pub fn genann_read(in_: *mut crate::types::FILE) -> Option<Box<crate::types::genann>> {
    let mut inputs: i32 = 0;
    let mut hidden_layers: i32 = 0;
    let mut hidden: i32 = 0;
    let mut outputs: i32 = 0;

    let header_fmt = b"%d %d %d %d\0".as_ptr() as *const i8;
    let rc = unsafe {
        *libc::__errno_location() = 0;
        fscanf(
            in_,
            header_fmt,
            &mut inputs as *mut i32,
            &mut hidden_layers as *mut i32,
            &mut hidden as *mut i32,
            &mut outputs as *mut i32,
        )
    };
    if rc < 4 || unsafe { *libc::__errno_location() } != 0 {
        unsafe {
            perror(b"fscanf\0".as_ptr() as *const i8);
        }
        return None;
    }

    let mut ann = crate::src_genann::genann_init(inputs, hidden_layers, hidden, outputs)?;

    let total_weights = ann.total_weights as usize;
    let weight_fmt = b" %le\0".as_ptr() as *const i8;

    for i in 0..total_weights {
        let rc = unsafe {
            *libc::__errno_location() = 0;
            fscanf(in_, weight_fmt, &mut ann.weight[i] as *mut f64)
        };
        if rc < 1 || unsafe { *libc::__errno_location() } != 0 {
            unsafe {
                perror(b"fscanf\0".as_ptr() as *const i8);
            }
            return None;
        }
    }

    Some(ann)
}

pub fn genann_copy(ann: &crate::types::genann) -> Box<crate::types::genann> {
    Box::new(ann.clone())
}

fn genann_rand_double() -> f64 {
    static STATE: AtomicU64 = AtomicU64::new(1);
    let old = STATE.load(Ordering::Relaxed);
    let new = old.wrapping_mul(1664525).wrapping_add(1013904223);
    STATE.store(new, Ordering::Relaxed);
    (new & 0x7FFFFFFF) as f64 / 0x7FFFFFFF as f64
}

pub fn genann_randomize(ann: &mut crate::types::genann) {
    let total_weights = ann.total_weights as usize;
    if total_weights == 0 {
        return;
    }
    let weight_slice = ann.weight.as_mut_slice();
    for w in weight_slice.iter_mut() {
        *w = genann_rand_double() - 0.5;
    }
}


pub fn genann_run<'a>(ann: &'a mut crate::types::genann, inputs: &[f64]) -> &'a [f64] {
    let inputs_count = ann.inputs as usize;
    let hidden_layers = ann.hidden_layers;
    let hidden = ann.hidden as usize;
    let outputs = ann.outputs as usize;
    let act: crate::types::genann_actfun = ann.activation_hidden;
    let acto: crate::types::genann_actfun = ann.activation_output;
    let total_weights = ann.total_weights as usize;
    let total_neurons = ann.total_neurons as usize;

    ann.output[..inputs_count].copy_from_slice(inputs);

    let mut w_idx: usize = 0;
    let mut o_idx: usize = inputs_count;
    let mut i_idx: usize = 0;

    // Hidden layers
    for h in 0..hidden_layers {
        for _j in 0..hidden {
            let mut sum = ann.weight[w_idx] * -1.0;
            w_idx += 1;
            let limit: usize = if h == 0 { inputs_count } else { hidden };
            for k in 0..limit {
                sum += ann.weight[w_idx] * ann.output[i_idx + k];
                w_idx += 1;
            }
            ann.output[o_idx] = act(sum);
            o_idx += 1;
        }
        i_idx += if h == 0 { inputs_count } else { hidden };
    }

    let ret_offset = o_idx;

    // Output layer
    for _j in 0..outputs {
        let mut sum = ann.weight[w_idx] * -1.0;
        w_idx += 1;
        let limit: usize =
            if hidden_layers != 0 { hidden } else { inputs_count };
        for k in 0..limit {
            sum += ann.weight[w_idx] * ann.output[i_idx + k];
            w_idx += 1;
        }
        ann.output[o_idx] = acto(sum);
        o_idx += 1;
    }

    assert!(w_idx == total_weights);
    assert!(o_idx == total_neurons);

    &ann.output[ret_offset..]
}

pub fn genann_train(ann: &mut crate::types::genann, inputs: &[f64], desired_outputs: &[f64], learning_rate: f64) {
    let _ = crate::src_genann::genann_run(ann, inputs);

    let inputs_count = ann.inputs as usize;
    let hidden_layers = ann.hidden_layers;
    let hidden = ann.hidden as usize;
    let outputs = ann.outputs as usize;
    let total_weights = ann.total_weights as usize;
    let total_neurons = ann.total_neurons as usize;

    let desired = desired_outputs;
    let desired_len = desired.len();
    assert!(desired_len == outputs, "desired_outputs length must equal outputs");

    let is_linear = ann.activation_output == crate::src_genann::genann_act_linear;

    // 1. Compute delta for output layer
    let o_start_o = inputs_count + hidden * hidden_layers as usize;
    let d_start_o = hidden * hidden_layers as usize;
    for (out_idx, &target) in desired.iter().enumerate() {
        let o_val = ann.output[o_start_o + out_idx];
        let d_val = if is_linear {
            target - o_val
        } else {
            (target - o_val) * o_val * (1.0 - o_val)
        };
        ann.delta[d_start_o + out_idx] = d_val;
    }

    // 2. Compute delta for hidden layers
    for h in (0..hidden_layers).rev() {
        let h_usize = h as usize;
        let o_start = inputs_count + h_usize * hidden;
        let d_start = h_usize * hidden;
        let dd_start = (h_usize + 1) * hidden;
        let ww_start = (inputs_count + 1) * hidden + (hidden + 1) * hidden * h_usize;
        let k_bound = if h == hidden_layers - 1 { outputs } else { hidden };
        for j in 0..hidden {
            let mut sum = 0.0;
            for k in 0..k_bound {
                let forward_delta = ann.delta[dd_start + k];
                let windex = k * (hidden + 1) + (j + 1);
                let forward_weight = ann.weight[ww_start + windex];
                sum += forward_delta * forward_weight;
            }
            ann.delta[d_start + j] = ann.output[o_start + j] * (1.0 - ann.output[o_start + j]) * sum;
        }
    }

    // 3. Update weights for output layer
    {
        let d_start = hidden * hidden_layers as usize;
        let (w_start, i_start) = if hidden_layers != 0 {
            (
                (inputs_count + 1) * hidden + (hidden + 1) * hidden * (hidden_layers as usize - 1),
                inputs_count + hidden * (hidden_layers as usize - 1),
            )
        } else {
            (0, 0)
        };
        let k_bound = if hidden_layers != 0 { hidden } else { inputs_count };
        let bound_plus1 = k_bound + 1;
        let mut w_idx = w_start;
        for out_idx in 0..outputs {
            for k in 0..bound_plus1 {
                let delta_val = ann.delta[d_start + out_idx];
                if k == 0 {
                    ann.weight[w_idx] += delta_val * learning_rate * (-1.0);
                } else {
                    ann.weight[w_idx] += delta_val * learning_rate * ann.output[i_start + k - 1];
                }
                w_idx += 1;
            }
        }
        assert!(w_idx == total_weights, "w - ann->weight == ann->total_weights");
    }

    // 4. Update weights for hidden layers
    for h in (0..hidden_layers).rev() {
        let h_usize = h as usize;
        let d_start = h_usize * hidden;
        let (w_start, i_start) = if h != 0 {
            (
                (inputs_count + 1) * hidden + (hidden + 1) * hidden * (h_usize - 1),
                inputs_count + hidden * (h_usize - 1),
            )
        } else {
            (0, 0)
        };
        let k_bound = if h == 0 { inputs_count } else { hidden };
        let bound_plus1 = k_bound + 1;
        let mut w_idx = w_start;
        for neuron_idx in 0..hidden {
            for k in 0..bound_plus1 {
                let delta_val = ann.delta[d_start + neuron_idx];
                if k == 0 {
                    ann.weight[w_idx] += delta_val * learning_rate * (-1.0);
                } else {
                    ann.weight[w_idx] += delta_val * learning_rate * ann.output[i_start + k - 1];
                }
                w_idx += 1;
            }
        }
    }
}

pub fn genann_write(ann: &crate::types::genann, out: *mut crate::types::FILE) {
    let inputs = ann.inputs;
    let hidden_layers = ann.hidden_layers;
    let hidden = ann.hidden;
    let outputs = ann.outputs;
    let total_weights = ann.total_weights;

    let header_fmt = b"%d %d %d %d\0".as_ptr() as *const ::core::ffi::c_char;
    unsafe {
        let _ = fprintf(out, header_fmt, inputs, hidden_layers, hidden, outputs);
    }

    let weight_fmt = b" %.20e\0".as_ptr() as *const ::core::ffi::c_char;
    for i in 0..total_weights as usize {
        unsafe {
            let _ = fprintf(out, weight_fmt, ann.weight[i]);
        }
    }
}
