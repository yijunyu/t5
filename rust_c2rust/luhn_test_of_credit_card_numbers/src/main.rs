#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luhn(mut cc: *const libc::c_char) -> libc::c_int {
    let m: [libc::c_int; 10] = [
        0 as libc::c_int,
        2 as libc::c_int,
        4 as libc::c_int,
        6 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        5 as libc::c_int,
        7 as libc::c_int,
        9 as libc::c_int,
    ];
    let mut i: libc::c_int = 0;
    let mut odd: libc::c_int = 1 as libc::c_int;
    let mut sum: libc::c_int = 0 as libc::c_int;
    i = strlen(cc) as libc::c_int;
    loop {
        let fresh0 = i;
        i = i - 1;
        if !(fresh0 != 0) {
            break;
        }
        let mut digit: libc::c_int = *cc.offset(i as isize) as libc::c_int - '0' as i32;
        sum += if odd != 0 { digit } else { m[digit as usize] };
        odd = (odd == 0) as libc::c_int;
    }
    return (sum % 10 as libc::c_int == 0 as libc::c_int) as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut cc: [*const libc::c_char; 5] = [
        b"49927398716\0" as *const u8 as *const libc::c_char,
        b"49927398717\0" as *const u8 as *const libc::c_char,
        b"1234567812345678\0" as *const u8 as *const libc::c_char,
        b"1234567812345670\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(cc[i as usize]).is_null() {
        printf(
            b"%16s\t%s\n\0" as *const u8 as *const libc::c_char,
            cc[i as usize],
            if luhn(cc[i as usize]) != 0 {
                b"ok\0" as *const u8 as *const libc::c_char
            } else {
                b"not ok\0" as *const u8 as *const libc::c_char
            },
        );
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
