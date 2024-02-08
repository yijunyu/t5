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
fn main_0() -> i32 {
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    unsafe {
        scanf(
            b"%d %d\0" as *const u8 as *const i8,
            &mut a as *mut i32,
            &mut b as *mut i32,
        );
    }
    if a < b {
        print!("{} is less than {}\n", a, b);
    }
    if a == b {
        print!("{} is equal to {}\n", a, b);
    }
    if a > b {
        print!("{} is greater than {}\n", a, b);
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
