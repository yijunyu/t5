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
use c2rust_out::*;
extern "C" {}
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
pub extern "C" fn factorial(mut n: i32) -> i64 {
    if n > 1 {
        return n as i64 * factorial(n - 1);
    }
    return 1;
}

#[no_mangle]
pub unsafe extern "C" fn sumOfFactorials(mut num: i32, mut args: ...) -> i64 {
    let mut vaList: ::core::ffi::VaListImpl;
    let mut sum: i64 = 0;
    vaList = args.clone();
    loop {
        let fresh0 = num;
        num = num - 1;
        if !(fresh0 != 0) {
            break;
        }
        sum += factorial(vaList.arg::<i32>());
    }
    return sum;
}

fn main_0() -> i32 {
    unsafe {
        print!(
            "\nSum of factorials of [1,5] : {}",
            sumOfFactorials(5, 1, 2, 3, 4, 5)
        );
        print!(
            "\nSum of factorials of [3,5] : {}",
            sumOfFactorials(3, 3, 4, 5)
        );
        print!(
            "\nSum of factorials of [1,3] : {}",
            sumOfFactorials(3, 1, 2, 3)
        );
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
