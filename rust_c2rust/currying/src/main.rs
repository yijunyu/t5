#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(c_variadic)]
use ::c2rust_out::*;
extern "C" {
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
pub unsafe extern "C" fn factorial(mut n: libc::c_int) -> libc::c_long {
    if n > 1 as libc::c_int {
        return n as libc::c_long * factorial(n - 1 as libc::c_int);
    }
    return 1 as libc::c_int as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn sumOfFactorials(
    mut num: libc::c_int,
    mut args: ...
) -> libc::c_long {
    let mut vaList: ::core::ffi::VaListImpl;
    let mut sum: libc::c_long = 0 as libc::c_int as libc::c_long;
    vaList = args.clone();
    loop {
        let fresh0 = num;
        num = num - 1;
        if !(fresh0 != 0) {
            break;
        }
        sum += factorial(vaList.arg::<libc::c_int>());
    }
    return sum;
}
unsafe fn main_0() -> libc::c_int {
    printf(
        b"\nSum of factorials of [1,5] : %ld\0" as *const u8 as *const libc::c_char,
        sumOfFactorials(
            5 as libc::c_int,
            1 as libc::c_int,
            2 as libc::c_int,
            3 as libc::c_int,
            4 as libc::c_int,
            5 as libc::c_int,
        ),
    );
    printf(
        b"\nSum of factorials of [3,5] : %ld\0" as *const u8 as *const libc::c_char,
        sumOfFactorials(
            3 as libc::c_int,
            3 as libc::c_int,
            4 as libc::c_int,
            5 as libc::c_int,
        ),
    );
    printf(
        b"\nSum of factorials of [1,3] : %ld\0" as *const u8 as *const libc::c_char,
        sumOfFactorials(
            3 as libc::c_int,
            1 as libc::c_int,
            2 as libc::c_int,
            3 as libc::c_int,
        ),
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
