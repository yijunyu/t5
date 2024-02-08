#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(c_variadic)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sma_obj {
    pub sma: libc::c_double,
    pub sum: libc::c_double,
    pub period: libc::c_int,
    pub values: *mut libc::c_double,
    pub lv: libc::c_int,
}
pub type sma_obj_t = sma_obj;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sma_result {
    pub handle: *mut sma_obj_t,
    pub sma: libc::c_double,
    pub values: *mut libc::c_double,
}
pub type sma_result_t = sma_result;
pub type Action = libc::c_uint;
pub const SMA_MEAN: Action = 4;
pub const SMA_ADD: Action = 3;
pub const SMA_VALUES: Action = 2;
pub const SMA_FREE: Action = 1;
pub const SMA_NEW: Action = 0;
#[no_mangle]
pub unsafe extern "C" fn sma(mut action: Action, mut args: ...) -> sma_result_t {
    let mut vl: ::core::ffi::VaListImpl;
    let mut r: sma_result_t = sma_result {
        handle: 0 as *mut sma_obj_t,
    };
    let mut o: *mut sma_obj_t = 0 as *mut sma_obj_t;
    let mut v_0: libc::c_double = 0.;
    vl = args.clone();
    match action as libc::c_uint {
        0 => {
            r
                .handle = malloc(::core::mem::size_of::<sma_obj_t>() as libc::c_ulong)
                as *mut sma_obj_t;
            (*r.handle).sma = 0.0f64;
            (*r.handle).period = vl.arg::<libc::c_int>();
            (*r.handle)
                .values = malloc(
                ((*r.handle).period as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_double;
            (*r.handle).lv = 0 as libc::c_int;
            (*r.handle).sum = 0.0f64;
        }
        1 => {
            r.handle = vl.arg::<*mut sma_obj_t>();
            free((*r.handle).values as *mut libc::c_void);
            free(r.handle as *mut libc::c_void);
            r.handle = 0 as *mut sma_obj_t;
        }
        2 => {
            o = vl.arg::<*mut sma_obj_t>();
            r.values = (*o).values;
        }
        4 => {
            o = vl.arg::<*mut sma_obj_t>();
            r.sma = (*o).sma;
        }
        3 => {
            o = vl.arg::<*mut sma_obj_t>();
            v_0 = vl.arg::<libc::c_double>();
            if (*o).lv < (*o).period {
                let fresh0 = (*o).lv;
                (*o).lv = (*o).lv + 1;
                *((*o).values).offset(fresh0 as isize) = v_0;
                (*o).sum += v_0;
                (*o).sma = (*o).sum / (*o).lv as libc::c_double;
            } else {
                (*o).sum -= *((*o).values).offset(((*o).lv % (*o).period) as isize);
                (*o).sum += v_0;
                (*o).sma = (*o).sum / (*o).period as libc::c_double;
                *((*o).values).offset(((*o).lv % (*o).period) as isize) = v_0;
                (*o).lv += 1;
                (*o).lv;
            }
            r.sma = (*o).sma;
        }
        _ => {}
    }
    return r;
}
#[no_mangle]
pub static mut v: [libc::c_double; 10] = [
    1 as libc::c_int as libc::c_double,
    2 as libc::c_int as libc::c_double,
    3 as libc::c_int as libc::c_double,
    4 as libc::c_int as libc::c_double,
    5 as libc::c_int as libc::c_double,
    5 as libc::c_int as libc::c_double,
    4 as libc::c_int as libc::c_double,
    3 as libc::c_int as libc::c_double,
    2 as libc::c_int as libc::c_double,
    1 as libc::c_int as libc::c_double,
];
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut h3: *mut sma_obj_t = (sma(SMA_NEW, 3 as libc::c_int)).handle;
    let mut h5: *mut sma_obj_t = (sma(SMA_NEW, 5 as libc::c_int)).handle;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[libc::c_double; 10]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
    {
        printf(
            b"next number %lf, SMA_3 = %lf, SMA_5 = %lf\n\0" as *const u8
                as *const libc::c_char,
            v[i as usize],
            (sma(SMA_ADD, h3, v[i as usize])).sma,
            (sma(SMA_ADD, h5, v[i as usize])).sma,
        );
        i += 1;
        i;
    }
    sma(SMA_FREE, h3);
    sma(SMA_FREE, h5);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
