#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
pub struct replace_info {
    pub n: libc::c_int,
    pub text: *mut libc::c_char,
}
#[no_mangle]
pub unsafe extern "C" fn compare(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut x: *mut replace_info = a as *mut replace_info;
    let mut y: *mut replace_info = b as *mut replace_info;
    return (*x).n - (*y).n;
}
#[no_mangle]
pub unsafe extern "C" fn generic_fizz_buzz(
    mut max: libc::c_int,
    mut info: *mut replace_info,
    mut info_length: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut it: libc::c_int = 0;
    let mut found_word: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < max {
        found_word = 0 as libc::c_int;
        it = 0 as libc::c_int;
        while it < info_length {
            if 0 as libc::c_int == i % (*info.offset(it as isize)).n {
                printf(
                    b"%s\0" as *const u8 as *const libc::c_char,
                    (*info.offset(it as isize)).text,
                );
                found_word = 1 as libc::c_int;
            }
            it += 1;
            it;
        }
        if 0 as libc::c_int == found_word {
            printf(b"%d\0" as *const u8 as *const libc::c_char, i);
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
}
unsafe fn main_0() -> libc::c_int {
    let mut info: [replace_info; 3] = [
        {
            let mut init = replace_info {
                n: 5 as libc::c_int,
                text: b"Buzz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = replace_info {
                n: 7 as libc::c_int,
                text: b"Baxx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = replace_info {
                n: 3 as libc::c_int,
                text: b"Fizz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
    ];
    qsort(
        info.as_mut_ptr() as *mut libc::c_void,
        3 as libc::c_int as size_t,
        ::core::mem::size_of::<replace_info>() as libc::c_ulong,
        Some(
            compare
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    generic_fizz_buzz(20 as libc::c_int, info.as_mut_ptr(), 3 as libc::c_int);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
