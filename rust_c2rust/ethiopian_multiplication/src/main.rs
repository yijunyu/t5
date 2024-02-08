#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn halve(mut x: *mut libc::c_int) {
    *x >>= 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn doublit(mut x: *mut libc::c_int) {
    *x <<= 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn iseven(x: libc::c_int) -> bool {
    return x & 1 as libc::c_int == 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ethiopian(
    mut plier: libc::c_int,
    mut plicand: libc::c_int,
    tutor: bool,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    if tutor {
        printf(
            b"ethiopian multiplication of %d by %d\n\0" as *const u8
                as *const libc::c_char,
            plier,
            plicand,
        );
    }
    while plier >= 1 as libc::c_int {
        if iseven(plier) {
            if tutor {
                printf(
                    b"%4d %6d struck\n\0" as *const u8 as *const libc::c_char,
                    plier,
                    plicand,
                );
            }
        } else {
            if tutor {
                printf(
                    b"%4d %6d kept\n\0" as *const u8 as *const libc::c_char,
                    plier,
                    plicand,
                );
            }
            result += plicand;
        }
        halve(&mut plier);
        doublit(&mut plicand);
    }
    return result;
}
unsafe fn main_0() -> libc::c_int {
    printf(
        b"%d\n\0" as *const u8 as *const libc::c_char,
        ethiopian(17 as libc::c_int, 34 as libc::c_int, 1 as libc::c_int != 0),
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
