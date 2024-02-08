#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_t {
    pub priority: libc::c_int,
    pub data: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct heap_t {
    pub nodes: *mut node_t,
    pub len: libc::c_int,
    pub size: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn push(
    mut h: *mut heap_t,
    mut priority: libc::c_int,
    mut data: *mut libc::c_char,
) {
    if (*h).len + 1 as libc::c_int >= (*h).size {
        (*h)
            .size = if (*h).size != 0 {
            (*h).size * 2 as libc::c_int
        } else {
            4 as libc::c_int
        };
        (*h)
            .nodes = realloc(
            (*h).nodes as *mut libc::c_void,
            ((*h).size as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<node_t>() as libc::c_ulong),
        ) as *mut node_t;
    }
    let mut i: libc::c_int = (*h).len + 1 as libc::c_int;
    let mut j: libc::c_int = i / 2 as libc::c_int;
    while i > 1 as libc::c_int && (*((*h).nodes).offset(j as isize)).priority > priority
    {
        *((*h).nodes).offset(i as isize) = *((*h).nodes).offset(j as isize);
        i = j;
        j = j / 2 as libc::c_int;
    }
    (*((*h).nodes).offset(i as isize)).priority = priority;
    let ref mut fresh0 = (*((*h).nodes).offset(i as isize)).data;
    *fresh0 = data;
    (*h).len += 1;
    (*h).len;
}
#[no_mangle]
pub unsafe extern "C" fn pop(mut h: *mut heap_t) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    if (*h).len == 0 {
        return 0 as *mut libc::c_char;
    }
    let mut data: *mut libc::c_char = (*((*h).nodes).offset(1 as libc::c_int as isize))
        .data;
    *((*h).nodes)
        .offset(1 as libc::c_int as isize) = *((*h).nodes).offset((*h).len as isize);
    (*h).len -= 1;
    (*h).len;
    i = 1 as libc::c_int;
    while i != (*h).len + 1 as libc::c_int {
        k = (*h).len + 1 as libc::c_int;
        j = 2 as libc::c_int * i;
        if j <= (*h).len
            && (*((*h).nodes).offset(j as isize)).priority
                < (*((*h).nodes).offset(k as isize)).priority
        {
            k = j;
        }
        if j + 1 as libc::c_int <= (*h).len
            && (*((*h).nodes).offset((j + 1 as libc::c_int) as isize)).priority
                < (*((*h).nodes).offset(k as isize)).priority
        {
            k = j + 1 as libc::c_int;
        }
        *((*h).nodes).offset(i as isize) = *((*h).nodes).offset(k as isize);
        i = k;
    }
    return data;
}
unsafe fn main_0() -> libc::c_int {
    let mut h: *mut heap_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<heap_t>() as libc::c_ulong,
    ) as *mut heap_t;
    push(
        h,
        3 as libc::c_int,
        b"Clear drains\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    push(
        h,
        4 as libc::c_int,
        b"Feed cat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    push(
        h,
        5 as libc::c_int,
        b"Make tea\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    push(
        h,
        1 as libc::c_int,
        b"Solve RC tasks\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    push(
        h,
        2 as libc::c_int,
        b"Tax return\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, pop(h));
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
