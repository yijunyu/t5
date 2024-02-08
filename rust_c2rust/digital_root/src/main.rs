#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn droot(
    mut x: libc::c_longlong,
    mut base: libc::c_int,
    mut pers: *mut libc::c_int,
) -> libc::c_int {
    let mut d: libc::c_int = 0 as libc::c_int;
    if !pers.is_null() {
        *pers = 0 as libc::c_int;
        while x >= base as libc::c_longlong {
            d = 0 as libc::c_int;
            while x != 0 {
                d = (d as libc::c_longlong + x % base as libc::c_longlong)
                    as libc::c_int;
                x /= base as libc::c_longlong;
            }
            x = d as libc::c_longlong;
            *pers += 1;
            *pers;
        }
    } else if x != 0
        && {
            d = (x % (base - 1 as libc::c_int) as libc::c_longlong) as libc::c_int;
            d == 0
        }
    {
        d = base - 1 as libc::c_int;
    }
    return d;
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut pers: libc::c_int = 0;
    let mut x: [libc::c_longlong; 4] = [
        627615 as libc::c_int as libc::c_longlong,
        39390 as libc::c_int as libc::c_longlong,
        588225 as libc::c_int as libc::c_longlong,
        393900588225 as libc::c_longlong,
    ];
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        d = droot(x[i as usize], 10 as libc::c_int, &mut pers);
        printf(
            b"%lld: pers %d, root %d\n\0" as *const u8 as *const libc::c_char,
            x[i as usize],
            pers,
            d,
        );
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
