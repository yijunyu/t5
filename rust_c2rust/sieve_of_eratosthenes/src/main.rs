#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn eratosthenes(
    mut n: libc::c_int,
    mut c: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut sieve_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    if n < 2 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    *c = n - 1 as libc::c_int;
    m = sqrt(n as libc::c_double) as libc::c_int;
    sieve_0 = calloc(
        (n + 1 as libc::c_int) as libc::c_ulong,
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
    ) as *mut libc::c_char;
    *sieve_0.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_char;
    *sieve_0.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_char;
    i = 2 as libc::c_int;
    while i <= m {
        if *sieve_0.offset(i as isize) == 0 {
            j = i * i;
            while j <= n {
                if *sieve_0.offset(j as isize) == 0 {
                    *sieve_0.offset(j as isize) = 1 as libc::c_int as libc::c_char;
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
unsafe fn main_0() -> libc::c_int {
    let mut array: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut n: libc::c_int = 10 as libc::c_int;
    array = malloc(
        ((n + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    sieve(array, n);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sieve(mut a: *mut libc::c_int, mut n: libc::c_int) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    i = 2 as libc::c_int;
    while i <= n {
        *a.offset(i as isize) = 1 as libc::c_int;
        i += 1;
        i;
    }
    i = 2 as libc::c_int;
    while i <= n {
        printf(b"\ni:%d\0" as *const u8 as *const libc::c_char, i);
        if *a.offset(i as isize) == 1 as libc::c_int {
            j = i;
            while i * j <= n {
                printf(b"\nj:%d\0" as *const u8 as *const libc::c_char, j);
                printf(
                    b"\nBefore a[%d*%d]: %d\0" as *const u8 as *const libc::c_char,
                    i,
                    j,
                    *a.offset((i * j) as isize),
                );
                *a.offset((i * j) as isize) = 0 as libc::c_int;
                printf(
                    b"\nAfter a[%d*%d]: %d\0" as *const u8 as *const libc::c_char,
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
    printf(
        b"\nPrimes numbers from 1 to %d are : \0" as *const u8 as *const libc::c_char,
        n,
    );
    i = 2 as libc::c_int;
    while i <= n {
        if *a.offset(i as isize) == 1 as libc::c_int {
            printf(b"%d, \0" as *const u8 as *const libc::c_char, i);
        }
        i += 1;
        i;
    }
    printf(b"\n\n\0" as *const u8 as *const libc::c_char);
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
