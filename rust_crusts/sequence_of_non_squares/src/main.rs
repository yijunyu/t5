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
    fn sqrt(_: f64) -> f64;
    fn floor(_: f64) -> f64;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
}
#[no_mangle]
pub extern "C" fn nonsqr(mut n: i32) -> i32 {
    unsafe {
        return n + (0.5f64 + sqrt(n as f64)) as i32;
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    i = 1;
    while i < 23 {
        print!("{} ", nonsqr(i));
        i += 1;
        i;
    }
    print!("\n");
    i = 1;
    unsafe {
        while i < 1000000 {
            let mut j: f64 = sqrt(nonsqr(i) as f64);
            if j != floor(j) {
            } else {
                __assert_fail(
                    b"j != floor(j)\0" as *const u8 as *const i8,
                    b"main.c\0" as *const u8 as *const i8,
                    21,
                    (*::core::mem::transmute::<&[u8; 11], &[i8; 11]>(b"int main()\0")).as_ptr(),
                );
            }
            'c_1861: {
                if j != floor(j) {
                } else {
                    __assert_fail(
                        b"j != floor(j)\0" as *const u8 as *const i8,
                        b"main.c\0" as *const u8 as *const i8,
                        21,
                        (*::core::mem::transmute::<&[u8; 11], &[i8; 11]>(b"int main()\0")).as_ptr(),
                    );
                }
            };
            i += 1;
            i;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
