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
    fn strtol(_: *const i8, _: *mut *mut i8, _: i32) -> i64;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_t {
    pub is_list: i32,
    pub ival: i32,
    pub lst: *mut list,
}
pub type list = *mut list_t;
#[no_mangle]
pub extern "C" fn new_list() -> list {
    unsafe {
        let mut x: list = malloc(::core::mem::size_of::<list_t>() as u64) as list;
        (*x).ival = 0;
        (*x).is_list = 1;
        (*x).lst = 0 as *mut list;
        return x;
    }
}

#[no_mangle]
pub extern "C" fn append(mut parent: list, mut child: list) {
    unsafe {
        (*parent).lst = realloc(
            (*parent).lst as *mut libc::c_void,
            (::core::mem::size_of::<list>() as u64).wrapping_mul(((*parent).ival + 1i32) as u64),
        ) as *mut list;
    }
    let fresh0 = (*parent).ival;
    (*parent).ival = (*parent).ival + 1;
    unsafe {
        let ref mut fresh1 = *((*parent).lst).offset(fresh0 as isize);
        *fresh1 = child;
    }
}

#[no_mangle]
pub extern "C" fn from_string(mut s: *mut i8, mut e: *mut *mut i8, mut parent: list) -> list {
    unsafe {
        let mut ret: list = 0 as list;
        if parent.is_null() {
            parent = new_list();
        }
        while *s as i32 != '\0' as i32 {
            if *s as i32 == ']' as i32 {
                if !e.is_null() {
                    *e = s.offset(1 as isize);
                }
                return parent;
            }
            if *s as i32 == '[' as i32 {
                ret = new_list();
                (*ret).is_list = 1;
                (*ret).ival = 0;
                append(parent, ret);
                from_string(s.offset(1 as isize), &mut s, ret);
            } else if *s as i32 >= '0' as i32 && *s as i32 <= '9' as i32 {
                ret = new_list();
                (*ret).is_list = 0;
                (*ret).ival = strtol(s, &mut s, 10) as i32;
                append(parent, ret);
            } else {
                s = s.offset(1);
                s;
            }
        }
        if !e.is_null() {
            *e = s;
        }
        return parent;
    }
}

#[no_mangle]
pub extern "C" fn show_list(mut l: list) {
    let mut i: i32 = 0;
    if l.is_null() {
        return;
    }
    if (*l).is_list == 0 {
        print!("{}", (*l).ival);
        return;
    }
    print!("[");
    i = 0;
    unsafe {
        while i < (*l).ival {
            show_list(*((*l).lst).offset(i as isize));
            if i < (*l).ival - 1 {
                print!(", ");
            }
            i += 1;
            i;
        }
    }
    print!("]");
}

#[no_mangle]
pub extern "C" fn flatten(mut from: list, mut to: list) -> list {
    let mut i: i32 = 0;
    let mut t: list = 0 as *mut list_t;
    if to.is_null() {
        to = new_list();
    }
    unsafe {
        if (*from).is_list == 0 {
            t = new_list();
            *t = *from;
            append(to, t);
        } else {
            i = 0;
            while i < (*from).ival {
                flatten(*((*from).lst).offset(i as isize), to);
                i += 1;
                i;
            }
        }
    }
    return to;
}

#[no_mangle]
pub extern "C" fn delete_list(mut l: list) {
    let mut i: i32 = 0;
    if l.is_null() {
        return;
    }
    unsafe {
        if (*l).is_list != 0 && (*l).ival != 0 {
            i = 0;
            while i < (*l).ival {
                delete_list(*((*l).lst).offset(i as isize));
                i += 1;
                i;
            }
            free((*l).lst as *mut libc::c_void);
        }
        free(l as *mut libc::c_void);
    }
}

fn main_0() -> i32 {
    let mut l: list = from_string(
        b"[1], 2, [[3,4], 5], [[[]]], [[[6]]], 7, 8, []\0" as *const u8 as *const i8 as *mut i8,
        0 as *mut *mut i8,
        0 as list,
    );
    print!("Nested: ");
    show_list(l);
    print!("\n");
    let mut flat: list = flatten(l, 0 as list);
    print!("Flattened: ");
    show_list(flat);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
