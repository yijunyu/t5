#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar(__c: libc::c_int) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[no_mangle]
pub unsafe extern "C" fn get_list(
    mut s: *const libc::c_char,
    mut e: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    loop {
        while *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            s = s.offset(1);
            s;
        }
        if get_rnge(s, e) == 0
            && {
                x = strtol(s, e, 10 as libc::c_int) as libc::c_int;
                !(*e != s as *mut libc::c_char)
            }
        {
            break;
        }
        s = *e;
        while *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            s = s.offset(1);
            s;
        }
        if *s as libc::c_int == '\0' as i32 {
            putchar('\n' as i32);
            return 1 as libc::c_int;
        }
        if !(*s as libc::c_int == ',' as i32) {
            break;
        }
        s = s.offset(1);
        s;
    }
    let ref mut fresh0 = *(e as *mut *const libc::c_char);
    *fresh0 = s;
    printf(b"\nSyntax error at %s\n\0" as *const u8 as *const libc::c_char, s);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_rnge(
    mut s: *const libc::c_char,
    mut e: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut ee: *mut libc::c_char = 0 as *mut libc::c_char;
    x = strtol(s, &mut ee, 10 as libc::c_int) as libc::c_int;
    if !(ee != s as *mut libc::c_char) {
        return 0 as libc::c_int;
    }
    s = ee;
    while *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        s = s.offset(1);
        s;
    }
    if *s as libc::c_int != '-' as i32 {
        let ref mut fresh1 = *(e as *mut *const libc::c_char);
        *fresh1 = s;
        return 0 as libc::c_int;
    }
    s = s.offset(1);
    s;
    y = strtol(s, e, 10 as libc::c_int) as libc::c_int;
    if !(*e != s as *mut libc::c_char) {
        return 0 as libc::c_int;
    }
    return add_range(x, y);
}
#[no_mangle]
pub unsafe extern "C" fn add_number(mut x: libc::c_int) {
    printf(b"%d \0" as *const u8 as *const libc::c_char, x);
}
#[no_mangle]
pub unsafe extern "C" fn add_range(
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> libc::c_int {
    if y <= x {
        return 0 as libc::c_int;
    }
    while x <= y {
        let fresh2 = x;
        x = x + 1;
        printf(b"%d \0" as *const u8 as *const libc::c_char, fresh2);
    }
    return 1 as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    if get_list(
        b"-6,-3--1,3-5,7-11,14,15,17-20\0" as *const u8 as *const libc::c_char,
        &mut end,
    ) != 0
    {
        puts(b"Ok\0" as *const u8 as *const libc::c_char);
    }
    get_list(
        b"-6 -3--1,3-5,7-11,14,15,17-20\0" as *const u8 as *const libc::c_char,
        &mut end,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
