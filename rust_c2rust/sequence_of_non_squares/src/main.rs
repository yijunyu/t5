#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(label_break_value)]
use ::c2rust_out::*;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
#[no_mangle]
pub unsafe extern "C" fn nonsqr(mut n: libc::c_int) -> libc::c_int {
    return n + (0.5f64 + sqrt(n as libc::c_double)) as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < 23 as libc::c_int {
        printf(b"%d \0" as *const u8 as *const libc::c_char, nonsqr(i));
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    i = 1 as libc::c_int;
    while i < 1000000 as libc::c_int {
        let mut j: libc::c_double = sqrt(nonsqr(i) as libc::c_double);
        if j != floor(j) {} else {
            __assert_fail(
                b"j != floor(j)\0" as *const u8 as *const libc::c_char,
                b"main.c\0" as *const u8 as *const libc::c_char,
                21 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_1861: {
            if j != floor(j) {} else {
                __assert_fail(
                    b"j != floor(j)\0" as *const u8 as *const libc::c_char,
                    b"main.c\0" as *const u8 as *const libc::c_char,
                    21 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
