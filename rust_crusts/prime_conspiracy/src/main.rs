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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Transition {
    pub a: u8,
    pub b: u8,
    pub c: u32,
}
#[no_mangle]
pub static mut transitions: [Transition; 100] = [Transition { a: 0, b: 0, c: 0 }; 100];
#[no_mangle]
pub extern "C" fn init() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0;
    unsafe {
        while i < 10 {
            j = 0;
            while j < 10 {
                let mut idx: i32 = i * 10 + j;
                transitions[idx as usize].a = i as u8;
                transitions[idx as usize].b = j as u8;
                transitions[idx as usize].c = 0;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn record(mut prev: i32, mut curr: i32) {
    let mut pd: u8 = (prev % 10i32) as u8;
    let mut cd: u8 = (curr % 10i32) as u8;
    let mut i: i32 = 0;
    i = 0;
    unsafe {
        while i < 100 {
            let mut z: i32 = 0;
            if transitions[i as usize].a as i32 == pd as i32 {
                let mut t: i32 = 0;
                if transitions[i as usize].b as i32 == cd as i32 {
                    transitions[i as usize].c = (transitions[i as usize].c).wrapping_add(1);
                    transitions[i as usize].c;
                    break;
                }
            }
            i += 1;
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn printTransitions(mut limit: i32, mut last_prime: i32) {
    let mut i: i32 = 0;
    print!("{} primes, last prime considered: {}\n", limit, last_prime);
    i = 0;
    unsafe {
        while i < 100 {
            if transitions[i as usize].c > 0 {
                print!(
                    "{}->{}  count: {:5}  frequency: {:.2}\n",
                    transitions[i as usize].a as i32,
                    transitions[i as usize].b as i32,
                    transitions[i as usize].c,
                    100.0f64 * transitions[i as usize].c as f64 / limit as f64
                );
            }
            i += 1;
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn isPrime(mut n: i32) -> bool {
    let mut s: i32 = 0;
    let mut t: i32 = 0;
    let mut a1: i32 = 0;
    let mut a2: i32 = 0;
    if n % 2 == 0 {
        return n == 2;
    }
    if n % 3 == 0 {
        return n == 3;
    }
    if n % 5 == 0 {
        return n == 5;
    }
    if n % 7 == 0 {
        return n == 7;
    }
    if n % 11 == 0 {
        return n == 11;
    }
    if n % 13 == 0 {
        return n == 13;
    }
    if n % 17 == 0 {
        return n == 17;
    }
    if n % 19 == 0 {
        return n == 19;
    }
    t = 23;
    a1 = 96;
    a2 = 216;
    s = t * t;
    unsafe {
        while s <= n {
            if n % t == 0 {
                return 0 != 0;
            }
            s += a1;
            t += 2;
            a1 += 24;
            if t * t == s {
            } else {
                __assert_fail(
                    b"t * t == s\0" as *const u8 as *const i8,
                    b"main.c\0" as *const u8 as *const i8,
                    77,
                    (*::core::mem::transmute::<&[u8; 19], &[i8; 19]>(b"_Bool isPrime(int)\0"))
                        .as_ptr(),
                );
            }
            'c_923: {
                if t * t == s {
                } else {
                    __assert_fail(
                        b"t * t == s\0" as *const u8 as *const i8,
                        b"main.c\0" as *const u8 as *const i8,
                        77,
                        (*::core::mem::transmute::<&[u8; 19], &[i8; 19]>(b"_Bool isPrime(int)\0"))
                            .as_ptr(),
                    );
                }
            };
            if s <= n {
                if n % t == 0 {
                    return 0 != 0;
                }
                s += a2;
                t += 4;
                a2 += 48;
                if t * t == s {
                } else {
                    __assert_fail(
                        b"t * t == s\0" as *const u8 as *const i8,
                        b"main.c\0" as *const u8 as *const i8,
                        86,
                        (*::core::mem::transmute::<&[u8; 19], &[i8; 19]>(b"_Bool isPrime(int)\0"))
                            .as_ptr(),
                    );
                }
                'c_846: {
                    if t * t == s {
                    } else {
                        __assert_fail(
                            b"t * t == s\0" as *const u8 as *const i8,
                            b"main.c\0" as *const u8 as *const i8,
                            86,
                            (*::core::mem::transmute::<&[u8; 19], &[i8; 19]>(
                                b"_Bool isPrime(int)\0",
                            ))
                            .as_ptr(),
                        );
                    }
                };
            }
        }
    }
    return 1 != 0;
}

fn main_0() -> i32 {
    let mut last_prime: i32 = 3;
    let mut n: i32 = 5;
    let mut count: i32 = 2;
    init();
    record(2, 3);
    while count < 1000000 {
        if isPrime(n) {
            record(last_prime, n);
            last_prime = n;
            count += 1;
            count;
        }
        n += 2;
        if count < 1000000 {
            if isPrime(n) {
                record(last_prime, n);
                last_prime = n;
                count += 1;
                count;
            }
            n += 4;
        }
    }
    printTransitions(1000000, last_prime);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
