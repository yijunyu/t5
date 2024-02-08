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
pub extern "C" fn bernoulli(mut rop: i32, mut n: u32) {
    let mut m: u32 = 0;
    let mut j: u32 = 0;
    let mut i: u64 = 0;
    m = 0;
    while m <= n {
        j = m;
        while j > 0 {
            j = j.wrapping_sub(1);
            j;
        }
        m = m.wrapping_add(1);
        m;
    }
    let mut i_0: u64 = 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
