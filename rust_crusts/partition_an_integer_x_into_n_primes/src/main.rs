#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, label_break_value)]
use c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: i64,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: i64,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: u64,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bit_array_tag {
    pub size: u32,
    pub array: *mut u32,
}
pub type bit_array = bit_array_tag;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sieve_tag {
    pub limit: u32,
    pub not_prime: bit_array,
}
pub type sieve = sieve_tag;
#[no_mangle]
pub extern "C" fn bit_array_create(mut b: *mut bit_array, mut size: u32) -> bool {
    unsafe {
        let mut array: *mut u32 = calloc(
            size.wrapping_add(31).wrapping_div(32) as u64,
            ::core::mem::size_of::<u32>() as u64,
        ) as *mut u32;
        if array.is_null() {
            return 0 != 0;
        };
        (*b).size = size;
        (*b).array = array;
        return 1 != 0;
    }
}

#[no_mangle]
pub extern "C" fn bit_array_destroy(mut b: *mut bit_array) {
    unsafe {
        free((*b).array as *mut libc::c_void);
        (*b).array = 0 as *mut u32;
    }
}

#[no_mangle]
pub extern "C" fn bit_array_set(mut b: *mut bit_array, mut index: u32, mut value: bool) {
    unsafe {
        if index < (*b).size {
        } else {
            __assert_fail(
                b"index < b->size\0" as *const u8 as *const i8,
                b"main.c\0" as *const u8 as *const i8,
                27,
                (*::core::mem::transmute::<&[u8; 49], &[i8; 49]>(
                    b"void bit_array_set(bit_array *, uint32_t, _Bool)\0",
                ))
                .as_ptr(),
            );
        }
        'c_1632: {
            if index < (*b).size {
            } else {
                __assert_fail(
                    b"index < b->size\0" as *const u8 as *const i8,
                    b"main.c\0" as *const u8 as *const i8,
                    27,
                    (*::core::mem::transmute::<&[u8; 49], &[i8; 49]>(
                        b"void bit_array_set(bit_array *, uint32_t, _Bool)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        let mut p: *mut u32 = &mut *((*b).array).offset((index >> 5i32) as isize) as *mut u32;
        let mut bit: u32 = (1i32 << (index & 31)) as u32;
        if value {
            *p |= bit;
        } else {
            *p &= !bit;
        };
    }
}

#[no_mangle]
pub extern "C" fn bit_array_get(mut b: *const bit_array, mut index: u32) -> bool {
    unsafe {
        if index < (*b).size {
        } else {
            __assert_fail(
                b"index < b->size\0" as *const u8 as *const i8,
                b"main.c\0" as *const u8 as *const i8,
                37,
                (*::core::mem::transmute::<&[u8; 49], &[i8; 49]>(
                    b"_Bool bit_array_get(const bit_array *, uint32_t)\0",
                ))
                .as_ptr(),
            );
        }
        'c_1716: {
            if index < (*b).size {
            } else {
                __assert_fail(
                    b"index < b->size\0" as *const u8 as *const i8,
                    b"main.c\0" as *const u8 as *const i8,
                    37,
                    (*::core::mem::transmute::<&[u8; 49], &[i8; 49]>(
                        b"_Bool bit_array_get(const bit_array *, uint32_t)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        let mut bit: u32 = (1i32 << (index & 31)) as u32;
        return *((*b).array).offset((index >> 5i32) as isize) & bit != 0;
    }
}

#[no_mangle]
pub extern "C" fn sieve_create(mut s: *mut sieve, mut limit: u32) -> bool {
    unsafe {
        if !bit_array_create(&mut (*s).not_prime, limit.wrapping_add(1)) {
            return 0 != 0;
        }
        bit_array_set(&mut (*s).not_prime, 0, 1 != 0);
        bit_array_set(&mut (*s).not_prime, 1, 1 != 0);
        let mut p: u32 = 2;
        while p.wrapping_mul(p) <= limit {
            if bit_array_get(&mut (*s).not_prime, p) as i32 == 0 {
                let mut q: u32 = p.wrapping_mul(p);
                while q <= limit {
                    bit_array_set(&mut (*s).not_prime, q, 1 != 0);
                    q = (q).wrapping_add(p) as u32;
                }
            }
            p = p.wrapping_add(1);
            p;
        }
        (*s).limit = limit;
        return 1 != 0;
    }
}

#[no_mangle]
pub extern "C" fn sieve_destroy(mut s: *mut sieve) {
    unsafe {
        bit_array_destroy(&mut (*s).not_prime);
    }
}

#[no_mangle]
pub extern "C" fn is_prime(mut s: *const sieve, mut n: u32) -> bool {
    unsafe {
        if n <= (*s).limit {
        } else {
            __assert_fail(
                b"n <= s->limit\0" as *const u8 as *const i8,
                b"main.c\0" as *const u8 as *const i8,
                67,
                (*::core::mem::transmute::<&[u8; 40], &[i8; 40]>(
                    b"_Bool is_prime(const sieve *, uint32_t)\0",
                ))
                .as_ptr(),
            );
        }
        'c_1919: {
            if n <= (*s).limit {
            } else {
                __assert_fail(
                    b"n <= s->limit\0" as *const u8 as *const i8,
                    b"main.c\0" as *const u8 as *const i8,
                    67,
                    (*::core::mem::transmute::<&[u8; 40], &[i8; 40]>(
                        b"_Bool is_prime(const sieve *, uint32_t)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        return bit_array_get(&(*s).not_prime, n) as i32 == 0;
    }
}

#[no_mangle]
pub extern "C" fn find_prime_partition(
    mut s: *const sieve,
    mut number: u32,
    mut count: u32,
    mut min_prime: u32,
    mut p: *mut u32,
) -> bool {
    unsafe {
        if count == 1 {
            if number >= min_prime && is_prime(s, number) as i32 != 0 {
                *p = number;
                return 1 != 0;
            }
            return 0 != 0;
        }
        let mut prime: u32 = min_prime;
        while prime < number {
            if is_prime(s, prime) {
                if find_prime_partition(
                    s,
                    number.wrapping_sub(prime),
                    count.wrapping_sub(1),
                    prime.wrapping_add(1),
                    p.offset(1 as isize),
                ) {
                    *p = prime;
                    return 1 != 0;
                }
            }
            prime = prime.wrapping_add(1);
            prime;
        }
        return 0 != 0;
    }
}

#[no_mangle]
pub extern "C" fn print_prime_partition(mut s: *const sieve, mut number: u32, mut count: u32) {
    unsafe {
        if count > 0 {
        } else {
            __assert_fail(
                b"count > 0\0" as *const u8 as *const i8,
                b"main.c\0" as *const u8 as *const i8,
                93,
                (*::core::mem::transmute::<&[u8; 62], &[i8; 62]>(
                    b"void print_prime_partition(const sieve *, uint32_t, uint32_t)\0",
                ))
                .as_ptr(),
            );
        }
        'c_2193: {
            if count > 0 {
            } else {
                __assert_fail(
                    b"count > 0\0" as *const u8 as *const i8,
                    b"main.c\0" as *const u8 as *const i8,
                    93,
                    (*::core::mem::transmute::<&[u8; 62], &[i8; 62]>(
                        b"void print_prime_partition(const sieve *, uint32_t, uint32_t)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        let mut primes: *mut u32 =
            malloc((count as u64).wrapping_mul(::core::mem::size_of::<u32>() as u64)) as *mut u32;
        if primes.is_null() {
            fprintf(stderr, b"Out of memory\n\0" as *const u8 as *const i8);
            return;
        }
        if !find_prime_partition(s, number, count, 2, primes) {
            print!("{} cannot be partitioned into {} primes.\n", number, count);
        } else {
            print!("{} = {}", number, *primes.offset(0 as isize));
            let mut i: u32 = 1;
            while i < count {
                print!(" + {}", *primes.offset(i as isize));
                i = i.wrapping_add(1);
                i;
            }
            print!("\n");
        }
        free(primes as *mut libc::c_void);
    }
}

fn main_0() -> i32 {
    let limit: u32 = 100000;
    let mut s: sieve = {
        let mut init = sieve_tag {
            limit: 0,
            not_prime: bit_array {
                size: 0,
                array: 0 as *mut u32,
            },
        };
        init
    };
    unsafe {
        if !sieve_create(&mut s, limit) {
            fprintf(stderr, b"Out of memory\n\0" as *const u8 as *const i8);
            return 1;
        }
    }
    print_prime_partition(&mut s, 99809, 1);
    print_prime_partition(&mut s, 18, 2);
    print_prime_partition(&mut s, 19, 3);
    print_prime_partition(&mut s, 20, 4);
    print_prime_partition(&mut s, 2017, 24);
    print_prime_partition(&mut s, 22699, 1);
    print_prime_partition(&mut s, 22699, 2);
    print_prime_partition(&mut s, 22699, 3);
    print_prime_partition(&mut s, 22699, 4);
    print_prime_partition(&mut s, 40355, 3);
    sieve_destroy(&mut s);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
