#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    scanf(
        b"%d%d\0" as *const u8 as *const libc::c_char,
        &mut a as *mut libc::c_int,
        &mut b as *mut libc::c_int,
    );
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, a + b);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
