#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
unsafe fn main_0() -> libc::c_int {
    let mut one: libc::c_int = 1 as libc::c_int;
    printf(
        b"word size = %d bits\n\0" as *const u8 as *const libc::c_char,
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<size_t>() as libc::c_ulong)
            as libc::c_int,
    );
    if *(&mut one as *mut libc::c_int as *mut libc::c_char) != 0 {
        printf(b"little endian\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"big endian\n\0" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
