#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use c2rust_out::*;
extern "C" {
    fn free(_: *mut libc::c_void);
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
}
fn main_0() -> i32 {
    unsafe {
        let mut ints: *mut i32 =
            malloc((::core::mem::size_of::<i32>() as u64).wrapping_mul(100)) as *mut i32;
        ints = realloc(
            ints as *mut libc::c_void,
            (::core::mem::size_of::<i32>() as u64).wrapping_mul((100 + 1i32) as u64),
        ) as *mut i32;
        let mut int2: *mut i32 = calloc(100, ::core::mem::size_of::<i32>() as u64) as *mut i32;
        free(ints as *mut libc::c_void);
        free(int2 as *mut libc::c_void);
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
