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
pub extern "C" fn evolve(mut state: u64, mut rule: i32) {
    let mut i: i32 = 0;
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    let mut b: i32 = 0;
    p = 0;
    while p < 10 {
        b = 0;
        q = 8;
        loop {
            let fresh0 = q;
            q = q - 1;
            if !(fresh0 != 0) {
                break;
            }
            let mut st: u64 = state;
            b = (b as u64 | (st & 1u64) << q) as i32;
            i = 0;
            state = i as u64;
            while (i as u64) < (::core::mem::size_of::<u64>() as u64).wrapping_mul(8) {
                if rule as u64
                    & 1u64
                        << (7
                            & (st >> i - 1
                                | st << (::core::mem::size_of::<u64>() as u64)
                                    .wrapping_mul(8)
                                    .wrapping_add(1)
                                    .wrapping_sub(i as u64)))
                    != 0
                {
                    state |= 1 << i;
                }
                i += 1;
                i;
            }
        }
        print!(" {}", b);
        p += 1;
        p;
    }
    print!("{}", '\n' as i32);
}

fn main_0() -> i32 {
    evolve(1, 30);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
