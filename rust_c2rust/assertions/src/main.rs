#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(label_break_value)]
use ::c2rust_out::*;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
unsafe fn main_0() -> libc::c_int {
    let mut a: libc::c_int = 0;
    if a == 42 as libc::c_int {} else {
        __assert_fail(
            b"a == 42\0" as *const u8 as *const libc::c_char,
            b"main.c\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_59: {
        if a == 42 as libc::c_int {} else {
            __assert_fail(
                b"a == 42\0" as *const u8 as *const libc::c_char,
                b"main.c\0" as *const u8 as *const libc::c_char,
                6 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
