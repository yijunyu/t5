#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub static mut N: libc::c_int = 15 as libc::c_int;
unsafe fn main_0() -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut num: libc::c_ulonglong = 0;
    let mut den: libc::c_ulonglong = 0;
    let mut catalan: libc::c_int = 0;
    printf(b"1 \0" as *const u8 as *const libc::c_char);
    n = 2 as libc::c_int;
    while n <= N {
        den = 1 as libc::c_int as libc::c_ulonglong;
        num = den;
        k = 2 as libc::c_int;
        while k <= n {
            num = num.wrapping_mul((n + k) as libc::c_ulonglong);
            den = den.wrapping_mul(k as libc::c_ulonglong);
            catalan = num.wrapping_div(den) as libc::c_int;
            k += 1;
            k;
        }
        printf(b"%d \0" as *const u8 as *const libc::c_char, catalan);
        n += 1;
        n;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
