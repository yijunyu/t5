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
    fn log(_: f64) -> f64;
    fn sqrt(_: f64) -> f64;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strlen(_: *const i8) -> u64;
}
#[no_mangle]
pub extern "C" fn es_check(mut sieve: *mut u32, mut n: u64) -> i32 {
    unsafe {
        if n != 2 && n & 1 == 0 || n < 2 {
            return 0;
        } else {
            return (*sieve.offset((n >> 6i32) as isize) & (1i32 << (n >> 1 & 31u64)) as u32 == 0)
                as i32;
        };
    }
}

#[no_mangle]
pub extern "C" fn es_sieve(nth: u64, mut es_size: *mut u64) -> *mut u32 {
    unsafe {
        *es_size = (nth as f64 * log(nth as f64)
            + nth as f64 * (log(log(nth as f64)) - 0.9385f32 as f64)
            + 1 as f64) as u64;
        let mut sieve: *mut u32 = calloc(
            (*es_size >> 6i32).wrapping_add(1),
            ::core::mem::size_of::<u32>() as u64,
        ) as *mut u32;
        let mut i: u64 = 3;
        while (i as f64) < sqrt(*es_size as f64) + 1 as f64 {
            if *sieve.offset((i >> 6i32) as isize) & (1i32 << (i >> 1 & 31)) as u32 == 0 {
                let mut j: u64 = i.wrapping_mul(i);
                while j < *es_size {
                    let ref mut fresh0 = *sieve.offset((j >> 6i32) as isize);
                    *fresh0 |= (1i32 << (j >> 1 & 31)) as u32;
                    j = (j).wrapping_add(i << 1) as u64;
                }
            }
            i = (i).wrapping_add(2) as u64;
        }
        return sieve;
    }
}

#[no_mangle]
pub extern "C" fn mpz_number_of_digits(op: i32) -> u64 {
    unsafe {
        let mut opstr: *mut i8 = 0 as *mut i8;
        let oplen: u64 = strlen(opstr);
        free(opstr as *mut libc::c_void);
        return oplen;
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut sieve_size: u64 = 0;
        let mut sieve: *mut u32 = es_sieve(100000, &mut sieve_size);
        let mut prime_count: u64 = 0;
        let mut print: i32 = 1;
        let mut unused: f64 = 0.;
        let mut i: u64 = 2;
        while i < sieve_size && prime_count <= 100000 {
            if print != 0 {
                print = 0;
            }
            if es_check(sieve, i) != 0 {
                prime_count = prime_count.wrapping_add(1);
                prime_count;
                print = 1;
            }
            i = i.wrapping_add(1);
            i;
        }
        free(sieve as *mut libc::c_void);
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
