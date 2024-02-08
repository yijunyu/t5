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
    fn strlen(_: *const i8) -> u64;
    fn strftime(__s: *mut i8, __maxsize: u64, __format: *const i8, __tp: *const tm) -> u64;
    fn gmtime(__timer: *const i64) -> *mut tm;
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
pub extern "C" fn is_palindrome(mut str: *const i8) -> bool {
    unsafe {
        let mut n: u64 = strlen(str);
        let mut i: u64 = 0;
        while i.wrapping_add(1) < n {
            if *str.offset(i as isize) as i32 != *str.offset(n.wrapping_sub(1) as isize) as i32 {
                return 0 != 0;
            }
            i = i.wrapping_add(1);
            i;
            n = n.wrapping_sub(1);
            n;
        }
        return 1 != 0;
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut timestamp: i64 = rust_time(None);
        let seconds_per_day: i32 = 24 * 60 * 60;
        let mut count: i32 = 15;
        let mut str: [i8; 32] = [0; 32];
        print!("Next {} palindrome dates:\n", count);
        while count > 0 {
            let mut ptr: *mut tm = gmtime(&mut timestamp);
            strftime(
                str.as_mut_ptr(),
                ::core::mem::size_of::<[i8; 32]>() as u64,
                b"%Y%m%d\0" as *const u8 as *const i8,
                ptr,
            );
            if is_palindrome(str.as_mut_ptr()) {
                strftime(
                    str.as_mut_ptr(),
                    ::core::mem::size_of::<[i8; 32]>() as u64,
                    b"%F\0" as *const u8 as *const i8,
                    ptr,
                );
                print!("{}\n", build_str_from_raw_ptr(str.as_mut_ptr() as *mut u8));
                count -= 1;
                count;
            }
            timestamp += seconds_per_day as i64;
        }
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
