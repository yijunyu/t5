#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
use c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: i32) -> !;
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
#[no_mangle]
pub extern "C" fn xmalloc(mut n: u64) -> *mut libc::c_void {
    unsafe {
        let mut ptr: *mut libc::c_void = malloc(n);
        if ptr.is_null() {
            fprintf(stderr, b"Out of memory\n\0" as *const u8 as *const i8);
            exit(1);
        }
        return ptr;
    }
}

#[no_mangle]
pub extern "C" fn xrealloc(mut p: *mut libc::c_void, mut n: u64) -> *mut libc::c_void {
    unsafe {
        let mut ptr: *mut libc::c_void = realloc(p, n);
        if ptr.is_null() {
            fprintf(stderr, b"Out of memory\n\0" as *const u8 as *const i8);
            exit(1);
        }
        return ptr;
    }
}

#[no_mangle]
pub extern "C" fn is_prime(mut n: u32) -> bool {
    if n == 2 {
        return 1 != 0;
    }
    if n < 2 || n.wrapping_rem(2) == 0 {
        return 0 != 0;
    }
    let mut p: u32 = 3;
    while p.wrapping_mul(p) <= n {
        if n.wrapping_rem(p) == 0 {
            return 0 != 0;
        }
        p = (p).wrapping_add(2) as u32;
    }
    return 1 != 0;
}

#[no_mangle]
pub extern "C" fn find_primes(mut from: u32, mut to: u32, mut primes: *mut *mut u32) -> u32 {
    unsafe {
        let mut count: u32 = 0;
        let mut buffer_length: u32 = 16;
        let mut buffer: *mut u32 =
            xmalloc((::core::mem::size_of::<u32>() as u64).wrapping_mul(buffer_length as u64))
                as *mut u32;
        let mut p: u32 = from;
        while p <= to {
            if is_prime(p) {
                if count >= buffer_length {
                    let mut new_length: u32 = buffer_length.wrapping_mul(2);
                    if new_length < count.wrapping_add(1) {
                        new_length = count.wrapping_add(1);
                    }
                    buffer = xrealloc(
                        buffer as *mut libc::c_void,
                        (::core::mem::size_of::<u32>() as u64).wrapping_mul(new_length as u64),
                    ) as *mut u32;
                    buffer_length = new_length;
                }
                let fresh0 = count;
                count = count.wrapping_add(1);
                *buffer.offset(fresh0 as isize) = p;
            }
            p = p.wrapping_add(1);
            p;
        }
        *primes = buffer;
        return count;
    }
}

#[no_mangle]
pub extern "C" fn free_numbers(mut numbers: *mut i32, mut count: u64) {}

#[no_mangle]
pub extern "C" fn print_nsmooth_numbers(mut n: u32, mut begin: u32, mut count: u32) {
    let mut num: u32 = begin.wrapping_add(count);
    print!("{}: ", n);
    let mut i: u32 = 1;
    while i < count {
        print!(", ");
        i = i.wrapping_add(1);
        i;
    }
    print!("\n");
}

fn main_0() -> i32 {
    print!("First 25 n-smooth numbers for n = 2 -> 29:\n");
    let mut n: u32 = 2;
    while n <= 29 {
        if is_prime(n) {
            print_nsmooth_numbers(n, 0, 25);
        }
        n = n.wrapping_add(1);
        n;
    }
    print!("\n3 n-smooth numbers starting from 3000th for n = 3 -> 29:\n");
    let mut n_0: u32 = 3;
    while n_0 <= 29 {
        if is_prime(n_0) {
            print_nsmooth_numbers(n_0, 2999, 3);
        }
        n_0 = n_0.wrapping_add(1);
        n_0;
    }
    print!("\n20 n-smooth numbers starting from 30,000th for n = 503 -> 521:\n");
    let mut n_1: u32 = 503;
    while n_1 <= 521 {
        if is_prime(n_1) {
            print_nsmooth_numbers(n_1, 29999, 20);
        }
        n_1 = n_1.wrapping_add(1);
        n_1;
    }
    return 0;
}

pub fn main() {
    unsafe {
        ::std::process::exit(main_0() as i32);
    }
}
