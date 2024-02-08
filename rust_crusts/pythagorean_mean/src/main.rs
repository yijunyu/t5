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
    fn atof(__nptr: *const i8) -> f64;
    fn pow(_: f64, _: f64) -> f64;
}
fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut count: i32 = 0;
        let mut f: f64 = 0.;
        let mut sum: f64 = 0.0f64;
        let mut prod: f64 = 1.0f64;
        let mut resum: f64 = 0.0f64;
        i = 1;
        while i < argc {
            f = atof(*argv.offset(i as isize));
            count += 1;
            count;
            sum += f;
            prod *= f;
            resum += 1.0f64 / f;
            i += 1;
            i;
        }
        print!("Arithmetic mean = {}\n", sum / count as f64);
        print!("Geometric mean = {}\n", pow(prod, 1.0f64 / count as f64));
        print!("Harmonic mean = {}\n", count as f64 / resum);
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
