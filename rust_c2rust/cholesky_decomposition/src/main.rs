#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn cholesky(
    mut A: *mut libc::c_double,
    mut n: libc::c_int,
) -> *mut libc::c_double {
    let mut L: *mut libc::c_double = calloc(
        (n * n) as libc::c_ulong,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    if L.is_null() {
        exit(1 as libc::c_int);
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < i + 1 as libc::c_int {
            let mut s: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut k: libc::c_int = 0 as libc::c_int;
            while k < j {
                s += *L.offset((i * n + k) as isize) * *L.offset((j * n + k) as isize);
                k += 1;
                k;
            }
            *L
                .offset(
                    (i * n + j) as isize,
                ) = if i == j {
                sqrt(*A.offset((i * n + i) as isize) - s)
            } else {
                1.0f64 / *L.offset((j * n + j) as isize)
                    * (*A.offset((i * n + j) as isize) - s)
            };
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return L;
}
#[no_mangle]
pub unsafe extern "C" fn show_matrix(mut A: *mut libc::c_double, mut n: libc::c_int) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < n {
            printf(
                b"%2.5f \0" as *const u8 as *const libc::c_char,
                *A.offset((i * n + j) as isize),
            );
            j += 1;
            j;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 3 as libc::c_int;
    let mut m1: [libc::c_double; 9] = [
        25 as libc::c_int as libc::c_double,
        15 as libc::c_int as libc::c_double,
        -(5 as libc::c_int) as libc::c_double,
        15 as libc::c_int as libc::c_double,
        18 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        -(5 as libc::c_int) as libc::c_double,
        0 as libc::c_int as libc::c_double,
        11 as libc::c_int as libc::c_double,
    ];
    let mut c1: *mut libc::c_double = cholesky(m1.as_mut_ptr(), n);
    show_matrix(c1, n);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    free(c1 as *mut libc::c_void);
    n = 4 as libc::c_int;
    let mut m2: [libc::c_double; 16] = [
        18 as libc::c_int as libc::c_double,
        22 as libc::c_int as libc::c_double,
        54 as libc::c_int as libc::c_double,
        42 as libc::c_int as libc::c_double,
        22 as libc::c_int as libc::c_double,
        70 as libc::c_int as libc::c_double,
        86 as libc::c_int as libc::c_double,
        62 as libc::c_int as libc::c_double,
        54 as libc::c_int as libc::c_double,
        86 as libc::c_int as libc::c_double,
        174 as libc::c_int as libc::c_double,
        134 as libc::c_int as libc::c_double,
        42 as libc::c_int as libc::c_double,
        62 as libc::c_int as libc::c_double,
        134 as libc::c_int as libc::c_double,
        106 as libc::c_int as libc::c_double,
    ];
    let mut c2: *mut libc::c_double = cholesky(m2.as_mut_ptr(), n);
    show_matrix(c2, n);
    free(c2 as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
