#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
}
#[no_mangle]
pub unsafe extern "C" fn agm(
    mut a: libc::c_double,
    mut g: libc::c_double,
) -> libc::c_double {
    let mut iota: libc::c_double = 1.0E-16f64;
    let mut a1: libc::c_double = 0.;
    let mut g1: libc::c_double = 0.;
    if a * g < 0.0f64 {
        printf(
            b"arithmetic-geometric mean undefined when x*y<0\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    while fabs(a - g) > iota {
        a1 = (a + g) / 2.0f64;
        g1 = sqrt(a * g);
        a = a1;
        g = g1;
    }
    return a;
}
unsafe fn main_0() -> libc::c_int {
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    printf(b"Enter two numbers: \0" as *const u8 as *const libc::c_char);
    scanf(
        b"%lf%lf\0" as *const u8 as *const libc::c_char,
        &mut x as *mut libc::c_double,
        &mut y as *mut libc::c_double,
    );
    printf(
        b"The arithmetic-geometric mean is %lf\n\0" as *const u8 as *const libc::c_char,
        agm(x, y),
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
