#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exp(_: libc::c_double) -> libc::c_double;
}
pub type deriv_f = Option::<
    unsafe extern "C" fn(libc::c_double, libc::c_double) -> libc::c_double,
>;
#[no_mangle]
pub unsafe extern "C" fn ivp_euler(
    mut f: deriv_f,
    mut y: libc::c_double,
    mut step: libc::c_int,
    mut end_t: libc::c_int,
) {
    let mut t: libc::c_int = 0 as libc::c_int;
    printf(b" Step %2d: \0" as *const u8 as *const libc::c_char, step);
    loop {
        if t % 10 as libc::c_int == 0 as libc::c_int {
            printf(b" %7.3f\0" as *const u8 as *const libc::c_char, y);
        }
        y
            += step as libc::c_double
                * f.expect("non-null function pointer")(t as libc::c_double, y);
        t += step;
        if !(t <= end_t) {
            break;
        }
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn analytic() {
    let mut t: libc::c_double = 0.;
    printf(b"    Time: \0" as *const u8 as *const libc::c_char);
    t = 0 as libc::c_int as libc::c_double;
    while t <= 100 as libc::c_int as libc::c_double {
        printf(b" %7g\0" as *const u8 as *const libc::c_char, t);
        t += 10 as libc::c_int as libc::c_double;
    }
    printf(b"\nAnalytic: \0" as *const u8 as *const libc::c_char);
    t = 0 as libc::c_int as libc::c_double;
    while t <= 100 as libc::c_int as libc::c_double {
        printf(
            b" %7.3f\0" as *const u8 as *const libc::c_char,
            20 as libc::c_int as libc::c_double
                + 80 as libc::c_int as libc::c_double * exp(-0.07f64 * t),
        );
        t += 10 as libc::c_int as libc::c_double;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn cooling(
    mut t: libc::c_double,
    mut temp: libc::c_double,
) -> libc::c_double {
    return -0.07f64 * (temp - 20 as libc::c_int as libc::c_double);
}
unsafe fn main_0() -> libc::c_int {
    analytic();
    ivp_euler(
        Some(
            cooling
                as unsafe extern "C" fn(libc::c_double, libc::c_double) -> libc::c_double,
        ),
        100 as libc::c_int as libc::c_double,
        2 as libc::c_int,
        100 as libc::c_int,
    );
    ivp_euler(
        Some(
            cooling
                as unsafe extern "C" fn(libc::c_double, libc::c_double) -> libc::c_double,
        ),
        100 as libc::c_int as libc::c_double,
        5 as libc::c_int,
        100 as libc::c_int,
    );
    ivp_euler(
        Some(
            cooling
                as unsafe extern "C" fn(libc::c_double, libc::c_double) -> libc::c_double,
        ),
        100 as libc::c_int as libc::c_double,
        10 as libc::c_int,
        100 as libc::c_int,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
