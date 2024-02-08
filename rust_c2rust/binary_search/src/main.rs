#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bsearch(
    mut a: *mut libc::c_int,
    mut n: libc::c_int,
    mut x: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = n - 1 as libc::c_int;
    while i <= j {
        let mut k: libc::c_int = i + (j - i) / 2 as libc::c_int;
        if *a.offset(k as isize) == x {
            return k
        } else if *a.offset(k as isize) < x {
            i = k + 1 as libc::c_int;
        } else {
            j = k - 1 as libc::c_int;
        }
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn bsearch_r(
    mut a: *mut libc::c_int,
    mut x: libc::c_int,
    mut i: libc::c_int,
    mut j: libc::c_int,
) -> libc::c_int {
    if j < i {
        return -(1 as libc::c_int);
    }
    let mut k: libc::c_int = i + (j - i) / 2 as libc::c_int;
    if *a.offset(k as isize) == x {
        return k
    } else if *a.offset(k as isize) < x {
        return bsearch_r(a, x, k + 1 as libc::c_int, j)
    } else {
        return bsearch_r(a, x, i, k - 1 as libc::c_int)
    };
}
unsafe fn main_0() -> libc::c_int {
    let mut a: [libc::c_int; 10] = [
        -(31 as libc::c_int),
        0 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        2 as libc::c_int,
        4 as libc::c_int,
        65 as libc::c_int,
        83 as libc::c_int,
        99 as libc::c_int,
        782 as libc::c_int,
    ];
    let mut n: libc::c_int = (::core::mem::size_of::<[libc::c_int; 10]>()
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_int;
    let mut x: libc::c_int = 2 as libc::c_int;
    let mut i: libc::c_int = bsearch(a.as_mut_ptr(), n, x);
    printf(b"%d is at index %d\n\0" as *const u8 as *const libc::c_char, x, i);
    x = 5 as libc::c_int;
    i = bsearch_r(a.as_mut_ptr(), x, 0 as libc::c_int, n - 1 as libc::c_int);
    printf(b"%d is at index %d\n\0" as *const u8 as *const libc::c_char, x, i);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
