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
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_double_t {
    pub left: node_double,
    pub right: node_double,
    pub value: f64,
}
pub type node_double = *mut node_double_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_int_t {
    pub left: node_int,
    pub right: node_int,
    pub value: i32,
}
pub type node_int = *mut node_int_t;
#[no_mangle]
pub extern "C" fn node_double_new(mut v: f64) -> node_double {
    unsafe {
        let mut node: node_double =
            malloc(::core::mem::size_of::<node_double_t>() as u64) as node_double;
        (*node).value = v;
        (*node).right = 0 as node_double;
        (*node).left = (*node).right;
        return node;
    }
}

#[no_mangle]
pub extern "C" fn node_double_insert(mut root: node_double, mut v: f64) -> node_double {
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
pub extern "C" fn node_int_new(mut v: i32) -> node_int {
    unsafe {
        let mut node: node_int = malloc(::core::mem::size_of::<node_int_t>() as u64) as node_int;
        (*node).value = v;
        (*node).right = 0 as node_int;
        (*node).left = (*node).right;
        return node;
    }
}

#[no_mangle]
pub extern "C" fn node_int_insert(mut root: node_int, mut v: i32) -> node_int {
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

fn main_0() -> i32 {
    let mut i: i32 = 0;
    unsafe {
        let mut root_d: node_double = node_double_new(rand() as f64 / 2147483647 as f64);
        i = 0;
        while i < 10000 {
            node_double_insert(root_d, rand() as f64 / 2147483647 as f64);
            i += 1;
            i;
        }
        let mut root_i: node_int = node_int_new(rand());
        i = 0;
        while i < 10000 {
            node_int_insert(root_i, rand());
            i += 1;
            i;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
