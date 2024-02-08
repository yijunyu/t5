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
    fn sqrt(_: f64) -> f64;
}
pub const COUNT: u32 = 3;
pub const VAR: u32 = 2;
pub const MEAN: u32 = 1;
pub const STDDEV: u32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat_obj_struct {
    pub sum: f64,
    pub sum2: f64,
    pub num: u64,
    pub action: u32,
}
pub type sStatObject = stat_obj_struct;
pub type StatObject = *mut stat_obj_struct;
#[no_mangle]
pub extern "C" fn NewStatObject(mut action: u32) -> StatObject {
    let mut so: StatObject = 0 as *mut stat_obj_struct;
    unsafe {
        so = malloc(::core::mem::size_of::<sStatObject>() as u64) as StatObject;
    }
    (*so).sum = 0.0f64;
    (*so).sum2 = 0.0f64;
    (*so).num = 0;
    (*so).action = action;
    return so;
}

#[no_mangle]
pub extern "C" fn stat_obj_value(mut so: StatObject, mut action: u32) -> f64 {
    let mut num: f64 = 0.;
    let mut mean: f64 = 0.;
    let mut var: f64 = 0.;
    let mut stddev: f64 = 0.;
    if (*so).num as f64 == 0.0f64 {
        return 0.0f64;
    }
    num = (*so).num as f64;
    if action as u32 == COUNT as u32 {
        return num;
    }
    mean = (*so).sum / num;
    if action as u32 == MEAN as u32 {
        return mean;
    }
    var = (*so).sum2 / num - mean * mean;
    if action as u32 == VAR as u32 {
        return var;
    }
    unsafe {
        stddev = sqrt(var);
    }
    if action as u32 == STDDEV as u32 {
        return stddev;
    }
    return 0 as f64;
}

#[no_mangle]
pub extern "C" fn stat_object_add(mut so: StatObject, mut v_0: f64) -> f64 {
    (*so).num = ((*so).num).wrapping_add(1);
    (*so).num;
    (*so).sum += v_0;
    (*so).sum2 += v_0 * v_0;
    return stat_obj_value(so, (*so).action);
}

#[no_mangle]
pub static mut v: [f64; 8] = [
    2 as f64, 4 as f64, 4 as f64, 4 as f64, 5 as f64, 5 as f64, 7 as f64, 9 as f64,
];
fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut so: StatObject = NewStatObject(STDDEV);
    i = 0;
    unsafe {
        while (i as u64)
            < (::core::mem::size_of::<[f64; 8]>() as u64)
                .wrapping_div(::core::mem::size_of::<f64>() as u64)
        {
            print!(
                "val: {}  std dev: {}\n",
                v[i as usize],
                stat_object_add(so, v[i as usize])
            );
            i += 1;
            i;
        }
        free(so as *mut libc::c_void);
    }
    so = 0 as StatObject;
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
