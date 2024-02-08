#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
#[no_mangle]
pub unsafe extern "C" fn test(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    let mut code: [libc::c_char; 9] = [
        0x8b as libc::c_int as libc::c_char,
        0x44 as libc::c_int as libc::c_char,
        0x24 as libc::c_int as libc::c_char,
        0x4 as libc::c_int as libc::c_char,
        0x3 as libc::c_int as libc::c_char,
        0x44 as libc::c_int as libc::c_char,
        0x24 as libc::c_int as libc::c_char,
        0x8 as libc::c_int as libc::c_char,
        0xc3 as libc::c_int as libc::c_char,
    ];
    let mut buf: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut c: libc::c_int = 0;
    buf = mmap(
        0 as *mut libc::c_void,
        ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
        0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int,
        0x2 as libc::c_int | 0x20 as libc::c_int,
        -(1 as libc::c_int),
        0 as libc::c_int as __off_t,
    );
    memcpy(
        buf,
        code.as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
    );
    c = (::core::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int>,
    >(buf))
        .expect("non-null function pointer")(a, b);
    munmap(buf, ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong);
    return c;
}
unsafe fn main_0() -> libc::c_int {
    printf(
        b"%d\n\0" as *const u8 as *const libc::c_char,
        test(7 as libc::c_int, 12 as libc::c_int),
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
