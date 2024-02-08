#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn is_leap_year(mut year: libc::c_int) -> libc::c_int {
    return if year % 4 as libc::c_int == 0 && year % 100 as libc::c_int != 0
        || year % 400 as libc::c_int == 0
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe fn main_0() -> libc::c_int {
    let mut test_case: [libc::c_int; 5] = [
        1900 as libc::c_int,
        1994 as libc::c_int,
        1996 as libc::c_int,
        1997 as libc::c_int,
        2000 as libc::c_int,
    ];
    let mut key: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut year: libc::c_int = 0;
    key = 0 as libc::c_int;
    end = (::core::mem::size_of::<[libc::c_int; 5]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_int;
    while key < end {
        year = test_case[key as usize];
        printf(
            b"%d is %sa leap year.\n\0" as *const u8 as *const libc::c_char,
            year,
            if is_leap_year(year) == 1 as libc::c_int {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b"not \0" as *const u8 as *const libc::c_char
            },
        );
        key += 1;
        key;
    }
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
