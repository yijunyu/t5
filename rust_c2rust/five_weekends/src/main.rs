#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn mktime(__tp: *mut tm) -> time_t;
}
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
static mut months: [*const libc::c_char; 12] = [
    b"January\0" as *const u8 as *const libc::c_char,
    b"February\0" as *const u8 as *const libc::c_char,
    b"March\0" as *const u8 as *const libc::c_char,
    b"April\0" as *const u8 as *const libc::c_char,
    b"May\0" as *const u8 as *const libc::c_char,
    b"June\0" as *const u8 as *const libc::c_char,
    b"July\0" as *const u8 as *const libc::c_char,
    b"August\0" as *const u8 as *const libc::c_char,
    b"September\0" as *const u8 as *const libc::c_char,
    b"October\0" as *const u8 as *const libc::c_char,
    b"November\0" as *const u8 as *const libc::c_char,
    b"December\0" as *const u8 as *const libc::c_char,
];
static mut long_months: [libc::c_int; 7] = [
    0 as libc::c_int,
    2 as libc::c_int,
    4 as libc::c_int,
    6 as libc::c_int,
    7 as libc::c_int,
    9 as libc::c_int,
    11 as libc::c_int,
];
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut t: tm = {
        let mut init = tm {
            tm_sec: 0 as libc::c_int,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: 0 as *const libc::c_char,
        };
        init
    };
    printf(b"Months with five weekends:\n\0" as *const u8 as *const libc::c_char);
    y = 1900 as libc::c_int;
    while y <= 2100 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 7 as libc::c_int {
            m = long_months[i as usize];
            t.tm_year = y - 1900 as libc::c_int;
            t.tm_mon = m;
            t.tm_mday = 1 as libc::c_int;
            if mktime(&mut t) == -(1 as libc::c_int) as libc::c_long {
                printf(
                    b"Error: %d %s\n\0" as *const u8 as *const libc::c_char,
                    y,
                    months[m as usize],
                );
            } else if t.tm_wday == 5 as libc::c_int {
                printf(
                    b"  %d %s\n\0" as *const u8 as *const libc::c_char,
                    y,
                    months[m as usize],
                );
                n += 1;
                n;
            }
            i += 1;
            i;
        }
        y += 1;
        y;
    }
    printf(b"%d total\n\0" as *const u8 as *const libc::c_char, n);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
