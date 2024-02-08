#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn transpose(
    mut dest: *mut libc::c_void,
    mut src: *mut libc::c_void,
    mut src_h: libc::c_int,
    mut src_w: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let vla = src_h as usize;
    let mut d: *mut libc::c_double = dest as *mut libc::c_double;
    let vla_0 = src_w as usize;
    let mut s: *mut libc::c_double = src as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < src_h {
        j = 0 as libc::c_int;
        while j < src_w {
            *d
                .offset(j as isize * vla as isize)
                .offset(
                    i as isize,
                ) = *s.offset(i as isize * vla_0 as isize).offset(j as isize);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut a: [[libc::c_double; 5]; 3] = [
        [
            0 as libc::c_int as libc::c_double,
            1 as libc::c_int as libc::c_double,
            2 as libc::c_int as libc::c_double,
            3 as libc::c_int as libc::c_double,
            4 as libc::c_int as libc::c_double,
        ],
        [
            5 as libc::c_int as libc::c_double,
            6 as libc::c_int as libc::c_double,
            7 as libc::c_int as libc::c_double,
            8 as libc::c_int as libc::c_double,
            9 as libc::c_int as libc::c_double,
        ],
        [
            1 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            42 as libc::c_int as libc::c_double,
        ],
    ];
    let mut b: [[libc::c_double; 3]; 5] = [[0.; 3]; 5];
    transpose(
        b.as_mut_ptr() as *mut libc::c_void,
        a.as_mut_ptr() as *mut libc::c_void,
        3 as libc::c_int,
        5 as libc::c_int,
    );
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            printf(
                b"%g%c\0" as *const u8 as *const libc::c_char,
                b[i as usize][j as usize],
                if j == 2 as libc::c_int { '\n' as i32 } else { ' ' as i32 },
            );
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
