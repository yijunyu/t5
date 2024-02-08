#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
unsafe fn main_0() -> libc::c_int {
    let mut len: libc::c_int = 0;
    printf(b"GMP says size is: %d\n\0" as *const u8 as *const libc::c_char, len);
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    len = strlen(s) as libc::c_int;
    printf(b"size really is %d\n\0" as *const u8 as *const libc::c_char, len);
    printf(
        b"Digits: %.20s...%s\n\0" as *const u8 as *const libc::c_char,
        s,
        s.offset(len as isize).offset(-(20 as libc::c_int as isize)),
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
