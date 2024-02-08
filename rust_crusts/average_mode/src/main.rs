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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn qsort(__base: *mut libc::c_void, __nmemb: u64, __size: u64, __compar: __compar_fn_t);
}
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vcount {
    pub v: f64,
    pub c: i32,
}
#[no_mangle]
pub extern "C" fn cmp_dbl(mut a: *const libc::c_void, mut b: *const libc::c_void) -> i32 {
    unsafe {
        let mut x: f64 = *(a as *const f64) - *(b as *const f64);
        return if x < 0 as f64 {
            -1
        } else {
            (x > 0 as f64) as i32
        };
    }
}

#[no_mangle]
pub extern "C" fn vc_cmp(mut a: *const libc::c_void, mut b: *const libc::c_void) -> i32 {
    unsafe {
        return (*(b as *const vcount)).c - (*(a as *const vcount)).c;
    }
}

#[no_mangle]
pub extern "C" fn get_mode(mut x: *mut f64, mut len_0: i32, mut list: *mut *mut vcount) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut vc: *mut vcount = 0 as *mut vcount;
        qsort(
            x as *mut libc::c_void,
            len_0 as u64,
            ::core::mem::size_of::<f64>() as u64,
            Some(cmp_dbl as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32),
        );
        i = 0;
        j = 1;
        while i < len_0 - 1 {
            i += 1;
            i;
            j += (*x.offset(i as isize) != *x.offset((i + 1i32) as isize)) as i32;
        }
        vc =
            malloc((::core::mem::size_of::<vcount>() as u64).wrapping_mul(j as u64)) as *mut vcount;
        *list = vc;
        (*vc.offset(0 as isize)).v = *x.offset(0 as isize);
        (*vc.offset(0 as isize)).c = 1;
        j = 0;
        i = j;
        while i < len_0 - 1 {
            if *x.offset(i as isize) != *x.offset((i + 1i32) as isize) {
                j += 1;
                (*vc.offset(j as isize)).v = *x.offset((i + 1i32) as isize);
            }
            i += 1;
            i;
            let ref mut fresh0 = (*vc.offset(j as isize)).c;
            *fresh0 += 1;
            *fresh0;
        }
        qsort(
            vc as *mut libc::c_void,
            (j + 1i32) as u64,
            ::core::mem::size_of::<vcount>() as u64,
            Some(vc_cmp as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32),
        );
        i = 0;
        while i <= j && (*vc.offset(i as isize)).c == (*vc.offset(0 as isize)).c {
            i += 1;
            i;
        }
        return i;
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut values: [f64; 13] = [
            1 as f64, 3 as f64, 6 as f64, 6 as f64, 6 as f64, 6 as f64, 7 as f64, 7 as f64,
            12 as f64, 12 as f64, 12 as f64, 12 as f64, 17 as f64,
        ];
        let mut vc: *mut vcount = 0 as *mut vcount;
        let mut i: i32 = 0;
        let mut n_modes: i32 = get_mode(
            values.as_mut_ptr(),
            (::core::mem::size_of::<[f64; 13]>() as u64)
                .wrapping_div(::core::mem::size_of::<f64>() as u64) as i32,
            &mut vc,
        );
        print!("got {} modes:\n", n_modes);
        i = 0;
        while i < n_modes {
            print!(
                "	value = {}, count = {}\n",
                (*vc.offset(i as isize)).v,
                (*vc.offset(i as isize)).c
            );
            i += 1;
            i;
        }
        free(vc as *mut libc::c_void);
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
