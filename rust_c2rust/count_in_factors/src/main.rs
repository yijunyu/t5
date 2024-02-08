#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type ULONG = libc::c_ulonglong;
#[no_mangle]
pub unsafe extern "C" fn get_prime(mut idx: libc::c_int) -> ULONG {
    static mut n_primes: libc::c_long = 0 as libc::c_int as libc::c_long;
    static mut alloc: libc::c_long = 0 as libc::c_int as libc::c_long;
    static mut primes: *mut ULONG = 0 as *const ULONG as *mut ULONG;
    let mut last: ULONG = 0;
    let mut p: ULONG = 0;
    let mut i: libc::c_int = 0;
    if idx as libc::c_long >= n_primes {
        if n_primes >= alloc {
            alloc += 16 as libc::c_int as libc::c_long;
            primes = realloc(
                primes as *mut libc::c_void,
                (::core::mem::size_of::<ULONG>() as libc::c_ulong)
                    .wrapping_mul(alloc as libc::c_ulong),
            ) as *mut ULONG;
        }
        if n_primes == 0 {
            *primes.offset(0 as libc::c_int as isize) = 2 as libc::c_int as ULONG;
            *primes.offset(1 as libc::c_int as isize) = 3 as libc::c_int as ULONG;
            n_primes = 2 as libc::c_int as libc::c_long;
        }
        last = *primes.offset((n_primes - 1 as libc::c_int as libc::c_long) as isize);
        while idx as libc::c_long >= n_primes {
            last = (last as libc::c_ulonglong)
                .wrapping_add(2 as libc::c_int as libc::c_ulonglong) as ULONG as ULONG;
            i = 0 as libc::c_int;
            while (i as libc::c_long) < n_primes {
                p = *primes.offset(i as isize);
                if p.wrapping_mul(p) > last {
                    let fresh0 = n_primes;
                    n_primes = n_primes + 1;
                    *primes.offset(fresh0 as isize) = last;
                    break;
                } else {
                    if last.wrapping_rem(p) == 0 as libc::c_int as libc::c_ulonglong {
                        break;
                    }
                    i += 1;
                    i;
                }
            }
        }
    }
    return *primes.offset(idx as isize);
}
unsafe fn main_0() -> libc::c_int {
    let mut n: ULONG = 0;
    let mut x: ULONG = 0;
    let mut p: ULONG = 0;
    let mut i: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    x = 1 as libc::c_int as ULONG;
    while x < 1000 as libc::c_int as libc::c_ulonglong {
        n = x;
        printf(b"%lld = \0" as *const u8 as *const libc::c_char, n);
        i = 0 as libc::c_int;
        first = 1 as libc::c_int;
        loop {
            p = get_prime(i);
            while n.wrapping_rem(p) == 0 as libc::c_int as libc::c_ulonglong {
                n = (n as libc::c_ulonglong).wrapping_div(p) as ULONG as ULONG;
                if first == 0 {
                    printf(b" x \0" as *const u8 as *const libc::c_char);
                }
                first = 0 as libc::c_int;
                printf(b"%lld\0" as *const u8 as *const libc::c_char, p);
            }
            if n <= p.wrapping_mul(p) {
                break;
            }
            i += 1;
            i;
        }
        if first != 0 {
            printf(b"%lld\n\0" as *const u8 as *const libc::c_char, n);
        } else if n > 1 as libc::c_int as libc::c_ulonglong {
            printf(b" x %lld\n\0" as *const u8 as *const libc::c_char, n);
        } else {
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        x = x.wrapping_add(1);
        x;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
