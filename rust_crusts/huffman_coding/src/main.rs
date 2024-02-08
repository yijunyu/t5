#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.offset(str_size as isize) != 0 {
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}

use c2rust_out::*;
extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abs(_: i32) -> i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct huffcode {
    pub nbits: i32,
    pub code: i32,
}
pub type huffcode_t = huffcode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct huffheap {
    pub h: *mut i32,
    pub n: i32,
    pub s: i32,
    pub cs: i32,
    pub f: *mut i64,
}
pub type heap_t = huffheap;
extern "C" fn _heap_create(mut s: i32, mut f: *mut i64) -> *mut heap_t {
    unsafe {
        let mut h: *mut heap_t = 0 as *mut heap_t;
        h = malloc(::core::mem::size_of::<heap_t>() as u64) as *mut heap_t;
        (*h).h = malloc((::core::mem::size_of::<i32>() as u64).wrapping_mul(s as u64)) as *mut i32;
        (*h).cs = s;
        (*h).s = (*h).cs;
        (*h).n = 0;
        (*h).f = f;
        return h;
    }
}

extern "C" fn _heap_destroy(mut heap: *mut heap_t) {
    unsafe {
        free((*heap).h as *mut libc::c_void);
        free(heap as *mut libc::c_void);
    }
}

extern "C" fn _heap_sort(mut heap: *mut heap_t) {
    unsafe {
        let mut i: i32 = 1;
        let mut j: i32 = 2;
        let mut a: *mut i32 = (*heap).h;
        while i < (*heap).n {
            if *((*heap).f).offset(*a.offset((i - 1i32) as isize) as isize)
                >= *((*heap).f).offset(*a.offset(i as isize) as isize)
            {
                i = j;
                j += 1;
                j;
            } else {
                let mut t_: i32 = 0;
                t_ = *a.offset((i - 1i32) as isize);
                *a.offset((i - 1i32) as isize) = *a.offset(i as isize);
                *a.offset(i as isize) = t_;
                i -= 1;
                i;
                i = if i == 0 {
                    let fresh0 = j;
                    j = j + 1;
                    fresh0
                } else {
                    i
                };
            }
        }
    }
}

extern "C" fn _heap_add(mut heap: *mut heap_t, mut c: i32) {
    unsafe {
        if (*heap).n + 1 > (*heap).s {
            (*heap).h = realloc(
                (*heap).h as *mut libc::c_void,
                ((*heap).s + (*heap).cs) as u64,
            ) as *mut i32;
            (*heap).s += (*heap).cs;
        };
        *((*heap).h).offset((*heap).n as isize) = c;
        (*heap).n += 1;
        (*heap).n;
        _heap_sort(heap);
    }
}

extern "C" fn _heap_remove(mut heap: *mut heap_t) -> i32 {
    unsafe {
        if (*heap).n > 0 {
            (*heap).n -= 1;
            (*heap).n;
            return *((*heap).h).offset((*heap).n as isize);
        }
        return -1;
    }
}

#[no_mangle]
pub extern "C" fn create_huffman_codes(mut freqs: *mut i64) -> *mut *mut huffcode_t {
    unsafe {
        let mut codes: *mut *mut huffcode_t = 0 as *mut *mut huffcode_t;
        let mut heap: *mut heap_t = 0 as *mut heap_t;
        let mut efreqs: [i64; 512] = [0; 512];
        let mut preds: [i32; 512] = [0; 512];
        let mut i: i32 = 0;
        let mut extf: i32 = 256;
        let mut r1: i32 = 0;
        let mut r2: i32 = 0;
        memcpy(
            efreqs.as_mut_ptr() as *mut libc::c_void,
            freqs as *const libc::c_void,
            (::core::mem::size_of::<i64>() as u64).wrapping_mul(256),
        );
        memset(
            &mut *efreqs.as_mut_ptr().offset(256 as isize) as *mut i64 as *mut libc::c_void,
            0,
            (::core::mem::size_of::<i64>() as u64).wrapping_mul(256),
        );
        heap = _heap_create(256 * 2, efreqs.as_mut_ptr());
        if heap.is_null() {
            return 0 as *mut *mut huffcode_t;
        }
        i = 0;
        while i < 256 {
            if efreqs[i as usize] > 0 {
                _heap_add(heap, i);
            }
            i += 1;
            i;
        }
        while (*heap).n > 1 {
            r1 = _heap_remove(heap);
            r2 = _heap_remove(heap);
            efreqs[extf as usize] = efreqs[r1 as usize] + efreqs[r2 as usize];
            _heap_add(heap, extf);
            preds[r1 as usize] = extf;
            preds[r2 as usize] = -extf;
            extf += 1;
            extf;
        }
        r1 = _heap_remove(heap);
        preds[r1 as usize] = r1;
        _heap_destroy(heap);
        codes = malloc((::core::mem::size_of::<*mut huffcode_t>() as u64).wrapping_mul(256))
            as *mut *mut huffcode_t;
        let mut bc: i32 = 0;
        let mut bn: i32 = 0;
        let mut ix: i32 = 0;
        i = 0;
        while i < 256 {
            bc = 0;
            bn = 0;
            if efreqs[i as usize] == 0 {
                let ref mut fresh1 = *codes.offset(i as isize);
                *fresh1 = 0 as *mut huffcode_t;
            } else {
                ix = i;
                while abs(preds[ix as usize]) != ix {
                    bc |= (if preds[ix as usize] >= 0 { 1 } else { 0 }) << bn;
                    ix = abs(preds[ix as usize]);
                    bn += 1;
                    bn;
                }
                let ref mut fresh2 = *codes.offset(i as isize);
                *fresh2 = malloc(::core::mem::size_of::<huffcode_t>() as u64) as *mut huffcode_t;
                (**codes.offset(i as isize)).nbits = bn;
                (**codes.offset(i as isize)).code = bc;
            }
            i += 1;
            i;
        }
        return codes;
    }
}

#[no_mangle]
pub extern "C" fn free_huffman_codes(mut c: *mut *mut huffcode_t) {
    unsafe {
        let mut i: i32 = 0;
        i = 0;
        while i < 256 {
            free(*c.offset(i as isize) as *mut libc::c_void);
            i += 1;
            i;
        }
        free(c as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn inttobits(mut c: i32, mut n: i32, mut s: *mut i8) {
    unsafe {
        *s.offset(n as isize) = 0;
        while n > 0 {
            *s.offset((n - 1i32) as isize) = (c % 2 + '0' as i32) as i8;
            c >>= 1;
            n -= 1;
            n;
        }
    }
}

#[no_mangle]
pub static mut test: *const i8 =
    b"this is an example for huffman encoding\0" as *const u8 as *const i8;
fn main_0() -> i32 {
    unsafe {
        let mut r: *mut *mut huffcode_t = 0 as *mut *mut huffcode_t;
        let mut i: i32 = 0;
        let mut strbit: [i8; 100] = [0; 100];
        let mut p: *const i8 = 0 as *const i8;
        let mut freqs: [i64; 256] = [0; 256];
        memset(
            freqs.as_mut_ptr() as *mut libc::c_void,
            0,
            ::core::mem::size_of::<[i64; 256]>() as u64,
        );
        p = test;
        while *p as i32 != '\0' as i32 {
            let fresh3 = p;
            p = p.offset(1);
            freqs[*fresh3 as usize] += 1;
            freqs[*fresh3 as usize];
        }
        r = create_huffman_codes(freqs.as_mut_ptr());
        i = 0;
        while i < 256 {
            if !(*r.offset(i as isize)).is_null() {
                inttobits(
                    (**r.offset(i as isize)).code,
                    (**r.offset(i as isize)).nbits,
                    strbit.as_mut_ptr(),
                );
                print!(
                    "{} ({}) {}\n",
                    i,
                    (**r.offset(i as isize)).code,
                    build_str_from_raw_ptr(strbit.as_mut_ptr() as *mut u8)
                );
            }
            i += 1;
            i;
        }
        free_huffman_codes(r);
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
