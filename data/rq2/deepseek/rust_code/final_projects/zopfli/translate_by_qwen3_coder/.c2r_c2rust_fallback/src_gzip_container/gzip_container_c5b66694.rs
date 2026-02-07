#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn realloc(__ptr: *mut core::ffi::c_void, __size: size_t) -> *mut core::ffi::c_void;
}
pub type size_t = core::ffi::c_ulong;
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
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
static mut crc32_table: [core::ffi::c_ulong; 256] = [
    0 as core::ffi::c_uint as core::ffi::c_ulong,
    1996959894 as core::ffi::c_uint as core::ffi::c_ulong,
    3993919788 as core::ffi::c_uint as core::ffi::c_ulong,
    2567524794 as core::ffi::c_uint as core::ffi::c_ulong,
    124634137 as core::ffi::c_uint as core::ffi::c_ulong,
    1886057615 as core::ffi::c_uint as core::ffi::c_ulong,
    3915621685 as core::ffi::c_uint as core::ffi::c_ulong,
    2657392035 as core::ffi::c_uint as core::ffi::c_ulong,
    249268274 as core::ffi::c_uint as core::ffi::c_ulong,
    2044508324 as core::ffi::c_uint as core::ffi::c_ulong,
    3772115230 as core::ffi::c_uint as core::ffi::c_ulong,
    2547177864 as core::ffi::c_uint as core::ffi::c_ulong,
    162941995 as core::ffi::c_uint as core::ffi::c_ulong,
    2125561021 as core::ffi::c_uint as core::ffi::c_ulong,
    3887607047 as core::ffi::c_uint as core::ffi::c_ulong,
    2428444049 as core::ffi::c_uint as core::ffi::c_ulong,
    498536548 as core::ffi::c_uint as core::ffi::c_ulong,
    1789927666 as core::ffi::c_uint as core::ffi::c_ulong,
    4089016648 as core::ffi::c_uint as core::ffi::c_ulong,
    2227061214 as core::ffi::c_uint as core::ffi::c_ulong,
    450548861 as core::ffi::c_uint as core::ffi::c_ulong,
    1843258603 as core::ffi::c_uint as core::ffi::c_ulong,
    4107580753 as core::ffi::c_uint as core::ffi::c_ulong,
    2211677639 as core::ffi::c_uint as core::ffi::c_ulong,
    325883990 as core::ffi::c_uint as core::ffi::c_ulong,
    1684777152 as core::ffi::c_uint as core::ffi::c_ulong,
    4251122042 as core::ffi::c_uint as core::ffi::c_ulong,
    2321926636 as core::ffi::c_uint as core::ffi::c_ulong,
    335633487 as core::ffi::c_uint as core::ffi::c_ulong,
    1661365465 as core::ffi::c_uint as core::ffi::c_ulong,
    4195302755 as core::ffi::c_uint as core::ffi::c_ulong,
    2366115317 as core::ffi::c_uint as core::ffi::c_ulong,
    997073096 as core::ffi::c_uint as core::ffi::c_ulong,
    1281953886 as core::ffi::c_uint as core::ffi::c_ulong,
    3579855332 as core::ffi::c_uint as core::ffi::c_ulong,
    2724688242 as core::ffi::c_uint as core::ffi::c_ulong,
    1006888145 as core::ffi::c_uint as core::ffi::c_ulong,
    1258607687 as core::ffi::c_uint as core::ffi::c_ulong,
    3524101629 as core::ffi::c_uint as core::ffi::c_ulong,
    2768942443 as core::ffi::c_uint as core::ffi::c_ulong,
    901097722 as core::ffi::c_uint as core::ffi::c_ulong,
    1119000684 as core::ffi::c_uint as core::ffi::c_ulong,
    3686517206 as core::ffi::c_uint as core::ffi::c_ulong,
    2898065728 as core::ffi::c_uint as core::ffi::c_ulong,
    853044451 as core::ffi::c_uint as core::ffi::c_ulong,
    1172266101 as core::ffi::c_uint as core::ffi::c_ulong,
    3705015759 as core::ffi::c_uint as core::ffi::c_ulong,
    2882616665 as core::ffi::c_uint as core::ffi::c_ulong,
    651767980 as core::ffi::c_uint as core::ffi::c_ulong,
    1373503546 as core::ffi::c_uint as core::ffi::c_ulong,
    3369554304 as core::ffi::c_uint as core::ffi::c_ulong,
    3218104598 as core::ffi::c_uint as core::ffi::c_ulong,
    565507253 as core::ffi::c_uint as core::ffi::c_ulong,
    1454621731 as core::ffi::c_uint as core::ffi::c_ulong,
    3485111705 as core::ffi::c_uint as core::ffi::c_ulong,
    3099436303 as core::ffi::c_uint as core::ffi::c_ulong,
    671266974 as core::ffi::c_uint as core::ffi::c_ulong,
    1594198024 as core::ffi::c_uint as core::ffi::c_ulong,
    3322730930 as core::ffi::c_uint as core::ffi::c_ulong,
    2970347812 as core::ffi::c_uint as core::ffi::c_ulong,
    795835527 as core::ffi::c_uint as core::ffi::c_ulong,
    1483230225 as core::ffi::c_uint as core::ffi::c_ulong,
    3244367275 as core::ffi::c_uint as core::ffi::c_ulong,
    3060149565 as core::ffi::c_uint as core::ffi::c_ulong,
    1994146192 as core::ffi::c_uint as core::ffi::c_ulong,
    31158534 as core::ffi::c_uint as core::ffi::c_ulong,
    2563907772 as core::ffi::c_uint as core::ffi::c_ulong,
    4023717930 as core::ffi::c_uint as core::ffi::c_ulong,
    1907459465 as core::ffi::c_uint as core::ffi::c_ulong,
    112637215 as core::ffi::c_uint as core::ffi::c_ulong,
    2680153253 as core::ffi::c_uint as core::ffi::c_ulong,
    3904427059 as core::ffi::c_uint as core::ffi::c_ulong,
    2013776290 as core::ffi::c_uint as core::ffi::c_ulong,
    251722036 as core::ffi::c_uint as core::ffi::c_ulong,
    2517215374 as core::ffi::c_uint as core::ffi::c_ulong,
    3775830040 as core::ffi::c_uint as core::ffi::c_ulong,
    2137656763 as core::ffi::c_uint as core::ffi::c_ulong,
    141376813 as core::ffi::c_uint as core::ffi::c_ulong,
    2439277719 as core::ffi::c_uint as core::ffi::c_ulong,
    3865271297 as core::ffi::c_uint as core::ffi::c_ulong,
    1802195444 as core::ffi::c_uint as core::ffi::c_ulong,
    476864866 as core::ffi::c_uint as core::ffi::c_ulong,
    2238001368 as core::ffi::c_uint as core::ffi::c_ulong,
    4066508878 as core::ffi::c_uint as core::ffi::c_ulong,
    1812370925 as core::ffi::c_uint as core::ffi::c_ulong,
    453092731 as core::ffi::c_uint as core::ffi::c_ulong,
    2181625025 as core::ffi::c_uint as core::ffi::c_ulong,
    4111451223 as core::ffi::c_uint as core::ffi::c_ulong,
    1706088902 as core::ffi::c_uint as core::ffi::c_ulong,
    314042704 as core::ffi::c_uint as core::ffi::c_ulong,
    2344532202 as core::ffi::c_uint as core::ffi::c_ulong,
    4240017532 as core::ffi::c_uint as core::ffi::c_ulong,
    1658658271 as core::ffi::c_uint as core::ffi::c_ulong,
    366619977 as core::ffi::c_uint as core::ffi::c_ulong,
    2362670323 as core::ffi::c_uint as core::ffi::c_ulong,
    4224994405 as core::ffi::c_uint as core::ffi::c_ulong,
    1303535960 as core::ffi::c_uint as core::ffi::c_ulong,
    984961486 as core::ffi::c_uint as core::ffi::c_ulong,
    2747007092 as core::ffi::c_uint as core::ffi::c_ulong,
    3569037538 as core::ffi::c_uint as core::ffi::c_ulong,
    1256170817 as core::ffi::c_uint as core::ffi::c_ulong,
    1037604311 as core::ffi::c_uint as core::ffi::c_ulong,
    2765210733 as core::ffi::c_uint as core::ffi::c_ulong,
    3554079995 as core::ffi::c_uint as core::ffi::c_ulong,
    1131014506 as core::ffi::c_uint as core::ffi::c_ulong,
    879679996 as core::ffi::c_uint as core::ffi::c_ulong,
    2909243462 as core::ffi::c_uint as core::ffi::c_ulong,
    3663771856 as core::ffi::c_uint as core::ffi::c_ulong,
    1141124467 as core::ffi::c_uint as core::ffi::c_ulong,
    855842277 as core::ffi::c_uint as core::ffi::c_ulong,
    2852801631 as core::ffi::c_uint as core::ffi::c_ulong,
    3708648649 as core::ffi::c_uint as core::ffi::c_ulong,
    1342533948 as core::ffi::c_uint as core::ffi::c_ulong,
    654459306 as core::ffi::c_uint as core::ffi::c_ulong,
    3188396048 as core::ffi::c_uint as core::ffi::c_ulong,
    3373015174 as core::ffi::c_uint as core::ffi::c_ulong,
    1466479909 as core::ffi::c_uint as core::ffi::c_ulong,
    544179635 as core::ffi::c_uint as core::ffi::c_ulong,
    3110523913 as core::ffi::c_uint as core::ffi::c_ulong,
    3462522015 as core::ffi::c_uint as core::ffi::c_ulong,
    1591671054 as core::ffi::c_uint as core::ffi::c_ulong,
    702138776 as core::ffi::c_uint as core::ffi::c_ulong,
    2966460450 as core::ffi::c_uint as core::ffi::c_ulong,
    3352799412 as core::ffi::c_uint as core::ffi::c_ulong,
    1504918807 as core::ffi::c_uint as core::ffi::c_ulong,
    783551873 as core::ffi::c_uint as core::ffi::c_ulong,
    3082640443 as core::ffi::c_uint as core::ffi::c_ulong,
    3233442989 as core::ffi::c_uint as core::ffi::c_ulong,
    3988292384 as core::ffi::c_uint as core::ffi::c_ulong,
    2596254646 as core::ffi::c_uint as core::ffi::c_ulong,
    62317068 as core::ffi::c_uint as core::ffi::c_ulong,
    1957810842 as core::ffi::c_uint as core::ffi::c_ulong,
    3939845945 as core::ffi::c_uint as core::ffi::c_ulong,
    2647816111 as core::ffi::c_uint as core::ffi::c_ulong,
    81470997 as core::ffi::c_uint as core::ffi::c_ulong,
    1943803523 as core::ffi::c_uint as core::ffi::c_ulong,
    3814918930 as core::ffi::c_uint as core::ffi::c_ulong,
    2489596804 as core::ffi::c_uint as core::ffi::c_ulong,
    225274430 as core::ffi::c_uint as core::ffi::c_ulong,
    2053790376 as core::ffi::c_uint as core::ffi::c_ulong,
    3826175755 as core::ffi::c_uint as core::ffi::c_ulong,
    2466906013 as core::ffi::c_uint as core::ffi::c_ulong,
    167816743 as core::ffi::c_uint as core::ffi::c_ulong,
    2097651377 as core::ffi::c_uint as core::ffi::c_ulong,
    4027552580 as core::ffi::c_uint as core::ffi::c_ulong,
    2265490386 as core::ffi::c_uint as core::ffi::c_ulong,
    503444072 as core::ffi::c_uint as core::ffi::c_ulong,
    1762050814 as core::ffi::c_uint as core::ffi::c_ulong,
    4150417245 as core::ffi::c_uint as core::ffi::c_ulong,
    2154129355 as core::ffi::c_uint as core::ffi::c_ulong,
    426522225 as core::ffi::c_uint as core::ffi::c_ulong,
    1852507879 as core::ffi::c_uint as core::ffi::c_ulong,
    4275313526 as core::ffi::c_uint as core::ffi::c_ulong,
    2312317920 as core::ffi::c_uint as core::ffi::c_ulong,
    282753626 as core::ffi::c_uint as core::ffi::c_ulong,
    1742555852 as core::ffi::c_uint as core::ffi::c_ulong,
    4189708143 as core::ffi::c_uint as core::ffi::c_ulong,
    2394877945 as core::ffi::c_uint as core::ffi::c_ulong,
    397917763 as core::ffi::c_uint as core::ffi::c_ulong,
    1622183637 as core::ffi::c_uint as core::ffi::c_ulong,
    3604390888 as core::ffi::c_uint as core::ffi::c_ulong,
    2714866558 as core::ffi::c_uint as core::ffi::c_ulong,
    953729732 as core::ffi::c_uint as core::ffi::c_ulong,
    1340076626 as core::ffi::c_uint as core::ffi::c_ulong,
    3518719985 as core::ffi::c_uint as core::ffi::c_ulong,
    2797360999 as core::ffi::c_uint as core::ffi::c_ulong,
    1068828381 as core::ffi::c_uint as core::ffi::c_ulong,
    1219638859 as core::ffi::c_uint as core::ffi::c_ulong,
    3624741850 as core::ffi::c_uint as core::ffi::c_ulong,
    2936675148 as core::ffi::c_uint as core::ffi::c_ulong,
    906185462 as core::ffi::c_uint as core::ffi::c_ulong,
    1090812512 as core::ffi::c_uint as core::ffi::c_ulong,
    3747672003 as core::ffi::c_uint as core::ffi::c_ulong,
    2825379669 as core::ffi::c_uint as core::ffi::c_ulong,
    829329135 as core::ffi::c_uint as core::ffi::c_ulong,
    1181335161 as core::ffi::c_uint as core::ffi::c_ulong,
    3412177804 as core::ffi::c_uint as core::ffi::c_ulong,
    3160834842 as core::ffi::c_uint as core::ffi::c_ulong,
    628085408 as core::ffi::c_uint as core::ffi::c_ulong,
    1382605366 as core::ffi::c_uint as core::ffi::c_ulong,
    3423369109 as core::ffi::c_uint as core::ffi::c_ulong,
    3138078467 as core::ffi::c_uint as core::ffi::c_ulong,
    570562233 as core::ffi::c_uint as core::ffi::c_ulong,
    1426400815 as core::ffi::c_uint as core::ffi::c_ulong,
    3317316542 as core::ffi::c_uint as core::ffi::c_ulong,
    2998733608 as core::ffi::c_uint as core::ffi::c_ulong,
    733239954 as core::ffi::c_uint as core::ffi::c_ulong,
    1555261956 as core::ffi::c_uint as core::ffi::c_ulong,
    3268935591 as core::ffi::c_uint as core::ffi::c_ulong,
    3050360625 as core::ffi::c_uint as core::ffi::c_ulong,
    752459403 as core::ffi::c_uint as core::ffi::c_ulong,
    1541320221 as core::ffi::c_uint as core::ffi::c_ulong,
    2607071920 as core::ffi::c_uint as core::ffi::c_ulong,
    3965973030 as core::ffi::c_uint as core::ffi::c_ulong,
    1969922972 as core::ffi::c_uint as core::ffi::c_ulong,
    40735498 as core::ffi::c_uint as core::ffi::c_ulong,
    2617837225 as core::ffi::c_uint as core::ffi::c_ulong,
    3943577151 as core::ffi::c_uint as core::ffi::c_ulong,
    1913087877 as core::ffi::c_uint as core::ffi::c_ulong,
    83908371 as core::ffi::c_uint as core::ffi::c_ulong,
    2512341634 as core::ffi::c_uint as core::ffi::c_ulong,
    3803740692 as core::ffi::c_uint as core::ffi::c_ulong,
    2075208622 as core::ffi::c_uint as core::ffi::c_ulong,
    213261112 as core::ffi::c_uint as core::ffi::c_ulong,
    2463272603 as core::ffi::c_uint as core::ffi::c_ulong,
    3855990285 as core::ffi::c_uint as core::ffi::c_ulong,
    2094854071 as core::ffi::c_uint as core::ffi::c_ulong,
    198958881 as core::ffi::c_uint as core::ffi::c_ulong,
    2262029012 as core::ffi::c_uint as core::ffi::c_ulong,
    4057260610 as core::ffi::c_uint as core::ffi::c_ulong,
    1759359992 as core::ffi::c_uint as core::ffi::c_ulong,
    534414190 as core::ffi::c_uint as core::ffi::c_ulong,
    2176718541 as core::ffi::c_uint as core::ffi::c_ulong,
    4139329115 as core::ffi::c_uint as core::ffi::c_ulong,
    1873836001 as core::ffi::c_uint as core::ffi::c_ulong,
    414664567 as core::ffi::c_uint as core::ffi::c_ulong,
    2282248934 as core::ffi::c_uint as core::ffi::c_ulong,
    4279200368 as core::ffi::c_uint as core::ffi::c_ulong,
    1711684554 as core::ffi::c_uint as core::ffi::c_ulong,
    285281116 as core::ffi::c_uint as core::ffi::c_ulong,
    2405801727 as core::ffi::c_uint as core::ffi::c_ulong,
    4167216745 as core::ffi::c_uint as core::ffi::c_ulong,
    1634467795 as core::ffi::c_uint as core::ffi::c_ulong,
    376229701 as core::ffi::c_uint as core::ffi::c_ulong,
    2685067896 as core::ffi::c_uint as core::ffi::c_ulong,
    3608007406 as core::ffi::c_uint as core::ffi::c_ulong,
    1308918612 as core::ffi::c_uint as core::ffi::c_ulong,
    956543938 as core::ffi::c_uint as core::ffi::c_ulong,
    2808555105 as core::ffi::c_uint as core::ffi::c_ulong,
    3495958263 as core::ffi::c_uint as core::ffi::c_ulong,
    1231636301 as core::ffi::c_uint as core::ffi::c_ulong,
    1047427035 as core::ffi::c_uint as core::ffi::c_ulong,
    2932959818 as core::ffi::c_uint as core::ffi::c_ulong,
    3654703836 as core::ffi::c_uint as core::ffi::c_ulong,
    1088359270 as core::ffi::c_uint as core::ffi::c_ulong,
    936918000 as core::ffi::c_uint as core::ffi::c_ulong,
    2847714899 as core::ffi::c_uint as core::ffi::c_ulong,
    3736837829 as core::ffi::c_uint as core::ffi::c_ulong,
    1202900863 as core::ffi::c_uint as core::ffi::c_ulong,
    817233897 as core::ffi::c_uint as core::ffi::c_ulong,
    3183342108 as core::ffi::c_uint as core::ffi::c_ulong,
    3401237130 as core::ffi::c_uint as core::ffi::c_ulong,
    1404277552 as core::ffi::c_uint as core::ffi::c_ulong,
    615818150 as core::ffi::c_uint as core::ffi::c_ulong,
    3134207493 as core::ffi::c_uint as core::ffi::c_ulong,
    3453421203 as core::ffi::c_uint as core::ffi::c_ulong,
    1423857449 as core::ffi::c_uint as core::ffi::c_ulong,
    601450431 as core::ffi::c_uint as core::ffi::c_ulong,
    3009837614 as core::ffi::c_uint as core::ffi::c_ulong,
    3294710456 as core::ffi::c_uint as core::ffi::c_ulong,
    1567103746 as core::ffi::c_uint as core::ffi::c_ulong,
    711928724 as core::ffi::c_uint as core::ffi::c_ulong,
    3020668471 as core::ffi::c_uint as core::ffi::c_ulong,
    3272380065 as core::ffi::c_uint as core::ffi::c_ulong,
    1510334235 as core::ffi::c_uint as core::ffi::c_ulong,
    755167117 as core::ffi::c_uint as core::ffi::c_ulong,
];
unsafe extern "C" fn CRC(
    mut data: *const core::ffi::c_uchar,
    mut size: size_t,
) -> core::ffi::c_ulong {
    let mut result: core::ffi::c_ulong = 0xffffffff as core::ffi::c_ulong;
    while size > 0 as core::ffi::c_ulong {
        let fresh0 = data;
        data = data.offset(1);
        result = crc32_table[((result ^ *fresh0 as core::ffi::c_ulong)
            & 0xff as core::ffi::c_ulong) as usize] ^ result >> 8 as core::ffi::c_int;
        size = size.wrapping_sub(1);
    }
    return result ^ 0xffffffff as core::ffi::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliGzipCompress(
    mut options: *const ZopfliOptions,
    mut in_0: *const core::ffi::c_uchar,
    mut insize: size_t,
    mut out: *mut *mut core::ffi::c_uchar,
    mut outsize: *mut size_t,
) {
    let mut crcvalue: core::ffi::c_ulong = CRC(in_0, insize);
    let mut bp: core::ffi::c_uchar = 0 as core::ffi::c_uchar;
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_uchar>() as size_t),
            )
        }) as *mut core::ffi::c_uchar;
    }
    *(*out).offset(*outsize as isize) = 31 as core::ffi::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_uchar>() as size_t),
            )
        }) as *mut core::ffi::c_uchar;
    }
    *(*out).offset(*outsize as isize) = 139 as core::ffi::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_uchar>() as size_t),
            )
        }) as *mut core::ffi::c_uchar;
    }
    *(*out).offset(*outsize as isize) = 8 as core::ffi::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_uchar>() as size_t),
            )
        }) as *mut core::ffi::c_uchar;
    }
    *(*out).offset(*outsize as isize) = 0 as core::ffi::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_uchar>() as size_t),
            )
        }) as *mut core::ffi::c_uchar;
    }
    *(*out).offset(*outsize as isize) = 0 as core::ffi::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_uchar>() as size_t),
            )
        }) as *mut core::ffi::c_uchar;
    }
    *(*out).offset(*outsize as isize) = 0 as core::ffi::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_uchar>() as size_t),
            )
        }) as *mut core::ffi::c_uchar;
    }
    *(*out).offset(*outsize as isize) = 0 as core::ffi::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_uchar>() as size_t),
            )
        }) as *mut core::ffi::c_uchar;
    }
    *(*out).offset(*outsize as isize) = 0 as core::ffi::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_uchar>() as size_t),
            )
        }) as *mut core::ffi::c_uchar;
    }
    *(*out).offset(*outsize as isize) = 2 as core::ffi::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_uchar>() as size_t),
            )
        }) as *mut core::ffi::c_uchar;
    }
    *(*out).offset(*outsize as isize) = 3 as core::ffi::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
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
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_uchar>() as size_t),
            )
        }) as *mut core::ffi::c_uchar;
    }
    *(*out).offset(*outsize as isize) = crcvalue.wrapping_rem(256 as core::ffi::c_ulong)
        as core::ffi::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_uchar>() as size_t),
            )
        }) as *mut core::ffi::c_uchar;
    }
    *(*out).offset(*outsize as isize) = (crcvalue >> 8 as core::ffi::c_int)
        .wrapping_rem(256 as core::ffi::c_ulong) as core::ffi::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_uchar>() as size_t),
            )
        }) as *mut core::ffi::c_uchar;
    }
    *(*out).offset(*outsize as isize) = (crcvalue >> 16 as core::ffi::c_int)
        .wrapping_rem(256 as core::ffi::c_ulong) as core::ffi::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_uchar>() as size_t),
            )
        }) as *mut core::ffi::c_uchar;
    }
    *(*out).offset(*outsize as isize) = (crcvalue >> 24 as core::ffi::c_int)
        .wrapping_rem(256 as core::ffi::c_ulong) as core::ffi::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_uchar>() as size_t),
            )
        }) as *mut core::ffi::c_uchar;
    }
    *(*out).offset(*outsize as isize) = (insize as core::ffi::c_ulong)
        .wrapping_rem(256 as core::ffi::c_ulong) as core::ffi::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_uchar>() as size_t),
            )
        }) as *mut core::ffi::c_uchar;
    }
    *(*out).offset(*outsize as isize) = (insize as core::ffi::c_ulong
        >> 8 as core::ffi::c_int)
        .wrapping_rem(256 as core::ffi::c_ulong) as core::ffi::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_uchar>() as size_t),
            )
        }) as *mut core::ffi::c_uchar;
    }
    *(*out).offset(*outsize as isize) = (insize as core::ffi::c_ulong
        >> 16 as core::ffi::c_int)
        .wrapping_rem(256 as core::ffi::c_ulong) as core::ffi::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_uchar>() as size_t),
            )
        }) as *mut core::ffi::c_uchar;
    }
    *(*out).offset(*outsize as isize) = (insize as core::ffi::c_ulong
        >> 24 as core::ffi::c_int)
        .wrapping_rem(256 as core::ffi::c_ulong) as core::ffi::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if (*options).verbose != 0 {
        fprintf(
            stderr,
            b"Original Size: %d, Gzip: %d, Compression: %f%% Removed\n\0" as *const u8
                as *const core::ffi::c_char,
            insize as core::ffi::c_int,
            *outsize as core::ffi::c_int,
            100.0f64 * insize.wrapping_sub(*outsize) as core::ffi::c_double
                / insize as core::ffi::c_double,
        );
    }
}
