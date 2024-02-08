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
pub extern "C" fn ackermann(mut m: i32, mut n: i32) -> i32 {
    if m == 0 {
        return n + 1;
    }
    if n == 0 {
        return ackermann(m - 1, 1);
    }
    return ackermann(m - 1, ackermann(m, n - 1));
}

fn main_0() -> i32 {
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    m = 0;
    while m <= 4 {
        n = 0;
        while n < 6 - m {
            print!("A({}, {}) = {}\n", m, n, ackermann(m, n));
            n += 1;
            n;
        }
        m += 1;
        m;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
