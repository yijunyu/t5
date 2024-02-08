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
    fn qsort(__base: *mut libc::c_void, __nmemb: u64, __size: u64, __compar: __compar_fn_t);
}
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floatList {
    pub list: *mut libc::c_float,
    pub size: i32,
}
pub type FloatList = *mut floatList;
#[no_mangle]
pub extern "C" fn floatcmp(mut a: *const libc::c_void, mut b: *const libc::c_void) -> i32 {
    unsafe {
        if *(a as *const libc::c_float) < *(b as *const libc::c_float) {
            return -1;
        } else {
            return (*(a as *const libc::c_float) > *(b as *const libc::c_float)) as i32;
        };
    }
}

#[no_mangle]
pub extern "C" fn median(mut fl: FloatList) -> libc::c_float {
    unsafe {
        qsort(
            (*fl).list as *mut libc::c_void,
            (*fl).size as u64,
            ::core::mem::size_of::<libc::c_float>() as u64,
            Some(floatcmp as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32),
        );
        return (0.5f64
            * (*((*fl).list).offset(((*fl).size / 2i32) as isize)
                + *((*fl).list).offset((((*fl).size - 1i32) / 2i32) as isize)) as f64)
            as libc::c_float;
    }
}

fn main_0() -> i32 {
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
            size: (::core::mem::size_of::<[libc::c_float; 6]>() as u64)
                .wrapping_div(::core::mem::size_of::<libc::c_float>() as u64)
                as i32,
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
            size: (::core::mem::size_of::<[libc::c_float; 5]>() as u64)
                .wrapping_div(::core::mem::size_of::<libc::c_float>() as u64)
                as i32,
        };
        init
    };
    print!("flist1 median is {:7.2}\n", median(&mut flist1) as f64);
    print!("flist2 median is {:7.2}\n", median(&mut flist2) as f64);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
