#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn leonardo(
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut step: libc::c_int,
    mut num: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut temp: libc::c_int = 0;
    printf(b"First 25 Leonardo numbers : \n\0" as *const u8 as *const libc::c_char);
    i = 1 as libc::c_int;
    while i <= num {
        if i == 1 as libc::c_int {
            printf(b" %d\0" as *const u8 as *const libc::c_char, a);
        } else if i == 2 as libc::c_int {
            printf(b" %d\0" as *const u8 as *const libc::c_char, b);
        } else {
            printf(b" %d\0" as *const u8 as *const libc::c_char, a + b + step);
            temp = a;
            a = b;
            b = temp + b + step;
        }
        i += 1;
        i;
    }
}
unsafe fn main_0() -> libc::c_int {
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut step: libc::c_int = 0;
    printf(
        b"Enter first two Leonardo numbers and increment step : \0" as *const u8
            as *const libc::c_char,
    );
    scanf(
        b"%d%d%d\0" as *const u8 as *const libc::c_char,
        &mut a as *mut libc::c_int,
        &mut b as *mut libc::c_int,
        &mut step as *mut libc::c_int,
    );
    leonardo(a, b, step, 25 as libc::c_int);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
