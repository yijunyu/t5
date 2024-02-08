#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut reference: libc::c_char = 0;
    if argc > 2 as libc::c_int {
        printf(
            b"Usage : %s <Test String>\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        return 0 as libc::c_int;
    }
    if argc == 1 as libc::c_int
        || strlen(*argv.offset(1 as libc::c_int as isize))
            == 1 as libc::c_int as libc::c_ulong
    {
        printf(
            b"Input string : \"%s\"\nLength : %d\nAll characters are identical.\n\0"
                as *const u8 as *const libc::c_char,
            if argc == 1 as libc::c_int {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                *argv.offset(1 as libc::c_int as isize) as *const libc::c_char
            },
            if argc == 1 as libc::c_int {
                0 as libc::c_int
            } else {
                strlen(*argv.offset(1 as libc::c_int as isize)) as libc::c_int
            },
        );
        return 0 as libc::c_int;
    }
    reference = *(*argv.offset(1 as libc::c_int as isize))
        .offset(0 as libc::c_int as isize);
    len = strlen(*argv.offset(1 as libc::c_int as isize)) as libc::c_int;
    i = 1 as libc::c_int;
    while i < len {
        if *(*argv.offset(1 as libc::c_int as isize)).offset(i as isize) as libc::c_int
            != reference as libc::c_int
        {
            printf(
                b"Input string : \"%s\"\nLength : %d\nFirst different character : \"%c\"(0x%x) at position : %d\n\0"
                    as *const u8 as *const libc::c_char,
                *argv.offset(1 as libc::c_int as isize),
                len,
                *(*argv.offset(1 as libc::c_int as isize)).offset(i as isize)
                    as libc::c_int,
                *(*argv.offset(1 as libc::c_int as isize)).offset(i as isize)
                    as libc::c_int,
                i + 1 as libc::c_int,
            );
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    printf(
        b"Input string : \"%s\"\nLength : %d\nAll characters are identical.\n\0"
            as *const u8 as *const libc::c_char,
        *argv.offset(1 as libc::c_int as isize),
        len,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
