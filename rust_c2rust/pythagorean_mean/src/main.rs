#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn atof(__nptr: *const libc::c_char) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut f: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.0f64;
    let mut prod: libc::c_double = 1.0f64;
    let mut resum: libc::c_double = 0.0f64;
    i = 1 as libc::c_int;
    while i < argc {
        f = atof(*argv.offset(i as isize));
        count += 1;
        count;
        sum += f;
        prod *= f;
        resum += 1.0f64 / f;
        i += 1;
        i;
    }
    printf(
        b"Arithmetic mean = %f\n\0" as *const u8 as *const libc::c_char,
        sum / count as libc::c_double,
    );
    printf(
        b"Geometric mean = %f\n\0" as *const u8 as *const libc::c_char,
        pow(prod, 1.0f64 / count as libc::c_double),
    );
    printf(
        b"Harmonic mean = %f\n\0" as *const u8 as *const libc::c_char,
        count as libc::c_double / resum,
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
