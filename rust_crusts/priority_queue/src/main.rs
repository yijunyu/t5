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
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_t {
    pub priority: i32,
    pub data: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct heap_t {
    pub nodes: *mut node_t,
    pub len: i32,
    pub size: i32,
}
#[no_mangle]
pub extern "C" fn push(mut h: *mut heap_t, mut priority: i32, mut data: *mut i8) {
    unsafe {
        if (*h).len + 1 >= (*h).size {
            (*h).size = if (*h).size != 0 { (*h).size * 2 } else { 4 };
            (*h).nodes = realloc(
                (*h).nodes as *mut libc::c_void,
                ((*h).size as u64).wrapping_mul(::core::mem::size_of::<node_t>() as u64),
            ) as *mut node_t;
        }
        let mut i: i32 = (*h).len + 1;
        let mut j: i32 = i / 2;
        while i > 1 && (*((*h).nodes).offset(j as isize)).priority > priority {
            *((*h).nodes).offset(i as isize) = *((*h).nodes).offset(j as isize);
            i = j;
            j = j / 2;
        }
        (*((*h).nodes).offset(i as isize)).priority = priority;
        let ref mut fresh0 = (*((*h).nodes).offset(i as isize)).data;
        *fresh0 = data;
        (*h).len += 1;
        (*h).len;
    }
}

#[no_mangle]
pub extern "C" fn pop(mut h: *mut heap_t) -> *mut i8 {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        if (*h).len == 0 {
            return 0 as *mut i8;
        }
        let mut data: *mut i8 = (*((*h).nodes).offset(1 as isize)).data;
        *((*h).nodes).offset(1 as isize) = *((*h).nodes).offset((*h).len as isize);
        (*h).len -= 1;
        (*h).len;
        i = 1;
        while i != (*h).len + 1 {
            k = (*h).len + 1;
            j = 2 * i;
            if j <= (*h).len
                && (*((*h).nodes).offset(j as isize)).priority
                    < (*((*h).nodes).offset(k as isize)).priority
            {
                k = j;
            }
            if j + 1 <= (*h).len
                && (*((*h).nodes).offset((j + 1i32) as isize)).priority
                    < (*((*h).nodes).offset(k as isize)).priority
            {
                k = j + 1;
            };
            *((*h).nodes).offset(i as isize) = *((*h).nodes).offset(k as isize);
            i = k;
        }
        return data;
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut h: *mut heap_t = calloc(1, ::core::mem::size_of::<heap_t>() as u64) as *mut heap_t;
        push(h, 3, b"Clear drains\0" as *const u8 as *const i8 as *mut i8);
        push(h, 4, b"Feed cat\0" as *const u8 as *const i8 as *mut i8);
        push(h, 5, b"Make tea\0" as *const u8 as *const i8 as *mut i8);
        push(
            h,
            1,
            b"Solve RC tasks\0" as *const u8 as *const i8 as *mut i8,
        );
        push(h, 2, b"Tax return\0" as *const u8 as *const i8 as *mut i8);
        let mut i: i32 = 0;
        i = 0;
        while i < 5 {
            print!("{}\n", build_str_from_raw_ptr(pop(h) as *mut u8));
            i += 1;
            i;
        }
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
