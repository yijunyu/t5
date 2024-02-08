#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn elem(
    mut a: *mut libc::c_int,
    mut n: size_t,
    mut e: libc::c_int,
) -> bool {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n {
        if *a.offset(i as isize) == e {
            return 1 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn nub(mut a: *mut libc::c_int, mut n: size_t) -> size_t {
    let mut m: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n {
        if !elem(a, m, *a.offset(i as isize)) {
            let fresh0 = m;
            m = m.wrapping_add(1);
            *a.offset(fresh0 as isize) = *a.offset(i as isize);
        }
        i = i.wrapping_add(1);
        i;
    }
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn nub_new(
    mut b: *mut *mut libc::c_int,
    mut a: *mut libc::c_int,
    mut n: size_t,
) -> size_t {
    let mut c: *mut libc::c_int = malloc(
        n.wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    memcpy(
        c as *mut libc::c_void,
        a as *const libc::c_void,
        n.wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    let mut m: libc::c_int = nub(c, n) as libc::c_int;
    *b = malloc(
        (m as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    memcpy(
        *b as *mut libc::c_void,
        c as *const libc::c_void,
        (m as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    free(c as *mut libc::c_void);
    return m as size_t;
}
unsafe fn main_0() -> libc::c_int {
    let mut a: [libc::c_int; 10] = [
        1 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        2 as libc::c_int,
        15 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
    ];
    let mut b: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut n: size_t = nub_new(
        &mut b,
        a.as_mut_ptr(),
        (::core::mem::size_of::<[libc::c_int; 10]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n {
        printf(b"%d \0" as *const u8 as *const libc::c_char, *b.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    puts(b"\0" as *const u8 as *const libc::c_char);
    free(b as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
