#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_t {
    pub is_list: libc::c_int,
    pub ival: libc::c_int,
    pub lst: *mut list,
}
pub type list = *mut list_t;
#[no_mangle]
pub unsafe extern "C" fn new_list() -> list {
    let mut x: list = malloc(::core::mem::size_of::<list_t>() as libc::c_ulong) as list;
    (*x).ival = 0 as libc::c_int;
    (*x).is_list = 1 as libc::c_int;
    (*x).lst = 0 as *mut list;
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn append(mut parent: list, mut child: list) {
    (*parent)
        .lst = realloc(
        (*parent).lst as *mut libc::c_void,
        (::core::mem::size_of::<list>() as libc::c_ulong)
            .wrapping_mul(((*parent).ival + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut list;
    let fresh0 = (*parent).ival;
    (*parent).ival = (*parent).ival + 1;
    let ref mut fresh1 = *((*parent).lst).offset(fresh0 as isize);
    *fresh1 = child;
}
#[no_mangle]
pub unsafe extern "C" fn from_string(
    mut s: *mut libc::c_char,
    mut e: *mut *mut libc::c_char,
    mut parent: list,
) -> list {
    let mut ret: list = 0 as list;
    if parent.is_null() {
        parent = new_list();
    }
    while *s as libc::c_int != '\0' as i32 {
        if *s as libc::c_int == ']' as i32 {
            if !e.is_null() {
                *e = s.offset(1 as libc::c_int as isize);
            }
            return parent;
        }
        if *s as libc::c_int == '[' as i32 {
            ret = new_list();
            (*ret).is_list = 1 as libc::c_int;
            (*ret).ival = 0 as libc::c_int;
            append(parent, ret);
            from_string(s.offset(1 as libc::c_int as isize), &mut s, ret);
        } else if *s as libc::c_int >= '0' as i32 && *s as libc::c_int <= '9' as i32 {
            ret = new_list();
            (*ret).is_list = 0 as libc::c_int;
            (*ret).ival = strtol(s, &mut s, 10 as libc::c_int) as libc::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn show_list(mut l: list) {
    let mut i: libc::c_int = 0;
    if l.is_null() {
        return;
    }
    if (*l).is_list == 0 {
        printf(b"%d\0" as *const u8 as *const libc::c_char, (*l).ival);
        return;
    }
    printf(b"[\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < (*l).ival {
        show_list(*((*l).lst).offset(i as isize));
        if i < (*l).ival - 1 as libc::c_int {
            printf(b", \0" as *const u8 as *const libc::c_char);
        }
        i += 1;
        i;
    }
    printf(b"]\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn flatten(mut from: list, mut to: list) -> list {
    let mut i: libc::c_int = 0;
    let mut t: list = 0 as *mut list_t;
    if to.is_null() {
        to = new_list();
    }
    if (*from).is_list == 0 {
        t = new_list();
        *t = *from;
        append(to, t);
    } else {
        i = 0 as libc::c_int;
        while i < (*from).ival {
            flatten(*((*from).lst).offset(i as isize), to);
            i += 1;
            i;
        }
    }
    return to;
}
#[no_mangle]
pub unsafe extern "C" fn delete_list(mut l: list) {
    let mut i: libc::c_int = 0;
    if l.is_null() {
        return;
    }
    if (*l).is_list != 0 && (*l).ival != 0 {
        i = 0 as libc::c_int;
        while i < (*l).ival {
            delete_list(*((*l).lst).offset(i as isize));
            i += 1;
            i;
        }
        free((*l).lst as *mut libc::c_void);
    }
    free(l as *mut libc::c_void);
}
unsafe fn main_0() -> libc::c_int {
    let mut l: list = from_string(
        b"[1], 2, [[3,4], 5], [[[]]], [[[6]]], 7, 8, []\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        0 as *mut *mut libc::c_char,
        0 as list,
    );
    printf(b"Nested: \0" as *const u8 as *const libc::c_char);
    show_list(l);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    let mut flat: list = flatten(l, 0 as list);
    printf(b"Flattened: \0" as *const u8 as *const libc::c_char);
    show_list(flat);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
