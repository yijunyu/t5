#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    puts(getenv(b"HOME\0" as *const u8 as *const libc::c_char));
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
