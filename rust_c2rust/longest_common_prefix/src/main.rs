#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(c_variadic)]
use ::c2rust_out::*;
extern "C" {
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
#[no_mangle]
pub unsafe extern "C" fn lcp(mut num: libc::c_int, mut args: ...) -> *mut libc::c_char {
    let mut vaList: ::core::ffi::VaListImpl;
    let mut vaList2: ::core::ffi::VaListImpl;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut min: libc::c_int = 0;
    let mut dest: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut strings: *mut *mut libc::c_char = malloc(
        (num as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    vaList = args.clone();
    vaList2 = args.clone();
    i = 0 as libc::c_int;
    while i < num {
        len = strlen(vaList.arg::<*mut libc::c_char>()) as libc::c_int;
        let ref mut fresh0 = *strings.offset(i as isize);
        *fresh0 = malloc(
            ((len + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
        strcpy(*strings.offset(i as isize), vaList2.arg::<*mut libc::c_char>());
        if i == 0 as libc::c_int {
            min = len;
        } else if len < min {
            min = len;
        }
        i += 1;
        i;
    }
    if min == 0 as libc::c_int {
        return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    i = 0 as libc::c_int;
    while i < min {
        j = 1 as libc::c_int;
        while j < num {
            if *(*strings.offset(j as isize)).offset(i as isize) as libc::c_int
                != *(*strings.offset(0 as libc::c_int as isize)).offset(i as isize)
                    as libc::c_int
            {
                if i == 0 as libc::c_int {
                    return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char
                } else {
                    dest = malloc(
                        (i as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ),
                    ) as *mut libc::c_char;
                    strncpy(
                        dest,
                        *strings.offset(0 as libc::c_int as isize),
                        (i - 1 as libc::c_int) as libc::c_ulong,
                    );
                    return dest;
                }
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    dest = malloc(
        ((min + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    strncpy(dest, *strings.offset(0 as libc::c_int as isize), min as libc::c_ulong);
    return dest;
}
unsafe fn main_0() -> libc::c_int {
    printf(
        b"\nLongest common prefix : %s\0" as *const u8 as *const libc::c_char,
        lcp(
            3 as libc::c_int,
            b"interspecies\0" as *const u8 as *const libc::c_char,
            b"interstellar\0" as *const u8 as *const libc::c_char,
            b"interstate\0" as *const u8 as *const libc::c_char,
        ),
    );
    printf(
        b"\nLongest common prefix : %s\0" as *const u8 as *const libc::c_char,
        lcp(
            2 as libc::c_int,
            b"throne\0" as *const u8 as *const libc::c_char,
            b"throne\0" as *const u8 as *const libc::c_char,
        ),
    );
    printf(
        b"\nLongest common prefix : %s\0" as *const u8 as *const libc::c_char,
        lcp(
            2 as libc::c_int,
            b"throne\0" as *const u8 as *const libc::c_char,
            b"dungeon\0" as *const u8 as *const libc::c_char,
        ),
    );
    printf(
        b"\nLongest common prefix : %s\0" as *const u8 as *const libc::c_char,
        lcp(
            3 as libc::c_int,
            b"throne\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            b"throne\0" as *const u8 as *const libc::c_char,
        ),
    );
    printf(
        b"\nLongest common prefix : %s\0" as *const u8 as *const libc::c_char,
        lcp(1 as libc::c_int, b"cheese\0" as *const u8 as *const libc::c_char),
    );
    printf(
        b"\nLongest common prefix : %s\0" as *const u8 as *const libc::c_char,
        lcp(1 as libc::c_int, b"\0" as *const u8 as *const libc::c_char),
    );
    printf(
        b"\nLongest common prefix : %s\0" as *const u8 as *const libc::c_char,
        lcp(0 as libc::c_int, 0 as *mut libc::c_void),
    );
    printf(
        b"\nLongest common prefix : %s\0" as *const u8 as *const libc::c_char,
        lcp(
            2 as libc::c_int,
            b"prefix\0" as *const u8 as *const libc::c_char,
            b"suffix\0" as *const u8 as *const libc::c_char,
        ),
    );
    printf(
        b"\nLongest common prefix : %s\0" as *const u8 as *const libc::c_char,
        lcp(
            2 as libc::c_int,
            b"foo\0" as *const u8 as *const libc::c_char,
            b"foobar\0" as *const u8 as *const libc::c_char,
        ),
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
