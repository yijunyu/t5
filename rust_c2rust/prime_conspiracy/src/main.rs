#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(label_break_value)]
use ::c2rust_out::*;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type byte = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Transition {
    pub a: byte,
    pub b: byte,
    pub c: libc::c_uint,
}
#[no_mangle]
pub static mut transitions: [Transition; 100] = [Transition { a: 0, b: 0, c: 0 }; 100];
#[no_mangle]
pub unsafe extern "C" fn init() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 10 as libc::c_int {
            let mut idx: libc::c_int = i * 10 as libc::c_int + j;
            transitions[idx as usize].a = i as byte;
            transitions[idx as usize].b = j as byte;
            transitions[idx as usize].c = 0 as libc::c_int as libc::c_uint;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn record(mut prev: libc::c_int, mut curr: libc::c_int) {
    let mut pd: byte = (prev % 10 as libc::c_int) as byte;
    let mut cd: byte = (curr % 10 as libc::c_int) as byte;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        let mut z: libc::c_int = 0 as libc::c_int;
        if transitions[i as usize].a as libc::c_int == pd as libc::c_int {
            let mut t: libc::c_int = 0 as libc::c_int;
            if transitions[i as usize].b as libc::c_int == cd as libc::c_int {
                transitions[i as usize].c = (transitions[i as usize].c).wrapping_add(1);
                transitions[i as usize].c;
                break;
            }
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn printTransitions(
    mut limit: libc::c_int,
    mut last_prime: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    printf(
        b"%d primes, last prime considered: %d\n\0" as *const u8 as *const libc::c_char,
        limit,
        last_prime,
    );
    i = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        if transitions[i as usize].c > 0 as libc::c_int as libc::c_uint {
            printf(
                b"%d->%d  count: %5d  frequency: %.2f\n\0" as *const u8
                    as *const libc::c_char,
                transitions[i as usize].a as libc::c_int,
                transitions[i as usize].b as libc::c_int,
                transitions[i as usize].c,
                100.0f64 * transitions[i as usize].c as libc::c_double
                    / limit as libc::c_double,
            );
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn isPrime(mut n: libc::c_int) -> bool {
    let mut s: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut a1: libc::c_int = 0;
    let mut a2: libc::c_int = 0;
    if n % 2 as libc::c_int == 0 as libc::c_int {
        return n == 2 as libc::c_int;
    }
    if n % 3 as libc::c_int == 0 as libc::c_int {
        return n == 3 as libc::c_int;
    }
    if n % 5 as libc::c_int == 0 as libc::c_int {
        return n == 5 as libc::c_int;
    }
    if n % 7 as libc::c_int == 0 as libc::c_int {
        return n == 7 as libc::c_int;
    }
    if n % 11 as libc::c_int == 0 as libc::c_int {
        return n == 11 as libc::c_int;
    }
    if n % 13 as libc::c_int == 0 as libc::c_int {
        return n == 13 as libc::c_int;
    }
    if n % 17 as libc::c_int == 0 as libc::c_int {
        return n == 17 as libc::c_int;
    }
    if n % 19 as libc::c_int == 0 as libc::c_int {
        return n == 19 as libc::c_int;
    }
    t = 23 as libc::c_int;
    a1 = 96 as libc::c_int;
    a2 = 216 as libc::c_int;
    s = t * t;
    while s <= n {
        if n % t == 0 as libc::c_int {
            return 0 as libc::c_int != 0;
        }
        s += a1;
        t += 2 as libc::c_int;
        a1 += 24 as libc::c_int;
        if t * t == s {} else {
            __assert_fail(
                b"t * t == s\0" as *const u8 as *const libc::c_char,
                b"main.c\0" as *const u8 as *const libc::c_char,
                77 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 19],
                    &[libc::c_char; 19],
                >(b"_Bool isPrime(int)\0"))
                    .as_ptr(),
            );
        }
        'c_923: {
            if t * t == s {} else {
                __assert_fail(
                    b"t * t == s\0" as *const u8 as *const libc::c_char,
                    b"main.c\0" as *const u8 as *const libc::c_char,
                    77 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 19],
                        &[libc::c_char; 19],
                    >(b"_Bool isPrime(int)\0"))
                        .as_ptr(),
                );
            }
        };
        if s <= n {
            if n % t == 0 as libc::c_int {
                return 0 as libc::c_int != 0;
            }
            s += a2;
            t += 4 as libc::c_int;
            a2 += 48 as libc::c_int;
            if t * t == s {} else {
                __assert_fail(
                    b"t * t == s\0" as *const u8 as *const libc::c_char,
                    b"main.c\0" as *const u8 as *const libc::c_char,
                    86 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 19],
                        &[libc::c_char; 19],
                    >(b"_Bool isPrime(int)\0"))
                        .as_ptr(),
                );
            }
            'c_846: {
                if t * t == s {} else {
                    __assert_fail(
                        b"t * t == s\0" as *const u8 as *const libc::c_char,
                        b"main.c\0" as *const u8 as *const libc::c_char,
                        86 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 19],
                            &[libc::c_char; 19],
                        >(b"_Bool isPrime(int)\0"))
                            .as_ptr(),
                    );
                }
            };
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe fn main_0() -> libc::c_int {
    let mut last_prime: libc::c_int = 3 as libc::c_int;
    let mut n: libc::c_int = 5 as libc::c_int;
    let mut count: libc::c_int = 2 as libc::c_int;
    init();
    record(2 as libc::c_int, 3 as libc::c_int);
    while count < 1000000 as libc::c_int {
        if isPrime(n) {
            record(last_prime, n);
            last_prime = n;
            count += 1;
            count;
        }
        n += 2 as libc::c_int;
        if count < 1000000 as libc::c_int {
            if isPrime(n) {
                record(last_prime, n);
                last_prime = n;
                count += 1;
                count;
            }
            n += 4 as libc::c_int;
        }
    }
    printTransitions(1000000 as libc::c_int, last_prime);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
