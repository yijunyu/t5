#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
}
pub type __int64_t = libc::c_long;
pub type int64_t = __int64_t;
#[no_mangle]
pub unsafe extern "C" fn rat_approx(
    mut f: libc::c_double,
    mut md: int64_t,
    mut num: *mut int64_t,
    mut denom: *mut int64_t,
) {
    let mut a: int64_t = 0;
    let mut h: [int64_t; 3] = [
        0 as libc::c_int as int64_t,
        1 as libc::c_int as int64_t,
        0 as libc::c_int as int64_t,
    ];
    let mut k: [int64_t; 3] = [
        1 as libc::c_int as int64_t,
        0 as libc::c_int as int64_t,
        0 as libc::c_int as int64_t,
    ];
    let mut x: int64_t = 0;
    let mut d: int64_t = 0;
    let mut n: int64_t = 1 as libc::c_int as int64_t;
    let mut i: libc::c_int = 0;
    let mut neg: libc::c_int = 0 as libc::c_int;
    if md <= 1 as libc::c_int as libc::c_long {
        *denom = 1 as libc::c_int as int64_t;
        *num = f as int64_t;
        return;
    }
    if f < 0 as libc::c_int as libc::c_double {
        neg = 1 as libc::c_int;
        f = -f;
    }
    while f != floor(f) {
        n <<= 1 as libc::c_int;
        f *= 2 as libc::c_int as libc::c_double;
    }
    d = f as int64_t;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        a = if n != 0 { d / n } else { 0 as libc::c_int as libc::c_long };
        if i != 0 && a == 0 {
            break;
        }
        x = d;
        d = n;
        n = x % n;
        x = a;
        if k[1 as libc::c_int as usize] * a + k[0 as libc::c_int as usize] >= md {
            x = (md - k[0 as libc::c_int as usize]) / k[1 as libc::c_int as usize];
            if !(x * 2 as libc::c_int as libc::c_long >= a
                || k[1 as libc::c_int as usize] >= md)
            {
                break;
            }
            i = 65 as libc::c_int;
        }
        h[2 as libc::c_int
            as usize] = x * h[1 as libc::c_int as usize] + h[0 as libc::c_int as usize];
        h[0 as libc::c_int as usize] = h[1 as libc::c_int as usize];
        h[1 as libc::c_int as usize] = h[2 as libc::c_int as usize];
        k[2 as libc::c_int
            as usize] = x * k[1 as libc::c_int as usize] + k[0 as libc::c_int as usize];
        k[0 as libc::c_int as usize] = k[1 as libc::c_int as usize];
        k[1 as libc::c_int as usize] = k[2 as libc::c_int as usize];
        i += 1;
        i;
    }
    *denom = k[1 as libc::c_int as usize];
    *num = if neg != 0 {
        -h[1 as libc::c_int as usize]
    } else {
        h[1 as libc::c_int as usize]
    };
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut d: int64_t = 0;
    let mut n: int64_t = 0;
    let mut f: libc::c_double = 0.;
    f = 1.0f64 / 7 as libc::c_int as libc::c_double;
    printf(b"f = %16.14f\n\0" as *const u8 as *const libc::c_char, f);
    i = 1 as libc::c_int;
    while i <= 20000000 as libc::c_int {
        printf(b"denom <= %d: \0" as *const u8 as *const libc::c_char, i);
        rat_approx(f, i as int64_t, &mut n, &mut d);
        printf(b"%lld/%lld\n\0" as *const u8 as *const libc::c_char, n, d);
        i *= 16 as libc::c_int;
    }
    f = atan2(1 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double)
        * 4 as libc::c_int as libc::c_double;
    printf(b"\nf = %16.14f\n\0" as *const u8 as *const libc::c_char, f);
    i = 1 as libc::c_int;
    while i <= 20000000 as libc::c_int {
        printf(b"denom <= %d: \0" as *const u8 as *const libc::c_char, i);
        rat_approx(f, i as int64_t, &mut n, &mut d);
        printf(b"%lld/%lld\n\0" as *const u8 as *const libc::c_char, n, d);
        i *= 16 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
