#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn t(mut n: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    i = n * (n - 1 as libc::c_int) / 2 as libc::c_int;
    c = 1 as libc::c_int;
    len = c;
    while c < i {
        c *= 10 as libc::c_int;
        len += 1;
        len;
    }
    c -= i;
    let mut num: libc::c_int = 0;
    i = 1 as libc::c_int;
    num = i;
    while i <= n {
        j = 1 as libc::c_int;
        while j <= i {
            let fresh0 = num;
            num = num + 1;
            printf(
                b"%*d%c\0" as *const u8 as *const libc::c_char,
                len - (j < c) as libc::c_int,
                fresh0,
                if i - j != 0 { ' ' as i32 } else { '\n' as i32 },
            );
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
unsafe fn main_0() -> libc::c_int {
    t(5 as libc::c_int);
    t(14 as libc::c_int);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
