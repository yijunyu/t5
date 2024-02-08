#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn atof(__nptr: *const libc::c_char) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn lineSlope(mut a: point, mut b: point) -> libc::c_double {
    if a.x - b.x == 0.0f64 {
        return ::core::f32::NAN as libc::c_double
    } else {
        return (a.y - b.y) / (a.x - b.x)
    };
}
#[no_mangle]
pub unsafe extern "C" fn extractPoint(mut str: *mut libc::c_char) -> point {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut holder: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: point = point { x: 0., y: 0. };
    i = 0 as libc::c_int;
    while *str.offset(i as isize) as libc::c_int != 0 as libc::c_int {
        if *str.offset(i as isize) as libc::c_int == '(' as i32 {
            start = i;
        }
        if *str.offset(i as isize) as libc::c_int == ',' as i32
            || *str.offset(i as isize) as libc::c_int == ')' as i32
        {
            end = i;
            length = end - start;
            holder = malloc(
                (length as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_char;
            j = 0 as libc::c_int;
            while j < length - 1 as libc::c_int {
                *holder
                    .offset(
                        j as isize,
                    ) = *str.offset((start + j + 1 as libc::c_int) as isize);
                j += 1;
                j;
            }
            *holder.offset(j as isize) = 0 as libc::c_int as libc::c_char;
            if *str.offset(i as isize) as libc::c_int == ',' as i32 {
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
#[no_mangle]
pub unsafe extern "C" fn intersectionPoint(
    mut a1: point,
    mut a2: point,
    mut b1: point,
    mut b2: point,
) -> point {
    let mut c: point = point { x: 0., y: 0. };
    let mut slopeA: libc::c_double = lineSlope(a1, a2);
    let mut slopeB: libc::c_double = lineSlope(b1, b2);
    if slopeA == slopeB {
        c.x = ::core::f32::NAN as libc::c_double;
        c.y = ::core::f32::NAN as libc::c_double;
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
unsafe fn main_0(
    mut argC: libc::c_int,
    mut argV: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: point = point { x: 0., y: 0. };
    if argC < 5 as libc::c_int {
        printf(
            b"Usage : %s <four points specified as (x,y) separated by a space>\0"
                as *const u8 as *const libc::c_char,
            *argV.offset(0 as libc::c_int as isize),
        );
    } else {
        c = intersectionPoint(
            extractPoint(*argV.offset(1 as libc::c_int as isize)),
            extractPoint(*argV.offset(2 as libc::c_int as isize)),
            extractPoint(*argV.offset(3 as libc::c_int as isize)),
            extractPoint(*argV.offset(4 as libc::c_int as isize)),
        );
        if (c.x).is_nan() as i32 != 0 {
            printf(
                b"The lines do not intersect, they are either parallel or co-incident.\0"
                    as *const u8 as *const libc::c_char,
            );
        } else {
            printf(
                b"Point of intersection : (%lf,%lf)\0" as *const u8
                    as *const libc::c_char,
                c.x,
                c.y,
            );
        }
    }
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
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
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
