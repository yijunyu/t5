#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vcount {
    pub v: libc::c_double,
    pub c: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn cmp_dbl(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut x: libc::c_double = *(a as *const libc::c_double)
        - *(b as *const libc::c_double);
    return if x < 0 as libc::c_int as libc::c_double {
        -(1 as libc::c_int)
    } else {
        (x > 0 as libc::c_int as libc::c_double) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn vc_cmp(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return (*(b as *const vcount)).c - (*(a as *const vcount)).c;
}
#[no_mangle]
pub unsafe extern "C" fn get_mode(
    mut x: *mut libc::c_double,
    mut len_0: libc::c_int,
    mut list: *mut *mut vcount,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut vc: *mut vcount = 0 as *mut vcount;
    qsort(
        x as *mut libc::c_void,
        len_0 as size_t,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
        Some(
            cmp_dbl
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    j = 1 as libc::c_int;
    while i < len_0 - 1 as libc::c_int {
        i += 1;
        i;
        j
            += (*x.offset(i as isize) != *x.offset((i + 1 as libc::c_int) as isize))
                as libc::c_int;
    }
    vc = malloc(
        (::core::mem::size_of::<vcount>() as libc::c_ulong)
            .wrapping_mul(j as libc::c_ulong),
    ) as *mut vcount;
    *list = vc;
    (*vc.offset(0 as libc::c_int as isize)).v = *x.offset(0 as libc::c_int as isize);
    (*vc.offset(0 as libc::c_int as isize)).c = 1 as libc::c_int;
    j = 0 as libc::c_int;
    i = j;
    while i < len_0 - 1 as libc::c_int {
        if *x.offset(i as isize) != *x.offset((i + 1 as libc::c_int) as isize) {
            j += 1;
            (*vc.offset(j as isize)).v = *x.offset((i + 1 as libc::c_int) as isize);
        }
        i += 1;
        i;
        let ref mut fresh0 = (*vc.offset(j as isize)).c;
        *fresh0 += 1;
        *fresh0;
    }
    qsort(
        vc as *mut libc::c_void,
        (j + 1 as libc::c_int) as size_t,
        ::core::mem::size_of::<vcount>() as libc::c_ulong,
        Some(
            vc_cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i <= j
        && (*vc.offset(i as isize)).c == (*vc.offset(0 as libc::c_int as isize)).c
    {
        i += 1;
        i;
    }
    return i;
}
unsafe fn main_0() -> libc::c_int {
    let mut values: [libc::c_double; 13] = [
        1 as libc::c_int as libc::c_double,
        3 as libc::c_int as libc::c_double,
        6 as libc::c_int as libc::c_double,
        6 as libc::c_int as libc::c_double,
        6 as libc::c_int as libc::c_double,
        6 as libc::c_int as libc::c_double,
        7 as libc::c_int as libc::c_double,
        7 as libc::c_int as libc::c_double,
        12 as libc::c_int as libc::c_double,
        12 as libc::c_int as libc::c_double,
        12 as libc::c_int as libc::c_double,
        12 as libc::c_int as libc::c_double,
        17 as libc::c_int as libc::c_double,
    ];
    let mut vc: *mut vcount = 0 as *mut vcount;
    let mut i: libc::c_int = 0;
    let mut n_modes: libc::c_int = get_mode(
        values.as_mut_ptr(),
        (::core::mem::size_of::<[libc::c_double; 13]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
            as libc::c_int,
        &mut vc,
    );
    printf(b"got %d modes:\n\0" as *const u8 as *const libc::c_char, n_modes);
    i = 0 as libc::c_int;
    while i < n_modes {
        printf(
            b"\tvalue = %g, count = %d\n\0" as *const u8 as *const libc::c_char,
            (*vc.offset(i as isize)).v,
            (*vc.offset(i as isize)).c,
        );
        i += 1;
        i;
    }
    free(vc as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
