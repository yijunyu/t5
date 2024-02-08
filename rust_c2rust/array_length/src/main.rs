#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut fruit: [*const libc::c_char; 2] = [
        b"apples\0" as *const u8 as *const libc::c_char,
        b"oranges\0" as *const u8 as *const libc::c_char,
    ];
    let mut length: libc::c_int = (::core::mem::size_of::<[*const libc::c_char; 2]>()
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
        as libc::c_int;
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, length);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
