#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn rand() -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type DATA = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct queue_t {
    pub buf: *mut DATA,
    pub head: size_t,
    pub tail: size_t,
    pub alloc: size_t,
}
pub type queue = *mut queue_t;
#[no_mangle]
pub unsafe extern "C" fn q_new() -> queue {
    let mut q: queue = malloc(::core::mem::size_of::<queue_t>() as libc::c_ulong)
        as queue;
    (*q).alloc = 4 as libc::c_int as size_t;
    (*q)
        .buf = malloc(
        (::core::mem::size_of::<DATA>() as libc::c_ulong).wrapping_mul((*q).alloc),
    ) as *mut DATA;
    (*q).tail = 0 as libc::c_int as size_t;
    (*q).head = (*q).tail;
    return q;
}
#[no_mangle]
pub unsafe extern "C" fn empty(mut q: queue) -> libc::c_int {
    return ((*q).tail == (*q).head) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn enqueue(mut q: queue, mut n: DATA) {
    if (*q).tail >= (*q).alloc {
        (*q).tail = 0 as libc::c_int as size_t;
    }
    let fresh0 = (*q).tail;
    (*q).tail = ((*q).tail).wrapping_add(1);
    *((*q).buf).offset(fresh0 as isize) = n;
    if (*q).tail == (*q).alloc {
        (*q)
            .buf = realloc(
            (*q).buf as *mut libc::c_void,
            (::core::mem::size_of::<DATA>() as libc::c_ulong)
                .wrapping_mul((*q).alloc)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        ) as *mut DATA;
        if (*q).head != 0 {
            memcpy(
                ((*q).buf).offset((*q).head as isize).offset((*q).alloc as isize)
                    as *mut libc::c_void,
                ((*q).buf).offset((*q).head as isize) as *const libc::c_void,
                (::core::mem::size_of::<DATA>() as libc::c_ulong)
                    .wrapping_mul(((*q).alloc).wrapping_sub((*q).head)),
            );
            (*q)
                .head = ((*q).head as libc::c_ulong).wrapping_add((*q).alloc) as size_t
                as size_t;
        } else {
            (*q).tail = (*q).alloc;
        }
        (*q)
            .alloc = ((*q).alloc as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dequeue(mut q: queue, mut n: *mut DATA) -> libc::c_int {
    if (*q).head == (*q).tail {
        return 0 as libc::c_int;
    }
    let fresh1 = (*q).head;
    (*q).head = ((*q).head).wrapping_add(1);
    *n = *((*q).buf).offset(fresh1 as isize);
    if (*q).head >= (*q).alloc {
        (*q).head = 0 as libc::c_int as size_t;
        if (*q).alloc >= 512 as libc::c_int as libc::c_ulong
            && (*q).tail < ((*q).alloc).wrapping_div(2 as libc::c_int as libc::c_ulong)
        {
            (*q)
                .alloc = ((*q).alloc as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            (*q)
                .buf = realloc(
                (*q).buf as *mut libc::c_void,
                (::core::mem::size_of::<DATA>() as libc::c_ulong)
                    .wrapping_mul((*q).alloc),
            ) as *mut DATA;
        }
    }
    return 1 as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut q: queue = q_new();
    i = 0 as libc::c_int;
    while i < 100000000 as libc::c_int {
        n = rand();
        if n > 2147483647 as libc::c_int / 2 as libc::c_int {
            enqueue(q, n);
        } else {
            dequeue(q, &mut n) == 0;
        }
        i += 1;
        i;
    }
    while dequeue(q, &mut n) != 0 {}
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
