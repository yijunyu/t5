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
    fn atoi(__nptr: *const i8) -> i32;
    fn exit(_: i32) -> !;
}
fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        if argc < 3 {
            exit(1);
        }
        argc -= 1;
        b = atoi(*argv.offset(argc as isize));
        if b == 0 {
            exit(2);
        }
        argc -= 1;
        a = atoi(*argv.offset(argc as isize));
        print!("a+b = {}\n", a + b);
        print!("a-b = {}\n", a - b);
        print!("a*b = {}\n", a * b);
        print!("a/b = {}\n", a / b);
        print!("a%b = {}\n", a % b);
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
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32,
        );
    }
}
