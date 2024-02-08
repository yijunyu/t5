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
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn SHA256(d: *const u8, n: u64, md: *mut u8) -> *mut u8;
}
#[no_mangle]
pub static mut coin_err: *const i8 = 0 as *const i8;
#[no_mangle]
pub extern "C" fn unbase58(mut s: *const i8, mut out: *mut u8) -> i32 {
    unsafe {
        static mut tmpl: *const i8 = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz\0"
            as *const u8 as *const i8;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut c: i32 = 0;
        let mut p: *const i8 = 0 as *const i8;
        memset(out as *mut libc::c_void, 0, 25);
        i = 0;
        while *s.offset(i as isize) != 0 {
            p = strchr(tmpl, *s.offset(i as isize) as i32);
            if p.is_null() {
                coin_err = b"bad char\0" as *const u8 as *const i8;
                return 0;
            }
            c = p.offset_from(tmpl) as i32;
            j = 25;
            loop {
                let fresh0 = j;
                j = j - 1;
                if !(fresh0 != 0) {
                    break;
                }
                c += 58 * *out.offset(j as isize) as i32;
                *out.offset(j as isize) = (c % 256i32) as u8;
                c /= 256;
            }
            if c != 0 {
                coin_err = b"address too long\0" as *const u8 as *const i8;
                return 0;
            }
            i += 1;
            i;
        }
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn valid(mut s: *const i8) -> i32 {
    unsafe {
        let mut dec: [u8; 32] = [0; 32];
        let mut d1: [u8; 32] = [0; 32];
        let mut d2: [u8; 32] = [0; 32];
        coin_err = b"\0" as *const u8 as *const i8;
        if unbase58(s, dec.as_mut_ptr()) == 0 {
            return 0;
        }
        SHA256(
            SHA256(dec.as_mut_ptr(), 21, d1.as_mut_ptr()),
            32,
            d2.as_mut_ptr(),
        );
        if memcmp(
            dec.as_mut_ptr().offset(21 as isize) as *const libc::c_void,
            d2.as_mut_ptr() as *const libc::c_void,
            4,
        ) != 0
        {
            coin_err = b"bad digest\0" as *const u8 as *const i8;
            return 0;
        }
        return 1;
    }
}

fn main_0() -> i32 {
    let mut s: [*const i8; 5] = [
        b"1Q1pE5vPGEEMqRcVRMbtBK842Y6Pzo6nK9\0" as *const u8 as *const i8,
        b"1AGNa15ZQXAZUgFiqJ2i7Z2DPU2J6hW62i\0" as *const u8 as *const i8,
        b"1Q1pE5vPGEEMqRcVRMbtBK842Y6Pzo6nJ9\0" as *const u8 as *const i8,
        b"1AGNa15ZQXAZUgFiqJ2i7Z2DPU2J6hW62I\0" as *const u8 as *const i8,
        0 as *const i8,
    ];
    let mut i: i32 = 0;
    i = 0;
    unsafe {
        while !(s[i as usize]).is_null() {
            let mut status: i32 = valid(s[i as usize]);
            if status != 0 {
                print!(
                    "{}: {}\n",
                    build_str_from_raw_ptr(s[i as usize] as *mut u8),
                    "Ok\0"
                )
            } else {
                print!(
                    "{}: {}\n",
                    build_str_from_raw_ptr(s[i as usize] as *mut u8),
                    build_str_from_raw_ptr(coin_err as *mut u8)
                )
            };
            i += 1;
            i;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
