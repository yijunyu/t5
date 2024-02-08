#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn even_sel(mut x: libc::c_int) -> libc::c_int {
    return (x & 1 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tri_sel(mut x: libc::c_int) -> libc::c_int {
    return x % 3 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn grep(
    mut in_0: *mut libc::c_int,
    mut len: libc::c_int,
    mut outlen: *mut libc::c_int,
    mut sel: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
    mut inplace: libc::c_int,
) -> *mut libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut out: *mut libc::c_int = 0 as *mut libc::c_int;
    if inplace != 0 {
        out = in_0;
    } else {
        out = malloc(
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(len as libc::c_ulong),
        ) as *mut libc::c_int;
    }
    j = 0 as libc::c_int;
    i = j;
    while i < len {
        if sel.expect("non-null function pointer")(*in_0.offset(i as isize)) != 0 {
            let fresh0 = j;
            j = j + 1;
            *out.offset(fresh0 as isize) = *in_0.offset(i as isize);
        }
        i += 1;
        i;
    }
    if inplace == 0 && j < len {
        out = realloc(
            out as *mut libc::c_void,
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(j as libc::c_ulong),
        ) as *mut libc::c_int;
    }
    *outlen = j;
    return out;
}
unsafe fn main_0() -> libc::c_int {
    let mut in_0: [libc::c_int; 10] = [
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        7 as libc::c_int,
        8 as libc::c_int,
        9 as libc::c_int,
        10 as libc::c_int,
    ];
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut even: *mut libc::c_int = grep(
        in_0.as_mut_ptr(),
        10 as libc::c_int,
        &mut len,
        Some(even_sel as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
        0 as libc::c_int,
    );
    printf(b"Filtered even:\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < len {
        printf(b" %d\0" as *const u8 as *const libc::c_char, *even.offset(i as isize));
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    grep(
        in_0.as_mut_ptr(),
        8 as libc::c_int,
        &mut len,
        Some(tri_sel as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
        1 as libc::c_int,
    );
    printf(
        b"In-place filtered not multiple of 3:\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < len {
        printf(b" %d\0" as *const u8 as *const libc::c_char, in_0[i as usize]);
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
