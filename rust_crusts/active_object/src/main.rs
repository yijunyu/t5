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
    fn sleep(__seconds: u32) -> u32;
    fn usleep(__useconds: u32) -> i32;
    fn atan2(_: f64, _: f64) -> f64;
    fn sin(_: f64) -> f64;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> i32;
    fn pthread_create(
        __newthread: *mut u64,
        __attr: *const pthread_attr_t,
        __start_routine: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        __arg: *mut libc::c_void,
    ) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [i8; 56],
    pub __align: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct integ_t {
    pub func: Option<unsafe extern "C" fn(f64) -> f64>,
    pub start: timeval,
    pub v: f64,
    pub last_v: f64,
    pub last_t: f64,
    pub id: u64,
}
pub type integ = *mut integ_t;
#[no_mangle]
pub extern "C" fn update(mut x: integ) {
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut t: f64 = 0.;
    let mut v: f64 = 0.;
    unsafe {
        let mut f: Option<unsafe extern "C" fn(f64) -> f64> = None;
        f = (*x).func;
        gettimeofday(&mut tv, 0 as *mut libc::c_void);
        t = ((tv.tv_sec - (*x).start.tv_sec) * 1000000 + tv.tv_usec - (*x).start.tv_usec) as f64
            * 1e-6f64;
        v = if f.is_some() {
            f.expect("non-null function pointer")(t)
        } else {
            0 as f64
        };
    }
    (*x).v += ((*x).last_v + v) * (t - (*x).last_t) / 2 as f64;
    (*x).last_t = t;
}

#[no_mangle]
pub extern "C" fn tick(mut a: *mut libc::c_void) -> *mut libc::c_void {
    unsafe {
        let mut x: integ = a as integ;
        loop {
            usleep(100000);
            update(x);
        }
    }
}

#[no_mangle]
pub extern "C" fn set_input(mut x: integ, mut func: Option<unsafe extern "C" fn(f64) -> f64>) {
    update(x);
    unsafe {
        (*x).func = func;
    }
    (*x).last_t = 0 as f64;
    unsafe {
        (*x).last_v = if func.is_some() {
            func.expect("non-null function pointer")(0 as f64)
        } else {
            0 as f64
        };
    }
}

#[no_mangle]
pub extern "C" fn new_integ(mut func: Option<unsafe extern "C" fn(f64) -> f64>) -> integ {
    unsafe {
        let mut x: integ = malloc(::core::mem::size_of::<integ_t>() as u64) as integ;
        (*x).last_v = 0 as f64;
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
}

#[no_mangle]
pub extern "C" fn sine(mut t: f64) -> f64 {
    unsafe {
        return sin(4 as f64 * atan2(1 as f64, 1 as f64) * t);
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut x: integ = new_integ(Some(sine as unsafe extern "C" fn(f64) -> f64));
        sleep(2);
        set_input(x, None);
        usleep(500000);
        print!("{}\n", (*x).v);
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
