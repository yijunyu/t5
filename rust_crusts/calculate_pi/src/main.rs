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
#[no_mangle]
pub extern "C" fn agm(in1: i32, in2: i32, mut out1: i32, mut out2: i32) {}

fn main_0() -> i32 {
    mpf_set_default_prec(300000);
    let mut n: i32 = 1;
    let mut i: i32 = 0;
    i = 0;
    while i < 8 {
        n += n;
        n += n;
        i += 1;
        i;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
