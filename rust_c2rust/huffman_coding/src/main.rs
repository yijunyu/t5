#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abs(_: libc::c_int) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct huffcode {
    pub nbits: libc::c_int,
    pub code: libc::c_int,
}
pub type huffcode_t = huffcode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct huffheap {
    pub h: *mut libc::c_int,
    pub n: libc::c_int,
    pub s: libc::c_int,
    pub cs: libc::c_int,
    pub f: *mut libc::c_long,
}
pub type heap_t = huffheap;
unsafe extern "C" fn _heap_create(
    mut s: libc::c_int,
    mut f: *mut libc::c_long,
) -> *mut heap_t {
    let mut h: *mut heap_t = 0 as *mut heap_t;
    h = malloc(::core::mem::size_of::<heap_t>() as libc::c_ulong) as *mut heap_t;
    (*h)
        .h = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(s as libc::c_ulong),
    ) as *mut libc::c_int;
    (*h).cs = s;
    (*h).s = (*h).cs;
    (*h).n = 0 as libc::c_int;
    (*h).f = f;
    return h;
}
unsafe extern "C" fn _heap_destroy(mut heap: *mut heap_t) {
    free((*heap).h as *mut libc::c_void);
    free(heap as *mut libc::c_void);
}
unsafe extern "C" fn _heap_sort(mut heap: *mut heap_t) {
    let mut i: libc::c_int = 1 as libc::c_int;
    let mut j: libc::c_int = 2 as libc::c_int;
    let mut a: *mut libc::c_int = (*heap).h;
    while i < (*heap).n {
        if *((*heap).f).offset(*a.offset((i - 1 as libc::c_int) as isize) as isize)
            >= *((*heap).f).offset(*a.offset(i as isize) as isize)
        {
            i = j;
            j += 1;
            j;
        } else {
            let mut t_: libc::c_int = 0;
            t_ = *a.offset((i - 1 as libc::c_int) as isize);
            *a.offset((i - 1 as libc::c_int) as isize) = *a.offset(i as isize);
            *a.offset(i as isize) = t_;
            i -= 1;
            i;
            i = if i == 0 as libc::c_int {
                let fresh0 = j;
                j = j + 1;
                fresh0
            } else {
                i
            };
        }
    }
}
unsafe extern "C" fn _heap_add(mut heap: *mut heap_t, mut c: libc::c_int) {
    if (*heap).n + 1 as libc::c_int > (*heap).s {
        (*heap)
            .h = realloc(
            (*heap).h as *mut libc::c_void,
            ((*heap).s + (*heap).cs) as libc::c_ulong,
        ) as *mut libc::c_int;
        (*heap).s += (*heap).cs;
    }
    *((*heap).h).offset((*heap).n as isize) = c;
    (*heap).n += 1;
    (*heap).n;
    _heap_sort(heap);
}
unsafe extern "C" fn _heap_remove(mut heap: *mut heap_t) -> libc::c_int {
    if (*heap).n > 0 as libc::c_int {
        (*heap).n -= 1;
        (*heap).n;
        return *((*heap).h).offset((*heap).n as isize);
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn create_huffman_codes(
    mut freqs: *mut libc::c_long,
) -> *mut *mut huffcode_t {
    let mut codes: *mut *mut huffcode_t = 0 as *mut *mut huffcode_t;
    let mut heap: *mut heap_t = 0 as *mut heap_t;
    let mut efreqs: [libc::c_long; 512] = [0; 512];
    let mut preds: [libc::c_int; 512] = [0; 512];
    let mut i: libc::c_int = 0;
    let mut extf: libc::c_int = 256 as libc::c_int;
    let mut r1: libc::c_int = 0;
    let mut r2: libc::c_int = 0;
    memcpy(
        efreqs.as_mut_ptr() as *mut libc::c_void,
        freqs as *const libc::c_void,
        (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
            .wrapping_mul(256 as libc::c_int as libc::c_ulong),
    );
    memset(
        &mut *efreqs.as_mut_ptr().offset(256 as libc::c_int as isize)
            as *mut libc::c_long as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
            .wrapping_mul(256 as libc::c_int as libc::c_ulong),
    );
    heap = _heap_create(256 as libc::c_int * 2 as libc::c_int, efreqs.as_mut_ptr());
    if heap.is_null() {
        return 0 as *mut *mut huffcode_t;
    }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if efreqs[i as usize] > 0 as libc::c_int as libc::c_long {
            _heap_add(heap, i);
        }
        i += 1;
        i;
    }
    while (*heap).n > 1 as libc::c_int {
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
    codes = malloc(
        (::core::mem::size_of::<*mut huffcode_t>() as libc::c_ulong)
            .wrapping_mul(256 as libc::c_int as libc::c_ulong),
    ) as *mut *mut huffcode_t;
    let mut bc: libc::c_int = 0;
    let mut bn: libc::c_int = 0;
    let mut ix: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        bc = 0 as libc::c_int;
        bn = 0 as libc::c_int;
        if efreqs[i as usize] == 0 as libc::c_int as libc::c_long {
            let ref mut fresh1 = *codes.offset(i as isize);
            *fresh1 = 0 as *mut huffcode_t;
        } else {
            ix = i;
            while abs(preds[ix as usize]) != ix {
                bc
                    |= (if preds[ix as usize] >= 0 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) << bn;
                ix = abs(preds[ix as usize]);
                bn += 1;
                bn;
            }
            let ref mut fresh2 = *codes.offset(i as isize);
            *fresh2 = malloc(::core::mem::size_of::<huffcode_t>() as libc::c_ulong)
                as *mut huffcode_t;
            (**codes.offset(i as isize)).nbits = bn;
            (**codes.offset(i as isize)).code = bc;
        }
        i += 1;
        i;
    }
    return codes;
}
#[no_mangle]
pub unsafe extern "C" fn free_huffman_codes(mut c: *mut *mut huffcode_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        free(*c.offset(i as isize) as *mut libc::c_void);
        i += 1;
        i;
    }
    free(c as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn inttobits(
    mut c: libc::c_int,
    mut n: libc::c_int,
    mut s: *mut libc::c_char,
) {
    *s.offset(n as isize) = 0 as libc::c_int as libc::c_char;
    while n > 0 as libc::c_int {
        *s
            .offset(
                (n - 1 as libc::c_int) as isize,
            ) = (c % 2 as libc::c_int + '0' as i32) as libc::c_char;
        c >>= 1 as libc::c_int;
        n -= 1;
        n;
    }
}
#[no_mangle]
pub static mut test: *const libc::c_char = b"this is an example for huffman encoding\0"
    as *const u8 as *const libc::c_char;
unsafe fn main_0() -> libc::c_int {
    let mut r: *mut *mut huffcode_t = 0 as *mut *mut huffcode_t;
    let mut i: libc::c_int = 0;
    let mut strbit: [libc::c_char; 100] = [0; 100];
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut freqs: [libc::c_long; 256] = [0; 256];
    memset(
        freqs.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_long; 256]>() as libc::c_ulong,
    );
    p = test;
    while *p as libc::c_int != '\0' as i32 {
        let fresh3 = p;
        p = p.offset(1);
        freqs[*fresh3 as usize] += 1;
        freqs[*fresh3 as usize];
    }
    r = create_huffman_codes(freqs.as_mut_ptr());
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if !(*r.offset(i as isize)).is_null() {
            inttobits(
                (**r.offset(i as isize)).code,
                (**r.offset(i as isize)).nbits,
                strbit.as_mut_ptr(),
            );
            printf(
                b"%c (%d) %s\n\0" as *const u8 as *const libc::c_char,
                i,
                (**r.offset(i as isize)).code,
                strbit.as_mut_ptr(),
            );
        }
        i += 1;
        i;
    }
    free_huffman_codes(r);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
