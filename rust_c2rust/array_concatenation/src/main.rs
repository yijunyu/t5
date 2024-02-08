#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn array_concat(
    mut a_0: *const libc::c_void,
    mut an: size_t,
    mut b_0: *const libc::c_void,
    mut bn: size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_char = malloc(s.wrapping_mul(an.wrapping_add(bn)))
        as *mut libc::c_char;
    memcpy(p as *mut libc::c_void, a_0, an.wrapping_mul(s));
    memcpy(
        p.offset(an.wrapping_mul(s) as isize) as *mut libc::c_void,
        b_0,
        bn.wrapping_mul(s),
    );
    return p as *mut libc::c_void;
}
#[no_mangle]
pub static mut a: [libc::c_int; 5] = [
    1 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
];
#[no_mangle]
pub static mut b: [libc::c_int; 5] = [
    6 as libc::c_int,
    7 as libc::c_int,
    8 as libc::c_int,
    9 as libc::c_int,
    0 as libc::c_int,
];
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut c: *mut libc::c_int = array_concat(
        a.as_ptr() as *const libc::c_void,
        5 as libc::c_int as size_t,
        b.as_ptr() as *const libc::c_void,
        5 as libc::c_int as size_t,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 10 as libc::c_int as libc::c_uint {
        printf(b"%d\n\0" as *const u8 as *const libc::c_char, *c.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    free(c as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
