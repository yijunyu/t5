#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut current: libc::c_int = 0 as libc::c_int;
    let mut square: libc::c_int = 0;
    loop {
        square = current * current;
        if !(square % 1000000 as libc::c_int != 269696 as libc::c_int
            && square < 2147483647 as libc::c_int)
        {
            break;
        }
        current += 1;
        current;
    }
    if square > 2147483647 as libc::c_int {
        printf(
            b"Condition not satisfied before INT_MAX reached.\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        printf(
            b"The smallest number whose square ends in 269696 is %d\n\0" as *const u8
                as *const libc::c_char,
            current,
        );
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
