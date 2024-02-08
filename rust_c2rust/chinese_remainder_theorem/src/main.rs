#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mul_inv(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    let mut b0: libc::c_int = b;
    let mut t: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    let mut x0: libc::c_int = 0 as libc::c_int;
    let mut x1: libc::c_int = 1 as libc::c_int;
    if b == 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    while a > 1 as libc::c_int {
        q = a / b;
        t = b;
        b = a % b;
        a = t;
        t = x0;
        x0 = x1 - q * x0;
        x1 = t;
    }
    if x1 < 0 as libc::c_int {
        x1 += b0;
    }
    return x1;
}
#[no_mangle]
pub unsafe extern "C" fn chinese_remainder(
    mut n: *mut libc::c_int,
    mut a: *mut libc::c_int,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut p: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut prod: libc::c_int = 1 as libc::c_int;
    let mut sum: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < len {
        prod *= *n.offset(i as isize);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < len {
        p = prod / *n.offset(i as isize);
        sum += *a.offset(i as isize) * mul_inv(p, *n.offset(i as isize)) * p;
        i += 1;
        i;
    }
    return sum % prod;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: [libc::c_int; 3] = [3 as libc::c_int, 5 as libc::c_int, 7 as libc::c_int];
    let mut a: [libc::c_int; 3] = [2 as libc::c_int, 3 as libc::c_int, 2 as libc::c_int];
    printf(
        b"%d\n\0" as *const u8 as *const libc::c_char,
        chinese_remainder(
            n.as_mut_ptr(),
            a.as_mut_ptr(),
            (::core::mem::size_of::<[libc::c_int; 3]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as libc::c_int,
        ),
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
