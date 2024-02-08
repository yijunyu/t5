#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn putchar(__c: libc::c_int) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node {
    pub val: libc::c_int,
    pub len: libc::c_int,
    pub next: *mut node,
}
#[no_mangle]
pub unsafe extern "C" fn lis(mut v: *mut libc::c_int, mut len: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut p: *mut node = 0 as *mut node;
    let mut n: *mut node = calloc(
        len as libc::c_ulong,
        ::core::mem::size_of::<node>() as libc::c_ulong,
    ) as *mut node;
    i = 0 as libc::c_int;
    while i < len {
        (*n.offset(i as isize)).val = *v.offset(i as isize);
        i += 1;
        i;
    }
    i = len;
    loop {
        let fresh0 = i;
        i = i - 1;
        if !(fresh0 != 0) {
            break;
        }
        p = n.offset(i as isize);
        loop {
            let fresh1 = p;
            p = p.offset(1);
            if !(fresh1 < n.offset(len as isize)) {
                break;
            }
            if (*p).val > (*n.offset(i as isize)).val
                && (*p).len >= (*n.offset(i as isize)).len
            {
                let ref mut fresh2 = (*n.offset(i as isize)).next;
                *fresh2 = p;
                (*n.offset(i as isize)).len = (*p).len + 1 as libc::c_int;
            }
        }
    }
    i = 0 as libc::c_int;
    p = n;
    while i < len {
        if (*n.offset(i as isize)).len > (*p).len {
            p = n.offset(i as isize);
        }
        i += 1;
        i;
    }
    loop {
        printf(b" %d\0" as *const u8 as *const libc::c_char, (*p).val);
        p = (*p).next;
        if p.is_null() {
            break;
        }
    }
    putchar('\n' as i32);
    free(n as *mut libc::c_void);
}
unsafe fn main_0() -> libc::c_int {
    let mut x: [libc::c_int; 6] = [
        3 as libc::c_int,
        2 as libc::c_int,
        6 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
    ];
    let mut y: [libc::c_int; 16] = [
        0 as libc::c_int,
        8 as libc::c_int,
        4 as libc::c_int,
        12 as libc::c_int,
        2 as libc::c_int,
        10 as libc::c_int,
        6 as libc::c_int,
        14 as libc::c_int,
        1 as libc::c_int,
        9 as libc::c_int,
        5 as libc::c_int,
        13 as libc::c_int,
        3 as libc::c_int,
        11 as libc::c_int,
        7 as libc::c_int,
        15 as libc::c_int,
    ];
    lis(
        x.as_mut_ptr(),
        (::core::mem::size_of::<[libc::c_int; 6]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int,
    );
    lis(
        y.as_mut_ptr(),
        (::core::mem::size_of::<[libc::c_int; 16]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
