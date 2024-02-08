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
    fn strchr(_: *const i8, _: i32) -> *mut i8;
}
#[no_mangle]
pub extern "C" fn is_pangram(mut s: *const i8) -> i32 {
    unsafe {
        let mut alpha: *const i8 =
            b"abcdefghjiklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ\0" as *const u8 as *const i8;
        let mut ch: i8 = 0;
        let mut wasused: [i8; 26] = [0; 26];
        let mut total: i32 = 0;
        loop {
            let fresh0 = s;
            s = s.offset(1);
            ch = *fresh0;
            if !(ch as i32 != '\0' as i32) {
                break;
            }
            let mut p: *const i8 = 0 as *const i8;
            let mut idx: i32 = 0;
            p = strchr(alpha, ch as i32);
            if p.is_null() {
                continue;
            }
            idx = (p.offset_from(alpha) as i64 % 26i64) as i32;
            total += (wasused[idx as usize] == 0) as i32;
            wasused[idx as usize] = 1;
            if total == 26 {
                return 1;
            }
        }
        return 0;
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut tests: [*const i8; 2] = [
        b"The quick brown fox jumps over the lazy dog.\0" as *const u8 as *const i8,
        b"The qu1ck brown fox jumps over the lazy d0g.\0" as *const u8 as *const i8,
    ];
    i = 0;
    unsafe {
        while i < 2 {
            if is_pangram(tests[i as usize]) != 0 {
                print!(
                    "\"{}\" is {}a pangram\n",
                    build_str_from_raw_ptr(tests[i as usize] as *mut u8),
                    "\0"
                )
            } else {
                print!(
                    "\"{}\" is {}a pangram\n",
                    build_str_from_raw_ptr(tests[i as usize] as *mut u8),
                    "not \0"
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
