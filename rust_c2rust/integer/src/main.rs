#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    if argc < 3 as libc::c_int {
        exit(1 as libc::c_int);
    }
    argc -= 1;
    b = atoi(*argv.offset(argc as isize));
    if b == 0 as libc::c_int {
        exit(2 as libc::c_int);
    }
    argc -= 1;
    a = atoi(*argv.offset(argc as isize));
    printf(b"a+b = %d\n\0" as *const u8 as *const libc::c_char, a + b);
    printf(b"a-b = %d\n\0" as *const u8 as *const libc::c_char, a - b);
    printf(b"a*b = %d\n\0" as *const u8 as *const libc::c_char, a * b);
    printf(b"a/b = %d\n\0" as *const u8 as *const libc::c_char, a / b);
    printf(b"a%%b = %d\n\0" as *const u8 as *const libc::c_char, a % b);
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
