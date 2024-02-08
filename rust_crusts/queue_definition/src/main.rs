#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use c2rust_out::*;
extern "C" {
    fn rand() -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct queue_t {
    pub buf: *mut i32,
    pub head: u64,
    pub tail: u64,
    pub alloc: u64,
}
pub type queue = *mut queue_t;
#[no_mangle]
pub extern "C" fn q_new() -> queue {
    unsafe {
        let mut q: queue = malloc(::core::mem::size_of::<queue_t>() as u64) as queue;
        (*q).alloc = 4;
        (*q).buf =
            malloc((::core::mem::size_of::<i32>() as u64).wrapping_mul((*q).alloc)) as *mut i32;
        (*q).tail = 0;
        (*q).head = (*q).tail;
        return q;
    }
}

#[no_mangle]
pub extern "C" fn empty(mut q: queue) -> i32 {
    return ((*q).tail == (*q).head) as i32;
}

#[no_mangle]
pub extern "C" fn enqueue(mut q: queue, mut n: i32) {
    if (*q).tail >= (*q).alloc {
        (*q).tail = 0;
    }
    let fresh0 = (*q).tail;
    (*q).tail = ((*q).tail).wrapping_add(1);
    unsafe {
        *((*q).buf).offset(fresh0 as isize) = n;
        if (*q).tail == (*q).alloc {
            (*q).buf = realloc(
                (*q).buf as *mut libc::c_void,
                (::core::mem::size_of::<i32>() as u64)
                    .wrapping_mul((*q).alloc)
                    .wrapping_mul(2),
            ) as *mut i32;
            if (*q).head != 0 {
                memcpy(
                    ((*q).buf)
                        .offset((*q).head as isize)
                        .offset((*q).alloc as isize) as *mut libc::c_void,
                    ((*q).buf).offset((*q).head as isize) as *const libc::c_void,
                    (::core::mem::size_of::<i32>() as u64)
                        .wrapping_mul(((*q).alloc).wrapping_sub((*q).head)),
                );
                (*q).head = ((*q).head as u64).wrapping_add((*q).alloc) as u64;
            } else {
                (*q).tail = (*q).alloc;
            };
            (*q).alloc = ((*q).alloc as u64).wrapping_mul(2) as u64;
        }
    }
}

#[no_mangle]
pub extern "C" fn dequeue(mut q: queue, mut n: *mut i32) -> i32 {
    unsafe {
        if (*q).head == (*q).tail {
            return 0;
        }
        let fresh1 = (*q).head;
        (*q).head = ((*q).head).wrapping_add(1);
        *n = *((*q).buf).offset(fresh1 as isize);
        if (*q).head >= (*q).alloc {
            (*q).head = 0;
            if (*q).alloc >= 512 && (*q).tail < ((*q).alloc).wrapping_div(2) {
                (*q).alloc = ((*q).alloc as u64).wrapping_div(2) as u64;
                (*q).buf = realloc(
                    (*q).buf as *mut libc::c_void,
                    (::core::mem::size_of::<i32>() as u64).wrapping_mul((*q).alloc),
                ) as *mut i32;
            }
        }
        return 1;
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    let mut q: queue = q_new();
    i = 0;
    unsafe {
        while i < 100000000 {
            n = rand();
            if n > 2147483647 / 2 {
                enqueue(q, n);
            } else {
                dequeue(q, &mut n) == 0;
            }
            i += 1;
            i;
        }
    }
    while dequeue(q, &mut n) != 0 {}
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
