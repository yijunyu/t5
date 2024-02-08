#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar(__c: libc::c_int) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct matrix_t {
    pub h: libc::c_int,
    pub w: libc::c_int,
    pub x: *mut libc::c_double,
}
pub type matrix = *mut matrix_t;
#[no_mangle]
pub unsafe extern "C" fn dot(
    mut a: *mut libc::c_double,
    mut b: *mut libc::c_double,
    mut len: libc::c_int,
    mut step: libc::c_int,
) -> libc::c_double {
    let mut r: libc::c_double = 0 as libc::c_int as libc::c_double;
    loop {
        let fresh0 = len;
        len = len - 1;
        if !(fresh0 != 0) {
            break;
        }
        let fresh1 = a;
        a = a.offset(1);
        r += *fresh1 * *b;
        b = b.offset(step as isize);
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn mat_new(mut h: libc::c_int, mut w: libc::c_int) -> matrix {
    let mut r: matrix = malloc(
        (::core::mem::size_of::<matrix_t>() as libc::c_ulong)
            .wrapping_add(
                (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul(w as libc::c_ulong)
                    .wrapping_mul(h as libc::c_ulong),
            ),
    ) as matrix;
    (*r).h = h;
    (*r).w = w;
    (*r).x = r.offset(1 as libc::c_int as isize) as *mut libc::c_double;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn mat_mul(mut a: matrix, mut b: matrix) -> matrix {
    let mut r: matrix = 0 as *mut matrix_t;
    let mut p: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut pa: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (*a).w != (*b).h {
        return 0 as matrix;
    }
    r = mat_new((*a).h, (*b).w);
    p = (*r).x;
    pa = (*a).x;
    i = 0 as libc::c_int;
    while i < (*a).h {
        j = 0 as libc::c_int;
        while j < (*b).w {
            let fresh2 = p;
            p = p.offset(1);
            *fresh2 = dot(pa, ((*b).x).offset(j as isize), (*a).w, (*b).w);
            j += 1;
            j;
        }
        i += 1;
        i;
        pa = pa.offset((*a).w as isize);
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn mat_show(mut a: matrix) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: *mut libc::c_double = (*a).x;
    i = 0 as libc::c_int;
    while i < (*a).h {
        j = 0 as libc::c_int;
        while j < (*a).w {
            let fresh3 = p;
            p = p.offset(1);
            printf(b"\t%7.3f\0" as *const u8 as *const libc::c_char, *fresh3);
            j += 1;
            j;
        }
        i += 1;
        i;
        putchar('\n' as i32);
    }
    putchar('\n' as i32);
}
unsafe fn main_0() -> libc::c_int {
    let mut da: [libc::c_double; 16] = [
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        2 as libc::c_int as libc::c_double,
        4 as libc::c_int as libc::c_double,
        8 as libc::c_int as libc::c_double,
        16 as libc::c_int as libc::c_double,
        3 as libc::c_int as libc::c_double,
        9 as libc::c_int as libc::c_double,
        27 as libc::c_int as libc::c_double,
        81 as libc::c_int as libc::c_double,
        4 as libc::c_int as libc::c_double,
        16 as libc::c_int as libc::c_double,
        64 as libc::c_int as libc::c_double,
        256 as libc::c_int as libc::c_double,
    ];
    let mut db: [libc::c_double; 12] = [
        4.0f64,
        -3.0f64,
        4.0f64 / 3 as libc::c_int as libc::c_double,
        -13.0f64 / 3 as libc::c_int as libc::c_double,
        19.0f64 / 4 as libc::c_int as libc::c_double,
        -7.0f64 / 3 as libc::c_int as libc::c_double,
        3.0f64 / 2 as libc::c_int as libc::c_double,
        -2.0f64,
        7.0f64 / 6 as libc::c_int as libc::c_double,
        -1.0f64 / 6 as libc::c_int as libc::c_double,
        1.0f64 / 4 as libc::c_int as libc::c_double,
        -1.0f64 / 6 as libc::c_int as libc::c_double,
    ];
    let mut a: matrix_t = {
        let mut init = matrix_t {
            h: 4 as libc::c_int,
            w: 4 as libc::c_int,
            x: da.as_mut_ptr(),
        };
        init
    };
    let mut b: matrix_t = {
        let mut init = matrix_t {
            h: 4 as libc::c_int,
            w: 3 as libc::c_int,
            x: db.as_mut_ptr(),
        };
        init
    };
    let mut c: matrix = mat_mul(&mut a, &mut b);
    mat_show(c);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
