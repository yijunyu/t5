#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn time(__timer: *mut time_t) -> time_t;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
unsafe fn main_0() -> libc::c_int {
    let mut a: [[libc::c_int; 10]; 10] = [[0; 10]; 10];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    srand(time(0 as *mut time_t) as libc::c_uint);
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 10 as libc::c_int {
            a[i as usize][j as usize] = rand() % 20 as libc::c_int + 1 as libc::c_int;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    's_32: while i < 10 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 10 as libc::c_int {
            printf(
                b" %d\0" as *const u8 as *const libc::c_char,
                a[i as usize][j as usize],
            );
            if a[i as usize][j as usize] == 20 as libc::c_int {
                break 's_32;
            }
            j += 1;
            j;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
