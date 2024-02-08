#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn sum_proper_divisors(n: libc::c_uint) -> libc::c_uint {
    let mut sum: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    let mut i: libc::c_uint = 3 as libc::c_int as libc::c_uint;
    let mut j: libc::c_uint = 0;
    while (i as libc::c_double)
        < sqrt(n as libc::c_double) + 1 as libc::c_int as libc::c_double
    {
        if n.wrapping_rem(i) == 0 as libc::c_int as libc::c_uint {
            j = n.wrapping_div(i);
            sum = sum
                .wrapping_add(
                    i
                        .wrapping_add(
                            (if i == j { 0 as libc::c_int as libc::c_uint } else { j }),
                        ),
                );
        }
        i = i.wrapping_add(2 as libc::c_int as libc::c_uint);
    }
    return sum;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *const libc::c_char,
) -> libc::c_int {
    let mut n: libc::c_uint = 0;
    let mut c: libc::c_uint = 0;
    n = 1 as libc::c_int as libc::c_uint;
    c = 0 as libc::c_int as libc::c_uint;
    while c < 25 as libc::c_int as libc::c_uint {
        if n < sum_proper_divisors(n) {
            c = c.wrapping_add(1);
            printf(b"%u: %u\n\0" as *const u8 as *const libc::c_char, c, n);
        }
        n = n.wrapping_add(2 as libc::c_int as libc::c_uint);
    }
    while c < 1000 as libc::c_int as libc::c_uint {
        if n < sum_proper_divisors(n) {
            c = c.wrapping_add(1);
            c;
        }
        n = n.wrapping_add(2 as libc::c_int as libc::c_uint);
    }
    printf(
        b"\nThe one thousandth abundant odd number is: %u\n\0" as *const u8
            as *const libc::c_char,
        n,
    );
    n = 1000000001 as libc::c_int as libc::c_uint;
    while !(n < sum_proper_divisors(n)) {
        n = n.wrapping_add(2 as libc::c_int as libc::c_uint);
    }
    printf(
        b"The first abundant odd number above one billion is: %u\n\0" as *const u8
            as *const libc::c_char,
        n,
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
                args.as_mut_ptr() as *mut *const libc::c_char,
            ) as i32,
        )
    }
}
