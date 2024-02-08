#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type coeff_func = Option::<unsafe extern "C" fn(libc::c_uint) -> libc::c_double>;
#[no_mangle]
pub unsafe extern "C" fn calc(
    mut f_a: coeff_func,
    mut f_b: coeff_func,
    mut expansions: libc::c_uint,
) -> libc::c_double {
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    r = 0.0f64;
    b = r;
    a = b;
    let mut i: libc::c_uint = 0;
    i = expansions;
    while i > 0 as libc::c_int as libc::c_uint {
        a = f_a.expect("non-null function pointer")(i);
        b = f_b.expect("non-null function pointer")(i);
        r = b / (a + r);
        i = i.wrapping_sub(1);
        i;
    }
    a = f_a.expect("non-null function pointer")(0 as libc::c_int as libc::c_uint);
    return a + r;
}
#[no_mangle]
pub unsafe extern "C" fn sqrt2_a(mut n: libc::c_uint) -> libc::c_double {
    return if n != 0 { 2.0f64 } else { 1.0f64 };
}
#[no_mangle]
pub unsafe extern "C" fn sqrt2_b(mut n: libc::c_uint) -> libc::c_double {
    return 1.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn napier_a(mut n: libc::c_uint) -> libc::c_double {
    return if n != 0 { n as libc::c_double } else { 2.0f64 };
}
#[no_mangle]
pub unsafe extern "C" fn napier_b(mut n: libc::c_uint) -> libc::c_double {
    return if n as libc::c_double > 1.0f64 {
        n as libc::c_double - 1.0f64
    } else {
        1.0f64
    };
}
#[no_mangle]
pub unsafe extern "C" fn pi_a(mut n: libc::c_uint) -> libc::c_double {
    return if n != 0 { 6.0f64 } else { 3.0f64 };
}
#[no_mangle]
pub unsafe extern "C" fn pi_b(mut n: libc::c_uint) -> libc::c_double {
    let mut c: libc::c_double = 2.0f64 * n as libc::c_double - 1.0f64;
    return c * c;
}
unsafe fn main_0() -> libc::c_int {
    let mut sqrt2: libc::c_double = 0.;
    let mut napier: libc::c_double = 0.;
    let mut pi: libc::c_double = 0.;
    sqrt2 = calc(
        Some(sqrt2_a as unsafe extern "C" fn(libc::c_uint) -> libc::c_double),
        Some(sqrt2_b as unsafe extern "C" fn(libc::c_uint) -> libc::c_double),
        1000 as libc::c_int as libc::c_uint,
    );
    napier = calc(
        Some(napier_a as unsafe extern "C" fn(libc::c_uint) -> libc::c_double),
        Some(napier_b as unsafe extern "C" fn(libc::c_uint) -> libc::c_double),
        1000 as libc::c_int as libc::c_uint,
    );
    pi = calc(
        Some(pi_a as unsafe extern "C" fn(libc::c_uint) -> libc::c_double),
        Some(pi_b as unsafe extern "C" fn(libc::c_uint) -> libc::c_double),
        1000 as libc::c_int as libc::c_uint,
    );
    printf(
        b"%12.10g\n%12.10g\n%12.10g\n\0" as *const u8 as *const libc::c_char,
        sqrt2,
        napier,
        pi,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
