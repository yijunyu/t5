#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar(__c: libc::c_int) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
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
pub static mut w: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut h: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut pix: *mut libc::c_uchar = 0 as *const libc::c_uchar as *mut libc::c_uchar;
#[no_mangle]
pub unsafe extern "C" fn refresh(mut x: libc::c_int, mut y: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    printf(b"\x1B[H\0" as *const u8 as *const libc::c_char);
    k = 0 as libc::c_int;
    i = k;
    while i < h {
        j = 0 as libc::c_int;
        while j < w {
            putchar(
                if *pix.offset(k as isize) as libc::c_int != 0 {
                    '#' as i32
                } else {
                    ' ' as i32
                },
            );
            j += 1;
            j;
            k += 1;
            k;
        }
        putchar('\n' as i32);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn walk() {
    let mut dx: libc::c_int = 0 as libc::c_int;
    let mut dy: libc::c_int = 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut x: libc::c_int = w / 2 as libc::c_int;
    let mut y: libc::c_int = h / 2 as libc::c_int;
    pix = calloc(1 as libc::c_int as libc::c_ulong, (w * h) as libc::c_ulong)
        as *mut libc::c_uchar;
    printf(b"\x1B[H\x1B[J\0" as *const u8 as *const libc::c_char);
    loop {
        i = y * w + x;
        if *pix.offset(i as isize) != 0 {
            k = dx;
            dx = -dy;
            dy = k;
        } else {
            k = dy;
            dy = -dx;
            dx = k;
        }
        *pix
            .offset(
                i as isize,
            ) = (*pix.offset(i as isize) == 0) as libc::c_int as libc::c_uchar;
        printf(
            b"\x1B[%d;%dH%c\0" as *const u8 as *const libc::c_char,
            y + 1 as libc::c_int,
            x + 1 as libc::c_int,
            if *pix.offset(i as isize) as libc::c_int != 0 {
                '#' as i32
            } else {
                ' ' as i32
            },
        );
        x += dx;
        y += dy;
        k = 0 as libc::c_int;
        if x < 0 as libc::c_int {
            memmove(
                pix.offset(1 as libc::c_int as isize) as *mut libc::c_void,
                pix as *const libc::c_void,
                (w * h - 1 as libc::c_int) as libc::c_ulong,
            );
            i = 0 as libc::c_int;
            while i < w * h {
                *pix.offset(i as isize) = 0 as libc::c_int as libc::c_uchar;
                i += w;
            }
            x += 1;
            x;
            k = 1 as libc::c_int;
        } else if x >= w {
            memmove(
                pix as *mut libc::c_void,
                pix.offset(1 as libc::c_int as isize) as *const libc::c_void,
                (w * h - 1 as libc::c_int) as libc::c_ulong,
            );
            i = w - 1 as libc::c_int;
            while i < w * h {
                *pix.offset(i as isize) = 0 as libc::c_int as libc::c_uchar;
                i += w;
            }
            x -= 1;
            x;
            k = 1 as libc::c_int;
        }
        if y >= h {
            memmove(
                pix as *mut libc::c_void,
                pix.offset(w as isize) as *const libc::c_void,
                (w * (h - 1 as libc::c_int)) as libc::c_ulong,
            );
            memset(
                pix.offset((w * (h - 1 as libc::c_int)) as isize) as *mut libc::c_void,
                0 as libc::c_int,
                w as libc::c_ulong,
            );
            y -= 1;
            y;
            k = 1 as libc::c_int;
        } else if y < 0 as libc::c_int {
            memmove(
                pix.offset(w as isize) as *mut libc::c_void,
                pix as *const libc::c_void,
                (w * (h - 1 as libc::c_int)) as libc::c_ulong,
            );
            memset(pix as *mut libc::c_void, 0 as libc::c_int, w as libc::c_ulong);
            y += 1;
            y;
            k = 1 as libc::c_int;
        }
        if k != 0 {
            refresh(x, y);
        }
        printf(
            b"\x1B[%d;%dH\x1B[31m@\x1B[m\0" as *const u8 as *const libc::c_char,
            y + 1 as libc::c_int,
            x + 1 as libc::c_int,
        );
        fflush(stdout);
        usleep(10000 as libc::c_int as __useconds_t);
    };
}
unsafe fn main_0(mut c: libc::c_int, mut v: *mut *mut libc::c_char) -> libc::c_int {
    if c > 1 as libc::c_int {
        w = atoi(*v.offset(1 as libc::c_int as isize));
    }
    if c > 2 as libc::c_int {
        h = atoi(*v.offset(2 as libc::c_int as isize));
    }
    if w < 40 as libc::c_int {
        w = 40 as libc::c_int;
    }
    if h < 25 as libc::c_int {
        h = 25 as libc::c_int;
    }
    walk();
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
