#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
#[no_mangle]
pub unsafe extern "C" fn digit_sum(mut n: uint64_t, mut sum: uint64_t) -> uint64_t {
    sum = sum.wrapping_add(1);
    sum;
    while n > 0 as libc::c_int as libc::c_ulong
        && n.wrapping_rem(10 as libc::c_int as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
    {
        sum = (sum as libc::c_ulong).wrapping_sub(9 as libc::c_int as libc::c_ulong)
            as uint64_t as uint64_t;
        n = (n as libc::c_ulong).wrapping_div(10 as libc::c_int as libc::c_ulong)
            as uint64_t as uint64_t;
    }
    return sum;
}
#[no_mangle]
pub unsafe extern "C" fn divisible(mut n: uint64_t, mut d: uint64_t) -> bool {
    if d & 1 as libc::c_int as libc::c_ulong == 0 as libc::c_int as libc::c_ulong
        && n & 1 as libc::c_int as libc::c_ulong == 1 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int != 0;
    }
    return n.wrapping_rem(d) == 0 as libc::c_int as libc::c_ulong;
}
unsafe fn main_0() -> libc::c_int {
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    let mut previous: uint64_t = 1 as libc::c_int as uint64_t;
    let mut gap: uint64_t = 0 as libc::c_int as uint64_t;
    let mut sum: uint64_t = 0 as libc::c_int as uint64_t;
    let mut niven_index: libc::c_int = 0 as libc::c_int;
    let mut gap_index: libc::c_int = 1 as libc::c_int;
    printf(
        b"Gap index  Gap    Niven index    Niven number\n\0" as *const u8
            as *const libc::c_char,
    );
    let mut niven: uint64_t = 1 as libc::c_int as uint64_t;
    while gap_index <= 32 as libc::c_int {
        sum = digit_sum(niven, sum);
        if divisible(niven, sum) {
            if niven > previous.wrapping_add(gap) {
                gap = niven.wrapping_sub(previous);
                let fresh0 = gap_index;
                gap_index = gap_index + 1;
                printf(
                    b"%'9d %'4llu %'14d %'15llu\n\0" as *const u8 as *const libc::c_char,
                    fresh0,
                    gap,
                    niven_index,
                    previous,
                );
            }
            previous = niven;
            niven_index += 1;
            niven_index;
        }
        niven = niven.wrapping_add(1);
        niven;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
