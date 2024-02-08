#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
}
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct integ_t {
    pub func: Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
    pub start: timeval,
    pub v: libc::c_double,
    pub last_v: libc::c_double,
    pub last_t: libc::c_double,
    pub id: pthread_t,
}
pub type integ = *mut integ_t;
#[no_mangle]
pub unsafe extern "C" fn update(mut x: integ) {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut t: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut f: Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double> = None;
    f = (*x).func;
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    t = ((tv.tv_sec - (*x).start.tv_sec) * 1000000 as libc::c_int as libc::c_long
        + tv.tv_usec - (*x).start.tv_usec) as libc::c_double * 1e-6f64;
    v = if f.is_some() {
        f.expect("non-null function pointer")(t)
    } else {
        0 as libc::c_int as libc::c_double
    };
    (*x).v += ((*x).last_v + v) * (t - (*x).last_t) / 2 as libc::c_int as libc::c_double;
    (*x).last_t = t;
}
#[no_mangle]
pub unsafe extern "C" fn tick(mut a: *mut libc::c_void) -> *mut libc::c_void {
    let mut x: integ = a as integ;
    loop {
        usleep(100000 as libc::c_int as __useconds_t);
        update(x);
    };
}
#[no_mangle]
pub unsafe extern "C" fn set_input(
    mut x: integ,
    mut func: Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
) {
    update(x);
    (*x).func = func;
    (*x).last_t = 0 as libc::c_int as libc::c_double;
    (*x)
        .last_v = if func.is_some() {
        func.expect("non-null function pointer")(0 as libc::c_int as libc::c_double)
    } else {
        0 as libc::c_int as libc::c_double
    };
}
#[no_mangle]
pub unsafe extern "C" fn new_integ(
    mut func: Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
) -> integ {
    let mut x: integ = malloc(::core::mem::size_of::<integ_t>() as libc::c_ulong)
        as integ;
    (*x).last_v = 0 as libc::c_int as libc::c_double;
    (*x).v = (*x).last_v;
    (*x).func = None;
    gettimeofday(&mut (*x).start, 0 as *mut libc::c_void);
    set_input(x, func);
    pthread_create(
        &mut (*x).id,
        0 as *const pthread_attr_t,
        Some(tick as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        x as *mut libc::c_void,
    );
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn sine(mut t: libc::c_double) -> libc::c_double {
    return sin(
        4 as libc::c_int as libc::c_double
            * atan2(
                1 as libc::c_int as libc::c_double,
                1 as libc::c_int as libc::c_double,
            ) * t,
    );
}
unsafe fn main_0() -> libc::c_int {
    let mut x: integ = new_integ(
        Some(sine as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
    );
    sleep(2 as libc::c_int as libc::c_uint);
    set_input(x, None);
    usleep(500000 as libc::c_int as __useconds_t);
    printf(b"%g\n\0" as *const u8 as *const libc::c_char, (*x).v);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
