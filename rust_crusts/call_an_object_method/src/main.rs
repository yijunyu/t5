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
    fn atoi(__nptr: *const i8) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct functionPair {
    pub x: i32,
    pub funcPtr: Option<unsafe extern "C" fn(i32) -> i32>,
}
#[no_mangle]
pub extern "C" fn factorial(mut num: i32) -> i32 {
    if num == 0 || num == 1 {
        return 1;
    } else {
        return num * factorial(num - 1);
    };
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut response: functionPair = functionPair {
            x: 0,
            funcPtr: None,
        };
        if argc != 2 {
            let str_to_print = format!(
                "Usage : {} <non negative integer>",
                build_str_from_raw_ptr(*argv.offset(0 as isize) as *mut u8)
            );
            print!("{}", str_to_print);
            return str_to_print.chars().count() as i32;
        } else {
            response = {
                let mut init = functionPair {
                    x: atoi(*argv.offset(1 as isize)),
                    funcPtr: Some(factorial as unsafe extern "C" fn(i32) -> i32),
                };
                init
            };
            print!(
                "\nFactorial of {} is {}\n",
                response.x,
                (response.funcPtr).expect("non-null function pointer")(response.x)
            );
        }
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
