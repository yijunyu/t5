#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn free(_: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn mpz_left_fac_ui(mut rop: libc::c_int, mut op: libc::c_ulong) {
    let mut i: size_t = 0;
    i = 1 as libc::c_int as size_t;
    while i <= op {
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpz_digitcount(mut op: libc::c_int) -> size_t {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: size_t = strlen(t);
    free(t as *mut libc::c_void);
    return ret;
}
unsafe fn main_0() -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i <= 110 as libc::c_int as libc::c_ulong {
        i <= 10 as libc::c_int as libc::c_ulong
            || i.wrapping_rem(10 as libc::c_int as libc::c_ulong)
                == 0 as libc::c_int as libc::c_ulong;
        i = i.wrapping_add(1);
        i;
    }
    i = 1000 as libc::c_int as size_t;
    while i <= 10000 as libc::c_int as libc::c_ulong {
        i = (i as libc::c_ulong).wrapping_add(1000 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
