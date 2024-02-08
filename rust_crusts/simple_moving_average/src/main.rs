#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic)]
use c2rust_out::*;
extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sma_obj {
    pub sma: f64,
    pub sum: f64,
    pub period: i32,
    pub values: *mut f64,
    pub lv: i32,
}
pub type sma_obj_t = sma_obj;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sma_result {
    pub handle: *mut sma_obj_t,
    pub sma: f64,
    pub values: *mut f64,
}
pub type sma_result_t = sma_result;
pub const SMA_MEAN: u32 = 4;
pub const SMA_ADD: u32 = 3;
pub const SMA_VALUES: u32 = 2;
pub const SMA_FREE: u32 = 1;
pub const SMA_NEW: u32 = 0;
#[no_mangle]
pub unsafe extern "C" fn sma(mut action: u32, mut args: ...) -> sma_result_t {
    let mut vl: ::core::ffi::VaListImpl;
    let mut r: sma_result_t = sma_result {
        handle: 0 as *mut sma_obj_t,
    };
    let mut o: *mut sma_obj_t = 0 as *mut sma_obj_t;
    let mut v_0: f64 = 0.;
    vl = args.clone();
    match action as u32 {
        0 => {
            r.handle = malloc(::core::mem::size_of::<sma_obj_t>() as u64) as *mut sma_obj_t;
            (*r.handle).sma = 0.0f64;
            (*r.handle).period = vl.arg::<i32>();
            (*r.handle).values = malloc(
                ((*r.handle).period as u64).wrapping_mul(::core::mem::size_of::<f64>() as u64),
            ) as *mut f64;
            (*r.handle).lv = 0;
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
            v_0 = vl.arg::<f64>();
            if (*o).lv < (*o).period {
                let fresh0 = (*o).lv;
                (*o).lv = (*o).lv + 1;
                *((*o).values).offset(fresh0 as isize) = v_0;
                (*o).sum += v_0;
                (*o).sma = (*o).sum / (*o).lv as f64;
            } else {
                (*o).sum -= *((*o).values).offset(((*o).lv % (*o).period) as isize);
                (*o).sum += v_0;
                (*o).sma = (*o).sum / (*o).period as f64;
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
pub static mut v: [f64; 10] = [
    1 as f64, 2 as f64, 3 as f64, 4 as f64, 5 as f64, 5 as f64, 4 as f64, 3 as f64, 2 as f64,
    1 as f64,
];
fn main_0() -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut h3: *mut sma_obj_t = (sma(SMA_NEW, 3)).handle;
        let mut h5: *mut sma_obj_t = (sma(SMA_NEW, 5)).handle;
        i = 0;
        while (i as u64)
            < (::core::mem::size_of::<[f64; 10]>() as u64)
                .wrapping_div(::core::mem::size_of::<f64>() as u64)
        {
            print!(
                "next number {}, SMA_3 = {}, SMA_5 = {}\n",
                v[i as usize],
                (sma(SMA_ADD, h3, v[i as usize])).sma,
                (sma(SMA_ADD, h5, v[i as usize])).sma
            );
            i += 1;
            i;
        }
        sma(SMA_FREE, h3);
        sma(SMA_FREE, h5);
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
