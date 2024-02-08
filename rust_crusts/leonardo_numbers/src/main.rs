#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use c2rust_out::*;
extern "C" {
    fn scanf(_: *const i8, _: ...) -> i32;
}
#[no_mangle]
pub extern "C" fn leonardo(mut a: i32, mut b: i32, mut step: i32, mut num: i32) {
    let mut i: i32 = 0;
    let mut temp: i32 = 0;
    print!("First 25 Leonardo numbers : \n");
    i = 1;
    while i <= num {
        if i == 1 {
            print!(" {}", a);
        } else if i == 2 {
            print!(" {}", b);
        } else {
            print!(" {}", a + b + step);
            temp = a;
            a = b;
            b = temp + b + step;
        }
        i += 1;
        i;
    }
}

fn main_0() -> i32 {
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut step: i32 = 0;
    print!("Enter first two Leonardo numbers and increment step : ");
    unsafe {
        scanf(
            b"%d%d%d\0" as *const u8 as *const i8,
            &mut a as *mut i32,
            &mut b as *mut i32,
            &mut step as *mut i32,
        );
    }
    leonardo(a, b, step, 25);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
