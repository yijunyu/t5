#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn quib(
    mut strs: *mut *const libc::c_char,
    mut size: size_t,
) -> *mut libc::c_char {
    let mut len: size_t = (3 as libc::c_int as libc::c_ulong)
        .wrapping_add(
            (if size > 1 as libc::c_int as libc::c_ulong {
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(size)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        );
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size {
        len = (len as libc::c_ulong).wrapping_add(strlen(*strs.offset(i as isize)))
            as size_t as size_t;
        i = i.wrapping_add(1);
        i;
    }
    let mut s: *mut libc::c_char = malloc(
        len.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    if s.is_null() {
        perror(b"Can't allocate memory!\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    strcpy(s, b"{\0" as *const u8 as *const libc::c_char);
    match size {
        0 => {}
        1 => {
            strcat(s, *strs.offset(0 as libc::c_int as isize));
        }
        _ => {
            i = 0 as libc::c_int as size_t;
            while i < size.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                strcat(s, *strs.offset(i as isize));
                if i < size.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
                    strcat(s, b", \0" as *const u8 as *const libc::c_char);
                } else {
                    strcat(s, b" and \0" as *const u8 as *const libc::c_char);
                }
                i = i.wrapping_add(1);
                i;
            }
            strcat(s, *strs.offset(i as isize));
        }
    }
    strcat(s, b"}\0" as *const u8 as *const libc::c_char);
    return s;
}
unsafe fn main_0() -> libc::c_int {
    let mut test: [*const libc::c_char; 4] = [
        b"ABC\0" as *const u8 as *const libc::c_char,
        b"DEF\0" as *const u8 as *const libc::c_char,
        b"G\0" as *const u8 as *const libc::c_char,
        b"H\0" as *const u8 as *const libc::c_char,
    ];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < 5 as libc::c_int as libc::c_ulong {
        s = quib(test.as_mut_ptr(), i);
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, s);
        free(s as *mut libc::c_void);
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
