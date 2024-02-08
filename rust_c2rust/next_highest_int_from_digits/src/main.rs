#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
#[no_mangle]
pub unsafe extern "C" fn swap(
    mut str: *mut libc::c_char,
    mut i: libc::c_int,
    mut j: libc::c_int,
) {
    let mut c: libc::c_char = *str.offset(i as isize);
    *str.offset(i as isize) = *str.offset(j as isize);
    *str.offset(j as isize) = c;
}
#[no_mangle]
pub unsafe extern "C" fn reverse(
    mut str: *mut libc::c_char,
    mut i: libc::c_int,
    mut j: libc::c_int,
) {
    while i < j {
        swap(str, i, j);
        i += 1;
        i;
        j -= 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn next_permutation(mut str: *mut libc::c_char) -> bool {
    let mut len: libc::c_int = strlen(str) as libc::c_int;
    if len < 2 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    let mut i: libc::c_int = len - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        let mut j: libc::c_int = i;
        let mut k: libc::c_int = 0;
        i -= 1;
        if (*str.offset(i as isize) as libc::c_int)
            < *str.offset(j as isize) as libc::c_int
        {
            k = len;
            loop {
                k -= 1;
                if !(*str.offset(i as isize) as libc::c_int
                    >= *str.offset(k as isize) as libc::c_int)
                {
                    break;
                }
            }
            swap(str, i, k);
            reverse(str, j, len - 1 as libc::c_int);
            return 1 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn next_highest_int(mut n: uint32_t) -> uint32_t {
    let mut str: [libc::c_char; 16] = [0; 16];
    snprintf(
        str.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        b"%u\0" as *const u8 as *const libc::c_char,
        n,
    );
    if !next_permutation(str.as_mut_ptr()) {
        return 0 as libc::c_int as uint32_t;
    }
    return strtoul(str.as_mut_ptr(), 0 as *mut *mut libc::c_char, 10 as libc::c_int)
        as uint32_t;
}
unsafe fn main_0() -> libc::c_int {
    let mut numbers: [uint32_t; 8] = [
        0 as libc::c_int as uint32_t,
        9 as libc::c_int as uint32_t,
        12 as libc::c_int as uint32_t,
        21 as libc::c_int as uint32_t,
        12453 as libc::c_int as uint32_t,
        738440 as libc::c_int as uint32_t,
        45072010 as libc::c_int as uint32_t,
        95322020 as libc::c_int as uint32_t,
    ];
    let count: libc::c_int = (::core::mem::size_of::<[uint32_t; 8]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        printf(
            b"%d -> %d\n\0" as *const u8 as *const libc::c_char,
            numbers[i as usize],
            next_highest_int(numbers[i as usize]),
        );
        i += 1;
        i;
    }
    let big: [libc::c_char; 23] = *::core::mem::transmute::<
        &[u8; 23],
        &[libc::c_char; 23],
    >(b"9589776899767587796600\0");
    let mut next: [libc::c_char; 23] = [0; 23];
    memcpy(
        next.as_mut_ptr() as *mut libc::c_void,
        big.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong,
    );
    next_permutation(next.as_mut_ptr());
    printf(
        b"%s -> %s\n\0" as *const u8 as *const libc::c_char,
        big.as_ptr(),
        next.as_mut_ptr(),
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
