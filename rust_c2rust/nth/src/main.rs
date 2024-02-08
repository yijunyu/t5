#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn putchar(__c: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn addSuffix(
    mut num: libc::c_int,
    mut buf: *mut libc::c_char,
    mut len: size_t,
) -> *mut libc::c_char {
    let mut suffixes: [*mut libc::c_char; 4] = [
        b"th\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"st\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"nd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"rd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    let mut i: libc::c_int = 0;
    match num % 10 as libc::c_int {
        1 => {
            i = if num % 100 as libc::c_int == 11 as libc::c_int {
                0 as libc::c_int
            } else {
                1 as libc::c_int
            };
        }
        2 => {
            i = if num % 100 as libc::c_int == 12 as libc::c_int {
                0 as libc::c_int
            } else {
                2 as libc::c_int
            };
        }
        3 => {
            i = if num % 100 as libc::c_int == 13 as libc::c_int {
                0 as libc::c_int
            } else {
                3 as libc::c_int
            };
        }
        _ => {
            i = 0 as libc::c_int;
        }
    }
    snprintf(
        buf,
        len,
        b"%d%s\0" as *const u8 as *const libc::c_char,
        num,
        suffixes[i as usize],
    );
    return buf;
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    printf(b"Set [0,25]:\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < 26 as libc::c_int {
        let mut s: [libc::c_char; 5] = [0; 5];
        printf(
            b"%s \0" as *const u8 as *const libc::c_char,
            addSuffix(i, s.as_mut_ptr(), 5 as libc::c_int as size_t),
        );
        i += 1;
        i;
    }
    putchar('\n' as i32);
    printf(b"Set [250,265]:\n\0" as *const u8 as *const libc::c_char);
    i = 250 as libc::c_int;
    while i < 266 as libc::c_int {
        let mut s_0: [libc::c_char; 6] = [0; 6];
        printf(
            b"%s \0" as *const u8 as *const libc::c_char,
            addSuffix(i, s_0.as_mut_ptr(), 6 as libc::c_int as size_t),
        );
        i += 1;
        i;
    }
    putchar('\n' as i32);
    printf(b"Set [1000,1025]:\n\0" as *const u8 as *const libc::c_char);
    i = 1000 as libc::c_int;
    while i < 1026 as libc::c_int {
        let mut s_1: [libc::c_char; 7] = [0; 7];
        printf(
            b"%s \0" as *const u8 as *const libc::c_char,
            addSuffix(i, s_1.as_mut_ptr(), 7 as libc::c_int as size_t),
        );
        i += 1;
        i;
    }
    putchar('\n' as i32);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
