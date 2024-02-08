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
    fn sqrt(_: f64) -> f64;
}
#[no_mangle]
pub extern "C" fn sum_proper_divisors(n: u32) -> u32 {
    let mut sum: u32 = 1;
    let mut i: u32 = 3;
    let mut j: u32 = 0;
    unsafe {
        while (i as f64) < sqrt(n as f64) + 1 as f64 {
            if n.wrapping_rem(i) == 0 {
                j = n.wrapping_div(i);
                sum = sum.wrapping_add(i.wrapping_add((if i == j { 0 } else { j })));
            }
            i = i.wrapping_add(2);
        }
    }
    return sum;
}

fn main_0(mut argc: i32, mut argv: *mut *const i8) -> i32 {
    unsafe {
        let mut n: u32 = 0;
        let mut c: u32 = 0;
        n = 1;
        c = 0;
        while c < 25 {
            if n < sum_proper_divisors(n) {
                c = c.wrapping_add(1);
                print!("{}: {}\n", c, n);
            }
            n = n.wrapping_add(2);
        }
        while c < 1000 {
            if n < sum_proper_divisors(n) {
                c = c.wrapping_add(1);
                c;
            }
            n = n.wrapping_add(2);
        }
        print!("\nThe one thousandth abundant odd number is: {}\n", n);
        n = 1000000001;
        while !(n < sum_proper_divisors(n)) {
            n = n.wrapping_add(2);
        }
        print!(
            "The first abundant odd number above one billion is: {}\n",
            n
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
    ::std::process::exit(
        main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *const i8) as i32,
    );
}
