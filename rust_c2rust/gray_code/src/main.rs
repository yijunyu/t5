#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gray_encode(mut n: libc::c_int) -> libc::c_int {
    return n ^ n >> 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gray_decode(mut n: libc::c_int) -> libc::c_int {
    let mut p: libc::c_int = n;
    loop {
        n >>= 1 as libc::c_int;
        if !(n != 0) {
            break;
        }
        p ^= n;
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn fmtbool(mut n: libc::c_int, mut buf: *mut libc::c_char) {
    let mut b: *mut libc::c_char = buf.offset(5 as libc::c_int as isize);
    *b = 0 as libc::c_int as libc::c_char;
    loop {
        b = b.offset(-1);
        *b = ('0' as i32 + (n & 1 as libc::c_int)) as libc::c_char;
        n >>= 1 as libc::c_int;
        if !(b != buf) {
            break;
        }
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut bi: [libc::c_char; 6] = [0; 6];
    let mut bg: [libc::c_char; 6] = [0; 6];
    let mut bb: [libc::c_char; 6] = [0; 6];
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        g = gray_encode(i);
        b = gray_decode(g);
        fmtbool(i, bi.as_mut_ptr());
        fmtbool(g, bg.as_mut_ptr());
        fmtbool(b, bb.as_mut_ptr());
        printf(
            b"%2d : %5s => %5s => %5s : %2d\n\0" as *const u8 as *const libc::c_char,
            i,
            bi.as_mut_ptr(),
            bg.as_mut_ptr(),
            bb.as_mut_ptr(),
            b,
        );
        i += 1;
        i;
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
