#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.offset(str_size as isize) != 0 {
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}

use c2rust_out::*;
extern "C" {
    fn mktime(__tp: *mut tm) -> i64;
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
static mut months: [*const i8; 12] = [
    b"January\0" as *const u8 as *const i8,
    b"February\0" as *const u8 as *const i8,
    b"March\0" as *const u8 as *const i8,
    b"April\0" as *const u8 as *const i8,
    b"May\0" as *const u8 as *const i8,
    b"June\0" as *const u8 as *const i8,
    b"July\0" as *const u8 as *const i8,
    b"August\0" as *const u8 as *const i8,
    b"September\0" as *const u8 as *const i8,
    b"October\0" as *const u8 as *const i8,
    b"November\0" as *const u8 as *const i8,
    b"December\0" as *const u8 as *const i8,
];
static mut long_months: [i32; 7] = [0, 2, 4, 6, 7, 9, 11];
fn main_0() -> i32 {
    let mut n: i32 = 0;
    let mut y: i32 = 0;
    let mut i: i32 = 0;
    let mut m: i32 = 0;
    let mut t: tm = {
        let mut init = tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: 0 as *const i8,
        };
        init
    };
    print!("Months with five weekends:\n");
    y = 1900;
    unsafe {
        while y <= 2100 {
            i = 0;
            while i < 7 {
                m = long_months[i as usize];
                t.tm_year = y - 1900;
                t.tm_mon = m;
                t.tm_mday = 1;
                if mktime(&mut t) == -1 as i64 {
                    print!(
                        "Error: {} {}\n",
                        y,
                        build_str_from_raw_ptr(months[m as usize] as *mut u8)
                    );
                } else if t.tm_wday == 5 {
                    print!(
                        "  {} {}\n",
                        y,
                        build_str_from_raw_ptr(months[m as usize] as *mut u8)
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
    }
    print!("{} total\n", n);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
