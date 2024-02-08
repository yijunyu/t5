#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn rand() -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn randn(mut m: libc::c_int) -> libc::c_int {
    let mut rand_max: libc::c_int = 2147483647 as libc::c_int
        - 2147483647 as libc::c_int % m;
    let mut r: libc::c_int = 0;
    loop {
        r = rand();
        if !(r > rand_max) {
            break;
        }
    }
    return r / (rand_max / m);
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut r2: libc::c_int = 0;
    let mut buf: [libc::c_ulong; 31] = [
        0 as libc::c_int as libc::c_ulong,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    i = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        x = randn(31 as libc::c_int) - 15 as libc::c_int;
        y = randn(31 as libc::c_int) - 15 as libc::c_int;
        r2 = x * x + y * y;
        if r2 >= 100 as libc::c_int && r2 <= 225 as libc::c_int {
            buf[(15 as libc::c_int + y) as usize]
                |= ((1 as libc::c_int) << x + 15 as libc::c_int) as libc::c_ulong;
            i += 1;
            i;
        }
    }
    y = 0 as libc::c_int;
    while y < 31 as libc::c_int {
        x = 0 as libc::c_int;
        while x < 31 as libc::c_int {
            printf(
                if buf[y as usize] & ((1 as libc::c_int) << x) as libc::c_ulong != 0 {
                    b". \0" as *const u8 as *const libc::c_char
                } else {
                    b"  \0" as *const u8 as *const libc::c_char
                },
            );
            x += 1;
            x;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        y += 1;
        y;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
