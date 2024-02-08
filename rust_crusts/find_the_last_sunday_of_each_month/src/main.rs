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
}
fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut days: [i32; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut m: i32 = 0;
        let mut y: i32 = 0;
        let mut w: i32 = 0;
        if argc < 2 || {
            y = atoi(*argv.offset(1 as isize));
            y <= 1752
        } {
            return 1;
        }
        days[1 as usize] -= (y % 4 != 0 || y % 100 == 0 && y % 400 != 0) as i32;
        w = y * 365 + 97 * (y - 1) / 400 + 4;
        m = 0;
        while m < 12 {
            w = (w + days[m as usize]) % 7;
            print!("{}-{:02}-{}\n", y, m + 1, days[m as usize] - w);
            m += 1;
            m;
        }
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
