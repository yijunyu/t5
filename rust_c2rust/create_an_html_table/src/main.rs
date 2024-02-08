#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn rand() -> libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    printf(
        b"<table style=\"text-align:center; border: 1px solid\"><th></th><th>X</th><th>Y</th><th>Z</th>\0"
            as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        printf(
            b"<tr><th>%d</th><td>%d</td><td>%d</td><td>%d</td></tr>\0" as *const u8
                as *const libc::c_char,
            i,
            rand() % 10000 as libc::c_int,
            rand() % 10000 as libc::c_int,
            rand() % 10000 as libc::c_int,
        );
        i += 1;
        i;
    }
    printf(b"</table>\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
