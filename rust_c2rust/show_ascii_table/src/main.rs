#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: [libc::c_char; 4] = [0; 4];
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        j = 32 as libc::c_int + i;
        while j < 128 as libc::c_int {
            match j {
                32 => {
                    sprintf(
                        k.as_mut_ptr(),
                        b"Spc\0" as *const u8 as *const libc::c_char,
                    );
                }
                127 => {
                    sprintf(
                        k.as_mut_ptr(),
                        b"Del\0" as *const u8 as *const libc::c_char,
                    );
                }
                _ => {
                    sprintf(
                        k.as_mut_ptr(),
                        b"%c\0" as *const u8 as *const libc::c_char,
                        j,
                    );
                }
            }
            printf(
                b"%3d : %-3s   \0" as *const u8 as *const libc::c_char,
                j,
                k.as_mut_ptr(),
            );
            j += 16 as libc::c_int;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
