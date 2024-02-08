#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value)]
use c2rust_out::*;
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn sqrt(_: f64) -> f64;
    fn fabs(_: f64) -> f64;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point_tag {
    pub x: f64,
    pub y: f64,
}
pub type point_t = point_tag;
#[no_mangle]
pub extern "C" fn perpendicular_distance(mut p: point_t, mut p1: point_t, mut p2: point_t) -> f64 {
    let mut dx: f64 = p2.x - p1.x;
    let mut dy: f64 = p2.y - p1.y;
    unsafe {
        let mut d: f64 = sqrt(dx * dx + dy * dy);
        return fabs(p.x * dy - p.y * dx + p2.x * p1.y - p2.y * p1.x) / d;
    }
}

#[no_mangle]
pub extern "C" fn douglas_peucker(
    mut points: *const point_t,
    mut n: u64,
    mut epsilon: f64,
    mut dest: *mut point_t,
    mut destlen: u64,
) -> u64 {
    unsafe {
        if n >= 2 {
        } else {
            __assert_fail(
                b"n >= 2\0" as *const u8 as *const i8,
                b"main.c\0" as *const u8 as *const i8,
                22,
                (*::core::mem::transmute::<&[u8; 75], &[i8; 75]>(
                    b"size_t douglas_peucker(const point_t *, size_t, double, point_t *, size_t)\0",
                ))
                .as_ptr(),
            );
        }
        'c_2158: {
            if n >= 2 {
            } else {
                __assert_fail (b"n >= 2\0" as * const u8 as * const i8, b"main.c\0" as * const u8 as * const i8, 22, (* :: core :: mem :: transmute :: < & [u8; 75], & [i8; 75] > (
                  b"size_t douglas_peucker(const point_t *, size_t, double, point_t *, size_t)\0",)).as_ptr (),);
            }
        };
        if epsilon >= 0 as f64 {
        } else {
            __assert_fail(
                b"epsilon >= 0\0" as *const u8 as *const i8,
                b"main.c\0" as *const u8 as *const i8,
                23,
                (*::core::mem::transmute::<&[u8; 75], &[i8; 75]>(
                    b"size_t douglas_peucker(const point_t *, size_t, double, point_t *, size_t)\0",
                ))
                .as_ptr(),
            );
        }
        'c_2115: {
            if epsilon >= 0 as f64 {
            } else {
                __assert_fail (b"epsilon >= 0\0" as * const u8 as * const i8, b"main.c\0" as * const u8 as * const i8, 23, (* :: core :: mem :: transmute :: < & [u8; 75], & [i8; 75] > (
                  b"size_t douglas_peucker(const point_t *, size_t, double, point_t *, size_t)\0",)).as_ptr (),);
            }
        };
        let mut max_dist: f64 = 0 as f64;
        let mut index: u64 = 0;
        let mut i: u64 = 1;
        while i.wrapping_add(1) < n {
            let mut dist: f64 = perpendicular_distance(
                *points.offset(i as isize),
                *points.offset(0 as isize),
                *points.offset(n.wrapping_sub(1) as isize),
            );
            if dist > max_dist {
                max_dist = dist;
                index = i;
            }
            i = i.wrapping_add(1);
            i;
        }
        if max_dist > epsilon {
            let mut n1: u64 =
                douglas_peucker(points, index.wrapping_add(1), epsilon, dest, destlen);
            if destlen >= n1.wrapping_sub(1) {
                destlen = (destlen as u64).wrapping_sub(n1.wrapping_sub(1)) as u64;
                dest = dest.offset(n1.wrapping_sub(1) as isize);
            } else {
                destlen = 0;
            }
            let mut n2: u64 = douglas_peucker(
                points.offset(index as isize),
                n.wrapping_sub(index),
                epsilon,
                dest,
                destlen,
            );
            return n1.wrapping_add(n2).wrapping_sub(1);
        }
        if destlen >= 2 {
            *dest.offset(0 as isize) = *points.offset(0 as isize);
            *dest.offset(1 as isize) = *points.offset(n.wrapping_sub(1) as isize);
        }
        return 2;
    }
}

#[no_mangle]
pub extern "C" fn print_points(mut points: *const point_t, mut n: u64) {
    unsafe {
        let mut i: u64 = 0;
        while i < n {
            if i > 0 {
                print!(" ");
            }
            print!(
                "({}, {})",
                (*points.offset(i as isize)).x,
                (*points.offset(i as isize)).y
            );
            i = i.wrapping_add(1);
            i;
        }
        print!("\n");
    }
}

fn main_0() -> i32 {
    let mut points: [point_t; 10] = [
        {
            let mut init = point_tag {
                x: 0 as f64,
                y: 0 as f64,
            };
            init
        },
        {
            let mut init = point_tag {
                x: 1 as f64,
                y: 0.1f64,
            };
            init
        },
        {
            let mut init = point_tag {
                x: 2 as f64,
                y: -0.1f64,
            };
            init
        },
        {
            let mut init = point_tag {
                x: 3 as f64,
                y: 5 as f64,
            };
            init
        },
        {
            let mut init = point_tag {
                x: 4 as f64,
                y: 6 as f64,
            };
            init
        },
        {
            let mut init = point_tag {
                x: 5 as f64,
                y: 7 as f64,
            };
            init
        },
        {
            let mut init = point_tag {
                x: 6 as f64,
                y: 8.1f64,
            };
            init
        },
        {
            let mut init = point_tag {
                x: 7 as f64,
                y: 9 as f64,
            };
            init
        },
        {
            let mut init = point_tag {
                x: 8 as f64,
                y: 9 as f64,
            };
            init
        },
        {
            let mut init = point_tag {
                x: 9 as f64,
                y: 9 as f64,
            };
            init
        },
    ];
    let len: u64 = (::core::mem::size_of::<[point_t; 10]>() as u64)
        .wrapping_div(::core::mem::size_of::<point_t>() as u64);
    let vla = len as usize;
    let mut out: Vec<point_t> = ::std::vec::from_elem(point_t { x: 0., y: 0. }, vla);
    let mut n: u64 = douglas_peucker(points.as_mut_ptr(), len, 1.0f64, out.as_mut_ptr(), len);
    print_points(out.as_mut_ptr(), n);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
