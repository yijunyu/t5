#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn isBal(
    mut s: *const libc::c_char,
    mut l: libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh0 = l;
        l = l - 1;
        if !(fresh0 != 0) {
            break;
        }
        if *s.offset(l as isize) as libc::c_int == ']' as i32 {
            c += 1;
            c;
        } else {
            if !(*s.offset(l as isize) as libc::c_int == '[' as i32) {
                continue;
            }
            c -= 1;
            if c < 0 as libc::c_int {
                break;
            }
        }
    }
    return (c == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn shuffle(mut s: *mut libc::c_char, mut h: libc::c_int) {
    let mut x: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut i: libc::c_int = h;
    loop {
        let fresh1 = i;
        i = i - 1;
        if !(fresh1 != 0) {
            break;
        }
        x = rand() % h;
        t = *s.offset(x as isize) as libc::c_int;
        *s.offset(x as isize) = *s.offset(i as isize);
        *s.offset(i as isize) = t as libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn genSeq(mut s: *mut libc::c_char, mut n: libc::c_int) {
    if n != 0 {
        memset(s as *mut libc::c_void, '[' as i32, n as libc::c_ulong);
        memset(
            s.offset(n as isize) as *mut libc::c_void,
            ']' as i32,
            n as libc::c_ulong,
        );
        shuffle(s, n * 2 as libc::c_int);
    }
    *s.offset((n * 2 as libc::c_int) as isize) = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn doSeq(mut n: libc::c_int) {
    let mut s: [libc::c_char; 64] = [0; 64];
    let mut o: *const libc::c_char = b"False\0" as *const u8 as *const libc::c_char;
    genSeq(s.as_mut_ptr(), n);
    if isBal(s.as_mut_ptr(), n * 2 as libc::c_int) != 0 {
        o = b"True\0" as *const u8 as *const libc::c_char;
    }
    printf(b"'%s': %s\n\0" as *const u8 as *const libc::c_char, s.as_mut_ptr(), o);
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0 as libc::c_int;
    while n < 9 as libc::c_int {
        let fresh2 = n;
        n = n + 1;
        doSeq(fresh2);
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
