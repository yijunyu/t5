#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub static mut a: libc::c_int = 0;
#[no_mangle]
pub static mut b: libc::c_int = 0;
#[no_mangle]
pub static mut c: libc::c_int = 0;
#[no_mangle]
pub static mut d: libc::c_int = 0;
#[no_mangle]
pub static mut e: libc::c_int = 0;
#[no_mangle]
pub static mut f: libc::c_int = 0;
#[no_mangle]
pub static mut g: libc::c_int = 0;
#[no_mangle]
pub static mut lo: libc::c_int = 0;
#[no_mangle]
pub static mut hi: libc::c_int = 0;
#[no_mangle]
pub static mut unique: libc::c_int = 0;
#[no_mangle]
pub static mut show: libc::c_int = 0;
#[no_mangle]
pub static mut solutions: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn bf() {
    f = lo;
    while f <= hi {
        if unique == 0 || f != a && f != c && f != d && f != g && f != e {
            b = e + f - c;
            if b >= lo && b <= hi
                && (unique == 0
                    || b != a && b != c && b != d && b != g && b != e && b != f)
            {
                solutions += 1;
                solutions;
                if show != 0 {
                    printf(
                        b"%d %d %d %d %d %d %d\n\0" as *const u8 as *const libc::c_char,
                        a,
                        b,
                        c,
                        d,
                        e,
                        f,
                        g,
                    );
                }
            }
        }
        f += 1;
        f;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ge() {
    e = lo;
    while e <= hi {
        if unique == 0 || e != a && e != c && e != d {
            g = d + e;
            if g >= lo && g <= hi
                && (unique == 0 || g != a && g != c && g != d && g != e)
            {
                bf();
            }
        }
        e += 1;
        e;
    }
}
#[no_mangle]
pub unsafe extern "C" fn acd() {
    c = lo;
    while c <= hi {
        d = lo;
        while d <= hi {
            if unique == 0 || c != d {
                a = c + d;
                if a >= lo && a <= hi
                    && (unique == 0 || c != 0 as libc::c_int && d != 0 as libc::c_int)
                {
                    ge();
                }
            }
            d += 1;
            d;
        }
        c += 1;
        c;
    }
}
#[no_mangle]
pub unsafe extern "C" fn foursquares(
    mut plo: libc::c_int,
    mut phi: libc::c_int,
    mut punique: libc::c_int,
    mut pshow: libc::c_int,
) {
    lo = plo;
    hi = phi;
    unique = punique;
    show = pshow;
    solutions = 0 as libc::c_int;
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    acd();
    if unique != 0 {
        printf(
            b"\n%d unique solutions in %d to %d\n\0" as *const u8 as *const libc::c_char,
            solutions,
            lo,
            hi,
        );
    } else {
        printf(
            b"\n%d non-unique solutions in %d to %d\n\0" as *const u8
                as *const libc::c_char,
            solutions,
            lo,
            hi,
        );
    };
}
unsafe fn main_0() -> libc::c_int {
    foursquares(1 as libc::c_int, 7 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    foursquares(3 as libc::c_int, 9 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    foursquares(0 as libc::c_int, 9 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
