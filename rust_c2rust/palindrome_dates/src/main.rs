#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn time(__timer: *mut time_t) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn gmtime(__timer: *const time_t) -> *mut tm;
}
pub type size_t = libc::c_ulong;
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
#[no_mangle]
pub unsafe extern "C" fn is_palindrome(mut str: *const libc::c_char) -> bool {
    let mut n: size_t = strlen(str);
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i.wrapping_add(1 as libc::c_int as libc::c_ulong) < n {
        if *str.offset(i as isize) as libc::c_int
            != *str.offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
        {
            return 0 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
        i;
        n = n.wrapping_sub(1);
        n;
    }
    return 1 as libc::c_int != 0;
}
unsafe fn main_0() -> libc::c_int {
    let mut timestamp: time_t = time(0 as *mut time_t);
    let seconds_per_day: libc::c_int = 24 as libc::c_int * 60 as libc::c_int
        * 60 as libc::c_int;
    let mut count: libc::c_int = 15 as libc::c_int;
    let mut str: [libc::c_char; 32] = [0; 32];
    printf(b"Next %d palindrome dates:\n\0" as *const u8 as *const libc::c_char, count);
    while count > 0 as libc::c_int {
        let mut ptr: *mut tm = gmtime(&mut timestamp);
        strftime(
            str.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%Y%m%d\0" as *const u8 as *const libc::c_char,
            ptr,
        );
        if is_palindrome(str.as_mut_ptr()) {
            strftime(
                str.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                b"%F\0" as *const u8 as *const libc::c_char,
                ptr,
            );
            printf(b"%s\n\0" as *const u8 as *const libc::c_char, str.as_mut_ptr());
            count -= 1;
            count;
        }
        timestamp += seconds_per_day as libc::c_long;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
