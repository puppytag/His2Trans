extern "C" {
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
    fn log(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn ZopfliLengthLimitedCodeLengths(
        frequencies: *const size_t,
        n: core::ffi::c_int,
        maxbits: core::ffi::c_int,
        bitlengths: *mut core::ffi::c_uint,
    ) -> core::ffi::c_int;
}
pub type size_t = usize;
#[no_mangle]
pub unsafe extern "C" fn ZopfliLengthsToSymbols(
    mut lengths: *const core::ffi::c_uint,
    mut n: size_t,
    mut maxbits: core::ffi::c_uint,
    mut symbols: *mut core::ffi::c_uint,
) {
    let mut bl_count: *mut size_t = malloc(
        (::core::mem::size_of::<size_t>() as size_t)
            .wrapping_mul(maxbits.wrapping_add(1 as core::ffi::c_uint) as size_t),
    ) as *mut size_t;
    let mut next_code: *mut size_t = malloc(
        (::core::mem::size_of::<size_t>() as size_t)
            .wrapping_mul(maxbits.wrapping_add(1 as core::ffi::c_uint) as size_t),
    ) as *mut size_t;
    let mut bits: core::ffi::c_uint = 0;
    let mut i: core::ffi::c_uint = 0;
    let mut code: core::ffi::c_uint = 0;
    i = 0 as core::ffi::c_uint;
    while (i as size_t) < n {
        *symbols.offset(i as isize) = 0 as core::ffi::c_uint;
        i = i.wrapping_add(1);
    }
    bits = 0 as core::ffi::c_uint;
    while bits <= maxbits {
        *bl_count.offset(bits as isize) = 0 as size_t;
        bits = bits.wrapping_add(1);
    }
    i = 0 as core::ffi::c_uint;
    while (i as size_t) < n {
        if *lengths.offset(i as isize) <= maxbits {} else {
            __assert_fail(
                b"lengths[i] <= maxbits\0" as *const u8 as *const core::ffi::c_char,
                b"src/zopfli/tree.c\0" as *const u8 as *const core::ffi::c_char,
                47 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 88],
                    [core::ffi::c_char; 88],
                >(
                    *b"void ZopfliLengthsToSymbols(const unsigned int *, size_t, unsigned int, unsigned int *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_630: {
            if *lengths.offset(i as isize) <= maxbits {} else {
                __assert_fail(
                    b"lengths[i] <= maxbits\0" as *const u8 as *const core::ffi::c_char,
                    b"src/zopfli/tree.c\0" as *const u8 as *const core::ffi::c_char,
                    47 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 88],
                        [core::ffi::c_char; 88],
                    >(
                        *b"void ZopfliLengthsToSymbols(const unsigned int *, size_t, unsigned int, unsigned int *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        let ref mut fresh0 = *bl_count.offset(*lengths.offset(i as isize) as isize);
        *fresh0 = (*fresh0).wrapping_add(1);
        i = i.wrapping_add(1);
    }
    code = 0 as core::ffi::c_uint;
    *bl_count.offset(0 as core::ffi::c_int as isize) = 0 as size_t;
    bits = 1 as core::ffi::c_uint;
    while bits <= maxbits {
        code = ((code as size_t)
            .wrapping_add(
                *bl_count.offset(bits.wrapping_sub(1 as core::ffi::c_uint) as isize),
            ) << 1 as core::ffi::c_int) as core::ffi::c_uint;
        *next_code.offset(bits as isize) = code as size_t;
        bits = bits.wrapping_add(1);
    }
    i = 0 as core::ffi::c_uint;
    while (i as size_t) < n {
        let mut len: core::ffi::c_uint = *lengths.offset(i as isize);
        if len != 0 as core::ffi::c_uint {
            *symbols.offset(i as isize) = *next_code.offset(len as isize)
                as core::ffi::c_uint;
            let ref mut fresh1 = *next_code.offset(len as isize);
            *fresh1 = (*fresh1).wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    free(bl_count as *mut core::ffi::c_void);
    free(next_code as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliCalculateEntropy(
    mut count: *const size_t,
    mut n: size_t,
    mut bitlengths: *mut core::ffi::c_double,
) {
    static mut kInvLog2: core::ffi::c_double = 1.4426950408889f64;
    let mut sum: core::ffi::c_uint = 0 as core::ffi::c_uint;
    let mut i: core::ffi::c_uint = 0;
    let mut log2sum: core::ffi::c_double = 0.;
    i = 0 as core::ffi::c_uint;
    while (i as size_t) < n {
        sum = (sum as core::ffi::c_ulong)
            .wrapping_add(*count.offset(i as isize) as core::ffi::c_ulong)
            as core::ffi::c_uint as core::ffi::c_uint;
        i = i.wrapping_add(1);
    }
    log2sum = (if sum == 0 as core::ffi::c_uint {
        log(n as core::ffi::c_double)
    } else {
        log(sum as core::ffi::c_double)
    }) * kInvLog2;
    i = 0 as core::ffi::c_uint;
    while (i as size_t) < n {
        if *count.offset(i as isize) == 0 as size_t {
            *bitlengths.offset(i as isize) = log2sum;
        } else {
            *bitlengths.offset(i as isize) = log2sum
                - log(*count.offset(i as isize) as core::ffi::c_double) * kInvLog2;
        }
        if *bitlengths.offset(i as isize) < 0 as core::ffi::c_int as core::ffi::c_double
            && *bitlengths.offset(i as isize) > -1e-5f64
        {
            *bitlengths.offset(i as isize) = 0 as core::ffi::c_int
                as core::ffi::c_double;
        }
        if *bitlengths.offset(i as isize) >= 0 as core::ffi::c_int as core::ffi::c_double
        {} else {
            __assert_fail(
                b"bitlengths[i] >= 0\0" as *const u8 as *const core::ffi::c_char,
                b"src/zopfli/tree.c\0" as *const u8 as *const core::ffi::c_char,
                92 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 62],
                    [core::ffi::c_char; 62],
                >(*b"void ZopfliCalculateEntropy(const size_t *, size_t, double *)\0"))
                    .as_ptr(),
            );
        }
        'c_756: {
            if *bitlengths.offset(i as isize)
                >= 0 as core::ffi::c_int as core::ffi::c_double
            {} else {
                __assert_fail(
                    b"bitlengths[i] >= 0\0" as *const u8 as *const core::ffi::c_char,
                    b"src/zopfli/tree.c\0" as *const u8 as *const core::ffi::c_char,
                    92 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 62],
                        [core::ffi::c_char; 62],
                    >(
                        *b"void ZopfliCalculateEntropy(const size_t *, size_t, double *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliCalculateBitLengths(
    mut count: *const size_t,
    mut n: size_t,
    mut maxbits: core::ffi::c_int,
    mut bitlengths: *mut core::ffi::c_uint,
) {
    let mut error: core::ffi::c_int = ZopfliLengthLimitedCodeLengths(
        count,
        n as core::ffi::c_int,
        maxbits,
        bitlengths,
    );
    if error == 0 {} else {
        __assert_fail(
            b"!error\0" as *const u8 as *const core::ffi::c_char,
            b"src/zopfli/tree.c\0" as *const u8 as *const core::ffi::c_char,
            100 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 76],
                [core::ffi::c_char; 76],
            >(
                *b"void ZopfliCalculateBitLengths(const size_t *, size_t, int, unsigned int *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_364: {
        if error == 0 {} else {
            __assert_fail(
                b"!error\0" as *const u8 as *const core::ffi::c_char,
                b"src/zopfli/tree.c\0" as *const u8 as *const core::ffi::c_char,
                100 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 76],
                    [core::ffi::c_char; 76],
                >(
                    *b"void ZopfliCalculateBitLengths(const size_t *, size_t, int, unsigned int *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
