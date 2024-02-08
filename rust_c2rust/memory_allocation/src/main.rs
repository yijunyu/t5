#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
unsafe fn main_0() -> libc::c_int {
    let mut ints: *mut libc::c_int = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(100 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_int;
    ints = realloc(
        ints as *mut libc::c_void,
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((100 as libc::c_int + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut int2: *mut libc::c_int = calloc(
        100 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    free(ints as *mut libc::c_void);
    free(int2 as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
