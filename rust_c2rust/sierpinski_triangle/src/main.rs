#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn fabs(_: libc::c_double) -> libc::c_double;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rgb {
    pub r: libc::c_double,
    pub g: libc::c_double,
    pub b: libc::c_double,
}
#[no_mangle]
pub static mut x: libc::c_longlong = 0;
#[no_mangle]
pub static mut y: libc::c_longlong = 0;
#[no_mangle]
pub static mut dx: libc::c_longlong = 0;
#[no_mangle]
pub static mut dy: libc::c_longlong = 0;
#[no_mangle]
pub static mut scale: libc::c_longlong = 0;
#[no_mangle]
pub static mut clen: libc::c_longlong = 0;
#[no_mangle]
pub static mut cscale: libc::c_longlong = 0;
#[no_mangle]
pub static mut pix: *mut *mut rgb = 0 as *const *mut rgb as *mut *mut rgb;
#[no_mangle]
pub unsafe extern "C" fn sc_up() {
    scale *= 2 as libc::c_int as libc::c_longlong;
    x *= 2 as libc::c_int as libc::c_longlong;
    y *= 2 as libc::c_int as libc::c_longlong;
    cscale *= 3 as libc::c_int as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn h_rgb(mut x_0: libc::c_longlong, mut y_0: libc::c_longlong) {
    let mut p: *mut rgb = &mut *(*pix.offset(y_0 as isize)).offset(x_0 as isize)
        as *mut rgb;
    let mut h: libc::c_double = 6.0f64 * clen as libc::c_double
        / cscale as libc::c_double;
    let mut VAL: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut c: libc::c_double = 1 as libc::c_int as libc::c_double * VAL;
    let mut X: libc::c_double = c
        * (1 as libc::c_int as libc::c_double
            - fabs(
                fmod(h, 2 as libc::c_int as libc::c_double)
                    - 1 as libc::c_int as libc::c_double,
            ));
    match h as libc::c_int {
        0 => {
            (*p).r += c;
            (*p).g += X;
            return;
        }
        1 => {
            (*p).r += X;
            (*p).g += c;
            return;
        }
        2 => {
            (*p).g += c;
            (*p).b += X;
            return;
        }
        3 => {
            (*p).g += X;
            (*p).b += c;
            return;
        }
        4 => {
            (*p).r += X;
            (*p).b += c;
            return;
        }
        _ => {
            (*p).r += c;
            (*p).b += X;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn iter_string(mut str: *const libc::c_char, mut d: libc::c_int) {
    let mut len: libc::c_longlong = 0;
    while *str as libc::c_int != '\0' as i32 {
        let fresh0 = str;
        str = str.offset(1);
        match *fresh0 as libc::c_int {
            88 => {
                if d != 0 {
                    iter_string(
                        b"XHXVX\0" as *const u8 as *const libc::c_char,
                        d - 1 as libc::c_int,
                    );
                } else {
                    clen += 1;
                    clen;
                    h_rgb(x / scale, y / scale);
                    x += dx;
                    y -= dy;
                }
            }
            86 => {
                len = ((1 as libc::c_ulonglong) << d) as libc::c_longlong;
                loop {
                    let fresh1 = len;
                    len = len - 1;
                    if !(fresh1 != 0) {
                        break;
                    }
                    clen += 1;
                    clen;
                    h_rgb(x / scale, y / scale);
                    y += dy;
                }
            }
            72 => {
                len = ((1 as libc::c_ulonglong) << d) as libc::c_longlong;
                loop {
                    let fresh2 = len;
                    len = len - 1;
                    if !(fresh2 != 0) {
                        break;
                    }
                    clen += 1;
                    clen;
                    h_rgb(x / scale, y / scale);
                    x -= dx;
                }
            }
            _ => {}
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sierp(mut leng: libc::c_long, mut depth: libc::c_int) {
    let mut i: libc::c_long = 0;
    let mut h: libc::c_long = leng + 20 as libc::c_int as libc::c_long;
    let mut w: libc::c_long = leng + 20 as libc::c_int as libc::c_long;
    let mut buf: *mut rgb = malloc(
        (::core::mem::size_of::<rgb>() as libc::c_ulong)
            .wrapping_mul(w as libc::c_ulong)
            .wrapping_mul(h as libc::c_ulong),
    ) as *mut rgb;
    pix = malloc(
        (::core::mem::size_of::<*mut rgb>() as libc::c_ulong)
            .wrapping_mul(h as libc::c_ulong),
    ) as *mut *mut rgb;
    i = 0 as libc::c_int as libc::c_long;
    while i < h {
        let ref mut fresh3 = *pix.offset(i as isize);
        *fresh3 = buf.offset((w * i) as isize);
        i += 1;
        i;
    }
    memset(
        buf as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<rgb>() as libc::c_ulong)
            .wrapping_mul(w as libc::c_ulong)
            .wrapping_mul(h as libc::c_ulong),
    );
    y = 10 as libc::c_int as libc::c_longlong;
    x = y;
    dx = leng as libc::c_longlong;
    dy = leng as libc::c_longlong;
    scale = 1 as libc::c_int as libc::c_longlong;
    clen = 0 as libc::c_int as libc::c_longlong;
    cscale = 3 as libc::c_int as libc::c_longlong;
    i = 0 as libc::c_int as libc::c_long;
    while i < depth as libc::c_long {
        sc_up();
        i += 1;
        i;
    }
    iter_string(b"VXH\0" as *const u8 as *const libc::c_char, depth);
    let mut fpix: *mut libc::c_uchar = malloc(
        (w * h * 3 as libc::c_int as libc::c_long) as libc::c_ulong,
    ) as *mut libc::c_uchar;
    let mut maxv: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut dbuf: *mut libc::c_double = buf as *mut libc::c_double;
    i = 3 as libc::c_int as libc::c_long * w * h - 1 as libc::c_int as libc::c_long;
    while i >= 0 as libc::c_int as libc::c_long {
        if *dbuf.offset(i as isize) > maxv {
            maxv = *dbuf.offset(i as isize);
        }
        i -= 1;
        i;
    }
    i = 3 as libc::c_int as libc::c_long * h * w - 1 as libc::c_int as libc::c_long;
    while i >= 0 as libc::c_int as libc::c_long {
        *fpix
            .offset(
                i as isize,
            ) = (255 as libc::c_int as libc::c_double * *dbuf.offset(i as isize) / maxv)
            as libc::c_uchar;
        i -= 1;
        i;
    }
    printf(b"P6\n%ld %ld\n255\n\0" as *const u8 as *const libc::c_char, w, h);
    fflush(stdout);
    fwrite(
        fpix as *const libc::c_void,
        (h * w * 3 as libc::c_int as libc::c_long) as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stdout,
    );
}
unsafe fn main_0(mut c: libc::c_int, mut v: *mut *mut libc::c_char) -> libc::c_int {
    let mut size: libc::c_int = 0;
    let mut depth: libc::c_int = 0;
    depth = if c > 1 as libc::c_int {
        atoi(*v.offset(1 as libc::c_int as isize))
    } else {
        10 as libc::c_int
    };
    size = (1 as libc::c_int) << depth;
    fprintf(
        stderr,
        b"size: %d depth: %d\n\0" as *const u8 as *const libc::c_char,
        size,
        depth,
    );
    sierp(size as libc::c_long, depth + 2 as libc::c_int);
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
