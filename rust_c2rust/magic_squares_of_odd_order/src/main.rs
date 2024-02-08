#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar(__c: libc::c_int) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn f(
    mut n: libc::c_int,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> libc::c_int {
    return (x + y * 2 as libc::c_int + 1 as libc::c_int) % n;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if argc != 2 as libc::c_int {
        return 1 as libc::c_int;
    }
    n = atoi(*argv.offset(1 as libc::c_int as isize));
    if n < 3 as libc::c_int || n % 2 as libc::c_int == 0 as libc::c_int {
        return 2 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < n {
        j = 0 as libc::c_int;
        while j < n {
            printf(
                b"% 4d\0" as *const u8 as *const libc::c_char,
                f(n, n - j - 1 as libc::c_int, i) * n + f(n, j, i) + 1 as libc::c_int,
            );
            j += 1;
            j;
        }
        putchar('\n' as i32);
        i += 1;
        i;
    }
    printf(
        b"\n Magic Constant: %d.\n\0" as *const u8 as *const libc::c_char,
        (n * n + 1 as libc::c_int) / 2 as libc::c_int * n,
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
