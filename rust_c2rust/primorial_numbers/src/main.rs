#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn es_check(
    mut sieve: *mut uint32_t,
    mut n: uint64_t,
) -> libc::c_int {
    if n != 2 as libc::c_int as libc::c_ulong
        && n & 1 as libc::c_int as libc::c_ulong == 0
        || n < 2 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int
    } else {
        return (*sieve.offset((n >> 6 as libc::c_int) as isize)
            & ((1 as libc::c_int)
                << (n >> 1 as libc::c_int & 31 as libc::c_int as libc::c_ulong))
                as libc::c_uint == 0) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn es_sieve(
    nth: uint64_t,
    mut es_size: *mut uint64_t,
) -> *mut uint32_t {
    *es_size = (nth as libc::c_double * log(nth as libc::c_double)
        + nth as libc::c_double
            * (log(log(nth as libc::c_double)) - 0.9385f32 as libc::c_double)
        + 1 as libc::c_int as libc::c_double) as uint64_t;
    let mut sieve: *mut uint32_t = calloc(
        (*es_size >> 6 as libc::c_int).wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
    ) as *mut uint32_t;
    let mut i: uint64_t = 3 as libc::c_int as uint64_t;
    while (i as libc::c_double)
        < sqrt(*es_size as libc::c_double) + 1 as libc::c_int as libc::c_double
    {
        if *sieve.offset((i >> 6 as libc::c_int) as isize)
            & ((1 as libc::c_int)
                << (i >> 1 as libc::c_int & 31 as libc::c_int as libc::c_ulong))
                as libc::c_uint == 0
        {
            let mut j: uint64_t = i.wrapping_mul(i);
            while j < *es_size {
                let ref mut fresh0 = *sieve.offset((j >> 6 as libc::c_int) as isize);
                *fresh0
                    |= ((1 as libc::c_int)
                        << (j >> 1 as libc::c_int & 31 as libc::c_int as libc::c_ulong))
                        as libc::c_uint;
                j = (j as libc::c_ulong).wrapping_add(i << 1 as libc::c_int) as uint64_t
                    as uint64_t;
            }
        }
        i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
            as uint64_t as uint64_t;
    }
    return sieve;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_number_of_digits(op: libc::c_int) -> size_t {
    let mut opstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let oplen: size_t = strlen(opstr);
    free(opstr as *mut libc::c_void);
    return oplen;
}
unsafe fn main_0() -> libc::c_int {
    let mut sieve_size: uint64_t = 0;
    let mut sieve: *mut uint32_t = es_sieve(
        100000 as libc::c_int as uint64_t,
        &mut sieve_size,
    );
    let mut prime_count: uint64_t = 0 as libc::c_int as uint64_t;
    let mut print: libc::c_int = 1 as libc::c_int;
    let mut unused: libc::c_double = 0.;
    let mut i: uint64_t = 2 as libc::c_int as uint64_t;
    while i < sieve_size && prime_count <= 100000 as libc::c_int as libc::c_ulong {
        if print != 0 {
            print = 0 as libc::c_int;
        }
        if es_check(sieve, i) != 0 {
            prime_count = prime_count.wrapping_add(1);
            prime_count;
            print = 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    free(sieve as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
