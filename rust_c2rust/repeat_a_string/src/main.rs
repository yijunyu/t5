#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn string_repeat(
    mut n: libc::c_int,
    mut s: *const libc::c_char,
) -> *mut libc::c_char {
    let mut slen: size_t = strlen(s);
    let mut dest: *mut libc::c_char = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(slen)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    p = dest;
    while i < n {
        memcpy(p as *mut libc::c_void, s as *const libc::c_void, slen);
        i += 1;
        i;
        p = p.offset(slen as isize);
    }
    *p = '\0' as i32 as libc::c_char;
    return dest;
}
unsafe fn main_0() -> libc::c_int {
    let mut result: *mut libc::c_char = string_repeat(
        5 as libc::c_int,
        b"ha\0" as *const u8 as *const libc::c_char,
    );
    puts(result);
    free(result as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
