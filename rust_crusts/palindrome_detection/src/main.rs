#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use c2rust_out::*;
extern "C" {
    fn strlen(_: *const i8) -> u64;
    fn printf(_: *const i8, _: ...) -> i32;
}
#[no_mangle]
pub extern "C" fn palindrome(mut s: *const i8) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut l: i32 = 0;
        l = strlen(s) as i32;
        i = 0;
        while i < l / 2 {
            if *s.offset(i as isize) as i32 != *s.offset((l - i - 1i32) as isize) as i32 {
                return 0;
            }
            i += 1;
            i;
        }
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn palindrome_r(mut s: *const i8, mut b: i32, mut e: i32) -> i32 {
    unsafe {
        if e - 1 <= b {
            return 1;
        }
        if *s.offset(b as isize) as i32 != *s.offset((e - 1i32) as isize) as i32 {
            return 0;
        }
        return palindrome_r(s, b + 1, e - 1);
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut t: *const i8 = b"ingirumimusnocteetconsumimurigni\0" as *const u8 as *const i8;
        let mut template: *const i8 =
            b"sequence \"%s\" is%s palindrome\n\0" as *const u8 as *const i8;
        let mut l: i32 = strlen(t) as i32;
        if palindrome(t) != 0 {
            printf(template, t, b"\0" as *const u8 as *const i8)
        } else {
            printf(template, t, b"n't\0" as *const u8 as *const i8)
        };
        if palindrome_r(t, 0, l) != 0 {
            printf(template, t, b"\0" as *const u8 as *const i8)
        } else {
            printf(template, t, b"n't\0" as *const u8 as *const i8)
        };
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
