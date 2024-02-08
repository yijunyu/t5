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
    fn puts(__s: *const i8) -> i32;
    fn exp(_: f64) -> f64;
}
fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut e: f64 = 0.;
        puts (b"The double precision in C give about 15 significant digits.\nValues below are presented with 16 digits after the decimal point.\n\0" as * const u8 as * const i8,);
        e = exp(1 as f64);
        print!("Euler constant e = {:.16}\n", e);
        let mut n: i32 = 8192;
        e = 1.0f64 + 1.0f64 / n as f64;
        let mut i: i32 = 0;
        while i < 13 {
            e *= e;
            i += 1;
            i;
        }
        print!("Euler constant e = {:.16}\n", e);
        let N: i32 = 1000;
        let mut a: [f64; 1000] = [0.; 1000];
        a[0 as usize] = 1.0f64;
        let mut i_0: i32 = 1;
        while i_0 < N {
            a[i_0 as usize] = a[(i_0 - 1i32) as usize] / i_0 as f64;
            i_0 += 1;
            i_0;
        }
        e = 1.0f64;
        let mut i_1: i32 = N - 1;
        while i_1 > 0 {
            e += a[i_1 as usize];
            i_1 -= 1;
            i_1;
        }
        print!("Euler constant e = {:.16}\n", e);
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
