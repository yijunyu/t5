#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
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
#[no_mangle]
pub unsafe extern "C" fn ddate(
    mut y: libc::c_int,
    mut d: libc::c_int,
) -> *mut libc::c_char {
    let mut dyear: libc::c_int = 1166 as libc::c_int + y;
    let mut result: *mut libc::c_char = malloc(
        (100 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    if y % 400 as libc::c_int == 0 as libc::c_int
        || y % 4 as libc::c_int == 0 as libc::c_int && y % 100 as libc::c_int != 0
    {
        if d == 60 as libc::c_int {
            sprintf(
                result,
                b"St. Tib's Day, YOLD %d\0" as *const u8 as *const libc::c_char,
                dyear,
            );
            return result;
        } else if d >= 60 as libc::c_int {
            d -= 1;
            d;
        }
    }
    sprintf(
        result,
        b"%s, %s %d, YOLD %d\0" as *const u8 as *const libc::c_char,
        if d % 5 as libc::c_int == 1 as libc::c_int {
            b"Sweetmorn\0" as *const u8 as *const libc::c_char
        } else if d % 5 as libc::c_int == 2 as libc::c_int {
            b"Boomtime\0" as *const u8 as *const libc::c_char
        } else if d % 5 as libc::c_int == 3 as libc::c_int {
            b"Pungenday\0" as *const u8 as *const libc::c_char
        } else if d % 5 as libc::c_int == 4 as libc::c_int {
            b"Prickle-Prickle\0" as *const u8 as *const libc::c_char
        } else {
            b"Setting Orange\0" as *const u8 as *const libc::c_char
        },
        if (if d % 73 as libc::c_int == 0 as libc::c_int {
            d - 1 as libc::c_int
        } else {
            d
        }) / 73 as libc::c_int == 0 as libc::c_int
        {
            b"Chaos\0" as *const u8 as *const libc::c_char
        } else if (if d % 73 as libc::c_int == 0 as libc::c_int {
            d - 1 as libc::c_int
        } else {
            d
        }) / 73 as libc::c_int == 1 as libc::c_int
        {
            b"Discord\0" as *const u8 as *const libc::c_char
        } else if (if d % 73 as libc::c_int == 0 as libc::c_int {
            d - 1 as libc::c_int
        } else {
            d
        }) / 73 as libc::c_int == 2 as libc::c_int
        {
            b"Confusion\0" as *const u8 as *const libc::c_char
        } else if (if d % 73 as libc::c_int == 0 as libc::c_int {
            d - 1 as libc::c_int
        } else {
            d
        }) / 73 as libc::c_int == 3 as libc::c_int
        {
            b"Bureaucracy\0" as *const u8 as *const libc::c_char
        } else {
            b"The Aftermath\0" as *const u8 as *const libc::c_char
        },
        if d % 73 as libc::c_int == 0 as libc::c_int {
            73 as libc::c_int
        } else {
            d % 73 as libc::c_int
        },
        dyear,
    );
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn day_of_year(
    mut y: libc::c_int,
    mut m: libc::c_int,
    mut d: libc::c_int,
) -> libc::c_int {
    let mut month_lengths: [libc::c_int; 12] = [
        31 as libc::c_int,
        28 as libc::c_int,
        31 as libc::c_int,
        30 as libc::c_int,
        31 as libc::c_int,
        30 as libc::c_int,
        31 as libc::c_int,
        31 as libc::c_int,
        30 as libc::c_int,
        31 as libc::c_int,
        30 as libc::c_int,
        31 as libc::c_int,
    ];
    while m > 1 as libc::c_int {
        d += month_lengths[(m - 2 as libc::c_int) as usize];
        if m == 3 as libc::c_int
            && (y % 400 as libc::c_int == 0 as libc::c_int
                || y % 4 as libc::c_int == 0 as libc::c_int
                    && y % 100 as libc::c_int != 0)
        {
            d += 1;
            d;
        }
        m -= 1;
        m;
    }
    return d;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut now: time_t = 0;
    let mut now_time: *mut tm = 0 as *mut tm;
    let mut year: libc::c_int = 0;
    let mut doy: libc::c_int = 0;
    if argc == 1 as libc::c_int {
        now = time(0 as *mut time_t);
        now_time = localtime(&mut now);
        year = (*now_time).tm_year + 1900 as libc::c_int;
        doy = (*now_time).tm_yday + 1 as libc::c_int;
    } else if argc == 4 as libc::c_int {
        year = atoi(*argv.offset(1 as libc::c_int as isize));
        doy = day_of_year(
            atoi(*argv.offset(1 as libc::c_int as isize)),
            atoi(*argv.offset(2 as libc::c_int as isize)),
            atoi(*argv.offset(3 as libc::c_int as isize)),
        );
    }
    let mut result: *mut libc::c_char = ddate(year, doy);
    puts(result);
    free(result as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
