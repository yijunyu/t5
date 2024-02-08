#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.offset(str_size as isize) != 0 {
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}

use c2rust_out::*;
extern "C" {
    fn rand() -> i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
}
#[no_mangle]
pub extern "C" fn isBal(mut s: *const i8, mut l: i32) -> i32 {
    unsafe {
        let mut c: i32 = 0;
        loop {
            let fresh0 = l;
            l = l - 1;
            if !(fresh0 != 0) {
                break;
            }
            if *s.offset(l as isize) as i32 == ']' as i32 {
                c += 1;
                c;
            } else {
                if !(*s.offset(l as isize) as i32 == '[' as i32) {
                    continue;
                }
                c -= 1;
                if c < 0 {
                    break;
                }
            }
        }
        return (c == 0) as i32;
    }
}

#[no_mangle]
pub extern "C" fn shuffle(mut s: *mut i8, mut h: i32) {
    unsafe {
        let mut x: i32 = 0;
        let mut t: i32 = 0;
        let mut i: i32 = h;
        loop {
            let fresh1 = i;
            i = i - 1;
            if !(fresh1 != 0) {
                break;
            }
            x = rand() % h;
            t = *s.offset(x as isize) as i32;
            *s.offset(x as isize) = *s.offset(i as isize);
            *s.offset(i as isize) = t as i8;
        }
    }
}

#[no_mangle]
pub extern "C" fn genSeq(mut s: *mut i8, mut n: i32) {
    unsafe {
        if n != 0 {
            memset(s as *mut libc::c_void, '[' as i32, n as u64);
            memset(
                s.offset(n as isize) as *mut libc::c_void,
                ']' as i32,
                n as u64,
            );
            shuffle(s, n * 2);
        };
        *s.offset((n * 2i32) as isize) = 0;
    }
}

#[no_mangle]
pub extern "C" fn doSeq(mut n: i32) {
    unsafe {
        let mut s: [i8; 64] = [0; 64];
        let mut o: *const i8 = b"False\0" as *const u8 as *const i8;
        genSeq(s.as_mut_ptr(), n);
        if isBal(s.as_mut_ptr(), n * 2) != 0 {
            o = b"True\0" as *const u8 as *const i8;
        }
        print!(
            "{}: {}\n",
            build_str_from_raw_ptr(s.as_mut_ptr() as *mut u8),
            build_str_from_raw_ptr(o as *mut u8)
        );
    }
}

fn main_0() -> i32 {
    let mut n: i32 = 0;
    while n < 9 {
        let fresh2 = n;
        n = n + 1;
        doSeq(fresh2);
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
