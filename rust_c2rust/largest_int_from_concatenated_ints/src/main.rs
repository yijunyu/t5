#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar(__c: libc::c_int) -> libc::c_int;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[no_mangle]
pub unsafe extern "C" fn catcmp(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut ab: [libc::c_char; 32] = [0; 32];
    let mut ba: [libc::c_char; 32] = [0; 32];
    sprintf(
        ab.as_mut_ptr(),
        b"%d%d\0" as *const u8 as *const libc::c_char,
        *(a as *mut libc::c_int),
        *(b as *mut libc::c_int),
    );
    sprintf(
        ba.as_mut_ptr(),
        b"%d%d\0" as *const u8 as *const libc::c_char,
        *(b as *mut libc::c_int),
        *(a as *mut libc::c_int),
    );
    return strcmp(ba.as_mut_ptr(), ab.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn maxcat(mut a: *mut libc::c_int, mut len: libc::c_int) {
    let mut i: libc::c_int = 0;
    qsort(
        a as *mut libc::c_void,
        len as size_t,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        Some(
            catcmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < len {
        printf(b"%d\0" as *const u8 as *const libc::c_char, *a.offset(i as isize));
        i += 1;
        i;
    }
    putchar('\n' as i32);
}
unsafe fn main_0() -> libc::c_int {
    let mut x: [libc::c_int; 8] = [
        1 as libc::c_int,
        34 as libc::c_int,
        3 as libc::c_int,
        98 as libc::c_int,
        9 as libc::c_int,
        76 as libc::c_int,
        45 as libc::c_int,
        4 as libc::c_int,
    ];
    let mut y: [libc::c_int; 4] = [
        54 as libc::c_int,
        546 as libc::c_int,
        548 as libc::c_int,
        60 as libc::c_int,
    ];
    maxcat(
        x.as_mut_ptr(),
        (::core::mem::size_of::<[libc::c_int; 8]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int,
    );
    maxcat(
        y.as_mut_ptr(),
        (::core::mem::size_of::<[libc::c_int; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
