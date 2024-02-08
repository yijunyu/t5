#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type intFn = Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int>;
#[no_mangle]
pub unsafe extern "C" fn reduce(
    mut fn_0: intFn,
    mut size: libc::c_int,
    mut elms: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut val: libc::c_int = *elms;
    i = 1 as libc::c_int;
    while i < size {
        val = fn_0.expect("non-null function pointer")(val, *elms.offset(i as isize));
        i += 1;
        i;
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn add(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return a + b;
}
#[no_mangle]
pub unsafe extern "C" fn sub(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return a - b;
}
#[no_mangle]
pub unsafe extern "C" fn mul(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return a * b;
}
unsafe fn main_0() -> libc::c_int {
    let mut nums: [libc::c_int; 5] = [
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
    ];
    printf(
        b"%d\n\0" as *const u8 as *const libc::c_char,
        reduce(
            Some(add as unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int),
            5 as libc::c_int,
            nums.as_mut_ptr(),
        ),
    );
    printf(
        b"%d\n\0" as *const u8 as *const libc::c_char,
        reduce(
            Some(sub as unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int),
            5 as libc::c_int,
            nums.as_mut_ptr(),
        ),
    );
    printf(
        b"%d\n\0" as *const u8 as *const libc::c_char,
        reduce(
            Some(mul as unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int),
            5 as libc::c_int,
            nums.as_mut_ptr(),
        ),
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
