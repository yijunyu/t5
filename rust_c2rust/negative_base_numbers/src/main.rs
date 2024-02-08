#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub static mut DIGITS: [libc::c_char; 63] = unsafe {
    *::core::mem::transmute::<
        &[u8; 63],
        &[libc::c_char; 63],
    >(b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz\0")
};
#[no_mangle]
pub static mut DIGITS_LEN: libc::c_int = 64 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn encodeNegativeBase(
    mut n: libc::c_long,
    mut base: libc::c_long,
    mut out: *mut libc::c_char,
) {
    let mut ptr: *mut libc::c_char = out;
    if base > -(1 as libc::c_int) as libc::c_long
        || base < -(62 as libc::c_int) as libc::c_long
    {
        out = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        return;
    }
    if n == 0 as libc::c_int as libc::c_long {
        out = b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        return;
    }
    while n != 0 as libc::c_int as libc::c_long {
        let mut rem: libc::c_long = n % base;
        n = n / base;
        if rem < 0 as libc::c_int as libc::c_long {
            n += 1;
            n;
            rem = rem - base;
        }
        *ptr = DIGITS[rem as usize];
        ptr = ptr.offset(1);
        ptr;
    }
    *ptr = 0 as libc::c_int as libc::c_char;
    ptr = ptr.offset(-1);
    ptr;
    while out < ptr {
        let mut t: libc::c_char = *out;
        *out = *ptr;
        *ptr = t;
        out = out.offset(1);
        out;
        ptr = ptr.offset(-1);
        ptr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn decodeNegativeBase(
    mut ns: *const libc::c_char,
    mut base: libc::c_long,
) -> libc::c_long {
    let mut value: libc::c_long = 0;
    let mut bb: libc::c_long = 0;
    let mut i: libc::c_int = 0;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    if base < -(62 as libc::c_int) as libc::c_long
        || base > -(1 as libc::c_int) as libc::c_long
    {
        return 0 as libc::c_int as libc::c_long;
    }
    if *ns.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
        || *ns.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
            && *ns.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
    {
        return 0 as libc::c_int as libc::c_long;
    }
    ptr = ns;
    while *ptr as libc::c_int != 0 as libc::c_int {
        ptr = ptr.offset(1);
        ptr;
    }
    value = 0 as libc::c_int as libc::c_long;
    bb = 1 as libc::c_int as libc::c_long;
    ptr = ptr.offset(-1);
    ptr;
    while ptr >= ns {
        i = 0 as libc::c_int;
        while i < DIGITS_LEN {
            if *ptr as libc::c_int == DIGITS[i as usize] as libc::c_int {
                value = value + i as libc::c_long * bb;
                bb = bb * base;
                break;
            } else {
                i += 1;
                i;
            }
        }
        ptr = ptr.offset(-1);
        ptr;
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn driver(mut n: libc::c_long, mut b: libc::c_long) {
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut value: libc::c_long = 0;
    encodeNegativeBase(n, b, buf.as_mut_ptr());
    printf(
        b"%12d encoded in base %3d = %12s\n\0" as *const u8 as *const libc::c_char,
        n,
        b,
        buf.as_mut_ptr(),
    );
    value = decodeNegativeBase(buf.as_mut_ptr(), b);
    printf(
        b"%12s decoded in base %3d = %12d\n\0" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        b,
        value,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    driver(10 as libc::c_int as libc::c_long, -(2 as libc::c_int) as libc::c_long);
    driver(146 as libc::c_int as libc::c_long, -(3 as libc::c_int) as libc::c_long);
    driver(15 as libc::c_int as libc::c_long, -(10 as libc::c_int) as libc::c_long);
    driver(12 as libc::c_int as libc::c_long, -(62 as libc::c_int) as libc::c_long);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
