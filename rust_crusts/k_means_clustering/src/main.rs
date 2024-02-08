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
    fn printf(_: *const i8, _: ...) -> i32;
    fn rand() -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn cos(_: f64) -> f64;
    fn sin(_: f64) -> f64;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point_t {
    pub x: f64,
    pub y: f64,
    pub group: i32,
}
pub type point = *mut point_t;
#[no_mangle]
pub extern "C" fn randf(mut m: f64) -> f64 {
    unsafe {
        return m * rand() as f64 / (2147483647 as f64 - 1.0f64);
    }
}

#[no_mangle]
pub extern "C" fn gen_xy(mut count: i32, mut radius: f64) -> point {
    let mut ang: f64 = 0.;
    let mut r: f64 = 0.;
    let mut p: point = 0 as *mut point_t;
    unsafe {
        let mut pt: point =
            malloc((::core::mem::size_of::<point_t>() as u64).wrapping_mul(count as u64)) as point;
        p = pt.offset(count as isize);
        loop {
            let fresh0 = p;
            p = p.offset(-1);
            if !(fresh0 > pt) {
                break;
            }
            ang = randf(2 as f64 * 3.14159265358979323846f64);
            r = randf(radius);
            (*p).x = r * cos(ang);
            (*p).y = r * sin(ang);
        }
        return pt;
    }
}

#[no_mangle]
pub extern "C" fn dist2(mut a: point, mut b: point) -> f64 {
    let mut x: f64 = (*a).x - (*b).x;
    let mut y: f64 = (*a).y - (*b).y;
    return x * x + y * y;
}

#[no_mangle]
pub extern "C" fn nearest(
    mut pt: point,
    mut cent: point,
    mut n_cluster: i32,
    mut d2: *mut f64,
) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut min_i: i32 = 0;
        let mut c: point = 0 as *mut point_t;
        let mut d: f64 = 0.;
        let mut min_d: f64 = 0.;
        c = cent;
        i = 0;
        while i < n_cluster {
            min_d = ::core::f64::INFINITY;
            min_i = (*pt).group;
            c = cent;
            i = 0;
            while i < n_cluster {
                d = dist2(c, pt);
                if min_d > d {
                    min_d = d;
                    min_i = i;
                }
                i += 1;
                i;
                c = c.offset(1);
                c;
            }
            i += 1;
            i;
            c = c.offset(1);
            c;
        }
        if !d2.is_null() {
            *d2 = min_d;
        }
        return min_i;
    }
}

#[no_mangle]
pub extern "C" fn kpp(mut pts: point, mut len: i32, mut cent: point, mut n_cent: i32) {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut n_cluster: i32 = 0;
        let mut sum: f64 = 0.;
        let mut d: *mut f64 =
            malloc((::core::mem::size_of::<f64>() as u64).wrapping_mul(len as u64)) as *mut f64;
        let mut p: point = 0 as *mut point_t;
        let mut c: point = 0 as *mut point_t;
        *cent.offset(0 as isize) = *pts.offset((rand() % len) as isize);
        n_cluster = 1;
        while n_cluster < n_cent {
            sum = 0 as f64;
            j = 0;
            p = pts;
            while j < len {
                nearest(p, cent, n_cluster, d.offset(j as isize));
                sum += *d.offset(j as isize);
                j += 1;
                j;
                p = p.offset(1);
                p;
            }
            sum = randf(sum);
            j = 0;
            p = pts;
            while j < len {
                sum -= *d.offset(j as isize);
                if sum > 0 as f64 {
                    j += 1;
                    j;
                    p = p.offset(1);
                    p;
                } else {
                    *cent.offset(n_cluster as isize) = *pts.offset(j as isize);
                    break;
                }
            }
            n_cluster += 1;
            n_cluster;
        }
        j = 0;
        p = pts;
        while j < len {
            (*p).group = nearest(p, cent, n_cluster, 0 as *mut f64);
            j += 1;
            j;
            p = p.offset(1);
            p;
        }
        free(d as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn lloyd(mut pts: point, mut len: i32, mut n_cluster: i32) -> point {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut min_i: i32 = 0;
    let mut changed: i32 = 0;
    unsafe {
        let mut cent: point =
            malloc((::core::mem::size_of::<point_t>() as u64).wrapping_mul(n_cluster as u64))
                as point;
        let mut p: point = 0 as *mut point_t;
        let mut c: point = 0 as *mut point_t;
        kpp(pts, len, cent, n_cluster);
        loop {
            c = cent;
            i = 0;
            while i < n_cluster {
                (*c).group = 0;
                (*c).y = 0 as f64;
                (*c).x = (*c).y;
                i += 1;
                i;
                c = c.offset(1);
                c;
            }
            j = 0;
            p = pts;
            while j < len {
                c = cent.offset((*p).group as isize);
                (*c).group += 1;
                (*c).group;
                (*c).x += (*p).x;
                (*c).y += (*p).y;
                j += 1;
                j;
                p = p.offset(1);
                p;
            }
            c = cent;
            i = 0;
            while i < n_cluster {
                (*c).x /= (*c).group as f64;
                (*c).y /= (*c).group as f64;
                i += 1;
                i;
                c = c.offset(1);
                c;
            }
            changed = 0;
            j = 0;
            p = pts;
            while j < len {
                min_i = nearest(p, cent, n_cluster, 0 as *mut f64);
                if min_i != (*p).group {
                    changed += 1;
                    changed;
                    (*p).group = min_i;
                }
                j += 1;
                j;
                p = p.offset(1);
                p;
            }
            if !(changed > len >> 10) {
                break;
            }
        }
        c = cent;
        i = 0;
        while i < n_cluster {
            (*c).group = i;
            i += 1;
            i;
            c = c.offset(1);
            c;
        }
        return cent;
    }
}

#[no_mangle]
pub extern "C" fn print_eps(mut pts: point, mut len: i32, mut cent: point, mut n_cluster: i32) {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut p: point = 0 as *mut point_t;
        let mut c: point = 0 as *mut point_t;
        let mut min_x: f64 = 0.;
        let mut max_x: f64 = 0.;
        let mut min_y: f64 = 0.;
        let mut max_y: f64 = 0.;
        let mut scale: f64 = 0.;
        let mut cx: f64 = 0.;
        let mut cy: f64 = 0.;
        let mut colors: *mut f64 = malloc(
            (::core::mem::size_of::<f64>() as u64)
                .wrapping_mul(n_cluster as u64)
                .wrapping_mul(3),
        ) as *mut f64;
        c = cent;
        i = 0;
        while i < n_cluster {
            *colors.offset((3 * i + 0i32) as isize) = (3 * (i + 1i32) % 11i32) as f64 / 11.0f64;
            *colors.offset((3 * i + 1i32) as isize) = (7 * i % 11i32) as f64 / 11.0f64;
            *colors.offset((3 * i + 2i32) as isize) = (9 * i % 11i32) as f64 / 11.0f64;
            i += 1;
            i;
            c = c.offset(1);
            c;
        }
        min_y = ::core::f64::INFINITY;
        min_x = min_y;
        max_y = -min_x;
        max_x = max_y;
        j = 0;
        p = pts;
        while j < len {
            if max_x < (*p).x {
                max_x = (*p).x;
            }
            if min_x > (*p).x {
                min_x = (*p).x;
            }
            if max_y < (*p).y {
                max_y = (*p).y;
            }
            if min_y > (*p).y {
                min_y = (*p).y;
            }
            j += 1;
            j;
            p = p.offset(1);
            p;
        }
        scale = 400 as f64 / (max_x - min_x);
        if scale > 400 as f64 / (max_y - min_y) {
            scale = 400 as f64 / (max_y - min_y);
        }
        cx = (max_x + min_x) / 2 as f64;
        cy = (max_y + min_y) / 2 as f64;
        print!(
            "%!PS-Adobe-3.0\n%%BoundingBox: -5 -5 {} {}\n",
            400 + 10,
            400 + 10
        );
        printf (
        b"/l {rlineto} def /m {rmoveto} def\n/c { .25 sub exch .25 sub exch .5 0 360 arc fill } def\n/s { moveto -2 0 m 2 2 l 2 -2 l -2 -2 l closepath \tgsave 1 setgray fill grestore gsave 3 setlinewidth 1 setgray stroke grestore 0 setgray stroke }def\n\0"
          as * const u8 as * const i8,);
        c = cent;
        i = 0;
        while i < n_cluster {
            print!(
                "{} {} {} setrgbcolor\n",
                *colors.offset((3 * i) as isize),
                *colors.offset((3 * i + 1i32) as isize),
                *colors.offset((3 * i + 2i32) as isize)
            );
            j = 0;
            p = pts;
            while j < len {
                if !((*p).group != i) {
                    print!(
                        "{:.3} {:.3} c\n",
                        ((*p).x - cx) * scale + (400 / 2i32) as f64,
                        ((*p).y - cy) * scale + (400 / 2i32) as f64
                    );
                }
                j += 1;
                j;
                p = p.offset(1);
                p;
            }
            print!(
                "\n0 setgray {} {} s\n",
                ((*c).x - cx) * scale + (400 / 2i32) as f64,
                ((*c).y - cy) * scale + (400 / 2i32) as f64
            );
            i += 1;
            i;
            c = c.offset(1);
            c;
        }
        printf(b"\n%%%%EOF\0" as *const u8 as *const i8);
        free(colors as *mut libc::c_void);
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut v: point = gen_xy(100000, 10 as f64);
    let mut c: point = lloyd(v, 100000, 11);
    print_eps(v, 100000, c, 11);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
