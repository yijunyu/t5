#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn log10(_: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
unsafe fn main_0() -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < 20 as libc::c_int as libc::c_ulong {
        let mut binstr: *mut libc::c_char = bin(i as uint32_t);
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, binstr);
        free(binstr as *mut libc::c_void);
        i = i.wrapping_add(1);
        i;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn bin(mut x: uint32_t) -> *mut libc::c_char {
    let mut bits: size_t = (if x == 0 as libc::c_int as libc::c_uint {
        1 as libc::c_int as libc::c_double
    } else {
        log10(x as libc::c_double) / log10(2 as libc::c_int as libc::c_double)
            + 1 as libc::c_int as libc::c_double
    }) as size_t;
    let mut ret: *mut libc::c_char = malloc(
        bits
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < bits {
        *ret
            .offset(
                bits.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as isize,
            ) = (if x & 1 as libc::c_int as libc::c_uint != 0 {
            '1' as i32
        } else {
            '0' as i32
        }) as libc::c_char;
        x >>= 1 as libc::c_int;
        i = i.wrapping_add(1);
        i;
    }
    *ret.offset(bits as isize) = '\0' as i32 as libc::c_char;
    return ret;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
