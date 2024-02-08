#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value)]
use c2rust_out::*;
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
}
fn main_0() -> i32 {
    let mut a: i32 = 0;
    unsafe {
        if a == 42 {
        } else {
            __assert_fail(
                b"a == 42\0" as *const u8 as *const i8,
                b"main.c\0" as *const u8 as *const i8,
                6,
                (*::core::mem::transmute::<&[u8; 11], &[i8; 11]>(b"int main()\0")).as_ptr(),
            );
        }
        'c_59: {
            if a == 42 {
            } else {
                __assert_fail(
                    b"a == 42\0" as *const u8 as *const i8,
                    b"main.c\0" as *const u8 as *const i8,
                    6,
                    (*::core::mem::transmute::<&[u8; 11], &[i8; 11]>(b"int main()\0")).as_ptr(),
                );
            }
        };
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
