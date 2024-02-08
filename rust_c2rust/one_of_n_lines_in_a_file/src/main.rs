#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn rand() -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn irand(mut n: libc::c_int) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut randmax: libc::c_int = 2147483647 as libc::c_int / n * n;
    loop {
        r = rand();
        if !(r >= randmax) {
            break;
        }
    }
    return r / (randmax / n);
}
#[no_mangle]
pub unsafe extern "C" fn one_of_n(mut n: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < n {
        if irand(i + 1 as libc::c_int) == 0 {
            r = i;
        }
        i += 1;
        i;
    }
    return r;
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut r: [libc::c_int; 10] = [0 as libc::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    i = 0 as libc::c_int;
    while i < 1000000 as libc::c_int {
        i += 1;
        i;
        r[one_of_n(10 as libc::c_int) as usize] += 1;
        r[one_of_n(10 as libc::c_int) as usize];
    }
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        printf(
            b"%d%c\0" as *const u8 as *const libc::c_char,
            r[i as usize],
            if i == 9 as libc::c_int { '\n' as i32 } else { ' ' as i32 },
        );
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
