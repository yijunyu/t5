#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type bool_0 = libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn sieve(
    mut limit: libc::c_int,
    mut primes: *mut libc::c_int,
    mut count: *mut libc::c_int,
) {
    let mut c: *mut bool_0 = calloc(
        (limit + 1 as libc::c_int) as libc::c_ulong,
        ::core::mem::size_of::<bool_0>() as libc::c_ulong,
    ) as *mut bool_0;
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 3 as libc::c_int;
    let mut p2: libc::c_int = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    p2 = p * p;
    while p2 <= limit {
        i = p2;
        while i <= limit {
            *c.offset(i as isize) = 1 as libc::c_int;
            i += 2 as libc::c_int * p;
        }
        loop {
            p += 2 as libc::c_int;
            if !(*c.offset(p as isize) != 0) {
                break;
            }
        }
        p2 = p * p;
    }
    i = 3 as libc::c_int;
    while i <= limit {
        if *c.offset(i as isize) == 0 {
            let fresh0 = n;
            n = n + 1;
            *primes.offset(fresh0 as isize) = i;
        }
        i += 2 as libc::c_int;
    }
    *count = n;
    free(c as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn findPeriod(mut n: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 1 as libc::c_int;
    let mut rr: libc::c_int = 0;
    let mut period: libc::c_int = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= n + 1 as libc::c_int {
        r = 10 as libc::c_int * r % n;
        i += 1;
        i;
    }
    rr = r;
    loop {
        r = 10 as libc::c_int * r % n;
        period += 1;
        period;
        if !(r != rr) {
            break;
        }
    }
    return period;
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut prime: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut index: libc::c_int = 0 as libc::c_int;
    let mut primeCount: libc::c_int = 0;
    let mut longCount: libc::c_int = 0 as libc::c_int;
    let mut numberCount: libc::c_int = 0;
    let mut primes: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut longPrimes: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut totals: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut numbers: [libc::c_int; 8] = [
        500 as libc::c_int,
        1000 as libc::c_int,
        2000 as libc::c_int,
        4000 as libc::c_int,
        8000 as libc::c_int,
        16000 as libc::c_int,
        32000 as libc::c_int,
        64000 as libc::c_int,
    ];
    primes = calloc(
        6500 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    numberCount = (::core::mem::size_of::<[libc::c_int; 8]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_int;
    totals = calloc(
        numberCount as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    sieve(64000 as libc::c_int, primes, &mut primeCount);
    longPrimes = calloc(
        primeCount as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < primeCount {
        prime = *primes.offset(i as isize);
        if findPeriod(prime) == prime - 1 as libc::c_int {
            let fresh1 = longCount;
            longCount = longCount + 1;
            *longPrimes.offset(fresh1 as isize) = prime;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < longCount {
        if *longPrimes.offset(i as isize) > numbers[index as usize] {
            let fresh2 = index;
            index = index + 1;
            *totals.offset(fresh2 as isize) = count;
        }
        i += 1;
        i;
        count += 1;
        count;
    }
    *totals.offset((numberCount - 1 as libc::c_int) as isize) = count;
    printf(
        b"The long primes up to %d are:\n\0" as *const u8 as *const libc::c_char,
        numbers[0 as libc::c_int as usize],
    );
    printf(b"[\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < *totals.offset(0 as libc::c_int as isize) {
        printf(
            b"%d \0" as *const u8 as *const libc::c_char,
            *longPrimes.offset(i as isize),
        );
        i += 1;
        i;
    }
    printf(b"\x08]\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"\nThe number of long primes up to:\n\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        printf(
            b"  %5d is %d\n\0" as *const u8 as *const libc::c_char,
            numbers[i as usize],
            *totals.offset(i as isize),
        );
        i += 1;
        i;
    }
    free(totals as *mut libc::c_void);
    free(longPrimes as *mut libc::c_void);
    free(primes as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
