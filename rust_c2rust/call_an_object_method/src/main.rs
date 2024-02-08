#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct functionPair {
    pub x: libc::c_int,
    pub funcPtr: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
}
#[no_mangle]
pub unsafe extern "C" fn factorial(mut num: libc::c_int) -> libc::c_int {
    if num == 0 as libc::c_int || num == 1 as libc::c_int {
        return 1 as libc::c_int
    } else {
        return num * factorial(num - 1 as libc::c_int)
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut response: functionPair = functionPair {
        x: 0,
        funcPtr: None,
    };
    if argc != 2 as libc::c_int {
        return printf(
            b"Usage : %s <non negative integer>\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        )
    } else {
        response = {
            let mut init = functionPair {
                x: atoi(*argv.offset(1 as libc::c_int as isize)),
                funcPtr: Some(
                    factorial as unsafe extern "C" fn(libc::c_int) -> libc::c_int,
                ),
            };
            init
        };
        printf(
            b"\nFactorial of %d is %d\n\0" as *const u8 as *const libc::c_char,
            response.x,
            (response.funcPtr).expect("non-null function pointer")(response.x),
        );
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
