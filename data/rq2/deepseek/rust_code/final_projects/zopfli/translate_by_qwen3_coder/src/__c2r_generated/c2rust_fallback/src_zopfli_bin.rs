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
    pub fn ZopfliInitOptions(options: *mut ZopfliOptions);
    pub fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
    pub fn ZopfliCompress(
        options: *const ZopfliOptions,
        output_type: ZopfliFormat,
        in_0: *const core::ffi::c_uchar,
        insize: size_t,
        out: *mut *mut core::ffi::c_uchar,
        outsize: *mut size_t,
    );
    pub fn atoi(__nptr: *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn strcpy(
        __dest: *mut core::ffi::c_char,
        __src: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_char;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    pub fn strcat(
        __dest: *mut core::ffi::c_char,
        __src: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_char;
    pub fn strcmp(
        __s1: *const core::ffi::c_char,
        __s2: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    pub fn fclose(__stream: *mut FILE) -> core::ffi::c_int;
    pub fn fopen(
        __filename: *const core::ffi::c_char,
        __modes: *const core::ffi::c_char,
    ) -> *mut FILE;
    pub fn fprintf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    pub fn strlen(__s: *const core::ffi::c_char) -> size_t;
    pub fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    pub fn free(__ptr: *mut core::ffi::c_void);
    pub fn exit(__status: core::ffi::c_int) -> !;
    pub fn fread(
        __ptr: *mut core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> core::ffi::c_ulong;
    pub fn fwrite(
        __ptr: *const core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> core::ffi::c_ulong;
    pub fn fseek(
        __stream: *mut FILE,
        __off: core::ffi::c_long,
        __whence: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn ftell(__stream: *mut FILE) -> core::ffi::c_long;
    pub fn rewind(__stream: *mut FILE);
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
pub unsafe extern "C" fn LoadFile(
    mut filename: *const core::ffi::c_char,
    mut out: *mut *mut core::ffi::c_uchar,
    mut outsize: *mut size_t,
) -> core::ffi::c_int {
    let mut file: *mut FILE = 0 as *mut FILE;
    *out = 0 as *mut core::ffi::c_uchar;
    *outsize = 0 as size_t;
    file = fopen(filename, b"rb\0" as *const u8 as *const core::ffi::c_char);
    if file.is_null() {
        return 0 as core::ffi::c_int;
    }
    fseek(file, 0 as core::ffi::c_long, 2 as core::ffi::c_int);
    *outsize = ftell(file) as size_t;
    if *outsize > 2147483647 as core::ffi::c_ulong {
        fprintf(
            stderr,
            b"Files larger than 2GB are not supported.\n\0" as *const u8
                as *const core::ffi::c_char,
        );
        exit(1 as core::ffi::c_int);
    }
    rewind(file);
    *out = malloc(*outsize) as *mut core::ffi::c_uchar;
    if *outsize != 0 && !(*out).is_null() {
        let mut testsize: size_t = fread(
            *out as *mut core::ffi::c_void,
            1 as size_t,
            *outsize,
            file,
        ) as size_t;
        if testsize != *outsize {
            free(*out as *mut core::ffi::c_void);
            *out = 0 as *mut core::ffi::c_uchar;
            *outsize = 0 as size_t;
            fclose(file);
            return 0 as core::ffi::c_int;
        }
    }
    if *outsize == 0 || !out.is_null() {} else {
        __assert_fail(
            b"!(*outsize) || out\0" as *const u8 as *const core::ffi::c_char,
            b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/zopfli_bin.c\0"
                as *const u8 as *const core::ffi::c_char,
            76 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 55],
                [core::ffi::c_char; 55],
            >(*b"int LoadFile(const char *, unsigned char **, size_t *)\0"))
                .as_ptr(),
        );
    }
    'c_2201: {
        if *outsize == 0 || !out.is_null() {} else {
            __assert_fail(
                b"!(*outsize) || out\0" as *const u8 as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/zopfli_bin.c\0"
                    as *const u8 as *const core::ffi::c_char,
                76 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 55],
                    [core::ffi::c_char; 55],
                >(*b"int LoadFile(const char *, unsigned char **, size_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    fclose(file);
    return 1 as core::ffi::c_int;
}
pub unsafe extern "C" fn SaveFile(
    mut filename: *const core::ffi::c_char,
    mut in_0: *const core::ffi::c_uchar,
    mut insize: size_t,
) {
    let mut file: *mut FILE = fopen(
        filename,
        b"wb\0" as *const u8 as *const core::ffi::c_char,
    );
    if file.is_null() {
        fprintf(
            stderr,
            b"Error: Cannot write to output file, terminating.\n\0" as *const u8
                as *const core::ffi::c_char,
        );
        exit(1 as core::ffi::c_int);
    }
    if !file.is_null() {} else {
        __assert_fail(
            b"file\0" as *const u8 as *const core::ffi::c_char,
            b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/zopfli_bin.c\0"
                as *const u8 as *const core::ffi::c_char,
            91 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 59],
                [core::ffi::c_char; 59],
            >(*b"void SaveFile(const char *, const unsigned char *, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_2452: {
        if !file.is_null() {} else {
            __assert_fail(
                b"file\0" as *const u8 as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/zopfli_bin.c\0"
                    as *const u8 as *const core::ffi::c_char,
                91 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 59],
                    [core::ffi::c_char; 59],
                >(*b"void SaveFile(const char *, const unsigned char *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    fwrite(
        in_0 as *mut core::ffi::c_char as *const core::ffi::c_void,
        1 as size_t,
        insize,
        file,
    );
    fclose(file);
}
pub unsafe extern "C" fn CompressFile(
    mut options: *const ZopfliOptions,
    mut output_type: ZopfliFormat,
    mut infilename: *const core::ffi::c_char,
    mut outfilename: *const core::ffi::c_char,
) {
    let mut in_0: *mut core::ffi::c_uchar = 0 as *mut core::ffi::c_uchar;
    let mut insize: size_t = 0;
    let mut out: *mut core::ffi::c_uchar = 0 as *mut core::ffi::c_uchar;
    let mut outsize: size_t = 0 as size_t;
    if LoadFile(infilename, &mut in_0, &mut insize) == 0 {
        fprintf(
            stderr,
            b"Invalid filename: %s\n\0" as *const u8 as *const core::ffi::c_char,
            infilename,
        );
        return;
    }
    ZopfliCompress(options, output_type, in_0, insize, &mut out, &mut outsize);
    if !outfilename.is_null() {
        SaveFile(outfilename, out, outsize);
    } else {
        fwrite(out as *const core::ffi::c_void, 1 as size_t, outsize, stdout);
    }
    free(out as *mut core::ffi::c_void);
    free(in_0 as *mut core::ffi::c_void);
}
pub unsafe extern "C" fn AddStrings(
    mut str1: *const core::ffi::c_char,
    mut str2: *const core::ffi::c_char,
) -> *mut core::ffi::c_char {
    let mut len: size_t = (strlen(str1)).wrapping_add(strlen(str2));
    let mut result: *mut core::ffi::c_char = malloc(len.wrapping_add(1 as size_t))
        as *mut core::ffi::c_char;
    if result.is_null() {
        exit(-(1 as core::ffi::c_int));
    }
    strcpy(result, str1);
    strcat(result, str2);
    return result;
}
pub unsafe extern "C" fn StringsEqual(
    mut str1: *const core::ffi::c_char,
    mut str2: *const core::ffi::c_char,
) -> core::ffi::c_char {
    return (strcmp(str1, str2) == 0 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_char;
}
pub unsafe fn main_0(
    mut argc: core::ffi::c_int,
    mut argv: *mut *mut core::ffi::c_char,
) -> core::ffi::c_int {
    let mut options: ZopfliOptions = ZopfliOptions {
        verbose: 0,
        verbose_more: 0,
        numiterations: 0,
        blocksplitting: 0,
        blocksplittinglast: 0,
        blocksplittingmax: 0,
    };
    let mut output_type: ZopfliFormat = ZOPFLI_FORMAT_GZIP;
    let mut filename: *const core::ffi::c_char = 0 as *const core::ffi::c_char;
    let mut output_to_stdout: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0;
    ZopfliInitOptions(&mut options);
    i = 1 as core::ffi::c_int;
    while i < argc {
        let mut arg: *const core::ffi::c_char = *argv.offset(i as isize);
        if StringsEqual(arg, b"-v\0" as *const u8 as *const core::ffi::c_char) != 0 {
            options.verbose = 1 as core::ffi::c_int;
        } else if StringsEqual(arg, b"-c\0" as *const u8 as *const core::ffi::c_char)
            != 0
        {
            output_to_stdout = 1 as core::ffi::c_int;
        } else if StringsEqual(
            arg,
            b"--deflate\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
        {
            output_type = ZOPFLI_FORMAT_DEFLATE;
        } else if StringsEqual(arg, b"--zlib\0" as *const u8 as *const core::ffi::c_char)
            != 0
        {
            output_type = ZOPFLI_FORMAT_ZLIB;
        } else if StringsEqual(arg, b"--gzip\0" as *const u8 as *const core::ffi::c_char)
            != 0
        {
            output_type = ZOPFLI_FORMAT_GZIP;
        } else if !(StringsEqual(
            arg,
            b"--splitlast\0" as *const u8 as *const core::ffi::c_char,
        ) != 0)
        {
            if *arg.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                == '-' as i32
                && *arg.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                    == '-' as i32
                && *arg.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                    == 'i' as i32
                && *arg.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
                    >= '0' as i32
                && *arg.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
                    <= '9' as i32
            {
                options.numiterations = atoi(arg.offset(3 as core::ffi::c_int as isize));
            } else if StringsEqual(arg, b"-h\0" as *const u8 as *const core::ffi::c_char)
                != 0
            {
                fprintf(
                    stderr,
                    b"Usage: zopfli [OPTION]... FILE...\n  -h    gives this help\n  -c    write the result on standard output, instead of disk filename + '.gz'\n  -v    verbose mode\n  --i#  perform # iterations (default 15). More gives more compression but is slower. Examples: --i10, --i50, --i1000\n\0"
                        as *const u8 as *const core::ffi::c_char,
                );
                fprintf(
                    stderr,
                    b"  --gzip        output to gzip format (default)\n  --zlib        output to zlib format instead of gzip\n  --deflate     output to deflate format instead of gzip\n  --splitlast   ignored, left for backwards compatibility\n\0"
                        as *const u8 as *const core::ffi::c_char,
                );
                return 0 as core::ffi::c_int;
            }
        }
        i += 1;
    }
    if options.numiterations < 1 as core::ffi::c_int {
        fprintf(
            stderr,
            b"Error: must have 1 or more iterations\n\0" as *const u8
                as *const core::ffi::c_char,
        );
        return 0 as core::ffi::c_int;
    }
    i = 1 as core::ffi::c_int;
    while i < argc {
        if *(*argv.offset(i as isize)).offset(0 as core::ffi::c_int as isize)
            as core::ffi::c_int != '-' as i32
        {
            let mut outfilename: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
            filename = *argv.offset(i as isize);
            if output_to_stdout != 0 {
                outfilename = 0 as *mut core::ffi::c_char;
            } else if output_type as core::ffi::c_uint
                == ZOPFLI_FORMAT_GZIP as core::ffi::c_int as core::ffi::c_uint
            {
                outfilename = AddStrings(
                    filename,
                    b".gz\0" as *const u8 as *const core::ffi::c_char,
                );
            } else if output_type as core::ffi::c_uint
                == ZOPFLI_FORMAT_ZLIB as core::ffi::c_int as core::ffi::c_uint
            {
                outfilename = AddStrings(
                    filename,
                    b".zlib\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {
                if output_type as core::ffi::c_uint
                    == ZOPFLI_FORMAT_DEFLATE as core::ffi::c_int as core::ffi::c_uint
                {} else {
                    __assert_fail(
                        b"output_type == ZOPFLI_FORMAT_DEFLATE\0" as *const u8
                            as *const core::ffi::c_char,
                        b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/zopfli_bin.c\0"
                            as *const u8 as *const core::ffi::c_char,
                        202 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 23],
                            [core::ffi::c_char; 23],
                        >(*b"int main(int, char **)\0"))
                            .as_ptr(),
                    );
                }
                'c_2810: {
                    if output_type as core::ffi::c_uint
                        == ZOPFLI_FORMAT_DEFLATE as core::ffi::c_int as core::ffi::c_uint
                    {} else {
                        __assert_fail(
                            b"output_type == ZOPFLI_FORMAT_DEFLATE\0" as *const u8
                                as *const core::ffi::c_char,
                            b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/zopfli_bin.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            202 as core::ffi::c_uint,
                            (::core::mem::transmute::<
                                [u8; 23],
                                [core::ffi::c_char; 23],
                            >(*b"int main(int, char **)\0"))
                                .as_ptr(),
                        );
                    }
                };
                outfilename = AddStrings(
                    filename,
                    b".deflate\0" as *const u8 as *const core::ffi::c_char,
                );
            }
            if options.verbose != 0 && !outfilename.is_null() {
                fprintf(
                    stderr,
                    b"Saving to: %s\n\0" as *const u8 as *const core::ffi::c_char,
                    outfilename,
                );
            }
            CompressFile(&mut options, output_type, filename, outfilename);
            free(outfilename as *mut core::ffi::c_void);
        }
        i += 1;
    }
    if filename.is_null() {
        fprintf(
            stderr,
            b"Please provide filename\nFor help, type: %s -h\n\0" as *const u8
                as *const core::ffi::c_char,
            *argv.offset(0 as core::ffi::c_int as isize),
        );
    }
    return 0 as core::ffi::c_int;
}
pub fn main() {
    let mut args: Vec<*mut core::ffi::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as core::ffi::c_int,
                args.as_mut_ptr() as *mut *mut core::ffi::c_char,
            ) as i32,
        )
    }
}
