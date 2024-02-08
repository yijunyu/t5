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
    fn scanf(_: *const i8, _: ...) -> i32;
}
fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut user1: i32 = 0;
        let mut user2: i32 = 0;
        print!("Enter two integers.  Space delimited, please:  ");
        scanf(
            b"%d %d\0" as *const u8 as *const i8,
            &mut user1 as *mut i32,
            &mut user2 as *mut i32,
        );
        let vla = user1 as usize;
        let vla_0 = user2 as usize;
        let mut array: Vec<i32> = ::std::vec::from_elem(0, vla * vla_0);
        *array
            .as_mut_ptr()
            .offset((user1 / 2i32) as isize * vla_0 as isize)
            .offset((user2 / 2i32) as isize) = user1 + user2;
        print!(
            "array[{}][{}] is {}\n",
            user1 / 2,
            user2 / 2,
            *array
                .as_mut_ptr()
                .offset((user1 / 2i32) as isize * vla_0 as isize)
                .offset((user2 / 2i32) as isize)
        );
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
