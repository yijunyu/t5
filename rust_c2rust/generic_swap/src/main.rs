#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct test {
    pub a: libc::c_int,
    pub b: libc::c_int,
    pub c: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn swap(
    mut va: *mut libc::c_void,
    mut vb: *mut libc::c_void,
    mut s: size_t,
) {
    let mut t: libc::c_char = 0;
    let mut a: *mut libc::c_char = va as *mut libc::c_char;
    let mut b: *mut libc::c_char = vb as *mut libc::c_char;
    loop {
        let fresh0 = s;
        s = s.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        t = *a.offset(s as isize);
        *a.offset(s as isize) = *b.offset(s as isize);
        *b.offset(s as isize) = t;
    };
}
unsafe fn main_0() -> libc::c_int {
    let mut t: test = {
        let mut init = test {
            a: 1 as libc::c_int,
            b: 2 as libc::c_int,
            c: 3 as libc::c_int,
        };
        init
    };
    let mut h: test = {
        let mut init = test {
            a: 4 as libc::c_int,
            b: 5 as libc::c_int,
            c: 6 as libc::c_int,
        };
        init
    };
    let mut alfa: libc::c_double = 0.45f64;
    let mut omega: libc::c_double = 9.98f64;
    let mut pt: *mut test = &mut t;
    let mut th: *mut test = &mut h;
    printf(b"%d %d %d\n\0" as *const u8 as *const libc::c_char, t.a, t.b, t.c);
    let mut _T: test = t;
    t = h;
    h = _T;
    printf(b"%d %d %d\n\0" as *const u8 as *const libc::c_char, t.a, t.b, t.c);
    printf(b"%d %d %d\n\0" as *const u8 as *const libc::c_char, h.a, h.b, h.c);
    printf(b"%lf\n\0" as *const u8 as *const libc::c_char, alfa);
    let mut _T_0: libc::c_double = alfa;
    alfa = omega;
    omega = _T_0;
    printf(b"%lf\n\0" as *const u8 as *const libc::c_char, alfa);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, (*pt).a);
    let mut _T_1: *mut test = pt;
    pt = th;
    th = _T_1;
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, (*pt).a);
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
