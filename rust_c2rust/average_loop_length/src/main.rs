#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
#[no_mangle]
pub unsafe extern "C" fn factorial(mut n: libc::c_int) -> libc::c_double {
    let mut f: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= n {
        f *= i as libc::c_double;
        i += 1;
        i;
    }
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn expected(mut n: libc::c_int) -> libc::c_double {
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= n {
        sum
            += factorial(n) / pow(n as libc::c_double, i as libc::c_double)
                / factorial(n - i);
        i += 1;
        i;
    }
    return sum;
}
#[no_mangle]
pub unsafe extern "C" fn randint(mut n: libc::c_int) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut rmax: libc::c_int = 2147483647 as libc::c_int / n * n;
    loop {
        r = rand();
        if !(r >= rmax) {
            break;
        }
    }
    return r / (2147483647 as libc::c_int / n);
}
#[no_mangle]
pub unsafe extern "C" fn test(
    mut n: libc::c_int,
    mut times: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < times {
        let mut x: libc::c_int = 1 as libc::c_int;
        let mut bits: libc::c_int = 0 as libc::c_int;
        while bits & x == 0 {
            count += 1;
            count;
            bits |= x;
            x = (1 as libc::c_int) << randint(n);
        }
        i += 1;
        i;
    }
    return count;
}
unsafe fn main_0() -> libc::c_int {
    srand(time(0 as *mut time_t) as libc::c_uint);
    puts(
        b" n\tavg\texp.\tdiff\n-------------------------------\0" as *const u8
            as *const libc::c_char,
    );
    let mut n: libc::c_int = 0;
    n = 1 as libc::c_int;
    while n <= 20 as libc::c_int {
        let mut cnt: libc::c_int = test(n, 1000000 as libc::c_int);
        let mut avg: libc::c_double = cnt as libc::c_double
            / 1000000 as libc::c_int as libc::c_double;
        let mut theory: libc::c_double = expected(n);
        let mut diff: libc::c_double = (avg / theory
            - 1 as libc::c_int as libc::c_double) * 100 as libc::c_int as libc::c_double;
        printf(
            b"%2d %8.4f %8.4f %6.3f%%\n\0" as *const u8 as *const libc::c_char,
            n,
            avg,
            theory,
            diff,
        );
        n += 1;
        n;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
