#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floatList {
    pub list: *mut libc::c_float,
    pub size: libc::c_int,
}
pub type FloatList = *mut floatList;
#[no_mangle]
pub unsafe extern "C" fn floatcmp(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    if *(a as *const libc::c_float) < *(b as *const libc::c_float) {
        return -(1 as libc::c_int)
    } else {
        return (*(a as *const libc::c_float) > *(b as *const libc::c_float))
            as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn median(mut fl: FloatList) -> libc::c_float {
    qsort(
        (*fl).list as *mut libc::c_void,
        (*fl).size as size_t,
        ::core::mem::size_of::<libc::c_float>() as libc::c_ulong,
        Some(
            floatcmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    return (0.5f64
        * (*((*fl).list).offset(((*fl).size / 2 as libc::c_int) as isize)
            + *((*fl).list)
                .offset((((*fl).size - 1 as libc::c_int) / 2 as libc::c_int) as isize))
            as libc::c_double) as libc::c_float;
}
unsafe fn main_0() -> libc::c_int {
    let mut floats1: [libc::c_float; 6] = [
        5.1f64 as libc::c_float,
        2.6f64 as libc::c_float,
        6.2f64 as libc::c_float,
        8.8f64 as libc::c_float,
        4.6f64 as libc::c_float,
        4.1f64 as libc::c_float,
    ];
    let mut flist1: floatList = {
        let mut init = floatList {
            list: floats1.as_mut_ptr(),
            size: (::core::mem::size_of::<[libc::c_float; 6]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_float>() as libc::c_ulong)
                as libc::c_int,
        };
        init
    };
    let mut floats2: [libc::c_float; 5] = [
        5.1f64 as libc::c_float,
        2.6f64 as libc::c_float,
        8.8f64 as libc::c_float,
        4.6f64 as libc::c_float,
        4.1f64 as libc::c_float,
    ];
    let mut flist2: floatList = {
        let mut init = floatList {
            list: floats2.as_mut_ptr(),
            size: (::core::mem::size_of::<[libc::c_float; 5]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_float>() as libc::c_ulong)
                as libc::c_int,
        };
        init
    };
    printf(
        b"flist1 median is %7.2f\n\0" as *const u8 as *const libc::c_char,
        median(&mut flist1) as libc::c_double,
    );
    printf(
        b"flist2 median is %7.2f\n\0" as *const u8 as *const libc::c_char,
        median(&mut flist2) as libc::c_double,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
