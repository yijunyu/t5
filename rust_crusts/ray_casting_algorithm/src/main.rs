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
    fn rand() -> i32;
    fn sqrt(_: f64) -> f64;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vec {
    pub x: f64,
    pub y: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct polygon_t {
    pub n: i32,
    pub v: *mut vec,
}
pub type polygon = *mut polygon_t;
#[no_mangle]
pub extern "C" fn vsub(mut a: vec, mut b: vec) -> vec {
    let mut c: vec = vec { x: 0., y: 0. };
    c.x = a.x - b.x;
    c.y = a.y - b.y;
    return c;
}

#[no_mangle]
pub extern "C" fn vadd(mut a: vec, mut b: vec) -> vec {
    let mut c: vec = vec { x: 0., y: 0. };
    c.x = a.x + b.x;
    c.y = a.y + b.y;
    return c;
}

#[no_mangle]
pub extern "C" fn vdot(mut a: vec, mut b: vec) -> f64 {
    return a.x * b.x + a.y * b.y;
}

#[no_mangle]
pub extern "C" fn vcross(mut a: vec, mut b: vec) -> f64 {
    return a.x * b.y - a.y * b.x;
}

#[no_mangle]
pub extern "C" fn vmadd(mut a: vec, mut s: f64, mut b: vec) -> vec {
    let mut c: vec = vec { x: 0., y: 0. };
    c.x = a.x + s * b.x;
    c.y = a.y + s * b.y;
    return c;
}

#[no_mangle]
pub extern "C" fn intersect(
    mut x0: vec,
    mut x1: vec,
    mut y0: vec,
    mut y1: vec,
    mut tol: f64,
    mut sect: *mut vec,
) -> i32 {
    unsafe {
        let mut dx: vec = vsub(x1, x0);
        let mut dy: vec = vsub(y1, y0);
        let mut d: f64 = vcross(dy, dx);
        let mut a: f64 = 0.;
        if d == 0. {
            return 0;
        }
        a = (vcross(x0, dx) - vcross(y0, dx)) / d;
        if !sect.is_null() {
            *sect = vmadd(y0, a, dy);
        }
        if a < -tol || a > 1 as f64 + tol {
            return -1;
        }
        if a < tol || a > 1 as f64 - tol {
            return 0;
        }
        a = (vcross(x0, dy) - vcross(y0, dy)) / d;
        if a < 0 as f64 || a > 1 as f64 {
            return -1;
        }
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn dist(mut x: vec, mut y0: vec, mut y1: vec, mut tol: f64) -> f64 {
    let mut dy: vec = vsub(y1, y0);
    let mut x1: vec = vec { x: 0., y: 0. };
    let mut s: vec = vec { x: 0., y: 0. };
    let mut r: i32 = 0;
    x1.x = x.x + dy.y;
    x1.y = x.y - dy.x;
    r = intersect(x, x1, y0, y1, tol, &mut s);
    if r == -1 {
        return ::core::f64::INFINITY;
    }
    s = vsub(s, x);
    unsafe {
        return sqrt(vdot(s, s));
    }
}

#[no_mangle]
pub extern "C" fn inside(mut v: vec, mut p: polygon, mut tol: f64) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut k: i32 = 0;
        let mut crosses: i32 = 0;
        let mut intersectResult: i32 = 0;
        let mut pv: *mut vec = 0 as *mut vec;
        let mut min_x: f64 = 0.;
        let mut max_x: f64 = 0.;
        let mut min_y: f64 = 0.;
        let mut max_y: f64 = 0.;
        i = 0;
        while i < (*p).n {
            k = (i + 1) % (*p).n;
            min_x = dist(
                v,
                *((*p).v).offset(i as isize),
                *((*p).v).offset(k as isize),
                tol,
            );
            if min_x < tol {
                return 0;
            }
            i += 1;
            i;
        }
        max_x = (*((*p).v).offset(0 as isize)).x;
        min_x = max_x;
        max_y = (*((*p).v).offset(1 as isize)).y;
        min_y = max_y;
        i = 0;
        pv = (*p).v;
        while i < (*p).n {
            if (*pv).x > max_x {
                max_x = (*pv).x;
            }
            if (*pv).x < min_x {
                min_x = (*pv).x;
            }
            if (*pv).y > max_y {
                max_y = (*pv).y;
            }
            if (*pv).y < min_y {
                min_y = (*pv).y;
            }
            i += 1;
            i;
            pv = pv.offset(1);
            pv;
        }
        if v.x < min_x || v.x > max_x || v.y < min_y || v.y > max_y {
            return -1;
        }
        max_x -= min_x;
        max_x *= 2 as f64;
        max_y -= min_y;
        max_y *= 2 as f64;
        max_x += max_y;
        let mut e: vec = vec { x: 0., y: 0. };
        loop {
            crosses = 0;
            e.x = v.x + (1 as f64 + rand() as f64 / (2147483647 as f64 + 1.0f64)) * max_x;
            e.y = v.y + (1 as f64 + rand() as f64 / (2147483647 as f64 + 1.0f64)) * max_x;
            i = 0;
            while i < (*p).n {
                k = (i + 1) % (*p).n;
                intersectResult = intersect(
                    v,
                    e,
                    *((*p).v).offset(i as isize),
                    *((*p).v).offset(k as isize),
                    tol,
                    0 as *mut vec,
                );
                if intersectResult == 0 {
                    break;
                }
                if intersectResult == 1 {
                    crosses += 1;
                    crosses;
                }
                i += 1;
                i;
            }
            if i == (*p).n {
                break;
            }
        }
        return if crosses & 1 != 0 { 1 } else { -1 };
    }
}

fn main_0() -> i32 {
    let mut vsq: [vec; 8] = [
        {
            let mut init = vec {
                x: 0 as f64,
                y: 0 as f64,
            };
            init
        },
        {
            let mut init = vec {
                x: 10 as f64,
                y: 0 as f64,
            };
            init
        },
        {
            let mut init = vec {
                x: 10 as f64,
                y: 10 as f64,
            };
            init
        },
        {
            let mut init = vec {
                x: 0 as f64,
                y: 10 as f64,
            };
            init
        },
        {
            let mut init = vec {
                x: 2.5f64,
                y: 2.5f64,
            };
            init
        },
        {
            let mut init = vec {
                x: 7.5f64,
                y: 0.1f64,
            };
            init
        },
        {
            let mut init = vec {
                x: 7.5f64,
                y: 7.5f64,
            };
            init
        },
        {
            let mut init = vec {
                x: 2.5f64,
                y: 7.5f64,
            };
            init
        },
    ];
    let mut sq: polygon_t = {
        let mut init = polygon_t {
            n: 4,
            v: vsq.as_mut_ptr(),
        };
        init
    };
    let mut sq_hole: polygon_t = {
        let mut init = polygon_t {
            n: 8,
            v: vsq.as_mut_ptr(),
        };
        init
    };
    let mut c: vec = {
        let mut init = vec {
            x: 10 as f64,
            y: 5 as f64,
        };
        init
    };
    let mut d: vec = {
        let mut init = vec {
            x: 5 as f64,
            y: 5 as f64,
        };
        init
    };
    print!("{}\n", inside(c, &mut sq, 1e-10f64));
    print!("{}\n", inside(c, &mut sq_hole, 1e-10f64));
    print!("{}\n", inside(d, &mut sq, 1e-10f64));
    print!("{}\n", inside(d, &mut sq_hole, 1e-10f64));
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
