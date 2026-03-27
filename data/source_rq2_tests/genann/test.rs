// Rust translation of genann test.c
// GENANN - Minimal C Artificial Neural Network
// Copyright (c) 2015-2018 Lewis Van Winkle
// Translated from C to Rust

use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{Read, Write as IoWrite};
use std::os::raw::{c_double, c_int, c_void};
use std::ptr;

// genann structure (opaque)
#[repr(C)]
pub struct genann {
    pub inputs: c_int,
    pub hidden_layers: c_int,
    pub hidden: c_int,
    pub outputs: c_int,
    pub activation_hidden: Option<unsafe extern "C" fn(*const genann, c_double) -> c_double>,
    pub activation_output: Option<unsafe extern "C" fn(*const genann, c_double) -> c_double>,
    pub total_weights: c_int,
    pub total_neurons: c_int,
    pub weight: *mut c_double,
    pub output: *mut c_double,
    pub delta: *mut c_double,
}

// External declarations for genann library functions
extern "C" {
    fn genann_init(inputs: c_int, hidden_layers: c_int, hidden: c_int, outputs: c_int) -> *mut genann;
    fn genann_free(ann: *mut genann);
    fn genann_copy(ann: *const genann) -> *mut genann;
    fn genann_run(ann: *const genann, inputs: *const c_double) -> *const c_double;
    fn genann_train(ann: *const genann, inputs: *const c_double, desired_outputs: *const c_double, learning_rate: c_double);
    fn genann_randomize(ann: *mut genann);
    fn genann_read(file: *mut libc::FILE) -> *mut genann;
    fn genann_write(ann: *const genann, file: *mut libc::FILE);
    fn genann_act_sigmoid(ann: *const genann, a: c_double) -> c_double;
    fn genann_act_sigmoid_cached(ann: *const genann, a: c_double) -> c_double;
    fn genann_act_threshold(ann: *const genann, a: c_double) -> c_double;
    fn srand(seed: c_int);
}

// Minctest framework translation
const LTEST_FLOAT_TOLERANCE: f64 = 0.001;

static mut LTESTS: i32 = 0;
static mut LFAILS: i32 = 0;

fn lresults() {
    unsafe {
        if LFAILS == 0 {
            println!("ALL TESTS PASSED ({}/{})", LTESTS, LTESTS);
        } else {
            println!("SOME TESTS FAILED ({}/{})", LTESTS - LFAILS, LTESTS);
        }
    }
}

fn lok(test: bool, file: &str, line: u32) {
    unsafe {
        LTESTS += 1;
        if !test {
            LFAILS += 1;
            println!("{}:{} error", file, line);
        }
    }
}

fn lequal(a: i32, b: i32, file: &str, line: u32) {
    unsafe {
        LTESTS += 1;
        if a != b {
            LFAILS += 1;
            println!("{}:{} ({} != {})", file, line, a, b);
        }
    }
}

fn lfequal(a: f64, b: f64, file: &str, line: u32) {
    unsafe {
        LTESTS += 1;
        if (a - b).abs() > LTEST_FLOAT_TOLERANCE {
            LFAILS += 1;
            println!("{}:{} ({} != {})", file, line, a, b);
        }
    }
}

macro_rules! lok {
    ($test:expr) => {
        lok($test, file!(), line!())
    };
}

macro_rules! lequal {
    ($a:expr, $b:expr) => {
        lequal($a as i32, $b as i32, file!(), line!())
    };
}

macro_rules! lfequal {
    ($a:expr, $b:expr) => {
        lfequal($a as f64, $b as f64, file!(), line!())
    };
}

fn lrun<F: FnOnce()>(name: &str, test: F) {
    unsafe {
        let ts = LTESTS;
        let fs = LFAILS;
        let start = std::time::Instant::now();
        print!("\t{:14}", name);
        test();
        let elapsed = start.elapsed().as_millis();
        println!(
            "pass:{:2}   fail:{:2}   {:4}ms",
            (LTESTS - ts) - (LFAILS - fs),
            LFAILS - fs,
            elapsed
        );
    }
}

// Test functions
fn test_basic() {
    unsafe {
        let ann = genann_init(1, 0, 0, 1);
        assert!(!ann.is_null());

        lequal!((*ann).total_weights, 2);

        let mut a: c_double;

        a = 0.0;
        *(*ann).weight.add(0) = 0.0;
        *(*ann).weight.add(1) = 0.0;
        lfequal!(0.5, *genann_run(ann, &a));

        a = 1.0;
        lfequal!(0.5, *genann_run(ann, &a));

        a = 11.0;
        lfequal!(0.5, *genann_run(ann, &a));

        a = 1.0;
        *(*ann).weight.add(0) = 1.0;
        *(*ann).weight.add(1) = 1.0;
        lfequal!(0.5, *genann_run(ann, &a));

        a = 10.0;
        *(*ann).weight.add(0) = 1.0;
        *(*ann).weight.add(1) = 1.0;
        lfequal!(1.0, *genann_run(ann, &a));

        a = -10.0;
        lfequal!(0.0, *genann_run(ann, &a));

        genann_free(ann);
    }
}

fn test_xor() {
    unsafe {
        let ann = genann_init(2, 1, 2, 1);
        (*ann).activation_hidden = Some(genann_act_threshold);
        (*ann).activation_output = Some(genann_act_threshold);

        lequal!((*ann).total_weights, 9);

        // First hidden
        *(*ann).weight.add(0) = 0.5;
        *(*ann).weight.add(1) = 1.0;
        *(*ann).weight.add(2) = 1.0;

        // Second hidden
        *(*ann).weight.add(3) = 1.0;
        *(*ann).weight.add(4) = 1.0;
        *(*ann).weight.add(5) = 1.0;

        // Output
        *(*ann).weight.add(6) = 0.5;
        *(*ann).weight.add(7) = 1.0;
        *(*ann).weight.add(8) = -1.0;

        let input: [[c_double; 2]; 4] = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
        let output: [c_double; 4] = [0.0, 1.0, 1.0, 0.0];

        lfequal!(output[0], *genann_run(ann, input[0].as_ptr()));
        lfequal!(output[1], *genann_run(ann, input[1].as_ptr()));
        lfequal!(output[2], *genann_run(ann, input[2].as_ptr()));
        lfequal!(output[3], *genann_run(ann, input[3].as_ptr()));

        genann_free(ann);
    }
}

fn test_backprop() {
    unsafe {
        let ann = genann_init(1, 0, 0, 1);

        let input: c_double = 0.5;
        let output: c_double = 1.0;

        let first_try = *genann_run(ann, &input);
        genann_train(ann, &input, &output, 0.5);
        let second_try = *genann_run(ann, &input);
        lok!((first_try - output).abs() > (second_try - output).abs());

        genann_free(ann);
    }
}

fn test_train_and() {
    unsafe {
        let input: [[c_double; 2]; 4] = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
        let output: [c_double; 4] = [0.0, 0.0, 0.0, 1.0];

        let ann = genann_init(2, 0, 0, 1);

        for _ in 0..50 {
            for j in 0..4 {
                genann_train(ann, input[j].as_ptr(), output.as_ptr().add(j), 0.8);
            }
        }

        (*ann).activation_output = Some(genann_act_threshold);
        lfequal!(output[0], *genann_run(ann, input[0].as_ptr()));
        lfequal!(output[1], *genann_run(ann, input[1].as_ptr()));
        lfequal!(output[2], *genann_run(ann, input[2].as_ptr()));
        lfequal!(output[3], *genann_run(ann, input[3].as_ptr()));

        genann_free(ann);
    }
}

fn test_train_or() {
    unsafe {
        let input: [[c_double; 2]; 4] = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
        let output: [c_double; 4] = [0.0, 1.0, 1.0, 1.0];

        let ann = genann_init(2, 0, 0, 1);
        genann_randomize(ann);

        for _ in 0..50 {
            for j in 0..4 {
                genann_train(ann, input[j].as_ptr(), output.as_ptr().add(j), 0.8);
            }
        }

        (*ann).activation_output = Some(genann_act_threshold);
        lfequal!(output[0], *genann_run(ann, input[0].as_ptr()));
        lfequal!(output[1], *genann_run(ann, input[1].as_ptr()));
        lfequal!(output[2], *genann_run(ann, input[2].as_ptr()));
        lfequal!(output[3], *genann_run(ann, input[3].as_ptr()));

        genann_free(ann);
    }
}

fn test_train_xor() {
    unsafe {
        let input: [[c_double; 2]; 4] = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
        let output: [c_double; 4] = [0.0, 1.0, 1.0, 0.0];

        let ann = genann_init(2, 1, 2, 1);

        for _ in 0..500 {
            for j in 0..4 {
                genann_train(ann, input[j].as_ptr(), output.as_ptr().add(j), 3.0);
            }
        }

        (*ann).activation_output = Some(genann_act_threshold);
        lfequal!(output[0], *genann_run(ann, input[0].as_ptr()));
        lfequal!(output[1], *genann_run(ann, input[1].as_ptr()));
        lfequal!(output[2], *genann_run(ann, input[2].as_ptr()));
        lfequal!(output[3], *genann_run(ann, input[3].as_ptr()));

        genann_free(ann);
    }
}

fn test_persist() {
    unsafe {
        let first = genann_init(1000, 5, 50, 10);

        // Write to file
        let filename = CString::new("persist.txt").unwrap();
        let mode_w = CString::new("w").unwrap();
        let out = libc::fopen(filename.as_ptr(), mode_w.as_ptr());
        genann_write(first, out);
        libc::fclose(out);

        // Read from file
        let mode_r = CString::new("r").unwrap();
        let inp = libc::fopen(filename.as_ptr(), mode_r.as_ptr());
        let second = genann_read(inp);
        libc::fclose(inp);

        lequal!((*first).inputs, (*second).inputs);
        lequal!((*first).hidden_layers, (*second).hidden_layers);
        lequal!((*first).hidden, (*second).hidden);
        lequal!((*first).outputs, (*second).outputs);
        lequal!((*first).total_weights, (*second).total_weights);

        for i in 0..(*first).total_weights as usize {
            lok!(*(*first).weight.add(i) == *(*second).weight.add(i));
        }

        genann_free(first);
        genann_free(second);
    }
}

fn test_copy() {
    unsafe {
        let first = genann_init(1000, 5, 50, 10);
        let second = genann_copy(first);

        lequal!((*first).inputs, (*second).inputs);
        lequal!((*first).hidden_layers, (*second).hidden_layers);
        lequal!((*first).hidden, (*second).hidden);
        lequal!((*first).outputs, (*second).outputs);
        lequal!((*first).total_weights, (*second).total_weights);

        for i in 0..(*first).total_weights as usize {
            lfequal!(*(*first).weight.add(i), *(*second).weight.add(i));
        }

        genann_free(first);
        genann_free(second);
    }
}

fn test_sigmoid() {
    unsafe {
        let mut i: f64 = -20.0;
        let max: f64 = 20.0;
        let d: f64 = 0.0001;

        while i < max {
            lfequal!(
                genann_act_sigmoid(ptr::null(), i),
                genann_act_sigmoid_cached(ptr::null(), i)
            );
            i += d;
        }
    }
}

fn main() {
    println!("GENANN TEST SUITE");

    unsafe {
        srand(100); // Repeatable test results
    }

    lrun("basic", test_basic);
    lrun("xor", test_xor);
    lrun("backprop", test_backprop);
    lrun("train and", test_train_and);
    lrun("train or", test_train_or);
    lrun("train xor", test_train_xor);
    lrun("persist", test_persist);
    lrun("copy", test_copy);
    lrun("sigmoid", test_sigmoid);

    lresults();

    unsafe {
        std::process::exit(if LFAILS != 0 { 1 } else { 0 });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genann_basic() {
        test_basic();
    }

    #[test]
    fn test_genann_xor() {
        test_xor();
    }

    #[test]
    fn test_genann_backprop() {
        test_backprop();
    }

    #[test]
    fn test_genann_train_and() {
        test_train_and();
    }

    #[test]
    fn test_genann_train_or() {
        test_train_or();
    }

    #[test]
    fn test_genann_train_xor() {
        test_train_xor();
    }

    #[test]
    fn test_genann_copy() {
        test_copy();
    }
}
