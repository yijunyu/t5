#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value)]
use c2rust_out::*;
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
}
#[no_mangle]
pub extern "C" fn egyptian_division(
    mut dividend: u64,
    mut divisor: u64,
    mut remainder: *mut u64,
) -> u64 {
    unsafe {
        static mut powers: [u64; 64] = [0; 64];
        static mut doublings: [u64; 64] = [0; 64];
        let mut i: i32 = 0;
        i = 0;
        while i < 64 {
            powers[i as usize] = (1i32 << i) as u64;
            doublings[i as usize] = divisor << i;
            if doublings[i as usize] > dividend {
                break;
            }
            i += 1;
            i;
        }
        let mut answer: u64 = 0;
        let mut accumulator: u64 = 0;
        i = i - 1;
        while i >= 0 {
            if accumulator.wrapping_add(doublings[i as usize]) <= dividend {
                accumulator = (accumulator).wrapping_add(doublings[i as usize]) as u64;
                answer = (answer).wrapping_add(powers[i as usize]) as u64;
            }
            i -= 1;
            i;
        }
        if !remainder.is_null() {
            *remainder = dividend.wrapping_sub(accumulator);
        }
        return answer;
    }
}

#[no_mangle]
pub extern "C" fn go(mut a: u64, mut b: u64) {
    let mut x: u64 = 0;
    let mut y: u64 = 0;
    x = egyptian_division(a, b, &mut y);
    print!("{} / {} = {} remainder {}\n", a, b, x, y);
    unsafe {
        if a == b.wrapping_mul(x).wrapping_add(y) {
        } else {
            __assert_fail(
                b"a == b * x + y\0" as *const u8 as *const i8,
                b"main.c\0" as *const u8 as *const i8,
                43,
                (*::core::mem::transmute::<&[u8; 28], &[i8; 28]>(b"void go(uint64_t, uint64_t)\0"))
                    .as_ptr(),
            );
        }
        'c_1638: {
            if a == b.wrapping_mul(x).wrapping_add(y) {
            } else {
                __assert_fail(
                    b"a == b * x + y\0" as *const u8 as *const i8,
                    b"main.c\0" as *const u8 as *const i8,
                    43,
                    (*::core::mem::transmute::<&[u8; 28], &[i8; 28]>(
                        b"void go(uint64_t, uint64_t)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
    }
}

fn main_0() -> i32 {
    go(580, 32);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
