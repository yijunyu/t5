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
    fn puts(__s: *const i8) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node {
    pub s: *mut i8,
    pub prev: *mut node,
}
#[no_mangle]
pub extern "C" fn powerset(mut v: *mut *mut i8, mut n: i32, mut up: *mut node) {
    unsafe {
        let mut me: node = node {
            s: 0 as *mut i8,
            prev: 0 as *mut node,
        };
        if n == 0 {
            print!("{}", '[' as i32);
            while !up.is_null() {
                print!(" {}", build_str_from_raw_ptr((*up).s as *mut u8));
                up = (*up).prev;
            }
            puts(b" ]\0" as *const u8 as *const i8);
        } else {
            me.s = *v;
            me.prev = up;
            powerset(v.offset(1 as isize), n - 1, up);
            powerset(v.offset(1 as isize), n - 1, &mut me);
        };
    }
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        powerset(argv.offset(1 as isize), argc - 1, 0 as *mut node);
        return 0;
    }
}

pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    ::std::process::exit(main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32);
}
