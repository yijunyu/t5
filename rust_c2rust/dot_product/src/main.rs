#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
unsafe fn main_0() -> libc::c_int {
    let mut a: [libc::c_int; 3] = [
        1 as libc::c_int,
        3 as libc::c_int,
        -(5 as libc::c_int),
    ];
    let mut b: [libc::c_int; 3] = [
        4 as libc::c_int,
        -(2 as libc::c_int),
        -(1 as libc::c_int),
    ];
    printf(
        b"%d\n\0" as *const u8 as *const libc::c_char,
        dot_product(
            a.as_mut_ptr(),
            b.as_mut_ptr(),
            (::core::mem::size_of::<[libc::c_int; 3]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ),
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dot_product(
    mut a: *mut libc::c_int,
    mut b: *mut libc::c_int,
    mut n: size_t,
) -> libc::c_int {
    let mut sum: libc::c_int = 0 as libc::c_int;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        sum += *a.offset(i as isize) * *b.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    return sum;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
