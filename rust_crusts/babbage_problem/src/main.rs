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
extern "C" {}
fn main_0() -> i32 {
    let mut current: i32 = 0;
    let mut square: i32 = 0;
    loop {
        square = current * current;
        if !(square % 1000000 != 269696 && square < 2147483647) {
            break;
        }
        current += 1;
        current;
    }
    if square > 2147483647 {
        print!("Condition not satisfied before INT_MAX reached.");
    } else {
        print!(
            "The smallest number whose square ends in 269696 is {}\n",
            current
        );
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
