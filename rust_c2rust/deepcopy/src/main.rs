#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct layer1 {
    pub a: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct layer2 {
    pub l1: layer1,
    pub b: libc::c_float,
    pub c: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct layer3 {
    pub l2: layer2,
    pub l1: layer1,
    pub d: libc::c_int,
    pub e: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn showCake(mut cake: layer3) {
    printf(b"\ncake.d = %d\0" as *const u8 as *const libc::c_char, cake.d);
    printf(b"\ncake.e = %d\0" as *const u8 as *const libc::c_char, cake.e);
    printf(b"\ncake.l1.a = %d\0" as *const u8 as *const libc::c_char, cake.l1.a);
    printf(
        b"\ncake.l2.b = %f\0" as *const u8 as *const libc::c_char,
        cake.l2.b as libc::c_double,
    );
    printf(b"\ncake.l2.l1.a = %d\0" as *const u8 as *const libc::c_char, cake.l2.l1.a);
}
unsafe fn main_0() -> libc::c_int {
    let mut cake1: layer3 = layer3 {
        l2: layer2 {
            l1: layer1 { a: 0 },
            b: 0.,
            c: 0.,
        },
        l1: layer1 { a: 0 },
        d: 0,
        e: 0,
    };
    let mut cake2: layer3 = layer3 {
        l2: layer2 {
            l1: layer1 { a: 0 },
            b: 0.,
            c: 0.,
        },
        l1: layer1 { a: 0 },
        d: 0,
        e: 0,
    };
    cake1.d = 1 as libc::c_int;
    cake1.e = 2 as libc::c_int;
    cake1.l1.a = 3 as libc::c_int;
    cake1.l2.b = 4 as libc::c_int as libc::c_float;
    cake1.l2.l1.a = 5 as libc::c_int;
    printf(b"Cake 1 is : \0" as *const u8 as *const libc::c_char);
    showCake(cake1);
    cake2 = cake1;
    cake2.l2.b += cake2.l2.l1.a as libc::c_float;
    printf(b"\nCake 2 is : \0" as *const u8 as *const libc::c_char);
    showCake(cake2);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
