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
    fn rand() -> i32;
    fn cos(_: f64) -> f64;
    fn log(_: f64) -> f64;
    fn sqrt(_: f64) -> f64;
}
#[no_mangle]
pub extern "C" fn drand() -> f64 {
    unsafe {
        return (rand() as f64 + 1.0f64) / (2147483647 as f64 + 1.0f64);
    }
}

#[no_mangle]
pub extern "C" fn random_normal() -> f64 {
    unsafe {
        return sqrt(-2i32 as f64 * log(drand()))
            * cos(2 as f64 * 3.14159265358979323846f64 * drand());
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut rands: [f64; 1000] = [0.; 1000];
    i = 0;
    while i < 1000 {
        rands[i as usize] = 1.0f64 + 0.5f64 * random_normal();
        i += 1;
        i;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
