#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn asin(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct double_to_double {
    pub fn_0: Option::<
        unsafe extern "C" fn(*mut double_to_double, libc::c_double) -> libc::c_double,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct compose_functor {
    pub fn_0: Option::<
        unsafe extern "C" fn(*mut compose_functor, libc::c_double) -> libc::c_double,
    >,
    pub f: *mut double_to_double,
    pub g: *mut double_to_double,
}
#[no_mangle]
pub unsafe extern "C" fn compose_call(
    mut this: *mut compose_functor,
    mut x: libc::c_double,
) -> libc::c_double {
    return ((*(*this).f).fn_0)
        .expect(
            "non-null function pointer",
        )(
        (*this).f,
        ((*(*this).g).fn_0).expect("non-null function pointer")((*this).g, x),
    );
}
#[no_mangle]
pub unsafe extern "C" fn compose(
    mut f: *mut double_to_double,
    mut g: *mut double_to_double,
) -> *mut double_to_double {
    let mut result: *mut compose_functor = malloc(
        ::core::mem::size_of::<compose_functor>() as libc::c_ulong,
    ) as *mut compose_functor;
    (*result)
        .fn_0 = Some(
        compose_call
            as unsafe extern "C" fn(
                *mut compose_functor,
                libc::c_double,
            ) -> libc::c_double,
    );
    (*result).f = f;
    (*result).g = g;
    return result as *mut double_to_double;
}
#[no_mangle]
pub unsafe extern "C" fn sin_call(
    mut this: *mut double_to_double,
    mut x: libc::c_double,
) -> libc::c_double {
    return sin(x);
}
#[no_mangle]
pub unsafe extern "C" fn asin_call(
    mut this: *mut double_to_double,
    mut x: libc::c_double,
) -> libc::c_double {
    return asin(x);
}
unsafe fn main_0() -> libc::c_int {
    let mut my_sin: *mut double_to_double = malloc(
        ::core::mem::size_of::<double_to_double>() as libc::c_ulong,
    ) as *mut double_to_double;
    (*my_sin)
        .fn_0 = Some(
        sin_call
            as unsafe extern "C" fn(
                *mut double_to_double,
                libc::c_double,
            ) -> libc::c_double,
    );
    let mut my_asin: *mut double_to_double = malloc(
        ::core::mem::size_of::<double_to_double>() as libc::c_ulong,
    ) as *mut double_to_double;
    (*my_asin)
        .fn_0 = Some(
        asin_call
            as unsafe extern "C" fn(
                *mut double_to_double,
                libc::c_double,
            ) -> libc::c_double,
    );
    let mut sin_asin: *mut double_to_double = compose(my_sin, my_asin);
    printf(
        b"%f\n\0" as *const u8 as *const libc::c_char,
        ((*sin_asin).fn_0).expect("non-null function pointer")(sin_asin, 0.5f64),
    );
    free(sin_asin as *mut libc::c_void);
    free(my_sin as *mut libc::c_void);
    free(my_asin as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
