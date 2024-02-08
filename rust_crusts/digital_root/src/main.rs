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
#[no_mangle]
pub extern "C" fn droot(mut x: i64, mut base: i32, mut pers: *mut i32) -> i32 {
    unsafe {
        let mut d: i32 = 0;
        if !pers.is_null() {
            *pers = 0;
            while x >= base as i64 {
                d = 0;
                while x != 0 {
                    d = (d as i64 + x % base as i64) as i32;
                    x /= base as i64;
                }
                x = d as i64;
                *pers += 1;
                *pers;
            }
        } else if x != 0 && {
            d = (x % (base - 1i32) as i64) as i32;
            d == 0
        } {
            d = base - 1;
        }
        return d;
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut d: i32 = 0;
    let mut pers: i32 = 0;
    let mut x: [i64; 4] = [627615, 39390, 588225, 393900588225];
    i = 0;
    while i < 4 {
        d = droot(x[i as usize], 10, &mut pers);
        print!("{}: pers {}, root {}\n", x[i as usize], pers, d);
        i += 1;
        i;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
