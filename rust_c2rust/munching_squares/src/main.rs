#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
pub unsafe extern "C" fn hue_to_rgb(
    mut hue: libc::c_double,
    mut sat: libc::c_double,
    mut p: *mut libc::c_uchar,
) {
    let mut x: libc::c_double = 0.;
    let mut c: libc::c_int = (255 as libc::c_int as libc::c_double * sat) as libc::c_int;
    hue /= 60 as libc::c_int as libc::c_double;
    x = (1 as libc::c_int as libc::c_double
        - fabs(
            fmod(hue, 2 as libc::c_int as libc::c_double)
                - 1 as libc::c_int as libc::c_double,
        )) * 255 as libc::c_int as libc::c_double;
    match hue as libc::c_int {
        0 => {
            *p.offset(0 as libc::c_int as isize) = c as libc::c_uchar;
            *p.offset(1 as libc::c_int as isize) = x as libc::c_uchar;
            *p.offset(2 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
            return;
        }
        1 => {
            *p.offset(0 as libc::c_int as isize) = x as libc::c_uchar;
            *p.offset(1 as libc::c_int as isize) = c as libc::c_uchar;
            *p.offset(2 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
            return;
        }
        2 => {
            *p.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
            *p.offset(1 as libc::c_int as isize) = c as libc::c_uchar;
            *p.offset(2 as libc::c_int as isize) = x as libc::c_uchar;
            return;
        }
        3 => {
            *p.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
            *p.offset(1 as libc::c_int as isize) = x as libc::c_uchar;
            *p.offset(2 as libc::c_int as isize) = c as libc::c_uchar;
            return;
        }
        4 => {
            *p.offset(0 as libc::c_int as isize) = x as libc::c_uchar;
            *p.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
            *p.offset(2 as libc::c_int as isize) = c as libc::c_uchar;
            return;
        }
        5 => {
            *p.offset(0 as libc::c_int as isize) = c as libc::c_uchar;
            *p.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
            *p.offset(2 as libc::c_int as isize) = x as libc::c_uchar;
            return;
        }
        _ => {}
    };
}
unsafe fn main_0() -> libc::c_int {
    let size: libc::c_int = 512 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut colors: *mut libc::c_uchar = malloc(
        (size * 3 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_uchar;
    let mut pix: *mut libc::c_uchar = malloc(
        (size * size * 3 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut fp: *mut FILE = 0 as *mut FILE;
    i = 0 as libc::c_int;
    while i < size {
        hue_to_rgb(
            i as libc::c_double * 240.0f64 / size as libc::c_double,
            i as libc::c_double * 1.0f64 / size as libc::c_double,
            colors.offset((3 as libc::c_int * i) as isize),
        );
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    p = pix;
    while i < size {
        j = 0 as libc::c_int;
        while j < size {
            memcpy(
                p as *mut libc::c_void,
                colors.offset(((i ^ j) * 3 as libc::c_int) as isize)
                    as *const libc::c_void,
                3 as libc::c_int as libc::c_ulong,
            );
            j += 1;
            j;
            p = p.offset(3 as libc::c_int as isize);
        }
        i += 1;
        i;
    }
    fp = fopen(
        b"xor.ppm\0" as *const u8 as *const libc::c_char,
        b"wb\0" as *const u8 as *const libc::c_char,
    );
    fprintf(fp, b"P6\n%d %d\n255\n\0" as *const u8 as *const libc::c_char, size, size);
    fwrite(
        pix as *const libc::c_void,
        (size * size * 3 as libc::c_int) as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        fp,
    );
    fclose(fp);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
