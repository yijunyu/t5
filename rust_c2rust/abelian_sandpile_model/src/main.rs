#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
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
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut sandPileEdge: libc::c_int = 0;
    let mut centerPileHeight: libc::c_int = 0;
    let mut processAgain: libc::c_int = 1 as libc::c_int;
    let mut top: libc::c_int = 0;
    let mut down: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut sandPile: *mut *mut libc::c_int = 0 as *mut *mut libc::c_int;
    let mut fileName: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut colour: [libc::c_uchar; 3] = [0; 3];
    if argc != 3 as libc::c_int {
        printf(
            b"Usage: %s <Sand pile side> <Center pile height>\0" as *const u8
                as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        return 0 as libc::c_int;
    }
    sandPileEdge = atoi(*argv.offset(1 as libc::c_int as isize));
    centerPileHeight = atoi(*argv.offset(2 as libc::c_int as isize));
    if sandPileEdge <= 0 as libc::c_int || centerPileHeight <= 0 as libc::c_int {
        printf(
            b"Sand pile and center pile dimensions must be positive integers.\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    sandPile = malloc(
        (sandPileEdge as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < sandPileEdge {
        let ref mut fresh0 = *sandPile.offset(i as isize);
        *fresh0 = calloc(
            sandPileEdge as libc::c_ulong,
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        i += 1;
        i;
    }
    *(*sandPile.offset((sandPileEdge / 2 as libc::c_int) as isize))
        .offset((sandPileEdge / 2 as libc::c_int) as isize) = centerPileHeight;
    printf(b"Initial sand pile :\n\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < sandPileEdge {
        j = 0 as libc::c_int;
        while j < sandPileEdge {
            printf(
                b"%3d\0" as *const u8 as *const libc::c_char,
                *(*sandPile.offset(i as isize)).offset(j as isize),
            );
            j += 1;
            j;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
    while processAgain == 1 as libc::c_int {
        processAgain = 0 as libc::c_int;
        top = 0 as libc::c_int;
        down = 0 as libc::c_int;
        left = 0 as libc::c_int;
        right = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < sandPileEdge {
            j = 0 as libc::c_int;
            while j < sandPileEdge {
                if *(*sandPile.offset(i as isize)).offset(j as isize) >= 4 as libc::c_int
                {
                    if i - 1 as libc::c_int >= 0 as libc::c_int {
                        top = 1 as libc::c_int;
                        *(*sandPile.offset((i - 1 as libc::c_int) as isize))
                            .offset(j as isize) += 1 as libc::c_int;
                        if *(*sandPile.offset((i - 1 as libc::c_int) as isize))
                            .offset(j as isize) >= 4 as libc::c_int
                        {
                            processAgain = 1 as libc::c_int;
                        }
                    }
                    if (i + 1 as libc::c_int) < sandPileEdge {
                        down = 1 as libc::c_int;
                        *(*sandPile.offset((i + 1 as libc::c_int) as isize))
                            .offset(j as isize) += 1 as libc::c_int;
                        if *(*sandPile.offset((i + 1 as libc::c_int) as isize))
                            .offset(j as isize) >= 4 as libc::c_int
                        {
                            processAgain = 1 as libc::c_int;
                        }
                    }
                    if j - 1 as libc::c_int >= 0 as libc::c_int {
                        left = 1 as libc::c_int;
                        *(*sandPile.offset(i as isize))
                            .offset((j - 1 as libc::c_int) as isize) += 1 as libc::c_int;
                        if *(*sandPile.offset(i as isize))
                            .offset((j - 1 as libc::c_int) as isize) >= 4 as libc::c_int
                        {
                            processAgain = 1 as libc::c_int;
                        }
                    }
                    if (j + 1 as libc::c_int) < sandPileEdge {
                        right = 1 as libc::c_int;
                        *(*sandPile.offset(i as isize))
                            .offset((j + 1 as libc::c_int) as isize) += 1 as libc::c_int;
                        if *(*sandPile.offset(i as isize))
                            .offset((j + 1 as libc::c_int) as isize) >= 4 as libc::c_int
                        {
                            processAgain = 1 as libc::c_int;
                        }
                    }
                    *(*sandPile.offset(i as isize)).offset(j as isize)
                        -= top + down + left + right;
                    if *(*sandPile.offset(i as isize)).offset(j as isize)
                        >= 4 as libc::c_int
                    {
                        processAgain = 1 as libc::c_int;
                    }
                }
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
    printf(b"Final sand pile : \n\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < sandPileEdge {
        j = 0 as libc::c_int;
        while j < sandPileEdge {
            printf(
                b"%3d\0" as *const u8 as *const libc::c_char,
                *(*sandPile.offset(i as isize)).offset(j as isize),
            );
            j += 1;
            j;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
    fileName = malloc(
        (strlen(*argv.offset(1 as libc::c_int as isize)))
            .wrapping_add(strlen(*argv.offset(2 as libc::c_int as isize)))
            .wrapping_add(23 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    strcpy(fileName, b"Final_Sand_Pile_\0" as *const u8 as *const libc::c_char);
    strcat(fileName, *argv.offset(1 as libc::c_int as isize));
    strcat(fileName, b"_\0" as *const u8 as *const libc::c_char);
    strcat(fileName, *argv.offset(2 as libc::c_int as isize));
    strcat(fileName, b".ppm\0" as *const u8 as *const libc::c_char);
    let mut fp: *mut FILE = fopen(fileName, b"wb\0" as *const u8 as *const libc::c_char);
    fprintf(
        fp,
        b"P6\n%d %d\n255\n\0" as *const u8 as *const libc::c_char,
        sandPileEdge,
        sandPileEdge,
    );
    i = 0 as libc::c_int;
    while i < sandPileEdge {
        j = 0 as libc::c_int;
        while j < sandPileEdge {
            colour[0 as libc::c_int
                as usize] = ((*(*sandPile.offset(i as isize)).offset(j as isize) + i)
                % 256 as libc::c_int) as libc::c_uchar;
            colour[1 as libc::c_int
                as usize] = ((*(*sandPile.offset(i as isize)).offset(j as isize) + j)
                % 256 as libc::c_int) as libc::c_uchar;
            colour[2 as libc::c_int
                as usize] = ((*(*sandPile.offset(i as isize)).offset(j as isize) + i * j)
                % 256 as libc::c_int) as libc::c_uchar;
            fwrite(
                colour.as_mut_ptr() as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                3 as libc::c_int as libc::c_ulong,
                fp,
            );
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    fclose(fp);
    printf(
        b"\nImage file written to %s\n\0" as *const u8 as *const libc::c_char,
        fileName,
    );
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
