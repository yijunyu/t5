#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn putchar(__c: libc::c_int) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct i128_0 {
    pub x: [uint64_t; 2],
}
#[no_mangle]
pub unsafe extern "C" fn show(mut v: i128_0) {
    let mut x: [uint32_t; 4] = [
        v.x[0 as libc::c_int as usize] as uint32_t,
        (v.x[0 as libc::c_int as usize] >> 32 as libc::c_int) as uint32_t,
        v.x[1 as libc::c_int as usize] as uint32_t,
        (v.x[1 as libc::c_int as usize] >> 32 as libc::c_int) as uint32_t,
    ];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 4 as libc::c_int;
    let mut buf: [libc::c_char; 100] = [0; 100];
    loop {
        let mut c: uint64_t = 0 as libc::c_int as uint64_t;
        i = len;
        loop {
            let fresh0 = i;
            i = i - 1;
            if !(fresh0 != 0) {
                break;
            }
            c = (c << 32 as libc::c_int).wrapping_add(x[i as usize] as libc::c_ulong);
            x[i
                as usize] = c.wrapping_div(10 as libc::c_int as libc::c_ulong)
                as uint32_t;
            c = (c as libc::c_ulong).wrapping_rem(10 as libc::c_int as libc::c_ulong)
                as uint64_t as uint64_t;
        }
        let fresh1 = j;
        j = j + 1;
        buf[fresh1
            as usize] = c.wrapping_add('0' as i32 as libc::c_ulong) as libc::c_char;
        len = 4 as libc::c_int;
        while x[(len - 1 as libc::c_int) as usize] == 0 {
            len -= 1;
            len;
        }
        if !(len != 0) {
            break;
        }
    }
    loop {
        let fresh2 = j;
        j = j - 1;
        if !(fresh2 != 0) {
            break;
        }
        putchar(buf[j as usize] as libc::c_int);
    }
    putchar('\n' as i32);
}
#[no_mangle]
pub unsafe extern "C" fn count(
    mut sum: libc::c_int,
    mut coins: *mut libc::c_int,
) -> i128_0 {
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    n = 0 as libc::c_int;
    while *coins.offset(n as isize) != 0 {
        n += 1;
        n;
    }
    let mut v: *mut *mut i128_0 = malloc(
        (::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut *mut i128_0;
    let mut idx: *mut libc::c_int = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        *idx.offset(i as isize) = *coins.offset(i as isize);
        let ref mut fresh3 = *v.offset(i as isize);
        *fresh3 = calloc(
            ::core::mem::size_of::<i128_0>() as libc::c_ulong,
            *coins.offset(i as isize) as libc::c_ulong,
        ) as *mut i128_0;
        i += 1;
        i;
    }
    *(*v.offset(0 as libc::c_int as isize))
        .offset(
            (*coins.offset(0 as libc::c_int as isize) - 1 as libc::c_int) as isize,
        ) = {
        let mut init = i128_0 {
            x: [1 as libc::c_int as uint64_t, 0 as libc::c_int as uint64_t],
        };
        init
    };
    k = 0 as libc::c_int;
    while k <= sum {
        i = 0 as libc::c_int;
        while i < n {
            let ref mut fresh4 = *idx.offset(i as isize);
            let fresh5 = *fresh4;
            *fresh4 = *fresh4 - 1;
            if fresh5 == 0 {
                *idx.offset(i as isize) = *coins.offset(i as isize) - 1 as libc::c_int;
            }
            i += 1;
            i;
        }
        let mut c: i128_0 = *(*v.offset(0 as libc::c_int as isize))
            .offset(*idx.offset(0 as libc::c_int as isize) as isize);
        i = 1 as libc::c_int;
        while i < n {
            let mut p: *mut i128_0 = (*v.offset(i as isize))
                .offset(*idx.offset(i as isize) as isize);
            (*p)
                .x[0 as libc::c_int
                as usize] = ((*p).x[0 as libc::c_int as usize] as libc::c_ulong)
                .wrapping_add(c.x[0 as libc::c_int as usize]) as uint64_t as uint64_t;
            (*p)
                .x[1 as libc::c_int
                as usize] = ((*p).x[1 as libc::c_int as usize] as libc::c_ulong)
                .wrapping_add(c.x[1 as libc::c_int as usize]) as uint64_t as uint64_t;
            if (*p).x[0 as libc::c_int as usize] < c.x[0 as libc::c_int as usize] {
                (*p)
                    .x[1 as libc::c_int
                    as usize] = ((*p).x[1 as libc::c_int as usize]).wrapping_add(1);
                (*p).x[1 as libc::c_int as usize];
            }
            c = *p;
            i += 1;
            i;
        }
        k += 1;
        k;
    }
    let mut r: i128_0 = *(*v.offset((n - 1 as libc::c_int) as isize))
        .offset(*idx.offset((n - 1 as libc::c_int) as isize) as isize);
    i = 0 as libc::c_int;
    while i < n {
        free(*v.offset(i as isize) as *mut libc::c_void);
        i += 1;
        i;
    }
    free(v as *mut libc::c_void);
    free(idx as *mut libc::c_void);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn count2(
    mut sum: libc::c_int,
    mut coins: *mut libc::c_int,
) -> libc::c_int {
    if *coins == 0 || sum < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if sum == 0 {
        return 1 as libc::c_int;
    }
    return count2(sum - *coins, coins)
        + count2(sum, coins.offset(1 as libc::c_int as isize));
}
unsafe fn main_0() -> libc::c_int {
    let mut us_coins: [libc::c_int; 7] = [
        100 as libc::c_int,
        50 as libc::c_int,
        25 as libc::c_int,
        10 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    ];
    let mut eu_coins: [libc::c_int; 9] = [
        200 as libc::c_int,
        100 as libc::c_int,
        50 as libc::c_int,
        20 as libc::c_int,
        10 as libc::c_int,
        5 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    ];
    show(
        count(
            100 as libc::c_int,
            us_coins.as_mut_ptr().offset(2 as libc::c_int as isize),
        ),
    );
    show(count(1000 as libc::c_int, us_coins.as_mut_ptr()));
    show(count(1000 as libc::c_int * 100 as libc::c_int, us_coins.as_mut_ptr()));
    show(count(10000 as libc::c_int * 100 as libc::c_int, us_coins.as_mut_ptr()));
    show(count(100000 as libc::c_int * 100 as libc::c_int, us_coins.as_mut_ptr()));
    putchar('\n' as i32);
    show(count(1 as libc::c_int * 100 as libc::c_int, eu_coins.as_mut_ptr()));
    show(count(1000 as libc::c_int * 100 as libc::c_int, eu_coins.as_mut_ptr()));
    show(count(10000 as libc::c_int * 100 as libc::c_int, eu_coins.as_mut_ptr()));
    show(count(100000 as libc::c_int * 100 as libc::c_int, eu_coins.as_mut_ptr()));
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
