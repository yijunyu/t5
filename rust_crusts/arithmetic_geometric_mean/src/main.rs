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
    fn fabs(_: f64) -> f64;
    fn scanf(_: *const i8, _: ...) -> i32;
    fn exit(_: i32) -> !;
}
#[no_mangle]
pub extern "C" fn agm(mut a: f64, mut g: f64) -> f64 {
    let mut iota: f64 = 1.0E-16f64;
    let mut a1: f64 = 0.;
    let mut g1: f64 = 0.;
    unsafe {
        if a * g < 0.0f64 {
            print!("arithmetic-geometric mean undefined when x*y<0\n");
            exit(1);
        }
        while fabs(a - g) > iota {
            a1 = (a + g) / 2.0f64;
            g1 = sqrt(a * g);
            a = a1;
            g = g1;
        }
    }
    return a;
}

fn main_0() -> i32 {
    let mut x: f64 = 0.;
    let mut y: f64 = 0.;
    print!("Enter two numbers: ");
    unsafe {
        scanf(
            b"%lf%lf\0" as *const u8 as *const i8,
            &mut x as *mut f64,
            &mut y as *mut f64,
        );
    }
    print!("The arithmetic-geometric mean is {}\n", agm(x, y));
    return 0;
}

pub fn main() {
    unsafe {
        ::std::process::exit(main_0() as i32);
    }
}
