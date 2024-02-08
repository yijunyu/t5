#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point_t {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub group: libc::c_int,
}
pub type point = *mut point_t;
#[no_mangle]
pub unsafe extern "C" fn randf(mut m: libc::c_double) -> libc::c_double {
    return m * rand() as libc::c_double
        / (2147483647 as libc::c_int as libc::c_double - 1.0f64);
}
#[no_mangle]
pub unsafe extern "C" fn gen_xy(
    mut count: libc::c_int,
    mut radius: libc::c_double,
) -> point {
    let mut ang: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    let mut p: point = 0 as *mut point_t;
    let mut pt: point = malloc(
        (::core::mem::size_of::<point_t>() as libc::c_ulong)
            .wrapping_mul(count as libc::c_ulong),
    ) as point;
    p = pt.offset(count as isize);
    loop {
        let fresh0 = p;
        p = p.offset(-1);
        if !(fresh0 > pt) {
            break;
        }
        ang = randf(2 as libc::c_int as libc::c_double * 3.14159265358979323846f64);
        r = randf(radius);
        (*p).x = r * cos(ang);
        (*p).y = r * sin(ang);
    }
    return pt;
}
#[no_mangle]
pub unsafe extern "C" fn dist2(mut a: point, mut b: point) -> libc::c_double {
    let mut x: libc::c_double = (*a).x - (*b).x;
    let mut y: libc::c_double = (*a).y - (*b).y;
    return x * x + y * y;
}
#[no_mangle]
pub unsafe extern "C" fn nearest(
    mut pt: point,
    mut cent: point,
    mut n_cluster: libc::c_int,
    mut d2: *mut libc::c_double,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut min_i: libc::c_int = 0;
    let mut c: point = 0 as *mut point_t;
    let mut d: libc::c_double = 0.;
    let mut min_d: libc::c_double = 0.;
    c = cent;
    i = 0 as libc::c_int;
    while i < n_cluster {
        min_d = ::core::f64::INFINITY;
        min_i = (*pt).group;
        c = cent;
        i = 0 as libc::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn kpp(
    mut pts: point,
    mut len: libc::c_int,
    mut cent: point,
    mut n_cent: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n_cluster: libc::c_int = 0;
    let mut sum: libc::c_double = 0.;
    let mut d: *mut libc::c_double = malloc(
        (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(len as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut p: point = 0 as *mut point_t;
    let mut c: point = 0 as *mut point_t;
    *cent.offset(0 as libc::c_int as isize) = *pts.offset((rand() % len) as isize);
    n_cluster = 1 as libc::c_int;
    while n_cluster < n_cent {
        sum = 0 as libc::c_int as libc::c_double;
        j = 0 as libc::c_int;
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
        j = 0 as libc::c_int;
        p = pts;
        while j < len {
            sum -= *d.offset(j as isize);
            if sum > 0 as libc::c_int as libc::c_double {
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
    j = 0 as libc::c_int;
    p = pts;
    while j < len {
        (*p).group = nearest(p, cent, n_cluster, 0 as *mut libc::c_double);
        j += 1;
        j;
        p = p.offset(1);
        p;
    }
    free(d as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn lloyd(
    mut pts: point,
    mut len: libc::c_int,
    mut n_cluster: libc::c_int,
) -> point {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut min_i: libc::c_int = 0;
    let mut changed: libc::c_int = 0;
    let mut cent: point = malloc(
        (::core::mem::size_of::<point_t>() as libc::c_ulong)
            .wrapping_mul(n_cluster as libc::c_ulong),
    ) as point;
    let mut p: point = 0 as *mut point_t;
    let mut c: point = 0 as *mut point_t;
    kpp(pts, len, cent, n_cluster);
    loop {
        c = cent;
        i = 0 as libc::c_int;
        while i < n_cluster {
            (*c).group = 0 as libc::c_int;
            (*c).y = 0 as libc::c_int as libc::c_double;
            (*c).x = (*c).y;
            i += 1;
            i;
            c = c.offset(1);
            c;
        }
        j = 0 as libc::c_int;
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
        i = 0 as libc::c_int;
        while i < n_cluster {
            (*c).x /= (*c).group as libc::c_double;
            (*c).y /= (*c).group as libc::c_double;
            i += 1;
            i;
            c = c.offset(1);
            c;
        }
        changed = 0 as libc::c_int;
        j = 0 as libc::c_int;
        p = pts;
        while j < len {
            min_i = nearest(p, cent, n_cluster, 0 as *mut libc::c_double);
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
        if !(changed > len >> 10 as libc::c_int) {
            break;
        }
    }
    c = cent;
    i = 0 as libc::c_int;
    while i < n_cluster {
        (*c).group = i;
        i += 1;
        i;
        c = c.offset(1);
        c;
    }
    return cent;
}
#[no_mangle]
pub unsafe extern "C" fn print_eps(
    mut pts: point,
    mut len: libc::c_int,
    mut cent: point,
    mut n_cluster: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: point = 0 as *mut point_t;
    let mut c: point = 0 as *mut point_t;
    let mut min_x: libc::c_double = 0.;
    let mut max_x: libc::c_double = 0.;
    let mut min_y: libc::c_double = 0.;
    let mut max_y: libc::c_double = 0.;
    let mut scale: libc::c_double = 0.;
    let mut cx: libc::c_double = 0.;
    let mut cy: libc::c_double = 0.;
    let mut colors: *mut libc::c_double = malloc(
        (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(n_cluster as libc::c_ulong)
            .wrapping_mul(3 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_double;
    c = cent;
    i = 0 as libc::c_int;
    while i < n_cluster {
        *colors
            .offset(
                (3 as libc::c_int * i + 0 as libc::c_int) as isize,
            ) = (3 as libc::c_int * (i + 1 as libc::c_int) % 11 as libc::c_int)
            as libc::c_double / 11.0f64;
        *colors
            .offset(
                (3 as libc::c_int * i + 1 as libc::c_int) as isize,
            ) = (7 as libc::c_int * i % 11 as libc::c_int) as libc::c_double / 11.0f64;
        *colors
            .offset(
                (3 as libc::c_int * i + 2 as libc::c_int) as isize,
            ) = (9 as libc::c_int * i % 11 as libc::c_int) as libc::c_double / 11.0f64;
        i += 1;
        i;
        c = c.offset(1);
        c;
    }
    min_y = ::core::f64::INFINITY;
    min_x = min_y;
    max_y = -min_x;
    max_x = max_y;
    j = 0 as libc::c_int;
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
    scale = 400 as libc::c_int as libc::c_double / (max_x - min_x);
    if scale > 400 as libc::c_int as libc::c_double / (max_y - min_y) {
        scale = 400 as libc::c_int as libc::c_double / (max_y - min_y);
    }
    cx = (max_x + min_x) / 2 as libc::c_int as libc::c_double;
    cy = (max_y + min_y) / 2 as libc::c_int as libc::c_double;
    printf(
        b"%%!PS-Adobe-3.0\n%%%%BoundingBox: -5 -5 %d %d\n\0" as *const u8
            as *const libc::c_char,
        400 as libc::c_int + 10 as libc::c_int,
        400 as libc::c_int + 10 as libc::c_int,
    );
    printf(
        b"/l {rlineto} def /m {rmoveto} def\n/c { .25 sub exch .25 sub exch .5 0 360 arc fill } def\n/s { moveto -2 0 m 2 2 l 2 -2 l -2 -2 l closepath \tgsave 1 setgray fill grestore gsave 3 setlinewidth 1 setgray stroke grestore 0 setgray stroke }def\n\0"
            as *const u8 as *const libc::c_char,
    );
    c = cent;
    i = 0 as libc::c_int;
    while i < n_cluster {
        printf(
            b"%g %g %g setrgbcolor\n\0" as *const u8 as *const libc::c_char,
            *colors.offset((3 as libc::c_int * i) as isize),
            *colors.offset((3 as libc::c_int * i + 1 as libc::c_int) as isize),
            *colors.offset((3 as libc::c_int * i + 2 as libc::c_int) as isize),
        );
        j = 0 as libc::c_int;
        p = pts;
        while j < len {
            if !((*p).group != i) {
                printf(
                    b"%.3f %.3f c\n\0" as *const u8 as *const libc::c_char,
                    ((*p).x - cx) * scale
                        + (400 as libc::c_int / 2 as libc::c_int) as libc::c_double,
                    ((*p).y - cy) * scale
                        + (400 as libc::c_int / 2 as libc::c_int) as libc::c_double,
                );
            }
            j += 1;
            j;
            p = p.offset(1);
            p;
        }
        printf(
            b"\n0 setgray %g %g s\n\0" as *const u8 as *const libc::c_char,
            ((*c).x - cx) * scale
                + (400 as libc::c_int / 2 as libc::c_int) as libc::c_double,
            ((*c).y - cy) * scale
                + (400 as libc::c_int / 2 as libc::c_int) as libc::c_double,
        );
        i += 1;
        i;
        c = c.offset(1);
        c;
    }
    printf(b"\n%%%%EOF\0" as *const u8 as *const libc::c_char);
    free(colors as *mut libc::c_void);
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut v: point = gen_xy(
        100000 as libc::c_int,
        10 as libc::c_int as libc::c_double,
    );
    let mut c: point = lloyd(v, 100000 as libc::c_int, 11 as libc::c_int);
    print_eps(v, 100000 as libc::c_int, c, 11 as libc::c_int);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
