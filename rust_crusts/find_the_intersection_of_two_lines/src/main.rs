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
    fn atof(__nptr: *const i8) -> f64;
    fn malloc(_: u64) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: f64,
    pub y: f64,
}
#[no_mangle]
pub extern "C" fn lineSlope(mut a: point, mut b: point) -> f64 {
    if a.x - b.x == 0.0f64 {
        return ::core::f32::NAN as f64;
    } else {
        return (a.y - b.y) / (a.x - b.x);
    };
}

#[no_mangle]
pub extern "C" fn extractPoint(mut str: *mut i8) -> point {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut start: i32 = 0;
        let mut end: i32 = 0;
        let mut length: i32 = 0;
        let mut holder: *mut i8 = 0 as *mut i8;
        let mut c: point = point { x: 0., y: 0. };
        i = 0;
        while *str.offset(i as isize) as i32 != 0 {
            if *str.offset(i as isize) as i32 == '(' as i32 {
                start = i;
            }
            if *str.offset(i as isize) as i32 == ',' as i32
                || *str.offset(i as isize) as i32 == ')' as i32
            {
                end = i;
                length = end - start;
                holder = malloc((length as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64))
                    as *mut i8;
                j = 0;
                while j < length - 1 {
                    *holder.offset(j as isize) = *str.offset((start + j + 1i32) as isize);
                    j += 1;
                    j;
                }
                *holder.offset(j as isize) = 0;
                if *str.offset(i as isize) as i32 == ',' as i32 {
                    start = i;
                    c.x = atof(holder);
                } else {
                    c.y = atof(holder);
                }
            }
            i += 1;
            i;
        }
        return c;
    }
}

#[no_mangle]
pub extern "C" fn intersectionPoint(
    mut a1: point,
    mut a2: point,
    mut b1: point,
    mut b2: point,
) -> point {
    let mut c: point = point { x: 0., y: 0. };
    let mut slopeA: f64 = lineSlope(a1, a2);
    let mut slopeB: f64 = lineSlope(b1, b2);
    if slopeA == slopeB {
        c.x = ::core::f32::NAN as f64;
        c.y = ::core::f32::NAN as f64;
    } else if slopeA.is_nan() as i32 != 0 && slopeB.is_nan() as i32 == 0 {
        c.x = a1.x;
        c.y = (a1.x - b1.x) * slopeB + b1.y;
    } else if slopeB.is_nan() as i32 != 0 && slopeA.is_nan() as i32 == 0 {
        c.x = b1.x;
        c.y = (b1.x - a1.x) * slopeA + a1.y;
    } else {
        c.x = (slopeA * a1.x - slopeB * b1.x + b1.y - a1.y) / (slopeA - slopeB);
        c.y = slopeB * (c.x - b1.x) + b1.y;
    }
    return c;
}

fn main_0(mut argC: i32, mut argV: *mut *mut i8) -> i32 {
    unsafe {
        let mut c: point = point { x: 0., y: 0. };
        if argC < 5 {
            print!(
                "Usage : {} <four points specified as (x,y) separated by a space>",
                build_str_from_raw_ptr(*argV.offset(0 as isize) as *mut u8)
            );
        } else {
            c = intersectionPoint(
                extractPoint(*argV.offset(1 as isize)),
                extractPoint(*argV.offset(2 as isize)),
                extractPoint(*argV.offset(3 as isize)),
                extractPoint(*argV.offset(4 as isize)),
            );
            if (c.x).is_nan() as i32 != 0 {
                print!("The lines do not intersect, they are either parallel or co-incident.");
            } else {
                print!("Point of intersection : ({},{})", c.x, c.y);
            }
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
