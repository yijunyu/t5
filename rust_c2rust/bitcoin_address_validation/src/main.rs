#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn SHA256(
        d: *const libc::c_uchar,
        n: size_t,
        md: *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub static mut coin_err: *const libc::c_char = 0 as *const libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn unbase58(
    mut s: *const libc::c_char,
    mut out: *mut libc::c_uchar,
) -> libc::c_int {
    static mut tmpl: *const libc::c_char = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz\0"
        as *const u8 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    memset(
        out as *mut libc::c_void,
        0 as libc::c_int,
        25 as libc::c_int as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while *s.offset(i as isize) != 0 {
        p = strchr(tmpl, *s.offset(i as isize) as libc::c_int);
        if p.is_null() {
            coin_err = b"bad char\0" as *const u8 as *const libc::c_char;
            return 0 as libc::c_int;
        }
        c = p.offset_from(tmpl) as libc::c_long as libc::c_int;
        j = 25 as libc::c_int;
        loop {
            let fresh0 = j;
            j = j - 1;
            if !(fresh0 != 0) {
                break;
            }
            c += 58 as libc::c_int * *out.offset(j as isize) as libc::c_int;
            *out.offset(j as isize) = (c % 256 as libc::c_int) as libc::c_uchar;
            c /= 256 as libc::c_int;
        }
        if c != 0 {
            coin_err = b"address too long\0" as *const u8 as *const libc::c_char;
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn valid(mut s: *const libc::c_char) -> libc::c_int {
    let mut dec: [libc::c_uchar; 32] = [0; 32];
    let mut d1: [libc::c_uchar; 32] = [0; 32];
    let mut d2: [libc::c_uchar; 32] = [0; 32];
    coin_err = b"\0" as *const u8 as *const libc::c_char;
    if unbase58(s, dec.as_mut_ptr()) == 0 {
        return 0 as libc::c_int;
    }
    SHA256(
        SHA256(dec.as_mut_ptr(), 21 as libc::c_int as size_t, d1.as_mut_ptr()),
        32 as libc::c_int as size_t,
        d2.as_mut_ptr(),
    );
    if memcmp(
        dec.as_mut_ptr().offset(21 as libc::c_int as isize) as *const libc::c_void,
        d2.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    ) != 0
    {
        coin_err = b"bad digest\0" as *const u8 as *const libc::c_char;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut s: [*const libc::c_char; 5] = [
        b"1Q1pE5vPGEEMqRcVRMbtBK842Y6Pzo6nK9\0" as *const u8 as *const libc::c_char,
        b"1AGNa15ZQXAZUgFiqJ2i7Z2DPU2J6hW62i\0" as *const u8 as *const libc::c_char,
        b"1Q1pE5vPGEEMqRcVRMbtBK842Y6Pzo6nJ9\0" as *const u8 as *const libc::c_char,
        b"1AGNa15ZQXAZUgFiqJ2i7Z2DPU2J6hW62I\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(s[i as usize]).is_null() {
        let mut status: libc::c_int = valid(s[i as usize]);
        printf(
            b"%s: %s\n\0" as *const u8 as *const libc::c_char,
            s[i as usize],
            if status != 0 {
                b"Ok\0" as *const u8 as *const libc::c_char
            } else {
                coin_err
            },
        );
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
