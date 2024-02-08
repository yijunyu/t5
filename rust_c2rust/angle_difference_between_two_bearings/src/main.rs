#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn atof(__nptr: *const libc::c_char) -> libc::c_double;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[no_mangle]
pub unsafe extern "C" fn processFile(mut name: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut records: libc::c_int = 0;
    let mut diff: libc::c_double = 0.;
    let mut b1: libc::c_double = 0.;
    let mut b2: libc::c_double = 0.;
    let mut fp: *mut FILE = fopen(name, b"r\0" as *const u8 as *const libc::c_char);
    fscanf(
        fp,
        b"%d\n\0" as *const u8 as *const libc::c_char,
        &mut records as *mut libc::c_int,
    );
    i = 0 as libc::c_int;
    while i < records {
        fscanf(
            fp,
            b"%lf%lf\0" as *const u8 as *const libc::c_char,
            &mut b1 as *mut libc::c_double,
            &mut b2 as *mut libc::c_double,
        );
        diff = fmod(b2 - b1, 360.0f64);
        printf(
            b"\nDifference between b2(%lf) and b1(%lf) is %lf\0" as *const u8
                as *const libc::c_char,
            b2,
            b1,
            if diff < -(180 as libc::c_int) as libc::c_double {
                diff + 360 as libc::c_int as libc::c_double
            } else if diff >= 180 as libc::c_int as libc::c_double {
                diff - 360 as libc::c_int as libc::c_double
            } else {
                diff
            },
        );
        i += 1;
        i;
    }
    fclose(fp);
}
unsafe fn main_0(
    mut argC: libc::c_int,
    mut argV: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut diff: libc::c_double = 0.;
    if argC < 2 as libc::c_int {
        printf(
            b"Usage : %s <bearings separated by a space OR full file name which contains the bearing list>\0"
                as *const u8 as *const libc::c_char,
            *argV.offset(0 as libc::c_int as isize),
        );
    } else if argC == 2 as libc::c_int {
        processFile(*argV.offset(1 as libc::c_int as isize));
    } else {
        diff = fmod(
            atof(*argV.offset(2 as libc::c_int as isize))
                - atof(*argV.offset(1 as libc::c_int as isize)),
            360.0f64,
        );
        printf(
            b"Difference between b2(%s) and b1(%s) is %lf\0" as *const u8
                as *const libc::c_char,
            *argV.offset(2 as libc::c_int as isize),
            *argV.offset(1 as libc::c_int as isize),
            if diff < -(180 as libc::c_int) as libc::c_double {
                diff + 360 as libc::c_int as libc::c_double
            } else if diff >= 180 as libc::c_int as libc::c_double {
                diff - 360 as libc::c_int as libc::c_double
            } else {
                diff
            },
        );
    }
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
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
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
