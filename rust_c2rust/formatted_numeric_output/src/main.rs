#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut r: libc::c_float = 7.125f64 as libc::c_float;
    printf(b" %9.3f\n\0" as *const u8 as *const libc::c_char, -r as libc::c_double);
    printf(b" %9.3f\n\0" as *const u8 as *const libc::c_char, r as libc::c_double);
    printf(b" %-9.3f\n\0" as *const u8 as *const libc::c_char, r as libc::c_double);
    printf(b" %09.3f\n\0" as *const u8 as *const libc::c_char, -r as libc::c_double);
    printf(b" %09.3f\n\0" as *const u8 as *const libc::c_char, r as libc::c_double);
    printf(b" %-09.3f\n\0" as *const u8 as *const libc::c_char, r as libc::c_double);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
