#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn incr(mut s: *mut libc::c_char) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut begin: libc::c_int = 0;
    let mut tail: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut neg: libc::c_int = (*s as libc::c_int == '-' as i32) as libc::c_int;
    let mut tgt: libc::c_char = (if neg != 0 { '0' as i32 } else { '9' as i32 })
        as libc::c_char;
    if strcmp(s, b"-1\0" as *const u8 as *const libc::c_char) == 0 {
        *s.offset(0 as libc::c_int as isize) = '0' as i32 as libc::c_char;
        *s.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        return s;
    }
    len = strlen(s) as libc::c_int;
    begin = if *s as libc::c_int == '-' as i32 || *s as libc::c_int == '+' as i32 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    tail = len - 1 as libc::c_int;
    while tail >= begin && *s.offset(tail as isize) as libc::c_int == tgt as libc::c_int
    {
        tail -= 1;
        tail;
    }
    if tail < begin && neg == 0 {
        if begin == 0 {
            s = realloc(
                s as *mut libc::c_void,
                (len + 2 as libc::c_int) as libc::c_ulong,
            ) as *mut libc::c_char;
        }
        *s.offset(0 as libc::c_int as isize) = '1' as i32 as libc::c_char;
        i = 1 as libc::c_int;
        while i <= len - begin {
            *s.offset(i as isize) = '0' as i32 as libc::c_char;
            i += 1;
            i;
        }
        *s.offset((len + 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
    } else if tail == begin && neg != 0
        && *s.offset(1 as libc::c_int as isize) as libc::c_int == '1' as i32
    {
        i = 1 as libc::c_int;
        while i < len - begin {
            *s.offset(i as isize) = '9' as i32 as libc::c_char;
            i += 1;
            i;
        }
        *s.offset((len - 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
    } else {
        i = len - 1 as libc::c_int;
        while i > tail {
            *s
                .offset(
                    i as isize,
                ) = (if neg != 0 { '9' as i32 } else { '0' as i32 }) as libc::c_char;
            i -= 1;
            i;
        }
        let ref mut fresh0 = *s.offset(tail as isize);
        *fresh0 = (*fresh0 as libc::c_int
            + if neg != 0 { -(1 as libc::c_int) } else { 1 as libc::c_int })
            as libc::c_char;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn string_test(mut s: *const libc::c_char) {
    let mut ret: *mut libc::c_char = malloc(strlen(s)) as *mut libc::c_char;
    strcpy(ret, s);
    printf(b"text: %s\n\0" as *const u8 as *const libc::c_char, ret);
    ret = incr(ret);
    printf(b"  ->: %s\n\0" as *const u8 as *const libc::c_char, ret);
    free(ret as *mut libc::c_void);
}
unsafe fn main_0() -> libc::c_int {
    string_test(b"+0\0" as *const u8 as *const libc::c_char);
    string_test(b"-1\0" as *const u8 as *const libc::c_char);
    string_test(b"-41\0" as *const u8 as *const libc::c_char);
    string_test(b"+41\0" as *const u8 as *const libc::c_char);
    string_test(b"999\0" as *const u8 as *const libc::c_char);
    string_test(b"+999\0" as *const u8 as *const libc::c_char);
    string_test(
        b"109999999999999999999999999999999999999999\0" as *const u8
            as *const libc::c_char,
    );
    string_test(
        b"-100000000000000000000000000000000000000000000\0" as *const u8
            as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
