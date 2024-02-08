#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut a2: libc::c_int = 0;
    let mut s: libc::c_int = 3 as libc::c_int;
    let mut s1: libc::c_int = 0;
    let mut s2: libc::c_int = 0;
    let mut r: [libc::c_int; 2201] = [0; 2201];
    memset(
        r.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_int; 2201]>() as libc::c_ulong,
    );
    let mut ab: *mut libc::c_int = calloc(
        (2200 as libc::c_int * 2200 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
            as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    a = 1 as libc::c_int;
    while a <= 2200 as libc::c_int {
        a2 = a * a;
        b = a;
        while b <= 2200 as libc::c_int {
            *ab.offset((a2 + b * b) as isize) = 1 as libc::c_int;
            b += 1;
            b;
        }
        a += 1;
        a;
    }
    c = 1 as libc::c_int;
    while c <= 2200 as libc::c_int {
        s1 = s;
        s += 2 as libc::c_int;
        s2 = s;
        d = c + 1 as libc::c_int;
        while d <= 2200 as libc::c_int {
            if *ab.offset(s1 as isize) != 0 {
                r[d as usize] = 1 as libc::c_int;
            }
            s1 += s2;
            s2 += 2 as libc::c_int;
            d += 1;
            d;
        }
        c += 1;
        c;
    }
    d = 1 as libc::c_int;
    while d <= 2200 as libc::c_int {
        if r[d as usize] == 0 {
            printf(b"%d \0" as *const u8 as *const libc::c_char, d);
        }
        d += 1;
        d;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    free(ab as *mut libc::c_void);
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
