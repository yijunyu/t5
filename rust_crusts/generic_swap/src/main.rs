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
extern "C" {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct test {
    pub a: i32,
    pub b: i32,
    pub c: i32,
}
#[no_mangle]
pub extern "C" fn swap(mut va: *mut libc::c_void, mut vb: *mut libc::c_void, mut s: u64) {
    unsafe {
        let mut t: i8 = 0;
        let mut a: *mut i8 = va as *mut i8;
        let mut b: *mut i8 = vb as *mut i8;
        loop {
            let fresh0 = s;
            s = s.wrapping_sub(1);
            if !(fresh0 != 0) {
                break;
            }
            t = *a.offset(s as isize);
            *a.offset(s as isize) = *b.offset(s as isize);
            *b.offset(s as isize) = t;
        }
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut t: test = {
            let mut init = test { a: 1, b: 2, c: 3 };
            init
        };
        let mut h: test = {
            let mut init = test { a: 4, b: 5, c: 6 };
            init
        };
        let mut alfa: f64 = 0.45f64;
        let mut omega: f64 = 9.98f64;
        let mut pt: *mut test = &mut t;
        let mut th: *mut test = &mut h;
        print!("{} {} {}\n", t.a, t.b, t.c);
        let mut _T: test = t;
        t = h;
        h = _T;
        print!("{} {} {}\n", t.a, t.b, t.c);
        print!("{} {} {}\n", h.a, h.b, h.c);
        print!("{}\n", alfa);
        let mut _T_0: f64 = alfa;
        alfa = omega;
        omega = _T_0;
        print!("{}\n", alfa);
        print!("{}\n", (*pt).a);
        let mut _T_1: *mut test = pt;
        pt = th;
        th = _T_1;
        print!("{}\n", (*pt).a);
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
