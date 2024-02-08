#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn x(mut i: libc::c_double) -> libc::c_double {
    static mut _n: libc::c_double = 1.0f64;
    _n += i;
    return _n;
}
#[no_mangle]
pub unsafe extern "C" fn y(mut i: libc::c_int) -> libc::c_int {
    static mut _n: libc::c_int = 3 as libc::c_int;
    _n += i;
    return _n;
}
#[no_mangle]
pub unsafe extern "C" fn z(mut i: libc::c_int) -> libc::c_int {
    static mut _n: libc::c_int = 'a' as i32;
    _n += i;
    return _n;
}
unsafe fn main_0() -> libc::c_int {
    printf(
        b"%f\n\0" as *const u8 as *const libc::c_char,
        x(5 as libc::c_int as libc::c_double),
    );
    printf(b"%f\n\0" as *const u8 as *const libc::c_char, x(2.3f64));
    printf(b"%i\n\0" as *const u8 as *const libc::c_char, y(5.0f64 as libc::c_int));
    printf(b"%i\n\0" as *const u8 as *const libc::c_char, y(3.3f64 as libc::c_int));
    printf(b"%c\n\0" as *const u8 as *const libc::c_char, z(5 as libc::c_int));
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
