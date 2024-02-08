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
pub extern "C" fn self_desc(mut xx: u64) -> i32 {
    let mut d: u32 = 0;
    let mut x: u32 = 0;
    let mut cnt: [u8; 10] = [0; 10];
    let mut dig: [u8; 10] = [0; 10];
    d = 0;
    while xx > !0 as u64 {
        let fresh0 = d;
        d = d.wrapping_add(1);
        dig[fresh0 as usize] = xx.wrapping_rem(10) as u8;
        cnt[dig[fresh0 as usize] as usize] = (cnt[dig[fresh0 as usize] as usize]).wrapping_add(1);
        cnt[dig[fresh0 as usize] as usize];
        xx = xx.wrapping_div(10);
    }
    x = xx as u32;
    while x != 0 {
        let fresh1 = d;
        d = d.wrapping_add(1);
        dig[fresh1 as usize] = x.wrapping_rem(10) as u8;
        cnt[dig[fresh1 as usize] as usize] = (cnt[dig[fresh1 as usize] as usize]).wrapping_add(1);
        cnt[dig[fresh1 as usize] as usize];
        x = x.wrapping_div(10);
    }
    loop {
        let fresh2 = d;
        d = d.wrapping_sub(1);
        if !(fresh2 != 0 && {
            let fresh3 = x;
            x = x.wrapping_add(1);
            dig[fresh3 as usize] as i32 == cnt[d as usize] as i32
        }) {
            break;
        }
    }
    return (d == (-1) as i32 as u32) as i32;
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    i = 1;
    while i < 100000000 {
        if self_desc(i as u64) != 0 {
            print!("{}\n", i);
        }
        i += 1;
        i;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
