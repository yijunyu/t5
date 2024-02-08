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
    fn getchar() -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
pub type cell = libc::c_uchar;
#[no_mangle]
pub static mut dx: [libc::c_int; 8] = [
    -(2 as libc::c_int),
    -(2 as libc::c_int),
    -(1 as libc::c_int),
    1 as libc::c_int,
    2 as libc::c_int,
    2 as libc::c_int,
    1 as libc::c_int,
    -(1 as libc::c_int),
];
#[no_mangle]
pub static mut dy: [libc::c_int; 8] = [
    -(1 as libc::c_int),
    1 as libc::c_int,
    2 as libc::c_int,
    2 as libc::c_int,
    1 as libc::c_int,
    -(1 as libc::c_int),
    -(2 as libc::c_int),
    -(2 as libc::c_int),
];
#[no_mangle]
pub unsafe extern "C" fn init_board(
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut a: *mut *mut cell,
    mut b: *mut *mut cell,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut p: libc::c_int = w + 4 as libc::c_int;
    let mut q: libc::c_int = h + 4 as libc::c_int;
    let ref mut fresh0 = *a.offset(0 as libc::c_int as isize);
    *fresh0 = a.offset(q as isize) as *mut cell;
    let ref mut fresh1 = *b.offset(0 as libc::c_int as isize);
    *fresh1 = (*a.offset(0 as libc::c_int as isize)).offset(2 as libc::c_int as isize);
    i = 1 as libc::c_int;
    while i < q {
        let ref mut fresh2 = *a.offset(i as isize);
        *fresh2 = (*a.offset((i - 1 as libc::c_int) as isize)).offset(p as isize);
        let ref mut fresh3 = *b.offset(i as isize);
        *fresh3 = (*a.offset(i as isize)).offset(2 as libc::c_int as isize);
        i += 1;
        i;
    }
    memset(
        *a.offset(0 as libc::c_int as isize) as *mut libc::c_void,
        255 as libc::c_int,
        (p * q) as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < h {
        j = 0 as libc::c_int;
        while j < w {
            k = 0 as libc::c_int;
            while k < 8 as libc::c_int {
                x = j + dx[k as usize];
                y = i + dy[k as usize];
                if *(*b.offset((i + 2 as libc::c_int) as isize)).offset(j as isize)
                    as libc::c_int == 255 as libc::c_int
                {
                    *(*b.offset((i + 2 as libc::c_int) as isize))
                        .offset(j as isize) = 0 as libc::c_int as cell;
                }
                let ref mut fresh4 = *(*b.offset((i + 2 as libc::c_int) as isize))
                    .offset(j as isize);
                *fresh4 = (*fresh4 as libc::c_int
                    + (x >= 0 as libc::c_int && x < w && y >= 0 as libc::c_int && y < h)
                        as libc::c_int) as cell;
                k += 1;
                k;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn walk_board(
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut b: *mut *mut cell,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut nx: libc::c_int = 0;
    let mut ny: libc::c_int = 0;
    let mut least: libc::c_int = 0;
    let mut steps: libc::c_int = 0 as libc::c_int;
    printf(
        b"\x1B[H\x1B[J\x1B[%d;%dH\x1B[32m[]\x1B[m\0" as *const u8 as *const libc::c_char,
        y + 1 as libc::c_int,
        1 as libc::c_int + 2 as libc::c_int * x,
    );
    loop {
        *(*b.offset(y as isize)).offset(x as isize) = 255 as libc::c_int as cell;
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            let ref mut fresh5 = *(*b.offset((y + dy[i as usize]) as isize))
                .offset((x + dx[i as usize]) as isize);
            *fresh5 = (*fresh5).wrapping_sub(1);
            *fresh5;
            i += 1;
            i;
        }
        least = 255 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            if (*(*b.offset((y + dy[i as usize]) as isize))
                .offset((x + dx[i as usize]) as isize) as libc::c_int) < least
            {
                nx = x + dx[i as usize];
                ny = y + dy[i as usize];
                least = *(*b.offset(ny as isize)).offset(nx as isize) as libc::c_int;
            }
            i += 1;
            i;
        }
        if least > 7 as libc::c_int {
            printf(
                b"\x1B[%dH\0" as *const u8 as *const libc::c_char,
                h + 2 as libc::c_int,
            );
            return (steps == w * h - 1 as libc::c_int) as libc::c_int;
        }
        let fresh6 = steps;
        steps = steps + 1;
        if fresh6 != 0 {
            printf(
                b"\x1B[%d;%dH[]\0" as *const u8 as *const libc::c_char,
                y + 1 as libc::c_int,
                1 as libc::c_int + 2 as libc::c_int * x,
            );
        }
        x = nx;
        y = ny;
        printf(
            b"\x1B[%d;%dH\x1B[31m[]\x1B[m\0" as *const u8 as *const libc::c_char,
            y + 1 as libc::c_int,
            1 as libc::c_int + 2 as libc::c_int * x,
        );
        fflush(stdout);
        usleep(120000 as libc::c_int as __useconds_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn solve(mut w: libc::c_int, mut h: libc::c_int) -> libc::c_int {
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    let mut a: *mut *mut cell = 0 as *mut *mut cell;
    let mut b: *mut *mut cell = 0 as *mut *mut cell;
    a = malloc(
        (((w + 4 as libc::c_int) * (h + 4 as libc::c_int)) as libc::c_ulong)
            .wrapping_add(
                (::core::mem::size_of::<*mut cell>() as libc::c_ulong)
                    .wrapping_mul((h + 4 as libc::c_int) as libc::c_ulong),
            ),
    ) as *mut *mut cell;
    b = malloc(
        ((h + 4 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut cell>() as libc::c_ulong),
    ) as *mut *mut cell;
    loop {
        init_board(w, h, a, b);
        if walk_board(w, h, x, y, b.offset(2 as libc::c_int as isize)) != 0 {
            printf(b"Success!\n\0" as *const u8 as *const libc::c_char);
            return 1 as libc::c_int;
        }
        x += 1;
        if x >= w {
            x = 0 as libc::c_int;
            y += 1;
            y;
        }
        if y >= h {
            printf(b"Failed to find a solution\n\0" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int;
        }
        printf(
            b"Any key to try next start position\0" as *const u8 as *const libc::c_char,
        );
        getchar();
    };
}
unsafe fn main_0(mut c: libc::c_int, mut v: *mut *mut libc::c_char) -> libc::c_int {
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    if c < 2 as libc::c_int
        || {
            w = atoi(*v.offset(1 as libc::c_int as isize));
            w <= 0 as libc::c_int
        }
    {
        w = 8 as libc::c_int;
    }
    if c < 3 as libc::c_int
        || {
            h = atoi(*v.offset(2 as libc::c_int as isize));
            h <= 0 as libc::c_int
        }
    {
        h = w;
    }
    solve(w, h);
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
