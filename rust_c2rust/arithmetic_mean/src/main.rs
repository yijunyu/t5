#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mean(
    mut v: *mut libc::c_double,
    mut len: libc::c_int,
) -> libc::c_double {
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        sum += *v.offset(i as isize);
        i += 1;
        i;
    }
    return sum / len as libc::c_double;
}
unsafe fn main_0() -> libc::c_int {
    let mut v: [libc::c_double; 5] = [
        1 as libc::c_int as libc::c_double,
        2 as libc::c_int as libc::c_double,
        2.718f64,
        3 as libc::c_int as libc::c_double,
        3.142f64,
    ];
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    len = 5 as libc::c_int;
    while len >= 0 as libc::c_int {
        printf(b"mean[\0" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while i < len {
            printf(
                if i != 0 {
                    b", %g\0" as *const u8 as *const libc::c_char
                } else {
                    b"%g\0" as *const u8 as *const libc::c_char
                },
                v[i as usize],
            );
            i += 1;
            i;
        }
        printf(
            b"] = %g\n\0" as *const u8 as *const libc::c_char,
            mean(v.as_mut_ptr(), len),
        );
        len -= 1;
        len;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
