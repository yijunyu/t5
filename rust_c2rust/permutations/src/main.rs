#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    if argc < 2 as libc::c_int {
        printf(
            b"Enter an argument. Example 1234 or dcba:\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while *(*argv.offset(1 as libc::c_int as isize)).offset(x as isize) as libc::c_int
        != '\0' as i32
    {
        x += 1;
        x;
    }
    let mut f: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    f = 0 as libc::c_int;
    while f < x {
        v = x - 1 as libc::c_int;
        while v > f {
            if *(*argv.offset(1 as libc::c_int as isize))
                .offset((v - 1 as libc::c_int) as isize) as libc::c_int
                > *(*argv.offset(1 as libc::c_int as isize)).offset(v as isize)
                    as libc::c_int
            {
                m = *(*argv.offset(1 as libc::c_int as isize))
                    .offset((v - 1 as libc::c_int) as isize) as libc::c_int;
                *(*argv.offset(1 as libc::c_int as isize))
                    .offset(
                        (v - 1 as libc::c_int) as isize,
                    ) = *(*argv.offset(1 as libc::c_int as isize)).offset(v as isize);
                *(*argv.offset(1 as libc::c_int as isize))
                    .offset(v as isize) = m as libc::c_char;
            }
            v -= 1;
            v;
        }
        f += 1;
        f;
    }
    let vla = x as usize;
    let mut a: Vec::<libc::c_char> = ::std::vec::from_elem(0, vla);
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut fact: libc::c_int = k + 1 as libc::c_int;
    while k != x {
        *a
            .as_mut_ptr()
            .offset(
                k as isize,
            ) = *(*argv.offset(1 as libc::c_int as isize)).offset(k as isize);
        k += 1;
        k;
        fact = k * fact;
    }
    *a.as_mut_ptr().offset(k as isize) = '\0' as i32 as libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut y: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_char = 0;
    while y != fact {
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, a.as_mut_ptr());
        i = x - 2 as libc::c_int;
        while *a.as_mut_ptr().offset(i as isize) as libc::c_int
            > *a.as_mut_ptr().offset((i + 1 as libc::c_int) as isize) as libc::c_int
        {
            i -= 1;
            i;
        }
        j = x - 1 as libc::c_int;
        while (*a.as_mut_ptr().offset(j as isize) as libc::c_int)
            < *a.as_mut_ptr().offset(i as isize) as libc::c_int
        {
            j -= 1;
            j;
        }
        c = *a.as_mut_ptr().offset(j as isize);
        *a.as_mut_ptr().offset(j as isize) = *a.as_mut_ptr().offset(i as isize);
        *a.as_mut_ptr().offset(i as isize) = c;
        i += 1;
        i;
        j = x - 1 as libc::c_int;
        while j > i {
            c = *a.as_mut_ptr().offset(i as isize);
            *a.as_mut_ptr().offset(i as isize) = *a.as_mut_ptr().offset(j as isize);
            *a.as_mut_ptr().offset(j as isize) = c;
            i += 1;
            i;
            j -= 1;
            j;
        }
        y += 1;
        y;
    }
    return 0;
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
