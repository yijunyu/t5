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
}
#[no_mangle]
pub extern "C" fn randn(mut m: i32) -> i32 {
    let mut rand_max: i32 = 2147483647 - 2147483647 % m;
    let mut r: i32 = 0;
    unsafe {
        loop {
            r = rand();
            if !(r > rand_max) {
                break;
            }
        }
    }
    return r / (rand_max / m);
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut r2: i32 = 0;
    let mut buf: [u64; 31] = [0; 31];
    i = 0;
    while i < 100 {
        x = randn(31) - 15;
        y = randn(31) - 15;
        r2 = x * x + y * y;
        if r2 >= 100 && r2 <= 225 {
            buf[(15 + y) as usize] |= (1i32 << x + 15) as u64;
            i += 1;
            i;
        }
    }
    y = 0;
    while y < 31 {
        x = 0;
        while x < 31 {
            if buf[y as usize] & (1i32 << x) as u64 != 0 {
                print!(". ")
            } else {
                print!("  ")
            };
            x += 1;
            x;
        }
        print!("\n");
        y += 1;
        y;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
