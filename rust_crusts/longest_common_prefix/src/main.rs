#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic)]
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
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn malloc(_: u64) -> *mut libc::c_void;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
#[no_mangle]
pub unsafe extern "C" fn lcp(mut num: i32, mut args: ...) -> *mut i8 {
    let mut vaList: ::core::ffi::VaListImpl;
    let mut vaList2: ::core::ffi::VaListImpl;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut len: i32 = 0;
    let mut min: i32 = 0;
    let mut dest: *mut i8 = 0 as *mut i8;
    let mut strings: *mut *mut i8 =
        malloc((num as u64).wrapping_mul(::core::mem::size_of::<*mut i8>() as u64)) as *mut *mut i8;
    vaList = args.clone();
    vaList2 = args.clone();
    i = 0;
    while i < num {
        len = strlen(vaList.arg::<*mut i8>()) as i32;
        let ref mut fresh0 = *strings.offset(i as isize);
        *fresh0 = malloc(((len + 1i32) as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64))
            as *mut i8;
        strcpy(*strings.offset(i as isize), vaList2.arg::<*mut i8>());
        if i == 0 {
            min = len;
        } else if len < min {
            min = len;
        }
        i += 1;
        i;
    }
    if min == 0 {
        return b"\0" as *const u8 as *const i8 as *mut i8;
    }
    i = 0;
    while i < min {
        j = 1;
        while j < num {
            if *(*strings.offset(j as isize)).offset(i as isize) as i32
                != *(*strings.offset(0 as isize)).offset(i as isize) as i32
            {
                if i == 0 {
                    return b"\0" as *const u8 as *const i8 as *mut i8;
                } else {
                    dest = malloc((i as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64))
                        as *mut i8;
                    strncpy(dest, *strings.offset(0 as isize), (i - 1i32) as u64);
                    return dest;
                }
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    dest =
        malloc(((min + 1i32) as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64)) as *mut i8;
    strncpy(dest, *strings.offset(0 as isize), min as u64);
    return dest;
}

fn main_0() -> i32 {
    unsafe {
        print!(
            "\nLongest common prefix : {}",
            build_str_from_raw_ptr(
                lcp(3, "interspecies\0", "interstellar\0", "interstate\0",) as *mut u8
            )
        );
        print!(
            "\nLongest common prefix : {}",
            build_str_from_raw_ptr(lcp(2, "throne\0", "throne\0",) as *mut u8)
        );
        print!(
            "\nLongest common prefix : {}",
            build_str_from_raw_ptr(lcp(2, "throne\0", "dungeon\0",) as *mut u8)
        );
        print!(
            "\nLongest common prefix : {}",
            build_str_from_raw_ptr(lcp(3, "throne\0", "\0", "throne\0",) as *mut u8)
        );
        print!(
            "\nLongest common prefix : {}",
            build_str_from_raw_ptr(lcp(1, "cheese\0") as *mut u8)
        );
        print!(
            "\nLongest common prefix : {}",
            build_str_from_raw_ptr(lcp(1, "\0") as *mut u8)
        );
        print!(
            "\nLongest common prefix : {}",
            build_str_from_raw_ptr(lcp(0, 0 as *mut libc::c_void) as *mut u8)
        );
        print!(
            "\nLongest common prefix : {}",
            build_str_from_raw_ptr(lcp(2, "prefix\0", "suffix\0",) as *mut u8)
        );
        print!(
            "\nLongest common prefix : {}",
            build_str_from_raw_ptr(lcp(2, "foo\0", "foobar\0",) as *mut u8)
        );
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
