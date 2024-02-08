#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use std::time::SystemTime;
pub fn rust_time(ref_result: Option<&mut i64>) -> i64 {
    let result = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };
    match ref_result {
        Some(r) => *r = result,
        None => {}
    }
    return result as i64;
}

use c2rust_out::*;
extern "C" {
    fn atoi(__nptr: *const i8) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn puts(__s: *const i8) -> i32;
    fn localtime(__timer: *const i64) -> *mut tm;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const i8,
}
#[no_mangle]
pub extern "C" fn ddate(mut y: i32, mut d: i32) -> *mut i8 {
    unsafe {
        let mut dyear: i32 = 1166 + y;
        let mut result: *mut i8 =
            malloc(100u64.wrapping_mul(::core::mem::size_of::<i8>() as u64)) as *mut i8;
        if y % 400 == 0 || y % 4 == 0 && y % 100 != 0 {
            if d == 60 {
                sprintf(
                    result,
                    b"St. Tib's Day, YOLD %d\0" as *const u8 as *const i8,
                    dyear,
                );
                return result;
            } else if d >= 60 {
                d -= 1;
                d;
            }
        }
        sprintf(
            result,
            b"%s, %s %d, YOLD %d\0" as *const u8 as *const i8,
            if d % 5 == 1 {
                b"Sweetmorn\0" as *const u8 as *const i8
            } else if d % 5 == 2 {
                b"Boomtime\0" as *const u8 as *const i8
            } else if d % 5 == 3 {
                b"Pungenday\0" as *const u8 as *const i8
            } else if d % 5 == 4 {
                b"Prickle-Prickle\0" as *const u8 as *const i8
            } else {
                b"Setting Orange\0" as *const u8 as *const i8
            },
            if (if d % 73 == 0 { d - 1 } else { d }) / 73 == 0 {
                b"Chaos\0" as *const u8 as *const i8
            } else if (if d % 73 == 0 { d - 1 } else { d }) / 73 == 1 {
                b"Discord\0" as *const u8 as *const i8
            } else if (if d % 73 == 0 { d - 1 } else { d }) / 73 == 2 {
                b"Confusion\0" as *const u8 as *const i8
            } else if (if d % 73 == 0 { d - 1 } else { d }) / 73 == 3 {
                b"Bureaucracy\0" as *const u8 as *const i8
            } else {
                b"The Aftermath\0" as *const u8 as *const i8
            },
            if d % 73 == 0 { 73 } else { d % 73 },
            dyear,
        );
        return result;
    }
}

#[no_mangle]
pub extern "C" fn day_of_year(mut y: i32, mut m: i32, mut d: i32) -> i32 {
    let mut month_lengths: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    while m > 1 {
        d += month_lengths[(m - 2i32) as usize];
        if m == 3 && (y % 400 == 0 || y % 4 == 0 && y % 100 != 0) {
            d += 1;
            d;
        }
        m -= 1;
        m;
    }
    return d;
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut now: i64 = 0;
        let mut now_time: *mut tm = 0 as *mut tm;
        let mut year: i32 = 0;
        let mut doy: i32 = 0;
        if argc == 1 {
            now = rust_time(None);
            now_time = localtime(&mut now);
            year = (*now_time).tm_year + 1900;
            doy = (*now_time).tm_yday + 1;
        } else if argc == 4 {
            year = atoi(*argv.offset(1 as isize));
            doy = day_of_year(
                atoi(*argv.offset(1 as isize)),
                atoi(*argv.offset(2 as isize)),
                atoi(*argv.offset(3 as isize)),
            );
        }
        let mut result: *mut i8 = ddate(year, doy);
        puts(result);
        free(result as *mut libc::c_void);
        return 0;
    }
}

pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    ::std::process::exit(main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32);
}
