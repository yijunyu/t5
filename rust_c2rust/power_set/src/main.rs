#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn putchar(__c: libc::c_int) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node {
    pub s: *mut libc::c_char,
    pub prev: *mut node,
}
#[no_mangle]
pub unsafe extern "C" fn powerset(
    mut v: *mut *mut libc::c_char,
    mut n: libc::c_int,
    mut up: *mut node,
) {
    let mut me: node = node {
        s: 0 as *mut libc::c_char,
        prev: 0 as *mut node,
    };
    if n == 0 {
        putchar('[' as i32);
        while !up.is_null() {
            printf(b" %s\0" as *const u8 as *const libc::c_char, (*up).s);
            up = (*up).prev;
        }
        puts(b" ]\0" as *const u8 as *const libc::c_char);
    } else {
        me.s = *v;
        me.prev = up;
        powerset(v.offset(1 as libc::c_int as isize), n - 1 as libc::c_int, up);
        powerset(v.offset(1 as libc::c_int as isize), n - 1 as libc::c_int, &mut me);
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    powerset(
        argv.offset(1 as libc::c_int as isize),
        argc - 1 as libc::c_int,
        0 as *mut node,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
