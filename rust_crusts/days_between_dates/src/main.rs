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
    fn strlen(_: *const i8) -> u64;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct date {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}
#[no_mangle]
pub extern "C" fn extractDate(mut str: *mut i8) -> date {
    unsafe {
        return {
            let mut init = date {
                year: 1000 * (*str.offset(0 as isize) as i32 - '0' as i32)
                    + 100 * (*str.offset(1 as isize) as i32 - '0' as i32)
                    + 10 * (*str.offset(2 as isize) as i32 - '0' as i32)
                    + (*str.offset(3 as isize) as i32 - '0' as i32),
                month: 10 * (*str.offset(5 as isize) as i32 - '0' as i32)
                    + (*str.offset(6 as isize) as i32 - '0' as i32),
                day: 10 * (*str.offset(8 as isize) as i32 - '0' as i32)
                    + (*str.offset(9 as isize) as i32 - '0' as i32),
            };
            init
        };
    }
}

#[no_mangle]
pub extern "C" fn isValidDate(mut str: *mut i8) -> bool {
    unsafe {
        let mut newDate: date = date {
            year: 0,
            month: 0,
            day: 0,
        };
        if strlen(str) != 10
            && *str.offset(4 as isize) as i32 != '-' as i32
            && *str.offset(7 as isize) as i32 != '-' as i32
        {
            return 0 != 0;
        }
        newDate = extractDate(str);
        if newDate.year <= 0
            || newDate.month <= 0
            || newDate.day <= 0
            || newDate.month > 12
            || newDate.month == 2 && newDate.day > 29
            || (newDate.month == 1
                || newDate.month == 3
                || newDate.month == 5
                || newDate.month == 7
                || newDate.month == 8
                || newDate.month == 10
                || newDate.month == 12)
                && newDate.day > 31
            || newDate.day > 30
            || newDate.year % 4 == 0 && newDate.month == 2 && newDate.month > 28
        {
            return 0 != 0;
        }
        return 1 != 0;
    }
}

#[no_mangle]
pub extern "C" fn diffDays(mut date1: date, mut date2: date) -> i32 {
    let mut days1: i32 = 0;
    let mut days2: i32 = 0;
    date1.month = (date1.month + 9) % 12;
    date1.year = date1.year - date1.month / 10;
    date2.month = (date2.month + 9) % 12;
    date2.year = date2.year - date2.month / 10;
    days1 = 365 * date1.year + date1.year / 4 - date1.year / 100
        + date1.year / 400
        + (date1.month * 306 + 5) / 10
        + (date1.day - 1);
    days2 = 365 * date2.year + date2.year / 4 - date2.year / 100
        + date2.year / 400
        + (date2.month * 306 + 5) / 10
        + (date2.day - 1);
    return days2 - days1;
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        if argc != 3 {
            let str_to_print = format!(
                "Usage : {} <yyyy-mm-dd> <yyyy-mm-dd>",
                build_str_from_raw_ptr(*argv.offset(0 as isize) as *mut u8)
            );
            print!("{}", str_to_print);
            return str_to_print.chars().count() as i32;
        }
        if isValidDate(*argv.offset(1 as isize)) as i32 != 0
            && isValidDate(*argv.offset(2 as isize)) as i32 == 0
        {
            let str_to_print = format!("Dates are invalid.\n");
            print!("{}", str_to_print);
            return str_to_print.chars().count() as i32;
        }
        print!(
            "Days Difference : {}\n",
            diffDays(
                extractDate(*argv.offset(1 as isize)),
                extractDate(*argv.offset(2 as isize)),
            )
        );
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
