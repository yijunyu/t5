#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[macro_use]
extern crate num_traits;

use c2rust_out::*;
use num_traits::ToPrimitive;
extern "C" {
    fn powl(_: f128::f128, _: f128::f128) -> f128::f128;
}
#[no_mangle]
pub extern "C" fn root(mut base: u64, mut n: u64) -> u64 {
    let mut n1: u64 = 0;
    let mut n2: u64 = 0;
    let mut n3: u64 = 0;
    let mut c: u64 = 0;
    let mut d: u64 = 0;
    let mut e: u64 = 0;
    if base < 2 {
        return base;
    }
    if n == 0 {
        return 1;
    }
    n1 = n.wrapping_sub(1);
    n2 = n;
    n3 = n1;
    c = 1;
    d = n3.wrapping_add(base).wrapping_div(n2);
    unsafe {
        e = n3
            .wrapping_mul(d)
            .wrapping_add(
                base.wrapping_div(
                    (powl(f128::f128::new(d), f128::f128::new(n1)))
                        .to_u64()
                        .unwrap(),
                ),
            )
            .wrapping_div(n2);
        while c != d && c != e {
            c = d;
            d = e;
            e = n3
                .wrapping_mul(e)
                .wrapping_add(
                    base.wrapping_div(
                        (powl(f128::f128::new(e), f128::f128::new(n1)))
                            .to_u64()
                            .unwrap(),
                    ),
                )
                .wrapping_div(n2);
        }
    }
    if d < e {
        return d;
    }
    return e;
}

fn main_0() -> i32 {
    let mut b: u64 = 2e18f64 as u64;
    print!("3rd root of 8 = {}\n", root(8, 3));
    print!("3rd root of 9 = {}\n", root(9, 3));
    print!("2nd root of {} = {}\n", b, root(b, 2));
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
