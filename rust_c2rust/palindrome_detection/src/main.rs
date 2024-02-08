#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn palindrome(mut s: *const libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    l = strlen(s) as libc::c_int;
    i = 0 as libc::c_int;
    while i < l / 2 as libc::c_int {
        if *s.offset(i as isize) as libc::c_int
            != *s.offset((l - i - 1 as libc::c_int) as isize) as libc::c_int
        {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn palindrome_r(
    mut s: *const libc::c_char,
    mut b: libc::c_int,
    mut e: libc::c_int,
) -> libc::c_int {
    if e - 1 as libc::c_int <= b {
        return 1 as libc::c_int;
    }
    if *s.offset(b as isize) as libc::c_int
        != *s.offset((e - 1 as libc::c_int) as isize) as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return palindrome_r(s, b + 1 as libc::c_int, e - 1 as libc::c_int);
}
unsafe fn main_0() -> libc::c_int {
    let mut t: *const libc::c_char = b"ingirumimusnocteetconsumimurigni\0" as *const u8
        as *const libc::c_char;
    let mut template: *const libc::c_char = b"sequence \"%s\" is%s palindrome\n\0"
        as *const u8 as *const libc::c_char;
    let mut l: libc::c_int = strlen(t) as libc::c_int;
    printf(
        template,
        t,
        if palindrome(t) != 0 {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"n't\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        template,
        t,
        if palindrome_r(t, 0 as libc::c_int, l) != 0 {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"n't\0" as *const u8 as *const libc::c_char
        },
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
