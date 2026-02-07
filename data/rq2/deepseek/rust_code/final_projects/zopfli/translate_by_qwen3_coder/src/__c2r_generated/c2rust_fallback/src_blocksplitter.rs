#![allow(deref_nullptr)]
#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
// === C2R_C2RUST_EXTERN_TYPES_BEGIN ===
// Auto-generated: downgraded c2rust `extern type` to stable-safe opaque structs.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IO_wide_data {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IO_codecvt {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IO_marker {
    _unused: [u8; 0],
}

// === C2R_C2RUST_EXTERN_TYPES_END ===

extern "C" {
    pub fn ZopfliAllocHash(window_size: size_t, h: *mut ZopfliHash);
    pub fn ZopfliCleanHash(h: *mut ZopfliHash);
    pub fn ZopfliInitLZ77Store(data: *const core::ffi::c_uchar, store: *mut ZopfliLZ77Store);
    pub fn ZopfliCleanLZ77Store(store: *mut ZopfliLZ77Store);
    pub fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
    pub fn ZopfliCalculateBlockSizeAutoType(
        lz77: *const ZopfliLZ77Store,
        lstart: size_t,
        lend: size_t,
    ) -> core::ffi::c_double;
    pub fn ZopfliInitBlockState(
        options: *const ZopfliOptions,
        blockstart: size_t,
        blockend: size_t,
        add_lmc: core::ffi::c_int,
        s: *mut ZopfliBlockState,
    );
    pub fn ZopfliCleanBlockState(s: *mut ZopfliBlockState);
    pub fn ZopfliLZ77Greedy(
        s: *mut ZopfliBlockState,
        in_0: *const core::ffi::c_uchar,
        instart: size_t,
        inend: size_t,
        store: *mut ZopfliLZ77Store,
        h: *mut ZopfliHash,
    );
    static mut stderr: *mut FILE;
    pub fn fprintf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    pub fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    pub fn realloc(__ptr: *mut core::ffi::c_void, __size: size_t) -> *mut core::ffi::c_void;
    pub fn free(__ptr: *mut core::ffi::c_void);
    pub fn exit(__status: core::ffi::c_int) -> !;
}
pub type size_t = core::ffi::c_ulong;
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliLongestMatchCache {
    pub length: *mut core::ffi::c_ushort,
    pub dist: *mut core::ffi::c_ushort,
    pub sublen: *mut core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliHash {
    pub head: *mut core::ffi::c_int,
    pub prev: *mut core::ffi::c_ushort,
    pub hashval: *mut core::ffi::c_int,
    pub val: core::ffi::c_int,
    pub head2: *mut core::ffi::c_int,
    pub prev2: *mut core::ffi::c_ushort,
    pub hashval2: *mut core::ffi::c_int,
    pub val2: core::ffi::c_int,
    pub same: *mut core::ffi::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliOptions {
    pub verbose: core::ffi::c_int,
    pub verbose_more: core::ffi::c_int,
    pub numiterations: core::ffi::c_int,
    pub blocksplitting: core::ffi::c_int,
    pub blocksplittinglast: core::ffi::c_int,
    pub blocksplittingmax: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliLZ77Store {
    pub litlens: *mut core::ffi::c_ushort,
    pub dists: *mut core::ffi::c_ushort,
    pub size: size_t,
    pub data: *const core::ffi::c_uchar,
    pub pos: *mut size_t,
    pub ll_symbol: *mut core::ffi::c_ushort,
    pub d_symbol: *mut core::ffi::c_ushort,
    pub ll_counts: *mut size_t,
    pub d_counts: *mut size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliBlockState {
    pub options: *const ZopfliOptions,
    pub lmc: *mut ZopfliLongestMatchCache,
    pub blockstart: size_t,
    pub blockend: size_t,
}
pub type FILE = _IO_FILE;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SplitCostContext {
    pub lz77: *const ZopfliLZ77Store,
    pub start: size_t,
    pub end: size_t,
}
pub type FindMinimumFun = unsafe extern "C" fn(
    size_t,
    *mut core::ffi::c_void,
) -> core::ffi::c_double;
pub unsafe extern "C" fn FindMinimum(
    mut f: Option<FindMinimumFun>,
    mut context: *mut core::ffi::c_void,
    mut start: size_t,
    mut end: size_t,
    mut smallest: *mut core::ffi::c_double,
) -> size_t {
    if end.wrapping_sub(start) < 1024 as core::ffi::c_ulong {
        let mut best: core::ffi::c_double = 1e30f64;
        let mut result: size_t = start;
        let mut i: size_t = 0;
        i = start;
        while i < end {
            let mut v: core::ffi::c_double = f
                .expect("non-null function pointer")(i, context);
            if v < best {
                best = v;
                result = i;
            }
            i = i.wrapping_add(1);
        }
        *smallest = best;
        return result;
    } else {
        let mut i_0: size_t = 0;
        let mut p: [size_t; 9] = [0; 9];
        let mut vp: [core::ffi::c_double; 9] = [0.; 9];
        let mut besti: size_t = 0;
        let mut best_0: core::ffi::c_double = 0.;
        let mut lastbest: core::ffi::c_double = 1e30f64;
        let mut pos: size_t = start;
        while !(end.wrapping_sub(start) <= 9 as core::ffi::c_ulong) {
            i_0 = 0 as size_t;
            while i_0 < 9 as core::ffi::c_ulong {
                p[i_0 as usize] = (start as core::ffi::c_ulong)
                    .wrapping_add(
                        (i_0 as core::ffi::c_ulong)
                            .wrapping_add(1 as core::ffi::c_ulong)
                            .wrapping_mul(
                                (end as core::ffi::c_ulong)
                                    .wrapping_sub(start as core::ffi::c_ulong)
                                    .wrapping_div(
                                        (9 as core::ffi::c_int + 1 as core::ffi::c_int)
                                            as core::ffi::c_ulong,
                                    ),
                            ),
                    ) as size_t;
                vp[i_0 as usize] = f
                    .expect("non-null function pointer")(p[i_0 as usize], context);
                i_0 = i_0.wrapping_add(1);
            }
            besti = 0 as size_t;
            best_0 = vp[0 as core::ffi::c_int as usize];
            i_0 = 1 as size_t;
            while i_0 < 9 as core::ffi::c_ulong {
                if vp[i_0 as usize] < best_0 {
                    best_0 = vp[i_0 as usize];
                    besti = i_0;
                }
                i_0 = i_0.wrapping_add(1);
            }
            if best_0 > lastbest {
                break;
            }
            start = (if besti == 0 as core::ffi::c_ulong {
                start as core::ffi::c_ulong
            } else {
                p[(besti as core::ffi::c_ulong).wrapping_sub(1 as core::ffi::c_ulong)
                    as usize]
            }) as size_t;
            end = (if besti
                == (9 as core::ffi::c_int - 1 as core::ffi::c_int) as core::ffi::c_ulong
            {
                end as core::ffi::c_ulong
            } else {
                p[(besti as core::ffi::c_ulong).wrapping_add(1 as core::ffi::c_ulong)
                    as usize]
            }) as size_t;
            pos = p[besti as usize];
            lastbest = best_0;
        }
        *smallest = lastbest;
        return pos;
    };
}
pub unsafe extern "C" fn EstimateCost(
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
) -> core::ffi::c_double {
    return ZopfliCalculateBlockSizeAutoType(lz77, lstart, lend);
}
pub unsafe extern "C" fn SplitCost(
    mut i: size_t,
    mut context: *mut core::ffi::c_void,
) -> core::ffi::c_double {
    let mut c: *mut SplitCostContext = context as *mut SplitCostContext;
    return EstimateCost((*c).lz77, (*c).start, i) + EstimateCost((*c).lz77, i, (*c).end);
}
pub unsafe extern "C" fn AddSorted(
    mut value: size_t,
    mut out: *mut *mut size_t,
    mut outsize: *mut size_t,
) {
    let mut i: size_t = 0;
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<size_t>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<size_t>() as size_t),
            )
        }) as *mut size_t;
    }
    *(*out).offset(*outsize as isize) = value;
    *outsize = (*outsize).wrapping_add(1);
    i = 0 as size_t;
    while (i as core::ffi::c_ulong).wrapping_add(1 as core::ffi::c_ulong) < *outsize {
        if *(*out).offset(i as isize) > value {
            let mut j: size_t = 0;
            j = (*outsize).wrapping_sub(1 as core::ffi::c_ulong) as size_t;
            while j > i {
                *(*out).offset(j as isize) = *(*out)
                    .offset(
                        (j as core::ffi::c_ulong).wrapping_sub(1 as core::ffi::c_ulong)
                            as isize,
                    );
                j = j.wrapping_sub(1);
            }
            *(*out).offset(i as isize) = value;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
}
pub unsafe extern "C" fn PrintBlockSplitPoints(
    mut lz77: *const ZopfliLZ77Store,
    mut lz77splitpoints: *const size_t,
    mut nlz77points: size_t,
) {
    let mut splitpoints: *mut size_t = 0 as *mut size_t;
    let mut npoints: size_t = 0 as size_t;
    let mut i: size_t = 0;
    let mut pos: size_t = 0 as size_t;
    if nlz77points > 0 as core::ffi::c_ulong {
        i = 0 as size_t;
        while i < (*lz77).size {
            let mut length: size_t = (if *((*lz77).dists).offset(i as isize)
                as core::ffi::c_int == 0 as core::ffi::c_int
            {
                1 as core::ffi::c_int
            } else {
                *((*lz77).litlens).offset(i as isize) as core::ffi::c_int
            }) as size_t;
            if *lz77splitpoints.offset(npoints as isize) == i {
                if npoints & npoints.wrapping_sub(1 as core::ffi::c_ulong) == 0 {
                    splitpoints = (if npoints == 0 as core::ffi::c_ulong {
                        malloc(::core::mem::size_of::<size_t>() as size_t)
                    } else {
                        realloc(
                            splitpoints as *mut core::ffi::c_void,
                            npoints
                                .wrapping_mul(2 as size_t)
                                .wrapping_mul(::core::mem::size_of::<size_t>() as size_t),
                        )
                    }) as *mut size_t;
                }
                *splitpoints.offset(npoints as isize) = pos;
                npoints = npoints.wrapping_add(1);
                if npoints == nlz77points {
                    break;
                }
            }
            pos = (pos as core::ffi::c_ulong).wrapping_add(length as core::ffi::c_ulong)
                as size_t as size_t;
            i = i.wrapping_add(1);
        }
    }
    if npoints == nlz77points {} else {
        __assert_fail(
            b"npoints == nlz77points\0" as *const u8 as *const core::ffi::c_char,
            b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/blocksplitter.c\0"
                as *const u8 as *const core::ffi::c_char,
            167 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 76],
                [core::ffi::c_char; 76],
            >(
                *b"void PrintBlockSplitPoints(const ZopfliLZ77Store *, const size_t *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1874: {
        if npoints == nlz77points {} else {
            __assert_fail(
                b"npoints == nlz77points\0" as *const u8 as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/blocksplitter.c\0"
                    as *const u8 as *const core::ffi::c_char,
                167 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 76],
                    [core::ffi::c_char; 76],
                >(
                    *b"void PrintBlockSplitPoints(const ZopfliLZ77Store *, const size_t *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    fprintf(stderr, b"block split points: \0" as *const u8 as *const core::ffi::c_char);
    i = 0 as size_t;
    while i < npoints {
        fprintf(
            stderr,
            b"%d \0" as *const u8 as *const core::ffi::c_char,
            *splitpoints.offset(i as isize) as core::ffi::c_int,
        );
        i = i.wrapping_add(1);
    }
    fprintf(stderr, b"(hex:\0" as *const u8 as *const core::ffi::c_char);
    i = 0 as size_t;
    while i < npoints {
        fprintf(
            stderr,
            b" %x\0" as *const u8 as *const core::ffi::c_char,
            *splitpoints.offset(i as isize) as core::ffi::c_int,
        );
        i = i.wrapping_add(1);
    }
    fprintf(stderr, b")\n\0" as *const u8 as *const core::ffi::c_char);
    free(splitpoints as *mut core::ffi::c_void);
}
pub unsafe extern "C" fn FindLargestSplittableBlock(
    mut lz77size: size_t,
    mut done: *const core::ffi::c_uchar,
    mut splitpoints: *const size_t,
    mut npoints: size_t,
    mut lstart: *mut size_t,
    mut lend: *mut size_t,
) -> core::ffi::c_int {
    let mut longest: size_t = 0 as size_t;
    let mut found: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i <= npoints {
        let mut start: size_t = if i == 0 as core::ffi::c_ulong {
            0 as size_t
        } else {
            *splitpoints
                .offset(
                    (i as core::ffi::c_ulong).wrapping_sub(1 as core::ffi::c_ulong)
                        as isize,
                )
        };
        let mut end: size_t = if i == npoints {
            lz77size.wrapping_sub(1 as size_t)
        } else {
            *splitpoints.offset(i as isize)
        };
        if *done.offset(start as isize) == 0 && end.wrapping_sub(start) > longest {
            *lstart = start;
            *lend = end;
            found = 1 as core::ffi::c_int;
            longest = end.wrapping_sub(start);
        }
        i = i.wrapping_add(1);
    }
    return found;
}
pub unsafe extern "C" fn ZopfliBlockSplitLZ77(
    mut options: *const ZopfliOptions,
    mut lz77: *const ZopfliLZ77Store,
    mut maxblocks: size_t,
    mut splitpoints: *mut *mut size_t,
    mut npoints: *mut size_t,
) {
    let mut lstart: size_t = 0;
    let mut lend: size_t = 0;
    let mut i: size_t = 0;
    let mut llpos: size_t = 0 as size_t;
    let mut numblocks: size_t = 1 as size_t;
    let mut done: *mut core::ffi::c_uchar = 0 as *mut core::ffi::c_uchar;
    let mut splitcost: core::ffi::c_double = 0.;
    let mut origcost: core::ffi::c_double = 0.;
    if (*lz77).size < 10 as core::ffi::c_ulong {
        return;
    }
    done = malloc((*lz77).size) as *mut core::ffi::c_uchar;
    if done.is_null() {
        exit(-(1 as core::ffi::c_int));
    }
    i = 0 as size_t;
    while i < (*lz77).size {
        *done.offset(i as isize) = 0 as core::ffi::c_uchar;
        i = i.wrapping_add(1);
    }
    lstart = 0 as size_t;
    lend = (*lz77).size;
    loop {
        let mut c: SplitCostContext = SplitCostContext {
            lz77: 0 as *const ZopfliLZ77Store,
            start: 0,
            end: 0,
        };
        if maxblocks > 0 as core::ffi::c_ulong && numblocks >= maxblocks {
            break;
        }
        c.lz77 = lz77;
        c.start = lstart;
        c.end = lend;
        if lstart < lend {} else {
            __assert_fail(
                b"lstart < lend\0" as *const u8 as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/blocksplitter.c\0"
                    as *const u8 as *const core::ffi::c_char,
                243 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 103],
                    [core::ffi::c_char; 103],
                >(
                    *b"void ZopfliBlockSplitLZ77(const ZopfliOptions *, const ZopfliLZ77Store *, size_t, size_t **, size_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_2994: {
            if lstart < lend {} else {
                __assert_fail(
                    b"lstart < lend\0" as *const u8 as *const core::ffi::c_char,
                    b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/blocksplitter.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    243 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 103],
                        [core::ffi::c_char; 103],
                    >(
                        *b"void ZopfliBlockSplitLZ77(const ZopfliOptions *, const ZopfliLZ77Store *, size_t, size_t **, size_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        llpos = FindMinimum(
            Some(
                SplitCost
                    as unsafe extern "C" fn(
                        size_t,
                        *mut core::ffi::c_void,
                    ) -> core::ffi::c_double,
            ),
            &mut c as *mut SplitCostContext as *mut core::ffi::c_void,
            lstart.wrapping_add(1 as size_t),
            lend,
            &mut splitcost,
        );
        if llpos > lstart {} else {
            __assert_fail(
                b"llpos > lstart\0" as *const u8 as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/blocksplitter.c\0"
                    as *const u8 as *const core::ffi::c_char,
                246 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 103],
                    [core::ffi::c_char; 103],
                >(
                    *b"void ZopfliBlockSplitLZ77(const ZopfliOptions *, const ZopfliLZ77Store *, size_t, size_t **, size_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_2584: {
            if llpos > lstart {} else {
                __assert_fail(
                    b"llpos > lstart\0" as *const u8 as *const core::ffi::c_char,
                    b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/blocksplitter.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    246 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 103],
                        [core::ffi::c_char; 103],
                    >(
                        *b"void ZopfliBlockSplitLZ77(const ZopfliOptions *, const ZopfliLZ77Store *, size_t, size_t **, size_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if llpos < lend {} else {
            __assert_fail(
                b"llpos < lend\0" as *const u8 as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/blocksplitter.c\0"
                    as *const u8 as *const core::ffi::c_char,
                247 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 103],
                    [core::ffi::c_char; 103],
                >(
                    *b"void ZopfliBlockSplitLZ77(const ZopfliOptions *, const ZopfliLZ77Store *, size_t, size_t **, size_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_2544: {
            if llpos < lend {} else {
                __assert_fail(
                    b"llpos < lend\0" as *const u8 as *const core::ffi::c_char,
                    b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/blocksplitter.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    247 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 103],
                        [core::ffi::c_char; 103],
                    >(
                        *b"void ZopfliBlockSplitLZ77(const ZopfliOptions *, const ZopfliLZ77Store *, size_t, size_t **, size_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        origcost = EstimateCost(lz77, lstart, lend);
        if splitcost > origcost
            || llpos
                == (lstart as core::ffi::c_ulong).wrapping_add(1 as core::ffi::c_ulong)
            || llpos == lend
        {
            *done.offset(lstart as isize) = 1 as core::ffi::c_uchar;
        } else {
            AddSorted(llpos, splitpoints, npoints);
            numblocks = numblocks.wrapping_add(1);
        }
        if FindLargestSplittableBlock(
            (*lz77).size,
            done,
            *splitpoints,
            *npoints,
            &mut lstart,
            &mut lend,
        ) == 0
        {
            break;
        }
        if lend.wrapping_sub(lstart) < 10 as core::ffi::c_ulong {
            break;
        }
    }
    if (*options).verbose != 0 {
        PrintBlockSplitPoints(lz77, *splitpoints, *npoints);
    }
    free(done as *mut core::ffi::c_void);
}
pub unsafe extern "C" fn ZopfliBlockSplit(
    mut options: *const ZopfliOptions,
    mut in_0: *const core::ffi::c_uchar,
    mut instart: size_t,
    mut inend: size_t,
    mut maxblocks: size_t,
    mut splitpoints: *mut *mut size_t,
    mut npoints: *mut size_t,
) {
    let mut pos: size_t = 0 as size_t;
    let mut i: size_t = 0;
    let mut s: ZopfliBlockState = ZopfliBlockState {
        options: 0 as *const ZopfliOptions,
        lmc: 0 as *mut ZopfliLongestMatchCache,
        blockstart: 0,
        blockend: 0,
    };
    let mut lz77splitpoints: *mut size_t = 0 as *mut size_t;
    let mut nlz77points: size_t = 0 as size_t;
    let mut store: ZopfliLZ77Store = ZopfliLZ77Store {
        litlens: 0 as *mut core::ffi::c_ushort,
        dists: 0 as *mut core::ffi::c_ushort,
        size: 0,
        data: 0 as *const core::ffi::c_uchar,
        pos: 0 as *mut size_t,
        ll_symbol: 0 as *mut core::ffi::c_ushort,
        d_symbol: 0 as *mut core::ffi::c_ushort,
        ll_counts: 0 as *mut size_t,
        d_counts: 0 as *mut size_t,
    };
    let mut hash: ZopfliHash = ZopfliHash {
        head: 0 as *mut core::ffi::c_int,
        prev: 0 as *mut core::ffi::c_ushort,
        hashval: 0 as *mut core::ffi::c_int,
        val: 0,
        head2: 0 as *mut core::ffi::c_int,
        prev2: 0 as *mut core::ffi::c_ushort,
        hashval2: 0 as *mut core::ffi::c_int,
        val2: 0,
        same: 0 as *mut core::ffi::c_ushort,
    };
    let mut h: *mut ZopfliHash = &mut hash;
    ZopfliInitLZ77Store(in_0, &mut store);
    ZopfliInitBlockState(options, instart, inend, 0 as core::ffi::c_int, &mut s);
    ZopfliAllocHash(32768 as size_t, h);
    *npoints = 0 as size_t;
    *splitpoints = 0 as *mut size_t;
    ZopfliLZ77Greedy(&mut s, in_0, instart, inend, &mut store, h);
    ZopfliBlockSplitLZ77(
        options,
        &mut store,
        maxblocks,
        &mut lz77splitpoints,
        &mut nlz77points,
    );
    pos = instart;
    if nlz77points > 0 as core::ffi::c_ulong {
        i = 0 as size_t;
        while i < store.size {
            let mut length: size_t = (if *(store.dists).offset(i as isize)
                as core::ffi::c_int == 0 as core::ffi::c_int
            {
                1 as core::ffi::c_int
            } else {
                *(store.litlens).offset(i as isize) as core::ffi::c_int
            }) as size_t;
            if *lz77splitpoints.offset(*npoints as isize) == i {
                if *npoints & (*npoints).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
                    *splitpoints = (if *npoints == 0 as core::ffi::c_ulong {
                        malloc(::core::mem::size_of::<size_t>() as size_t)
                    } else {
                        realloc(
                            *splitpoints as *mut core::ffi::c_void,
                            (*npoints)
                                .wrapping_mul(2 as size_t)
                                .wrapping_mul(::core::mem::size_of::<size_t>() as size_t),
                        )
                    }) as *mut size_t;
                }
                *(*splitpoints).offset(*npoints as isize) = pos;
                *npoints = (*npoints).wrapping_add(1);
                if *npoints == nlz77points {
                    break;
                }
            }
            pos = (pos as core::ffi::c_ulong).wrapping_add(length as core::ffi::c_ulong)
                as size_t as size_t;
            i = i.wrapping_add(1);
        }
    }
    if *npoints == nlz77points {} else {
        __assert_fail(
            b"*npoints == nlz77points\0" as *const u8 as *const core::ffi::c_char,
            b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/blocksplitter.c\0"
                as *const u8 as *const core::ffi::c_char,
            314 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 113],
                [core::ffi::c_char; 113],
            >(
                *b"void ZopfliBlockSplit(const ZopfliOptions *, const unsigned char *, size_t, size_t, size_t, size_t **, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3189: {
        if *npoints == nlz77points {} else {
            __assert_fail(
                b"*npoints == nlz77points\0" as *const u8 as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/blocksplitter.c\0"
                    as *const u8 as *const core::ffi::c_char,
                314 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 113],
                    [core::ffi::c_char; 113],
                >(
                    *b"void ZopfliBlockSplit(const ZopfliOptions *, const unsigned char *, size_t, size_t, size_t, size_t **, size_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    free(lz77splitpoints as *mut core::ffi::c_void);
    ZopfliCleanBlockState(&mut s);
    ZopfliCleanLZ77Store(&mut store);
    ZopfliCleanHash(h);
}
pub unsafe extern "C" fn ZopfliBlockSplitSimple(
    mut in_0: *const core::ffi::c_uchar,
    mut instart: size_t,
    mut inend: size_t,
    mut blocksize: size_t,
    mut splitpoints: *mut *mut size_t,
    mut npoints: *mut size_t,
) {
    let mut i: size_t = instart;
    while i < inend {
        if *npoints & (*npoints).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
            *splitpoints = (if *npoints == 0 as core::ffi::c_ulong {
                malloc(::core::mem::size_of::<size_t>() as size_t)
            } else {
                realloc(
                    *splitpoints as *mut core::ffi::c_void,
                    (*npoints)
                        .wrapping_mul(2 as size_t)
                        .wrapping_mul(::core::mem::size_of::<size_t>() as size_t),
                )
            }) as *mut size_t;
        }
        *(*splitpoints).offset(*npoints as isize) = i;
        *npoints = (*npoints).wrapping_add(1);
        i = (i as core::ffi::c_ulong).wrapping_add(blocksize as core::ffi::c_ulong)
            as size_t as size_t;
    }
}
