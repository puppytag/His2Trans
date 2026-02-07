#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, label_break_value)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut core::ffi::c_int;
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
    fn exp(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn fscanf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn rand() -> core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn perror(__s: *const core::ffi::c_char);
}
pub type size_t = core::ffi::c_ulong;
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: core::ffi::c_int,
    pub _IO_read_ptr: *mut core::ffi::c_char,
    pub _IO_read_end: *mut core::ffi::c_char,
    pub _IO_read_base: *mut core::ffi::c_char,
    pub _IO_write_base: *mut core::ffi::c_char,
    pub _IO_write_ptr: *mut core::ffi::c_char,
    pub _IO_write_end: *mut core::ffi::c_char,
    pub _IO_buf_base: *mut core::ffi::c_char,
    pub _IO_buf_end: *mut core::ffi::c_char,
    pub _IO_save_base: *mut core::ffi::c_char,
    pub _IO_backup_base: *mut core::ffi::c_char,
    pub _IO_save_end: *mut core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: core::ffi::c_int,
    pub _flags2: core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: core::ffi::c_ushort,
    pub _vtable_offset: core::ffi::c_schar,
    pub _shortbuf: [core::ffi::c_char; 1],
    pub _lock: *mut core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: core::ffi::c_int,
    pub _unused2: [core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type genann_actfun = Option<
    unsafe extern "C" fn(core::ffi::c_double) -> core::ffi::c_double,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct genann {
    pub inputs: core::ffi::c_int,
    pub hidden_layers: core::ffi::c_int,
    pub hidden: core::ffi::c_int,
    pub outputs: core::ffi::c_int,
    pub activation_hidden: genann_actfun,
    pub activation_output: genann_actfun,
    pub total_weights: core::ffi::c_int,
    pub total_neurons: core::ffi::c_int,
    pub weight: *mut core::ffi::c_double,
    pub output: *mut core::ffi::c_double,
    pub delta: *mut core::ffi::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn genann_act_sigmoid(
    mut a: core::ffi::c_double,
) -> core::ffi::c_double {
    if a < -45.0f64 {
        return 0 as core::ffi::c_int as core::ffi::c_double;
    }
    if a > 45.0f64 {
        return 1 as core::ffi::c_int as core::ffi::c_double;
    }
    return 1.0f64 / (1 as core::ffi::c_int as core::ffi::c_double + exp(-a));
}
#[no_mangle]
pub unsafe extern "C" fn genann_act_sigmoid_cached(
    mut a: core::ffi::c_double,
) -> core::ffi::c_double {
    let min: core::ffi::c_double = -15.0f64;
    let max: core::ffi::c_double = 15.0f64;
    static mut interval: core::ffi::c_double = 0.;
    static mut initialized: core::ffi::c_int = 0 as core::ffi::c_int;
    static mut lookup: [core::ffi::c_double; 4096] = [0.; 4096];
    if initialized == 0 {
        interval = (max - min) / 4096 as core::ffi::c_int as core::ffi::c_double;
        let mut i: core::ffi::c_int = 0;
        i = 0 as core::ffi::c_int;
        while i < 4096 as core::ffi::c_int {
            lookup[i as usize] = genann_act_sigmoid(
                min + interval * i as core::ffi::c_double,
            );
            i += 1;
        }
        initialized = 1 as core::ffi::c_int;
    }
    let mut i_0: core::ffi::c_int = 0;
    i_0 = ((a - min) / interval + 0.5f64) as core::ffi::c_int;
    if i_0 <= 0 as core::ffi::c_int {
        return lookup[0 as core::ffi::c_int as usize];
    }
    if i_0 >= 4096 as core::ffi::c_int {
        return lookup[(4096 as core::ffi::c_int - 1 as core::ffi::c_int) as usize];
    }
    return lookup[i_0 as usize];
}
#[no_mangle]
pub unsafe extern "C" fn genann_act_threshold(
    mut a: core::ffi::c_double,
) -> core::ffi::c_double {
    return (a > 0 as core::ffi::c_int as core::ffi::c_double) as core::ffi::c_int
        as core::ffi::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn genann_act_linear(
    mut a: core::ffi::c_double,
) -> core::ffi::c_double {
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn genann_init(
    mut inputs: core::ffi::c_int,
    mut hidden_layers: core::ffi::c_int,
    mut hidden: core::ffi::c_int,
    mut outputs: core::ffi::c_int,
) -> *mut genann {
    if hidden_layers < 0 as core::ffi::c_int {
        return 0 as *mut genann;
    }
    if inputs < 1 as core::ffi::c_int {
        return 0 as *mut genann;
    }
    if outputs < 1 as core::ffi::c_int {
        return 0 as *mut genann;
    }
    if hidden_layers > 0 as core::ffi::c_int && hidden < 1 as core::ffi::c_int {
        return 0 as *mut genann;
    }
    let hidden_weights: core::ffi::c_int = if hidden_layers != 0 {
        (inputs + 1 as core::ffi::c_int) * hidden
            + (hidden_layers - 1 as core::ffi::c_int) * (hidden + 1 as core::ffi::c_int)
                * hidden
    } else {
        0 as core::ffi::c_int
    };
    let output_weights: core::ffi::c_int = (if hidden_layers != 0 {
        hidden + 1 as core::ffi::c_int
    } else {
        inputs + 1 as core::ffi::c_int
    }) * outputs;
    let total_weights: core::ffi::c_int = hidden_weights + output_weights;
    let total_neurons: core::ffi::c_int = inputs + hidden * hidden_layers + outputs;
    let size: core::ffi::c_int = (::core::mem::size_of::<genann>() as usize)
        .wrapping_add(
            (::core::mem::size_of::<core::ffi::c_double>() as usize)
                .wrapping_mul(
                    (total_weights + total_neurons + (total_neurons - inputs)) as usize,
                ),
        ) as core::ffi::c_int;
    let mut ret: *mut genann = malloc(size as size_t) as *mut genann;
    if ret.is_null() {
        return 0 as *mut genann;
    }
    (*ret).inputs = inputs;
    (*ret).hidden_layers = hidden_layers;
    (*ret).hidden = hidden;
    (*ret).outputs = outputs;
    (*ret).total_weights = total_weights;
    (*ret).total_neurons = total_neurons;
    (*ret).weight = (ret as *mut core::ffi::c_char)
        .offset(::core::mem::size_of::<genann>() as usize as isize)
        as *mut core::ffi::c_double;
    (*ret).output = ((*ret).weight).offset((*ret).total_weights as isize);
    (*ret).delta = ((*ret).output).offset((*ret).total_neurons as isize);
    genann_randomize(ret);
    (*ret).activation_hidden = Some(
        genann_act_sigmoid_cached
            as unsafe extern "C" fn(core::ffi::c_double) -> core::ffi::c_double,
    ) as genann_actfun;
    (*ret).activation_output = Some(
        genann_act_sigmoid_cached
            as unsafe extern "C" fn(core::ffi::c_double) -> core::ffi::c_double,
    ) as genann_actfun;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn genann_read(mut in_0: *mut FILE) -> *mut genann {
    let mut inputs: core::ffi::c_int = 0;
    let mut hidden_layers: core::ffi::c_int = 0;
    let mut hidden: core::ffi::c_int = 0;
    let mut outputs: core::ffi::c_int = 0;
    let mut rc: core::ffi::c_int = 0;
    *__errno_location() = 0 as core::ffi::c_int;
    rc = fscanf(
        in_0,
        b"%d %d %d %d\0" as *const u8 as *const core::ffi::c_char,
        &mut inputs as *mut core::ffi::c_int,
        &mut hidden_layers as *mut core::ffi::c_int,
        &mut hidden as *mut core::ffi::c_int,
        &mut outputs as *mut core::ffi::c_int,
    );
    if rc < 4 as core::ffi::c_int || *__errno_location() != 0 as core::ffi::c_int {
        perror(b"fscanf\0" as *const u8 as *const core::ffi::c_char);
        return 0 as *mut genann;
    }
    let mut ann: *mut genann = genann_init(inputs, hidden_layers, hidden, outputs);
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < (*ann).total_weights {
        *__errno_location() = 0 as core::ffi::c_int;
        rc = fscanf(
            in_0,
            b" %le\0" as *const u8 as *const core::ffi::c_char,
            ((*ann).weight).offset(i as isize),
        );
        if rc < 1 as core::ffi::c_int || *__errno_location() != 0 as core::ffi::c_int {
            perror(b"fscanf\0" as *const u8 as *const core::ffi::c_char);
            genann_free(ann);
            return 0 as *mut genann;
        }
        i += 1;
    }
    return ann;
}
#[no_mangle]
pub unsafe extern "C" fn genann_copy(mut ann: *const genann) -> *mut genann {
    let size: core::ffi::c_int = (::core::mem::size_of::<genann>() as usize)
        .wrapping_add(
            (::core::mem::size_of::<core::ffi::c_double>() as usize)
                .wrapping_mul(
                    ((*ann).total_weights + (*ann).total_neurons
                        + ((*ann).total_neurons - (*ann).inputs)) as usize,
                ),
        ) as core::ffi::c_int;
    let mut ret: *mut genann = malloc(size as size_t) as *mut genann;
    if ret.is_null() {
        return 0 as *mut genann;
    }
    memcpy(
        ret as *mut core::ffi::c_void,
        ann as *const core::ffi::c_void,
        size as size_t,
    );
    (*ret).weight = (ret as *mut core::ffi::c_char)
        .offset(::core::mem::size_of::<genann>() as usize as isize)
        as *mut core::ffi::c_double;
    (*ret).output = ((*ret).weight).offset((*ret).total_weights as isize);
    (*ret).delta = ((*ret).output).offset((*ret).total_neurons as isize);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn genann_randomize(mut ann: *mut genann) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < (*ann).total_weights {
        let mut r: core::ffi::c_double = rand() as core::ffi::c_double
            / 2147483647 as core::ffi::c_int as core::ffi::c_double;
        *((*ann).weight).offset(i as isize) = r - 0.5f64;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn genann_free(mut ann: *mut genann) {
    free(ann as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn genann_run(
    mut ann: *const genann,
    mut inputs: *const core::ffi::c_double,
) -> *const core::ffi::c_double {
    let mut w: *const core::ffi::c_double = (*ann).weight;
    let mut o: *mut core::ffi::c_double = ((*ann).output).offset((*ann).inputs as isize);
    let mut i: *const core::ffi::c_double = (*ann).output;
    memcpy(
        (*ann).output as *mut core::ffi::c_void,
        inputs as *const core::ffi::c_void,
        (::core::mem::size_of::<core::ffi::c_double>() as size_t)
            .wrapping_mul((*ann).inputs as size_t),
    );
    let mut h: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut k: core::ffi::c_int = 0;
    let act: genann_actfun = (*ann).activation_hidden;
    let acto: genann_actfun = (*ann).activation_output;
    h = 0 as core::ffi::c_int;
    while h < (*ann).hidden_layers {
        j = 0 as core::ffi::c_int;
        while j < (*ann).hidden {
            let fresh0 = w;
            w = w.offset(1);
            let mut sum: core::ffi::c_double = *fresh0 * -1.0f64;
            k = 0 as core::ffi::c_int;
            while k
                < (if h == 0 as core::ffi::c_int {
                    (*ann).inputs
                } else {
                    (*ann).hidden
                })
            {
                let fresh1 = w;
                w = w.offset(1);
                sum += *fresh1 * *i.offset(k as isize);
                k += 1;
            }
            let fresh2 = o;
            o = o.offset(1);
            *fresh2 = act.expect("non-null function pointer")(sum);
            j += 1;
        }
        i = i
            .offset(
                (if h == 0 as core::ffi::c_int { (*ann).inputs } else { (*ann).hidden })
                    as isize,
            );
        h += 1;
    }
    let mut ret: *const core::ffi::c_double = o;
    j = 0 as core::ffi::c_int;
    while j < (*ann).outputs {
        let fresh3 = w;
        w = w.offset(1);
        let mut sum_0: core::ffi::c_double = *fresh3 * -1.0f64;
        k = 0 as core::ffi::c_int;
        while k < (if (*ann).hidden_layers != 0 { (*ann).hidden } else { (*ann).inputs })
        {
            let fresh4 = w;
            w = w.offset(1);
            sum_0 += *fresh4 * *i.offset(k as isize);
            k += 1;
        }
        let fresh5 = o;
        o = o.offset(1);
        *fresh5 = acto.expect("non-null function pointer")(sum_0);
        j += 1;
    }
    if w.offset_from((*ann).weight) as core::ffi::c_long
        == (*ann).total_weights as core::ffi::c_long
    {} else {
        __assert_fail(
            b"w - ann->weight == ann->total_weights\0" as *const u8
                as *const core::ffi::c_char,
            b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/genann/src/genann.c\0"
                as *const u8 as *const core::ffi::c_char,
            225 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 57],
                [core::ffi::c_char; 57],
            >(*b"const double *genann_run(const genann *, const double *)\0"))
                .as_ptr(),
        );
    }
    'c_1501: {
        if w.offset_from((*ann).weight) as core::ffi::c_long
            == (*ann).total_weights as core::ffi::c_long
        {} else {
            __assert_fail(
                b"w - ann->weight == ann->total_weights\0" as *const u8
                    as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/genann/src/genann.c\0"
                    as *const u8 as *const core::ffi::c_char,
                225 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 57],
                    [core::ffi::c_char; 57],
                >(*b"const double *genann_run(const genann *, const double *)\0"))
                    .as_ptr(),
            );
        }
    };
    if o.offset_from((*ann).output) as core::ffi::c_long
        == (*ann).total_neurons as core::ffi::c_long
    {} else {
        __assert_fail(
            b"o - ann->output == ann->total_neurons\0" as *const u8
                as *const core::ffi::c_char,
            b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/genann/src/genann.c\0"
                as *const u8 as *const core::ffi::c_char,
            226 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 57],
                [core::ffi::c_char; 57],
            >(*b"const double *genann_run(const genann *, const double *)\0"))
                .as_ptr(),
        );
    }
    'c_1436: {
        if o.offset_from((*ann).output) as core::ffi::c_long
            == (*ann).total_neurons as core::ffi::c_long
        {} else {
            __assert_fail(
                b"o - ann->output == ann->total_neurons\0" as *const u8
                    as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/genann/src/genann.c\0"
                    as *const u8 as *const core::ffi::c_char,
                226 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 57],
                    [core::ffi::c_char; 57],
                >(*b"const double *genann_run(const genann *, const double *)\0"))
                    .as_ptr(),
            );
        }
    };
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn genann_train(
    mut ann: *const genann,
    mut inputs: *const core::ffi::c_double,
    mut desired_outputs: *const core::ffi::c_double,
    mut learning_rate: core::ffi::c_double,
) {
    genann_run(ann, inputs);
    let mut h: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut k: core::ffi::c_int = 0;
    let mut o: *const core::ffi::c_double = ((*ann).output)
        .offset((*ann).inputs as isize)
        .offset(((*ann).hidden * (*ann).hidden_layers) as isize);
    let mut d: *mut core::ffi::c_double = ((*ann).delta)
        .offset(((*ann).hidden * (*ann).hidden_layers) as isize);
    let mut t: *const core::ffi::c_double = desired_outputs;
    if (*ann).activation_output
        == Some(
            genann_act_linear
                as unsafe extern "C" fn(core::ffi::c_double) -> core::ffi::c_double,
        )
    {
        j = 0 as core::ffi::c_int;
        while j < (*ann).outputs {
            let fresh6 = t;
            t = t.offset(1);
            let fresh7 = o;
            o = o.offset(1);
            let fresh8 = d;
            d = d.offset(1);
            *fresh8 = *fresh6 - *fresh7;
            j += 1;
        }
    } else {
        j = 0 as core::ffi::c_int;
        while j < (*ann).outputs {
            let fresh9 = d;
            d = d.offset(1);
            *fresh9 = (*t - *o) * *o * (1.0f64 - *o);
            o = o.offset(1);
            t = t.offset(1);
            j += 1;
        }
    }
    h = (*ann).hidden_layers - 1 as core::ffi::c_int;
    while h >= 0 as core::ffi::c_int {
        let mut o_0: *const core::ffi::c_double = ((*ann).output)
            .offset((*ann).inputs as isize)
            .offset((h * (*ann).hidden) as isize);
        let mut d_0: *mut core::ffi::c_double = ((*ann).delta)
            .offset((h * (*ann).hidden) as isize);
        let dd: *const core::ffi::c_double = ((*ann).delta)
            .offset(((h + 1 as core::ffi::c_int) * (*ann).hidden) as isize);
        let ww: *const core::ffi::c_double = ((*ann).weight)
            .offset((((*ann).inputs + 1 as core::ffi::c_int) * (*ann).hidden) as isize)
            .offset(
                (((*ann).hidden + 1 as core::ffi::c_int) * (*ann).hidden * h) as isize,
            );
        j = 0 as core::ffi::c_int;
        while j < (*ann).hidden {
            let mut delta: core::ffi::c_double = 0 as core::ffi::c_int
                as core::ffi::c_double;
            k = 0 as core::ffi::c_int;
            while k
                < (if h == (*ann).hidden_layers - 1 as core::ffi::c_int {
                    (*ann).outputs
                } else {
                    (*ann).hidden
                })
            {
                let forward_delta: core::ffi::c_double = *dd.offset(k as isize);
                let windex: core::ffi::c_int = k
                    * ((*ann).hidden + 1 as core::ffi::c_int)
                    + (j + 1 as core::ffi::c_int);
                let forward_weight: core::ffi::c_double = *ww.offset(windex as isize);
                delta += forward_delta * forward_weight;
                k += 1;
            }
            *d_0 = *o_0 * (1.0f64 - *o_0) * delta;
            d_0 = d_0.offset(1);
            o_0 = o_0.offset(1);
            j += 1;
        }
        h -= 1;
    }
    let mut d_1: *const core::ffi::c_double = ((*ann).delta)
        .offset(((*ann).hidden * (*ann).hidden_layers) as isize);
    let mut w: *mut core::ffi::c_double = ((*ann).weight)
        .offset(
            (if (*ann).hidden_layers != 0 {
                ((*ann).inputs + 1 as core::ffi::c_int) * (*ann).hidden
                    + ((*ann).hidden + 1 as core::ffi::c_int) * (*ann).hidden
                        * ((*ann).hidden_layers - 1 as core::ffi::c_int)
            } else {
                0 as core::ffi::c_int
            }) as isize,
        );
    let i: *const core::ffi::c_double = ((*ann).output)
        .offset(
            (if (*ann).hidden_layers != 0 {
                (*ann).inputs
                    + (*ann).hidden * ((*ann).hidden_layers - 1 as core::ffi::c_int)
            } else {
                0 as core::ffi::c_int
            }) as isize,
        );
    j = 0 as core::ffi::c_int;
    while j < (*ann).outputs {
        k = 0 as core::ffi::c_int;
        while k
            < (if (*ann).hidden_layers != 0 { (*ann).hidden } else { (*ann).inputs })
                + 1 as core::ffi::c_int
        {
            if k == 0 as core::ffi::c_int {
                let fresh10 = w;
                w = w.offset(1);
                *fresh10 += *d_1 * learning_rate * -1.0f64;
            } else {
                let fresh11 = w;
                w = w.offset(1);
                *fresh11
                    += *d_1 * learning_rate
                        * *i.offset((k - 1 as core::ffi::c_int) as isize);
            }
            k += 1;
        }
        d_1 = d_1.offset(1);
        j += 1;
    }
    if w.offset_from((*ann).weight) as core::ffi::c_long
        == (*ann).total_weights as core::ffi::c_long
    {} else {
        __assert_fail(
            b"w - ann->weight == ann->total_weights\0" as *const u8
                as *const core::ffi::c_char,
            b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/genann/src/genann.c\0"
                as *const u8 as *const core::ffi::c_char,
            318 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 74],
                [core::ffi::c_char; 74],
            >(
                *b"void genann_train(const genann *, const double *, const double *, double)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1990: {
        if w.offset_from((*ann).weight) as core::ffi::c_long
            == (*ann).total_weights as core::ffi::c_long
        {} else {
            __assert_fail(
                b"w - ann->weight == ann->total_weights\0" as *const u8
                    as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/genann/src/genann.c\0"
                    as *const u8 as *const core::ffi::c_char,
                318 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 74],
                    [core::ffi::c_char; 74],
                >(
                    *b"void genann_train(const genann *, const double *, const double *, double)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    h = (*ann).hidden_layers - 1 as core::ffi::c_int;
    while h >= 0 as core::ffi::c_int {
        let mut d_2: *const core::ffi::c_double = ((*ann).delta)
            .offset((h * (*ann).hidden) as isize);
        let mut i_0: *const core::ffi::c_double = ((*ann).output)
            .offset(
                (if h != 0 {
                    (*ann).inputs + (*ann).hidden * (h - 1 as core::ffi::c_int)
                } else {
                    0 as core::ffi::c_int
                }) as isize,
            );
        let mut w_0: *mut core::ffi::c_double = ((*ann).weight)
            .offset(
                (if h != 0 {
                    ((*ann).inputs + 1 as core::ffi::c_int) * (*ann).hidden
                        + ((*ann).hidden + 1 as core::ffi::c_int) * (*ann).hidden
                            * (h - 1 as core::ffi::c_int)
                } else {
                    0 as core::ffi::c_int
                }) as isize,
            );
        j = 0 as core::ffi::c_int;
        while j < (*ann).hidden {
            k = 0 as core::ffi::c_int;
            while k
                < (if h == 0 as core::ffi::c_int {
                    (*ann).inputs
                } else {
                    (*ann).hidden
                }) + 1 as core::ffi::c_int
            {
                if k == 0 as core::ffi::c_int {
                    let fresh12 = w_0;
                    w_0 = w_0.offset(1);
                    *fresh12 += *d_2 * learning_rate * -1.0f64;
                } else {
                    let fresh13 = w_0;
                    w_0 = w_0.offset(1);
                    *fresh13
                        += *d_2 * learning_rate
                            * *i_0.offset((k - 1 as core::ffi::c_int) as isize);
                }
                k += 1;
            }
            d_2 = d_2.offset(1);
            j += 1;
        }
        h -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn genann_write(mut ann: *const genann, mut out: *mut FILE) {
    fprintf(
        out,
        b"%d %d %d %d\0" as *const u8 as *const core::ffi::c_char,
        (*ann).inputs,
        (*ann).hidden_layers,
        (*ann).hidden,
        (*ann).outputs,
    );
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < (*ann).total_weights {
        fprintf(
            out,
            b" %.20e\0" as *const u8 as *const core::ffi::c_char,
            *((*ann).weight).offset(i as isize),
        );
        i += 1;
    }
}
