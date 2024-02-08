#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bit_array_tag {
    pub size: uint32_t,
    pub array: *mut uint32_t,
}
pub type bit_array = bit_array_tag;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sieve_tag {
    pub limit: uint32_t,
    pub not_prime: bit_array,
}
pub type sieve = sieve_tag;
#[no_mangle]
pub unsafe extern "C" fn bit_array_create(
    mut b: *mut bit_array,
    mut size: uint32_t,
) -> bool {
    let mut array: *mut uint32_t = calloc(
        size
            .wrapping_add(31 as libc::c_int as libc::c_uint)
            .wrapping_div(32 as libc::c_int as libc::c_uint) as libc::c_ulong,
        ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
    ) as *mut uint32_t;
    if array.is_null() {
        return 0 as libc::c_int != 0;
    }
    (*b).size = size;
    (*b).array = array;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn bit_array_destroy(mut b: *mut bit_array) {
    free((*b).array as *mut libc::c_void);
    (*b).array = 0 as *mut uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn bit_array_set(
    mut b: *mut bit_array,
    mut index: uint32_t,
    mut value: bool,
) {
    if index < (*b).size {} else {
        __assert_fail(
            b"index < b->size\0" as *const u8 as *const libc::c_char,
            b"main.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 49],
                &[libc::c_char; 49],
            >(b"void bit_array_set(bit_array *, uint32_t, _Bool)\0"))
                .as_ptr(),
        );
    }
    'c_1632: {
        if index < (*b).size {} else {
            __assert_fail(
                b"index < b->size\0" as *const u8 as *const libc::c_char,
                b"main.c\0" as *const u8 as *const libc::c_char,
                27 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 49],
                    &[libc::c_char; 49],
                >(b"void bit_array_set(bit_array *, uint32_t, _Bool)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut p: *mut uint32_t = &mut *((*b).array)
        .offset((index >> 5 as libc::c_int) as isize) as *mut uint32_t;
    let mut bit: uint32_t = ((1 as libc::c_int)
        << (index & 31 as libc::c_int as libc::c_uint)) as uint32_t;
    if value {
        *p |= bit;
    } else {
        *p &= !bit;
    };
}
#[no_mangle]
pub unsafe extern "C" fn bit_array_get(
    mut b: *const bit_array,
    mut index: uint32_t,
) -> bool {
    if index < (*b).size {} else {
        __assert_fail(
            b"index < b->size\0" as *const u8 as *const libc::c_char,
            b"main.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 49],
                &[libc::c_char; 49],
            >(b"_Bool bit_array_get(const bit_array *, uint32_t)\0"))
                .as_ptr(),
        );
    }
    'c_1716: {
        if index < (*b).size {} else {
            __assert_fail(
                b"index < b->size\0" as *const u8 as *const libc::c_char,
                b"main.c\0" as *const u8 as *const libc::c_char,
                37 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 49],
                    &[libc::c_char; 49],
                >(b"_Bool bit_array_get(const bit_array *, uint32_t)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut bit: uint32_t = ((1 as libc::c_int)
        << (index & 31 as libc::c_int as libc::c_uint)) as uint32_t;
    return *((*b).array).offset((index >> 5 as libc::c_int) as isize) & bit
        != 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn sieve_create(mut s: *mut sieve, mut limit: uint32_t) -> bool {
    if !bit_array_create(
        &mut (*s).not_prime,
        limit.wrapping_add(1 as libc::c_int as libc::c_uint),
    ) {
        return 0 as libc::c_int != 0;
    }
    bit_array_set(
        &mut (*s).not_prime,
        0 as libc::c_int as uint32_t,
        1 as libc::c_int != 0,
    );
    bit_array_set(
        &mut (*s).not_prime,
        1 as libc::c_int as uint32_t,
        1 as libc::c_int != 0,
    );
    let mut p: uint32_t = 2 as libc::c_int as uint32_t;
    while p.wrapping_mul(p) <= limit {
        if bit_array_get(&mut (*s).not_prime, p) as libc::c_int == 0 as libc::c_int {
            let mut q: uint32_t = p.wrapping_mul(p);
            while q <= limit {
                bit_array_set(&mut (*s).not_prime, q, 1 as libc::c_int != 0);
                q = (q as libc::c_uint).wrapping_add(p) as uint32_t as uint32_t;
            }
        }
        p = p.wrapping_add(1);
        p;
    }
    (*s).limit = limit;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn sieve_destroy(mut s: *mut sieve) {
    bit_array_destroy(&mut (*s).not_prime);
}
#[no_mangle]
pub unsafe extern "C" fn is_prime(mut s: *const sieve, mut n: uint32_t) -> bool {
    if n <= (*s).limit {} else {
        __assert_fail(
            b"n <= s->limit\0" as *const u8 as *const libc::c_char,
            b"main.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"_Bool is_prime(const sieve *, uint32_t)\0"))
                .as_ptr(),
        );
    }
    'c_1919: {
        if n <= (*s).limit {} else {
            __assert_fail(
                b"n <= s->limit\0" as *const u8 as *const libc::c_char,
                b"main.c\0" as *const u8 as *const libc::c_char,
                67 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"_Bool is_prime(const sieve *, uint32_t)\0"))
                    .as_ptr(),
            );
        }
    };
    return bit_array_get(&(*s).not_prime, n) as libc::c_int == 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn find_prime_partition(
    mut s: *const sieve,
    mut number: uint32_t,
    mut count: uint32_t,
    mut min_prime: uint32_t,
    mut p: *mut uint32_t,
) -> bool {
    if count == 1 as libc::c_int as libc::c_uint {
        if number >= min_prime && is_prime(s, number) as libc::c_int != 0 {
            *p = number;
            return 1 as libc::c_int != 0;
        }
        return 0 as libc::c_int != 0;
    }
    let mut prime: uint32_t = min_prime;
    while prime < number {
        if is_prime(s, prime) {
            if find_prime_partition(
                s,
                number.wrapping_sub(prime),
                count.wrapping_sub(1 as libc::c_int as libc::c_uint),
                prime.wrapping_add(1 as libc::c_int as libc::c_uint),
                p.offset(1 as libc::c_int as isize),
            ) {
                *p = prime;
                return 1 as libc::c_int != 0;
            }
        }
        prime = prime.wrapping_add(1);
        prime;
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn print_prime_partition(
    mut s: *const sieve,
    mut number: uint32_t,
    mut count: uint32_t,
) {
    if count > 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"count > 0\0" as *const u8 as *const libc::c_char,
            b"main.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"void print_prime_partition(const sieve *, uint32_t, uint32_t)\0"))
                .as_ptr(),
        );
    }
    'c_2193: {
        if count > 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"count > 0\0" as *const u8 as *const libc::c_char,
                b"main.c\0" as *const u8 as *const libc::c_char,
                93 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 62],
                    &[libc::c_char; 62],
                >(b"void print_prime_partition(const sieve *, uint32_t, uint32_t)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut primes: *mut uint32_t = malloc(
        (count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
    ) as *mut uint32_t;
    if primes.is_null() {
        fprintf(stderr, b"Out of memory\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    if !find_prime_partition(s, number, count, 2 as libc::c_int as uint32_t, primes) {
        printf(
            b"%u cannot be partitioned into %u primes.\n\0" as *const u8
                as *const libc::c_char,
            number,
            count,
        );
    } else {
        printf(
            b"%u = %u\0" as *const u8 as *const libc::c_char,
            number,
            *primes.offset(0 as libc::c_int as isize),
        );
        let mut i: uint32_t = 1 as libc::c_int as uint32_t;
        while i < count {
            printf(
                b" + %u\0" as *const u8 as *const libc::c_char,
                *primes.offset(i as isize),
            );
            i = i.wrapping_add(1);
            i;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    free(primes as *mut libc::c_void);
}
unsafe fn main_0() -> libc::c_int {
    let limit: uint32_t = 100000 as libc::c_int as uint32_t;
    let mut s: sieve = {
        let mut init = sieve_tag {
            limit: 0 as libc::c_int as uint32_t,
            not_prime: bit_array {
                size: 0,
                array: 0 as *mut uint32_t,
            },
        };
        init
    };
    if !sieve_create(&mut s, limit) {
        fprintf(stderr, b"Out of memory\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    print_prime_partition(
        &mut s,
        99809 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
    );
    print_prime_partition(
        &mut s,
        18 as libc::c_int as uint32_t,
        2 as libc::c_int as uint32_t,
    );
    print_prime_partition(
        &mut s,
        19 as libc::c_int as uint32_t,
        3 as libc::c_int as uint32_t,
    );
    print_prime_partition(
        &mut s,
        20 as libc::c_int as uint32_t,
        4 as libc::c_int as uint32_t,
    );
    print_prime_partition(
        &mut s,
        2017 as libc::c_int as uint32_t,
        24 as libc::c_int as uint32_t,
    );
    print_prime_partition(
        &mut s,
        22699 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
    );
    print_prime_partition(
        &mut s,
        22699 as libc::c_int as uint32_t,
        2 as libc::c_int as uint32_t,
    );
    print_prime_partition(
        &mut s,
        22699 as libc::c_int as uint32_t,
        3 as libc::c_int as uint32_t,
    );
    print_prime_partition(
        &mut s,
        22699 as libc::c_int as uint32_t,
        4 as libc::c_int as uint32_t,
    );
    print_prime_partition(
        &mut s,
        40355 as libc::c_int as uint32_t,
        3 as libc::c_int as uint32_t,
    );
    sieve_destroy(&mut s);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
