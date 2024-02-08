#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wday(
    mut year: libc::c_int,
    mut month: libc::c_int,
    mut day: libc::c_int,
) -> libc::c_int {
    let mut adjustment: libc::c_int = 0;
    let mut mm: libc::c_int = 0;
    let mut yy: libc::c_int = 0;
    adjustment = (14 as libc::c_int - month) / 12 as libc::c_int;
    mm = month + 12 as libc::c_int * adjustment - 2 as libc::c_int;
    yy = year - adjustment;
    return (day + (13 as libc::c_int * mm - 1 as libc::c_int) / 5 as libc::c_int + yy
        + yy / 4 as libc::c_int - yy / 100 as libc::c_int + yy / 400 as libc::c_int)
        % 7 as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut y: libc::c_int = 0;
    y = 2008 as libc::c_int;
    while y <= 2121 as libc::c_int {
        if wday(y, 12 as libc::c_int, 25 as libc::c_int) == 0 as libc::c_int {
            printf(b"%04d-12-25\n\0" as *const u8 as *const libc::c_char, y);
        }
        y += 1;
        y;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
