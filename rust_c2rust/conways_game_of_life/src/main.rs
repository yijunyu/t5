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
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn rand() -> libc::c_int;
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
pub unsafe extern "C" fn show(
    mut u: *mut libc::c_void,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    let vla = w as usize;
    let mut univ: *mut libc::c_int = u as *mut libc::c_int;
    printf(b"\x1B[H\0" as *const u8 as *const libc::c_char);
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < h {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < w {
            printf(
                if *univ.offset(y as isize * vla as isize).offset(x as isize) != 0 {
                    b"\x1B[07m  \x1B[m\0" as *const u8 as *const libc::c_char
                } else {
                    b"  \0" as *const u8 as *const libc::c_char
                },
            );
            x += 1;
            x;
        }
        printf(b"\x1B[E\0" as *const u8 as *const libc::c_char);
        y += 1;
        y;
    }
    fflush(stdout);
}
#[no_mangle]
pub unsafe extern "C" fn evolve(
    mut u: *mut libc::c_void,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    let vla = w as usize;
    let mut univ: *mut libc::c_uint = u as *mut libc::c_uint;
    let vla_0 = h as usize;
    let vla_1 = w as usize;
    let mut new: Vec::<libc::c_uint> = ::std::vec::from_elem(0, vla_0 * vla_1);
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < h {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < w {
            let mut n: libc::c_int = 0 as libc::c_int;
            let mut y1: libc::c_int = y - 1 as libc::c_int;
            while y1 <= y + 1 as libc::c_int {
                let mut x1: libc::c_int = x - 1 as libc::c_int;
                while x1 <= x + 1 as libc::c_int {
                    if *univ
                        .offset(((y1 + h) % h) as isize * vla as isize)
                        .offset(((x1 + w) % w) as isize) != 0
                    {
                        n += 1;
                        n;
                    }
                    x1 += 1;
                    x1;
                }
                y1 += 1;
                y1;
            }
            if *univ.offset(y as isize * vla as isize).offset(x as isize) != 0 {
                n -= 1;
                n;
            }
            *new
                .as_mut_ptr()
                .offset(y as isize * vla_1 as isize)
                .offset(
                    x as isize,
                ) = (n == 3 as libc::c_int
                || n == 2 as libc::c_int
                    && *univ.offset(y as isize * vla as isize).offset(x as isize) != 0)
                as libc::c_int as libc::c_uint;
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    let mut y_0: libc::c_int = 0 as libc::c_int;
    while y_0 < h {
        let mut x_0: libc::c_int = 0 as libc::c_int;
        while x_0 < w {
            *univ
                .offset(y_0 as isize * vla as isize)
                .offset(
                    x_0 as isize,
                ) = *new
                .as_mut_ptr()
                .offset(y_0 as isize * vla_1 as isize)
                .offset(x_0 as isize);
            x_0 += 1;
            x_0;
        }
        y_0 += 1;
        y_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn game(mut w: libc::c_int, mut h: libc::c_int) {
    let vla = h as usize;
    let vla_0 = w as usize;
    let mut univ: Vec::<libc::c_uint> = ::std::vec::from_elem(0, vla * vla_0);
    let mut x: libc::c_int = 0 as libc::c_int;
    while x < w {
        let mut y: libc::c_int = 0 as libc::c_int;
        while y < h {
            *univ
                .as_mut_ptr()
                .offset(y as isize * vla_0 as isize)
                .offset(
                    x as isize,
                ) = (if rand() < 2147483647 as libc::c_int / 10 as libc::c_int {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_uint;
            y += 1;
            y;
        }
        x += 1;
        x;
    }
    loop {
        show(univ.as_mut_ptr() as *mut libc::c_void, w, h);
        evolve(univ.as_mut_ptr() as *mut libc::c_void, w, h);
        usleep(200000 as libc::c_int as __useconds_t);
    };
}
unsafe fn main_0(mut c: libc::c_int, mut v: *mut *mut libc::c_char) -> libc::c_int {
    let mut w: libc::c_int = 0 as libc::c_int;
    let mut h: libc::c_int = 0 as libc::c_int;
    if c > 1 as libc::c_int {
        w = atoi(*v.offset(1 as libc::c_int as isize));
    }
    if c > 2 as libc::c_int {
        h = atoi(*v.offset(2 as libc::c_int as isize));
    }
    if w <= 0 as libc::c_int {
        w = 30 as libc::c_int;
    }
    if h <= 0 as libc::c_int {
        h = 30 as libc::c_int;
    }
    game(w, h);
    return 0;
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
