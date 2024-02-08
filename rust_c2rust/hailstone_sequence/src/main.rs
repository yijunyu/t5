#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn hailstone(
    mut n: libc::c_int,
    mut arry: *mut libc::c_int,
) -> libc::c_int {
    let mut hs: libc::c_int = 1 as libc::c_int;
    while n != 1 as libc::c_int {
        hs += 1;
        hs;
        if !arry.is_null() {
            let fresh0 = arry;
            arry = arry.offset(1);
            *fresh0 = n;
        }
        n = if n & 1 as libc::c_int != 0 {
            3 as libc::c_int * n + 1 as libc::c_int
        } else {
            n / 2 as libc::c_int
        };
    }
    if !arry.is_null() {
        let fresh1 = arry;
        arry = arry.offset(1);
        *fresh1 = n;
    }
    return hs;
}
unsafe fn main_0() -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut hmax: libc::c_int = 0 as libc::c_int;
    let mut jatmax: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut arry: *mut libc::c_int = 0 as *mut libc::c_int;
    j = 1 as libc::c_int;
    while j < 100000 as libc::c_int {
        n = hailstone(j, 0 as *mut libc::c_int);
        if hmax < n {
            hmax = n;
            jatmax = j;
        }
        j += 1;
        j;
    }
    n = hailstone(27 as libc::c_int, 0 as *mut libc::c_int);
    arry = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    n = hailstone(27 as libc::c_int, arry);
    printf(
        b"[ %d, %d, %d, %d, ...., %d, %d, %d, %d] len=%d\n\0" as *const u8
            as *const libc::c_char,
        *arry.offset(0 as libc::c_int as isize),
        *arry.offset(1 as libc::c_int as isize),
        *arry.offset(2 as libc::c_int as isize),
        *arry.offset(3 as libc::c_int as isize),
        *arry.offset((n - 4 as libc::c_int) as isize),
        *arry.offset((n - 3 as libc::c_int) as isize),
        *arry.offset((n - 2 as libc::c_int) as isize),
        *arry.offset((n - 1 as libc::c_int) as isize),
        n,
    );
    printf(b"Max %d at j= %d\n\0" as *const u8 as *const libc::c_char, hmax, jatmax);
    free(arry as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
