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
    fn toupper(_: i32) -> i32;
}
#[no_mangle]
pub extern "C" fn can_make_words(mut b: *mut *mut i8, mut word: *mut i8) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut ret: i32 = 0;
        let mut c: i32 = toupper(*word as i32);
        if c == 0 {
            return 1;
        }
        if (*b.offset(0 as isize)).is_null() {
            return 0;
        }
        i = 0;
        while !(*b.offset(i as isize)).is_null() && ret == 0 {
            if !(*(*b.offset(i as isize)).offset(0 as isize) as i32 != c
                && *(*b.offset(i as isize)).offset(1 as isize) as i32 != c)
            {
                if *b.offset(i as isize) != *b.offset(0 as isize) {
                    let mut tmp: *mut i8 = *b.offset(i as isize);
                    let ref mut fresh0 = *b.offset(i as isize);
                    *fresh0 = *b.offset(0 as isize);
                    let ref mut fresh1 = *b.offset(0 as isize);
                    *fresh1 = tmp;
                }
                ret = can_make_words(b.offset(1 as isize), word.offset(1 as isize));
                if *b.offset(i as isize) != *b.offset(0 as isize) {
                    let mut tmp_0: *mut i8 = *b.offset(i as isize);
                    let ref mut fresh2 = *b.offset(i as isize);
                    *fresh2 = *b.offset(0 as isize);
                    let ref mut fresh3 = *b.offset(0 as isize);
                    *fresh3 = tmp_0;
                }
            }
            i += 1;
            i;
        }
        return ret;
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut blocks: [*mut i8; 21] = [
            b"BO\0" as *const u8 as *const i8 as *mut i8,
            b"XK\0" as *const u8 as *const i8 as *mut i8,
            b"DQ\0" as *const u8 as *const i8 as *mut i8,
            b"CP\0" as *const u8 as *const i8 as *mut i8,
            b"NA\0" as *const u8 as *const i8 as *mut i8,
            b"GT\0" as *const u8 as *const i8 as *mut i8,
            b"RE\0" as *const u8 as *const i8 as *mut i8,
            b"TG\0" as *const u8 as *const i8 as *mut i8,
            b"QD\0" as *const u8 as *const i8 as *mut i8,
            b"FS\0" as *const u8 as *const i8 as *mut i8,
            b"JW\0" as *const u8 as *const i8 as *mut i8,
            b"HU\0" as *const u8 as *const i8 as *mut i8,
            b"VI\0" as *const u8 as *const i8 as *mut i8,
            b"AN\0" as *const u8 as *const i8 as *mut i8,
            b"OB\0" as *const u8 as *const i8 as *mut i8,
            b"ER\0" as *const u8 as *const i8 as *mut i8,
            b"FS\0" as *const u8 as *const i8 as *mut i8,
            b"LY\0" as *const u8 as *const i8 as *mut i8,
            b"PC\0" as *const u8 as *const i8 as *mut i8,
            b"ZM\0" as *const u8 as *const i8 as *mut i8,
            0 as *mut i8,
        ];
        let mut words: [*mut i8; 9] = [
            b"\0" as *const u8 as *const i8 as *mut i8,
            b"A\0" as *const u8 as *const i8 as *mut i8,
            b"BARK\0" as *const u8 as *const i8 as *mut i8,
            b"BOOK\0" as *const u8 as *const i8 as *mut i8,
            b"TREAT\0" as *const u8 as *const i8 as *mut i8,
            b"COMMON\0" as *const u8 as *const i8 as *mut i8,
            b"SQUAD\0" as *const u8 as *const i8 as *mut i8,
            b"Confuse\0" as *const u8 as *const i8 as *mut i8,
            0 as *mut i8,
        ];
        let mut w: *mut *mut i8 = 0 as *mut *mut i8;
        w = words.as_mut_ptr();
        while !(*w).is_null() {
            print!(
                "{}	{}\n",
                build_str_from_raw_ptr(*w as *mut u8),
                can_make_words(blocks.as_mut_ptr(), *w)
            );
            w = w.offset(1);
            w;
        }
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
