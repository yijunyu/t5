#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapRange(
    mut a1: libc::c_double,
    mut a2: libc::c_double,
    mut b1: libc::c_double,
    mut b2: libc::c_double,
    mut s: libc::c_double,
) -> libc::c_double {
    return b1 + (s - a1) * (b2 - b1) / (a2 - a1);
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    puts(
        b"Mapping [0,10] to [-1,0] at intervals of 1:\0" as *const u8
            as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i <= 10 as libc::c_int {
        printf(
            b"f(%d) = %g\n\0" as *const u8 as *const libc::c_char,
            i,
            mapRange(
                0 as libc::c_int as libc::c_double,
                10 as libc::c_int as libc::c_double,
                -(1 as libc::c_int) as libc::c_double,
                0 as libc::c_int as libc::c_double,
                i as libc::c_double,
            ),
        );
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
