#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub type mat = *mut *mut libc::c_double;
#[no_mangle]
pub unsafe extern "C" fn mat_zero(mut x: mat, mut n: libc::c_int) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < n {
            *(*x.offset(i as isize))
                .offset(j as isize) = 0 as libc::c_int as libc::c_double;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mat_new(mut n: libc::c_int) -> mat {
    let mut x: mat = malloc(
        (::core::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as mat;
    let ref mut fresh0 = *x.offset(0 as libc::c_int as isize);
    *fresh0 = malloc(
        (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let ref mut fresh1 = *x.offset(i as isize);
        *fresh1 = (*x.offset(0 as libc::c_int as isize)).offset((n * i) as isize);
        i += 1;
        i;
    }
    mat_zero(x, n);
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn mat_copy(mut s: *mut libc::c_void, mut n: libc::c_int) -> mat {
    let mut x: mat = mat_new(n);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < n {
            let vla = n as usize;
            *(*x.offset(i as isize))
                .offset(
                    j as isize,
                ) = *(s as *mut libc::c_double)
                .offset(i as isize * vla as isize)
                .offset(j as isize);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn mat_del(mut x: mat) {
    free(*x.offset(0 as libc::c_int as isize) as *mut libc::c_void);
    free(x as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mat_show(
    mut x: mat,
    mut fmt: *mut libc::c_char,
    mut n: libc::c_int,
) {
    if fmt.is_null() {
        fmt = b"%8.4g\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        printf(
            if i != 0 {
                b"      \0" as *const u8 as *const libc::c_char
            } else {
                b" [ \0" as *const u8 as *const libc::c_char
            },
        );
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < n {
            printf(fmt, *(*x.offset(i as isize)).offset(j as isize));
            printf(
                if j < n - 1 as libc::c_int {
                    b"  \0" as *const u8 as *const libc::c_char
                } else if i == n - 1 as libc::c_int {
                    b" ]\n\0" as *const u8 as *const libc::c_char
                } else {
                    b"\n\0" as *const u8 as *const libc::c_char
                },
            );
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mat_mul(mut a: mat, mut b: mat, mut n: libc::c_int) -> mat {
    let mut c: mat = 0 as *mut *mut libc::c_double;
    c = mat_new(n);
    c = c;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < n {
            let mut k: libc::c_int = 0 as libc::c_int;
            while k < n {
                *(*c.offset(i as isize)).offset(j as isize)
                    += *(*a.offset(i as isize)).offset(k as isize)
                        * *(*b.offset(k as isize)).offset(j as isize);
                k += 1;
                k;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn mat_pivot(mut a: mat, mut p: mat, mut n: libc::c_int) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < n {
            *(*p.offset(i as isize))
                .offset(j as isize) = (i == j) as libc::c_int as libc::c_double;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < n {
        let mut max_j: libc::c_int = i_0;
        let mut j_0: libc::c_int = i_0;
        while j_0 < n {
            if fabs(*(*a.offset(j_0 as isize)).offset(i_0 as isize))
                > fabs(*(*a.offset(max_j as isize)).offset(i_0 as isize))
            {
                max_j = j_0;
            }
            j_0 += 1;
            j_0;
        }
        if max_j != i_0 {
            let mut k: libc::c_int = 0 as libc::c_int;
            while k < n {
                let mut tmp: libc::c_double = *(*p.offset(i_0 as isize))
                    .offset(k as isize);
                *(*p.offset(i_0 as isize))
                    .offset(
                        k as isize,
                    ) = *(*p.offset(max_j as isize)).offset(k as isize);
                *(*p.offset(max_j as isize)).offset(k as isize) = tmp;
                k += 1;
                k;
            }
        }
        i_0 += 1;
        i_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mat_LU(
    mut A: mat,
    mut L: mat,
    mut U: mat,
    mut P: mat,
    mut n: libc::c_int,
) {
    mat_zero(L, n);
    mat_zero(U, n);
    mat_pivot(A, P, n);
    let mut Aprime: mat = mat_mul(P, A, n);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        *(*L.offset(i as isize)).offset(i as isize) = 1 as libc::c_int as libc::c_double;
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < n {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < n {
            let mut s: libc::c_double = 0.;
            if j <= i_0 {
                s = 0 as libc::c_int as libc::c_double;
                let mut k: libc::c_int = 0 as libc::c_int;
                while k < j {
                    s
                        += *(*L.offset(j as isize)).offset(k as isize)
                            * *(*U.offset(k as isize)).offset(i_0 as isize);
                    k += 1;
                    k;
                }
                *(*U.offset(j as isize))
                    .offset(
                        i_0 as isize,
                    ) = *(*Aprime.offset(j as isize)).offset(i_0 as isize) - s;
            }
            if j >= i_0 {
                s = 0 as libc::c_int as libc::c_double;
                let mut k_0: libc::c_int = 0 as libc::c_int;
                while k_0 < i_0 {
                    s
                        += *(*L.offset(j as isize)).offset(k_0 as isize)
                            * *(*U.offset(k_0 as isize)).offset(i_0 as isize);
                    k_0 += 1;
                    k_0;
                }
                *(*L.offset(j as isize))
                    .offset(
                        i_0 as isize,
                    ) = (*(*Aprime.offset(j as isize)).offset(i_0 as isize) - s)
                    / *(*U.offset(i_0 as isize)).offset(i_0 as isize);
            }
            j += 1;
            j;
        }
        i_0 += 1;
        i_0;
    }
    mat_del(Aprime);
}
#[no_mangle]
pub static mut A3: [[libc::c_double; 3]; 3] = [
    [
        1 as libc::c_int as libc::c_double,
        3 as libc::c_int as libc::c_double,
        5 as libc::c_int as libc::c_double,
    ],
    [
        2 as libc::c_int as libc::c_double,
        4 as libc::c_int as libc::c_double,
        7 as libc::c_int as libc::c_double,
    ],
    [
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
    ],
];
#[no_mangle]
pub static mut A4: [[libc::c_double; 4]; 4] = [
    [
        11 as libc::c_int as libc::c_double,
        9 as libc::c_int as libc::c_double,
        24 as libc::c_int as libc::c_double,
        2 as libc::c_int as libc::c_double,
    ],
    [
        1 as libc::c_int as libc::c_double,
        5 as libc::c_int as libc::c_double,
        2 as libc::c_int as libc::c_double,
        6 as libc::c_int as libc::c_double,
    ],
    [
        3 as libc::c_int as libc::c_double,
        17 as libc::c_int as libc::c_double,
        18 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
    ],
    [
        2 as libc::c_int as libc::c_double,
        5 as libc::c_int as libc::c_double,
        7 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
    ],
];
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 3 as libc::c_int;
    let mut A: mat = 0 as *mut *mut libc::c_double;
    let mut L: mat = 0 as *mut *mut libc::c_double;
    let mut P: mat = 0 as *mut *mut libc::c_double;
    let mut U: mat = 0 as *mut *mut libc::c_double;
    L = mat_new(n);
    P = mat_new(n);
    U = mat_new(n);
    A = mat_copy(A3.as_mut_ptr() as *mut libc::c_void, n);
    mat_LU(A, L, U, P, n);
    printf(b"A =\0" as *const u8 as *const libc::c_char);
    mat_show(A, 0 as *mut libc::c_char, n);
    printf(b"L =\0" as *const u8 as *const libc::c_char);
    mat_show(L, 0 as *mut libc::c_char, n);
    printf(b"U =\0" as *const u8 as *const libc::c_char);
    mat_show(U, 0 as *mut libc::c_char, n);
    printf(b"P =\0" as *const u8 as *const libc::c_char);
    mat_show(P, 0 as *mut libc::c_char, n);
    mat_del(A);
    mat_del(L);
    mat_del(U);
    mat_del(P);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    n = 4 as libc::c_int;
    L = mat_new(n);
    P = mat_new(n);
    U = mat_new(n);
    A = mat_copy(A4.as_mut_ptr() as *mut libc::c_void, n);
    mat_LU(A, L, U, P, n);
    printf(b"A =\0" as *const u8 as *const libc::c_char);
    mat_show(A, 0 as *mut libc::c_char, n);
    printf(b"L =\0" as *const u8 as *const libc::c_char);
    mat_show(L, 0 as *mut libc::c_char, n);
    printf(b"U =\0" as *const u8 as *const libc::c_char);
    mat_show(U, 0 as *mut libc::c_char, n);
    printf(b"P =\0" as *const u8 as *const libc::c_char);
    mat_show(P, 0 as *mut libc::c_char, n);
    mat_del(A);
    mat_del(L);
    mat_del(U);
    mat_del(P);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
