#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abs(_: libc::c_int) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SquareMatrix {
    pub n: libc::c_int,
    pub elems: *mut *mut libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn init_square_matrix(
    mut n: libc::c_int,
    mut elems: *mut libc::c_double,
) -> SquareMatrix {
    let vla = n as usize;
    let mut A: SquareMatrix = {
        let mut init = SquareMatrix {
            n: n,
            elems: malloc(
                (n as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<*mut libc::c_double>() as libc::c_ulong,
                    ),
            ) as *mut *mut libc::c_double,
        };
        init
    };
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let ref mut fresh0 = *(A.elems).offset(i as isize);
        *fresh0 = malloc(
            (n as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
        ) as *mut libc::c_double;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < n {
            *(*(A.elems).offset(i as isize))
                .offset(
                    j as isize,
                ) = *elems.offset(i as isize * vla as isize).offset(j as isize);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn copy_square_matrix(mut src: SquareMatrix) -> SquareMatrix {
    let mut dest: SquareMatrix = SquareMatrix {
        n: 0,
        elems: 0 as *mut *mut libc::c_double,
    };
    dest.n = src.n;
    dest
        .elems = malloc(
        (dest.n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_double>() as libc::c_ulong),
    ) as *mut *mut libc::c_double;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < dest.n {
        let ref mut fresh1 = *(dest.elems).offset(i as isize);
        *fresh1 = malloc(
            (dest.n as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
        ) as *mut libc::c_double;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < dest.n {
            *(*(dest.elems).offset(i as isize))
                .offset(
                    j as isize,
                ) = *(*(src.elems).offset(i as isize)).offset(j as isize);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return dest;
}
#[no_mangle]
pub unsafe extern "C" fn det(mut A: SquareMatrix) -> libc::c_double {
    let mut det_0: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < A.n {
        let mut i_max: libc::c_int = j;
        let mut i: libc::c_int = j;
        while i < A.n {
            if *(*(A.elems).offset(i as isize)).offset(j as isize)
                > *(*(A.elems).offset(i_max as isize)).offset(j as isize)
            {
                i_max = i;
            }
            i += 1;
            i;
        }
        if i_max != j {
            let mut k: libc::c_int = 0 as libc::c_int;
            while k < A.n {
                let mut tmp: libc::c_double = *(*(A.elems).offset(i_max as isize))
                    .offset(k as isize);
                *(*(A.elems).offset(i_max as isize))
                    .offset(
                        k as isize,
                    ) = *(*(A.elems).offset(j as isize)).offset(k as isize);
                *(*(A.elems).offset(j as isize)).offset(k as isize) = tmp;
                k += 1;
                k;
            }
            det_0 *= -(1 as libc::c_int) as libc::c_double;
        }
        if (abs(*(*(A.elems).offset(j as isize)).offset(j as isize) as libc::c_int)
            as libc::c_double) < 1e-12f64
        {
            puts(b"Singular matrix!\0" as *const u8 as *const libc::c_char);
            return ::core::f32::NAN as libc::c_double;
        }
        let mut i_0: libc::c_int = j + 1 as libc::c_int;
        while i_0 < A.n {
            let mut mult: libc::c_double = -*(*(A.elems).offset(i_0 as isize))
                .offset(j as isize)
                / *(*(A.elems).offset(j as isize)).offset(j as isize);
            let mut k_0: libc::c_int = 0 as libc::c_int;
            while k_0 < A.n {
                *(*(A.elems).offset(i_0 as isize)).offset(k_0 as isize)
                    += mult * *(*(A.elems).offset(j as isize)).offset(k_0 as isize);
                k_0 += 1;
                k_0;
            }
            i_0 += 1;
            i_0;
        }
        j += 1;
        j;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < A.n {
        det_0 *= *(*(A.elems).offset(i_1 as isize)).offset(i_1 as isize);
        i_1 += 1;
        i_1;
    }
    return det_0;
}
#[no_mangle]
pub unsafe extern "C" fn deinit_square_matrix(mut A: SquareMatrix) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < A.n {
        free(*(A.elems).offset(i as isize) as *mut libc::c_void);
        i += 1;
        i;
    }
    free(A.elems as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn cramer_solve(
    mut A: SquareMatrix,
    mut det_A: libc::c_double,
    mut b: *mut libc::c_double,
    mut var: libc::c_int,
) -> libc::c_double {
    let mut tmp: SquareMatrix = copy_square_matrix(A);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < tmp.n {
        *(*(tmp.elems).offset(i as isize)).offset(var as isize) = *b.offset(i as isize);
        i += 1;
        i;
    }
    let mut det_tmp: libc::c_double = det(tmp);
    deinit_square_matrix(tmp);
    return det_tmp / det_A;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut elems: [[libc::c_double; 4]; 4] = [
        [
            2 as libc::c_int as libc::c_double,
            -(1 as libc::c_int) as libc::c_double,
            5 as libc::c_int as libc::c_double,
            1 as libc::c_int as libc::c_double,
        ],
        [
            3 as libc::c_int as libc::c_double,
            2 as libc::c_int as libc::c_double,
            2 as libc::c_int as libc::c_double,
            -(6 as libc::c_int) as libc::c_double,
        ],
        [
            1 as libc::c_int as libc::c_double,
            3 as libc::c_int as libc::c_double,
            3 as libc::c_int as libc::c_double,
            -(1 as libc::c_int) as libc::c_double,
        ],
        [
            5 as libc::c_int as libc::c_double,
            -(2 as libc::c_int) as libc::c_double,
            -(3 as libc::c_int) as libc::c_double,
            3 as libc::c_int as libc::c_double,
        ],
    ];
    let mut A: SquareMatrix = init_square_matrix(
        4 as libc::c_int,
        elems.as_mut_ptr() as *mut libc::c_double,
    );
    let mut tmp: SquareMatrix = copy_square_matrix(A);
    let mut det_A: libc::c_int = det(tmp) as libc::c_int;
    deinit_square_matrix(tmp);
    let mut b: [libc::c_double; 4] = [
        -(3 as libc::c_int) as libc::c_double,
        -(32 as libc::c_int) as libc::c_double,
        -(47 as libc::c_int) as libc::c_double,
        49 as libc::c_int as libc::c_double,
    ];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        printf(
            b"%7.3lf\n\0" as *const u8 as *const libc::c_char,
            cramer_solve(A, det_A as libc::c_double, b.as_mut_ptr(), i),
        );
        i += 1;
        i;
    }
    deinit_square_matrix(A);
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
