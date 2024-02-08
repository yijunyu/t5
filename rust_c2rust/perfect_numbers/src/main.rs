#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn perfect(mut n: libc::c_int) -> libc::c_int {
    let mut max: libc::c_int = sqrt(n as libc::c_double) as libc::c_int
        + 1 as libc::c_int;
    let mut tot: libc::c_int = 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 2 as libc::c_int;
    while i < max {
        if n % i == 0 as libc::c_int {
            tot += i;
            let mut q: libc::c_int = n / i;
            if q > i {
                tot += q;
            }
        }
        i += 1;
        i;
    }
    return (tot == n) as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    n = 2 as libc::c_int;
    while n < 200 as libc::c_int {
        if perfect(n) != 0 {
            printf(b"%d\n\0" as *const u8 as *const libc::c_char, n);
        }
        n += 1;
        n;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
