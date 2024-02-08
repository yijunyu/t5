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
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[no_mangle]
pub extern "C" fn sieve(mut limit: i32, mut primes: *mut i32, mut count: *mut i32) {
    unsafe {
        let mut c: *mut i32 =
            calloc((limit + 1i32) as u64, ::core::mem::size_of::<i32>() as u64) as *mut i32;
        let mut i: i32 = 0;
        let mut p: i32 = 3;
        let mut p2: i32 = 0;
        let mut n: i32 = 0;
        p2 = p * p;
        while p2 <= limit {
            i = p2;
            while i <= limit {
                *c.offset(i as isize) = 1;
                i += 2 * p;
            }
            loop {
                p += 2;
                if !(*c.offset(p as isize) != 0) {
                    break;
                }
            }
            p2 = p * p;
        }
        i = 3;
        while i <= limit {
            if *c.offset(i as isize) == 0 {
                let fresh0 = n;
                n = n + 1;
                *primes.offset(fresh0 as isize) = i;
            }
            i += 2;
        }
        *count = n;
        free(c as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn findPeriod(mut n: i32) -> i32 {
    let mut i: i32 = 0;
    let mut r: i32 = 1;
    let mut rr: i32 = 0;
    let mut period: i32 = 0;
    i = 1;
    while i <= n + 1 {
        r = 10 * r % n;
        i += 1;
        i;
    }
    rr = r;
    loop {
        r = 10 * r % n;
        period += 1;
        period;
        if !(r != rr) {
            break;
        }
    }
    return period;
}

fn main_0() -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut prime: i32 = 0;
        let mut count: i32 = 0;
        let mut index: i32 = 0;
        let mut primeCount: i32 = 0;
        let mut longCount: i32 = 0;
        let mut numberCount: i32 = 0;
        let mut primes: *mut i32 = 0 as *mut i32;
        let mut longPrimes: *mut i32 = 0 as *mut i32;
        let mut totals: *mut i32 = 0 as *mut i32;
        let mut numbers: [i32; 8] = [500, 1000, 2000, 4000, 8000, 16000, 32000, 64000];
        primes = calloc(6500, ::core::mem::size_of::<i32>() as u64) as *mut i32;
        numberCount = (::core::mem::size_of::<[i32; 8]>() as u64)
            .wrapping_div(::core::mem::size_of::<i32>() as u64) as i32;
        totals = calloc(numberCount as u64, ::core::mem::size_of::<i32>() as u64) as *mut i32;
        sieve(64000, primes, &mut primeCount);
        longPrimes = calloc(primeCount as u64, ::core::mem::size_of::<i32>() as u64) as *mut i32;
        i = 0;
        while i < primeCount {
            prime = *primes.offset(i as isize);
            if findPeriod(prime) == prime - 1 {
                let fresh1 = longCount;
                longCount = longCount + 1;
                *longPrimes.offset(fresh1 as isize) = prime;
            }
            i += 1;
            i;
        }
        i = 0;
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
        *totals.offset((numberCount - 1i32) as isize) = count;
        print!("The long primes up to {} are:\n", numbers[0 as usize]);
        print!("[");
        i = 0;
        while i < *totals.offset(0 as isize) {
            print!("{} ", *longPrimes.offset(i as isize));
            i += 1;
            i;
        }
        print!("\x08]\n");
        print!("\nThe number of long primes up to:\n");
        i = 0;
        while i < 8 {
            print!(
                "  {:5} is {}\n",
                numbers[i as usize],
                *totals.offset(i as isize)
            );
            i += 1;
            i;
        }
        free(totals as *mut libc::c_void);
        free(longPrimes as *mut libc::c_void);
        free(primes as *mut libc::c_void);
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
