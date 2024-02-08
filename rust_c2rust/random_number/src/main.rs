#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn rand() -> libc::c_int;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn drand() -> libc::c_double {
    return (rand() as libc::c_double + 1.0f64)
        / (2147483647 as libc::c_int as libc::c_double + 1.0f64);
}
#[no_mangle]
pub unsafe extern "C" fn random_normal() -> libc::c_double {
    return sqrt(-(2 as libc::c_int) as libc::c_double * log(drand()))
        * cos(2 as libc::c_int as libc::c_double * 3.14159265358979323846f64 * drand());
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut rands: [libc::c_double; 1000] = [0.; 1000];
    i = 0 as libc::c_int;
    while i < 1000 as libc::c_int {
        rands[i as usize] = 1.0f64 + 0.5f64 * random_normal();
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
