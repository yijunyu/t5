#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ackermann(
    mut m: libc::c_int,
    mut n: libc::c_int,
) -> libc::c_int {
    if m == 0 {
        return n + 1 as libc::c_int;
    }
    if n == 0 {
        return ackermann(m - 1 as libc::c_int, 1 as libc::c_int);
    }
    return ackermann(m - 1 as libc::c_int, ackermann(m, n - 1 as libc::c_int));
}
unsafe fn main_0() -> libc::c_int {
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    m = 0 as libc::c_int;
    while m <= 4 as libc::c_int {
        n = 0 as libc::c_int;
        while n < 6 as libc::c_int - m {
            printf(
                b"A(%d, %d) = %d\n\0" as *const u8 as *const libc::c_char,
                m,
                n,
                ackermann(m, n),
            );
            n += 1;
            n;
        }
        m += 1;
        m;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
