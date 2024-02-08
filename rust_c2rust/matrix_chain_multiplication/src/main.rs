#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[no_mangle]
pub static mut m: *mut *mut libc::c_int = 0 as *const *mut libc::c_int
    as *mut *mut libc::c_int;
#[no_mangle]
pub static mut s: *mut *mut libc::c_int = 0 as *const *mut libc::c_int
    as *mut *mut libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn optimal_matrix_chain_order(
    mut dims: *mut libc::c_int,
    mut n: libc::c_int,
) {
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut temp: libc::c_int = 0;
    let mut cost: libc::c_int = 0;
    n -= 1;
    n;
    m = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh0 = *m.offset(i as isize);
        *fresh0 = calloc(
            n as libc::c_ulong,
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        i += 1;
        i;
    }
    s = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh1 = *s.offset(i as isize);
        *fresh1 = calloc(
            n as libc::c_ulong,
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        i += 1;
        i;
    }
    len = 1 as libc::c_int;
    while len < n {
        i = 0 as libc::c_int;
        while i < n - len {
            j = i + len;
            *(*m.offset(i as isize)).offset(j as isize) = 2147483647 as libc::c_int;
            k = i;
            while k < j {
                temp = *dims.offset(i as isize)
                    * *dims.offset((k + 1 as libc::c_int) as isize)
                    * *dims.offset((j + 1 as libc::c_int) as isize);
                cost = *(*m.offset(i as isize)).offset(k as isize)
                    + *(*m.offset((k + 1 as libc::c_int) as isize)).offset(j as isize)
                    + temp;
                if cost < *(*m.offset(i as isize)).offset(j as isize) {
                    *(*m.offset(i as isize)).offset(j as isize) = cost;
                    *(*s.offset(i as isize)).offset(j as isize) = k;
                }
                k += 1;
                k;
            }
            i += 1;
            i;
        }
        len += 1;
        len;
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_optimal_chain_order(
    mut i: libc::c_int,
    mut j: libc::c_int,
) {
    if i == j {
        printf(b"%c\0" as *const u8 as *const libc::c_char, i + 65 as libc::c_int);
    } else {
        printf(b"(\0" as *const u8 as *const libc::c_char);
        print_optimal_chain_order(i, *(*s.offset(i as isize)).offset(j as isize));
        print_optimal_chain_order(
            *(*s.offset(i as isize)).offset(j as isize) + 1 as libc::c_int,
            j,
        );
        printf(b")\0" as *const u8 as *const libc::c_char);
    };
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut a1: [libc::c_int; 4] = [
        5 as libc::c_int,
        6 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
    ];
    let mut a2: [libc::c_int; 13] = [
        1 as libc::c_int,
        5 as libc::c_int,
        25 as libc::c_int,
        30 as libc::c_int,
        100 as libc::c_int,
        70 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        100 as libc::c_int,
        250 as libc::c_int,
        1 as libc::c_int,
        1000 as libc::c_int,
        2 as libc::c_int,
    ];
    let mut a3: [libc::c_int; 12] = [
        1000 as libc::c_int,
        1 as libc::c_int,
        500 as libc::c_int,
        12 as libc::c_int,
        1 as libc::c_int,
        700 as libc::c_int,
        2500 as libc::c_int,
        3 as libc::c_int,
        2 as libc::c_int,
        5 as libc::c_int,
        14 as libc::c_int,
        10 as libc::c_int,
    ];
    let mut dims_list: [*mut libc::c_int; 3] = [
        a1.as_mut_ptr(),
        a2.as_mut_ptr(),
        a3.as_mut_ptr(),
    ];
    let mut sizes: [libc::c_int; 3] = [
        4 as libc::c_int,
        13 as libc::c_int,
        12 as libc::c_int,
    ];
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        printf(b"Dims  : [\0" as *const u8 as *const libc::c_char);
        n = sizes[i as usize];
        j = 0 as libc::c_int;
        while j < n {
            printf(
                b"%d\0" as *const u8 as *const libc::c_char,
                *(dims_list[i as usize]).offset(j as isize),
            );
            if j < n - 1 as libc::c_int {
                printf(b", \0" as *const u8 as *const libc::c_char);
            } else {
                printf(b"]\n\0" as *const u8 as *const libc::c_char);
            }
            j += 1;
            j;
        }
        optimal_matrix_chain_order(dims_list[i as usize], n);
        printf(b"Order : \0" as *const u8 as *const libc::c_char);
        print_optimal_chain_order(0 as libc::c_int, n - 2 as libc::c_int);
        printf(
            b"\nCost  : %d\n\n\0" as *const u8 as *const libc::c_char,
            *(*m.offset(0 as libc::c_int as isize))
                .offset((n - 2 as libc::c_int) as isize),
        );
        j = 0 as libc::c_int;
        while j <= n - 2 as libc::c_int {
            free(*m.offset(j as isize) as *mut libc::c_void);
            j += 1;
            j;
        }
        free(m as *mut libc::c_void);
        j = 0 as libc::c_int;
        while j <= n - 2 as libc::c_int {
            free(*s.offset(j as isize) as *mut libc::c_void);
            j += 1;
            j;
        }
        free(s as *mut libc::c_void);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
