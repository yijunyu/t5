#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
#[no_mangle]
pub static mut p: [libc::c_int; 100001] = [0; 100001];
#[no_mangle]
pub unsafe extern "C" fn calc(mut n: libc::c_int) {
    let mut k: libc::c_int = 1 as libc::c_int;
    while k <= n {
        let mut d: libc::c_int = n
            - k * (3 as libc::c_int * k - 1 as libc::c_int) / 2 as libc::c_int;
        if d < 0 as libc::c_int {
            break;
        }
        d -= k;
        if d < 0 as libc::c_int {
            break;
        }
        k += 1;
        k;
    }
}
unsafe fn main_0() -> libc::c_int {
    let mut idx: [libc::c_int; 10] = [
        23 as libc::c_int,
        123 as libc::c_int,
        1234 as libc::c_int,
        12345 as libc::c_int,
        20000 as libc::c_int,
        30000 as libc::c_int,
        40000 as libc::c_int,
        50000 as libc::c_int,
        100000 as libc::c_int,
        0 as libc::c_int,
    ];
    let mut at: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 1 as libc::c_int;
    while idx[at as usize] != 0 {
        calc(i);
        if !(i != idx[at as usize]) {
            at += 1;
            at;
        }
        i += 1;
        i;
    }
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
