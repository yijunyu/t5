#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn rand() -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_double_t {
    pub left: node_double,
    pub right: node_double,
    pub value: libc::c_double,
}
pub type node_double = *mut node_double_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_int_t {
    pub left: node_int,
    pub right: node_int,
    pub value: libc::c_int,
}
pub type node_int = *mut node_int_t;
#[no_mangle]
pub unsafe extern "C" fn node_double_new(mut v: libc::c_double) -> node_double {
    let mut node: node_double = malloc(
        ::core::mem::size_of::<node_double_t>() as libc::c_ulong,
    ) as node_double;
    (*node).value = v;
    (*node).right = 0 as node_double;
    (*node).left = (*node).right;
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn node_double_insert(
    mut root: node_double,
    mut v: libc::c_double,
) -> node_double {
    let mut n: node_double = node_double_new(v);
    while !root.is_null() {
        if (*root).value < (*n).value {
            if ((*root).left).is_null() {
                (*root).left = n;
                return (*root).left;
            } else {
                root = (*root).left;
            }
        } else if ((*root).right).is_null() {
            (*root).right = n;
            return (*root).right;
        } else {
            root = (*root).right;
        }
    }
    return 0 as node_double;
}
#[no_mangle]
pub unsafe extern "C" fn node_int_new(mut v: libc::c_int) -> node_int {
    let mut node: node_int = malloc(
        ::core::mem::size_of::<node_int_t>() as libc::c_ulong,
    ) as node_int;
    (*node).value = v;
    (*node).right = 0 as node_int;
    (*node).left = (*node).right;
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn node_int_insert(
    mut root: node_int,
    mut v: libc::c_int,
) -> node_int {
    let mut n: node_int = node_int_new(v);
    while !root.is_null() {
        if (*root).value < (*n).value {
            if ((*root).left).is_null() {
                (*root).left = n;
                return (*root).left;
            } else {
                root = (*root).left;
            }
        } else if ((*root).right).is_null() {
            (*root).right = n;
            return (*root).right;
        } else {
            root = (*root).right;
        }
    }
    return 0 as node_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut root_d: node_double = node_double_new(
        rand() as libc::c_double / 2147483647 as libc::c_int as libc::c_double,
    );
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        node_double_insert(
            root_d,
            rand() as libc::c_double / 2147483647 as libc::c_int as libc::c_double,
        );
        i += 1;
        i;
    }
    let mut root_i: node_int = node_int_new(rand());
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        node_int_insert(root_i, rand());
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
