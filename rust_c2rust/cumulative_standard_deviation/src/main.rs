#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type Action = libc::c_uint;
pub const COUNT: Action = 3;
pub const VAR: Action = 2;
pub const MEAN: Action = 1;
pub const STDDEV: Action = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat_obj_struct {
    pub sum: libc::c_double,
    pub sum2: libc::c_double,
    pub num: size_t,
    pub action: Action,
}
pub type sStatObject = stat_obj_struct;
pub type StatObject = *mut stat_obj_struct;
#[no_mangle]
pub unsafe extern "C" fn NewStatObject(mut action: Action) -> StatObject {
    let mut so: StatObject = 0 as *mut stat_obj_struct;
    so = malloc(::core::mem::size_of::<sStatObject>() as libc::c_ulong) as StatObject;
    (*so).sum = 0.0f64;
    (*so).sum2 = 0.0f64;
    (*so).num = 0 as libc::c_int as size_t;
    (*so).action = action;
    return so;
}
#[no_mangle]
pub unsafe extern "C" fn stat_obj_value(
    mut so: StatObject,
    mut action: Action,
) -> libc::c_double {
    let mut num: libc::c_double = 0.;
    let mut mean: libc::c_double = 0.;
    let mut var: libc::c_double = 0.;
    let mut stddev: libc::c_double = 0.;
    if (*so).num as libc::c_double == 0.0f64 {
        return 0.0f64;
    }
    num = (*so).num as libc::c_double;
    if action as libc::c_uint == COUNT as libc::c_int as libc::c_uint {
        return num;
    }
    mean = (*so).sum / num;
    if action as libc::c_uint == MEAN as libc::c_int as libc::c_uint {
        return mean;
    }
    var = (*so).sum2 / num - mean * mean;
    if action as libc::c_uint == VAR as libc::c_int as libc::c_uint {
        return var;
    }
    stddev = sqrt(var);
    if action as libc::c_uint == STDDEV as libc::c_int as libc::c_uint {
        return stddev;
    }
    return 0 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn stat_object_add(
    mut so: StatObject,
    mut v_0: libc::c_double,
) -> libc::c_double {
    (*so).num = ((*so).num).wrapping_add(1);
    (*so).num;
    (*so).sum += v_0;
    (*so).sum2 += v_0 * v_0;
    return stat_obj_value(so, (*so).action);
}
#[no_mangle]
pub static mut v: [libc::c_double; 8] = [
    2 as libc::c_int as libc::c_double,
    4 as libc::c_int as libc::c_double,
    4 as libc::c_int as libc::c_double,
    4 as libc::c_int as libc::c_double,
    5 as libc::c_int as libc::c_double,
    5 as libc::c_int as libc::c_double,
    7 as libc::c_int as libc::c_double,
    9 as libc::c_int as libc::c_double,
];
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut so: StatObject = NewStatObject(STDDEV);
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[libc::c_double; 8]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
    {
        printf(
            b"val: %lf  std dev: %lf\n\0" as *const u8 as *const libc::c_char,
            v[i as usize],
            stat_object_add(so, v[i as usize]),
        );
        i += 1;
        i;
    }
    free(so as *mut libc::c_void);
    so = 0 as StatObject;
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
