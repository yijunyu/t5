#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
}
pub type bool_0 = libc::c_int;
pub type ull = libc::c_ulonglong;
#[no_mangle]
pub unsafe extern "C" fn is_prime(mut n: ull) -> bool_0 {
    let mut d: ull = 0;
    if n < 2 as libc::c_int as libc::c_ulonglong {
        return 0 as libc::c_int;
    }
    if n.wrapping_rem(2 as libc::c_int as libc::c_ulonglong) == 0 {
        return (n == 2 as libc::c_int as libc::c_ulonglong) as libc::c_int;
    }
    if n.wrapping_rem(3 as libc::c_int as libc::c_ulonglong) == 0 {
        return (n == 3 as libc::c_int as libc::c_ulonglong) as libc::c_int;
    }
    d = 5 as libc::c_int as ull;
    while d.wrapping_mul(d) <= n {
        if n.wrapping_rem(d) == 0 {
            return 0 as libc::c_int;
        }
        d = (d as libc::c_ulonglong).wrapping_add(2 as libc::c_int as libc::c_ulonglong)
            as ull as ull;
        if n.wrapping_rem(d) == 0 {
            return 0 as libc::c_int;
        }
        d = (d as libc::c_ulonglong).wrapping_add(4 as libc::c_int as libc::c_ulonglong)
            as ull as ull;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ord(mut res: *mut libc::c_char, mut n: libc::c_int) {
    let mut suffix: [libc::c_char; 3] = [0; 3];
    let mut m: libc::c_int = n % 100 as libc::c_int;
    if m >= 4 as libc::c_int && m <= 20 as libc::c_int {
        sprintf(res, b"%dth\0" as *const u8 as *const libc::c_char, n);
        return;
    }
    match m % 10 as libc::c_int {
        1 => {
            strcpy(suffix.as_mut_ptr(), b"st\0" as *const u8 as *const libc::c_char);
        }
        2 => {
            strcpy(suffix.as_mut_ptr(), b"nd\0" as *const u8 as *const libc::c_char);
        }
        3 => {
            strcpy(suffix.as_mut_ptr(), b"rd\0" as *const u8 as *const libc::c_char);
        }
        _ => {
            strcpy(suffix.as_mut_ptr(), b"th\0" as *const u8 as *const libc::c_char);
        }
    }
    sprintf(res, b"%d%s\0" as *const u8 as *const libc::c_char, n, suffix.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn is_magnanimous(mut n: ull) -> bool_0 {
    let mut p: ull = 0;
    let mut q: ull = 0;
    let mut r: ull = 0;
    if n < 10 as libc::c_int as libc::c_ulonglong {
        return 1 as libc::c_int;
    }
    p = 10 as libc::c_int as ull;
    loop {
        q = n.wrapping_div(p);
        r = n.wrapping_rem(p);
        if is_prime(q.wrapping_add(r)) == 0 {
            return 0 as libc::c_int;
        }
        if q < 10 as libc::c_int as libc::c_ulonglong {
            break;
        }
        p = (p as libc::c_ulonglong).wrapping_mul(10 as libc::c_int as libc::c_ulonglong)
            as ull as ull;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn list_mags(
    mut from: libc::c_int,
    mut thru: libc::c_int,
    mut digs: libc::c_int,
    mut per_line: libc::c_int,
) {
    let mut i: ull = 0 as libc::c_int as ull;
    let mut c: libc::c_int = 0 as libc::c_int;
    let mut res1: [libc::c_char; 13] = [0; 13];
    let mut res2: [libc::c_char; 13] = [0; 13];
    if from < 2 as libc::c_int {
        printf(
            b"\nFirst %d magnanimous numbers:\n\0" as *const u8 as *const libc::c_char,
            thru,
        );
    } else {
        ord(res1.as_mut_ptr(), from);
        ord(res2.as_mut_ptr(), thru);
        printf(
            b"\n%s through %s magnanimous numbers:\n\0" as *const u8
                as *const libc::c_char,
            res1.as_mut_ptr(),
            res2.as_mut_ptr(),
        );
    }
    while c < thru {
        if is_magnanimous(i) != 0 {
            c += 1;
            if c >= from {
                printf(b"%*llu \0" as *const u8 as *const libc::c_char, digs, i);
                if c % per_line == 0 {
                    printf(b"\n\0" as *const u8 as *const libc::c_char);
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe fn main_0() -> libc::c_int {
    list_mags(1 as libc::c_int, 45 as libc::c_int, 3 as libc::c_int, 15 as libc::c_int);
    list_mags(
        241 as libc::c_int,
        250 as libc::c_int,
        1 as libc::c_int,
        10 as libc::c_int,
    );
    list_mags(
        391 as libc::c_int,
        400 as libc::c_int,
        1 as libc::c_int,
        10 as libc::c_int,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
