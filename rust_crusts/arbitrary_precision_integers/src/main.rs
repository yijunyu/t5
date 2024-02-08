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
fn main_0() -> i32 {
    unsafe {
        let mut len: i32 = 0;
        print!("GMP says size is: {}\n", len);
        let mut s: *mut i8 = 0 as *mut i8;
        len = strlen(s) as i32;
        print!("size really is {}\n", len);
        print!(
            "Digits: {:.20}...{}\n",
            build_str_from_raw_ptr(s as *mut u8),
            build_str_from_raw_ptr(s.offset(len as isize).offset(-(20 as isize)) as *mut u8)
        );
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
