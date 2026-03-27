extern "C" {
    fn ZopfliDeflate(
        options: *const ZopfliOptions,
        btype: core::ffi::c_int,
        final_0: core::ffi::c_int,
        in_0: *const core::ffi::c_uchar,
        insize: size_t,
        bp: *mut core::ffi::c_uchar,
        out: *mut *mut core::ffi::c_uchar,
        outsize: *mut size_t,
    );
    fn ZopfliGzipCompress(
        options: *const ZopfliOptions,
        in_0: *const core::ffi::c_uchar,
        insize: size_t,
        out: *mut *mut core::ffi::c_uchar,
        outsize: *mut size_t,
    );
    fn ZopfliZlibCompress(
        options: *const ZopfliOptions,
        in_0: *const core::ffi::c_uchar,
        insize: size_t,
        out: *mut *mut core::ffi::c_uchar,
        outsize: *mut size_t,
    );
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
}
pub type size_t = usize;
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
pub type ZopfliFormat = core::ffi::c_uint;
pub const ZOPFLI_FORMAT_DEFLATE: ZopfliFormat = 2;
pub const ZOPFLI_FORMAT_ZLIB: ZopfliFormat = 1;
pub const ZOPFLI_FORMAT_GZIP: ZopfliFormat = 0;
pub const __ASSERT_FUNCTION: [core::ffi::c_char; 116] = unsafe {
    ::core::mem::transmute::<
        [u8; 116],
        [core::ffi::c_char; 116],
    >(
        *b"void ZopfliCompress(const ZopfliOptions *, ZopfliFormat, const unsigned char *, size_t, unsigned char **, size_t *)\0",
    )
};
#[no_mangle]
pub unsafe extern "C" fn ZopfliCompress(
    mut options: *const ZopfliOptions,
    mut output_type: ZopfliFormat,
    mut in_0: *const core::ffi::c_uchar,
    mut insize: size_t,
    mut out: *mut *mut core::ffi::c_uchar,
    mut outsize: *mut size_t,
) {
    if output_type as core::ffi::c_uint
        == ZOPFLI_FORMAT_GZIP as core::ffi::c_int as core::ffi::c_uint
    {
        ZopfliGzipCompress(options, in_0, insize, out, outsize);
    } else if output_type as core::ffi::c_uint
        == ZOPFLI_FORMAT_ZLIB as core::ffi::c_int as core::ffi::c_uint
    {
        ZopfliZlibCompress(options, in_0, insize, out, outsize);
    } else if output_type as core::ffi::c_uint
        == ZOPFLI_FORMAT_DEFLATE as core::ffi::c_int as core::ffi::c_uint
    {
        let mut bp: core::ffi::c_uchar = 0 as core::ffi::c_uchar;
        ZopfliDeflate(
            options,
            2 as core::ffi::c_int,
            1 as core::ffi::c_int,
            in_0,
            insize,
            &mut bp,
            out,
            outsize,
        );
    } else {
        __assert_fail(
            b"0\0" as *const u8 as *const core::ffi::c_char,
            b"src/zopfli/zopfli_lib.c\0" as *const u8 as *const core::ffi::c_char,
            40 as core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
        'c_1190: {
            __assert_fail(
                b"0\0" as *const u8 as *const core::ffi::c_char,
                b"src/zopfli/zopfli_lib.c\0" as *const u8 as *const core::ffi::c_char,
                40 as core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        };
    };
}
