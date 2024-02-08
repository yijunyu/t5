#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type llong_t = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PrimeArray {
    pub ptr: *mut llong_t,
    pub size: size_t,
    pub capacity: size_t,
}
#[no_mangle]
pub unsafe extern "C" fn allocate() -> PrimeArray {
    let mut primes: PrimeArray = PrimeArray {
        ptr: 0 as *mut llong_t,
        size: 0,
        capacity: 0,
    };
    primes.size = 0 as libc::c_int as size_t;
    primes.capacity = 10 as libc::c_int as size_t;
    primes
        .ptr = malloc(
        (primes.capacity)
            .wrapping_mul(::core::mem::size_of::<llong_t>() as libc::c_ulong),
    ) as *mut llong_t;
    return primes;
}
#[no_mangle]
pub unsafe extern "C" fn deallocate(mut primes: *mut PrimeArray) {
    free((*primes).ptr as *mut libc::c_void);
    (*primes).ptr = 0 as *mut llong_t;
}
#[no_mangle]
pub unsafe extern "C" fn push_back(mut primes: *mut PrimeArray, mut p: llong_t) {
    if (*primes).size >= (*primes).capacity {
        let mut new_capacity: size_t = (3 as libc::c_int as libc::c_ulong)
            .wrapping_mul((*primes).capacity)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        let mut temp: *mut llong_t = realloc(
            (*primes).ptr as *mut libc::c_void,
            new_capacity.wrapping_mul(::core::mem::size_of::<llong_t>() as libc::c_ulong),
        ) as *mut llong_t;
        if temp.is_null() {
            fprintf(
                stderr,
                b"Failed to reallocate the prime array.\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        } else {
            (*primes).ptr = temp;
            (*primes).capacity = new_capacity;
        }
    }
    let fresh0 = (*primes).size;
    (*primes).size = ((*primes).size).wrapping_add(1);
    *((*primes).ptr).offset(fresh0 as isize) = p;
}
unsafe fn main_0() -> libc::c_int {
    let cutOff: libc::c_int = 200 as libc::c_int;
    let bigUn: libc::c_int = 100000 as libc::c_int;
    let chunks: libc::c_int = 50 as libc::c_int;
    let little: libc::c_int = bigUn / chunks;
    let mut primes: PrimeArray = allocate();
    let mut c: libc::c_int = 0 as libc::c_int;
    let mut showEach: bool = 1 as libc::c_int != 0;
    let mut u: llong_t = 0 as libc::c_int as llong_t;
    let mut v: llong_t = 1 as libc::c_int as llong_t;
    let mut i: llong_t = 0;
    push_back(&mut primes, 3 as libc::c_int as llong_t);
    push_back(&mut primes, 5 as libc::c_int as llong_t);
    printf(
        b"The first %d cuban primes:\n\0" as *const u8 as *const libc::c_char,
        cutOff,
    );
    i = 1 as libc::c_int as llong_t;
    while i < 9223372036854775807 as libc::c_longlong {
        let mut found: bool = 0 as libc::c_int != 0;
        u += 6 as libc::c_int as libc::c_longlong;
        v += u;
        let mut mx: llong_t = ceil(sqrt(v as libc::c_double)) as llong_t;
        let mut j: llong_t = 0;
        j = 0 as libc::c_int as llong_t;
        while (j as libc::c_ulonglong) < primes.size as libc::c_ulonglong {
            if *(primes.ptr).offset(j as isize) > mx {
                break;
            }
            if v % *(primes.ptr).offset(j as isize)
                == 0 as libc::c_int as libc::c_longlong
            {
                found = 1 as libc::c_int != 0;
                break;
            } else {
                j += 1;
                j;
            }
        }
        if !found {
            c += 1 as libc::c_int;
            if showEach {
                let mut z: llong_t = 0;
                z = *(primes.ptr)
                    .offset(
                        (primes.size).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) + 2 as libc::c_int as libc::c_longlong;
                while z <= v - 2 as libc::c_int as libc::c_longlong {
                    let mut fnd: bool = 0 as libc::c_int != 0;
                    j = 0 as libc::c_int as llong_t;
                    while (j as libc::c_ulonglong) < primes.size as libc::c_ulonglong {
                        if *(primes.ptr).offset(j as isize) > mx {
                            break;
                        }
                        if z % *(primes.ptr).offset(j as isize)
                            == 0 as libc::c_int as libc::c_longlong
                        {
                            fnd = 1 as libc::c_int != 0;
                            break;
                        } else {
                            j += 1;
                            j;
                        }
                    }
                    if !fnd {
                        push_back(&mut primes, z);
                    }
                    z += 2 as libc::c_int as libc::c_longlong;
                }
                push_back(&mut primes, v);
                printf(b"%11lld\0" as *const u8 as *const libc::c_char, v);
                if c % 10 as libc::c_int == 0 as libc::c_int {
                    printf(b"\n\0" as *const u8 as *const libc::c_char);
                }
                if c == cutOff {
                    showEach = 0 as libc::c_int != 0;
                    printf(
                        b"\nProgress to the %dth cuban prime: \0" as *const u8
                            as *const libc::c_char,
                        bigUn,
                    );
                }
            }
            if c % little == 0 as libc::c_int {
                printf(b".\0" as *const u8 as *const libc::c_char);
                if c == bigUn {
                    break;
                }
            }
        }
        i += 1;
        i;
    }
    printf(
        b"\nThe %dth cuban prime is %lld\n\0" as *const u8 as *const libc::c_char,
        c,
        v,
    );
    deallocate(&mut primes);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
