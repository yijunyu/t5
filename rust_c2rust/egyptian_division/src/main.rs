#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(label_break_value)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
#[no_mangle]
pub unsafe extern "C" fn egyptian_division(
    mut dividend: uint64_t,
    mut divisor: uint64_t,
    mut remainder: *mut uint64_t,
) -> uint64_t {
    static mut powers: [uint64_t; 64] = [0; 64];
    static mut doublings: [uint64_t; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        powers[i as usize] = ((1 as libc::c_int) << i) as uint64_t;
        doublings[i as usize] = divisor << i;
        if doublings[i as usize] > dividend {
            break;
        }
        i += 1;
        i;
    }
    let mut answer: uint64_t = 0 as libc::c_int as uint64_t;
    let mut accumulator: uint64_t = 0 as libc::c_int as uint64_t;
    i = i - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if accumulator.wrapping_add(doublings[i as usize]) <= dividend {
            accumulator = (accumulator as libc::c_ulong)
                .wrapping_add(doublings[i as usize]) as uint64_t as uint64_t;
            answer = (answer as libc::c_ulong).wrapping_add(powers[i as usize])
                as uint64_t as uint64_t;
        }
        i -= 1;
        i;
    }
    if !remainder.is_null() {
        *remainder = dividend.wrapping_sub(accumulator);
    }
    return answer;
}
#[no_mangle]
pub unsafe extern "C" fn go(mut a: uint64_t, mut b: uint64_t) {
    let mut x: uint64_t = 0;
    let mut y: uint64_t = 0;
    x = egyptian_division(a, b, &mut y);
    printf(
        b"%llu / %llu = %llu remainder %llu\n\0" as *const u8 as *const libc::c_char,
        a,
        b,
        x,
        y,
    );
    if a == b.wrapping_mul(x).wrapping_add(y) {} else {
        __assert_fail(
            b"a == b * x + y\0" as *const u8 as *const libc::c_char,
            b"main.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void go(uint64_t, uint64_t)\0"))
                .as_ptr(),
        );
    }
    'c_1638: {
        if a == b.wrapping_mul(x).wrapping_add(y) {} else {
            __assert_fail(
                b"a == b * x + y\0" as *const u8 as *const libc::c_char,
                b"main.c\0" as *const u8 as *const libc::c_char,
                43 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void go(uint64_t, uint64_t)\0"))
                    .as_ptr(),
            );
        }
    };
}
unsafe fn main_0() -> libc::c_int {
    go(580 as libc::c_int as uint64_t, 32 as libc::c_int as uint64_t);
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
