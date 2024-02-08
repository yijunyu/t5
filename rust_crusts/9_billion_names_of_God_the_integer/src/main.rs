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
pub static mut p: [i32; 100001] = [0; 100001];
#[no_mangle]
pub extern "C" fn calc(mut n: i32) {
    let mut k: i32 = 1;
    while k <= n {
        let mut d: i32 = n - k * (3 * k - 1) / 2;
        if d < 0 {
            break;
        }
        d -= k;
        if d < 0 {
            break;
        }
        k += 1;
        k;
    }
}

fn main_0() -> i32 {
    let mut idx: [i32; 10] = [23, 123, 1234, 12345, 20000, 30000, 40000, 50000, 100000, 0];
    let mut at: i32 = 0;
    let mut i: i32 = 1;
    while idx[at as usize] != 0 {
        calc(i);
        if !(i != idx[at as usize]) {
            at += 1;
            at;
        }
        i += 1;
        i;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
