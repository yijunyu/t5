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
extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn sqrt(_: f64) -> f64;
    fn printf(_: *const i8, _: ...) -> i32;
}
#[no_mangle]
pub extern "C" fn eratosthenes(mut n: i32, mut c: *mut i32) -> *mut i8 {
    unsafe {
        let mut sieve_0: *mut i8 = 0 as *mut i8;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut m: i32 = 0;
        if n < 2 {
            return 0 as *mut i8;
        }
        *c = n - 1;
        m = sqrt(n as f64) as i32;
        sieve_0 = calloc((n + 1i32) as u64, ::core::mem::size_of::<i8>() as u64) as *mut i8;
        *sieve_0.offset(0 as isize) = 1;
        *sieve_0.offset(1 as isize) = 1;
        i = 2;
        while i <= m {
            if *sieve_0.offset(i as isize) == 0 {
                j = i * i;
                while j <= n {
                    if *sieve_0.offset(j as isize) == 0 {
                        *sieve_0.offset(j as isize) = 1;
                        *c -= 1;
                        *c;
                    }
                    j += i;
                }
            }
            i += 1;
            i;
        }
        return sieve_0;
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut array: *mut i32 = 0 as *mut i32;
        let mut n: i32 = 10;
        array = malloc(((n + 1i32) as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64))
            as *mut i32;
        sieve(array, n);
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn sieve(mut a: *mut i32, mut n: i32) {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        i = 2;
        while i <= n {
            *a.offset(i as isize) = 1;
            i += 1;
            i;
        }
        i = 2;
        while i <= n {
            print!("\ni:{}", i);
            if *a.offset(i as isize) == 1 {
                j = i;
                while i * j <= n {
                    print!("\nj:{}", j);
                    printf(
                        b"\nBefore a[%d*%d]: %d\0" as *const u8 as *const i8,
                        i,
                        j,
                        *a.offset((i * j) as isize),
                    );
                    *a.offset((i * j) as isize) = 0;
                    printf(
                        b"\nAfter a[%d*%d]: %d\0" as *const u8 as *const i8,
                        i,
                        j,
                        *a.offset((i * j) as isize),
                    );
                    j += 1;
                    j;
                }
            }
            i += 1;
            i;
        }
        print!("\nPrimes numbers from 1 to {} are : ", n);
        i = 2;
        while i <= n {
            if *a.offset(i as isize) == 1 {
                print!("{}, ", i);
            }
            i += 1;
            i;
        }
        print!("\n\n");
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
