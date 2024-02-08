#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn putchar(__c: libc::c_int) -> libc::c_int;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub static mut shades: *const libc::c_char = b".:!*oe&#%@\0" as *const u8
    as *const libc::c_char;
#[no_mangle]
pub static mut light: [libc::c_double; 3] = [
    30 as libc::c_int as libc::c_double,
    30 as libc::c_int as libc::c_double,
    -(50 as libc::c_int) as libc::c_double,
];
#[no_mangle]
pub unsafe extern "C" fn normalize(mut v: *mut libc::c_double) {
    let mut len: libc::c_double = sqrt(
        *v.offset(0 as libc::c_int as isize) * *v.offset(0 as libc::c_int as isize)
            + *v.offset(1 as libc::c_int as isize) * *v.offset(1 as libc::c_int as isize)
            + *v.offset(2 as libc::c_int as isize) * *v.offset(2 as libc::c_int as isize),
    );
    *v.offset(0 as libc::c_int as isize) /= len;
    *v.offset(1 as libc::c_int as isize) /= len;
    *v.offset(2 as libc::c_int as isize) /= len;
}
#[no_mangle]
pub unsafe extern "C" fn dot(
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
) -> libc::c_double {
    let mut d: libc::c_double = *x.offset(0 as libc::c_int as isize)
        * *y.offset(0 as libc::c_int as isize)
        + *x.offset(1 as libc::c_int as isize) * *y.offset(1 as libc::c_int as isize)
        + *x.offset(2 as libc::c_int as isize) * *y.offset(2 as libc::c_int as isize);
    return if d < 0 as libc::c_int as libc::c_double {
        -d
    } else {
        0 as libc::c_int as libc::c_double
    };
}
#[no_mangle]
pub unsafe extern "C" fn draw_sphere(
    mut R: libc::c_double,
    mut k: libc::c_double,
    mut ambient: libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut intensity: libc::c_int = 0;
    let mut b: libc::c_double = 0.;
    let mut vec: [libc::c_double; 3] = [0.; 3];
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    i = floor(-R) as libc::c_int;
    while i as libc::c_double <= ceil(R) {
        x = i as libc::c_double + 0.5f64;
        j = floor(-(2 as libc::c_int) as libc::c_double * R) as libc::c_int;
        while j as libc::c_double <= ceil(2 as libc::c_int as libc::c_double * R) {
            y = j as libc::c_double / 2.0f64 + 0.5f64;
            if x * x + y * y <= R * R {
                vec[0 as libc::c_int as usize] = x;
                vec[1 as libc::c_int as usize] = y;
                vec[2 as libc::c_int as usize] = sqrt(R * R - x * x - y * y);
                normalize(vec.as_mut_ptr());
                b = pow(dot(light.as_mut_ptr(), vec.as_mut_ptr()), k) + ambient;
                intensity = ((1 as libc::c_int as libc::c_double - b)
                    * (::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        as libc::c_double) as libc::c_int;
                if intensity < 0 as libc::c_int {
                    intensity = 0 as libc::c_int;
                }
                if intensity as libc::c_ulong
                    >= (::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                {
                    intensity = (::core::mem::size_of::<*const libc::c_char>()
                        as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong) as libc::c_int;
                }
                putchar(*shades.offset(intensity as isize) as libc::c_int);
            } else {
                putchar(' ' as i32);
            }
            j += 1;
            j;
        }
        putchar('\n' as i32);
        i += 1;
        i;
    }
}
unsafe fn main_0() -> libc::c_int {
    normalize(light.as_mut_ptr());
    draw_sphere(
        20 as libc::c_int as libc::c_double,
        4 as libc::c_int as libc::c_double,
        0.1f64,
    );
    draw_sphere(
        10 as libc::c_int as libc::c_double,
        2 as libc::c_int as libc::c_double,
        0.4f64,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
