#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
unsafe fn main_0() -> libc::c_int {
    let N: libc::c_int = 2 as libc::c_int;
    let mut base: libc::c_int = 10 as libc::c_int;
    let mut c1: libc::c_int = 0 as libc::c_int;
    let mut c2: libc::c_int = 0 as libc::c_int;
    let mut k: libc::c_int = 0;
    k = 1 as libc::c_int;
    while (k as libc::c_double) < pow(base as libc::c_double, N as libc::c_double) {
        c1 += 1;
        c1;
        if k % (base - 1 as libc::c_int) == k * k % (base - 1 as libc::c_int) {
            c2 += 1;
            c2;
            printf(b"%d \0" as *const u8 as *const libc::c_char, k);
        }
        k += 1;
        k;
    }
    printf(
        b"\nTring %d numbers instead of %d numbers saves %f%%\n\0" as *const u8
            as *const libc::c_char,
        c2,
        c1,
        100.0f64 - 100.0f64 * c2 as libc::c_double / c1 as libc::c_double,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
