#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn puts(__s: *const libc::c_char) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn damm(
    mut input: *mut libc::c_uchar,
    mut length: size_t,
) -> bool {
    static mut table: [[libc::c_uchar; 10]; 10] = [
        [
            0 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            7 as libc::c_int as libc::c_uchar,
            5 as libc::c_int as libc::c_uchar,
            9 as libc::c_int as libc::c_uchar,
            8 as libc::c_int as libc::c_uchar,
            6 as libc::c_int as libc::c_uchar,
            4 as libc::c_int as libc::c_uchar,
            2 as libc::c_int as libc::c_uchar,
        ],
        [
            7 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            9 as libc::c_int as libc::c_uchar,
            2 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            5 as libc::c_int as libc::c_uchar,
            4 as libc::c_int as libc::c_uchar,
            8 as libc::c_int as libc::c_uchar,
            6 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
        ],
        [
            4 as libc::c_int as libc::c_uchar,
            2 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            6 as libc::c_int as libc::c_uchar,
            8 as libc::c_int as libc::c_uchar,
            7 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
            5 as libc::c_int as libc::c_uchar,
            9 as libc::c_int as libc::c_uchar,
        ],
        [
            1 as libc::c_int as libc::c_uchar,
            7 as libc::c_int as libc::c_uchar,
            5 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            9 as libc::c_int as libc::c_uchar,
            8 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
            4 as libc::c_int as libc::c_uchar,
            2 as libc::c_int as libc::c_uchar,
            6 as libc::c_int as libc::c_uchar,
        ],
        [
            6 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            2 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            4 as libc::c_int as libc::c_uchar,
            5 as libc::c_int as libc::c_uchar,
            9 as libc::c_int as libc::c_uchar,
            7 as libc::c_int as libc::c_uchar,
            8 as libc::c_int as libc::c_uchar,
        ],
        [
            3 as libc::c_int as libc::c_uchar,
            6 as libc::c_int as libc::c_uchar,
            7 as libc::c_int as libc::c_uchar,
            4 as libc::c_int as libc::c_uchar,
            2 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            9 as libc::c_int as libc::c_uchar,
            5 as libc::c_int as libc::c_uchar,
            8 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
        ],
        [
            5 as libc::c_int as libc::c_uchar,
            8 as libc::c_int as libc::c_uchar,
            6 as libc::c_int as libc::c_uchar,
            9 as libc::c_int as libc::c_uchar,
            7 as libc::c_int as libc::c_uchar,
            2 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
            4 as libc::c_int as libc::c_uchar,
        ],
        [
            8 as libc::c_int as libc::c_uchar,
            9 as libc::c_int as libc::c_uchar,
            4 as libc::c_int as libc::c_uchar,
            5 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
            6 as libc::c_int as libc::c_uchar,
            2 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            7 as libc::c_int as libc::c_uchar,
        ],
        [
            9 as libc::c_int as libc::c_uchar,
            4 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
            8 as libc::c_int as libc::c_uchar,
            6 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            7 as libc::c_int as libc::c_uchar,
            2 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            5 as libc::c_int as libc::c_uchar,
        ],
        [
            2 as libc::c_int as libc::c_uchar,
            5 as libc::c_int as libc::c_uchar,
            8 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            4 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
            6 as libc::c_int as libc::c_uchar,
            7 as libc::c_int as libc::c_uchar,
            9 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
        ],
    ];
    let mut interim: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < length {
        interim = table[interim as usize][*input.offset(i as isize) as usize];
        i = i.wrapping_add(1);
        i;
    }
    return interim as libc::c_int == 0 as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut input: [libc::c_uchar; 4] = [
        5 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
    ];
    puts(
        if damm(input.as_mut_ptr(), 4 as libc::c_int as size_t) as libc::c_int != 0 {
            b"Checksum correct\0" as *const u8 as *const libc::c_char
        } else {
            b"Checksum incorrect\0" as *const u8 as *const libc::c_char
        },
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
